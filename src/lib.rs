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

pub mod error;
pub mod query;
pub mod repository;
pub mod transaction;

pub use error::RepositoryError;
pub use query::{
    Condition, ConditionValue, FilterCriteria, Operator, Page, Pagination, SortDirection, SortOrder,
};
pub use repository::{Queryable, Repository};
pub use transaction::Transactional;
