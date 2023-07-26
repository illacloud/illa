# ILLA cli subcommands

## Deploy

Command name: `deploy`

Use: Deploy a new ILLA Builder Docker instance with the given name `illa_builder`.

Options:

- `-S, --self`: Self-hosted installation

- `-C, --cloud`: ILLA Cloud installation

- `-V, --builder-version <X.Y.Z>`: Set the version of ILLA Builder. The default value is `latest`.

- `-p, --port <PORT>`: Set the port of ILLA Builder. The default value is `80`.

- `-m, --mount <PATH>`: The mount path for the ILLA Builder. The default value is `/var/lib/illa`.

- `-h, --help`: Prints help information

## Stop

Command name: `stop`

Use: Stop one or more ILLA Builder.

Options:

- `-S, --self`: Stop Self-hosted ILLA Builder

- `-C, --cloud`: Stop ILLA Builder on ILLA Cloud

- `-h, --help`: Prints help information

## Restart

Command name: `restart`

Use: Restart one or more ILLA Builder.

Options:

- `-S, --self`: Restart Self-hosted ILLA Builder

- `-C, --cloud`: Restart ILLA Builder on ILLA Cloud

- `-h, --help`: Prints help information

## Remove

Command name: `remove`

Use: Remove one or more ILLA Builder.

Options:

- `-S, --self`: Remove Self-hosted ILLA Builder

- `-C, --cloud`: Remove ILLA Builder on ILLA Cloud

- `-f, --force`: Force the removal of a ILLA Builder Docker instance (uses SIGKILL)

- `-d, --data`: Remove the persistent data of ILLA Builder

- `-h, --help`: Prints help information

## Update

Command name: `update`

Use: Update ILLA Builder to latest version.

Options:

- `-S, --self`: Update Self-hosted ILLA Builder

- `-C, --cloud`: Update ILLA Builder on ILLA Cloud

- `-h, --help`: Prints help information

## List

Command name: `list`

Use: List ILLA Builder.

Options:

- `-A, --all`: All ILLA Builder

- `-S, --self`: List Self-hosted ILLA Builder

- `-C, --cloud`: List ILLA Builder on ILLA Cloud

- `-h, --help`: Prints help information

## Doctor

Command name: `doctor`

Use: Check the pre-requisites of self-host.

Options:

- `-h, --help`: Prints help information

## Help

Command name: `help`

User: Print help information.