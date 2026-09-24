use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCatalogShapeObservation, IndexExclusionConstraintCatalogShapeSnapshot,
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintForeignActionCodes,
    IndexExclusionConstraintForeignPayloadPresence, IndexExclusionConstraintIndexNameSnapshot,
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

fn index(name: &str, exclusion: bool) -> IndexObservation {
    IndexObservation::new(
        name,
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
    .with_catalog_flags(IndexCatalogFlags::new(false, exclusion, true, false, false, false))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn relation(name: &str, indexes: Vec<IndexObservation>) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
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
    .with_indexes(indexes)
    .unwrap()
}

struct Stack {
    base: PostgresSchemaSnapshotV3,
    relations: RelationPartitionSnapshot,
    indexes: IndexPartitionSnapshot,
    constraints: IndexExclusionConstraintSnapshot,
    shapes: IndexExclusionConstraintCatalogShapeSnapshot,
}

fn stack(specs: &[(&str, &str, &str)]) -> Stack {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-index-name-v1",
        "2026-09-16T13:42:00Z",
        specs
            .iter()
            .map(|(relation_name, index_name, _)| {
                relation(relation_name, vec![index(index_name, true)])
            })
            .collect(),
        vec![],
        vec![],
    )
    .unwrap();
    stack_from_base(&base, specs)
}

fn stack_from_base(base: &PostgresSchemaSnapshotV3, specs: &[(&str, &str, &str)]) -> Stack {
    let relation_observations = base
        .relations()
        .iter()
        .map(|relation| {
            RelationPartitionObservation::non_partition(
                relation.schema_name(),
                relation.relation_name(),
                relation.kind(),
            )
            .unwrap()
        })
        .collect();
    let relations = RelationPartitionSnapshot::new(base, relation_observations).unwrap();

    let index_observations = base
        .relations()
        .iter()
        .flat_map(|relation| {
            relation.indexes().iter().map(|observed_index| {
                IndexPartitionObservation::non_partition(
                    IndexPartitionCoordinate::new(
                        relation.schema_name(),
                        relation.relation_name(),
                        relation.kind(),
                        observed_index.index_name(),
                    )
                    .unwrap(),
                    IndexRelationKind::Index,
                )
                .unwrap()
            })
        })
        .collect();
    let indexes = IndexPartitionSnapshot::new(base, &relations, index_observations).unwrap();

    let constraint_observations = specs
        .iter()
        .map(|(relation_name, index_name, constraint_name)| {
            IndexExclusionConstraintObservation::root(
                coordinate(relation_name, constraint_name),
                index_coordinate(relation_name, index_name),
            )
            .unwrap()
        })
        .collect();
    let constraints =
        IndexExclusionConstraintSnapshot::new(base, &relations, &indexes, constraint_observations)
            .unwrap();

    let shape_observations = specs
        .iter()
        .map(|(relation_name, _, constraint_name)| shape(coordinate(relation_name, constraint_name)))
        .collect();
    let shapes =
        IndexExclusionConstraintCatalogShapeSnapshot::new(&constraints, shape_observations).unwrap();

    Stack {
        base: base.clone(),
        relations,
        indexes,
        constraints,
        shapes,
    }
}

fn coordinate(relation_name: &str, constraint_name: &str) -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        relation_name,
        RelationKind::Table,
        constraint_name,
    )
    .unwrap()
}

fn index_coordinate(relation_name: &str, index_name: &str) -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        relation_name,
        RelationKind::Table,
        index_name,
    )
    .unwrap()
}

fn shape(
    coordinate: IndexExclusionConstraintCoordinate,
) -> IndexExclusionConstraintCatalogShapeObservation {
    IndexExclusionConstraintCatalogShapeObservation::new(
        coordinate,
        'x',
        true,
        false,
        IndexExclusionConstraintForeignActionCodes::new(' ', ' ', ' '),
        IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], false),
        false,
    )
    .unwrap()
}

#[test]
fn matching_constraint_and_backing_index_name_is_admitted_and_receipted() {
    let stack = stack(&[("bookings", "bookings_no_overlap", "bookings_no_overlap")]);
    let snapshot = IndexExclusionConstraintIndexNameSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.constraints,
        &stack.shapes,
    )
    .expect("ordinary EXCLUDE constraint and exact conindid index names must agree");

    let receipt = snapshot
        .source_receipt(coordinate("bookings", "bookings_no_overlap"))
        .unwrap();
    assert_eq!(receipt.location().coordinate().constraint_name(), "bookings_no_overlap");
    assert_eq!(receipt.location().backing_index().index_name(), "bookings_no_overlap");
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn mismatched_constraint_and_backing_index_name_fails_closed() {
    let stack = stack(&[("bookings", "bookings_excl_idx", "bookings_no_overlap")]);
    let error = IndexExclusionConstraintIndexNameSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.constraints,
        &stack.shapes,
    )
    .expect_err("PostgreSQL names an EXCLUDE backing index exactly like its constraint");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_name_state",
        }
    );
}

#[test]
fn same_schema_duplicate_backing_index_name_fails_closed() {
    let error = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-index-name-v1",
        "2026-09-16T13:42:00Z",
        vec![
            relation("bookings", vec![index("shared_no_overlap", true)]),
            relation("reservations", vec![index("shared_no_overlap", true)]),
        ],
        vec![],
        vec![],
    )
    .expect_err("pg_class relation names, including indexes, are schema-scoped");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "schema_relation_namespace",
        }
    );
}

#[test]
fn backing_index_name_colliding_with_relation_name_fails_closed() {
    let error = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-index-name-v1",
        "2026-09-16T13:42:00Z",
        vec![
            relation("bookings", vec![index("shared_relation_name", true)]),
            relation("shared_relation_name", vec![]),
        ],
        vec![],
        vec![],
    )
    .expect_err("an index cannot reuse a pg_class relation name in the same schema");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "schema_relation_namespace",
        }
    );
}

#[test]
fn unknown_index_name_receipt_fails_closed() {
    let stack = stack(&[("bookings", "bookings_no_overlap", "bookings_no_overlap")]);
    let snapshot = IndexExclusionConstraintIndexNameSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.constraints,
        &stack.shapes,
    )
    .unwrap();
    let unknown = coordinate("other", "other_no_overlap");
    assert!(matches!(
        snapshot.source_receipt(unknown),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
