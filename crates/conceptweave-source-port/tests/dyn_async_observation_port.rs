use std::{
    future::Future,
    pin::Pin,
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

struct Cancellation;

impl ObservationCancellation for Cancellation {
    fn is_cancelled(&self) -> bool {
        false
    }
}

struct EchoPort;

impl SourceObservationPort for EchoPort {
    type Snapshot = String;

    fn observe<'a>(
        &'a self,
        request: &'a AuthorizedObservationRequest,
        _cancellation: &'a dyn ObservationCancellation,
    ) -> Pin<
        Box<dyn Future<Output = Result<Self::Snapshot, SourceObservationFailure>> + Send + 'a>,
    > {
        Box::pin(async move {
            Ok(request
                .source_connection()
                .source_connection_key()
                .to_owned())
        })
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

fn execute_through_port_object(
    port: &dyn SourceObservationPort<Snapshot = String>,
    request: &AuthorizedObservationRequest,
) -> Result<String, SourceObservationFailure> {
    poll_ready(port.observe(request, &Cancellation))
}

#[test]
fn awaitable_source_port_preserves_dynamic_adapter_dispatch() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        ObservationRequestBudget::new(8, 512).expect("bounded metadata"),
        ObservationLimits::with_timeouts(10_000, 2_500, 5_000, 1_048_576, 2)
            .expect("bounded limits"),
    )
    .expect("valid request")
    .authorize(&ExactRegistry)
    .expect("authorized request");

    let port = EchoPort;
    assert_eq!(
        execute_through_port_object(&port, &request),
        Ok("grc_readonly_connection".to_owned())
    );
}
