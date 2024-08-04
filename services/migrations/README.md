<div align="center">
  <h1>feed621 - migrations</h1>
  <p>
    <strong>This is a readme about migration service, general purpose readme <a href="../../README.md">here</a>.</strong>
  </p>
</div>

This service should be started after the database and before any other service. This ensures that other services will not encounter schema desynchronization errors with the actual table schema in the database.

Note that we use [sqlx](https://github.com/launchbadge/sqlx) migrator, so there is no problem using this service with [MySQL](https://www.mysql.com) or any other database supported by [sqlx](https://github.com/launchdadge/sqlx).
All that you need is the correct `DATABASE_URL` environment variable.

## Requirements

1. [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

## FAQ

**All commands should be executed from this directory.**

1. Install sqlx-cli via cargo: `cargo install sqlx-cli`
2. Execute `source ../../scripts/set_database_url.sh` or manually set `DATABASE_URL`

```bash
# Create migration
$ sqlx migrate add -r <name>

# Migrate
$ sqlx migrate run

# Revert migration
$ sqlx migrate revert
```

### [License](../../LICENSE)
Feel free to use, open issues and etc.
