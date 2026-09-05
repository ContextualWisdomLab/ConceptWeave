//! Bounded Source Observation port contracts for ConceptWeave.
//!
//! This crate owns provider-independent access budgets, exact source allowlists, caller
//! cancellation, and fail-closed adapter outcomes. PostgreSQL drivers, credentials, catalog SQL,
//! and immutable snapshot construction remain behind an adapter implementation.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{collections::BTreeSet, future::Future};

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

/// Explicit positive resource limits that the Source Observation runtime must enforce.
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
    /// every request. Use [`Self::with_timeouts`] when authorization/connection/catalog work needs a
    /// larger total budget than any individual source statement.
    pub const fn new(
        statement_timeout_ms: u64,
        max_rows: u64,
        max_bytes: u64,
        max_concurrent_queries: u32,
    ) -> Result<Self, ObservationLimitError> {
        if statement_timeout_ms == 0 {
            return Err(ObservationLimitError::ZeroStatementTimeout);
        }
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

    /// Returns the policy ceiling for authorization, connection and all catalog work.
    ///
    /// Registry authorization occurs before [`SourceObservationPort::observe`], so the concrete
    /// application/adapter integration must account for that elapsed time when enforcing this
    /// end-to-end limit rather than restarting the budget at adapter entry.
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

/// Invalid zero-valued authorization-metadata bounds for one observation request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationRequestBudgetError {
    /// The maximum number of authorized schema identifiers was zero.
    ZeroSchemaCountLimit,
    /// The maximum retained UTF-8 bytes across authorized schema identifiers was zero.
    ZeroSchemaByteLimit,
}

/// Caller-selected positive bounds for authorization metadata retained by an observation request.
///
/// These bounds are intentionally provider-independent. They limit how much exact schema-selection
/// metadata ConceptWeave accepts before registry or database access without assuming PostgreSQL's
/// build-time identifier length or normalizing source spelling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRequestBudget {
    max_schema_count: usize,
    max_schema_bytes: usize,
}

impl ObservationRequestBudget {
    /// Creates explicit positive count and total UTF-8 byte bounds for the exact schema allowlist.
    pub const fn new(
        max_schema_count: usize,
        max_schema_bytes: usize,
    ) -> Result<Self, ObservationRequestBudgetError> {
        if max_schema_count == 0 {
            return Err(ObservationRequestBudgetError::ZeroSchemaCountLimit);
        }
        if max_schema_bytes == 0 {
            return Err(ObservationRequestBudgetError::ZeroSchemaByteLimit);
        }
        Ok(Self {
            max_schema_count,
            max_schema_bytes,
        })
    }

    /// Returns the maximum number of exact schema identifiers the request may retain.
    #[must_use]
    pub const fn max_schema_count(&self) -> usize {
        self.max_schema_count
    }

    /// Returns the maximum total UTF-8 bytes retained across exact schema identifiers.
    #[must_use]
    pub const fn max_schema_bytes(&self) -> usize {
        self.max_schema_bytes
    }
}

/// Invalid source-observation request metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationRequestError {
    /// The source-connection registry key was blank or not a bounded multiword snake_case key.
    InvalidSourceConnectionKey,
    /// The syntactically valid key was absent from the caller's authorized source registry.
    UnknownSourceConnectionKey,
    /// No source schema was explicitly authorized for observation.
    EmptySchemaAllowlist,
    /// The requested schema count exceeded the caller-selected authorization-metadata budget.
    SchemaCountLimitExceeded {
        /// Maximum allowed schema count.
        max_schema_count: usize,
    },
    /// The requested schema identifiers exceeded the caller-selected total UTF-8 byte budget.
    SchemaByteLimitExceeded {
        /// Maximum allowed total UTF-8 bytes across schema identifiers.
        max_schema_bytes: usize,
    },
    /// One authorized source schema identifier was blank.
    InvalidSchemaName,
    /// The exact same source schema identifier was authorized twice.
    DuplicateSchemaName {
        /// Exact duplicated source schema identifier.
        schema_name: String,
    },
}

/// Read-only registry boundary used to authorize an opaque source connection key.
pub trait SourceConnectionRegistry {
    /// Returns whether the exact key names a source the caller may observe.
    fn contains_source_connection(&self, source_connection_key: &str) -> bool;
}

/// Opaque proof that a source key was resolved by an authorized registry boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSourceConnection {
    source_connection_key: String,
}

impl ResolvedSourceConnection {
    /// Returns the resolved opaque registry key, never connection material.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }
}

/// One fail-closed request to observe explicitly authorized source schemas.
///
/// `source_connection_key` is a bounded opaque identifier, not source authority by itself. Before
/// adapter execution, [`Self::authorize`] must resolve it through the caller's authorized
/// [`SourceConnectionRegistry`] and bind the resulting capability into an
/// [`AuthorizedObservationRequest`]. The adapter later maps that authorized opaque capability to
/// credentials inside its own ACL. Schema identifiers retain exact source spelling and are sorted
/// only to make request identity deterministic. Callers must also provide an explicit
/// provider-independent authorization-metadata budget before the request can be constructed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationRequest {
    source_connection_key: String,
    allowed_schema_names: Vec<String>,
    request_budget: ObservationRequestBudget,
    limits: ObservationLimits,
}

impl ObservationRequest {
    /// Creates a bounded request with an explicit non-empty exact-schema allowlist.
    pub fn new(
        source_connection_key: impl Into<String>,
        mut allowed_schema_names: Vec<String>,
        request_budget: ObservationRequestBudget,
        limits: ObservationLimits,
    ) -> Result<Self, ObservationRequestError> {
        let source_connection_key = source_connection_key.into();
        if !is_valid_source_connection_key(&source_connection_key) {
            return Err(ObservationRequestError::InvalidSourceConnectionKey);
        }
        if allowed_schema_names.is_empty() {
            return Err(ObservationRequestError::EmptySchemaAllowlist);
        }
        if allowed_schema_names.len() > request_budget.max_schema_count {
            return Err(ObservationRequestError::SchemaCountLimitExceeded {
                max_schema_count: request_budget.max_schema_count,
            });
        }

        let mut schema_bytes = 0_usize;
        for schema_name in &allowed_schema_names {
            let Some(next_schema_bytes) = schema_bytes.checked_add(schema_name.len()) else {
                return Err(ObservationRequestError::SchemaByteLimitExceeded {
                    max_schema_bytes: request_budget.max_schema_bytes,
                });
            };
            schema_bytes = next_schema_bytes;
            if schema_bytes > request_budget.max_schema_bytes {
                return Err(ObservationRequestError::SchemaByteLimitExceeded {
                    max_schema_bytes: request_budget.max_schema_bytes,
                });
            }
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
            request_budget,
            limits,
        })
    }

    /// Returns the opaque source-connection registry key, never a DSN or credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Resolves this request's opaque key through the caller's authorized registry.
    pub fn resolve_source_connection(
        &self,
        registry: &dyn SourceConnectionRegistry,
    ) -> Result<ResolvedSourceConnection, ObservationRequestError> {
        if !registry.contains_source_connection(&self.source_connection_key) {
            return Err(ObservationRequestError::UnknownSourceConnectionKey);
        }
        Ok(ResolvedSourceConnection {
            source_connection_key: self.source_connection_key.clone(),
        })
    }

    /// Consumes this request after registry authorization and binds the resulting capability to it.
    ///
    /// The returned execution envelope is the only request type accepted by [`SourceObservationPort`].
    /// Unknown registry keys therefore fail before an adapter can receive the request, while
    /// credential material remains outside this contract.
    pub fn authorize(
        self,
        registry: &dyn SourceConnectionRegistry,
    ) -> Result<AuthorizedObservationRequest, ObservationRequestError> {
        let source_connection = self.resolve_source_connection(registry)?;
        Ok(AuthorizedObservationRequest {
            request: self,
            source_connection,
        })
    }

    /// Returns exact authorized schema identifiers in deterministic lexical order.
    #[must_use]
    pub fn allowed_schema_names(&self) -> &[String] {
        &self.allowed_schema_names
    }

    /// Returns the authorization-metadata budget applied before registry or database access.
    #[must_use]
    pub const fn request_budget(&self) -> ObservationRequestBudget {
        self.request_budget
    }

    /// Returns the resource limits the operation runtime and source adapter must jointly enforce.
    #[must_use]
    pub const fn limits(&self) -> ObservationLimits {
        self.limits
    }
}

/// Registry-authorized request envelope accepted by a concrete source adapter.
///
/// This value can only be created by [`ObservationRequest::authorize`], which binds the exact
/// request to the opaque [`ResolvedSourceConnection`] issued by the authorized registry. It carries
/// no connection string, credential, token, or provider-specific connection object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedObservationRequest {
    request: ObservationRequest,
    source_connection: ResolvedSourceConnection,
}

impl AuthorizedObservationRequest {
    /// Returns the validated request metadata and execution budgets bound to this authorization.
    #[must_use]
    pub const fn request(&self) -> &ObservationRequest {
        &self.request
    }

    /// Returns the opaque authorized source capability used by the adapter ACL.
    #[must_use]
    pub const fn source_connection(&self) -> &ResolvedSourceConnection {
        &self.source_connection
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
///
/// The signal is shareable across an await point so a concrete asynchronous adapter can expose a
/// `Send` observation future without weakening cancellation semantics.
pub trait ObservationCancellation: Sync {
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
/// Implementations receive only a registry-authorized request, resolve credentials from its opaque
/// source capability inside the adapter ACL, use only read-only source access, honor the exact
/// schema allowlist, the remaining end-to-end operation budget plus every adapter-side
/// [`ObservationLimits`] bound, check caller cancellation, and return a typed failure rather than a
/// partial or invented snapshot when captured metadata cannot construct the immutable snapshot.
/// The surrounding operation runtime is responsible for including pre-adapter registry authorization
/// in the same total deadline. Observation execution is awaitable so asynchronous database clients
/// do not need to hide a nested executor or block an asynchronous web executor thread.
pub trait SourceObservationPort: Sync {
    /// Immutable snapshot type produced only after a complete bounded observation.
    type Snapshot;

    /// Executes one bounded asynchronous observation after registry authorization has issued the source capability.
    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a;
}
