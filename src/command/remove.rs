use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{container::RemoveContainerOptions, Docker};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use console::style;
use std::process;

// Executes the `illa remove` command to
// remove one or more ILLA Builder
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("remove")
        .required(true)
        .args(&["self-host", "cloud"]),
))]
/// Remove one or more ILLA Builder
pub struct Cmd {
    /// Remove Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Remove ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,

    /// If the ILLA Builder is running, kill it before removing it
    #[clap(short = 'f', long = "force", action = SetTrue)]
    force: bool,

    /// Remove the persistent data of ILLA Builder
    #[clap(short = 'd', long = "data", action = SetTrue)]
    data: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => remove_local(self.force, self.data).await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn remove_local(is_force: bool, data: bool) -> Result {
    println!("{} Trying to remove the ILLA Builder...", ui::emoji::BUILD);

    if data {
        utils::local_bind_delete();
    }

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
    let options = Some(RemoveContainerOptions {
        force: is_force,
        ..Default::default()
    });
    let stop_builder = _docker.remove_container("illa_builder", options).await;
    if stop_builder.is_err() {
        println!(
            "{} {} {}",
            ui::emoji::FAIL,
            String::from("Try to remove ILLA Builder error:"),
            style(stop_builder.err().unwrap()).red(),
        );
        process::exit(1);
    }

    println!(
        "{} {}",
        ui::emoji::SUCCESS,
        style("Successfully remove the ILLA Builder.").green(),
    );

    Ok(())
}
