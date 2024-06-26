<div align="center">
  <h1>feed621</h1>
  <p>
    <strong><a href="https://telegram.org">Telegram</a> bot and app that works like a <a href="https://reddit.com">Reddit</a> feed for <a href="https://e621.net">e621</a> <span style="color: red">(NSFW)</span> pictures.</strong>
  </p>
</div>

## Requirements

If you are using [docker](https://docker.com) you don't need anything else.

If you are using [podman](https://podman.io):

<details>
  <summary>Using podman-compose</summary>

  To be honest i don't think you will need anything else but [podman-compose](https://github.com/containers/podman-compose).
</details>

<details>
  <summary>Using docker-compose</summary>

  Install:

  1. [podman-docker](https://github.com/containers/podman)
  2. [docker-compose](https://github.com/docker/compose)

  AND setup a socket for podman on `unix:///run/user/1000/podman/podman.sock`:

  Systemd: `$ sudo systemctl start podman.socket` (comes with `podman-docker`)
  
  Dinit: `$ sudo dinitctl start podman.socket`

  <details>
    <summary>Dinit <code>podman.socket</code> service</summary>

    ```
    type = process
    command = /usr/bin/podman system service -t 0
    logfile = /var/log/podman-api-socket.log
    ```

  </details>
</details>

## Installation

```bash
# Clone repo
$ git clone https://github.com/h4-h/feed621

# change directory to project
$ cd feed621

# Run project
$ ./scripts/start.sh <docker | podman> <dev | prod>
```

`./scripts/start.sh <docker | podman> dev` starts a development build with autoreloading containers and development builds.

`./scripts/start.sh <docker | podman> prod` starts a production build with staged lightweight containers and release builds.

## Development

See service readme for development information:

- [backend](./services/backend/README.md)
- [database](./services/database/README.md)

#### [LICENSE](./LICENSE)

Whole project is under MIT license.
