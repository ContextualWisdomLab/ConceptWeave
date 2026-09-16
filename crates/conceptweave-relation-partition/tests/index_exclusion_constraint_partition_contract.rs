use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintObservation,
    IndexExclusionConstraintSnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, PartitionParentRelationCoordinate,
    RelationPartitionObservation, RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";
struct Registry;
impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool { key == "warehouse" }
    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "warehouse").then(|| POLICY_BINDING.to_owned())
    }
    fn authorizes_schema_scope(&self, source: &ResolvedSourceConnection, schemas: &[String]) -> bool {
        source.source_connection_key() == "warehouse"
            && source.connection_policy_binding() == POLICY_BINDING
            && schemas == ["public"]
    }
    fn authorizes_resource_envelope(&self, source: &ResolvedSourceConnection, envelope: ObservationResourceEnvelope) -> bool {
        source.source_connection_key() == "warehouse"
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
        "warehouse", vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    ).unwrap().authorize(&Registry).unwrap()
}

fn exclusion_index(name: &str) -> IndexObservation {
    IndexObservation::new(
        name, false, Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()], vec![],
    ).unwrap()
        .with_access_method("btree")
        .with_key_semantics(vec![IndexKeySemantics::new(
            1, None, QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(), 0,
        ).unwrap()]).unwrap()
        .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false)).unwrap()
        .with_ready(true).with_valid(true).with_live(true)
}

fn relation(name: &str, kind: RelationKind, index_name: &str) -> RelationObservation {
    RelationObservation::new(
        "public", name, kind,
        vec![ColumnObservationV3::new(
            "id", 1, "bigint", QualifiedTypeName::new("pg_catalog", "int8").unwrap(), false, None,
        ).unwrap()],
    ).unwrap().with_indexes(vec![exclusion_index(index_name)]).unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(), "extractor-index-exclusion-constraint-v1", "2026-09-16T02:25:00Z",
        vec![
            relation("bookings", RelationKind::PartitionedTable, "bookings_excl"),
            relation("bookings_2026", RelationKind::Table, "bookings_2026_excl"),
        ], vec![], vec![],
    ).unwrap()
}

fn relation_partitions(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(base, vec![
        RelationPartitionObservation::non_partition("public", "bookings", RelationKind::PartitionedTable).unwrap(),
        RelationPartitionObservation::partition(
            "public", "bookings_2026", RelationKind::Table,
            PartitionParentRelationCoordinate::new("public", "bookings").unwrap(), false,
        ).unwrap(),
    ]).unwrap()
}
fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new("public", "bookings", RelationKind::PartitionedTable, "bookings_excl").unwrap()
}
fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new("public", "bookings_2026", RelationKind::Table, "bookings_2026_excl").unwrap()
}
fn index_partitions(base: &PostgresSchemaSnapshotV3, relations: &RelationPartitionSnapshot) -> IndexPartitionSnapshot {
    IndexPartitionSnapshot::new(base, relations, vec![
        IndexPartitionObservation::non_partition(parent_index(), IndexRelationKind::PartitionedIndex).unwrap(),
        IndexPartitionObservation::partition(child_index(), IndexRelationKind::Index, parent_index(), false).unwrap(),
    ]).unwrap()
}
fn parent_constraint() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new("public", "bookings", RelationKind::PartitionedTable, "bookings_excl").unwrap()
}
fn child_constraint() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new("public", "bookings_2026", RelationKind::Table, "bookings_2026_excl").unwrap()
}

#[test]
fn attached_exclusion_constraint_rejects_missing_inherited_state() {
    let base = base_snapshot(); let relations = relation_partitions(&base); let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(&base, &relations, &indexes, vec![
        IndexExclusionConstraintObservation::root(parent_constraint()).unwrap(),
        IndexExclusionConstraintObservation::partition(child_constraint(), parent_constraint(), true, 0).unwrap(),
    ]).expect_err("partitioned EXCLUDE constraints must preserve parentage and inherited state");
    assert_eq!(error, ObservationError::InvalidObservationField { field: "index_exclusion_constraint_inheritance_state" });
}

#[test]
fn exact_exclusion_constraint_partition_state_is_admitted_and_receipted() {
    let base = base_snapshot(); let relations = relation_partitions(&base); let indexes = index_partitions(&base, &relations);
    let snapshot = IndexExclusionConstraintSnapshot::new(&base, &relations, &indexes, vec![
        IndexExclusionConstraintObservation::root(parent_constraint()).unwrap(),
        IndexExclusionConstraintObservation::partition(child_constraint(), parent_constraint(), false, 1).unwrap(),
    ]).expect("exact PostgreSQL exclusion-constraint partition state must be admitted");
    let receipt = snapshot.source_receipt(child_constraint()).expect("observed child exclusion constraint must issue provenance");
    assert_eq!(receipt.location().parent_constraint(), Some(&parent_constraint()));
    assert!(!receipt.location().is_local());
    assert_eq!(receipt.location().inheritance_count(), 1);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn exclusion_constraint_inventory_is_complete_over_exclusion_indexes() {
    let base = base_snapshot(); let relations = relation_partitions(&base); let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(
        &base, &relations, &indexes,
        vec![IndexExclusionConstraintObservation::root(parent_constraint()).unwrap()],
    ).expect_err("every non-key indisexclusion index must retain its pg_constraint row");
    assert_eq!(error, ObservationError::InvalidObservationField { field: "index_exclusion_constraint_completeness" });
}
