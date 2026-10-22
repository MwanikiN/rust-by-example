### This will encompass all rust examples as detailed in the rust-by-example.html.

#### check out the rust-by-example for better understanding and guidance.

[rust-by-example](https://share.google/9V25vMLT4sQee7sP9)

# rust-by-example

A collection of Rust examples and practical implementations covering core language features, abstraction patterns, memory management, and fundamental data structures. Each example focuses on a specific concept and provides a compact reference implementation that can be studied, executed, and extended.

## Contents

- [Repository Structure](#repository-structure)
- [Topics](#topics)
- [Prerequisites](#prerequisites)
- [Running Examples](#running-examples)
- [Code Organization](#code-organization)
- [Design Notes](#design-notes)

## Repository Structure

```text
rust-by-example/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   └── bin/
│       ├── traits/
│       ├── generics/
│       ├── trait-objects/
│       ├── ownership/
│       ├── type-conversions/
│       └── data-structures/
└── lib/
    ├── printable.rs
    ├── notify.rs
    ├── draw.rs
    ├── shape.rs
    └── ...
```

The directory names above illustrate the organization; the actual folders and filenames in the repository define the available examples.

- **`src/bin/`** — executable examples grouped into topic-specific subdirectories. Each binary demonstrates a focused concept or implementation.
- **`lib/`** — reusable modules containing trait definitions, trait implementations, and related types used by the examples.
- **`Cargo.toml`** — package metadata, build configuration, and dependencies.
- **`Cargo.lock`** — records resolved dependency versions for reproducible builds of the package and its binaries.

## Topics

### Traits and Abstraction

- Defining traits and implementing shared behaviour.
- Generic functions constrained by trait bounds.
- Trait objects using `dyn Trait`.
- Dynamic dispatch and heterogeneous collections.
- Polymorphic interfaces for notification systems and GUI components.

### Generics and Type Safety

- Generic structs and methods.
- Newtype wrappers for domain-specific types.
- Type conversions through `From` and `Into`.
- Generic containers and reusable data structures.

### Ownership and Resource Management

- Ownership, borrowing, and mutable references.
- Scope-based destruction.
- Implementing `Drop` for custom cleanup behaviour.
- Heap allocation and `Box<T>`.

### Data Structures

- Stack implementation using `Vec<T>`.
- Last In, First Out (LIFO) operations.
- Push, pop, and top/peek operations.
- Empty-stack handling with `Option<T>`.

### Applied Examples

- Celsius-to-Fahrenheit conversion.
- GUI component rendering through a shared trait.
- Shape area calculation using trait objects.
- Generic smart-box implementation with automatic cleanup.

## Prerequisites

Install the stable Rust toolchain and Cargo using [rustup](https://www.rust-lang.org/tools/install).

Verify the installation:

```bash
rustc --version
cargo --version
```

## Running Examples

Run commands from the repository root.

### List available binaries

```bash
cargo run --bin <binary-name>
```

Cargo uses the name of the binary target, normally derived from its source filename. For example, `src/bin/shape_area.rs` typically corresponds to:

```bash
cargo run --bin shape_area
```

Use the actual binary names defined by the project.

### Run with standard input

Examples that read from standard input can receive values interactively or through a pipe:

```bash
printf "2.0\n3.0\n" | cargo run --bin shape_area
```

### Build all targets

```bash
cargo build --bins
```

### Run tests

```bash
cargo test
```

### Check compilation without producing executables

```bash
cargo check --all-targets
```

## Code Organization

### Executable examples

Files under `src/bin/` are binary crate roots. Each executable has its own `main()` function and can be run independently through Cargo.

Grouping binaries into subdirectories keeps related examples together while preserving separate entry points.

### Reusable modules

The `lib/` directory contains reusable Rust source files for shared traits, types, and implementations. A module can be reused by a binary when it is exposed through a Cargo library target or included through an appropriate module declaration.

For conventional Cargo library integration, shared modules are commonly placed under `src/` and exposed through `src/lib.rs`. If the repository keeps modules in a root-level `lib/` directory, ensure `Cargo.toml` and the module declarations configure that layout explicitly.

For example, a shared trait might define the interface:

```rust
pub trait Shape {
    fn area(&self) -> f64;
}
```

A separate module can implement that interface for concrete types:

```rust
use crate::shape::Shape;

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
```

This separation keeps interface definitions independent of concrete implementations and allows multiple examples to reuse the same abstraction.

## Design Notes

- **Single responsibility:** Each executable focuses on one primary concept or behaviour.
- **Explicit interfaces:** Traits define the operations available across different types.
- **Reusable implementations:** Shared definitions are separated from executable entry points where practical.
- **Type safety:** Newtypes and generic constraints make invalid combinations harder to express.
- **Idiomatic error handling:** `Option` and `Result` represent absent values and recoverable failures.
- **Minimal dependencies:** Prefer the Rust standard library for examples unless an external crate provides a clear benefit.

## Contributing

When adding an example:

1. Place it in the relevant topic directory under `src/bin/`.
2. Put reusable traits or implementations in the appropriate shared module under `lib/`.
3. Use descriptive filenames and standard Rust naming conventions.
4. Include representative input/output examples where applicable.
5. Add tests for non-trivial behaviour and edge cases.

---

**Project:** `rust-by-example`  
**Language:** Rust  
**Build system:** Cargo  
**Status:** Ongoing collection of technical examples