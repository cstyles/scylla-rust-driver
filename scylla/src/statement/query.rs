use super::StatementConfig;
use crate::frame::types::{Consistency, SerialConsistency};
use crate::transport::retry_policy::RetryPolicy;
use std::time::Duration;

/// CQL query statement.
///
/// This represents a CQL query that can be executed on a server.
#[derive(Clone)]
pub struct Query {
    pub(crate) config: StatementConfig,

    pub contents: String,
    page_size: Option<i32>,
}

impl Query {
    /// Creates a new `Query` from a CQL query string.
    pub fn new(query_text: impl Into<String>) -> Self {
        Self {
            contents: query_text.into(),
            page_size: None,
            config: Default::default(),
        }
    }

    /// Returns self with page size set to the given value
    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the page size for this CQL query.
    pub fn set_page_size(&mut self, page_size: i32) {
        assert!(page_size > 0, "page size must be larger than 0");
        self.page_size = Some(page_size);
    }

    /// Disables paging for this CQL query.
    pub fn disable_paging(&mut self) {
        self.page_size = None;
    }

    /// Returns the page size for this CQL query.
    pub fn get_page_size(&self) -> Option<i32> {
        self.page_size
    }

    /// Sets the consistency to be used when executing this statement.
    pub fn set_consistency(&mut self, c: Consistency) {
        self.config.consistency = Some(c);
    }

    /// Gets the consistency to be used when executing this query if it is filled.
    /// If this is empty, the default_consistency of the session will be used.
    pub fn get_consistency(&self) -> Option<Consistency> {
        self.config.consistency
    }

    /// Sets the serial consistency to be used when executing this statement.
    /// (Ignored unless the statement is an LWT)
    pub fn set_serial_consistency(&mut self, sc: Option<SerialConsistency>) {
        self.config.serial_consistency = sc;
    }

    /// Gets the serial consistency to be used when executing this statement.
    /// (Ignored unless the statement is an LWT)
    pub fn get_serial_consistency(&self) -> Option<SerialConsistency> {
        self.config.serial_consistency
    }

    /// Sets the idempotence of this statement
    /// A query is idempotent if it can be applied multiple times without changing the result of the initial application
    /// If set to `true` we can be sure that it is idempotent
    /// If set to `false` it is unknown whether it is idempotent
    /// This is used in [`RetryPolicy`] to decide if retrying a query is safe
    pub fn set_is_idempotent(&mut self, is_idempotent: bool) {
        self.config.is_idempotent = is_idempotent;
    }

    /// Gets the idempotence of this statement
    pub fn get_is_idempotent(&self) -> bool {
        self.config.is_idempotent
    }

    /// Sets a custom [`RetryPolicy`] to be used with this statement
    /// By default Session's retry policy is used, this allows to use a custom retry policy
    pub fn set_retry_policy(&mut self, retry_policy: Box<dyn RetryPolicy>) {
        self.config.retry_policy = Some(retry_policy);
    }

    /// Gets custom [`RetryPolicy`] used by this statement
    pub fn get_retry_policy(&self) -> &Option<Box<dyn RetryPolicy>> {
        &self.config.retry_policy
    }

    /// Enable or disable CQL Tracing for this statement
    /// If enabled session.query() will return a QueryResult containing tracing_id
    /// which can be used to query tracing information about the execution of this query
    pub fn set_tracing(&mut self, should_trace: bool) {
        self.config.tracing = should_trace;
    }

    /// Gets whether tracing is enabled for this statement
    pub fn get_tracing(&self) -> bool {
        self.config.tracing
    }

    /// Sets the default timestamp for this statement in microseconds.
    /// If not None, it will replace the server side assigned timestamp as default timestamp
    /// If a statement contains a `USING TIMESTAMP` clause, calling this method won't change
    /// anything
    pub fn set_timestamp(&mut self, timestamp: Option<i64>) {
        self.config.timestamp = timestamp
    }

    /// Gets the default timestamp for this statement in microseconds.
    pub fn get_timestamp(&self) -> Option<i64> {
        self.config.timestamp
    }

    /// Sets the client-side timeout for this statement.
    /// If not None, the driver will stop waiting for the request
    /// to finish after `timeout` passed.
    /// Otherwise, default session client timeout will be applied.
    pub fn set_request_timeout(&mut self, timeout: Option<Duration>) {
        self.config.request_timeout = timeout
    }

    /// Gets client timeout associated with this query
    pub fn get_request_timeout(&self) -> Option<Duration> {
        self.config.request_timeout
    }
}

impl From<String> for Query {
    fn from(s: String) -> Query {
        Query::new(s)
    }
}

impl<'a> From<&'a str> for Query {
    fn from(s: &'a str) -> Query {
        Query::new(s.to_owned())
    }
}
