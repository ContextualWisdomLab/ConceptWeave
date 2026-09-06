use std::{
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, ResolvedSourceConnection, SourceConnectionRegistry,
    SourceObservationFailure, SourceObservationPort,
};

struct MutableRegistry {
    active_binding: Arc<Mutex<&'static str>>,
}

impl SourceConnectionRegistry for MutableRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "grc_readonly_connection")
            .then(|| (*self.active_binding.lock().expect("binding lock")).to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "grc_readonly_connection"
            && source_connection.connection_policy_binding()
                == *self.active_binding.lock().expect("binding lock")
            && allowed_schema_names == ["governance_core"]
    }
}

struct Cancellation;

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        false
    }
}

struct RetargetableAdapter {
    active_binding: Arc<Mutex<&'static str>>,
    source_accesses: AtomicUsize,
    snapshot_constructions: AtomicUsize,
}

impl SourceObservationPort for RetargetableAdapter {
    type Snapshot = String;

    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        _cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a {
        async move {
            let active_binding = *self.active_binding.lock().expect("binding lock");
            if request.source_connection().connection_policy_binding() != active_binding {
                return Err(SourceObservationFailure::SourceUnavailable);
            }

            self.source_accesses.fetch_add(1, Ordering::Relaxed);
            self.snapshot_constructions.fetch_add(1, Ordering::Relaxed);
            Ok(format!(
                "{}:{active_binding}",
                request.source_connection().source_connection_key()
            ))
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

fn request() -> ObservationRequest {
    ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        ObservationRequestBudget::new(4, 256).expect("bounded request metadata"),
        ObservationLimits::new(1_000, 10, 1_024, 1).expect("bounded limits"),
    )
    .expect("valid observation request")
}

#[test]
fn stale_connection_policy_binding_fails_before_source_or_snapshot_side_effects() {
    let active_binding = Arc::new(Mutex::new("policy_revision_a"));
    let registry = MutableRegistry {
        active_binding: Arc::clone(&active_binding),
    };
    let authorized = request()
        .authorize(&registry)
        .expect("revision A source and exact schema scope are authorized");
    assert_eq!(
        authorized.source_connection().connection_policy_binding(),
        "policy_revision_a"
    );

    *active_binding.lock().expect("binding lock") = "policy_revision_b";
    let adapter = RetargetableAdapter {
        active_binding,
        source_accesses: AtomicUsize::new(0),
        snapshot_constructions: AtomicUsize::new(0),
    };

    assert_eq!(
        poll_ready(adapter.observe(&authorized, &Cancellation)),
        Err(SourceObservationFailure::SourceUnavailable),
        "an authorization issued for policy revision A must not silently retarget to revision B"
    );
    assert_eq!(adapter.source_accesses.load(Ordering::Relaxed), 0);
    assert_eq!(adapter.snapshot_constructions.load(Ordering::Relaxed), 0);
}

#[test]
fn unchanged_connection_policy_binding_executes_exactly_once() {
    let active_binding = Arc::new(Mutex::new("policy_revision_a"));
    let registry = MutableRegistry {
        active_binding: Arc::clone(&active_binding),
    };
    let authorized = request()
        .authorize(&registry)
        .expect("revision A source and exact schema scope are authorized");
    let adapter = RetargetableAdapter {
        active_binding,
        source_accesses: AtomicUsize::new(0),
        snapshot_constructions: AtomicUsize::new(0),
    };

    assert_eq!(
        poll_ready(adapter.observe(&authorized, &Cancellation)),
        Ok("grc_readonly_connection:policy_revision_a".to_owned())
    );
    assert_eq!(adapter.source_accesses.load(Ordering::Relaxed), 1);
    assert_eq!(adapter.snapshot_constructions.load(Ordering::Relaxed), 1);
}
