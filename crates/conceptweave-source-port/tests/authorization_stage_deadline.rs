use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use conceptweave_source_port::{
    ObservationLimits, ObservationRequest, ObservationRequestBudget, ObservationRequestError,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const SOURCE_KEY: &str = "grc_readonly_connection";
const POLICY_BINDING: &str = "policy_revision_a";

fn request(operation_timeout_ms: u64) -> ObservationRequest {
    ObservationRequest::new(
        SOURCE_KEY,
        vec!["governance_core".to_owned()],
        ObservationRequestBudget::new(8, 512).expect("bounded request metadata"),
        ObservationLimits::with_timeouts(operation_timeout_ms, 5, 5_000, 1_048_576, 2)
            .expect("bounded observation limits"),
    )
    .expect("valid observation request")
}

#[derive(Default)]
struct SlowSourceLookupRegistry {
    binding_calls: AtomicUsize,
    schema_calls: AtomicUsize,
    resource_calls: AtomicUsize,
}

impl SourceConnectionRegistry for SlowSourceLookupRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        assert_eq!(source_connection_key, SOURCE_KEY);
        thread::sleep(Duration::from_millis(20));
        true
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        assert_eq!(source_connection_key, SOURCE_KEY);
        self.binding_calls.fetch_add(1, Ordering::Relaxed);
        Some(POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        _source_connection: &ResolvedSourceConnection,
        _allowed_schema_names: &[String],
    ) -> bool {
        self.schema_calls.fetch_add(1, Ordering::Relaxed);
        true
    }

    fn authorizes_resource_envelope(
        &self,
        _source_connection: &ResolvedSourceConnection,
        _resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        self.resource_calls.fetch_add(1, Ordering::Relaxed);
        true
    }
}

#[derive(Default)]
struct SlowBindingRegistry {
    schema_calls: AtomicUsize,
    resource_calls: AtomicUsize,
}

impl SourceConnectionRegistry for SlowBindingRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == SOURCE_KEY
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        assert_eq!(source_connection_key, SOURCE_KEY);
        thread::sleep(Duration::from_millis(20));
        Some(POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        _source_connection: &ResolvedSourceConnection,
        _allowed_schema_names: &[String],
    ) -> bool {
        self.schema_calls.fetch_add(1, Ordering::Relaxed);
        true
    }

    fn authorizes_resource_envelope(
        &self,
        _source_connection: &ResolvedSourceConnection,
        _resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        self.resource_calls.fetch_add(1, Ordering::Relaxed);
        true
    }
}

#[derive(Default)]
struct SlowSchemaRegistry {
    resource_calls: AtomicUsize,
}

impl SourceConnectionRegistry for SlowSchemaRegistry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == SOURCE_KEY
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == SOURCE_KEY).then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        assert_eq!(source_connection.source_connection_key(), SOURCE_KEY);
        assert_eq!(
            source_connection.connection_policy_binding(),
            POLICY_BINDING
        );
        assert_eq!(allowed_schema_names, ["governance_core"]);
        thread::sleep(Duration::from_millis(20));
        true
    }

    fn authorizes_resource_envelope(
        &self,
        _source_connection: &ResolvedSourceConnection,
        _resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        self.resource_calls.fetch_add(1, Ordering::Relaxed);
        true
    }
}

#[test]
fn expired_source_lookup_stops_before_later_registry_policy_stages() {
    let registry = SlowSourceLookupRegistry::default();

    assert_eq!(
        request(5).authorize(&registry),
        Err(ObservationRequestError::OperationTimeout)
    );
    assert_eq!(registry.binding_calls.load(Ordering::Relaxed), 0);
    assert_eq!(registry.schema_calls.load(Ordering::Relaxed), 0);
    assert_eq!(registry.resource_calls.load(Ordering::Relaxed), 0);
}

#[test]
fn expired_binding_lookup_stops_before_schema_and_resource_policy_stages() {
    let registry = SlowBindingRegistry::default();

    assert_eq!(
        request(5).authorize(&registry),
        Err(ObservationRequestError::OperationTimeout)
    );
    assert_eq!(registry.schema_calls.load(Ordering::Relaxed), 0);
    assert_eq!(registry.resource_calls.load(Ordering::Relaxed), 0);
}

#[test]
fn expired_schema_policy_stops_before_resource_policy_stage() {
    let registry = SlowSchemaRegistry::default();

    assert_eq!(
        request(5).authorize(&registry),
        Err(ObservationRequestError::OperationTimeout)
    );
    assert_eq!(registry.resource_calls.load(Ordering::Relaxed), 0);
}
