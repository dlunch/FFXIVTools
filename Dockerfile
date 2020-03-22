FROM rust:latest

WORKDIR /src
COPY . .
COPY --from=builder /src /src
COPY --from=builder /usr/local/cargo /usr/local/cargo

RUN apt-get update
RUN apt-get install cmake -y
RUN cargo build --release

ENTRYPOINT ["true"]
