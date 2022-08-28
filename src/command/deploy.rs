use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::container::{Config, CreateContainerOptions, LogsOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::{service::CreateImageInfo, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use futures_util::{StreamExt, TryStreamExt};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fmt::format;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

const ILLA_BUILDER_IMAGE: &str = "appsmith/appsmith-editor";
const ILLA_BUILDER_VERSION: &str = "latest";

// Executes the `illa deploy` command to
// deploy your ILLA Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("install")
        .required(true)
        .args(&["self-host", "cloud"]),
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
    #[clap(short = 'p', long = "port", default_value = "8999")]
    port: u16,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
            .unwrap()
            .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => deploy_self_host(self.builder_version.as_ref(), self.port, spinner_style)
                .await
                .unwrap(),
            (_, true) => deploy_cloud(spinner_style).await.unwrap(),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn deploy_self_host(
    version: Option<&String>,
    port: u16,
    progress_style: ProgressStyle,
) -> Result {
    let pb = ProgressBar::new(0);
    pb.set_style(progress_style.clone());
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

    let default_version = ILLA_BUILDER_VERSION.to_owned();
    let builder_version = version.unwrap_or(&default_version);
    let builder_image = ILLA_BUILDER_IMAGE.to_owned() + ":" + builder_version;

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
        pb.set_message(format!("Downloading {}...", builder_image.clone()));
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));
        if value.is_err() {
            pb.finish_with_message(format!(
                "{} {} {}",
                ui::emoji::FAIL,
                String::from("Download image error:"),
                style(value.err().unwrap()).red(),
            ));
            process::exit(1);
        };
    }
    println!(
        "{} Downloaded in {}",
        ui::emoji::SUCCESS,
        HumanDuration(download_started.elapsed())
    );

    Ok(())
}

async fn deploy_cloud(progress_style: ProgressStyle) -> Result {
    println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND);

    Ok(())
}
