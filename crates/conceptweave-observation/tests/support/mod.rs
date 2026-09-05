use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ResolvedSourceConnection,
    SourceConnectionRegistry,
};

struct ExactRegistry<'a>(&'a str);

impl SourceConnectionRegistry for ExactRegistry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.0
    }
}

pub fn resolved_source(source_connection_key: &str) -> ResolvedSourceConnection {
    ObservationRequest::new(
        source_connection_key,
        vec!["public".to_owned()],
        ObservationRequestBudget::new(4, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .resolve_source_connection(&ExactRegistry(source_connection_key))
    .unwrap()
}
