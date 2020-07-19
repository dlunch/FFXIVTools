FROM rust:stretch as builder

RUN apt-get update
RUN apt-get install cmake -y

WORKDIR /src
COPY . .

RUN cargo install --path server --locked --bins --root build

FROM debian:stretch-slim
COPY --from=builder /src/build/bin /server

EXPOSE 8080
WORKDIR "/server"
ENTRYPOINT ["/server/server"]
