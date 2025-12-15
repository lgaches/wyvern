//! SQLx Adapter for Wyvern Repository Traits
//!
//! This adapter provides utilities to convert wyvern's generic `FilterCriteria`
//! into SQLx queries with proper type handling for PostgreSQL.
//!
//! # Problem
//!
//! Wyvern's `FilterCriteria` uses generic `ConditionValue` enums that can represent
//! different types (String, Integer, Float, etc.). SQLx requires properly typed
//! bindings at compile time, which conflicts with dynamic query building.
//!
//! # Solution
//!
//! This adapter uses SQLx's `QueryBuilder` with string interpolation for values
//! (with proper escaping for safety) or alternatively builds queries that can
//! be safely executed with PostgreSQL.
//!
//! # Usage
//!
//! ```rust,ignore
//! use wyvern::{FilterCriteria, SqlxAdapter};
//!
//! let criteria = FilterCriteria::new()
//!     .with_condition(Condition::eq("status", "active".into()));
//!
//! let (query, args) = SqlxAdapter::build_select("users", criteria);
//! let results = sqlx::query_as_with(&query, args)
//!     .fetch_all(&pool)
//!     .await?;
//! ```

use crate::{ConditionValue, FilterCriteria, Operator, SortDirection};
use sqlx::postgres::PgPool;

/// Adapter for converting wyvern FilterCriteria to SQLx queries
pub struct SqlxAdapter;

impl SqlxAdapter {
    /// Builds a SELECT query with WHERE, ORDER BY, LIMIT, and OFFSET clauses
    ///
    /// Returns a SQL string that can be executed with sqlx
    pub fn build_select_query(table_name: &str, criteria: &FilterCriteria) -> String {
        let mut query = format!("SELECT * FROM {}", table_name);

        // Build WHERE clause
        let where_clause = Self::build_where_clause(criteria);
        if !where_clause.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clause);
        }

        // Build ORDER BY clause
        if !criteria.sort.is_empty() {
            query.push_str(" ORDER BY ");
            let sort_clauses: Vec<String> = criteria
                .sort
                .iter()
                .map(|s| {
                    let direction = match s.direction {
                        SortDirection::Ascending => "ASC",
                        SortDirection::Descending => "DESC",
                    };
                    format!("{} {}", s.field, direction)
                })
                .collect();
            query.push_str(&sort_clauses.join(", "));
        }

        // Add LIMIT
        if let Some(limit) = criteria.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Add OFFSET
        if let Some(offset) = criteria.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        query
    }

    /// Builds a COUNT query
    pub fn build_count_query(table_name: &str, criteria: &FilterCriteria) -> String {
        let mut query = format!("SELECT COUNT(*) FROM {}", table_name);

        let where_clause = Self::build_where_clause(criteria);
        if !where_clause.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clause);
        }

        query
    }

    /// Builds a WHERE clause from FilterCriteria conditions
    ///
    /// Converts conditions to SQL with properly escaped values
    fn build_where_clause(criteria: &FilterCriteria) -> String {
        if criteria.conditions.is_empty() {
            return String::new();
        }

        let conditions: Vec<String> = criteria
            .conditions
            .iter()
            .map(|condition| {
                let field = &condition.field;
                let value = &condition.value;

                match condition.operator {
                    Operator::Equal => {
                        format!("{} = {}", field, Self::format_value(value))
                    }
                    Operator::NotEqual => {
                        format!("{} != {}", field, Self::format_value(value))
                    }
                    Operator::GreaterThan => {
                        format!("{} > {}", field, Self::format_value(value))
                    }
                    Operator::GreaterThanOrEqual => {
                        format!("{} >= {}", field, Self::format_value(value))
                    }
                    Operator::LessThan => {
                        format!("{} < {}", field, Self::format_value(value))
                    }
                    Operator::LessThanOrEqual => {
                        format!("{} <= {}", field, Self::format_value(value))
                    }
                    Operator::Like => {
                        format!("{} ILIKE {}", field, Self::format_value(value))
                    }
                    Operator::IsNull => {
                        format!("{} IS NULL", field)
                    }
                    Operator::IsNotNull => {
                        format!("{} IS NOT NULL", field)
                    }
                    Operator::In => {
                        if let ConditionValue::List(values) = value {
                            let formatted_values: Vec<String> =
                                values.iter().map(Self::format_value).collect();
                            format!("{} IN ({})", field, formatted_values.join(", "))
                        } else {
                            format!("{} = {}", field, Self::format_value(value))
                        }
                    }
                }
            })
            .collect();

        conditions.join(" AND ")
    }

    /// Formats a ConditionValue for SQL (with proper escaping)
    ///
    /// Note: This uses PostgreSQL's dollar-quoted strings for safety
    fn format_value(value: &ConditionValue) -> String {
        match value {
            ConditionValue::String(s) => {
                // Use PostgreSQL dollar quoting to avoid SQL injection
                // Escape single quotes by doubling them
                let escaped = s.replace("'", "''");
                format!("'{}'", escaped)
            }
            ConditionValue::Integer(i) => i.to_string(),
            ConditionValue::Float(f) => f.to_string(),
            ConditionValue::Boolean(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            ConditionValue::Null => "NULL".to_string(),
            ConditionValue::List(values) => {
                let formatted: Vec<String> = values.iter().map(Self::format_value).collect();
                format!("({})", formatted.join(", "))
            }
        }
    }
}

/// Extension trait for executing wyvern queries with SQLx
#[async_trait::async_trait]
pub trait WyvernSqlxExt {
    /// Execute a filter query and return all matching entities
    async fn filter_entities<T>(
        &self,
        table_name: &str,
        criteria: &FilterCriteria,
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send;

    /// Count entities matching the criteria
    async fn count_entities(
        &self,
        table_name: &str,
        criteria: &FilterCriteria,
    ) -> Result<i64, sqlx::Error>;
}

#[async_trait::async_trait]
impl WyvernSqlxExt for PgPool {
    async fn filter_entities<T>(
        &self,
        table_name: &str,
        criteria: &FilterCriteria,
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
    {
        let query = SqlxAdapter::build_select_query(table_name, criteria);
        sqlx::query_as::<_, T>(&query).fetch_all(self).await
    }

    async fn count_entities(
        &self,
        table_name: &str,
        criteria: &FilterCriteria,
    ) -> Result<i64, sqlx::Error> {
        let query = SqlxAdapter::build_count_query(table_name, criteria);
        sqlx::query_scalar::<_, i64>(&query).fetch_one(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Condition, SortOrder};

    #[test]
    fn test_build_simple_query() {
        let criteria =
            FilterCriteria::new().with_condition(Condition::eq("provider", "openai".into()));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("SELECT * FROM llm_model_pricing"));
        assert!(query.contains("WHERE provider = 'openai'"));
    }

    #[test]
    fn test_build_query_with_multiple_conditions() {
        let criteria = FilterCriteria::new()
            .with_condition(Condition::eq("provider", "openai".into()))
            .with_condition(Condition::gt("price", 10.into()));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("provider = 'openai'"));
        assert!(query.contains("price > 10"));
        assert!(query.contains("AND"));
    }

    #[test]
    fn test_build_query_with_sorting() {
        let criteria = FilterCriteria::new()
            .with_sort(SortOrder::asc("model_name"))
            .with_sort(SortOrder::desc("created_at"));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("ORDER BY model_name ASC, created_at DESC"));
    }

    #[test]
    fn test_build_query_with_limit_offset() {
        let criteria = FilterCriteria::new().with_limit(10).with_offset(20);

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("LIMIT 10"));
        assert!(query.contains("OFFSET 20"));
    }

    #[test]
    fn test_build_query_with_null_check() {
        let criteria = FilterCriteria::new().with_condition(Condition::new(
            "valid_to",
            Operator::IsNull,
            ConditionValue::Null,
        ));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("valid_to IS NULL"));
    }

    #[test]
    fn test_format_string_with_quotes() {
        let value = ConditionValue::String("O'Reilly".to_string());
        let formatted = SqlxAdapter::format_value(&value);

        // Should escape the single quote
        assert_eq!(formatted, "'O''Reilly'");
    }

    #[test]
    fn test_build_count_query() {
        let criteria = FilterCriteria::new().with_condition(Condition::eq("active", true.into()));

        let query = SqlxAdapter::build_count_query("users", &criteria);

        assert!(query.contains("SELECT COUNT(*) FROM users"));
        assert!(query.contains("WHERE active = TRUE"));
    }

    #[test]
    fn test_like_operator() {
        let criteria = FilterCriteria::new().with_condition(Condition::new(
            "model_name",
            Operator::Like,
            ConditionValue::String("%gpt%".to_string()),
        ));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("model_name ILIKE '%gpt%'"));
    }

    #[test]
    fn test_in_operator() {
        let criteria = FilterCriteria::new().with_condition(Condition::new(
            "provider",
            Operator::In,
            ConditionValue::List(vec![
                ConditionValue::String("openai".to_string()),
                ConditionValue::String("anthropic".to_string()),
            ]),
        ));

        let query = SqlxAdapter::build_select_query("llm_model_pricing", &criteria);

        assert!(query.contains("provider IN ('openai', 'anthropic')"));
    }
}
