# Running ILLA cli Setup

This document explains how you can setup a development environment for `illa` cli.

## Pre-requisite

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

## Local Setup

1. Clone the repository

```bash
git clone https://github.com/illacloud/illa.git
cd illa
```

2. Check the default Docker socket

```bash
find /var/run/docker.sock
```

3. Building and running the code

```bash
cargo run -- [subcommand] --help
```

## Need Assistance

- If you are unable to resolve any issue while doing the setup, please feel free to ask questions on our [Discord channel](https://discord.com/invite/illacloud) or initiate a [Github discussion](https://github.com/orgs/illacloud/discussions). We'll be happy to help you.
- In case you notice any discrepancy, please raise an issue on Github.