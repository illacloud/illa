use crate::{command::*, result::Result};
use anyhow::Ok;
use clap::{ArgAction::SetTrue, ArgGroup, Args};

// Executes the `illa update` command to
// update the ILLA Builder with the latest docker images
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("update")
        .required(true)
        .args(&["self-host", "cloud"]),
))]
/// List ILLA Builder
pub struct Cmd {
    /// Update Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// Update ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,

    /// The ILLA Builder will update
    #[clap(short = 'i', long = "id", value_parser, requires = "update")]
    id: String,

    /// The port on which you want ILLA Builder to run
    #[clap(short = 'p', long = "port", default_value = "80")]
    port: u16,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (self_host, cloud) = (self.self_host, self.cloud);
        match (self_host, cloud) {
            (true, _) => update_local(self.id.clone()).await?,
            (_, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn update_local(id: String) -> Result {
    println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND);
    Ok(())
}
