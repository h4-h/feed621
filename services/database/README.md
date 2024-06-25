<div align="center">
  <h1>feed621 - database</h1>
  <p>
    <strong>This is a readme about development, general purpose readme <a href="../../README.md">here</a>.</strong>
  </p>
</div>

## Requirements

1. docker/podman (see [general purpose readme](../../README.md) for more information)
2. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

## Used technologies

1. [podman](https://podman.io) - containerization
2. [docker-compose](https://github.com/docker/compose) - declarative multi-container tool
3. [postgresql](https://www.postgresql.org) - i don't know why you would use something else

## Migrations

1. Install sqlx-cli via cargo: `cargo install sqlx-cli`
2. Execute `source ./scripts/set_database_url.sh`
3. change directory `cd ./services/database`

```bash
# Create migration
$ sqlx migrate add -r <name>

# Migrate
$ sqlx migrate run

# Revert migration
$ sqlx migrate revert
```
