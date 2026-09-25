use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintNamespaceObservation,
    IndexExclusionConstraintNamespaceSnapshot, IndexExclusionConstraintObservation,
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
        key == "warehouse_primary"
    }
    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "warehouse_primary").then(|| POLICY_BINDING.to_owned())
    }
    fn authorizes_schema_scope(
        &self,
        source: &ResolvedSourceConnection,
        schemas: &[String],
    ) -> bool {
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
    .with_catalog_flags(IndexCatalogFlags::new(
        false, true, true, false, false, false,
    ))
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
        vec![
            ColumnObservationV3::new(
                "id",
                1,
                "bigint",
                QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
                false,
                None,
            )
            .unwrap(),
        ],
    )
    .unwrap()
    .with_indexes(vec![exclusion_index(index_name)])
    .unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-namespace-v1",
        "2026-09-16T11:50:00Z",
        vec![
            relation(
                "bookings",
                RelationKind::PartitionedTable,
                "bookings_excl_idx",
            ),
            relation(
                "bookings_2026",
                RelationKind::Table,
                "bookings_2026_excl_idx",
            ),
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

#[test]
fn exact_constraint_namespace_is_retained() {
    let constraints = exclusion_constraint_snapshot();
    let snapshot = IndexExclusionConstraintNamespaceSnapshot::new(
        &constraints,
        vec![
            IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "public")
                .unwrap(),
            IndexExclusionConstraintNamespaceObservation::new(child_constraint(), "public")
                .unwrap(),
        ],
    )
    .expect("ordinary EXCLUDE connamespace must match the owning relation namespace");

    let receipt = snapshot.source_receipt(child_constraint()).unwrap();
    assert_eq!(receipt.location().constraint_schema_name(), "public");
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/constraint-namespace")
    );
}

#[test]
fn mismatched_constraint_namespace_fails_closed() {
    let error = IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "archive")
        .expect_err("pg_constraint.connamespace must not drift from the owning relation schema");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_namespace_state",
        }
    );
}

#[test]
fn blank_constraint_namespace_fails_closed() {
    let error = IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "   ")
        .expect_err("resolved constraint namespace must be present");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_namespace_name",
        }
    );
}

#[test]
fn constraint_namespace_inventory_must_be_complete() {
    let constraints = exclusion_constraint_snapshot();
    let error = IndexExclusionConstraintNamespaceSnapshot::new(
        &constraints,
        vec![
            IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "public")
                .unwrap(),
        ],
    )
    .expect_err("every ordinary EXCLUDE constraint must retain resolved connamespace");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_namespace_completeness",
        }
    );
}

#[test]
fn duplicate_constraint_namespace_coordinate_fails_closed() {
    let constraints = exclusion_constraint_snapshot();
    let parent =
        IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "public").unwrap();
    let error =
        IndexExclusionConstraintNamespaceSnapshot::new(&constraints, vec![parent.clone(), parent])
            .expect_err("one catalog row must not be admitted twice");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_namespace_coordinate",
        }
    );
}

#[test]
fn unknown_namespace_receipt_fails_closed() {
    let constraints = exclusion_constraint_snapshot();
    let snapshot = IndexExclusionConstraintNamespaceSnapshot::new(
        &constraints,
        vec![
            IndexExclusionConstraintNamespaceObservation::new(parent_constraint(), "public")
                .unwrap(),
            IndexExclusionConstraintNamespaceObservation::new(child_constraint(), "public")
                .unwrap(),
        ],
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
