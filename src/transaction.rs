//! Transaction management traits

use async_trait::async_trait;
use std::error::Error;

/// Trait for repositories that support transactional operations.
#[async_trait]
pub trait Transactional: Send + Sync {
    /// The transaction type used by this repository
    type Transaction: Send;

    /// The error type returned by transaction operations
    type Error: Error + Send + Sync;

    /// Begins a new transaction.
    async fn begin_transaction(&self) -> Result<Self::Transaction, Self::Error>;

    /// Commits the given transaction.
    async fn commit_transaction(&self, transaction: Self::Transaction) -> Result<(), Self::Error>;

    /// Rolls back the given transaction.
    async fn rollback_transaction(&self, transaction: Self::Transaction)
    -> Result<(), Self::Error>;
}
