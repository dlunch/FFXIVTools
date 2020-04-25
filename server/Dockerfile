FROM scratch as init
COPY Cargo.toml /src
COPY Cargo.toml /usr/local/cargo

FROM dlunch/ffxivtools:builder as previous
FROM rust:stretch as builder

RUN apt-get update
RUN apt-get install cmake -y

WORKDIR /src
COPY --from=previous /src /src
COPY --from=previous /usr/local/cargo /usr/local/cargo
COPY . .

RUN cargo install --path server --locked --bins --root build

FROM debian:stretch-slim
COPY --from=builder /src/build/bin /server

EXPOSE 8080
WORKDIR "/server"
ENTRYPOINT ["/server/server"]
