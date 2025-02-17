# Rust base image
FROM rust:latest AS builder

WORKDIR /app

# copy necessary files
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

# copy temmplates folder
COPY templates/ /app/templates/

# base image to run
FROM ubuntu:22.04

WORKDIR /app

RUN apt-get update && apt-get install -y libgcc1 libc6

# copy application
COPY --from=builder /app/target/release/my_rust_app /app/my_rust_app

COPY --from=builder /app/templates /app/templates

EXPOSE 3000

CMD ["./my_rust_app"]