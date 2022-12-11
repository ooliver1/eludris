# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /pandemonium

COPY Cargo.lock pandemonium/Cargo.toml ./
COPY ./todel /todel
COPY ./pandemonium/src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/pandemonium/target \
    cargo build --release
# Other image cannot access the target folder.
RUN --mount=type=cache,target=/pandemonium/target \
    cp ./target/release/pandemonium /usr/local/bin/pandemonium

FROM debian:buster-slim

COPY ./wait-for-it.sh ./wait-for-it.sh
 
# Don't forget to also publish these ports in the docker-compose.yml file.
ARG PORT=7160

EXPOSE $PORT
ENV PANDEMONIUM_ADDRESS 0.0.0.0
ENV PANDEMONIUM_PORT $PORT

ENV RUST_LOG debug

COPY --from=builder /usr/local/bin/pandemonium /bin/pandemonium

CMD ["./wait-for-it.sh", "mariadb:3306", "-t", "0", "--", "/bin/pandemonium"]
