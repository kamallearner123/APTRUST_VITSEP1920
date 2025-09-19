
# 🦀 Rust Structs Cheat Sheet

This document explains **structs in Rust** with examples, methods, traits, and usage.

---

## 1. What is a Struct?
A **struct** groups related data into a single custom data type.

Types of structs:
- **Named struct**
- **Tuple struct**
- **Unit-like struct**

---

## 2. Named Struct

```rust
struct Student {
    name: String,
    age: u8,
    active: bool,
}

fn main() {
    let s = Student {
        name: "Alice".to_string(),
        age: 20,
        active: true,
    };
    println!("{} is {} years old", s.name, s.age);
}
```

✅ Best for readability.

---

## 3. Tuple Struct

```rust
struct Point(i32, i32);

fn main() {
    let p = Point(10, 20);
    println!("x={}, y={}", p.0, p.1);
}
```

✅ Useful for lightweight wrappers.

---

## 4. Unit-like Struct

```rust
struct Marker;

fn main() {
    let _m = Marker;
}
```

✅ Acts as a marker or type-level flag.

---

## 5. Methods (`impl`)

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn new(w: f64, h: f64) -> Self {
        Self { width: w, height: h }
    }
}

fn main() {
    let r = Rectangle::new(4.0, 5.0);
    println!("Area = {}", r.area());
}
```

✅ Use `&self` for borrowing, `&mut self` for mutable methods, `Self` for constructors.

---

## 6. Struct Update Syntax

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let u1 = User {
        username: "kamal".into(),
        email: "k@example.com".into(),
        active: true,
    };

    // update syntax
    let u2 = User {
        email: "new@example.com".into(),
        ..u1
    };
}
```

✅ Reuses fields from another instance.

---

## 7. Deriving Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let b1 = Book { title: "Rust".into(), pages: 100 };
    let b2 = b1.clone();
    println!("{:?}", b1);
    println!("Equal? {}", b1 == b2);
}
```

✅ Common derives: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Copy`.

---

## 8. Structs with Lifetimes

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

fn main() {
    let b = Book {
        title: "Rust Programming",
        author: "Ferris",
    };
    println!("{} by {}", b.title, b.author);
}
```

✅ Needed when storing references.

---

## 9. Example with Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let a = Article { title: "Rust News".into(), author: "Kamal".into() };
    println!("{}", a.summarize());
}
```

✅ Structs can implement traits for polymorphism.

---

# ✅ Summary

- **Named struct** → clear, readable data grouping.  
- **Tuple struct** → lightweight wrappers.  
- **Unit struct** → markers.  
- **Methods** → use `impl` for functionality.  
- **Update syntax** → copy values from another instance.  
- **Traits** → add shared behavior.  
- **Derives** → auto-implement common traits.  
- **Lifetimes** → required for structs with references.  

---

# 🔗 References
- [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
