# Use Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/myrustapp

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Expose port 3000
EXPOSE 3000

# Run the application
CMD ["./target/release/myrustapp"]