use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintAccessMethodCapabilityObservation,
    IndexExclusionConstraintAccessMethodCapabilitySnapshot, IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintObservation, IndexExclusionConstraintSnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    RelationPartitionObservation, RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";
const CONSTRAINT_NAME: &str = "bookings_no_overlap";

struct Registry;
impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool {
        key == "warehouse_primary"
    }

    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "warehouse_primary").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(&self, source: &ResolvedSourceConnection, schemas: &[String]) -> bool {
        source.source_connection_key() == "warehouse_primary"
            && source.connection_policy_binding() == POLICY_BINDING
            && schemas == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source: &ResolvedSourceConnection,
        envelope: ObservationResourceEnvelope,
    ) -> bool {
        source.source_connection_key() == "warehouse_primary"
            && source.connection_policy_binding() == POLICY_BINDING
            && envelope.request_budget().max_schema_count() <= 1
            && envelope.request_budget().max_schema_bytes() <= 256
            && envelope.limits().operation_timeout_ms() <= 1_000
            && envelope.limits().statement_timeout_ms() <= 1_000
            && envelope.limits().max_rows() <= 10
            && envelope.limits().max_bytes() <= 1_024
            && envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    ObservationRequest::new(
        "warehouse_primary",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn exclusion_index(access_method: &str) -> IndexObservation {
    IndexObservation::new(
        CONSTRAINT_NAME,
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method(access_method)
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn base_snapshot(access_method: &str) -> PostgresSchemaSnapshotV3 {
    let relation = RelationObservation::new(
        "public",
        "bookings",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "id",
            1,
            "bigint",
            QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
            false,
            None,
        )
        .unwrap()],
    )
    .unwrap()
    .with_indexes(vec![exclusion_index(access_method)])
    .unwrap();

    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-am-capability-v1",
        "2026-09-16T14:50:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap()
}

fn relation_partitions(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(
        base,
        vec![RelationPartitionObservation::non_partition(
            "public",
            "bookings",
            RelationKind::Table,
        )
        .unwrap()],
    )
    .unwrap()
}

fn backing_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        CONSTRAINT_NAME,
    )
    .unwrap()
}

fn index_partitions(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> IndexPartitionSnapshot {
    IndexPartitionSnapshot::new(
        base,
        relations,
        vec![IndexPartitionObservation::non_partition(
            backing_index(),
            IndexRelationKind::Index,
        )
        .unwrap()],
    )
    .unwrap()
}

fn constraint_coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        CONSTRAINT_NAME,
    )
    .unwrap()
}

fn exclusion_constraints(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
    indexes: &IndexPartitionSnapshot,
) -> IndexExclusionConstraintSnapshot {
    IndexExclusionConstraintSnapshot::new(
        base,
        relations,
        indexes,
        vec![IndexExclusionConstraintObservation::root(
            constraint_coordinate(),
            backing_index(),
        )
        .unwrap()],
    )
    .unwrap()
}

fn build_capability(
    base_access_method: &str,
    observed_access_method: &str,
    can_exclude: bool,
) -> Result<IndexExclusionConstraintAccessMethodCapabilitySnapshot, ObservationError> {
    let base = base_snapshot(base_access_method);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    IndexExclusionConstraintAccessMethodCapabilitySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        vec![IndexExclusionConstraintAccessMethodCapabilityObservation::new(
            constraint_coordinate(),
            backing_index(),
            observed_access_method,
            can_exclude,
        )
        .unwrap()],
    )
}

#[test]
fn ordinary_exclusion_rejects_access_method_without_exclusion_capability() {
    let error = build_capability("gin", "gin", false)
        .expect_err("GIN-style can_exclude=false evidence must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_capability_state",
        }
    );
}

#[test]
fn ordinary_exclusion_rejects_access_method_binding_drift() {
    let error = build_capability("gist", "gin", true)
        .expect_err("capability evidence must bind to the exact backing-index access method");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_capability_binding",
        }
    );
}

#[test]
fn ordinary_exclusion_rejects_backing_index_binding_drift() {
    let base = base_snapshot("gist");
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    let wrong_backing = IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "other_index",
    )
    .unwrap();
    let error = IndexExclusionConstraintAccessMethodCapabilitySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        vec![IndexExclusionConstraintAccessMethodCapabilityObservation::new(
            constraint_coordinate(),
            wrong_backing,
            "gist",
            true,
        )
        .unwrap()],
    )
    .expect_err("capability evidence must bind to the exact conindid backing index");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_capability_binding",
        }
    );
}

#[test]
fn capability_observation_rejects_blank_access_method_name() {
    let error = IndexExclusionConstraintAccessMethodCapabilityObservation::new(
        constraint_coordinate(),
        backing_index(),
        "  ",
        true,
    )
    .expect_err("blank access-method identity cannot become governed evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_name",
        }
    );
}

#[test]
fn ordinary_exclusion_requires_complete_capability_evidence() {
    let base = base_snapshot("gist");
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    let error = IndexExclusionConstraintAccessMethodCapabilitySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        vec![],
    )
    .expect_err("every ordinary EXCLUDE constraint needs one explicit can_exclude observation");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_capability_completeness",
        }
    );
}

#[test]
fn ordinary_exclusion_rejects_duplicate_capability_coordinates() {
    let base = base_snapshot("gist");
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    let observation = IndexExclusionConstraintAccessMethodCapabilityObservation::new(
        constraint_coordinate(),
        backing_index(),
        "gist",
        true,
    )
    .unwrap();
    let error = IndexExclusionConstraintAccessMethodCapabilitySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate capability coordinates must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_access_method_capability_coordinate",
        }
    );
}

#[test]
fn extension_access_method_is_admitted_from_observed_capability_not_name_allowlist() {
    let snapshot = build_capability("acme_exclusion_am", "acme_exclusion_am", true)
        .expect("extension access methods are valid when can_exclude is observed true");
    let observation = &snapshot.observations()[0];
    assert_eq!(observation.access_method_name(), "acme_exclusion_am");
    assert!(observation.can_exclude());

    let receipt = snapshot
        .source_receipt(constraint_coordinate())
        .expect("validated capability evidence must issue provenance");
    assert_eq!(receipt.location().backing_index(), &backing_index());
    assert_eq!(receipt.location().access_method_name(), "acme_exclusion_am");
    assert!(receipt.location().can_exclude());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn capability_receipt_rejects_unknown_constraint_coordinate() {
    let snapshot = build_capability("gist", "gist", true).unwrap();
    let unknown = IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "unobserved_exclusion",
    )
    .unwrap();
    let error = snapshot
        .source_receipt(unknown)
        .expect_err("unobserved constraint capability cannot issue provenance");
    assert!(matches!(error, ObservationError::UnknownObservationLocation { .. }));
}
