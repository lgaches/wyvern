//! Core repository traits

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
pub trait Repository<T>: Send + Sync {
    /// The type used to identify entities (typically i64 or Uuid)
    type Id: Send + Sync;

    /// The error type returned by repository operations
    type Error: Error + Send + Sync;

    /// Creates a new entity in the repository.
    fn create(&self, entity: T) -> impl Future<Output = Result<T, Self::Error>> + Send;

    /// Finds an entity by its unique identifier.
    fn find_by_id(
        &self,
        id: Self::Id,
    ) -> impl Future<Output = Result<Option<T>, Self::Error>> + Send;

    /// Updates an existing entity in the repository.
    fn update(&self, entity: T) -> impl Future<Output = Result<T, Self::Error>> + Send;

    /// Deletes an entity by its unique identifier.
    fn delete(&self, id: Self::Id) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    /// Retrieves all entities from the repository.
    ///
    /// # Warning
    ///
    /// This method can return a large amount of data. Consider using
    /// pagination or filtering for production use.
    fn find_all(&self) -> impl Future<Output = Result<Vec<T>, Self::Error>> + Send;
}

/// Trait for repositories that support advanced querying capabilities.
///
/// This trait extends basic CRUD operations with filtering, sorting,
/// pagination, and counting capabilities.
pub trait Queryable<T>: Repository<T> {
    /// Executes a query with the given criteria.
    fn filter(
        &self,
        criteria: FilterCriteria,
    ) -> impl Future<Output = Result<Vec<T>, <Self as Repository<T>>::Error>> + Send;

    /// Counts entities matching the given criteria.
    fn count(
        &self,
        criteria: FilterCriteria,
    ) -> impl Future<Output = Result<i64, <Self as Repository<T>>::Error>> + Send;

    /// Executes a paginated query.
    fn paginate(
        &self,
        criteria: FilterCriteria,
        pagination: Pagination,
    ) -> impl Future<Output = Result<Page<T>, <Self as Repository<T>>::Error>> + Send;

    /// Checks if any entities match the given criteria.
    fn exists(
        &self,
        criteria: FilterCriteria,
    ) -> impl Future<Output = Result<bool, <Self as Repository<T>>::Error>> + Send;
}
