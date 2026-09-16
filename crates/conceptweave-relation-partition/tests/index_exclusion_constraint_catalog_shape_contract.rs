use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCatalogShapeObservation, IndexExclusionConstraintCatalogShapeSnapshot,
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintForeignActionCodes,
    IndexExclusionConstraintForeignPayloadPresence, IndexExclusionConstraintObservation,
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
    fn contains_source_connection(&self, key: &str) -> bool {
        key == "warehouse"
    }

    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "warehouse").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(&self, source: &ResolvedSourceConnection, schemas: &[String]) -> bool {
        source.source_connection_key() == "warehouse"
            && source.connection_policy_binding() == POLICY_BINDING
            && schemas == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source: &ResolvedSourceConnection,
        envelope: ObservationResourceEnvelope,
    ) -> bool {
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
        "warehouse",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn exclusion_index(name: &str) -> IndexObservation {
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
    .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn relation(name: &str, kind: RelationKind, index_name: &str) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
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
    .with_indexes(vec![exclusion_index(index_name)])
    .unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-catalog-shape-v1",
        "2026-09-16T12:30:00Z",
        vec![
            relation("bookings", RelationKind::PartitionedTable, "bookings_excl_idx"),
            relation("bookings_2026", RelationKind::Table, "bookings_2026_excl_idx"),
        ],
        vec![],
        vec![],
    )
    .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::PartitionedTable,
        "bookings_excl_idx",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings_2026",
        RelationKind::Table,
        "bookings_2026_excl_idx",
    )
    .unwrap()
}

fn parent_constraint() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::PartitionedTable,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn child_constraint() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings_2026",
        RelationKind::Table,
        "bookings_2026_no_overlap",
    )
    .unwrap()
}

fn exclusion_constraint_snapshot() -> IndexExclusionConstraintSnapshot {
    let base = base_snapshot();
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "bookings",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "bookings_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "bookings").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::partition(
                child_index(),
                IndexRelationKind::Index,
                parent_index(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexExclusionConstraintObservation::root(parent_constraint(), parent_index()).unwrap(),
            IndexExclusionConstraintObservation::partition(
                child_constraint(),
                child_index(),
                parent_constraint(),
                false,
                1,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn empty_actions() -> IndexExclusionConstraintForeignActionCodes {
    IndexExclusionConstraintForeignActionCodes::new(' ', ' ', ' ')
}

fn empty_foreign_payload() -> IndexExclusionConstraintForeignPayloadPresence {
    IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], false)
}

fn shape(
    coordinate: IndexExclusionConstraintCoordinate,
) -> IndexExclusionConstraintCatalogShapeObservation {
    IndexExclusionConstraintCatalogShapeObservation::new(
        coordinate,
        'x',
        false,
        empty_actions(),
        empty_foreign_payload(),
        false,
    )
    .unwrap()
}

#[test]
fn exact_exclusion_catalog_family_shape_is_retained() {
    let constraints = exclusion_constraint_snapshot();
    let snapshot = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![shape(parent_constraint()), shape(child_constraint())],
    )
    .expect("ordinary EXCLUDE rows must have only exclusion-family catalog payload");

    let receipt = snapshot.source_receipt(child_constraint()).unwrap();
    assert_eq!(receipt.location().constraint_type_code(), 'x');
    assert!(!receipt.location().domain_owner_present());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt.location().canonical_location().ends_with("/catalog-family-shape"));
}

#[test]
fn non_exclusion_constraint_type_fails_closed() {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        parent_constraint(),
        'u',
        false,
        empty_actions(),
        empty_foreign_payload(),
        false,
    )
    .expect_err("ordinary EXCLUDE evidence must come from contype=x");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_type",
        }
    );
}

#[test]
fn domain_owner_residue_fails_closed() {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        parent_constraint(),
        'x',
        true,
        empty_actions(),
        empty_foreign_payload(),
        false,
    )
    .expect_err("table EXCLUDE rows must have contypid=0");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape",
        }
    );
}

#[test]
fn foreign_action_residue_fails_closed() {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        parent_constraint(),
        'x',
        false,
        IndexExclusionConstraintForeignActionCodes::new('a', ' ', ' '),
        empty_foreign_payload(),
        false,
    )
    .expect_err("non-FK rows must retain space sentinels for FK action codes");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape",
        }
    );
}

#[test]
fn foreign_array_residue_fails_closed() {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        parent_constraint(),
        'x',
        false,
        empty_actions(),
        IndexExclusionConstraintForeignPayloadPresence::new(false, true, [false; 3], false),
        false,
    )
    .expect_err("confkey and other FK-only arrays must be NULL for EXCLUDE rows");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape",
        }
    );
}

#[test]
fn check_expression_residue_fails_closed() {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        parent_constraint(),
        'x',
        false,
        empty_actions(),
        empty_foreign_payload(),
        true,
    )
    .expect_err("conbin must be NULL outside CHECK constraints");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape",
        }
    );
}

#[test]
fn catalog_shape_inventory_must_be_complete_and_unique() {
    let constraints = exclusion_constraint_snapshot();
    let incomplete = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![shape(parent_constraint())],
    )
    .expect_err("every predecessor EXCLUDE constraint must have one family-shape observation");
    assert_eq!(
        incomplete,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape_completeness",
        }
    );

    let parent = shape(parent_constraint());
    let duplicate = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![parent.clone(), parent],
    )
    .expect_err("one catalog row must not be admitted twice");
    assert_eq!(
        duplicate,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape_coordinate",
        }
    );
}

#[test]
fn unknown_catalog_shape_receipt_fails_closed() {
    let constraints = exclusion_constraint_snapshot();
    let snapshot = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![shape(parent_constraint()), shape(child_constraint())],
    )
    .unwrap();
    let unknown = IndexExclusionConstraintCoordinate::new(
        "public",
        "other",
        RelationKind::Table,
        "other_no_overlap",
    )
    .unwrap();
    assert!(matches!(
        snapshot.source_receipt(unknown),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
