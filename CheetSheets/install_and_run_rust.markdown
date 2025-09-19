# Installing Rust and Running Programs/Projects

This guide provides step-by-step instructions for installing Rust, setting up the development environment, and executing sample programs and projects. It is tailored for BTech students or beginners, with examples, checklists, and references. The content is suitable for inclusion in a Rust programming workshop, such as the one outlined for Apt Computing Labs.

## Why Rust?
Rust is a systems programming language emphasizing safety, performance, and concurrency. It’s ideal for building reliable software, from command-line tools to web servers, without common bugs like null pointer dereferences or data races.

## Installing Rust
Rust is installed using `rustup`, the official Rust toolchain installer, which manages Rust versions, Cargo (build tool), and other components.

### Step-by-Step Installation
1. **Download rustup**:
   - Visit [rustup.rs](https://rustup.rs).
   - Follow platform-specific instructions:
     - **Windows**: Download and run `rustup-init.exe`.
     - **Linux/macOS**: Run the command:
       ```bash
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
       ```
       - Press `1` for default installation when prompted.

2. **Set up environment**:
   - On Linux/macOS, add Cargo’s bin directory to your PATH:
     ```bash
     source $HOME/.cargo/env
     ```
   - Windows automatically updates PATH during installation.

3. **Verify installation**:
   - Check Rust compiler and Cargo versions:
     ```bash
     rustc --version
     cargo --version
     ```
   - Expected output (version numbers may vary):
     ```
     rustc 1.81.0 (2024-09-12)
     cargo 1.81.0 (2024-09-12)
     ```

4. **Install additional tools**:
   - Install `rust-analyzer` for IDE support (e.g., VS Code extension).
   - Optional: Install `cargo-watch` for live reloading:
     ```bash
     cargo install cargo-watch
     ```

### Checklist for Installation
- [ ] Run `rustup-init` or the curl command successfully.
- [ ] Verify `rustc --version` and `cargo --version`.
- [ ] Install VS Code and `rust-analyzer` extension.
- [ ] Ensure Cargo’s bin directory is in PATH.

## Setting Up a Development Environment
- **IDE**: Use VS Code with `rust-analyzer` for code completion and error checking.
- **Optional Tools**:
  - `rustfmt`: Auto-format code (`cargo fmt`).
  - `clippy`: Linting tool (`cargo clippy`).
  - Install both:
    ```bash
    rustup component add rustfmt clippy
    ```

## Running Simple Rust Programs
Below are examples of basic Rust programs to demonstrate compilation and execution.

### Example 1: Hello World
**Purpose**: Test basic Rust setup and compilation.

```rust
fn main() {
    println!("Hello, Rust! Welcome to Apt Computing Labs!");
}
```

**Steps**:
1. Create a file `hello.rs`.
2. Save the code above.
3. Compile and run:
   ```bash
   rustc hello.rs
   ./hello  # On Windows: hello.exe
   ```
4. Output:
   ```
   Hello, Rust! Welcome to Apt Computing Labs!
   ```

**Checklist**:
- [ ] Save and compile `hello.rs`.
- [ ] Run the program and verify output.
- [ ] Try modifying the message and recompiling.

### Example 2: Using Cargo
**Purpose**: Introduce Cargo for project management.

1. Create a new project:
   ```bash
   cargo new hello_cargo
   cd hello_cargo
   ```
2. Cargo generates:
   - `src/main.rs` (default code: prints "Hello, world!").
   - `Cargo.toml` (project configuration).

3. Run the project:
   ```bash
   cargo run
   ```
   - Compiles and runs in one step.
   - Output: `Hello, world!`

4. Build without running:
   ```bash
   cargo build
   ```
   - Creates `target/debug/hello_cargo`.

**Checklist**:
- [ ] Create and run a Cargo project.
- [ ] Explore `Cargo.toml` and `src/main.rs`.
- [ ] Run `cargo build` and check the `target` directory.

## Sample Project: Word Frequency Counter
This project builds on the previous request for a word frequency counter, demonstrating user input, `HashMap`, and error handling. It’s suitable for a workshop mini-project.

### Project Code
```rust
use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter text (press Enter twice to finish):");

    let mut input = String::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim().is_empty() {
                    break;
                }
                input.push_str(&line);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                return;
            }
        }
    }

    let words: Vec<&str> = input
        .to_lowercase()
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .collect();

    let mut word_freq = HashMap::new();
    for word in words {
        *word_freq.entry(word.to_string()).or_insert(0) += 1;
    }

    if word_freq.is_empty() {
        println!("No words entered.");
    } else {
        println!("\nWord Frequency:");
        for (word, count) in &word_freq {
            println!("{}: {}", word, count);
        }
    }
}
```

### Steps to Run
1. Create a new Cargo project:
   ```bash
   cargo new word_frequency
   cd word_frequency
   ```
2. Replace `src/main.rs` with the code above.
3. Add dependency to `Cargo.toml`:
   ```toml
   [dependencies]
   std = "0.1"
   ```
4. Run:
   ```bash
   cargo run
   ```
5. Test input:
   ```
   The quick brown fox jumps over the lazy dog.
   The dog sleeps.
   <Enter>
   <Enter>
   ```
   **Output**:
   ```
   Word Frequency:
   the: 2
   quick: 1
   brown: 1
   fox: 1
   jumps: 1
   over: 1
   lazy: 1
   dog: 2
   sleeps: 1
   ```

### Checklist for Word Frequency Project
- [ ] Create and run the project with Cargo.
- [ ] Test with different inputs (e.g., punctuation, multiple lines).
- [ ] Modify to ignore words shorter than 3 characters.
- [ ] Add a unit test for word counting.

## Sample Project: Simple Web Server with Actix Web
This project demonstrates Rust’s web programming capabilities, building a basic REST API.

### Project Setup
1. Create a new project:
   ```bash
   cargo new web_server
   cd web_server
   ```
2. Update `Cargo.toml`:
   ```toml
   [package]
   name = "web_server"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   actix-web = "4.5.0"
   serde = { version = "1.0", features = ["derive"] }
   ```

3. Replace `src/main.rs` with:
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
    HttpResponse::Ok().json(item.0)
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

4. Run:
   ```bash
   cargo run
   ```
5. Test:
   - **GET**: `curl http://localhost:8080/items`
     - Output: `[{"id":1,"name":"Laptop"},{"id":2,"name":"Phone"}]`
   - **POST**: `curl -X POST -H "Content-Type: application/json" -d '{"id":3,"name":"Tablet"}' http://localhost:8080/items`

### Checklist for Web Server Project
- [ ] Add dependencies to `Cargo.toml`.
- [ ] Run the server and test GET/POST endpoints.
- [ ] Add a new route (e.g., `GET /items/{id}`).
- [ ] Check JSON serialization with `serde`.

## Common Issues and Solutions
- **Rustup fails**: Ensure internet connectivity; retry with `curl` command.
- **Cargo build errors**: Check `Cargo.toml` for correct dependency versions.
- **Port conflicts (web server)**: Change port in `bind(("127.0.0.1", 8080))` to another (e.g., 8081).
- **IDE issues**: Update `rust-analyzer` and restart VS Code.

## Important Questions
1. What is the role of `rustup` in Rust installation?
2. How does `cargo run` differ from `rustc` for compilation?
3. Explain the structure of a Cargo project (`Cargo.toml`, `src/main.rs`).
4. How do you handle user input errors in the word frequency program?
5. What is the purpose of `serde` in the web server project?
6. How would you add a unit test for the word frequency counter?
7. Why use `actix_web::main` macro in the web server?
8. How do you update Rust to the latest version?
9. What happens if you omit `trim_matches` in the word frequency program?
10. How would you modify the web server to return a single item by ID?

## References and Resources
- [Rust Installation Guide](https://www.rust-lang.org/tools/install) - Official rustup instructions.
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Comprehensive guide.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Code examples.
- [Actix Web Documentation](https://actix.rs/docs/) - Web framework guide.
- [Cargo Documentation](https://doc.rust-lang.org/cargo/) - Package management.
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises.
- [CodeChef Rust Problems](https://www.codechef.com/learn/course/rust) - Practice projects.

## MCQ Assessment
Test your knowledge with [Rust Quiz](https://www.javaguides.net/2024/05/rust-quiz-mcq-questions-and-answers.html).  
**Sample MCQ**:
- **Question**: What command creates a new Rust project?
  - A) `rustc new`
  - B) `cargo new`
  - C) `rustup new`
  - D) `cargo init --new`
  - **Answer**: B) `cargo new`

**QR Code**: Generate a QR code for the quiz link using [qr-code-generator.com](https://www.qr-code-generator.com) and include it in your workshop materials.

## Conclusion
This guide covers installing Rust, setting up a development environment, and running programs/projects like a word frequency counter and a web server. These examples reinforce core Rust concepts (I/O, collections, async) and are ideal for hands-on learning in a BTech workshop. Explore the references and join the Rust community (e.g., r/rust on Reddit) for further support.