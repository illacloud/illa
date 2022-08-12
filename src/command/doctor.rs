use crate::{
    command::*,
    result::Result,
};
use bollard::{Docker, system::Version};


// Executes the `illa doctor` command to 
// check the prerequisites of self hosting
#[derive(Debug, StructOpt)]
/// Checks the prerequisites of self-hosted
pub struct Cmd {

}

impl Cmd {
    pub async fn run(&self) -> Result {
        println!("🛠 Checking the prerequisites for self hosting...");
        let _docker = Docker::connect_with_local_defaults().unwrap();
        match _docker.version().await {
            Ok(version) => println!("docker version: {}", version.version.unwrap()),
            Err(e) => println!("error")
        }
        Ok(())
    }
}