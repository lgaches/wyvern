//! Core repository traits

use async_trait::async_trait;
use std::error::Error;

use crate::query::{FilterCriteria, Page, Pagination};

/// Base repository trait providing standard CRUD operations.
///
/// This trait defines the fundamental operations that all repositories should support:
/// - Create: Insert a new entity
/// - Read: Retrieve entities by ID or list all
/// - Update: Modify an existing entity
/// - Delete: Remove an entity
///
/// # Type Parameters
///
/// * `T` - The entity type this repository manages
#[async_trait]
pub trait Repository<T>: Send + Sync {
    /// The type used to identify entities (typically i64 or Uuid)
    type Id: Send + Sync;

    /// The error type returned by repository operations
    type Error: Error + Send + Sync;

    /// Creates a new entity in the repository.
    async fn create(&self, entity: T) -> Result<T, Self::Error>;

    /// Finds an entity by its unique identifier.
    async fn find_by_id(&self, id: Self::Id) -> Result<Option<T>, Self::Error>;

    /// Updates an existing entity in the repository.
    async fn update(&self, entity: T) -> Result<T, Self::Error>;

    /// Deletes an entity by its unique identifier.
    async fn delete(&self, id: Self::Id) -> Result<bool, Self::Error>;

    /// Retrieves all entities from the repository.
    ///
    /// # Warning
    ///
    /// This method can return a large amount of data. Consider using
    /// pagination or filtering for production use.
    async fn find_all(&self) -> Result<Vec<T>, Self::Error>;
}

/// Trait for repositories that support advanced querying capabilities.
///
/// This trait extends basic CRUD operations with filtering, sorting,
/// pagination, and counting capabilities.
#[async_trait]
pub trait Queryable<T>: Repository<T> {
    /// Executes a query with the given criteria.
    async fn filter(
        &self,
        criteria: FilterCriteria,
    ) -> Result<Vec<T>, <Self as Repository<T>>::Error>;

    /// Counts entities matching the given criteria.
    async fn count(&self, criteria: FilterCriteria) -> Result<i64, <Self as Repository<T>>::Error>;

    /// Executes a paginated query.
    async fn paginate(
        &self,
        criteria: FilterCriteria,
        pagination: Pagination,
    ) -> Result<Page<T>, <Self as Repository<T>>::Error>;

    /// Checks if any entities match the given criteria.
    async fn exists(
        &self,
        criteria: FilterCriteria,
    ) -> Result<bool, <Self as Repository<T>>::Error>;
}
