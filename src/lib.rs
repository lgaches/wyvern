//! # Repository Traits
//!
//! A generic repository pattern implementation for Rust applications.
//!
//! This crate provides a set of traits and types that define common patterns
//! for data access layers, including:
//!
//! - **CRUD operations**: Basic create, read, update, and delete functionality
//! - **Querying**: Advanced filtering, sorting, and pagination
//! - **Transactions**: Support for transactional operations
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wyvern = { git = "https://github.com/lgaches/wyvern",  branch = "main" }
//! ```
//!
//! Then implement the traits for your repositories:
//!
//! ```rust,ignore
//! use wyvern::{Repository, Queryable};
//!
//! // Your implementation here
//! ```
//!
//! ## Optional Features
//!
//! - **sqlx**: Provides adapters for SQLx with PostgreSQL support
//!
//! ```toml
//! [dependencies]
//! wyvern = { git = "https://github.com/lgaches/wyvern", branch = "main", features = ["sqlx"] }
//! ```
//!
//! Example usage with SQLx:
//!
//! ```rust,ignore
//! use wyvern::{FilterCriteria, Condition, SqlxAdapter, WyvernSqlxExt};
//! use sqlx::PgPool;
//!
//! let pool = PgPool::connect("postgresql://...").await?;
//! let criteria = FilterCriteria::new()
//!     .with_condition(Condition::eq("status", "active".into()));
//!
//! let users: Vec<User> = pool.filter_entities("users", &criteria).await?;
//! ```

pub mod error;
pub mod query;
pub mod repository;
pub mod transaction;

#[cfg(feature = "sqlx")]
pub mod adapters;

pub use error::RepositoryError;
pub use query::{
    Condition, ConditionValue, FilterCriteria, Operator, Page, Pagination, SortDirection, SortOrder,
};
pub use repository::{Queryable, Repository};
pub use transaction::Transactional;

#[cfg(feature = "sqlx")]
pub use adapters::{SqlxAdapter, WyvernSqlxExt};
