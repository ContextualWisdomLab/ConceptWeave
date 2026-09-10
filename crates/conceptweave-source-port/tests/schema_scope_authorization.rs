use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationRequestError,
    SourceConnectionRegistry,
};

struct SourceOnlyRegistry;

impl SourceConnectionRegistry for SourceOnlyRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "grc_readonly_connection"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "grc_readonly_connection").then(|| "policy_revision_a".to_owned())
    }
}

#[test]
fn source_key_and_binding_cannot_self_authorize_arbitrary_schema_scope() {
    let request = ObservationRequest::new(
        "grc_readonly_connection",
        vec!["restricted_finance".to_owned()],
        ObservationRequestBudget::new(4, 256).expect("bounded request metadata"),
        ObservationLimits::new(1_000, 10, 1_024, 1).expect("bounded observation limits"),
    )
    .expect("request metadata is syntactically valid");

    assert_eq!(
        request.authorize(&SourceOnlyRegistry),
        Err(ObservationRequestError::UnauthorizedSchemaScope),
        "authorizing only the source key and policy binding must not implicitly authorize a caller-selected schema scope"
    );
}
