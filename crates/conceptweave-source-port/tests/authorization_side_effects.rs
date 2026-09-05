use std::cell::Cell;

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
    adapter_invocations: Cell<usize>,
    source_accesses: Cell<usize>,
    snapshot_constructions: Cell<usize>,
}

impl SourceObservationPort for CountedObservationPort {
    type Snapshot = String;

    fn observe(
        &self,
        request: &AuthorizedObservationRequest,
        cancellation: &dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        self.adapter_invocations
            .set(self.adapter_invocations.get() + 1);

        if cancellation.is_cancelled() {
            return Err(SourceObservationFailure::Cancelled);
        }

        self.source_accesses.set(self.source_accesses.get() + 1);
        let snapshot = request
            .source_connection()
            .source_connection_key()
            .to_owned();
        self.snapshot_constructions
            .set(self.snapshot_constructions.get() + 1);
        Ok(snapshot)
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
    assert_eq!(port.adapter_invocations.get(), 0);
    assert_eq!(port.source_accesses.get(), 0);
    assert_eq!(port.snapshot_constructions.get(), 0);

    let authorized = request
        .authorize(&ExactRegistry)
        .expect("known registry key must issue the execution capability");
    assert_eq!(
        port.observe(&authorized, &Cancellation(false)),
        Ok("grc_readonly_connection".to_owned())
    );
    assert_eq!(port.adapter_invocations.get(), 1);
    assert_eq!(port.source_accesses.get(), 1);
    assert_eq!(port.snapshot_constructions.get(), 1);
}
