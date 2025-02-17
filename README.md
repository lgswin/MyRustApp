## Assignemtn2 with My Rust app

### 1. **Web Application (API Server)**

- Exposes two API endpoints:
    - **POST `/user`**: Creates a new user.
    - **GET `/user/{id}`**: Fetches user details by ID.
- Written in **Rust** using the **Actix Web** framework.
- Logs all requests and responses to a local file inside the container.

- `/src/main.rc` has `/user` as post, `user/{id}` as get. their functions are connected to methods in `/src/lib.rc`
  
```rust
HttpServer::new(move || {
    App::new()
        .app_data(state.clone())
        .route("/health", web::get().to(health_check))
        .route("/user", web::post().to(create_user))
        .route("/user/{id}", web::get().to(get_user))
})
.bind("0.0.0.0:80")?
.run()
.await
```

- The application's logging initialized in `main.rc` as follows
  
```rust
// Create a log file where log messages will be written
let log_file = File::create("/var/log/myapp/application.log").unwrap();

// Initialize the logger to write logs at the Info level (and above) to the file
CombinedLogger::init(vec![
    WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
]).unwrap();
```

- I created a Dockerfile to containerize my Rust application.

```yml
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
COPY . .
RUN cargo build --release
COPY templates/ /app/templates/

FROM ubuntu:22.04
WORKDIR /app

RUN apt-get update && apt-get install -y libgcc1 libc6

COPY --from=builder /app/target/release/my_rust_app /app/my_rust_app
COPY --from=builder /app/templates /app/templates

EXPOSE 3000

CMD ["./my_rust_app"]
```
- This sets up a Docker-based environment for building and running the Rust application.

- Additionally, I added '/' to show an index page and it loads index.html from /templates foler.

- Related codes are follows

```rust
async fn index() -> impl Responder {
    let path = Path::new("templates/index.html");
    if path.exists() {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(std::fs::read_to_string(path).unwrap_or_else(|_| "<h1>Error: Could not load file</h1>".to_string()))
    } else {
        HttpResponse::InternalServerError().body("<h1>File not found in container--</h1>")
    }
}
```

### 2. **Database Container**

- `MySql server` is set up, which initializes the user table confitured in `docker-compose.yml`
- This creates a MySQL container.
  
```yml
services:
  mysql:
    image: mysql:8.0
    container_name: mysql_container
    restart: always
    environment:
      MYSQL_ROOT_PASSWORD: root_password
      MYSQL_DATABASE: rust_db
      MYSQL_USER: rust_user
      MYSQL_PASSWORD: rust_password
    ports:
      - "3306:3306"
    volumes:
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql  # inital sql script
      - mysql_data:/var/lib/mysql  # keep MySQL data when it is restarted.
```

- The database tables contain `ID` and `First/Last name fields` as defined in `init.sql`
```sql
CREATE DATABASE IF NOT EXISTS rust_db;
USE rust_db;

CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL
);

-- Insert a default user if the table is empty
INSERT INTO users (first_name, last_name)
SELECT 'John', 'Doe' FROM DUAL
WHERE NOT EXISTS (SELECT 1 FROM users LIMIT 1);
```

### 3. **Volumes and Bind Mounts**

- To ensure data persists between container resarts, `docker volumes` is used in `docker-compose.yml`
```yml
services:
  mysql:
    ...
    volumes:
      - mysql_data:/var/lib/mysql  # keep MySQL data when it is restarted.
```

- To save the application’s log file on your local machine, the log folder in the rust_app container is bind-mounted to a folder on your host

```yml
  rust_app:
    ...
    volumes:
      - ./logs:/var/log/myapp
```
- This setup allows you to check the application.log file both in the `./logs` folder on your local machine and in the `/var/log/myapp` folder inside the rust_app container.


### 4. **Networking**

- Two containers are created—one for the `MySQL server` and one for the rust_app (the application backend). 
- Both containers can communicate using the `MySQL credentials` defined in the `.env` file.

- **.env**
```script
DATABASE_URL=mysql://rust_user:rust_password@mysql:3306/rust_db
```  

- In `main.rs`, the connection to the MySQL server is established using the environment variable

```rust
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
println!("Connecting to database at: {}", database_url);
let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to MySQL");
```

### 5. **Scaling and Load Balancing**

- The application is made scalable by creating replicas in `docker-compose.yml`

```yml
rust_app:
    build: .
    deploy:
      replicas: 3 
      restart_policy:
        condition: on-failure
      mode: replicated  
      endpoint_mode: dnsrr  # load balancing policy
```

- An `Nginx` server is also implemented as a `reverse proxy` to handle `load balancing`

```yml
nginx:
  image: nginx:latest
  container_name: nginx_lb
  restart: always
  depends_on:
    - rust_app
  ports:
    - "80:80"
  volumes:
    - ./nginx.conf:/etc/nginx/nginx.conf:ro  # Nginx setting file mount
  networks:
    - rust_network
```

### 6. **Security Best Practices**

- For security best practices, credentials and security keys are stored in a `.env` file.
- User Authentication (MYSQL_USER and MYSQL_PASSWORD)
	*	Authentication is enabled with MYSQL_USER=rust_user and MYSQL_PASSWORD=rust_password.
	*	The root account (root) is protected with MYSQL_ROOT_PASSWORD=root_password.
	*	Anonymous users are not allowed by default.
- Password Authentication Applied
	*	Passwordless access is not allowed as MYSQL_USER and MYSQL_ROOT_PASSWORD are set.
	*	Authentication is enforced using either mysql_native_password or caching_sha2_password.


### 7. How to run 

- `docker compose up -d` (to create containers and run them)
- `docker ps` (to confirm containers running)
```
CONTAINER ID   IMAGE                COMMAND                  CREATED          STATUS                   PORTS                                     NAMES
a6aca62096a1   nginx:latest         "/docker-entrypoint.…"   4 minutes ago    Up 4 minutes             0.0.0.0:80->80/tcp                        nginx_lb
33ee43352f56   myrustapp-rust_app   "./my_rust_app"          4 minutes ago    Up 4 minutes             3000/tcp, 0.0.0.0:3000->80/tcp            myrustapp-rust_app-1
8229a5eac715   myrustapp-rust_app   "./my_rust_app"          4 minutes ago    Up 4 minutes             3000/tcp, 0.0.0.0:3001->80/tcp            myrustapp-rust_app-2
fe774e44f9e3   myrustapp-rust_app   "./my_rust_app"          4 minutes ago    Up 4 minutes             3000/tcp, 0.0.0.0:3002->80/tcp            myrustapp-rust_app-3
057c423d90a5   mysql:8.0            "docker-entrypoint.s…"   4 minutes ago    Up 4 minutes (healthy)   0.0.0.0:3306->3306/tcp, 33060/tcp         mysql_container
```
- cargo test (to test funtionality of apis)
  * this test will add `{ "id": 1, "firstname": "Alice", "lastname": "Lee" }` and check if it finds in the database successfully
- Open `localhost:3000` in a browser
- It shows `index.html` from myrustapp-rust_app container


