# Rust Command‑Line Arguments — A Practical Guide

This guide walks you through building command‑line programs in Rust, from the standard library’s `std::env::args` to ergonomic argument parsers like **clap**. You’ll see runnable examples, best practices, tests, and common pitfalls.

---

## Table of Contents
1. [Quick Start: `std::env::args`](#quick-start-stdenvargs)
2. [Safer Unicode Handling: `args_os` and `OsString`](#safer-unicode-handling-args_os-and-osstring)
3. [Parsing Patterns (numbers, flags, options)](#parsing-patterns-numbers-flags-options)
4. [Designing Testable CLIs (no global state in `main`)](#designing-testable-clis-no-global-state-in-main)
5. [Exit Codes & Error Messages](#exit-codes--error-messages)
6. [Using `clap` (derive API) for DX](#using-clap-derive-api-for-dx)
7. [Subcommands with `clap`](#subcommands-with-clap)
8. [Reading from STDIN vs. Arguments](#reading-from-stdin-vs-arguments)
9. [Quoting, Globbing & Shell Expansion](#quoting-globbing--shell-expansion)
10. [Cross‑Platform Notes (Windows, Linux, macOS)](#crossplatform-notes-windows-linux-macos)
11. [Mini Projects](#mini-projects)
12. [Further Tips](#further-tips)

---

## Quick Start: `std::env::args`

`std::env::args()` returns an iterator over **UTF‑8** `String`s: the program name (argv[0]) and all arguments.

```rust
// src/main.rs
use std::env;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "app".into());

    // Expect exactly two positional args
    let file = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {program} <input-file> <output-file>");
        std::process::exit(2);
    });
    let out = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {program} <input-file> <output-file>");
        std::process::exit(2);
    });

    if args.next().is_some() {
        eprintln!("Too many arguments");
        std::process::exit(2);
    }

    println!("Reading from: {file}");
    println!("Writing to  : {out}");
}
```

Run:
```bash
cargo run -- input.txt out.txt
```

**Gotcha:** `env::args()` **will panic** if any argument isn’t valid UTF‑8. Prefer `args_os` when you can’t guarantee UTF‑8 (see next section).

---

## Safer Unicode Handling: `args_os` and `OsString`

Use `std::env::args_os()` to accept **non‑UTF‑8** paths (e.g., Windows). It yields `OsString`:

```rust
use std::{env, ffi::OsString};

fn main() {
    let mut args = env::args_os(); // Iterator<Item = OsString>
    let _program: OsString = args.next().unwrap_or_default();

    let path: OsString = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: myapp <path>");
            std::process::exit(2);
        }
    };

    println!("Path (OS native): {:?}", path);
}
```

Convert `OsString` to `PathBuf` for filesystem work:
```rust
use std::path::PathBuf;
// let pb = PathBuf::from(path);
```

---

## Parsing Patterns (numbers, flags, options)

### Positional numbers
```rust
use std::env;

fn main() {
    let mut args = env::args();
    let _prog = args.next();
    let n: i64 = args
        .next()
        .unwrap_or_else(|| {
            eprintln!("Usage: sum_to <N>");
            std::process::exit(2);
        })
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("N must be an integer: {e}");
            std::process::exit(2);
        });

    let sum: i64 = (1..=n).sum();
    println!("{sum}");
}
```

### Boolean flags and key=value options (manual)
```rust
use std::env;

fn main() {
    let mut verbose = false;
    let mut count: usize = 1;
    let mut files: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            _ if arg.starts_with("--count=") => {
                let v = &arg["--count=".len()..];
                count = v.parse().unwrap_or_else(|_| {
                    eprintln!("--count expects a positive integer");
                    std::process::exit(2);
                });
            }
            _ if arg.starts_with('-') => {
                eprintln!("Unknown flag: {arg}");
                std::process::exit(2);
            }
            _ => files.push(arg),
        }
    }

    if verbose {
        eprintln!("count={count}, files={files:?}");
    }
    for _ in 0..count {
        for f in &files {
            println!("{f}");
        }
    }
}
```

---

## Designing Testable CLIs (no global state in `main`)

Structure your app so logic doesn’t live in `main()`. Pass an iterator of args and return `Result`:

```rust
use std::error::Error;

pub fn run<I, S>(mut args: I) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = S>,
    S: Into<String>,
{
    let _program = args.next();
    let name = args.next().ok_or("Usage: greet <name>")?.into();
    println!("Hello, {name}!");
    Ok(())
}

fn main() {
    if let Err(e) = run(std::env::args()) {
        eprintln!("{e}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greets() {
        let args = vec!["app", "Rust"].into_iter().map(String::from);
        assert!(run(args).is_ok());
    }
}
```

This pattern makes unit testing easy and keeps `main` minimal.

---

## Exit Codes & Error Messages

Use non‑zero exit codes on error:
- `1` for general errors
- `2` for usage / argument errors
- `0` for success

```rust
fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    // work...
    Ok(())
}
```

---

## Using `clap` (derive API) for DX

**clap** dramatically improves parsing (help, validation, defaults).

`Cargo.toml`:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
// src/main.rs
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Mode { Fast, Safe }

#[derive(Parser, Debug)]
#[command(name="wc-lite", version, about="Count lines/words/chars")]
struct Args {
    /// Input file (omit for stdin)
    #[arg(short, long)]
    file: Option<String>,

    /// Choose mode
    #[arg(value_enum, short, long, default_value_t=Mode::Safe)]
    mode: Mode,

    /// Be verbose
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        eprintln!("Args: {:?}", args);
    }
    // open args.file or read stdin...
}
```

`clap` gives you `--help`, `--version`, validation, and good error messages for free.

---

## Subcommands with `clap`

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="tool", version, about="A demo with subcommands")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet { name: String },
    /// Sum integers
    Sum { nums: Vec<i64> },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Greet { name } => println!("Hello, {name}!"),
        Commands::Sum { nums } => {
            let s: i64 = nums.iter().sum();
            println!("{s}");
        }
    }
}
```

Examples:
```bash
cargo run -- greet Alice
cargo run -- sum 10 20 30
```

---

## Reading from STDIN vs. Arguments

A common pattern: if `--file` (or positional) isn’t provided, read from `stdin`.

```rust
use std::io::{self, Read};

fn read_all_from_stdin() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    Ok(s)
}
```

Combine with `clap` optional `file: Option<PathBuf>` to support both files and stdin.

---

## Quoting, Globbing & Shell Expansion

- **Globbing** (`*.txt`) is done by the **shell**, not Rust. Your program receives expanded paths.
- If you need a literal `*`, quote it: `'*'` or `"*"`.
- On Windows `cmd.exe` doesn’t expand globs by default; consider adding your own glob handling (e.g., `glob` crate).

---

## Cross‑Platform Notes (Windows, Linux, macOS)

- Use `args_os`/`OsString` and `PathBuf` for file paths to avoid Unicode issues.
- Newlines: text files may use `\n` (Unix) or `\r\n` (Windows). Use Rust’s `lines()` which handles both.
- Path separators: Use `Path` APIs, never hardcode `/` or `\`.
- Line buffering on stdout/stderr may differ; flush when needed (`use std::io::Write`).

---

## Mini Projects

1. **`uniq`‑like filter**: Read from stdin, print unique consecutive lines. Add `-c/--count` to prefix counts.
2. **`head`**: `head [-n N] <file>` with default `N=10`. Accept stdin if no file.
3. **`sum`**: `sum <ints...>` prints total; add `--avg` to compute mean.
4. **`grep`‑lite**: `grep-lite <pattern> [file]` with `-i` for case‑insensitive and `-n` to show line numbers.
5. **`json2csv`**: Read JSON lines from stdin, output CSV; flags for selected keys.

Each is perfect for practicing positional args, flags, options, and stdin handling.

---

## Further Tips

- Prefer **`clap`** for real apps; manual parsing is fine for tiny tools.
- Keep `main()` tiny—delegate to a `run()` that returns `Result`.
- Print friendly usage on arg errors; provide examples in `--help`.
- Log to `stderr` for diagnostics, write results to `stdout`.
- Use `anyhow` or `thiserror` for richer error handling in larger CLIs.
- Add CI tests with sample command invocations (use `assert_cmd` and `predicates` crates).

Happy hacking! 🦀
