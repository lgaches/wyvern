//! Database adapters for Wyvern
//!
//! This module provides adapters for various database libraries to work
//! seamlessly with Wyvern's repository traits.

#[cfg(feature = "sqlx")]
pub mod sqlx;

#[cfg(feature = "sqlx")]
pub use self::sqlx::{SqlxAdapter, WyvernSqlxExt};
