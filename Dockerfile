
#
# This Dockerfile will build and test the library.
#
# Copyright (c) 2024, Bastian Bense <bb@neosw.de>
#

FROM rust:alpine

# Set the search path for the dynamic linker
# This is required because libharu is installed in /usr/local/lib and the
# rust docker image does not include this path in the search path.
#
ENV LD_LIBRARY_PATH=/usr/local/lib


# Install dependencies
#
# When using Ubuntu/Debian:
#
# RUN apt-get update && apt-get install -y \
#     build-essential cmake libpng-dev libz3-dev \
#     && rm -rf /var/lib/apt/lists/*

RUN apk add --no-cache \
    build-base \
    libpng-dev \
    zlib-dev \
    cmake

WORKDIR /app

# Copy source code
#
COPY libharu libharu
COPY src src
COPY Cargo.toml Cargo.lock build.rs ./

# Build and test
#
RUN cargo build --release

RUN cargo test --release