<div align="center">
  <h1>feed621 - database</h1>
  <p>
    <strong>This is a readme about development, general purpose readme <a href="../../README.md">here</a>.</strong>
  </p>
</div>

Database service, tbh responsible only for the migrations.

## Requirements

1. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

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
