# Rust Programming Language - Comprehensive Notes & Documentation Guide

> **"A language empowering everyone to build reliable and efficient software."**  
> Official Rust Book | Rust by Example | Rust Reference

**Last Updated:** May 2025  
**Version Coverage:** Rust 1.85+ (Edition 2021/2024)

---

## Table of Contents

- [Introduction](#introduction)
- [Installation & Setup](#installation--setup)
- [Basic Syntax & Hello World](#basic-syntax--hello-world)
- [Data Types](#data-types)
- [Ownership, Borrowing & Lifetimes](#ownership-borrowing-lifetimes)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Structs, Enums & Pattern Matching](#structs-enums--pattern-matching)
- [Modules, Crates & Packages](#modules-crates--packages)
- [Collections](#collections)
- [Error Handling](#error-handling)
- [Generics, Traits & Advanced Types](#generics-traits--advanced-types)
- [Concurrency & Async](#concurrency--async)
- [Testing](#testing)
- [Cargo & Project Management](#cargo--project-management)
- [Common Patterns & Idioms](#common-patterns--idioms)
- [Performance & Unsafe Rust](#performance--unsafe-rust)
- [Useful Crates & Ecosystem](#useful-crates--ecosystem)
- [Learning Resources & Official Docs](#learning-resources--official-docs)

---

## Introduction

Rust is a modern systems programming language that focuses on **performance**, **memory safety** without garbage collection, and **thread safety**.

### Key Features
- **Ownership** system prevents data races and memory bugs at compile time
- Zero-cost abstractions
- Fearless concurrency
- Expressive type system with traits and generics
- Great tooling (`cargo`, `rust-analyzer`, `clippy`)
- Growing ecosystem (web, embedded, CLI, WASM, blockchain, etc.)

**Editions**: 2015, 2018, 2021, **2024** (latest stable improvements)

---

## Installation & Setup

```bash
# Official installer (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update toolchain
rustup update

# Check installation
rustc --version
cargo --version
```
## VS Code + rust-analyzer is the most popular setup.

## Basic Syntax & Hello World
```bash
Rust// main.rs
fn main() {
    println!("Hello, Rust!");           // macro
    eprintln!("Error message");         // to stderr
    println!("{} + {} = {}", 2, 3, 5);  // formatting
}
```
## Variables (immutable by default):
```bash
Rustlet x = 5;           // immutable
let mut y = 10;      // mutable
const MAX_POINTS: u32 = 100_000;  // constant
Shadowing is allowed:
Rustlet x = 5;
let x = x + 1;       // new x
let x = "now a string";
```
## Data Types
### Scalar Types

```bash Integers: i8, i16, i32 (default), i64, i128, u8...u128, isize, usize
Floating point: f32, f64 (default)
Boolean: bool
Character: char (Unicode scalar value)
```
```
Compound Types
Rustlet tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
```
```
let arr = [1, 2, 3, 4, 5];        // fixed size
let arr2 = [3; 5];                // [3,3,3,3,3]
```
## Ownership, Borrowing & Lifetimes
### Core concept of Rust.
### Ownership Rules

### Each value has an owner.
### There can only be one owner at a time.
### When owner goes out of scope, value is dropped.
```
Rustlet s1 = String::from("hello");
let s2 = s1;        // s1 moved to s2 (not copied)
println!("{}", s1); // ERROR
Borrowing
Rustfn calculate_length(s: &String) -> usize {  // immutable borrow
    s.len()
}

fn change(s: &mut String) {                 // mutable borrow
    s.push_str(", world");
}
```
Rules:

At any time, you can have one mutable reference OR any number of immutable references.
References must always be valid (lifetimes).

Lifetimes
Rustfn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

Control Flow
Rustif condition { } else if { } else { }

// Expression
let number = if condition { 5 } else { 6 };

// Loops
loop { break; }
while condition { }
for element in arr { }

// Range
for i in 0..5 { }        // exclusive
for i in 0..=5 { }       // inclusive
Match (very powerful):
Rustmatch value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=10 => println!("range"),
    _ => println!("other"),
}

Functions
Rustfn add(x: i32, y: i32) -> i32 {
    x + y          // no semicolon = return value
}

fn never_returns() -> ! {  // diverging function
    panic!("error");
}

Structs, Enums & Pattern Matching
Structs
Rust#[derive(Debug)]
struct User {
    username: String,
    active: bool,
}

let user = User { username: String::from("ayush"), active: true };

// Tuple struct
struct Color(i32, i32, i32);
Enums
Rustenum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
Option and Result are enums:
Rustenum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }

Modules, Crates & Packages
Crate = compilation unit (binary or library).
Rust// lib.rs or main.rs
mod front_of_house;           // from front_of_house.rs

pub mod hosting {             // public module
    pub fn add_to_waitlist() {}
}
Use keyword:
Rustuse crate::front_of_house::hosting;
use std::collections::HashMap;

Collections

Vec<T>
String
HashMap<K, V>
HashSet<T>

Rustlet mut v = vec![1, 2, 3];
v.push(4);

let mut map = HashMap::new();
map.insert("key", "value");

Error Handling
Recoverable Errors (Result)
Rustfn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")
}

// Propagation
let content = read_file()?;   // ? operator
Unrecoverable (panic!)
Rustpanic!("crash and burn");
Custom Error Types via thiserror crate (recommended).

Generics, Traits & Advanced Types
Rustfn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

trait Summary {
    fn summarize(&self) -> String;
}

impl<T: Display> Summary for T { ... }   // blanket impl
Associated Types, Supertraits, Trait Objects (dyn Trait).

Concurrency & Async
Threads
Rustuse std::thread;

let handle = thread::spawn(|| {
    // child thread
});

handle.join().unwrap();
Channels (mpsc), Mutex, Arc.
Async / Tokio (most popular)
Rust#[tokio::main]
async fn main() {
    let result = some_async_fn().await;
}

Testing
Rust#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_panics() { ... }
}
cargo test, cargo test -- --test-threads=1

Cargo & Project Management
Bashcargo new myproject
cargo build
cargo run
cargo check          # fast
cargo clippy         # linting
cargo fmt            # formatting
cargo doc --open     # generate docs
cargo publish
Cargo.toml key sections: [dependencies], [dev-dependencies], [[bin]], [lib]

Common Patterns & Idioms

Builder Pattern
Newtype Pattern
RAII (Resource Acquisition Is Initialization)
Interior Mutability (Cell, RefCell, Mutex)
Zero-Cost abstractions


Performance & Unsafe Rust
Rust is blazing fast — comparable to C/C++.
unsafe blocks allow:

Dereferencing raw pointers
Calling unsafe functions
Implementing unsafe traits

Use sparingly and with caution.

Useful Crates & Ecosystem

Web: axum, actix-web, rocket
Async: tokio, async-std
CLI: clap, structopt
Data: serde, polars, arrow
WASM: wasm-bindgen, trunk
Embedded: embedded-hal
Testing: rstest, proptest
Error: thiserror, anyhow


Learning Resources & Official Docs
Primary Resources

The Rust Programming Language Book (Must read)
Rust by Example
Rust Reference
The Rust Standard Library
Async Book
Rustonomicon (The Dark Arts of Unsafe Rust)

Communities

users.rust-lang.org
Discord
Reddit: r/rust, r/learnrust


Happy Rusting! 🦀
Made with ❤️ for learning Rust effectively.

Contribution: Feel free to fork and improve these notes.
