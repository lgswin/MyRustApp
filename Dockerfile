# Step 1: Build the Rust application
FROM rust:latest AS builder
WORKDIR /app

# Copy only necessary files first (better caching)
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the rest of the source code
COPY . .

# Ensure clean build
RUN cargo clean && cargo build --release

# Step 2: Create a smaller final image
FROM debian:latest
WORKDIR /app
COPY --from=builder /app/target/release/my_rust_app .
RUN chmod +x my_rust_app
CMD ["./my_rust_app"]