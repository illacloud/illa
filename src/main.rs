#![allow(unused)]
use clap::{Parser, Subcommand};
use illa::{
    command::{deploy, doctor, list, update},
    result::Result,
};
use std::process;

#[derive(Debug, Parser)]
#[clap(name = "illa")]
#[clap(version)]
/// Deploy a modern low-code platform in 5 Seconds!
struct Cli {
    #[clap(subcommand)]
    cmd: Cmds,
}

#[derive(Debug, Subcommand)]
enum Cmds {
    Doctor(doctor::Cmd),
    Deploy(deploy::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli).await {
        eprintln!("error: {:?}", e);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result {
    match cli.cmd {
        Cmds::Doctor(cmd) => cmd.run().await,
        Cmds::Deploy(cmd) => cmd.run().await,
        Cmds::List(cmd) => cmd.run().await,
        Cmds::Update(cmd) => cmd.run().await,
    }
}
