# MyRustApp

## Overview
MyRustApp is a simple Rust-based web application that demonstrates a CI/CD pipeline for automated testing and deployment.

## How to Run
   
   `cargo run`
   
   - Default route (/):

     Open http://localhost:3000/  > Hello, Rust.
	
   - Greeting route (/greet/:name):

     Open http://localhost:3000/greet/Alice

     Expected JSON response: > { "message": "Hello, Alice!" }

## Technology Used
The following technologies are used in this project:

- **Rust**: The primary programming language.
- **Cargo**: Rust’s package manager and build system.
- **Actix Web**: Web framework for building the HTTP server.
- **GitHub Actions**: Used for Continuous Integration (CI) and automated testing.
- **Docker**: Containerization for deployment.

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
- Define a **CI workflow** in `.github/workflows/rust.yml`.

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
    - name: Checkout Repository
      uses: actions/checkout@v4

    # Install Rust (if not available)
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    # Run Cargo Build & Test before Dockerizing
    - name: Build Rust Application
      run: cargo build --verbose

    - name: Run Rust Tests
      run: cargo test --verbose

    # Build Docker Image
    - name: Build Docker Image
      run: docker build -t my_rust_app .

    # Run Container to Verify It Works
    - name: Run Docker Container
      run: docker run --rm -d -p 3000:3000 --name rust_container my_rust_app

    # Check if the container is running properly
    - name: Check Running Container Logs
      run: docker logs rust_container

    # Stop the container
    - name: Stop Container
      run: docker stop rust_container
```