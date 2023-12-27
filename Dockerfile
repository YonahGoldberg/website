# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest as builder

# Copy local code to the container image.
WORKDIR /app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

# FROM debian:buster-slim

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

ENTRYPOINT [ "./target/release/website" ]