# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /effis

COPY ./migrations /migrations
COPY ./Cargo.toml /Cargo.toml
COPY ./oprish/Cargo.toml /oprish/Cargo.toml
COPY ./pandemonium/Cargo.toml /pandemonium/Cargo.toml
COPY ./todel /todel
COPY ./effis .

RUN mkdir /oprish/src /pandemonium/src
RUN touch /oprish/src/main.rs /pandemonium/src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release
# Other image cannot access the target folder.
RUN --mount=type=cache,target=/target \
    cp /target/release/effis /usr/local/bin/effis

FROM debian:buster-slim

COPY ./wait-for-it.sh ./wait-for-it.sh

# Don't forget to also publish these ports in the docker-compose.yml file.
ARG PORT=7161

EXPOSE $PORT
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT $PORT

ENV RUST_LOG debug

COPY --from=builder /usr/local/bin/effis /bin/effis

CMD ["./wait-for-it.sh", "mariadb:3306", "-t", "0", "--", "/bin/effis"]
