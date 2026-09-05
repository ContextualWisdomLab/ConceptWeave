use std::{
    future::Future,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, ObservationLimits, ObservationRequest,
    ObservationRequestBudget, SourceConnectionRegistry, SourceObservationFailure,
    SourceObservationPort,
};

struct ExactRegistry;

impl SourceConnectionRegistry for ExactRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
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
        request: &'a AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> impl Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a {
        async move {
            if cancellation.is_cancelled() {
                return Err(SourceObservationFailure::Cancelled);
            }
            Ok(request
                .source_connection()
                .source_connection_key()
                .to_owned())
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
fn source_port_accepts_a_send_awaitable_adapter_without_a_runtime_dependency() {
    let request = authorized_request();

    let cancelled = assert_send(AsyncEchoPort.observe(&request, &Cancellation(true)));
    assert_eq!(
        poll_ready(cancelled),
        Err(SourceObservationFailure::Cancelled)
    );

    let completed = assert_send(AsyncEchoPort.observe(&request, &Cancellation(false)));
    assert_eq!(
        poll_ready(completed),
        Ok("grc_readonly_connection".to_owned())
    );
}
