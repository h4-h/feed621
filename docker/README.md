<div align="center">
  <h1>feed621 - <a href="https:://docker.com">docker</a></h1>
  <p>
    <strong>This is a readme about docker compose configuration, general purpose readme <a href="../../README.md">here</a>.</strong>
  </p>
</div>

This directory contains [docker-compose](https://docs.docker.com/compose) configuration files for this project.

- `docker-compose.yaml` - contains basic services that used in production and development modes: database, nginx etc
- `docker-compose.dev.yaml` - mostly contains services that autoreloads on file changes
- `docker-compose.prod.yaml` - contains only release builds of the applications and bare-linux containers (achieved by two stage dockerfiles)

## Requirements

1. [docker](https://docker.com)

## FAQ

This behavior is achieved by [docker-compose](https://docs.docker.com/compose) [merge feature](https://docs.docker.com/compose/multiple-compose-files/merge) that allows passing list of files to the command:

```bash
# example from docs
$ docker compose -f compose.yml -f compose.admin.yml run backup_db
```

For simplier use there are [start.sh](../scripts/start.sh) script.

Usage(run from root of project):

```bash
$ ./scripts/start.sh <dev | prod>
```

#### [LICENSE](../LICENSE)

Feel free to use, open issues and etc.
