FROM builder

WORKDIR /src
RUN cargo install --path server --locked --bins --root build

FROM debian:buster-slim
COPY --from=0 /src/build /server

CMD ["/server/bin/server"]
