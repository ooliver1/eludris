# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /oprish

COPY ./Cargo.toml /Cargo.toml
COPY ./pandemonium/Cargo.toml /pandemonium/Cargo.toml
COPY ./effis/Cargo.toml /effis/Cargo.toml
COPY ./todel /todel
COPY ./oprish .

RUN mkdir /pandemonium/src /effis/src
RUN touch /pandemonium/src/main.rs /effis/src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release
# Other image cannot access the target folder.
RUN --mount=type=cache,target=/target \
    cp /target/release/oprish /usr/local/bin/oprish

FROM debian:buster-slim

COPY ./wait-for-it.sh ./wait-for-it.sh

# Don't forget to also publish these ports in the docker-compose.yml file.
ARG PORT=7159

EXPOSE $PORT
ENV ROCKET_ADDRESS 0.0.0.0
ENV OPRISH_PORT $PORT

ENV RUST_LOG debug

COPY --from=builder /usr/local/bin/oprish /bin/oprish

CMD ["./wait-for-it.sh", "mariadb:3306", "-t", "0", "--", "/bin/oprish"]
