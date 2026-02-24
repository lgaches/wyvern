//! SQLx Adapter Example
//!
//! This example demonstrates how to use Wyvern's SQLx adapter to query
//! a PostgreSQL database with type-safe filters.
//!
//! Prerequisites:
//! - PostgreSQL running locally
//! - A database with a users table
//!
//! To run this example:
//! ```bash
//! cargo run --example sqlx_example --features sqlx
//! ```

use sqlx::FromRow;
use sqlx::PgPool;
use wyvern::{
    Condition, ConditionValue, FilterCriteria, Operator, SortOrder, SqlxAdapter, WyvernSqlxExt,
};

#[allow(dead_code)]
#[derive(Debug, FromRow)]
struct User {
    id: i32,
    email: String,
    name: String,
    age: i32,
    active: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database URL - in production, use environment variables
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/wyvern_example".to_string());

    println!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;

    // Create example table (for demonstration purposes)
    setup_database(&pool).await?;

    println!("\n=== Example 1: Simple Equality Filter ===");
    example_simple_filter(&pool).await?;

    println!("\n=== Example 2: Multiple Conditions ===");
    example_multiple_conditions(&pool).await?;

    println!("\n=== Example 3: Sorting and Pagination ===");
    example_sorting_pagination(&pool).await?;

    println!("\n=== Example 4: Pattern Matching with LIKE ===");
    example_like_operator(&pool).await?;

    println!("\n=== Example 5: IN Operator ===");
    example_in_operator(&pool).await?;

    println!("\n=== Example 6: Counting Records ===");
    example_count(&pool).await?;

    println!("\n=== Example 7: Using SqlxAdapter Directly ===");
    example_direct_adapter(&pool).await?;

    // Cleanup
    cleanup_database(&pool).await?;
    pool.close().await;

    Ok(())
}

async fn example_simple_filter(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new().with_condition(Condition::eq("active", true.into()));

    let users: Vec<User> = pool.filter_entities("users", &criteria).await?;

    println!("Found {} active users:", users.len());
    for user in users {
        println!("  - {} ({})", user.name, user.email);
    }

    Ok(())
}

async fn example_multiple_conditions(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new()
        .with_condition(Condition::eq("active", true.into()))
        .with_condition(Condition::new(
            "age",
            Operator::GreaterThanOrEqual,
            ConditionValue::Integer(25),
        ));

    let users: Vec<User> = pool.filter_entities("users", &criteria).await?;

    println!("Active users aged 25 or older:");
    for user in users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    Ok(())
}

async fn example_sorting_pagination(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new()
        .with_sort(SortOrder::desc("age"))
        .with_sort(SortOrder::asc("name"))
        .with_limit(3)
        .with_offset(0);

    let users: Vec<User> = pool.filter_entities("users", &criteria).await?;

    println!("First 3 users (sorted by age desc, name asc):");
    for user in users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    Ok(())
}

async fn example_like_operator(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new().with_condition(Condition::new(
        "email",
        Operator::Like,
        ConditionValue::String("%@example.com".to_string()),
    ));

    let users: Vec<User> = pool.filter_entities("users", &criteria).await?;

    println!("Users with @example.com emails:");
    for user in users {
        println!("  - {} ({})", user.name, user.email);
    }

    Ok(())
}

async fn example_in_operator(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new().with_condition(Condition::new(
        "name",
        Operator::In,
        ConditionValue::List(vec![
            ConditionValue::String("Alice".to_string()),
            ConditionValue::String("Bob".to_string()),
        ]),
    ));

    let users: Vec<User> = pool.filter_entities("users", &criteria).await?;

    println!("Users named Alice or Bob:");
    for user in users {
        println!("  - {} ({})", user.name, user.email);
    }

    Ok(())
}

async fn example_count(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new().with_condition(Condition::eq("active", true.into()));

    let count: i64 = pool.count_entities("users", &criteria).await?;

    println!("Total active users: {}", count);

    Ok(())
}

async fn example_direct_adapter(pool: &PgPool) -> Result<(), sqlx::Error> {
    let criteria = FilterCriteria::new()
        .with_condition(Condition::gt("age", 30.into()))
        .with_sort(SortOrder::asc("name"))
        .with_limit(5);

    // Build the query manually using SqlxAdapter
    let query = SqlxAdapter::build_select_query("users", &criteria);
    println!("Generated SQL: {}", query);

    // Execute it
    let users: Vec<User> = sqlx::query_as(&query).fetch_all(pool).await?;

    println!("\nUsers over 30 (using direct adapter):");
    for user in users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    Ok(())
}

// Helper functions for setup and cleanup

async fn setup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Drop table if exists
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await?;

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL,
            age INTEGER NOT NULL,
            active BOOLEAN NOT NULL DEFAULT TRUE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Insert sample data
    let sample_users = vec![
        ("alice@example.com", "Alice", 28, true),
        ("bob@example.com", "Bob", 35, true),
        ("charlie@test.com", "Charlie", 22, false),
        ("diana@example.com", "Diana", 31, true),
        ("eve@test.com", "Eve", 45, true),
        ("frank@example.com", "Frank", 19, false),
    ];

    for (email, name, age, active) in sample_users {
        sqlx::query("INSERT INTO users (email, name, age, active) VALUES ($1, $2, $3, $4)")
            .bind(email)
            .bind(name)
            .bind(age)
            .bind(active)
            .execute(pool)
            .await?;
    }

    println!("Database setup complete with sample data");

    Ok(())
}

async fn cleanup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await?;

    println!("\nDatabase cleanup complete");

    Ok(())
}
