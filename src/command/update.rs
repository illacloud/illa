use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{
    container::{
        Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions,
        RemoveContainerOptions, StartContainerOptions, StatsOptions,
    },
    image::CreateImageOptions,
    models::{HostConfig, Mount, MountTypeEnum},
    Docker,
};
use clap::{builder, ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use futures_util::{StreamExt, TryStreamExt};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::{
    collections::HashMap,
    process, thread,
    time::{Duration, Instant},
};

// Executes the `illa update` command to
// update the ILLA Builder with the latest docker image
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("update")
        .required(true)
        .args(&["self_host", "cloud"]),
))]
/// Update ILLA Builder
pub struct Cmd {
    /// Update Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Update ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
            .unwrap()
            .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => update_local(spinner_style.clone()).await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn update_local(progress_style: ProgressStyle) -> Result {
    println!(
        "{} Updating the ILLA Builder with the latest docker image...",
        ui::emoji::BUILD
    );

    let _docker = Docker::connect_with_local_defaults().unwrap();
    if (_docker.ping().await).is_err() {
        println!(
            "{} {}\n{} {}",
            ui::emoji::FAIL,
            String::from("No running docker found."),
            ui::emoji::WARN,
            style("Please check the status of docker.").red(),
        );
        process::exit(1);
    }

    let m = MultiProgress::new();
    let finish_spinner_style = ProgressStyle::with_template("{wide_msg}").unwrap();

    let pb_setup = m.add(ProgressBar::new(0));
    pb_setup.set_style(progress_style.clone());
    for _ in 0..10 {
        pb_setup.set_message("Initializing...");
        pb_setup.inc(1);
        thread::sleep(Duration::from_millis(200));
    }
    let inspect_options = Some(InspectContainerOptions { size: false });
    let builder_detail = &_docker
        .inspect_container("illa_builder", inspect_options)
        .await;
    if builder_detail.is_err() {
        println!(
            "{} {}\n",
            ui::emoji::FAIL,
            String::from("No ILLA Builder found."),
        );
        process::exit(1);
    }
    let builder_info = builder_detail.as_ref().unwrap();
    let builder_env_cp = &builder_info.config.as_ref().unwrap().env.clone().unwrap();
    let builder_env = vec![
        builder_env_cp[0].as_str(),
        builder_env_cp[1].as_str(),
        builder_env_cp[2].as_str(),
    ];
    let builder_mount_cp = &builder_info
        .host_config
        .as_ref()
        .unwrap()
        .mounts
        .clone()
        .unwrap();
    let mounts = vec![Mount {
        target: Some("/var/lib/postgresql/data".to_string()),
        source: Some(builder_mount_cp[0].source.clone().unwrap()),
        typ: Some(MountTypeEnum::BIND),
        read_only: Some(false),
        ..Default::default()
    }];
    let mut builder_labels = HashMap::new();
    builder_labels.insert("maintainer", "opensource@illasoft.com");
    builder_labels.insert("license", "Apache-2.0");
    let builder_port_bindings = builder_info
        .host_config
        .as_ref()
        .unwrap()
        .port_bindings
        .clone();
    let port = builder_port_bindings
        .clone()
        .unwrap()
        .get("80/tcp")
        .unwrap()
        .clone();
    pb_setup.set_style(finish_spinner_style.clone());
    pb_setup.finish_with_message(format!("{} Setup complete", ui::emoji::SUCCESS));

    let pb_rm = m.add(ProgressBar::new(0));
    pb_rm.set_style(progress_style.clone());
    for _ in 0..10 {
        pb_rm.set_message("Removing ILLA Builder...");
        pb_rm.inc(1);
        thread::sleep(Duration::from_millis(200));
    }
    let rm_options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });
    let stop_builder = _docker.remove_container("illa_builder", rm_options).await;
    if stop_builder.is_err() {
        println!(
            "{} {} {}",
            ui::emoji::FAIL,
            String::from("Try to remove ILLA Builder error:"),
            style(stop_builder.err().unwrap()).red(),
        );
        process::exit(1);
    }
    pb_rm.set_style(finish_spinner_style.clone());
    pb_rm.finish_with_message(format!(
        "{} {}",
        ui::emoji::SUCCESS,
        style("Successfully remove the old ILLA Builder."),
    ));

    let pb_download = m.add(ProgressBar::new(0));
    pb_download.set_style(progress_style.clone());
    let builder_image = "illasoft/illa-builder:latest";
    let download_started = Instant::now();
    let stream_list = &mut _docker.create_image(
        Some(CreateImageOptions {
            from_image: builder_image,
            ..Default::default()
        }),
        None,
        None,
    );
    while let Some(value) = stream_list.next().await {
        pb_download.set_message(format!("Downloading {builder_image}..."));
        pb_download.inc(1);
        thread::sleep(Duration::from_millis(100));
        if value.is_err() {
            pb_download.set_style(finish_spinner_style.clone());
            pb_download.finish_with_message(format!(
                "{} {} {}",
                ui::emoji::FAIL,
                String::from("Download image error:"),
                style(value.err().unwrap()).red(),
            ));
            process::exit(1);
        };
    }
    pb_download.set_style(finish_spinner_style.clone());
    pb_download.finish_with_message(format!(
        "{} Downloaded in {}",
        ui::emoji::SUCCESS,
        HumanDuration(download_started.elapsed())
    ));

    let pb_deploy = m.add(ProgressBar::new(0));
    pb_deploy.set_style(progress_style.clone());
    let builder_config = Config {
        image: Some(builder_image),
        env: Some(builder_env),
        labels: Some(builder_labels),
        host_config: Some(HostConfig {
            port_bindings: builder_port_bindings.clone(),
            mounts: Some(mounts),
            ..Default::default()
        }),
        ..Default::default()
    };

    let create_builder = &_docker
        .create_container(
            Some(CreateContainerOptions {
                name: "illa_builder",
            }),
            builder_config,
        )
        .await;

    let start_builder = &_docker
        .start_container("illa_builder", None::<StartContainerOptions<String>>)
        .await;

    match (create_builder.is_err(), start_builder.is_err()) {
        (true, _) => {
            pb_deploy.set_style(finish_spinner_style.clone());
            pb_deploy.finish_with_message(format!(
                "{} {} {}",
                ui::emoji::FAIL,
                String::from("Create ILLA Builder error:"),
                style(create_builder.as_ref().err().unwrap()).red(),
            ));
            process::exit(1);
        }
        (false, true) => {
            pb_deploy.set_style(finish_spinner_style.clone());
            pb_deploy.finish_with_message(format!(
                "{} {} {}",
                ui::emoji::FAIL,
                String::from("Start ILLA Builder error:"),
                style(start_builder.as_ref().err().unwrap()).red(),
            ));
            process::exit(1);
        }
        _ => {
            pb_deploy.set_style(finish_spinner_style.clone());
            pb_deploy.finish_with_message(format!(
                "{} {} {}",
                ui::emoji::SPARKLE,
                String::from("ILLA Builder started, please visit"),
                style(format!(
                    "{}:{}",
                    "http://localhost",
                    port.clone()
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .host_port
                        .as_ref()
                        .unwrap()
                ))
                .blue(),
            ));
            process::exit(0);
        }
    };

    Ok(())
}
