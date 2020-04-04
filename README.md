# ffxiv.dlunch.net

Reimplementation of [ffxiv.dlunch.net](ffxiv.dlunch.net) in rust.

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
docker run -d --volume=<data path>:/server/data dlunch/ffxivtools:server
```

### Uploading server image

Execute once

```
docker buildx build . --file Dockerfile.init --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build Builder

```
docker buildx build . --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build Server

```
docker buildx build server --push --tag dlunch/ffxivtools:server --platform linux/arm/v7,linux/amd64
```
