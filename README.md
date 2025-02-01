# MyRustApp

## Overview
MyRustApp is a simple Rust-based web application that demonstrates a CI/CD pipeline for automated testing and deployment.

## Technology Used
The following technologies are used in this project:

- **Rust**: The primary programming language.
- **Cargo**: Rust’s package manager and build system.
- **Actix Web**: Web framework for building the HTTP server.
- **GitHub Actions**: Used for Continuous Integration (CI) and automated testing.
- **Docker**: Containerization for deployment.
- **AWS EC2 / S3 (Optional)**: Deployment environment.
- **Terraform (Optional)**: Infrastructure as Code (IaC) for provisioning cloud resources.

---

## Steps Followed to Implement the Pipeline

### 1. **Project Setup**
- Initialize a new Rust project using `cargo init`.
- Implement a simple Actix Web application.
- Create unit tests for the application.

### 2. **Version Control**
- Set up a GitHub repository for the project.
- Add a `.gitignore` file to exclude unnecessary files.

### 3. **CI/CD Pipeline Implementation**
- Configure **GitHub Actions** to automate testing and deployment.
- Write a **Dockerfile** to containerize the application.
- Define a **CI workflow** in `.github/workflows/ci.yml`.

### 4. **Infrastructure Setup (Optional)**
- Use Terraform to provision an EC2 instance on AWS.
- Set up a deployment strategy using GitHub Actions to deploy to AWS.

---

## How Each CI Stage is Implemented

### **1. Build Stage**
- Ensures that the Rust application compiles correctly.
- Uses GitHub Actions to install dependencies and build the application.

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Build project
        run: cargo build --verbose