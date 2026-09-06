//! Bounded Source Observation port contracts for ConceptWeave.
//!
//! This crate owns provider-independent access budgets, exact source allowlists, trusted local
//! policy admission, caller cancellation, and fail-closed adapter outcomes. PostgreSQL drivers,
//! credentials, catalog SQL, and immutable snapshot construction remain behind an adapter
//! implementation.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{
    collections::BTreeSet,
    future::Future,
    time::{Duration, Instant},
};

const MAX_SOURCE_CONNECTION_KEY_BYTES: usize = 128;
const MAX_CONNECTION_POLICY_BINDING_BYTES: usize = 128;
/// Canonical product-level maximum number of exact schema identifiers retained before trusted source policy runs.
pub const MAX_STRUCTURAL_SCHEMA_COUNT: usize = 4_096;
/// Canonical product-level maximum UTF-8 bytes retained across exact schema identifiers before trusted source policy runs.
pub const MAX_STRUCTURAL_SCHEMA_BYTES: usize = 1_048_576;

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

/// Explicit positive resource limits requested for one Source Observation operation.
///
/// Positive values make the request structurally bounded, but they are not authority. The trusted
/// local [`SourceConnectionRegistry`] must explicitly admit the complete resource envelope before
/// adapter execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationLimits {
    operation_timeout_ms: u64,
    statement_timeout_ms: u64,
    max_rows: u64,
    max_bytes: u64,
    max_concurrent_queries: u32,
}

impl ObservationLimits {
    /// Creates a conservative bounded request whose total operation deadline equals the statement timeout.
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

    /// Creates a bounded request with separate end-to-end and per-statement time budgets.
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

    /// Returns the requested ceiling for authorization, connection and all catalog work.
    #[must_use]
    pub const fn operation_timeout_ms(&self) -> u64 {
        self.operation_timeout_ms
    }

    /// Returns the requested maximum time one source statement may execute, in milliseconds.
    #[must_use]
    pub const fn statement_timeout_ms(&self) -> u64 {
        self.statement_timeout_ms
    }

    /// Returns the requested maximum number of source metadata rows the request may observe.
    #[must_use]
    pub const fn max_rows(&self) -> u64 {
        self.max_rows
    }

    /// Returns the requested maximum number of source metadata bytes the request may retain.
    #[must_use]
    pub const fn max_bytes(&self) -> u64 {
        self.max_bytes
    }

    /// Returns the requested maximum number of catalog queries the adapter may run concurrently.
    #[must_use]
    pub const fn max_concurrent_queries(&self) -> u32 {
        self.max_concurrent_queries
    }
}

/// Invalid authorization-metadata bounds for one observation request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationRequestBudgetError {
    /// The maximum number of authorized schema identifiers was zero.
    ZeroSchemaCountLimit,
    /// The maximum retained UTF-8 bytes across authorized schema identifiers was zero.
    ZeroSchemaByteLimit,
    /// The caller requested a schema-count structural ceiling above ConceptWeave's provider-independent hard cap.
    SchemaCountLimitTooLarge {
        /// Maximum structural schema-count ceiling accepted before trusted source policy runs.
        maximum: usize,
    },
    /// The caller requested a schema-byte structural ceiling above ConceptWeave's provider-independent hard cap.
    SchemaByteLimitTooLarge {
        /// Maximum structural schema-byte ceiling accepted before trusted source policy runs.
        maximum: usize,
    },
}

/// Caller-selected positive bounds for authorization metadata retained by an observation request.
///
/// These bounds are intentionally provider-independent. They limit how much exact schema-selection
/// metadata ConceptWeave accepts before registry or database access without assuming PostgreSQL's
/// build-time identifier length or normalizing source spelling. Callers may request only values at
/// or below [`MAX_STRUCTURAL_SCHEMA_COUNT`] and [`MAX_STRUCTURAL_SCHEMA_BYTES`]; trusted local source
/// policy must still admit an equal-or-narrower complete resource envelope afterward.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRequestBudget {
    max_schema_count: usize,
    max_schema_bytes: usize,
}

impl ObservationRequestBudget {
    /// Creates explicit positive count and total UTF-8 byte bounds for the exact schema allowlist.
    ///
    /// The canonical structural caps are product-level denial-of-service guardrails, not PostgreSQL
    /// identifier semantics or source-specific authorization. They prevent callers from minting an
    /// effectively unbounded retained-metadata envelope before trusted registry policy can run.
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
        if max_schema_count > MAX_STRUCTURAL_SCHEMA_COUNT {
            return Err(ObservationRequestBudgetError::SchemaCountLimitTooLarge {
                maximum: MAX_STRUCTURAL_SCHEMA_COUNT,
            });
        }
        if max_schema_bytes > MAX_STRUCTURAL_SCHEMA_BYTES {
            return Err(ObservationRequestBudgetError::SchemaByteLimitTooLarge {
                maximum: MAX_STRUCTURAL_SCHEMA_BYTES,
            });
        }
        Ok(Self {
            max_schema_count,
            max_schema_bytes,
        })
    }

    /// Returns the requested maximum number of exact schema identifiers the request may retain.
    #[must_use]
    pub const fn max_schema_count(&self) -> usize {
        self.max_schema_count
    }

    /// Returns the requested maximum total UTF-8 bytes retained across exact schema identifiers.
    #[must_use]
    pub const fn max_schema_bytes(&self) -> usize {
        self.max_schema_bytes
    }
}

/// Complete provider-independent resource request evaluated by trusted local source policy.
///
/// This value combines authorization-metadata and runtime ceilings so one policy decision cannot
/// admit only part of the resource contract. Constructing the value does not confer authority;
/// [`ObservationRequest::authorize`] must obtain an explicit registry decision for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationResourceEnvelope {
    request_budget: ObservationRequestBudget,
    limits: ObservationLimits,
}

impl ObservationResourceEnvelope {
    /// Combines the caller-requested metadata and runtime ceilings into one policy input.
    #[must_use]
    pub const fn new(
        request_budget: ObservationRequestBudget,
        limits: ObservationLimits,
    ) -> Self {
        Self {
            request_budget,
            limits,
        }
    }

    /// Returns the requested authorization-metadata ceilings.
    #[must_use]
    pub const fn request_budget(&self) -> ObservationRequestBudget {
        self.request_budget
    }

    /// Returns the requested runtime resource ceilings.
    #[must_use]
    pub const fn limits(&self) -> ObservationLimits {
        self.limits
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
    /// The source and schema were authorized, but trusted local policy did not admit the requested resource envelope.
    UnauthorizedResourceEnvelope,
    /// Registry authorization exhausted the request's end-to-end operation budget.
    OperationTimeout,
    /// No source schema was explicitly authorized for observation.
    EmptySchemaAllowlist,
    /// The requested schema count exceeded the caller-selected authorization-metadata budget.
    SchemaCountLimitExceeded {
        /// Maximum allowed schema count within the caller-requested metadata envelope.
        max_schema_count: usize,
    },
    /// The requested schema identifiers exceeded the caller-selected total UTF-8 byte budget.
    SchemaByteLimitExceeded {
        /// Maximum allowed total UTF-8 bytes within the caller-requested metadata envelope.
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

/// Read-only registry boundary used to authorize source identity, exact schema scope and resources.
///
/// A source key is only a lookup coordinate. A successful registry implementation must also issue
/// an opaque immutable connection-policy binding for the exact mapping it authorizes. Schema scope
/// and the complete provider-independent resource envelope are then evaluated against that same
/// resolved key-and-binding pair. Both policy decisions default to deny.
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

    /// Returns whether trusted local policy admits the complete requested resource envelope.
    ///
    /// The default is fail-closed. Implementations must evaluate the envelope against the same
    /// immutable source-policy binding used for schema authorization. A wider-than-policy request
    /// must be rejected; equal or narrower requests may be admitted explicitly. Provider-specific
    /// settings, credentials, DSNs and runtime connection objects do not belong in this decision.
    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        let _ = (source_connection, resource_envelope);
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
/// the exact requested schema scope and complete provider-independent resource envelope against that
/// same resolved binding, and carry the capability into an [`AuthorizedObservationRequest`]. The
/// adapter later maps only that exact authorized binding to credentials inside its own ACL. Schema
/// identifiers retain exact source spelling and are sorted only to make request identity
/// deterministic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationRequest {
    source_connection_key: String,
    allowed_schema_names: Vec<String>,
    request_budget: ObservationRequestBudget,
    limits: ObservationLimits,
}

impl ObservationRequest {
    /// Creates a structurally bounded request with an explicit non-empty exact-schema allowlist.
    ///
    /// Successful construction does not mean the caller-selected resource ceilings are authorized;
    /// trusted registry policy must admit them in [`Self::authorize`].
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

    /// Consumes this request after trusted registry authorization and binds the capability to it.
    ///
    /// The operation budget starts before source-key, immutable policy-binding, exact-schema and
    /// resource-envelope authorization. The returned execution envelope is the only request type
    /// accepted by [`SourceObservationPort`] and privately retains the monotonic start coordinate so
    /// adapter code can query the remaining budget without receiving wall-clock provenance. If any
    /// registry stage consumes the budget, timeout takes precedence over that stage's authorization
    /// result and no later registry policy stage is started.
    pub fn authorize(
        self,
        registry: &dyn SourceConnectionRegistry,
    ) -> Result<AuthorizedObservationRequest, ObservationRequestError> {
        let operation_started_at = Instant::now();
        let operation_timeout = Duration::from_millis(self.limits.operation_timeout_ms);
        let budget_exhausted = || {
            Instant::now().saturating_duration_since(operation_started_at) >= operation_timeout
        };

        let source_exists = registry.contains_source_connection(&self.source_connection_key);
        if budget_exhausted() {
            return Err(ObservationRequestError::OperationTimeout);
        }
        if !source_exists {
            return Err(ObservationRequestError::UnknownSourceConnectionKey);
        }

        let connection_policy_binding =
            registry.connection_policy_binding(&self.source_connection_key);
        if budget_exhausted() {
            return Err(ObservationRequestError::OperationTimeout);
        }
        let connection_policy_binding = connection_policy_binding
            .ok_or(ObservationRequestError::MissingConnectionPolicyBinding)?;
        if !is_valid_opaque_multiword_identifier(
            &connection_policy_binding,
            MAX_CONNECTION_POLICY_BINDING_BYTES,
        ) {
            return Err(ObservationRequestError::InvalidConnectionPolicyBinding);
        }
        let source_connection = ResolvedSourceConnection {
            source_connection_key: self.source_connection_key.clone(),
            connection_policy_binding,
        };

        let schema_scope_authorized =
            registry.authorizes_schema_scope(&source_connection, &self.allowed_schema_names);
        if budget_exhausted() {
            return Err(ObservationRequestError::OperationTimeout);
        }
        if !schema_scope_authorized {
            return Err(ObservationRequestError::UnauthorizedSchemaScope);
        }

        let resource_envelope = self.resource_envelope();
        let resource_envelope_authorized =
            registry.authorizes_resource_envelope(&source_connection, resource_envelope);
        if budget_exhausted() {
            return Err(ObservationRequestError::OperationTimeout);
        }
        if !resource_envelope_authorized {
            return Err(ObservationRequestError::UnauthorizedResourceEnvelope);
        }

        Ok(AuthorizedObservationRequest {
            request: self,
            source_connection,
            operation_started_at,
        })
    }

    /// Returns exact requested schema identifiers in deterministic lexical order.
    #[must_use]
    pub fn allowed_schema_names(&self) -> &[String] {
        &self.allowed_schema_names
    }

    /// Returns the caller-requested authorization-metadata budget.
    #[must_use]
    pub const fn request_budget(&self) -> ObservationRequestBudget {
        self.request_budget
    }

    /// Returns the caller-requested runtime resource limits.
    #[must_use]
    pub const fn limits(&self) -> ObservationLimits {
        self.limits
    }

    /// Returns the complete provider-independent resource envelope evaluated by trusted policy.
    #[must_use]
    pub const fn resource_envelope(&self) -> ObservationResourceEnvelope {
        ObservationResourceEnvelope::new(self.request_budget, self.limits)
    }
}

/// Registry-authorized request envelope accepted by a concrete source adapter.
///
/// This value can only be created by [`ObservationRequest::authorize`], which binds the exact
/// request to the opaque [`ResolvedSourceConnection`] issued by the authorized registry after the
/// same policy boundary has explicitly accepted both the exact schema scope and complete requested
/// resource envelope against the same immutable connection-policy revision. It also retains a
/// private monotonic operation-start coordinate so the adapter can cap connection, transaction,
/// statement and cancellation work by the true remaining budget. It carries no connection string,
/// credential, token, provider-specific connection object, or wall-clock time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedObservationRequest {
    request: ObservationRequest,
    source_connection: ResolvedSourceConnection,
    operation_started_at: Instant,
}

impl AuthorizedObservationRequest {
    /// Returns the validated and policy-admitted request metadata and resource ceilings.
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
    /// Observed metadata exceeded the explicitly admitted row budget.
    RowLimitExceeded {
        /// Policy-admitted maximum row count.
        max_rows: u64,
    },
    /// Observed metadata exceeded the explicitly admitted byte budget.
    ByteLimitExceeded {
        /// Policy-admitted maximum retained byte count.
        max_bytes: u64,
    },
    /// The adapter could not remain within the explicitly admitted concurrent-query budget.
    ConcurrencyLimitExceeded {
        /// Policy-admitted maximum concurrent query count.
        max_concurrent_queries: u32,
    },
}

/// Port implemented by a concrete read-only source adapter.
///
/// Implementations receive only a registry-authorized request whose exact schema scope and complete
/// provider-independent resource envelope were accepted against the same immutable connection-policy
/// binding. They resolve credentials from that exact opaque capability inside the adapter ACL, use
/// only read-only source access, honor the exact schema allowlist, query
/// [`AuthorizedObservationRequest::remaining_operation_budget`] before adapter-side blocking work,
/// enforce every policy-admitted [`ObservationLimits`] bound, check caller cancellation, and return
/// a typed failure rather than a partial or invented snapshot when captured metadata cannot construct
/// the immutable snapshot. Observation execution is awaitable so asynchronous database clients do not
/// need to hide a nested executor or block an asynchronous web executor thread.
pub trait SourceObservationPort: Sync {
    /// Immutable snapshot type produced only after a complete bounded observation.
    type Snapshot;

    /// Executes one bounded asynchronous observation after trusted registry policy has issued the source capability.
    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a;
}
