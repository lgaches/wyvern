# Repository Traits

A generic repository pattern implementation for Rust applications providing type-safe abstractions for data access layers.

## Features

- **Generic CRUD Operations**: Standard create, read, update, and delete operations
- **Advanced Querying**: Filtering, sorting, pagination, and counting
- **Transaction Support**: ACID-compliant transaction management
- **Async-first**: Built with async/await using `async-trait`
- **Database Agnostic**: Works with any database backend
- **Type Safe**: Leverages Rust's type system for compile-time safety

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wyvern = { git = "https://github.com/lgaches/wyvern",  branch = "main" }
async-trait = "0.1"
