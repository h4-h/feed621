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
$ ./scripts/start.sh <dev | prod>
```

- `dev` starts a development build with autoreloading containers and development builds.
- `prod` starts a production build with staged lightweight containers and release builds.

All configuration done in the `.env` file.

## Development

All development happens inside containers, do not try run services outside of the containers.

For development run: `./scripts/start.sh dev`

See service readme for development information:

- [nginx](./services/nginx/README.md)
- [migrations](./services/migrations/README.md)

#### [LICENSE](./LICENSE)

Whole project is under MIT license. Feel free to use, open issues etc.
