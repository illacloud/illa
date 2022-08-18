#![allow(unused)]
use illa::{
    command::{
        doctor, Opts,
    },
    result::Result,
};
use std::process;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Cmds,
}

#[derive(Debug, Subcommand)]
enum Cmds {
    Doctor(doctor::Cmd),
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
    }
}