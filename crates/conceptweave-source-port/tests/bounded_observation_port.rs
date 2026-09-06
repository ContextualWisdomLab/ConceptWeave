use std::{
    future::Future,
    task::{Context, Poll, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimitError,
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationRequestBudgetError,
    ObservationRequestError, ObservationResourceEnvelope, ResolvedSourceConnection,
    SourceConnectionRegistry, SourceObservationFailure, SourceObservationPort,
};

fn limits() -> ObservationLimits {
    ObservationLimits::new(2_500, 5_000, 1_048_576, 2).expect("bounded limits")
}

fn request_budget() -> ObservationRequestBudget {
    ObservationRequestBudget::new(8, 512).expect("bounded request metadata")
}

#[test]
fn limits_preserve_timeout_row_byte_and_concurrency_bounds() {
    let limits = limits();

    assert_eq!(limits.statement_timeout_ms(), 2_500);
    assert_eq!(limits.operation_timeout_ms(), 2_500);
    assert_eq!(limits.max_rows(), 5_000);
    assert_eq!(limits.max_bytes(), 1_048_576);
    assert_eq!(limits.max_concurrent_queries(), 2);
}

#[test]
fn explicit_total_operation_deadline_is_distinct_from_statement_timeout() {
    let limits = ObservationLimits::with_timeouts(10_000, 2_500, 5_000, 1_048_576, 2)
        .expect("bounded limits with an end-to-end deadline");

    assert_eq!(limits.operation_timeout_ms(), 10_000);
    assert_eq!(limits.statement_timeout_ms(), 2_500);
    assert_eq!(
        ObservationLimits::with_timeouts(0, 1, 1, 1, 1),
        Err(ObservationLimitError::ZeroOperationTimeout)
    );
    assert_eq!(
        ObservationLimits::with_timeouts(1, 0, 1, 1, 1),
        Err(ObservationLimitError::ZeroStatementTimeout)
    );
}

#[test]
fn every_zero_resource_bound_fails_closed() {
    assert_eq!(
        ObservationLimits::new(0, 1, 1, 1),
        Err(ObservationLimitError::ZeroStatementTimeout)
    );
    assert_eq!(
        ObservationLimits::new(1, 0, 1, 1),
        Err(ObservationLimitError::ZeroRowLimit)
    );
    assert_eq!(
        ObservationLimits::new(1, 1, 0, 1),
        Err(ObservationLimitError::ZeroByteLimit)
    );
    assert_eq!(
        ObservationLimits::new(1, 1, 1, 0),
        Err(ObservationLimitError::ZeroConcurrencyLimit)
    );
}

#[test]
fn request_metadata_budget_requires_explicit_positive_count_and_byte_bounds() {
    assert_eq!(
        ObservationRequestBudget::new(0, 1),
        Err(ObservationRequestBudgetError::ZeroSchemaCountLimit)
    );
    assert_eq!(
        ObservationRequestBudget::new(1, 0),
        Err(ObservationRequestBudgetError::ZeroSchemaByteLimit)
    );

    let budget = ObservationRequestBudget::new(2, 32).expect("positive request budget");
    assert_eq!(budget.max_schema_count(), 2);
    assert_eq!(budget.max_schema_bytes(), 32);
}

#[test]
fn request_rejects_allowlist_count_and_bytes_before_registry_or_adapter_access() {
    let count_budget = ObservationRequestBudget::new(1, 64).expect("positive count budget");
    assert_eq!(
        ObservationRequest::new(
            "grc_readonly_connection",
            vec!["audit".to_owned(), "public".to_owned()],
            count_budget,
            limits(),
        ),
        Err(ObservationRequestError::SchemaCountLimitExceeded {
            max_schema_count: 1,
        })
    );

    let byte_budget = ObservationRequestBudget::new(2, 10).expect("positive byte budget");
    assert_eq!(
        ObservationRequest::new(
            "grc_readonly_connection",
            vec!["Audit/Event".to_owned()],
            byte_budget,
            limits(),
        ),
        Err(ObservationRequestError::SchemaByteLimitExceeded {
            max_schema_bytes: 10,
        })
    );
}

#[test]
fn request_preserves_exact_source_reference_and_canonicalizes_allowlist_only_by_order() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["Risk-Core".to_owned(), "Audit/Event".to_owned()],
        request_budget(),
        limits(),
    )
    .expect("valid request");

    assert_eq!(request.source_connection_key(), "grc_readonly_connection");
    assert_eq!(request.allowed_schema_names(), ["Audit/Event", "Risk-Core"]);
    assert_eq!(request.request_budget(), request_budget());
    assert_eq!(request.limits(), limits());
    assert_eq!(
        request.resource_envelope(),
        ObservationResourceEnvelope::new(request_budget(), limits())
    );
}

#[test]
fn request_rejects_non_registry_source_connection_keys_before_adapter_access() {
    for source_connection_key in [
        "postgres://reader:secret@example.invalid/database",
        "host=example.invalid password=secret",
        "warehouse",
        "Warehouse_primary",
        "warehouse-primary",
        "warehouse__primary",
        "_warehouse_primary",
        "warehouse_primary_",
    ] {
        assert_eq!(
            ObservationRequest::new(
                source_connection_key,
                vec!["public".to_owned()],
                request_budget(),
                limits(),
            ),
            Err(ObservationRequestError::InvalidSourceConnectionKey),
            "source connection keys must be opaque multiword snake_case registry identifiers: {source_connection_key}"
        );
    }

    let oversized_key = format!("source_{}", "a".repeat(122));
    assert_eq!(oversized_key.len(), 129);
    assert_eq!(
        ObservationRequest::new(
            oversized_key,
            vec!["public".to_owned()],
            request_budget(),
            limits(),
        ),
        Err(ObservationRequestError::InvalidSourceConnectionKey)
    );
}

#[test]
fn request_rejects_blank_source_empty_or_blank_schema_and_exact_duplicates() {
    assert_eq!(
        ObservationRequest::new("  ", vec!["public".to_owned()], request_budget(), limits(),),
        Err(ObservationRequestError::InvalidSourceConnectionKey)
    );
    assert_eq!(
        ObservationRequest::new("source_ref", Vec::new(), request_budget(), limits()),
        Err(ObservationRequestError::EmptySchemaAllowlist)
    );
    assert_eq!(
        ObservationRequest::new(
            "source_ref",
            vec!["\t".to_owned()],
            request_budget(),
            limits(),
        ),
        Err(ObservationRequestError::InvalidSchemaName)
    );
    assert_eq!(
        ObservationRequest::new(
            "source_ref",
            vec!["public".to_owned(), "public".to_owned()],
            request_budget(),
            limits(),
        ),
        Err(ObservationRequestError::DuplicateSchemaName {
            schema_name: "public".to_owned(),
        })
    );
}

struct ExactRegistry;

impl SourceConnectionRegistry for ExactRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "grc_readonly_connection").then(|| "policy_revision_a".to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "grc_readonly_connection"
            && source_connection.connection_policy_binding() == "policy_revision_a"
            && allowed_schema_names.len() == 1
            && allowed_schema_names[0] == "governance_core"
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        let request_budget = resource_envelope.request_budget();
        let limits = resource_envelope.limits();
        source_connection.source_connection_key() == "grc_readonly_connection"
            && source_connection.connection_policy_binding() == "policy_revision_a"
            && request_budget.max_schema_count() <= 8
            && request_budget.max_schema_bytes() <= 512
            && limits.operation_timeout_ms() <= 2_500
            && limits.statement_timeout_ms() <= 2_500
            && limits.max_rows() <= 5_000
            && limits.max_bytes() <= 1_048_576
            && limits.max_concurrent_queries() <= 2
    }
}

struct DenyRegistry;

impl SourceConnectionRegistry for DenyRegistry {
    fn contains_source_connection(&self, _source_connection_key: &str) -> bool {
        false
    }
}

#[test]
fn adapter_execution_requires_a_registry_authorized_request() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        request_budget(),
        limits(),
    )
    .expect("valid request metadata");

    assert_eq!(
        request.clone().authorize(&DenyRegistry),
        Err(ObservationRequestError::UnknownSourceConnectionKey)
    );

    let authorized = request
        .authorize(&ExactRegistry)
        .expect("registry authorization must issue the source-policy-schema-and-resource execution capability");
    assert_eq!(
        authorized.request().source_connection_key(),
        "grc_readonly_connection"
    );
    assert_eq!(
        authorized.source_connection().source_connection_key(),
        "grc_readonly_connection"
    );
    assert_eq!(
        authorized.source_connection().connection_policy_binding(),
        "policy_revision_a"
    );
}

struct Cancellation(bool);

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

struct EchoPort;

impl SourceObservationPort for EchoPort {
    type Snapshot = String;

    async fn observe<'a>(
        &'a self,
        request: AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        if cancellation.is_cancelled() {
            return Err(SourceObservationFailure::Cancelled);
        }
        Ok(format!(
            "{}:{}",
            request.source_connection().source_connection_key(),
            request.source_connection().connection_policy_binding()
        ))
    }
}

fn poll_ready<F: Future>(future: F) -> F::Output {
    let mut context = Context::from_waker(Waker::noop());
    let mut future = std::pin::pin!(future);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("synthetic adapter unexpectedly required an external wakeup"),
    }
}

#[test]
fn explicit_port_carries_authorization_and_cancellation_without_inventing_success() {
    let cancelled_request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        request_budget(),
        limits(),
    )
    .expect("valid request")
    .authorize(&ExactRegistry)
    .expect("authorized request");
    let active_request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        request_budget(),
        limits(),
    )
    .expect("valid request")
    .authorize(&ExactRegistry)
    .expect("authorized request");

    assert_eq!(
        poll_ready(EchoPort.observe(cancelled_request, &Cancellation(true))),
        Err(SourceObservationFailure::Cancelled)
    );
    assert_eq!(
        poll_ready(EchoPort.observe(active_request, &Cancellation(false))),
        Ok("grc_readonly_connection:policy_revision_a".to_owned())
    );

    let bounded_failures = [
        SourceObservationFailure::SourceUnavailable,
        SourceObservationFailure::OperationTimeout,
        SourceObservationFailure::StatementTimeout,
        SourceObservationFailure::InvalidCapturedMetadata,
        SourceObservationFailure::RowLimitExceeded { max_rows: 5_000 },
        SourceObservationFailure::ByteLimitExceeded {
            max_bytes: 1_048_576,
        },
        SourceObservationFailure::ConcurrencyLimitExceeded {
            max_concurrent_queries: 2,
        },
    ];
    assert_eq!(bounded_failures.len(), 7);
}
