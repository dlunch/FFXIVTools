FROM dlunch/ffxivtools:builder as builder

WORKDIR /src
RUN cargo install --path server --locked --bins --root build

FROM debian:stretch-slim
COPY --from=0 /src/build/bin /server

EXPOSE 8080
WORKDIR "/server"
ENTRYPOINT ["/server/server"]
