use crate::{command::*, result::Result};
use anyhow::Ok;
use bollard::{
    container::{ListContainersOptions, StatsOptions},
    Docker,
};
use clap::{ArgAction::SetTrue, ArgGroup, Args};
use prettytable::{color, Attr};
use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use std::process;

// Executes the `illa list` command to
// get ILLA Builder info
#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("list")
        .required(true)
        .args(&["all", "self_host", "cloud"]),
))]
/// List ILLA Builder
pub struct Cmd {
    /// All ILLA Builder
    #[clap(short = 'A', long = "all", action = SetTrue)]
    all: bool,

    /// Self-hosted ILLA Builder
    #[clap(short = 'S', long = "self", action = SetTrue)]
    self_host: bool,

    /// ILLA Builder on ILLA Cloud
    #[clap(short = 'C', long = "cloud", action = SetTrue)]
    cloud: bool,
}

impl Cmd {
    pub async fn run(&self) -> Result {
        let (all, self_host, cloud) = (self.all, self.self_host, self.cloud);
        match (all, self_host, cloud) {
            (true, _, _) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            (_, true, _) => list_local().await?,
            (_, _, true) => println!("{} Looking forward to onboarding you!", ui::emoji::DIAMOND),
            _ => unreachable!(),
        };
        Ok(())
    }
}

async fn list_local() -> Result {
    let _docker = Docker::connect_with_local_defaults().unwrap();

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("ID").with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Name").with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Image").with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("State").with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    if (_docker.ping().await).is_err() {
        table.printstd();
        process::exit(1);
    }

    let mut ls_containers_filters = HashMap::new();
    ls_containers_filters.insert("name".to_string(), vec!["illa_builder".to_string()]);
    let builders = &_docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            filters: ls_containers_filters,
            ..Default::default()
        }))
        .await
        .unwrap();
    for builder in builders {
        table.add_row(Row::new(vec![
            Cell::new(&builder.id.as_ref().unwrap().as_str()[0..12])
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(builder.names.as_ref().unwrap()[0].as_str()),
            Cell::new(builder.image.as_ref().unwrap().as_str()),
            Cell::new(builder.state.as_ref().unwrap().as_str()),
        ]));
    }
    table.printstd();

    Ok(())
}
