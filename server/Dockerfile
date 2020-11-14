#syntax=docker/dockerfile:experimental

# We have to use stretch for armv7 build due to https://bugs.launchpad.net/qemu/+bug/1805913, but docker rust:stretch doesn't have latest version, so we manually install rust on debian:stretch

FROM debian:stretch as builder

RUN apt-get update && apt-get install curl -y

ENV RUSTUP_HOME=/rust
ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:/rust/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt-get install cmake libssl-dev pkg-config -y

WORKDIR /src
COPY . .

RUN --mount=type=cache,target=/src/target cargo install --path server --locked --bins --root build

FROM debian:stretch-slim
COPY --from=builder /src/build/bin /server

EXPOSE 8000
WORKDIR "/server"
ENV ROCKET_ENV production
ENTRYPOINT ["/server/server"]
