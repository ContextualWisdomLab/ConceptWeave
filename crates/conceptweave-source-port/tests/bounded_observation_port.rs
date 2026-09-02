use conceptweave_source_port::{
    ObservationCancellation, ObservationLimitError, ObservationLimits, ObservationRequest,
    ObservationRequestError, SourceObservationFailure, SourceObservationPort,
};

fn limits() -> ObservationLimits {
    ObservationLimits::new(2_500, 5_000, 1_048_576, 2).expect("bounded limits")
}

#[test]
fn limits_preserve_timeout_row_byte_and_concurrency_bounds() {
    let limits = limits();

    assert_eq!(limits.statement_timeout_ms(), 2_500);
    assert_eq!(limits.max_rows(), 5_000);
    assert_eq!(limits.max_bytes(), 1_048_576);
    assert_eq!(limits.max_concurrent_queries(), 2);
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
fn request_preserves_exact_source_reference_and_canonicalizes_allowlist_only_by_order() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["Risk-Core".to_owned(), "Audit/Event".to_owned()],
        limits(),
    )
    .expect("valid request");

    assert_eq!(request.source_connection_key(), "grc_readonly_connection");
    assert_eq!(request.allowed_schema_names(), ["Audit/Event", "Risk-Core"]);
    assert_eq!(request.limits(), limits());
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
            ObservationRequest::new(source_connection_key, vec!["public".to_owned()], limits(),),
            Err(ObservationRequestError::InvalidSourceConnectionKey),
            "source connection keys must be opaque multiword snake_case registry identifiers: {source_connection_key}"
        );
    }

    let oversized_key = format!("source_{}", "a".repeat(122));
    assert_eq!(oversized_key.len(), 129);
    assert_eq!(
        ObservationRequest::new(oversized_key, vec!["public".to_owned()], limits()),
        Err(ObservationRequestError::InvalidSourceConnectionKey)
    );
}

#[test]
fn request_rejects_blank_source_empty_or_blank_schema_and_exact_duplicates() {
    assert_eq!(
        ObservationRequest::new("  ", vec!["public".to_owned()], limits()),
        Err(ObservationRequestError::InvalidSourceConnectionKey)
    );
    assert_eq!(
        ObservationRequest::new("source_ref", Vec::new(), limits()),
        Err(ObservationRequestError::EmptySchemaAllowlist)
    );
    assert_eq!(
        ObservationRequest::new("source_ref", vec!["\t".to_owned()], limits()),
        Err(ObservationRequestError::InvalidSchemaName)
    );
    assert_eq!(
        ObservationRequest::new(
            "source_ref",
            vec!["public".to_owned(), "public".to_owned()],
            limits(),
        ),
        Err(ObservationRequestError::DuplicateSchemaName {
            schema_name: "public".to_owned(),
        })
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

    fn observe(
        &self,
        request: &ObservationRequest,
        cancellation: &dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        if cancellation.is_cancelled() {
            return Err(SourceObservationFailure::Cancelled);
        }
        Ok(request.source_connection_key().to_owned())
    }
}

#[test]
fn explicit_port_carries_caller_cancellation_without_inventing_success() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        limits(),
    )
    .expect("valid request");

    assert_eq!(
        EchoPort.observe(&request, &Cancellation(true)),
        Err(SourceObservationFailure::Cancelled)
    );
    assert_eq!(
        EchoPort.observe(&request, &Cancellation(false)),
        Ok("grc_readonly_connection".to_owned())
    );

    let bounded_failures = [
        SourceObservationFailure::SourceUnavailable,
        SourceObservationFailure::StatementTimeout,
        SourceObservationFailure::RowLimitExceeded { max_rows: 5_000 },
        SourceObservationFailure::ByteLimitExceeded {
            max_bytes: 1_048_576,
        },
        SourceObservationFailure::ConcurrencyLimitExceeded {
            max_concurrent_queries: 2,
        },
    ];
    assert_eq!(bounded_failures.len(), 5);
}
