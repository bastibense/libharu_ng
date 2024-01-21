
#
# This Dockerfile will build and test the library.
#
# Copyright (c) 2024, Bastian Bense <bb@neosw.de>
#

FROM rust:slim

# Set the search path for the dynamic linker
# This is required because libharu is installed in /usr/local/lib and the
# rust docker image does not include this path in the search path.
#
ENV LD_LIBRARY_PATH=/usr/local/lib

# Install dependencies
#
RUN apt-get update && apt-get install -y \
    build-essential wget cmake \
    && rm -rf /var/lib/apt/lists/*

# Download libharu
#
WORKDIR /app/libharu
RUN wget https://github.com/libharu/libharu/archive/refs/tags/v2.4.4.tar.gz -O libharu.tar.gz \
    && tar -xzf libharu.tar.gz \
    && rm libharu.tar.gz \
    && mkdir build \
    && cd build \
    && cmake ../libharu-2.4.4 \
    && make \
    && make install

WORKDIR /app

# Copy source code
#
COPY src src
COPY Cargo.toml Cargo.lock build.rs ./

# Build and test
#
RUN cargo build --release
RUN cargo test --release