FROM dlunch/ffxivtools:builder as builder
FROM rust:stretch

RUN apt-get update
RUN apt-get install cmake -y

WORKDIR /src
COPY --from=builder /src /src
COPY --from=builder /usr/local/cargo /usr/local/cargo
COPY . .

# build server only until wgpu-native containing https://github.com/gfx-rs/wgpu/pull/430 release
RUN cargo build --release --bin server
