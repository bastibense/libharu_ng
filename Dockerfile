FROM rust:alpine

WORKDIR /opt

COPY json_to_pdf /opt/json_to_pdf
COPY libharu_ng /opt/libharu_ng
COPY Cargo.toml /opt/Cargo.toml

RUN apk add --no-cache libharu-dev libharu

RUN cargo build --release