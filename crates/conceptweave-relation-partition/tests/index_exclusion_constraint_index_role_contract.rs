use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintImmediacySnapshot,
    IndexExclusionConstraintIndexRoleSnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintSnapshot, IndexExclusionConstraintTimingObservation,
    IndexExclusionConstraintTimingSnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
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

fn exclusion_index(name: &str, unique: bool, primary: bool) -> IndexObservation {
    IndexObservation::new(
        name,
        unique,
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
        primary, true, true, false, false, false,
    ))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    unique: bool,
    primary: bool,
) -> RelationObservation {
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
    .with_indexes(vec![exclusion_index(index_name, unique, primary)])
    .unwrap()
}

fn base_snapshot(unique: bool, primary: bool) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-index-role-v1",
        "2026-09-16T09:55:00Z",
        vec![
            relation(
                "bookings",
                RelationKind::PartitionedTable,
                "bookings_excl_idx",
                unique,
                primary,
            ),
            relation(
                "bookings_2026",
                RelationKind::Table,
                "bookings_2026_excl_idx",
                unique,
                primary,
            ),
        ],
        vec![],
        vec![],
    )
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

fn exclusion_constraints(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
    indexes: &IndexPartitionSnapshot,
) -> IndexExclusionConstraintSnapshot {
    IndexExclusionConstraintSnapshot::new(
        base,
        relations,
        indexes,
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

fn timing_snapshot(
    constraints: &IndexExclusionConstraintSnapshot,
) -> IndexExclusionConstraintTimingSnapshot {
    IndexExclusionConstraintTimingSnapshot::new(
        constraints,
        vec![
            IndexExclusionConstraintTimingObservation::new(parent_constraint(), false, false)
                .unwrap(),
            IndexExclusionConstraintTimingObservation::new(child_constraint(), false, false)
                .unwrap(),
        ],
    )
    .unwrap()
}

fn build_role(
    unique: bool,
    primary: bool,
) -> Result<IndexExclusionConstraintIndexRoleSnapshot, ObservationError> {
    let base = base_snapshot(unique, primary);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    let timing = timing_snapshot(&constraints);
    let immediacy = IndexExclusionConstraintImmediacySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &timing,
    )
    .unwrap();
    IndexExclusionConstraintIndexRoleSnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &timing,
        &immediacy,
    )
}

#[test]
fn ordinary_exclusion_rejects_unique_backing_index() {
    let error = build_role(true, false)
        .expect_err("ordinary EXCLUDE cannot be backed by pg_index.indisunique=true");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_role_state",
        }
    );
}

#[test]
fn ordinary_exclusion_rejects_primary_backing_index() {
    let error = build_role(true, true)
        .expect_err("ordinary EXCLUDE cannot be backed by pg_index.indisprimary=true");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_role_state",
        }
    );
}

#[test]
fn ordinary_exclusion_requires_nonunique_nonprimary_exclusion_index() {
    let snapshot = build_role(false, false)
        .expect("ordinary EXCLUDE must retain the PostgreSQL backing-index role vector");
    assert!(snapshot.observations().iter().all(|observation| {
        !observation.index_unique()
            && !observation.index_primary()
            && observation.index_exclusion()
    }));

    let receipt = snapshot
        .source_receipt(child_constraint())
        .expect("validated child backing-index role must issue provenance");
    assert_eq!(receipt.location().backing_index(), &child_index());
    assert!(!receipt.location().index_unique());
    assert!(!receipt.location().index_primary());
    assert!(receipt.location().index_exclusion());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}
