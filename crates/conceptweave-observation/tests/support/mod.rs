use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    SourceConnectionRegistry,
};

struct ExactRegistry<'a> {
    source_connection_key: &'a str,
    allowed_schema_names: &'a [&'a str],
}

impl SourceConnectionRegistry for ExactRegistry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.source_connection_key
    }

    fn authorizes_schema_scope(
        &self,
        source_connection_key: &str,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection_key == self.source_connection_key
            && allowed_schema_names.iter().all(|schema_name| {
                self.allowed_schema_names
                    .iter()
                    .any(|allowed| *allowed == schema_name.as_str())
            })
    }
}

pub fn authorized_source(
    source_connection_key: &str,
    allowed_schema_names: &[&str],
) -> AuthorizedObservationRequest {
    ObservationRequest::new(
        source_connection_key,
        allowed_schema_names
            .iter()
            .map(|schema_name| (*schema_name).to_owned())
            .collect(),
        ObservationRequestBudget::new(8, 512).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&ExactRegistry {
        source_connection_key,
        allowed_schema_names,
    })
    .unwrap()
}

pub fn resolved_source(source_connection_key: &str) -> AuthorizedObservationRequest {
    authorized_source(source_connection_key, &["Sales/~North", "audit", "public"])
}
