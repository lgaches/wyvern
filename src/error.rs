//! Repository error types

use std::error::Error;
use std::fmt;

/// Standard repository error type.
#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    ConstraintViolation(String),
    ConnectionError(String),
    TransactionError(String),
    QueryError(String),
    InvalidInput(String),
    Internal(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Entity not found: {}", msg),
            Self::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Self::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
            Self::QueryError(msg) => write!(f, "Query error: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl Error for RepositoryError {}
