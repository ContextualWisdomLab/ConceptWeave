use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintEnforcementObservation,
    IndexExclusionConstraintEnforcementSnapshot, IndexExclusionConstraintObservation,
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
        "extractor-index-exclusion-constraint-enforcement-v1",
        "2026-09-16T03:44:00Z",
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

fn timing_snapshot() -> IndexExclusionConstraintTimingSnapshot {
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
    let constraints = IndexExclusionConstraintSnapshot::new(
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
    .unwrap();
    IndexExclusionConstraintTimingSnapshot::new(
        &constraints,
        vec![
            IndexExclusionConstraintTimingObservation::new(parent_constraint(), false, false)
                .unwrap(),
            IndexExclusionConstraintTimingObservation::new(child_constraint(), false, false)
                .unwrap(),
        ],
    )
    .unwrap()
}

#[test]
fn exclusion_constraint_cannot_be_observed_as_not_enforced() {
    let error = IndexExclusionConstraintEnforcementObservation::new(parent_constraint(), false)
        .expect_err("PostgreSQL 18 does not admit NOT ENFORCED EXCLUDE constraints");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_enforcement_state",
        }
    );
}

#[test]
fn exclusion_constraint_enforcement_inventory_must_be_complete() {
    let timing = timing_snapshot();
    let error = IndexExclusionConstraintEnforcementSnapshot::new(
        &timing,
        vec![
            IndexExclusionConstraintEnforcementObservation::new(parent_constraint(), true).unwrap(),
        ],
    )
    .expect_err("every EXCLUDE constraint must retain pg_constraint.conenforced");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_enforcement_completeness",
        }
    );
}

#[test]
fn enforced_exclusion_constraint_issues_exact_provenance() {
    let timing = timing_snapshot();
    let snapshot = IndexExclusionConstraintEnforcementSnapshot::new(
        &timing,
        vec![
            IndexExclusionConstraintEnforcementObservation::new(parent_constraint(), true).unwrap(),
            IndexExclusionConstraintEnforcementObservation::new(child_constraint(), true).unwrap(),
        ],
    )
    .expect("PostgreSQL 18 EXCLUDE constraints are enforced evidence");

    let receipt = snapshot
        .source_receipt(child_constraint())
        .expect("observed EXCLUDE enforcement must issue provenance");
    assert!(receipt.location().enforced());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/enforcement")
    );
}
