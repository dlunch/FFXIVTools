# Model Viewer

Standalone Build

```
cargo build
cargo run --bin model_viewer
```

Web Build

```
npm run build
```

# Server

Init
```
docker buildx build . --file Dockerfile.init --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build
```
docker buildx build . --push --tag dlunch/ffxivtools:builder --platform linux/arm/v7,linux/amd64
```

Build Server
```
docker buildx build server --push --tag dlunch/ffxivtools:server --platform linux/arm/v7,linux/amd64
```

Run
```
docker-compose pull
docker-compose up --detach
```

Stop
```
docker-compose down
```
