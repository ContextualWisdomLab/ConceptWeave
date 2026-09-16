use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    NotNullConstraintObservation, ObservationError, ParentNotNullConstraintCoordinate,
    PostgresSchemaSnapshotV3, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation,
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
    .with_key_semantics(
        vec![IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
            0,
        )
        .unwrap()],
    )
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

fn relation_with_check_constraint(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    constraint_name: &str,
) -> RelationObservation {
    relation(name, kind, index_name)
        .with_constraints(vec![TableConstraintObservation::Check(
            CheckConstraintObservation::new(constraint_name, "id > 0", true, true, false).unwrap(),
        )])
        .unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-v1",
        "2026-09-16T02:25:00Z",
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

fn base_snapshot_with_constraint_on(
    relation_name: &str,
    constraint_name: &str,
) -> PostgresSchemaSnapshotV3 {
    let parent = if relation_name == "bookings" {
        relation_with_check_constraint(
            "bookings",
            RelationKind::PartitionedTable,
            "bookings_excl_idx",
            constraint_name,
        )
    } else {
        relation(
            "bookings",
            RelationKind::PartitionedTable,
            "bookings_excl_idx",
        )
    };
    let child = if relation_name == "bookings_2026" {
        relation_with_check_constraint(
            "bookings_2026",
            RelationKind::Table,
            "bookings_2026_excl_idx",
            constraint_name,
        )
    } else {
        relation(
            "bookings_2026",
            RelationKind::Table,
            "bookings_2026_excl_idx",
        )
    };
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-v1",
        "2026-09-16T02:25:00Z",
        vec![parent, child],
        vec![],
        vec![],
    )
    .unwrap()
}

fn base_snapshot_with_not_null_collision() -> PostgresSchemaSnapshotV3 {
    let parent_constraint = NotNullConstraintObservation::new(
        "public",
        "bookings",
        RelationKind::PartitionedTable,
        "bookings_no_overlap",
        "id",
        true,
        true,
        true,
        0,
        false,
    )
    .unwrap();
    let child_constraint = NotNullConstraintObservation::new(
        "public",
        "bookings_2026",
        RelationKind::Table,
        "bookings_no_overlap",
        "id",
        true,
        true,
        false,
        1,
        false,
    )
    .unwrap()
    .with_parent_constraint(
        ParentNotNullConstraintCoordinate::new(
            "public",
            "bookings",
            RelationKind::PartitionedTable,
            "bookings_no_overlap",
        )
        .unwrap(),
    )
    .unwrap()
    .with_partition_parent_relation("public", "bookings", false)
    .unwrap();

    base_snapshot()
        .with_observed_not_null_constraints(vec![parent_constraint, child_constraint])
        .unwrap()
}

fn relation_partitions(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(
        base,
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
fn index_partitions(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> IndexPartitionSnapshot {
    IndexPartitionSnapshot::new(
        base,
        relations,
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
fn index_partitions(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> IndexPartitionSnapshot {
    IndexPartitionSnapshot::new(
        base,
        relations,
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

fn exact_exclusion_observations() -> Vec<IndexExclusionConstraintObservation> {
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
    ]
}

#[test]
fn attached_exclusion_constraint_rejects_missing_inherited_state() {
    let base = base_snapshot();
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexExclusionConstraintObservation::root(parent_constraint(), parent_index()).unwrap(),
            IndexExclusionConstraintObservation::partition(
                child_constraint(),
                child_index(),
                parent_constraint(),
                true,
                0,
            )
            .unwrap(),
        ],
    )
    .expect_err("partitioned EXCLUDE constraints must preserve parentage and inherited state");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_inheritance_state"
        }
    );
}

#[test]
fn exact_exclusion_constraint_partition_state_is_admitted_and_receipted() {
    let base = base_snapshot();
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let snapshot = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        exact_exclusion_observations(),
    )
    .expect("exact PostgreSQL exclusion-constraint partition state must be admitted");
    let receipt = snapshot
        .source_receipt(child_constraint())
        .expect("observed child exclusion constraint must issue provenance");
    assert_eq!(receipt.location().backing_index(), &child_index());
    assert_eq!(
        receipt.location().parent_constraint(),
        Some(&parent_constraint())
    );
    assert!(!receipt.location().is_local());
    assert_eq!(receipt.location().inheritance_count(), 1);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn exclusion_constraint_inventory_is_complete_over_exclusion_indexes() {
    let base = base_snapshot();
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![IndexExclusionConstraintObservation::root(
            parent_constraint(),
            parent_index(),
        )
        .unwrap()],
    )
    .expect_err("every non-key indisexclusion index must retain its exact conindid-backed constraint row");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_completeness"
        }
    );
}

#[test]
fn exclusion_constraint_rejects_name_collision_with_existing_relation_constraint() {
    let base = base_snapshot_with_constraint_on("bookings", "bookings_no_overlap");
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        exact_exclusion_observations(),
    )
    .expect_err("one PostgreSQL relation cannot contain two constraints with the same conname");
    assert_eq!(
        error,
        ObservationError::DuplicateConstraintName {
            schema_name: "public".to_owned(),
            table_name: "bookings".to_owned(),
            constraint_name: "bookings_no_overlap".to_owned(),
        }
    );
}

#[test]
fn exclusion_constraint_rejects_name_collision_with_observed_not_null_constraint() {
    let base = base_snapshot_with_not_null_collision();
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let error = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        exact_exclusion_observations(),
    )
    .expect_err("first-class NOT NULL and EXCLUDE rows share relation-wide conname uniqueness");
    assert_eq!(
        error,
        ObservationError::DuplicateConstraintName {
            schema_name: "public".to_owned(),
            table_name: "bookings".to_owned(),
            constraint_name: "bookings_no_overlap".to_owned(),
        }
    );
}

#[test]
fn same_constraint_name_on_different_relation_does_not_collide() {
    let base = base_snapshot_with_constraint_on("bookings_2026", "bookings_no_overlap");
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        exact_exclusion_observations(),
    )
    .expect("PostgreSQL permits the same constraint name on different relations");
}
