use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, ObservationRequestError, SourceConnectionRegistry,
    SourceObservationFailure, SourceObservationPort,
};

fn limits() -> ObservationLimits {
    ObservationLimits::new(2_500, 5_000, 1_048_576, 2).expect("bounded limits")
}

fn request_budget() -> ObservationRequestBudget {
    ObservationRequestBudget::new(8, 512).expect("bounded request metadata")
}

struct ExactRegistry;

impl SourceConnectionRegistry for ExactRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn authorizes_schema_scope(
        &self,
        source_connection_key: &str,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection_key == "grc_readonly_connection"
            && allowed_schema_names.len() == 1
            && allowed_schema_names[0] == "governance_core"
    }
}

struct DenyRegistry;

impl SourceConnectionRegistry for DenyRegistry {
    fn contains_source_connection(&self, _source_connection_key: &str) -> bool {
        false
    }
}

struct Cancellation(bool);

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

#[derive(Default)]
struct CountedObservationPort {
    adapter_invocations: AtomicUsize,
    source_accesses: AtomicUsize,
    snapshot_constructions: AtomicUsize,
}

impl SourceObservationPort for CountedObservationPort {
    type Snapshot = String;

    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a {
        async move {
            self.adapter_invocations.fetch_add(1, Ordering::Relaxed);

            if cancellation.is_cancelled() {
                return Err(SourceObservationFailure::Cancelled);
            }

            self.source_accesses.fetch_add(1, Ordering::Relaxed);
            let snapshot = request
                .source_connection()
                .source_connection_key()
                .to_owned();
            self.snapshot_constructions.fetch_add(1, Ordering::Relaxed);
            Ok(snapshot)
        }
    }
}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
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

#[test]
fn denied_authorization_has_no_execution_side_effects_and_authorized_control_executes() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        request_budget(),
        limits(),
    )
    .expect("syntactically valid request metadata");
    let port = CountedObservationPort::default();

    let denied = request.clone().authorize(&DenyRegistry);
    let denied_execution = denied
        .as_ref()
        .ok()
        .map(|authorized| port.observe(authorized, &Cancellation(false)));

    assert_eq!(
        denied,
        Err(ObservationRequestError::UnknownSourceConnectionKey)
    );
    assert!(denied_execution.is_none());
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 0);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 0);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 0);

    let authorized = request
        .authorize(&ExactRegistry)
        .expect("known registry key and schema scope must issue the execution capability");
    assert_eq!(
        poll_ready(port.observe(&authorized, &Cancellation(false))),
        Ok("grc_readonly_connection".to_owned())
    );
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 1);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 1);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 1);
}
