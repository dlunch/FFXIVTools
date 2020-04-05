# FFXIVTools

Reimplementation of <https://ffxiv.dlunch.net> in rust.

- Latest `master` branch preview: <https://ffxiv-dev-new.dlunch.net>
- Latest stable: <https://ffxiv-new.dlunch.net>

# Status

Nothing is working.

# Build instructions

## Main website

Build

```
npm run build
```

Devserver

```
npm run devserver
```

## Server

Run

```
docker run -p 8080:8080 -d --volume=<data path>:/server/data dlunch/ffxivtools:server
```

### Uploading server image

Execute once

```
docker buildx build . --file server/Dockerfile.init --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build Builder

```
docker buildx build . --file server/Dockerfile --target builder --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build Server

```
docker buildx build . --file server/Dockerfile --push --tag dlunch/ffxivtools:server --platform linux/arm/v7,linux/amd64
```
