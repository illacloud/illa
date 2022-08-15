use crate::{
    command::*,
    result::Result,
};
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;
use bollard::{Docker, system::Version};
use console::{Emoji, style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "");
static FAIL: Emoji<'_, '_> = Emoji("❌ ", "");

// Executes the `illa doctor` command to 
// check the prerequisites of self hosting
#[derive(Debug, StructOpt)]
/// Checks the prerequisites of self-host
pub struct Cmd {

}

impl Cmd {
    pub async fn run(&self) -> Result {
        let spinner_style = ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹",
                "▸▹▹",
                "▹▸▹",
                "▹▹▸",
            ]);
        
        println!(
            "{} Checking the prerequisites of self-host...",
            LOOKING_GLASS
        );

        let pb = ProgressBar::new(0);
        pb.set_style(spinner_style.clone());

        for _ in 0..10 {
            pb.set_message(format!("{}", String::from("Checking the version of Docker...")));
            pb.inc(1);
            thread::sleep(Duration::from_millis(100));
        }

        let new_spinner_style = ProgressStyle::with_template("{wide_msg}")
        .unwrap();
        pb.set_style(new_spinner_style);
        let _docker = Docker::connect_with_local_defaults().unwrap();
        match _docker.version().await {
            Ok(version) =>  pb.finish_with_message(format!(
                "{} {}: {}\n{}",
                SUCCESS,
                String::from("Docker version"),
                version.version.unwrap(),
                String::from("123"),
            )),
            Err(e) => pb.finish_with_message(format!(
                "{} {}",
                FAIL,
                String::from("No docker exist"),
            ))
        }
        Ok(())
    }
}