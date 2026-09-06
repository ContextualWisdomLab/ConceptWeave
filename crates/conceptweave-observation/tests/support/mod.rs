use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const TEST_POLICY_BINDING: &str = "fixture_policy_revision_a";

struct ExactRegistry<'a> {
    source_connection_key: &'a str,
    allowed_schema_names: &'a [&'a str],
}

impl SourceConnectionRegistry for ExactRegistry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.source_connection_key
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == self.source_connection_key)
            .then(|| TEST_POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == self.source_connection_key
            && source_connection.connection_policy_binding() == TEST_POLICY_BINDING
            && allowed_schema_names
                .iter()
                .all(|schema_name| self.allowed_schema_names.contains(&schema_name.as_str()))
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        let request_budget = resource_envelope.request_budget();
        let limits = resource_envelope.limits();
        source_connection.source_connection_key() == self.source_connection_key
            && source_connection.connection_policy_binding() == TEST_POLICY_BINDING
            && request_budget.max_schema_count() <= 8
            && request_budget.max_schema_bytes() <= 512
            && limits.operation_timeout_ms() <= 1_000
            && limits.statement_timeout_ms() <= 1_000
            && limits.max_rows() <= 10
            && limits.max_bytes() <= 1_024
            && limits.max_concurrent_queries() <= 1
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
