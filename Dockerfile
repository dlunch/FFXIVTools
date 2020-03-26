FROM dlunch/ffxivtools:builder as builder
FROM rust:stretch

WORKDIR /src
COPY . .
COPY --from=builder /src /src
COPY --from=builder /usr/local/cargo /usr/local/cargo

RUN apt-get update
RUN apt-get install cmake -y
# build server only until wgpu-native containing https://github.com/gfx-rs/wgpu/pull/430 release
RUN cargo build --release --bin server

ENTRYPOINT ["true"]
