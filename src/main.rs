#![allow(unused)]
use illa::{
    command::{
        doctor, Opts,
    },
    result::Result,
};
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(flatten)]
    cmd: Cmd,

    #[structopt(flatten)]
    opts: Opts,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    Doctor(doctor::Cmd),
}

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();
    if let Err(e) = run(cli).await {
        eprintln!("error: {:?}", e);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result {
    match cli.cmd {
        Cmd::Doctor(cmd) => cmd.run().await,
    }
}