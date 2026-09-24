use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCatalogShapeObservation, IndexExclusionConstraintCatalogShapeSnapshot,
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintForeignActionCodes,
    IndexExclusionConstraintForeignPayloadPresence, IndexExclusionConstraintIndexNameSnapshot,
    IndexExclusionConstraintIndexNamespaceObservation, IndexExclusionConstraintIndexNamespaceSnapshot,
    IndexExclusionConstraintObservation, IndexExclusionConstraintSnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    RelationPartitionObservation, RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;
impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool { key == "warehouse_primary" }
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

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public", "bookings", RelationKind::Table, "bookings_no_overlap",
    )
    .unwrap()
}

fn index_coordinate() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public", "bookings", RelationKind::Table, "bookings_no_overlap",
    )
    .unwrap()
}

fn index() -> IndexObservation {
    IndexObservation::new(
        "bookings_no_overlap",
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
        0,
    )
    .unwrap()])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

struct Stack {
    names: IndexExclusionConstraintIndexNameSnapshot,
}

fn stack() -> Stack {
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
    .with_indexes(vec![index()])
    .unwrap();
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-index-namespace-v1",
        "2026-09-17T00:20:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap();
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![RelationPartitionObservation::non_partition(
            "public", "bookings", RelationKind::Table,
        )
        .unwrap()],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![IndexPartitionObservation::non_partition(
            index_coordinate(),
            IndexRelationKind::Index,
        )
        .unwrap()],
    )
    .unwrap();
    let constraints = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![IndexExclusionConstraintObservation::root(coordinate(), index_coordinate()).unwrap()],
    )
    .unwrap();
    let shapes = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![IndexExclusionConstraintCatalogShapeObservation::new(
            coordinate(),
            'x',
            true,
            false,
            IndexExclusionConstraintForeignActionCodes::new(' ', ' ', ' '),
            IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], false),
            false,
        )
        .unwrap()],
    )
    .unwrap();
    let names = IndexExclusionConstraintIndexNameSnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &shapes,
    )
    .unwrap();
    Stack { names }
}

fn namespace(schema_name: &str) -> IndexExclusionConstraintIndexNamespaceObservation {
    IndexExclusionConstraintIndexNamespaceObservation::new(
        coordinate(),
        index_coordinate(),
        schema_name,
    )
    .unwrap()
}

#[test]
fn exact_backing_index_namespace_is_retained_and_receipted() {
    let stack = stack();
    let snapshot = IndexExclusionConstraintIndexNamespaceSnapshot::new(
        &stack.names,
        vec![namespace("public")],
    )
    .expect("resolved pg_class.relnamespace must agree with the exact conindid index coordinate");

    let receipt = snapshot.source_receipt(coordinate()).unwrap();
    assert_eq!(receipt.location().index_schema_name(), "public");
    assert_eq!(receipt.location().backing_index(), &index_coordinate());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt.location().canonical_location().ends_with("/backing-index-namespace"));
}

#[test]
fn mismatched_backing_index_namespace_fails_closed() {
    let stack = stack();
    let observation = IndexExclusionConstraintIndexNamespaceObservation::new(
        coordinate(),
        index_coordinate(),
        "archive",
    )
    .unwrap();
    let error = IndexExclusionConstraintIndexNamespaceSnapshot::new(&stack.names, vec![observation])
        .expect_err("pg_class.relnamespace must not be normalized from the owning relation schema");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_namespace_state",
        }
    );
}

#[test]
fn blank_backing_index_namespace_fails_closed() {
    let error = IndexExclusionConstraintIndexNamespaceObservation::new(
        coordinate(),
        index_coordinate(),
        "   ",
    )
    .expect_err("resolved pg_class.relnamespace must be present");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_namespace_name",
        }
    );
}

#[test]
fn backing_index_namespace_inventory_must_be_complete() {
    let stack = stack();
    let error = IndexExclusionConstraintIndexNamespaceSnapshot::new(&stack.names, vec![])
        .expect_err("every ordinary EXCLUDE constraint must retain backing-index relnamespace");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_namespace_completeness",
        }
    );
}

#[test]
fn backing_index_namespace_must_bind_the_exact_predecessor_index() {
    let stack = stack();
    let wrong_index = IndexPartitionCoordinate::new(
        "public", "bookings", RelationKind::Table, "other_index",
    )
    .unwrap();
    let observation = IndexExclusionConstraintIndexNamespaceObservation::new(
        coordinate(), wrong_index, "public",
    )
    .unwrap();
    let error = IndexExclusionConstraintIndexNamespaceSnapshot::new(&stack.names, vec![observation])
        .expect_err("namespace evidence must bind the exact conindid index proven by the predecessor");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_namespace_completeness",
        }
    );
}

#[test]
fn duplicate_backing_index_namespace_coordinate_fails_closed() {
    let stack = stack();
    let observation = namespace("public");
    let error = IndexExclusionConstraintIndexNamespaceSnapshot::new(
        &stack.names,
        vec![observation.clone(), observation],
    )
    .expect_err("one pg_class namespace observation must not be admitted twice");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_namespace_coordinate",
        }
    );
}

#[test]
fn unknown_backing_index_namespace_receipt_fails_closed() {
    let stack = stack();
    let snapshot = IndexExclusionConstraintIndexNamespaceSnapshot::new(
        &stack.names,
        vec![namespace("public")],
    )
    .unwrap();
    let unknown = IndexExclusionConstraintCoordinate::new(
        "public", "other", RelationKind::Table, "other_no_overlap",
    )
    .unwrap();
    assert!(matches!(
        snapshot.source_receipt(unknown),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
