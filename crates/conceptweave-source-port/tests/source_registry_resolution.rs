use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationRequestError,
    SourceConnectionRegistry,
};

struct TestRegistry;

impl SourceConnectionRegistry for TestRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }
}

fn request(source_connection_key: &str) -> ObservationRequest {
    ObservationRequest::new(
        source_connection_key,
        vec!["public".to_owned()],
        ObservationRequestBudget::new(4, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
}

#[test]
fn registry_resolution_issues_identity_only_for_a_registered_source() {
    let identity = request("grc_readonly_connection")
        .resolve_source_connection(&TestRegistry)
        .unwrap();
    assert_eq!(identity.source_connection_key(), "grc_readonly_connection");

    assert_eq!(
        request("password_hunter2").resolve_source_connection(&TestRegistry),
        Err(ObservationRequestError::UnknownSourceConnectionKey)
    );
}
