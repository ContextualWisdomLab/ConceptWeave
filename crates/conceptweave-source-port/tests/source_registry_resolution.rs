use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationRequestError,
    SourceConnectionRegistry,
};

struct TestRegistry;

impl SourceConnectionRegistry for TestRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "grc_readonly_connection")
            .then(|| "policy_revision_a".to_owned())
    }
}

struct KeyOnlyRegistry;

impl SourceConnectionRegistry for KeyOnlyRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }
}

struct BlankBindingRegistry;

impl SourceConnectionRegistry for BlankBindingRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "grc_readonly_connection").then(|| "  ".to_owned())
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
fn registry_resolution_issues_key_and_policy_binding_only_for_a_registered_source() {
    let identity = request("grc_readonly_connection")
        .resolve_source_connection(&TestRegistry)
        .unwrap();
    assert_eq!(identity.source_connection_key(), "grc_readonly_connection");
    assert_eq!(identity.connection_policy_binding(), "policy_revision_a");

    assert_eq!(
        request("password_hunter2").resolve_source_connection(&TestRegistry),
        Err(ObservationRequestError::UnknownSourceConnectionKey)
    );
}

#[test]
fn known_source_without_an_immutable_policy_binding_fails_closed() {
    assert_eq!(
        request("grc_readonly_connection").resolve_source_connection(&KeyOnlyRegistry),
        Err(ObservationRequestError::MissingConnectionPolicyBinding)
    );
    assert_eq!(
        request("grc_readonly_connection").resolve_source_connection(&BlankBindingRegistry),
        Err(ObservationRequestError::InvalidConnectionPolicyBinding)
    );
}
