<div align="center">
  <h1>feed621</h1>
  <p>
    <strong><a href="https://reddit.com">Reddit</a>-like feed for <a href="https://e621.net">e621</a> <span style="color: red">(NSFW)</span> pictures.</strong>
  </p>
</div>

## Requirements

[docker](https://docker.com) and nothing else.

## Installation

```bash
# Clone repo and change directory
$ git clone --depth=1 https://github.com/h4-h/feed621 && cd feed621

# Run project
$ ./scripts/start.sh prod
```

More info about docker-compose configuration and start.sh script in [docker](./docker) directory.

All configuration done in the `.env` file.

## Development

**All scripts can be executed only from the root of the project.**

For development run: `./scripts/start.sh dev`

See service readme for more information:

- [nginx](./services/nginx/README.md)
- [migrations](./services/migrations/README.md)

#### [LICENSE](./LICENSE)

Whole project is under MIT license. Feel free to use, open issues etc.
