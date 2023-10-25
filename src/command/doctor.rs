use crate::{command::*, result::Result};
use bollard::Docker;
use clap::Args;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

// Executes the `illa doctor` command to
// check the prerequisites of self hosting
#[derive(Debug, Args)]
/// Checks the prerequisites of self-host
pub struct Cmd {}

impl Cmd {
    pub async fn run(&self) -> Result {
        let spinner_style = ProgressStyle::with_template("{spinner} {wide_msg}")
            .unwrap()
            .tick_strings(&["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "]);

        println!(
            "{} Checking the prerequisites of self-host...",
            ui::emoji::LOOKING_GLASS
        );

        let pb = ProgressBar::new(0);
        pb.set_style(spinner_style.clone());

        for _ in 0..10 {
            pb.set_message("Checking the version of Docker...");
            pb.inc(1);
            thread::sleep(Duration::from_millis(200));
        }

        let new_spinner_style = ProgressStyle::with_template("{wide_msg}").unwrap();
        pb.set_style(new_spinner_style);
        let _docker = Docker::connect_with_local_defaults().unwrap();
        let error_info = |pb: ProgressBar| {
            pb.println(format!(
                "{} {}\n{} {}\n\n{}\n\n{}\n\n{}\n",
                ui::emoji::FAIL,
                String::from("No running docker found."),
                ui::emoji::WARN,
                style("Please check the status of docker with command: docker info").red(),
                String::from("If you do not have Docker installed, please refer to the following content for instructions on how to install it: "),
                style("https://docs.docker.com/engine/install/").blue(),
                String::from("Once Docker is installed, please try running the command again."),
            ));
            pb.finish_with_message(format!("illa doctor exited."));
        };
        match _docker.version().await {
            Ok(version) =>  pb.finish_with_message(format!(
                "{} {}: {}\n{} {}",
                ui::emoji::SUCCESS,
                String::from("Docker version"),
                version.version.unwrap(),
                ui::emoji::SPARKLE,
                style("Success! The minimum requirement for deploying ILLA has been satisfied. Self-Host your ILLA Builder by command [illa deploy].").green(),
            )),
            Err(e) => error_info(pb),
        }
        println!();
        Ok(())
    }
}
