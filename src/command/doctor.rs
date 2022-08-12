use crate::{
    command::*,
    result::Result,
};

// Executes the `illa doctor` command to 
// check the prerequisites of self hosting
#[derive(Debug, StructOpt)]
/// Checks the prerequisites of self-hosted
pub struct Cmd {

}

impl Cmd {
    pub async fn run(&self) -> Result {
        println!("🛠 Checking the prerequisites for self hosting...");
        Ok(())
    }
}