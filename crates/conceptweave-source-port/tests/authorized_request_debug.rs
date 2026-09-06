use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationResourceEnvelope,
    ResolvedSourceConnection, SourceConnectionRegistry,
};

struct Registry;

impl SourceConnectionRegistry for Registry {
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
            && allowed_schema_names == ["governance_core"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == "grc_readonly_connection"
            && source_connection.connection_policy_binding() == "policy_revision_a"
            && resource_envelope.request_budget()
                == ObservationRequestBudget::new(4, 256).expect("bounded request metadata")
            && resource_envelope.limits()
                == ObservationLimits::new(1_000, 10, 1_024, 1).expect("bounded limits")
    }
}

#[test]
fn authorized_request_debug_does_not_expose_private_monotonic_start_coordinate() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["governance_core".to_owned()],
        ObservationRequestBudget::new(4, 256).expect("bounded request metadata"),
        ObservationLimits::new(1_000, 10, 1_024, 1).expect("bounded limits"),
    )
    .expect("valid observation request");
    let authorized = request
        .authorize(&Registry)
        .expect("source, schema scope and resource envelope are authorized");

    let debug = format!("{authorized:?}");
    assert!(
        !debug.contains("operation_started_at") && !debug.contains("Instant"),
        "the private monotonic operation-start coordinate must not be exposed through Debug: {debug}"
    );
}
