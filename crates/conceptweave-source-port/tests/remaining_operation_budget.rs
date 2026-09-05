use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    thread,
    time::Duration,
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, ObservationRequestError, SourceConnectionRegistry,
    SourceObservationFailure, SourceObservationPort,
};

fn request_with_key(source_connection_key: &str, operation_timeout_ms: u64) -> ObservationRequest {
    ObservationRequest::new(
        source_connection_key,
        vec!["governance_core".to_owned()],
        ObservationRequestBudget::new(8, 512).expect("bounded request metadata"),
        ObservationLimits::with_timeouts(operation_timeout_ms, 5, 5_000, 1_048_576, 2)
            .expect("bounded observation limits"),
    )
    .expect("valid observation request")
}

fn request(operation_timeout_ms: u64) -> ObservationRequest {
    request_with_key("grc_readonly_connection", operation_timeout_ms)
}

struct DelayedRegistry {
    delay: Duration,
}

impl SourceConnectionRegistry for DelayedRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        thread::sleep(self.delay);
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
    type Snapshot = Duration;

    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        _cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a {
        async move {
            self.adapter_invocations.fetch_add(1, Ordering::Relaxed);
            let Some(remaining) = request.remaining_operation_budget() else {
                return Err(SourceObservationFailure::OperationTimeout);
            };
            self.source_accesses.fetch_add(1, Ordering::Relaxed);
            self.snapshot_constructions.fetch_add(1, Ordering::Relaxed);
            Ok(remaining)
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
fn registry_authorization_consumes_the_same_operation_budget_seen_by_the_adapter() {
    let port = CountedObservationPort::default();
    let authorized = request(250)
        .authorize(&DelayedRegistry {
            delay: Duration::from_millis(20),
        })
        .expect("authorization must complete inside the operation budget");

    let remaining = poll_ready(port.observe(&authorized, &Cancellation))
        .expect("adapter must receive the unexpired remainder");

    assert!(remaining <= Duration::from_millis(230));
    assert!(remaining > Duration::ZERO);
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 1);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 1);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 1);
}

#[test]
fn exhausted_authorization_fails_before_adapter_source_or_snapshot_side_effects() {
    let port = CountedObservationPort::default();
    let authorization = request(5).authorize(&DelayedRegistry {
        delay: Duration::from_millis(20),
    });

    assert_eq!(authorization, Err(ObservationRequestError::OperationTimeout));
    assert_eq!(port.adapter_invocations.load(Ordering::Relaxed), 0);
    assert_eq!(port.source_accesses.load(Ordering::Relaxed), 0);
    assert_eq!(port.snapshot_constructions.load(Ordering::Relaxed), 0);
}

#[test]
fn elapsed_budget_takes_precedence_after_a_slow_unknown_registry_lookup() {
    let authorization = request_with_key("unknown_readonly_connection", 5).authorize(
        &DelayedRegistry {
            delay: Duration::from_millis(20),
        },
    );

    assert_eq!(authorization, Err(ObservationRequestError::OperationTimeout));
}
