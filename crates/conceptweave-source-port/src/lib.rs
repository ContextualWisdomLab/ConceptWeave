//! Bounded Source Observation port contracts for ConceptWeave.
//!
//! This crate owns provider-independent access budgets, exact source allowlists, caller
//! cancellation, and fail-closed adapter outcomes. PostgreSQL drivers, credentials, catalog SQL,
//! and immutable snapshot construction remain behind an adapter implementation.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{
    collections::BTreeSet,
    future::Future,
    time::{Duration, Instant},
};

const MAX_SOURCE_CONNECTION_KEY_BYTES: usize = 128;
const MAX_CONNECTION_POLICY_BINDING_BYTES: usize = 128;

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
    #[must_use]
    pub const fn operation_timeout_ms(&self) -> u64 {
        self.operation_timeout_ms
    }

    /// Returns the maximum time one source statement may execute, in milliseconds.
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

/// Invalid request metadata or fail-closed registry-authorization outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationRequestError {
    /// The source-connection registry key was blank or not a bounded multiword snake_case key.
    InvalidSourceConnectionKey,
    /// The syntactically valid key was absent from the caller's authorized source registry.
    UnknownSourceConnectionKey,
    /// The known source did not expose an immutable connection-policy binding.
    MissingConnectionPolicyBinding,
    /// The registry returned a policy binding that is not a bounded opaque multiword snake_case id.
    InvalidConnectionPolicyBinding,
    /// The source existed, but the registry did not authorize the exact requested schema scope.
    UnauthorizedSchemaScope,
    /// Registry authorization exhausted the request's end-to-end operation budget.
    OperationTimeout,
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

/// Read-only registry boundary used to authorize an opaque source connection and exact schema scope.
///
/// A source key is only a lookup coordinate. A successful registry implementation must also issue
/// an opaque immutable connection-policy binding for the exact mapping it authorizes. Schema scope
/// is then evaluated against that resolved key-and-binding pair, preventing a later key remap from
/// silently inheriting an earlier authorization.
pub trait SourceConnectionRegistry {
    /// Returns whether the exact key names a source the caller may observe.
    fn contains_source_connection(&self, source_connection_key: &str) -> bool;

    /// Returns the opaque immutable policy revision for the exact registered source mapping.
    ///
    /// The default is fail-closed. The returned value must be a bounded lowercase multiword
    /// `snake_case` identifier, such as `policy_revision_a` or a digest encoded as an opaque
    /// identifier. It is provider-independent evidence, not a DSN, credential, token, connection
    /// object, or wall-clock timestamp.
    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        let _ = source_connection_key;
        None
    }

    /// Returns whether the exact requested schema scope is authorized for the resolved source binding.
    ///
    /// The default is fail-closed so a registry that only recognizes a source key cannot silently
    /// turn caller-selected schema names into application authorization. Implementations that grant
    /// schema access must compare the supplied binding with the same policy revision that owns the
    /// scope and must preserve exact identifier spelling rather than broadening access through case
    /// or Unicode normalization.
    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        let _ = (source_connection, allowed_schema_names);
        false
    }
}

/// Opaque proof that an exact source key and immutable policy revision were resolved together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSourceConnection {
    source_connection_key: String,
    connection_policy_binding: String,
}

impl ResolvedSourceConnection {
    /// Returns the resolved opaque registry key, never connection material.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the opaque immutable connection-policy revision authorized for this source.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }
}

/// One fail-closed request to observe explicitly authorized source schemas.
///
/// `source_connection_key` is a bounded opaque identifier, not source authority by itself. Before
/// adapter execution, [`Self::authorize`] must resolve it through the caller's authorized
/// [`SourceConnectionRegistry`], bind the registry's immutable connection-policy revision, verify
/// the exact requested schema scope against that same resolved binding, and carry the capability
/// into an [`AuthorizedObservationRequest`]. The adapter later maps only that exact authorized
/// binding to credentials inside its own ACL. Schema identifiers retain exact source spelling and
/// are sorted only to make request identity deterministic. Callers must also provide an explicit
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
        if !is_valid_opaque_multiword_identifier(
            &source_connection_key,
            MAX_SOURCE_CONNECTION_KEY_BYTES,
        ) {
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

    /// Resolves this request's opaque key and immutable policy revision through the registry.
    pub fn resolve_source_connection(
        &self,
        registry: &dyn SourceConnectionRegistry,
    ) -> Result<ResolvedSourceConnection, ObservationRequestError> {
        if !registry.contains_source_connection(&self.source_connection_key) {
            return Err(ObservationRequestError::UnknownSourceConnectionKey);
        }
        let connection_policy_binding = registry
            .connection_policy_binding(&self.source_connection_key)
            .ok_or(ObservationRequestError::MissingConnectionPolicyBinding)?;
        if !is_valid_opaque_multiword_identifier(
            &connection_policy_binding,
            MAX_CONNECTION_POLICY_BINDING_BYTES,
        ) {
            return Err(ObservationRequestError::InvalidConnectionPolicyBinding);
        }
        Ok(ResolvedSourceConnection {
            source_connection_key: self.source_connection_key.clone(),
            connection_policy_binding,
        })
    }

    /// Consumes this request after registry authorization and binds the resulting capability to it.
    ///
    /// The operation budget starts before source-key, immutable policy-binding, and exact-schema
    /// authorization. The returned execution envelope is the only request type accepted by
    /// [`SourceObservationPort`] and privately retains the monotonic start coordinate so adapter
    /// code can query the remaining budget without receiving wall-clock provenance. If registry
    /// work consumes the budget, timeout takes precedence over either authorization result so
    /// over-budget policy work never leaks into adapter admission.
    pub fn authorize(
        self,
        registry: &dyn SourceConnectionRegistry,
    ) -> Result<AuthorizedObservationRequest, ObservationRequestError> {
        let operation_started_at = Instant::now();
        let source_connection = self.resolve_source_connection(registry);
        let schema_scope_authorized = source_connection.as_ref().is_ok_and(|resolved| {
            registry.authorizes_schema_scope(resolved, &self.allowed_schema_names)
        });
        let elapsed = Instant::now().saturating_duration_since(operation_started_at);
        let operation_timeout = Duration::from_millis(self.limits.operation_timeout_ms);
        if elapsed >= operation_timeout {
            return Err(ObservationRequestError::OperationTimeout);
        }
        let source_connection = source_connection?;
        if !schema_scope_authorized {
            return Err(ObservationRequestError::UnauthorizedSchemaScope);
        }
        Ok(AuthorizedObservationRequest {
            request: self,
            source_connection,
            operation_started_at,
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
/// request to the opaque [`ResolvedSourceConnection`] issued by the authorized registry after the
/// same policy boundary has explicitly accepted the request's exact schema scope against the same
/// immutable connection-policy revision. It also retains a private monotonic operation-start
/// coordinate so the adapter can cap connection, transaction, statement and cancellation work by
/// the true remaining budget. It carries no connection string, credential, token,
/// provider-specific connection object, or wall-clock time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedObservationRequest {
    request: ObservationRequest,
    source_connection: ResolvedSourceConnection,
    operation_started_at: Instant,
}

impl AuthorizedObservationRequest {
    /// Returns the validated request metadata and execution budgets bound to this authorization.
    #[must_use]
    pub const fn request(&self) -> &ObservationRequest {
        &self.request
    }

    /// Returns the exact opaque source-and-policy capability used by the adapter ACL.
    #[must_use]
    pub const fn source_connection(&self) -> &ResolvedSourceConnection {
        &self.source_connection
    }

    /// Returns the remaining end-to-end operation budget at the instant of this call.
    ///
    /// `None` means the original budget, which began before registry authorization, is exhausted.
    /// The opaque monotonic start coordinate is never exposed or serialized.
    #[must_use]
    pub fn remaining_operation_budget(&self) -> Option<Duration> {
        let elapsed = Instant::now().saturating_duration_since(self.operation_started_at);
        let operation_timeout = Duration::from_millis(self.request.limits.operation_timeout_ms);
        if elapsed >= operation_timeout {
            None
        } else {
            Some(operation_timeout - elapsed)
        }
    }
}

fn is_valid_opaque_multiword_identifier(value: &str, max_bytes: usize) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() > max_bytes {
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
/// Implementations receive only a registry-authorized request whose exact schema scope was accepted
/// against the same immutable connection-policy binding, resolve credentials from that exact opaque
/// capability inside the adapter ACL, use only read-only source access, honor the exact schema
/// allowlist, query [`AuthorizedObservationRequest::remaining_operation_budget`] before adapter-side
/// blocking work, enforce every adapter-side [`ObservationLimits`] bound, check caller cancellation,
/// and return a typed failure rather than a partial or invented snapshot when captured metadata
/// cannot construct the immutable snapshot. Observation execution is awaitable so asynchronous
/// database clients do not need to hide a nested executor or block an asynchronous web executor
/// thread.
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
