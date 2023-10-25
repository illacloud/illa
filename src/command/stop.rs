use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{container::StopContainerOptions, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use std::process;

// Executes the `illa stop` command to
// stop one or more ILLA Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("stop")
        .required(true)
        .args(&["self_host", "cloud"]),
))]
/// Stop one or more ILLA Builder
pub struct Cmd {
    /// Stop Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Stop ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => stop_local().await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn stop_local() -> Result {
    println!("{} Trying to stop the ILLA Builder...", ui::emoji::BUILD);

    let _docker = Docker::connect_with_local_defaults().unwrap();
    if (_docker.ping().await).is_err() {
        println!(
            "{} {}\n{} {}\n\n{}\n\n{}\n\n{}\n",
            ui::emoji::FAIL,
            String::from("No running docker found."),
            ui::emoji::WARN,
            style("Please check the status of docker with command: docker info").red(),
            String::from("If you do not have Docker installed, please refer to the following content for instructions on how to install it: "),
            style("https://docs.docker.com/engine/install/").blue(),
            String::from("Once Docker is installed, please try running the command again."),
        );
        process::exit(1);
    }

    let options = Some(StopContainerOptions { t: 30 });
    let stop_builder = _docker.stop_container("illa_builder", options).await;
    if stop_builder.is_err() {
        println!(
            "{} {} {}",
            ui::emoji::FAIL,
            String::from("Try to stop ILLA Builder error:"),
            style(stop_builder.err().unwrap()).red(),
        );
        process::exit(1);
    }

    println!(
        "{} {}",
        ui::emoji::SUCCESS,
        style("Successfully stop the ILLA Builder.").green(),
    );

    Ok(())
}
