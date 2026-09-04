//! Bounded Source Observation port contracts for ConceptWeave.
//!
//! This crate owns provider-independent access budgets, exact source allowlists, caller
//! cancellation, and fail-closed adapter outcomes. PostgreSQL drivers, credentials, catalog SQL,
//! and immutable snapshot construction remain behind an adapter implementation.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::collections::BTreeSet;

const MAX_SOURCE_CONNECTION_KEY_BYTES: usize = 128;

/// Invalid zero-valued resource bounds for one source-observation request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationLimitError {
    /// The total observation-operation timeout was zero and therefore unbounded.
    ZeroOperationTimeout,
    /// The statement timeout was zero and therefore could permit an unbounded wait.
    ZeroStatementTimeout,
    /// The maximum observed-row count was zero.
    ZeroRowLimit,
    /// The maximum observed-byte count was zero.
    ZeroByteLimit,
    /// The maximum concurrent-query count was zero.
    ZeroConcurrencyLimit,
}

/// Explicit positive resource limits that every Source Observation adapter must enforce.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationLimits {
    operation_timeout_ms: u64,
    statement_timeout_ms: u64,
    max_rows: u64,
    max_bytes: u64,
    max_concurrent_queries: u32,
}

impl ObservationLimits {
    /// Creates a conservative bounded policy whose total operation deadline equals the statement timeout.
    ///
    /// This constructor preserves the original API while making the end-to-end deadline explicit for
    /// every request. Use [`Self::with_timeouts`] when connection/registry/catalog work needs a larger
    /// total budget than any individual source statement.
    pub const fn new(
        statement_timeout_ms: u64,
        max_rows: u64,
        max_bytes: u64,
        max_concurrent_queries: u32,
    ) -> Result<Self, ObservationLimitError> {
        Self::with_timeouts(
            statement_timeout_ms,
            statement_timeout_ms,
            max_rows,
            max_bytes,
            max_concurrent_queries,
        )
    }

    /// Creates a bounded policy with separate end-to-end and per-statement time budgets.
    pub const fn with_timeouts(
        operation_timeout_ms: u64,
        statement_timeout_ms: u64,
        max_rows: u64,
        max_bytes: u64,
        max_concurrent_queries: u32,
    ) -> Result<Self, ObservationLimitError> {
        if operation_timeout_ms == 0 {
            return Err(ObservationLimitError::ZeroOperationTimeout);
        }
        if statement_timeout_ms == 0 {
            return Err(ObservationLimitError::ZeroStatementTimeout);
        }
        if max_rows == 0 {
            return Err(ObservationLimitError::ZeroRowLimit);
        }
        if max_bytes == 0 {
            return Err(ObservationLimitError::ZeroByteLimit);
        }
        if max_concurrent_queries == 0 {
            return Err(ObservationLimitError::ZeroConcurrencyLimit);
        }
        Ok(Self {
            operation_timeout_ms,
            statement_timeout_ms,
            max_rows,
            max_bytes,
            max_concurrent_queries,
        })
    }

    /// Returns the maximum elapsed time for registry resolution, connection and all catalog work.
    #[must_use]
    pub const fn operation_timeout_ms(&self) -> u64 {
        self.operation_timeout_ms
    }

    /// Returns the maximum time one PostgreSQL statement may execute, in milliseconds.
    #[must_use]
    pub const fn statement_timeout_ms(&self) -> u64 {
        self.statement_timeout_ms
    }

    /// Returns the maximum number of source metadata rows the request may observe.
    #[must_use]
    pub const fn max_rows(&self) -> u64 {
        self.max_rows
    }

    /// Returns the maximum number of source metadata bytes the request may retain.
    #[must_use]
    pub const fn max_bytes(&self) -> u64 {
        self.max_bytes
    }

    /// Returns the maximum number of catalog queries the adapter may run concurrently.
    #[must_use]
    pub const fn max_concurrent_queries(&self) -> u32 {
        self.max_concurrent_queries
    }
}

/// Invalid source-observation request metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationRequestError {
    /// The source-connection registry key was blank or not a bounded multiword snake_case key.
    InvalidSourceConnectionKey,
    /// No source schema was explicitly authorized for observation.
    EmptySchemaAllowlist,
    /// One authorized source schema identifier was blank.
    InvalidSchemaName,
    /// The exact same source schema identifier was authorized twice.
    DuplicateSchemaName {
        /// Exact duplicated source schema identifier.
        schema_name: String,
    },
}

/// One fail-closed request to observe explicitly authorized source schemas.
///
/// `source_connection_key` is an opaque registry identifier resolved by the adapter's credential
/// boundary. It is deliberately restricted to a bounded, lowercase, multiword `snake_case` key so
/// DSNs, URLs, shell-style connection parameters, or other credential-bearing connection material
/// cannot accidentally cross this port as a connection reference. Schema identifiers retain exact
/// source spelling and are sorted only to make request identity deterministic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationRequest {
    source_connection_key: String,
    allowed_schema_names: Vec<String>,
    limits: ObservationLimits,
}

impl ObservationRequest {
    /// Creates a bounded request with an explicit non-empty exact-schema allowlist.
    pub fn new(
        source_connection_key: impl Into<String>,
        mut allowed_schema_names: Vec<String>,
        limits: ObservationLimits,
    ) -> Result<Self, ObservationRequestError> {
        let source_connection_key = source_connection_key.into();
        if !is_valid_source_connection_key(&source_connection_key) {
            return Err(ObservationRequestError::InvalidSourceConnectionKey);
        }
        if allowed_schema_names.is_empty() {
            return Err(ObservationRequestError::EmptySchemaAllowlist);
        }

        let mut seen_schema_names = BTreeSet::new();
        for schema_name in &allowed_schema_names {
            if schema_name.trim().is_empty() {
                return Err(ObservationRequestError::InvalidSchemaName);
            }
            if !seen_schema_names.insert(schema_name.clone()) {
                return Err(ObservationRequestError::DuplicateSchemaName {
                    schema_name: schema_name.clone(),
                });
            }
        }
        allowed_schema_names.sort();

        Ok(Self {
            source_connection_key,
            allowed_schema_names,
            limits,
        })
    }

    /// Returns the opaque source-connection registry key, never a DSN or credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns exact authorized schema identifiers in deterministic lexical order.
    #[must_use]
    pub fn allowed_schema_names(&self) -> &[String] {
        &self.allowed_schema_names
    }

    /// Returns the execution limits the adapter must enforce for this request.
    #[must_use]
    pub const fn limits(&self) -> ObservationLimits {
        self.limits
    }
}

fn is_valid_source_connection_key(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() > MAX_SOURCE_CONNECTION_KEY_BYTES {
        return false;
    }

    let mut word_count = 0_u8;
    for word in value.split('_') {
        let mut word_bytes = word.bytes();
        let Some(first) = word_bytes.next() else {
            return false;
        };
        if !first.is_ascii_lowercase() {
            return false;
        }
        if !word_bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()) {
            return false;
        }
        word_count = word_count.saturating_add(1);
    }

    word_count >= 2
}

/// Caller-owned cooperative cancellation signal passed across the Source Observation port.
pub trait ObservationCancellation {
    /// Returns `true` once the caller has cancelled the observation.
    fn is_cancelled(&self) -> bool;
}

/// Fail-closed outcomes a concrete source adapter may return instead of fabricating a snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceObservationFailure {
    /// The caller cancelled the observation before a valid snapshot completed.
    Cancelled,
    /// The referenced source disappeared or could not be reached.
    SourceUnavailable,
    /// The complete observation exceeded its end-to-end operation deadline.
    OperationTimeout,
    /// A source metadata statement exceeded the request timeout.
    StatementTimeout,
    /// Captured source metadata was malformed, contradictory, duplicated, or otherwise inadmissible.
    InvalidCapturedMetadata,
    /// Observed metadata exceeded the explicit row budget.
    RowLimitExceeded {
        /// Configured maximum row count.
        max_rows: u64,
    },
    /// Observed metadata exceeded the explicit byte budget.
    ByteLimitExceeded {
        /// Configured maximum retained byte count.
        max_bytes: u64,
    },
    /// The adapter could not remain within the explicit concurrent-query budget.
    ConcurrencyLimitExceeded {
        /// Configured maximum concurrent query count.
        max_concurrent_queries: u32,
    },
}

/// Port implemented by a concrete read-only source adapter.
///
/// Implementations must resolve credentials outside this contract, use only read-only source
/// access, honor the exact schema allowlist, the total operation deadline, and every per-resource
/// [`ObservationLimits`] bound, check caller cancellation, and return a typed failure rather than a
/// partial or invented snapshot when captured metadata cannot construct the immutable snapshot.
/// Implementations own their scheduling model; blocking database work must not be performed on an
/// asynchronous web executor thread.
pub trait SourceObservationPort {
    /// Immutable snapshot type produced only after a complete bounded observation.
    type Snapshot;

    /// Executes one bounded observation against an implementation-owned source adapter.
    fn observe(
        &self,
        request: &ObservationRequest,
        cancellation: &dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure>;
}
