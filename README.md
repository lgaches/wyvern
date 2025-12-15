# Repository Traits

A generic repository pattern implementation for Rust applications providing type-safe abstractions for data access layers.

## Features

- **Generic CRUD Operations**: Standard create, read, update, and delete operations
- **Advanced Querying**: Filtering, sorting, pagination, and counting
- **Transaction Support**: ACID-compliant transaction management
- **Async-first**: Built with async/await using `async-trait`
- **Database Agnostic**: Works with any database backend
- **Type Safe**: Leverages Rust's type system for compile-time safety
- **SQLx Adapter** _(optional)_: Ready-to-use adapter for PostgreSQL via SQLx

## Installation

### Basic Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wyvern = { git = "https://github.com/lgaches/wyvern",  branch = "main" }
async-trait = "0.1"
```

### With SQLx Support

To use the built-in SQLx adapter for PostgreSQL:

```toml
[dependencies]
wyvern = { git = "https://github.com/lgaches/wyvern", branch = "main", features = ["sqlx"] }
async-trait = "0.1"
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio"] }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the SQLx Adapter

The SQLx adapter allows you to use Wyvern's `FilterCriteria` directly with your PostgreSQL database:

```rust
use wyvern::{FilterCriteria, Condition, SqlxAdapter, WyvernSqlxExt};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connect to database
    let pool = PgPool::connect("postgresql://user:pass@localhost/mydb").await?;
    
    // Build filter criteria
    let criteria = FilterCriteria::new()
        .with_condition(Condition::eq("status", "active".into()))
        .with_condition(Condition::gt("age", 18.into()))
        .with_limit(10);
    
    // Query using the extension trait
    let users: Vec<User> = pool
        .filter_entities("users", &criteria)
        .await?;
    
    // Or use the adapter directly
    let query = SqlxAdapter::build_select_query("users", &criteria);
    let users: Vec<User> = sqlx::query_as(&query)
        .fetch_all(&pool)
        .await?;
    
    Ok(())
}
```

### Supported Query Features

The SQLx adapter supports all of Wyvern's query features:

- **Filtering**: `Equal`, `NotEqual`, `GreaterThan`, `LessThan`, `Like`, `In`, `IsNull`, etc.
- **Sorting**: Ascending and descending order on multiple fields
- **Pagination**: `LIMIT` and `OFFSET`
- **Counting**: Count entities matching criteria

```rust
use wyvern::{FilterCriteria, Condition, SortOrder, Operator, ConditionValue};

let criteria = FilterCriteria::new()
    // Multiple conditions (combined with AND)
    .with_condition(Condition::eq("department", "engineering".into()))
    .with_condition(Condition::new("salary", Operator::GreaterThanOrEqual, ConditionValue::Integer(50000)))
    
    // Pattern matching
    .with_condition(Condition::new("email", Operator::Like, ConditionValue::String("%@company.com".into())))
    
    // IN clause
    .with_condition(Condition::new(
        "role",
        Operator::In,
        ConditionValue::List(vec![
            ConditionValue::String("admin".into()),
            ConditionValue::String("manager".into()),
        ])
    ))
    
    // Sorting
    .with_sort(SortOrder::desc("created_at"))
    .with_sort(SortOrder::asc("name"))
    
    // Pagination
    .with_limit(20)
    .with_offset(40);

// Execute query
let employees: Vec<Employee> = pool.filter_entities("employees", &criteria).await?;

// Or count matching records
let count: i64 = pool.count_entities("employees", &criteria).await?;
```
