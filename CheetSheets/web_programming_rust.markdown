# Web Programming in Rust

## Introduction
Rust, known for its safety, performance, and concurrency, is increasingly popular for web programming. Its memory safety guarantees and zero-cost abstractions make it ideal for building fast, reliable web applications. This guide introduces web programming in Rust, focusing on key frameworks, tools, and examples suitable for BTech students or beginners. It includes practical code snippets, setup instructions, references, and a checklist to ensure understanding.

## Why Rust for Web Development?
- **Performance**: Comparable to C++ with no garbage collection overhead.
- **Safety**: Prevents common bugs like null pointer dereferences and data races.
- **Ecosystem**: Growing libraries (crates) like Actix, Rocket, and Axum for web development.
- **Use Cases**: Backend APIs, full-stack apps, and WebAssembly for frontend.

## Key Frameworks
Rust offers several web frameworks, each with unique strengths:
1. **Actix Web**: High-performance, actor-based framework. Ideal for APIs. Known for topping benchmarks (e.g., Techempower).
2. **Rocket**: Developer-friendly with a focus on simplicity and type safety. Requires nightly Rust for some features.
3. **Axum**: Lightweight, modular, built on Tokio. Great for async web services.
4. **Yew/Wasm-Bindgen**: For frontend development using WebAssembly.

This guide focuses on **Actix Web** for its performance and popularity, with notes on others.

## Setting Up the Environment
To start web programming in Rust:
1. Install Rust via [rustup.rs](https://rustup.rs).
2. Verify installation: `rustc --version` and `cargo --version`.
3. Use an IDE like VS Code with the `rust-analyzer` extension.
4. Add dependencies to `Cargo.toml` for the chosen framework.

**Example `Cargo.toml` for Actix Web**:
```toml
[package]
name = "rust-web-app"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.5.0"
serde = { version = "1.0", features = ["derive"] }
```

## Basic Actix Web Application
Below is a simple REST API using Actix Web to handle GET and POST requests.

### Example Code
```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

#[get("/items")]
async fn get_items() -> impl Responder {
    let items = vec![
        Item { id: 1, name: "Laptop".to_string() },
        Item { id: 2, name: "Phone".to_string() },
    ];
    HttpResponse::Ok().json(items)
}

#[post("/items")]
async fn add_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Ok().json(item.0) // Echo back the posted item
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_items)
            .service(add_item)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Explanation
- **Dependencies**: `actix-web` for the web server, `serde` for JSON serialization/deserialization.
- **Routes**:
  - `GET /items`: Returns a hardcoded list of items as JSON.
  - `POST /items`: Accepts a JSON payload (e.g., `{"id": 3, "name": "Tablet"}`) and echoes it.
- **Async/Await**: Uses Rust’s async runtime (`actix_web::main` macro).
- **Run**: Execute with `cargo run`, then access `http://localhost:8080/items` in a browser or tool like Postman.

### Testing the API
- **GET Request**: Use `curl http://localhost:8080/items` or a browser to see the JSON response.
- **POST Request**: Use `curl -X POST -H "Content-Type: application/json" -d '{"id": 3, "name": "Tablet"}' http://localhost:8080/items`.

**Expected GET Output**:
```json
[
    {"id": 1, "name": "Laptop"},
    {"id": 2, "name": "Phone"}
]
```

## Checklist for Actix Web Setup
- [ ] Install Rust and verify with `rustc --version`.
- [ ] Add `actix-web` and `serde` to `Cargo.toml`.
- [ ] Run the example code and access `http://localhost:8080/items`.
- [ ] Test POST request with a JSON payload.
- [ ] Modify the code to add a new route (e.g., `GET /items/{id}`).

## WebAssembly with Yew
For frontend development, Rust can compile to WebAssembly (WASM) using `Yew`. Here’s a simple example:

### Setup for Yew
1. Install `wasm-pack`: `cargo install wasm-pack`.
2. Install Node.js and `npm` for frontend tooling.
3. Create a Yew project: `cargo new yew-app --bin && cd yew-app`.
4. Add dependencies:
```toml
[dependencies]
yew = "0.21"
```

### Example Yew Component
```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <h1>{"Counter App"}</h1>
            <p>{format!("Count: {}", *counter)}</p>
            <button {onclick}>{"+1"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### Build and Run
1. Run `wasm-pack build --target web`.
2. Serve with a static server: `npm install -g serve && serve pkg`.
3. Access `http://localhost:5000` to see a counter app.

**Checklist for Yew**:
- [ ] Install `wasm-pack` and Node.js.
- [ ] Build and run the Yew counter app.
- [ ] Add a decrement button to the counter.

## Advanced Topics
- **Database Integration**: Use `sqlx` or `diesel` for PostgreSQL/MySQL (e.g., `sqlx::query!("SELECT * FROM items")`).
- **Middleware**: Actix Web supports middleware for logging, authentication (e.g., `actix_web::middleware::Logger`).
- **Error Handling**: Use `Result` and custom error types for robust APIs.
- **Deployment**: Deploy Rust apps on AWS, Heroku, or Fly.io using Docker.

## Example: Adding a Database (PostgreSQL with sqlx)
```rust
use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::postgres::PgPool;

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let users = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://user:pass@localhost/db")
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

**Dependencies**:
```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-actix-native-tls", "postgres"] }
```

**Checklist for Database**:
- [ ] Set up a local PostgreSQL database.
- [ ] Add `sqlx` to `Cargo.toml` and run the example.
- [ ] Query the `/users` endpoint.

## Important Questions
1. What makes Rust suitable for web development compared to Node.js?
2. How does Actix Web handle async requests? Explain `#[actix_web::main]`.
3. Write a new Actix Web route to delete an item by ID.
4. How does WebAssembly enable Rust for frontend development?
5. Explain the role of `serde` in the Actix Web example.
6. How would you add middleware for logging in Actix Web?
7. What is the purpose of `web::Data` in the database example?
8. Compare Actix Web, Rocket, and Axum for a REST API project.
9. How do you handle errors in a Rust web application?
10. Write a Yew component to display a list of items fetched from an API.

## References and Resources
- [Actix Web Documentation](https://actix.rs/docs/) - Official guide for Actix Web.
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Covers async and I/O.
- [Yew Documentation](https://yew.rs/) - Frontend development with WebAssembly.
- [sqlx Documentation](https://docs.rs/sqlx/) - Database integration.
- [Rust by Example (Async)](https://doc.rust-lang.org/rust-by-example/std_misc/async.html) - Async programming.
- [Rust Web Framework Comparison](https://www.arewewebyet.org/) - Overview of web tools.
- [WebAssembly Guide](https://rustwasm.github.io/docs/book/) - WASM with Rust.

## MCQ Assessment
Test your knowledge with this [Rust Web Quiz](https://www.example.com/rust-web-quiz) (replace with a real quiz link, e.g., from CodeChef or create one).  
**Sample MCQ**:
- **Question**: What is the purpose of `#[actix_web::main]`?
  - A) Defines a route
  - B) Sets up an async runtime
  - C) Configures middleware
  - D) Serializes JSON
  - **Answer**: B) Sets up an async runtime

**QR Code**: Generate a QR code for the quiz link using [qr-code-generator.com](https://www.qr-code-generator.com) and include it in your presentation or documentation.

## Conclusion
Rust’s web programming ecosystem is robust, offering tools like Actix Web for backend and Yew for frontend via WebAssembly. Its safety and performance make it a compelling choice for modern web applications. Practice with the examples above, explore the referenced resources, and join the Rust community (e.g., r/rust on Reddit) for further learning.