FROM dlunch/ffxivtools:builder as builder
FROM debian:stretch-slim
COPY --from=builder /src/build/bin /server

EXPOSE 8080
WORKDIR "/server"
ENTRYPOINT ["/server/server"]
