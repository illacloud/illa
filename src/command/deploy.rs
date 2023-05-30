use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::container::{Config, CreateContainerOptions, LogsOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::{HostConfig, Mount, MountTypeEnum};
use bollard::service::PortBinding;
use bollard::{service::CreateImageInfo, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use futures_util::{StreamExt, TryStreamExt};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::thread;
use std::time::{Duration, Instant};
use std::{env, process, string};
use uuid::Uuid;

const ILLA_BUILDER_IMAGE: &str = "illasoft/illa-builder";
const ILLA_BUILDER_VERSION: &str = "latest";

// Executes the `illa deploy` command to
// deploy your ILLA Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("install")
        .required(true)
        .args(&["self_host", "cloud"]),
))]
/// Deploy the ILLA Builder
pub struct Cmd {
    /// Self-hosted installation
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// ILLA Cloud installation
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,

    /// Set the version of ILLA Builder [default: latest]
    #[clap(short = 'V', long = "builder-version", value_name = "X.Y.Z")]
    builder_version: Option<String>,

    /// The port on which you want ILLA Builder to run
    #[clap(short = 'p', long = "port", default_value = "80")]
    port: u16,

    /// The mount path for the ILLA Builder
    #[clap(short = 'm', long = "mount", value_name = "/TEMP/DIR/ILLA-BUILDER")]
    mount_path: Option<String>,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
            .unwrap()
            .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => {
                deploy_self_host(
                    self.builder_version.as_ref(),
                    self.port,
                    self.mount_path.as_ref(),
                    spinner_style,
                )
                .await?
            }
            (_, true) => deploy_cloud(spinner_style).await?,
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn deploy_self_host(
    version: Option<&String>,
    port: u16,
    mount_path: Option<&String>,
    progress_style: ProgressStyle,
) -> Result {
    println!("{} Running a self-hosted installation...", ui::emoji::BUILD);

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
    let pb_download = m.add(ProgressBar::new(0));
    pb_download.set_style(progress_style.clone());
    let finish_spinner_style = ProgressStyle::with_template("{wide_msg}").unwrap();

    let default_version = ILLA_BUILDER_VERSION.to_owned();
    let builder_version = version.unwrap_or(&default_version);
    let builder_image = ILLA_BUILDER_IMAGE.to_owned() + ":" + builder_version;

    let default_mount_path = utils::get_default_mount();
    let mount_path = mount_path.unwrap_or(&default_mount_path);

    let download_started = Instant::now();
    let stream_list = &mut _docker.create_image(
        Some(CreateImageOptions {
            from_image: builder_image.clone(),
            ..Default::default()
        }),
        None,
        None,
    );

    while let Some(value) = stream_list.next().await {
        pb_download.set_message(format!("Downloading {}...", builder_image.clone()));
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

    let pg_pwd = Uuid::new_v4();
    let builder_env = vec![
        "ILLA_SERVER_MODE=release".to_string(),
        "ILLA_DEPLOY_MODE=self-host".to_string(),
        format!("POSTGRES_PASSWORD={pg_pwd}"),
    ];
    let mut builder_labels = HashMap::new();
    builder_labels.insert(
        "maintainer".to_string(),
        "opensource@illasoft.com".to_string(),
    );
    builder_labels.insert("license".to_string(), "Apache-2.0".to_string());
    let mut builder_port_bindings = HashMap::new();
    builder_port_bindings.insert(
        "2022/tcp".to_string(),
        Some(vec![PortBinding {
            host_port: Some(port.to_string()),
            host_ip: Some("0.0.0.0".to_string()),
        }]),
    );

    let local_dir = utils::local_bind_init(mount_path);
    let mounts = vec![Mount {
        target: Some("/opt/illa/database".to_string()),
        source: Some(local_dir),
        typ: Some(MountTypeEnum::BIND),
        read_only: Some(false),
        ..Default::default()
    }];

    let builder_config = Config {
        image: Some(builder_image),
        env: Some(builder_env),
        labels: Some(builder_labels),
        host_config: Some(HostConfig {
            port_bindings: Some(builder_port_bindings),
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
                style(format!("{}:{}", "http://localhost", port)).blue(),
            ));
            process::exit(0);
        }
    };

    Ok(())
}

async fn deploy_cloud(progress_style: ProgressStyle) -> Result {
    println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND);

    Ok(())
}
