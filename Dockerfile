# Use Rust official image
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

# Use Ubuntu 22.04 for GLIBC 2.35+
FROM ubuntu:22.04

WORKDIR /app
RUN apt-get update && apt-get install -y libgcc1 libc6  # Ensure GLIBC dependencies are installed

COPY --from=builder /app/target/release/my_rust_app /app/my_rust_app

EXPOSE 3000

CMD ["./my_rust_app"]