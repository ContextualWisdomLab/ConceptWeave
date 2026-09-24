use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintPeriodObservation, IndexExclusionConstraintPeriodSnapshot,
    IndexExclusionConstraintSnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, RelationPartitionObservation,
    RelationPartitionSnapshot,
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

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    let index = IndexObservation::new(
        "bookings_no_overlap_idx",
        false,
        Some(false),
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "resource_id").unwrap(),
            IndexAttributeObservation::expression(2, IndexAttributeKind::Key, "lower(note)").unwrap(),
        ],
        vec![IndexAttributeObservation::column(
            3,
            IndexAttributeKind::Include,
            "payload",
        )
        .unwrap()],
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
        IndexKeySemantics::new(
            2,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true);

    let relation = RelationObservation::new(
        "public",
        "bookings",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "resource_id",
                1,
                "bigint",
                QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
                false,
                None,
            )
            .unwrap(),
            ColumnObservationV3::new(
                "note",
                2,
                "text",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                true,
                None,
            )
            .unwrap(),
            ColumnObservationV3::new(
                "payload",
                3,
                "text",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                true,
                None,
            )
            .unwrap(),
        ],
    )
    .unwrap()
    .with_indexes(vec![index])
    .unwrap();

    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-constraint-key-v1",
        "2026-09-16T06:45:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap()
}

fn index_coordinate() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap_idx",
    )
    .unwrap()
}

fn constraint_coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn predecessor_snapshots() -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
    IndexExclusionConstraintSnapshot,
    IndexExclusionConstraintPeriodSnapshot,
) {
    let base = base_snapshot();
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![RelationPartitionObservation::non_partition(
            "public",
            "bookings",
            RelationKind::Table,
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
        vec![IndexExclusionConstraintObservation::root(
            constraint_coordinate(),
            index_coordinate(),
        )
        .unwrap()],
    )
    .unwrap();
    let period = IndexExclusionConstraintPeriodSnapshot::new(
        &constraints,
        vec![IndexExclusionConstraintPeriodObservation::new(
            constraint_coordinate(),
            false,
        )
        .unwrap()],
    )
    .unwrap();
    (base, relations, indexes, constraints, period)
}

#[test]
fn ordinary_exclude_preserves_exact_conkey_with_expression_zero_and_omits_include() {
    let (base, relations, indexes, constraints, period) = predecessor_snapshots();
    let snapshot = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
        vec![IndexExclusionConstraintKeyObservation::new(
            constraint_coordinate(),
            vec![1, 0],
        )
        .unwrap()],
    )
    .expect("conkey preserves key attnums/expression zero and excludes INCLUDE payload");

    assert_eq!(
        snapshot
            .source_receipt(constraint_coordinate())
            .unwrap()
            .location()
            .attribute_numbers(),
        &[1, 0]
    );
}

#[test]
fn ordinary_exclude_rejects_conkey_that_disagrees_with_backing_index_key_layout() {
    let (base, relations, indexes, constraints, period) = predecessor_snapshots();
    let error = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
        vec![IndexExclusionConstraintKeyObservation::new(
            constraint_coordinate(),
            vec![2, 0],
        )
        .unwrap()],
    )
    .expect_err("conkey is independently stored catalog evidence and must match pg_index.indkey keys");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_key_state",
        }
    );
}

#[test]
fn exclusion_constraint_key_inventory_must_be_complete() {
    let (base, relations, indexes, constraints, period) = predecessor_snapshots();
    let error = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
        vec![],
    )
    .expect_err("every ordinary EXCLUDE constraint must retain pg_constraint.conkey");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_key_completeness",
        }
    );
}

#[test]
fn exact_conkey_issues_domain_separated_provenance() {
    let (base, relations, indexes, constraints, period) = predecessor_snapshots();
    let snapshot = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
        vec![IndexExclusionConstraintKeyObservation::new(
            constraint_coordinate(),
            vec![1, 0],
        )
        .unwrap()],
    )
    .unwrap();

    let receipt = snapshot.source_receipt(constraint_coordinate()).unwrap();
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt.location().canonical_location().ends_with("/key-attributes"));
}
