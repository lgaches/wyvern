//! Transaction management traits

use std::error::Error;

/// Trait for repositories that support transactional operations.
pub trait Transactional: Send + Sync {
    /// The transaction type used by this repository
    type Transaction: Send;

    /// The error type returned by transaction operations
    type Error: Error + Send + Sync;

    /// Begins a new transaction.
    fn begin_transaction(
        &self,
    ) -> impl Future<Output = Result<Self::Transaction, Self::Error>> + Send;

    /// Commits the given transaction.
    fn commit_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Rolls back the given transaction.
    fn rollback_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
