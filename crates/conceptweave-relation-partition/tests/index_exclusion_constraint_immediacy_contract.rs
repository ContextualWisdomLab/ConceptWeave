use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintImmediacySnapshot,
    IndexExclusionConstraintObservation, IndexExclusionConstraintSnapshot,
    IndexExclusionConstraintTimingObservation, IndexExclusionConstraintTimingSnapshot,
    IndexPartitionCoordinate, IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    PartitionParentRelationCoordinate, RelationPartitionObservation, RelationPartitionSnapshot,
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

fn exclusion_index(name: &str, immediate: bool) -> IndexObservation {
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
        false, true, immediate, false, false, false,
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
    immediate: bool,
) -> RelationObservation {
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
    .with_indexes(vec![exclusion_index(index_name, immediate)])
    .unwrap()
}

fn base_snapshot(immediate: bool) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-immediacy-v1",
        "2026-09-16T08:42:00Z",
        vec![
            relation(
                "bookings",
                RelationKind::PartitionedTable,
                "bookings_excl_idx",
                immediate,
            ),
            relation(
                "bookings_2026",
                RelationKind::Table,
                "bookings_2026_excl_idx",
                immediate,
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
    deferrable: bool,
    initially_deferred: bool,
) -> IndexExclusionConstraintTimingSnapshot {
    IndexExclusionConstraintTimingSnapshot::new(
        constraints,
        vec![
            IndexExclusionConstraintTimingObservation::new(
                parent_constraint(),
                deferrable,
                initially_deferred,
            )
            .unwrap(),
            IndexExclusionConstraintTimingObservation::new(
                child_constraint(),
                deferrable,
                initially_deferred,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn build_immediacy(
    index_immediate: bool,
    deferrable: bool,
    initially_deferred: bool,
) -> Result<IndexExclusionConstraintImmediacySnapshot, ObservationError> {
    let base = base_snapshot(index_immediate);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);
    let constraints = exclusion_constraints(&base, &relations, &indexes);
    let timing = timing_snapshot(&constraints, deferrable, initially_deferred);
    IndexExclusionConstraintImmediacySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &timing,
    )
}

#[test]
fn deferrable_exclusion_requires_non_immediate_backing_index() {
    let error = build_immediacy(true, true, false)
        .expect_err("condeferrable=true cannot coexist with indimmediate=true");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_immediacy_state",
        }
    );
}

#[test]
fn nondeferrable_exclusion_requires_immediate_backing_index() {
    let error = build_immediacy(false, false, false)
        .expect_err("condeferrable=false cannot coexist with indimmediate=false");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_immediacy_state",
        }
    );
}

#[test]
fn both_deferrable_initial_modes_share_non_immediate_index_but_remain_distinct() {
    let initially_immediate = build_immediacy(false, true, false)
        .expect("DEFERRABLE INITIALLY IMMEDIATE must use a non-immediate backing index");
    let initially_deferred = build_immediacy(false, true, true)
        .expect("DEFERRABLE INITIALLY DEFERRED must use a non-immediate backing index");

    assert_ne!(
        initially_immediate.snapshot_digest(),
        initially_deferred.snapshot_digest(),
        "condeferred remains material timing identity even though indimmediate is false in both cases",
    );

    let receipt = initially_deferred
        .source_receipt(child_constraint())
        .expect("validated child timing/index edge must issue provenance");
    assert!(receipt.location().constraint_deferrable());
    assert!(!receipt.location().index_immediate());
    assert_eq!(receipt.location().backing_index(), &child_index());
    assert_eq!(
        receipt.source_digest(),
        initially_deferred.snapshot_digest()
    );
}

#[test]
fn nondeferrable_exclusion_with_immediate_index_is_admitted() {
    let snapshot = build_immediacy(true, false, false)
        .expect("NOT DEFERRABLE ordinary EXCLUDE must retain indimmediate=true");
    assert!(
        snapshot.observations().iter().all(
            |observation| !observation.constraint_deferrable() && observation.index_immediate()
        )
    );
}
