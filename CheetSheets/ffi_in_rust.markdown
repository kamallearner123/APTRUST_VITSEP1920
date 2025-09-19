# Foreign Function Interface (FFI) in Rust

This guide introduces the Foreign Function Interface (FFI) in Rust, designed for BTech students or beginners in a workshop setting, such as the one organized by Apt Computing Labs. It explains how to call C functions from Rust and vice versa, with setup instructions, practical examples, checklists, important questions, and references. The content integrates with the Rust programming curriculum previously discussed (e.g., installation, web programming) and is suitable for a hands-on session.

## What is FFI?
The Foreign Function Interface (FFI) enables Rust to interoperate with other programming languages, primarily C, by calling functions or sharing data across language boundaries. This is crucial for leveraging existing C libraries, integrating with legacy code, or embedding Rust in systems where C is prevalent (e.g., OS APIs, embedded systems).

### Why Use FFI in Rust?
- **Interoperability**: Use mature C libraries (e.g., `libcurl`, `openssl`) without rewriting.
- **Performance**: Combine Rust’s memory safety with C’s low-level control.
- **Flexibility**: Access system-level APIs or libraries unavailable in Rust’s crate ecosystem.
- **Use Cases**: System programming, binding to hardware drivers, or integrating Rust into C/C++ projects.

### Key Concepts
- **Safety**: FFI requires `unsafe` blocks because foreign code doesn’t adhere to Rust’s safety guarantees.
- **Bindings**: Tools like `bindgen` generate Rust bindings from C header files.
- **Data Types**: Rust types must map to C types (e.g., `i32` to `int`, `*mut c_void` to `void*`).
- **Calling Conventions**: Use `extern "C"` to ensure compatibility with C’s function calling mechanism.
- **Memory Management**: Rust’s ownership rules don’t apply to C; manual memory handling is needed.

## Setting Up FFI in Rust
### Prerequisites
1. **Rust Toolchain**: Install via `rustup` (see [rustup.rs](https://rustup.rs)).
   - Verify: `rustc --version` and `cargo --version`.
2. **C Compiler**: Required for compiling C code.
   - **Linux**: `sudo apt-get install build-essential`.
   - **macOS**: `xcode-select --install`.
   - **Windows**: Install MinGW (`mingw-w64`) or MSVC build tools.
3. **Bindgen**: For generating Rust bindings from C headers.
   - Install: `cargo install bindgen-cli`.
   - Requires `libclang` (install via `apt-get install libclang-dev` on Linux or equivalent).
4. **IDE**: Use VS Code with `rust-analyzer` for code completion.

### Checklist for Setup
- [ ] Verify Rust installation (`rustc --version`).
- [ ] Install a C compiler (`gcc --version` or `clang --version`).
- [ ] Install `bindgen-cli` and `libclang`.
- [ ] Set up VS Code with `rust-analyzer`.

## Example 1: Calling a C Function from Rust
This example calls a simple C function to compute the square of a number.

### C Code (`square.c`)
```c
int square(int num) {
    return num * num;
}
```

### Compile C Code
1. Compile to a static library:
   ```bash
   gcc -c square.c -o square.o
   ar rcs libsquare.a square.o
   ```

### Rust Code
Create a Cargo project:
```bash
cargo new rust_ffi_example
cd rust_ffi_example
```

Add to `Cargo.toml`:
```toml
[package]
name = "rust_ffi_example"
version = "0.1.0"
edition = "2021"

[dependencies]

[build-dependencies]
cc = "1.0"
```

Create `build.rs` in the project root:
```rust
fn main() {
    cc::Build::new()
        .file("square.c")
        .compile("square");
}
```

Replace `src/main.rs` with:
```rust
extern "C" {
    fn square(num: i32) -> i32;
}

fn main() {
    unsafe {
        let result = square(5);
        println!("Square of 5 is: {}", result);
    }
}
```

### Steps to Run
1. Place `square.c` in the project root.
2. Run:
   ```bash
   cargo run
   ```
3. Output:
   ```
   Square of 5 is: 25
   ```

### Explanation
- **C Code**: Defines a simple `square` function.
- **Rust Code**:
  - `extern "C"` declares the C function’s signature.
  - `unsafe` is used because Rust cannot guarantee C code safety.
- **Build Script**: Uses the `cc` crate to compile `square.c` into a static library.
- **Linking**: Cargo links `libsquare.a` automatically.

### Checklist for Example 1
- [ ] Create and compile `square.c`.
- [ ] Set up the Rust project with `build.rs` and `cc` crate.
- [ ] Run the program and verify output.
- [ ] Modify to square a user-input number.

## Example 2: Using Bindgen with a C Library
This example uses `bindgen` to call functions from the C standard library (`libc`).

### Setup
1. Ensure `libclang` is installed (required by `bindgen`).
2. Create a new project:
   ```bash
   cargo new rust_bindgen_example
   cd rust_bindgen_example
   ```

3. Update `Cargo.toml`:
```toml
[package]
name = "rust_bindgen_example"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[build-dependencies]
bindgen = "0.68"
```

4. Create `build.rs`:
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

5. Create `wrapper.h` in the project root:
```c
#include <stdio.h>
#include <string.h>
```

6. Replace `src/main.rs` with:
```rust
use libc::c_char;
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    fn printf(format: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let c_string = CString::new("Hello from Rust via FFI!").expect("CString failed");
    unsafe {
        printf(c_string.as_ptr());
        let len = strlen(c_string.as_ptr());
        println!("\nLength of string: {}", len);
    }
}
```

### Steps to Run
1. Ensure `wrapper.h` is in the project root.
2. Run:
   ```bash
   cargo run
   ```
3. Output:
   ```
   Hello from Rust via FFI!
   Length of string: 23
   ```

### Explanation
- **Bindgen**: Generates Rust bindings for `stdio.h` and `string.h` via `wrapper.h`.
- **CString**: Converts Rust `String` to C-compatible null-terminated string.
- **libc**: Provides C type definitions (e.g., `c_char`).
- **Unsafe**: Required for calling C functions and handling raw pointers.

### Checklist for Example 2
- [ ] Install `bindgen` and `libclang`.
- [ ] Set up the project with `build.rs` and `wrapper.h`.
- [ ] Run and verify `printf` and `strlen` outputs.
- [ ] Modify to print a user-defined string.

## Example 3: Calling Rust from C
This example shows how C code can call a Rust function.

### Rust Code
Create a new library project:
```bash
cargo new --lib rust_from_c
cd rust_from_c
```

Replace `src/lib.rs` with:
```rust
#[no_mangle]
pub extern "C" fn rust_multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

### C Code (`main.c`)
```c
#include <stdio.h>

extern int rust_multiply(int a, int b);

int main() {
    int result = rust_multiply(4, 5);
    printf("4 * 5 = %d\n", result);
    return 0;
}
```

### Steps to Run
1. Build the Rust library:
   ```bash
   cargo build --release
   ```
   - Creates `target/release/librust_from_c.a`.
2. Compile and link the C code:
   ```bash
   gcc main.c -L ./target/release -lrust_from_c -o main -ldl -pthread
   ```
3. Run:
   ```bash
   ./main
   ```
4. Output:
   ```
   4 * 5 = 20
   ```

### Explanation
- **`#[no_mangle]`**: Prevents Rust from mangling the function name, making it accessible to C.
- **extern "C"**: Ensures C-compatible calling convention.
- **Linking**: The C compiler links against the Rust static library.

### Checklist for Example 3
- [ ] Build the Rust library with `cargo build --release`.
- [ ] Compile and run the C program.
- [ ] Verify the multiplication result.
- [ ] Add a new Rust function (e.g., `rust_add`) and call it from C.

## Integration with Rust Workshop
FFI can be taught as an advanced topic on **Day 2** of the Rust workshop (e.g., after concurrency or the mini-project). Add it as **Topic 11: Introduction to FFI** (1 hour):

### Workshop Plan Addition
```markdown
### Topic 11: Introduction to FFI (1 Hour)
**Objectives**: Understand how to call C functions from Rust and vice versa.

**Content**:
- FFI basics: `extern "C"`, `unsafe`, and type mapping.
- Tools: `bindgen` for bindings, `cc` for compiling C code.
- Example: Call a C `square` function from Rust.

**Example**:
```rust
extern "C" {
    fn square(num: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Square of 5: {}", square(5));
    }
}
```

**Activity**: Implement Example 1 (call C `square` function).

**Checklist**:
- [ ] Compile and link C code with Rust.
- [ ] Run the program and verify output.
- [ ] Discuss why `unsafe` is needed for FFI.
```

## Common Issues and Solutions
- **Linker Errors**: Ensure the C library is in the linker path (`-L` flag).
- **Bindgen Fails**: Verify `libclang` installation and set `LIBCLANG_PATH` if needed.
- **Segmentation Faults**: Check pointer validity in `unsafe` blocks.
- **Windows Linking**: Use MinGW or MSVC; ensure `-ldl -pthread` for Linux/macOS.

## Important Questions
1. What is the purpose of `extern "C"` in Rust FFI?
2. Why are `unsafe` blocks required for FFI calls?
3. How does `bindgen` simplify FFI development?
4. Explain the role of `#[no_mangle]` in calling Rust from C.
5. How do you map a Rust `String` to a C string?
6. What is the difference between `c_char` and Rust’s `char`?
7. How would you call a C function that returns a `void*`?
8. Why must you manually manage memory in FFI?
9. How do you compile C code in a Rust project using `cc`?
10. Write a Rust function callable from C to compute the factorial of a number.

## References and Resources
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html) - Official Nomicon chapter on FFI.
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-an-unsafe-function-or-method) - Unsafe Rust and FFI.
- [Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/) - Generating bindings.
- [libc Crate](https://docs.rs/libc) - C standard library types.
- [cc Crate](https://docs.rs/cc) - Compiling C code in Rust projects.
- [Rust by Example (FFI)](https://doc.rust-lang.org/rust-by-example/ffi.html) - Practical examples.
- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) - Advanced FFI patterns.

## MCQ Assessment
Test your knowledge with a Rust FFI quiz (create one or use a platform like CodeChef).  
**Sample MCQ**:
- **Question**: What does `#[no_mangle]` do in Rust FFI?
  - A) Enables async execution
  - B) Prevents function name mangling
  - C) Specifies a C type
  - D) Marks a function as unsafe
  - **Answer**: B) Prevents function name mangling

**QR Code**: Generate a QR code for a quiz link using [qr-code-generator.com](https://www.qr-code-generator.com) and include it in workshop materials.

## Conclusion
Rust’s FFI enables seamless integration with C, making it powerful for system-level programming and leveraging existing libraries. The examples provided (calling C from Rust, using `bindgen`, and calling Rust from C) offer practical experience for BTech students. Incorporate FFI into your Rust workshop to demonstrate Rust’s interoperability, and explore the referenced resources for deeper learning.