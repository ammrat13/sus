# Dockerfile for SUS
#
# This docker image exists to provide a consistent environment for development.
# It can also be used for builds, though. All the dependencies are installed
# beforehand.

FROM rust:1.56.0-buster

# Set the working directory
# Mount a volume here for development, or clone into this directory
WORKDIR /usr/local/src/sus/


# Install dependencies
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-audit --version 0.15.2
RUN cargo install cargo-make --version 0.35.5
