FROM builder

WORKDIR /build
RUN cargo install --path server --locked --bins --root bin

FROM debian:buster-slim
COPY --from=0 /build/bin /server

CMD ["/server/bin/server"]
