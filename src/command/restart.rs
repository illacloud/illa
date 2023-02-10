use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{container::RestartContainerOptions, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use std::process;

// Executes the `illa restart` command to
// restart one or more ILLA Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("restart")
        .required(true)
        .args(&["self_host", "cloud"]),
))]
/// Restart one or more ILLA Builder
pub struct Cmd {
    /// Restart Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Restart ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => restart_local().await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn restart_local() -> Result {
    println!("{} Trying to restart the ILLA Builder...", ui::emoji::BUILD);

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
    let options = Some(RestartContainerOptions { t: 30 });
    let stop_builder = _docker.restart_container("illa_builder", options).await;
    if stop_builder.is_err() {
        println!(
            "{} {} {}",
            ui::emoji::FAIL,
            String::from("Try to restart ILLA Builder error:"),
            style(stop_builder.err().unwrap()).red(),
        );
        process::exit(1);
    }

    println!(
        "{} {}",
        ui::emoji::SUCCESS,
        style("Successfully restart the ILLA Builder.").green(),
    );

    Ok(())
}
