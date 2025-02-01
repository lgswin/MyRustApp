# Step 1: Build stage for the Rust application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the entire source directory first
COPY . .

# Verify Cargo.toml and dependencies
RUN ls -al /app && cat /app/Cargo.toml

# Fetch dependencies
RUN cargo fetch

# Build the release version
RUN cargo build --release

# Step 2: Use a newer runtime base image (e.g., Ubuntu 22.04)
FROM ubuntu:22.04 AS runtime

# Install dependencies required by Rust applications
RUN apt update && apt install -y libc6

# Copy the built executable
COPY --from=builder /app/target/release/my_rust_app /usr/local/bin/my_rust_app

# Expose the application's port (assuming it runs on 8080)
EXPOSE 3000

# Default command to run the application
CMD ["my_rust_app"]