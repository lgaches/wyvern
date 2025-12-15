//! Query filtering, sorting, and pagination types

/// Filter criteria for querying entities.
#[derive(Debug, Clone, Default)]
pub struct FilterCriteria {
    /// Field-value conditions to match
    pub conditions: Vec<Condition>,
    /// Sort order for results
    pub sort: Vec<SortOrder>,
    /// Optional limit on number of results
    pub limit: Option<i64>,
    /// Optional offset for pagination
    pub offset: Option<i64>,
}

impl FilterCriteria {
    /// Creates a new empty filter criteria.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a condition to the filter.
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Adds a sort order to the filter.
    pub fn with_sort(mut self, sort: SortOrder) -> Self {
        self.sort.push(sort);
        self
    }

    /// Sets the limit for the filter.
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the offset for the filter.
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// A single filter condition.
#[derive(Debug, Clone)]
pub struct Condition {
    /// The field name to filter on
    pub field: String,
    /// The operator to apply
    pub operator: Operator,
    /// The value to compare against
    pub value: ConditionValue,
}

impl Condition {
    /// Creates a new condition.
    pub fn new(field: impl Into<String>, operator: Operator, value: ConditionValue) -> Self {
        Self {
            field: field.into(),
            operator,
            value,
        }
    }

    /// Creates an equality condition.
    pub fn eq(field: impl Into<String>, value: ConditionValue) -> Self {
        Self::new(field, Operator::Equal, value)
    }

    /// Creates a not-equal condition.
    pub fn ne(field: impl Into<String>, value: ConditionValue) -> Self {
        Self::new(field, Operator::NotEqual, value)
    }

    /// Creates a greater-than condition.
    pub fn gt(field: impl Into<String>, value: ConditionValue) -> Self {
        Self::new(field, Operator::GreaterThan, value)
    }

    /// Creates a less-than condition.
    pub fn lt(field: impl Into<String>, value: ConditionValue) -> Self {
        Self::new(field, Operator::LessThan, value)
    }

    /// Creates an IN condition.
    pub fn in_list(field: impl Into<String>, values: Vec<ConditionValue>) -> Self {
        Self::new(field, Operator::In, ConditionValue::List(values))
    }
}

/// Comparison operators for filter conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    In,
    IsNull,
    IsNotNull,
}

/// Values used in filter conditions.
#[derive(Debug, Clone)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<ConditionValue>),
    Null,
}

impl From<String> for ConditionValue {
    fn from(s: String) -> Self {
        ConditionValue::String(s)
    }
}

impl From<&str> for ConditionValue {
    fn from(s: &str) -> Self {
        ConditionValue::String(s.to_string())
    }
}

impl From<i64> for ConditionValue {
    fn from(i: i64) -> Self {
        ConditionValue::Integer(i)
    }
}

impl From<i32> for ConditionValue {
    fn from(i: i32) -> Self {
        ConditionValue::Integer(i as i64)
    }
}

impl From<f64> for ConditionValue {
    fn from(f: f64) -> Self {
        ConditionValue::Float(f)
    }
}

impl From<bool> for ConditionValue {
    fn from(b: bool) -> Self {
        ConditionValue::Boolean(b)
    }
}

/// Sort order specification.
#[derive(Debug, Clone)]
pub struct SortOrder {
    pub field: String,
    pub direction: SortDirection,
}

impl SortOrder {
    pub fn new(field: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }

    pub fn asc(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Ascending)
    }

    pub fn desc(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Descending)
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Pagination parameters.
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

impl Pagination {
    pub fn new(page: i64, per_page: i64) -> Self {
        Self { page, per_page }
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> i64 {
        self.per_page
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// A page of results with metadata.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, page: i64, per_page: i64, total_items: i64) -> Self {
        let total_pages = (total_items + per_page - 1) / per_page;
        Self {
            items,
            page,
            per_page,
            total_items,
            total_pages,
        }
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    pub fn has_previous(&self) -> bool {
        self.page > 1
    }

    pub fn next_page(&self) -> Option<i64> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }

    pub fn previous_page(&self) -> Option<i64> {
        if self.has_previous() {
            Some(self.page - 1)
        } else {
            None
        }
    }
}
