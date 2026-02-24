# SQLx Adapter Integration Guide

This document describes the integration of the SQLx adapter into the Wyvern library as an optional feature.

## Overview

The SQLx adapter provides a bridge between Wyvern's generic `FilterCriteria` and SQLx's PostgreSQL queries. It converts Wyvern's domain-specific query language into SQL queries that can be executed via SQLx.

## Architecture

### Module Structure

```
wyvern/
├── src/
│   ├── adapters/
│   │   ├── mod.rs          # Adapters module (conditional)
│   │   └── sqlx.rs         # SQLx adapter implementation
│   ├── error.rs
│   ├── query.rs            # FilterCriteria, Condition, etc.
│   ├── repository.rs       # Core traits
│   ├── transaction.rs
│   └── lib.rs
└── examples/
    └── sqlx_example.rs     # Usage example
```

### Key Components

1. **`SqlxAdapter`**: A utility struct that converts `FilterCriteria` to SQL strings
   - `build_select_query()`: Generates SELECT queries with WHERE, ORDER BY, LIMIT, OFFSET
   - `build_count_query()`: Generates COUNT queries
   - `format_value()`: Safely formats condition values for SQL (with proper escaping)

2. **`WyvernSqlxExt`**: An async trait extending `PgPool` with convenient methods
   - `filter_entities()`: Execute filter and return typed results
   - `count_entities()`: Count records matching criteria

## Feature Flag

The SQLx adapter is behind the `sqlx` feature flag to keep the core library lightweight.

### Cargo.toml Configuration

```toml
[features]
default = []
sqlx = ["dep:sqlx"]

[dependencies]
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio"], optional = true }
```

### Conditional Compilation

The adapter module is only compiled when the `sqlx` feature is enabled:

```rust
#[cfg(feature = "sqlx")]
pub mod adapters;

#[cfg(feature = "sqlx")]
pub use adapters::{SqlxAdapter, WyvernSqlxExt};
```

## Usage

### Basic Usage with Extension Trait

```rust
use wyvern::{FilterCriteria, Condition, WyvernSqlxExt};
use sqlx::PgPool;

let pool = PgPool::connect("postgresql://...").await?;

let criteria = FilterCriteria::new()
    .with_condition(Condition::eq("status", "active".into()))
    .with_limit(10);

let users: Vec<User> = pool.filter_entities("users", &criteria).await?;
let count: i64 = pool.count_entities("users", &criteria).await?;
```

### Direct Adapter Usage

```rust
use wyvern::{FilterCriteria, SqlxAdapter};

let criteria = FilterCriteria::new()
    .with_condition(Condition::gt("age", 18.into()));

let query = SqlxAdapter::build_select_query("users", &criteria);
// Returns: "SELECT * FROM users WHERE age > 18"

let users: Vec<User> = sqlx::query_as(&query).fetch_all(&pool).await?;
```

## Supported SQL Features

### Operators

| Wyvern Operator | SQL Translation |
|----------------|-----------------|
| `Equal` | `=` |
| `NotEqual` | `!=` |
| `GreaterThan` | `>` |
| `GreaterThanOrEqual` | `>=` |
| `LessThan` | `<` |
| `LessThanOrEqual` | `<=` |
| `Like` | `ILIKE` (case-insensitive) |
| `IsNull` | `IS NULL` |
| `IsNotNull` | `IS NOT NULL` |
| `In` | `IN (...)` |

### Query Clauses

- **WHERE**: Multiple conditions combined with AND
- **ORDER BY**: Multiple sort fields with ASC/DESC
- **LIMIT**: Result set size limiting
- **OFFSET**: Result set pagination

### Value Types

- `String`: Single-quoted with proper escaping (`'` → `''`)
- `Integer`: Direct numeric value
- `Float`: Direct numeric value
- `Boolean`: `TRUE` or `FALSE`
- `Null`: `NULL`
- `List`: Comma-separated values in parentheses

## Security Considerations

### SQL Injection Prevention

The adapter uses PostgreSQL's standard string escaping:

```rust
fn format_value(value: &ConditionValue) -> String {
    match value {
        ConditionValue::String(s) => {
            // Escape single quotes by doubling them
            let escaped = s.replace("'", "''");
            format!("'{}'", escaped)
        }
        // ... other types
    }
}
```

This approach is safe for PostgreSQL but requires careful implementation. All string values have single quotes escaped according to SQL standard.

### Field Names

**Important**: Field names are NOT escaped in the current implementation. They should be validated at the application level to prevent SQL injection. Future versions may add identifier quoting.

## Testing

The adapter includes comprehensive unit tests:

```bash
# Run all tests including SQLx adapter tests
cargo test --all-features

# Run only SQLx adapter tests
cargo test --features sqlx adapters::sqlx

# Run the example
cargo run --example sqlx_example --features sqlx
```

### Test Coverage

- Simple equality filters
- Multiple conditions (AND)
- Sorting (single and multiple fields)
- Pagination (LIMIT/OFFSET)
- NULL checks
- String escaping
- Pattern matching (LIKE/ILIKE)
- IN operator with lists
- COUNT queries

## Examples

A complete example is available in `examples/sqlx_example.rs`:

```bash
# Set up a PostgreSQL database first
createdb wyvern_example

# Run the example
DATABASE_URL=postgresql://localhost/wyvern_example \
  cargo run --example sqlx_example --features sqlx
```

## Extending the Adapter

### Adding Support for Other Databases

The current implementation is PostgreSQL-specific. To add support for other databases:

1. Create a new module (e.g., `src/adapters/mysql.rs`)
2. Implement database-specific query building
3. Add appropriate feature flags

Example:

```toml
[features]
sqlx-postgres = ["dep:sqlx"]
sqlx-mysql = ["dep:sqlx"]
```

### Adding Custom Query Methods

You can extend `WyvernSqlxExt` with additional methods:

```rust
pub trait WyvernSqlxExt {
    // ... existing methods ...
    
    fn find_one<T>(
        &self,
        table_name: &str,
        criteria: &FilterCriteria,
    ) -> impl std::future::Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send;
}
```

## Future Enhancements

Potential improvements for future versions:

1. **Parameterized Queries**: Use SQLx's parameter binding instead of string interpolation
2. **Query Builder API**: More fluent interface for complex queries
3. **JOIN Support**: Add support for table joins
4. **Subquery Support**: Enable nested queries
5. **Transaction Integration**: Integrate with Wyvern's `Transactional` trait
6. **OR Logic**: Support OR conditions in addition to AND
7. **Field Name Escaping**: Properly quote identifiers to prevent injection
8. **Other Databases**: MySQL, SQLite, MSSQL adapters
9. **Async Streaming**: Support for `fetch()` instead of `fetch_all()`
10. **Custom Type Mapping**: Allow custom type conversions

## Migration Guide

If you have existing code using the standalone `wyvern_sqlx_adapter.rs`:

1. Update dependencies:
   ```toml
   wyvern = { git = "...", features = ["sqlx"] }
   ```

2. Update imports:
   ```rust
   // Old
   use crate::wyvern_sqlx_adapter::{SqlxAdapter, WyvernSqlxExt};
   
   // New
   use wyvern::{SqlxAdapter, WyvernSqlxExt};
   ```

3. No code changes needed - the API remains identical

## Troubleshooting

### Feature Not Found

```
error[E0432]: unresolved import `wyvern::SqlxAdapter`
```

**Solution**: Enable the `sqlx` feature:
```toml
wyvern = { git = "...", features = ["sqlx"] }
```

### Type Mismatch

```
the trait `FromRow<'_, PgRow>` is not implemented for `MyType`
```

**Solution**: Derive `FromRow` on your struct:
```rust
#[derive(sqlx::FromRow)]
struct MyType {
    // fields matching your table columns
}
```

### SQL Injection Concerns

While the adapter escapes string values, be cautious with:
- Dynamic table names (validate/whitelist)
- Field names in conditions (validate/whitelist)
- User-provided raw SQL fragments (avoid entirely)

## Contributing

To contribute to the SQLx adapter:

1. Ensure all tests pass: `cargo test --all-features`
2. Add tests for new functionality
3. Update documentation
4. Consider backward compatibility
5. Follow Rust API guidelines

## License

The SQLx adapter is part of Wyvern and shares the same MIT license.