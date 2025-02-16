# Rust 빌드용 베이스 이미지
FROM rust:latest AS builder

WORKDIR /app

# 필요한 파일 복사
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY . .
RUN cargo build --release

# ✅ templates 폴더도 복사
COPY templates/ /app/templates/

# 실행용 베이스 이미지
FROM ubuntu:22.04

WORKDIR /app

RUN apt-get update && apt-get install -y libgcc1 libc6

# 애플리케이션 실행 파일 복사
COPY --from=builder /app/target/release/my_rust_app /app/my_rust_app

# ✅ templates 폴더도 복사
COPY --from=builder /app/templates /app/templates

EXPOSE 3000

CMD ["./my_rust_app"]