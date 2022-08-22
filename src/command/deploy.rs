use crate::{command::*, result::Result};
use anyhow::Ok;
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use indicatif::{ProgressBar, ProgressStyle};

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

    /// Set the version of ILLA Builder
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
            (true, _) => deploy_self_host(self.builder_version.as_ref(), self.port, spinner_style),
            (_, true) => deploy_cloud(spinner_style),
            _ => unreachable!(),
        };
        Ok(())
    }
}

fn deploy_self_host(version: Option<&String>, port: u16, progress_style: ProgressStyle) -> Result {
    println!("{} Running a self-hosted installation...", ui::emoji::BUILD);
    Ok(())
}

fn deploy_cloud(progress_style: ProgressStyle) -> Result {
    println!("{} Looking forward to onboarding you!", ui::emoji::CLOCK);
    Ok(())
}
