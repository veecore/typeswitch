# TypeSwitch for Rust

A powerful, Go-inspired macro to perform clean, declarative runtime type switching on `dyn Any` trait objects.

## 💡 The Inspiration

In **Go**, the type switch is a staple of the language:

```go
switch v := i.(type) {
case int:
    fmt.Println("Integer:", v)
case string:
    fmt.Println("String:", v)
default:
    fmt.Println("Unknown")
}

```

In standard Rust, downcasting `dyn Any` usually requires a tedious chain of `if-let` blocks. `typeswitch` brings that ergonomic Go-style syntax to Rust while respecting Rust's strict rules on ownership, mutability, and borrowing.

## 🚀 Features

* **Clean Syntax**: No more `if let Some(x) = var.downcast_ref::<Type>()` boilerplate.
* **Go-Style Binding**: Automatically bind the downcasted value to a variable for all branches using the `as` keyword.
* **Mutability Control**: Easily switch between immutable (`&T`) and mutable (`&mut T`) access.
* **Owned Consumption**: Move values directly out of a `Box<dyn Any>`.
* **Or-Patterns**: Match multiple types in a single branch (e.g., `i32 | i64 => ...`).

## 🛠 Usage

### 1. Automatic Binding (The "Go" Way)

By using `v as subject`, the identifier `v` is automatically bound to the concrete type in every branch.

```rust
use typeswitch::typeswitch;
use std::any::Any;

let x: Box<dyn Any> = Box::new(42i32);

typeswitch!(v as x {
    i32    => println!("It's an i32: {}", v + 1),
    String => println!("It's a string: {}", v.as_str()),
    _      => println!("Unknown type"),
});

```

### 2. Mutable Switching

Prefix the subject with `mut` to get mutable references in your branches.

```rust
let mut x: Box<dyn Any> = Box::new(100i32);

typeswitch!(mut v as x {
    i32 => { *v += 1; },
    _   => {},
});

```

### 3. Owned Consumption

Need the actual value? Use the `box` keyword. This consumes the `Box` if the type matches.

```rust
let x: Box<dyn Any> = Box::new(String::from("Ownership!"));

typeswitch! { x {
    box s: String => println!("Consumed string: {}", s),
    _ => println!("Not a string, box is still alive here."),
}}

```

### 4. Custom Bindings and Or-Patterns

You can define specific variable names for each arm and match multiple types.

```rust
typeswitch! { x {
    i32 | i64 => println!("Some integer"),
    s: String      => println!("String: {}", s),
    _              => println!("Fallback"),
}}

```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typeswitch = "0.1.0"

```

## ⚖️ License

Licensed under either of [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT license](https://opensource.org/licenses/MIT) at your option.