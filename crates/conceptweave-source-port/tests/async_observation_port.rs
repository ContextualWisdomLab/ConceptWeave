use std::{
    future::Future,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, ObservationResourceEnvelope, ResolvedSourceConnection,
    SourceConnectionRegistry, SourceObservationFailure, SourceObservationPort,
};

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
            && limits.operation_timeout_ms() <= 10_000
            && limits.statement_timeout_ms() <= 2_500
            && limits.max_rows() <= 5_000
            && limits.max_bytes() <= 1_048_576
            && limits.max_concurrent_queries() <= 2
    }
}

struct Cancellation(bool);

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

struct AsyncEchoPort;

impl SourceObservationPort for AsyncEchoPort {
    type Snapshot = String;

    fn observe<'a>(
        &'a self,
        request: AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a {
        async move {
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
}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn assert_send<T: Send>(value: T) -> T {
    value
}

fn poll_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("synthetic adapter unexpectedly required an external wakeup"),
    }
}

fn authorized_request() -> AuthorizedObservationRequest {
    let limits = ObservationLimits::with_timeouts(10_000, 2_500, 5_000, 1_048_576, 2)
        .expect("bounded limits");
    let request_budget = ObservationRequestBudget::new(8, 512).expect("bounded metadata");

    ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        request_budget,
        limits,
    )
    .expect("valid request")
    .authorize(&ExactRegistry)
    .expect("authorized request")
}

#[test]
fn source_port_consumes_one_authorized_operation_capability_per_execution() {
    let cancelled_request = authorized_request();
    let active_request = authorized_request();
    let cancelled_signal = Cancellation(true);
    let active_signal = Cancellation(false);

    let cancelled = assert_send(AsyncEchoPort.observe(cancelled_request, &cancelled_signal));
    assert_eq!(
        poll_ready(cancelled),
        Err(SourceObservationFailure::Cancelled)
    );

    let completed = assert_send(AsyncEchoPort.observe(active_request, &active_signal));
    assert_eq!(
        poll_ready(completed),
        Ok("grc_readonly_connection:policy_revision_a".to_owned())
    );
}
