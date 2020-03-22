FROM rust:latest

WORKDIR /build
COPY . .

RUN apt-get update
RUN apt-get install cmake -y
RUN cargo build --release

CMD ["true"]
