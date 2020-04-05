FROM dlunch/ffxivtools:builder as builder
FROM rust:stretch

RUN apt-get update
RUN apt-get install cmake -y

WORKDIR /src
COPY --from=builder /src /src
COPY --from=builder /usr/local/cargo /usr/local/cargo
COPY . .

RUN cargo install --path server --locked --bins --root build
