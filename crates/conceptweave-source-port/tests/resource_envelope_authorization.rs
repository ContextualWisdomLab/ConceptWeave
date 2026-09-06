use std::{
    future::Future,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, ObservationRequestError, ObservationResourceEnvelope,
    ResolvedSourceConnection, SourceConnectionRegistry, SourceObservationFailure,
    SourceObservationPort,
};

const SOURCE_KEY: &str = "grc_readonly_connection";
const POLICY_BINDING: &str = "policy_revision_a";

fn request(
    request_budget: ObservationRequestBudget,
    limits: ObservationLimits,
) -> ObservationRequest {
    ObservationRequest::new(
        SOURCE_KEY,
        vec!["governance_core".to_owned()],
        request_budget,
        limits,
    )
    .expect("valid observation request")
}

fn source_and_schema_match(
    source_connection: &ResolvedSourceConnection,
    allowed_schema_names: &[String],
) -> bool {
    source_connection.source_connection_key() == SOURCE_KEY
        && source_connection.connection_policy_binding() == POLICY_BINDING
        && allowed_schema_names == ["governance_core"]
}

struct SchemaOnlyRegistry;

impl SourceConnectionRegistry for SchemaOnlyRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == SOURCE_KEY
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == SOURCE_KEY).then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_and_schema_match(source_connection, allowed_schema_names)
    }
}

struct CappedRegistry;

impl SourceConnectionRegistry for CappedRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == SOURCE_KEY
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == SOURCE_KEY).then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_and_schema_match(source_connection, allowed_schema_names)
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        if source_connection.source_connection_key() != SOURCE_KEY
            || source_connection.connection_policy_binding() != POLICY_BINDING
        {
            return false;
        }

        let request_budget = resource_envelope.request_budget();
        let limits = resource_envelope.limits();
        request_budget.max_schema_count() <= 4
            && request_budget.max_schema_bytes() <= 256
            && limits.operation_timeout_ms() <= 5_000
            && limits.statement_timeout_ms() <= 2_500
            && limits.max_rows() <= 5_000
            && limits.max_bytes() <= 1_048_576
            && limits.max_concurrent_queries() <= 2
    }
}

struct Cancellation;

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        false
    }
}

#[derive(Default)]
struct CountedObservationPort {
    adapter_invocations: AtomicUsize,
    source_accesses: AtomicUsize,
    snapshot_constructions: AtomicUsize,
}

impl SourceObservationPort for CountedObservationPort {
    type Snapshot = ObservationResourceEnvelope;

    async fn observe<'a>(
        &'a self,
        request: AuthorizedObservationRequest,
        _cancellation: &'a dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        self.adapter_invocations.fetch_add(1, Ordering::Relaxed);
        self.source_accesses.fetch_add(1, Ordering::Relaxed);
        let resource_envelope = request.request().resource_envelope();
        self.snapshot_constructions.fetch_add(1, Ordering::Relaxed);
        Ok(resource_envelope)
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
fn schema_authorization_without_trusted_resource_policy_fails_closed() {
    let authorization = request(
        ObservationRequestBudget::new(4, 256).expect("bounded request metadata"),
        ObservationLimits::with_timeouts(5_000, 2_500, 5_000, 1_048_576, 2)
            .expect("bounded observation limits"),
    )
    .authorize(&SchemaOnlyRegistry);

    assert_eq!(
        authorization,
        Err(ObservationRequestError::UnauthorizedResourceEnvelope)
    );
}

#[test]
fn wider_than_policy_resource_envelope_fails_before_adapter_source_or_snapshot_side_effects() {
    let port = CountedObservationPort::default();
    let authorization = request(
        ObservationRequestBudget::new(8, 512).expect("caller-selected request metadata"),
        ObservationLimits::with_timeouts(10_000, 5_000, 10_000, 2_097_152, 4)
            .expect("caller-selected observation limits"),
    )
    .authorize(&CappedRegistry);

    assert_eq!(
        authorization,
        Err(ObservationRequestError::UnauthorizedResourceEnvelope)
    );
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 0);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 0);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 0);
}

#[test]
fn equal_and_narrower_resource_envelopes_are_explicitly_admitted() {
    let equal_budget = ObservationRequestBudget::new(4, 256).expect("policy ceiling metadata");
    let equal_limits = ObservationLimits::with_timeouts(5_000, 2_500, 5_000, 1_048_576, 2)
        .expect("policy ceiling limits");
    let equal = request(equal_budget, equal_limits)
        .authorize(&CappedRegistry)
        .expect("equal policy envelope is admitted");
    assert_eq!(
        equal.request().resource_envelope(),
        ObservationResourceEnvelope::new(equal_budget, equal_limits)
    );

    let narrower_budget = ObservationRequestBudget::new(2, 64).expect("narrower metadata budget");
    let narrower_limits = ObservationLimits::with_timeouts(1_000, 500, 100, 4_096, 1)
        .expect("narrower observation limits");
    let narrower = request(narrower_budget, narrower_limits)
        .authorize(&CappedRegistry)
        .expect("narrower policy envelope is admitted");

    let port = CountedObservationPort::default();
    assert_eq!(
        poll_ready(port.observe(narrower, &Cancellation)),
        Ok(ObservationResourceEnvelope::new(
            narrower_budget,
            narrower_limits,
        ))
    );
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 1);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 1);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 1);
}
