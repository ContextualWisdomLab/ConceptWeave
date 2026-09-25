use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintOperatorObservation, IndexExclusionConstraintOperatorSemanticsLineage,
    IndexExclusionConstraintOperatorSnapshot, IndexExclusionConstraintOperatorSourceLineage,
    IndexExclusionConstraintPeriodObservation, IndexExclusionConstraintPeriodSnapshot,
    IndexExclusionConstraintSnapshot, IndexExclusionSemanticsSnapshot,
    IndexKeyExclusionSemanticsObservation, IndexKeyOperatorFamilyObservation,
    IndexOperatorFamilySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, QualifiedOperatorFamilyName,
    QualifiedOperatorSignature, QualifiedProcedureSignature, RelationPartitionObservation,
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

fn qualified_operator(name: &str) -> QualifiedOperatorSignature {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    QualifiedOperatorSignature::new("pg_catalog", name, int4.clone(), int4).unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    let index = IndexObservation::new(
        "bookings_excl_idx",
        false,
        Some(false),
        vec![IndexAttributeObservation::column(1, IndexAttributeKind::Key, "resource_id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
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
    .with_live(true);

    let relation = RelationObservation::new(
        "public",
        "bookings",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "resource_id",
                1,
                "integer",
                QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
                false,
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
        "extractor-index-exclusion-constraint-operator-v1",
        "2026-09-16T07:50:00Z",
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
        "bookings_excl_idx",
    )
    .unwrap()
}

fn constraint_coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_excl",
    )
    .unwrap()
}

fn predecessor_snapshots() -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
    IndexExclusionConstraintSnapshot,
    IndexExclusionConstraintPeriodSnapshot,
    IndexExclusionConstraintKeySnapshot,
    IndexOperatorFamilySnapshot,
    IndexExclusionSemanticsSnapshot,
) {
    let base = base_snapshot();
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition("public", "bookings", RelationKind::Table)
                .unwrap(),
        ],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(index_coordinate(), IndexRelationKind::Index)
                .unwrap(),
        ],
    )
    .unwrap();
    let constraints = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexExclusionConstraintObservation::root(constraint_coordinate(), index_coordinate())
                .unwrap(),
        ],
    )
    .unwrap();
    let period = IndexExclusionConstraintPeriodSnapshot::new(
        &constraints,
        vec![
            IndexExclusionConstraintPeriodObservation::new(constraint_coordinate(), false).unwrap(),
        ],
    )
    .unwrap();
    let keys = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
        vec![
            IndexExclusionConstraintKeyObservation::new(constraint_coordinate(), vec![1]).unwrap(),
        ],
    )
    .unwrap();
    let families = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexKeyOperatorFamilyObservation::new(
                index_coordinate(),
                1,
                QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
                QualifiedOperatorFamilyName::new("btree", "pg_catalog", "integer_ops").unwrap(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let semantics = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![
            IndexKeyExclusionSemanticsObservation::new(
                index_coordinate(),
                1,
                qualified_operator("="),
                QualifiedProcedureSignature::new("pg_catalog", "int4eq", vec![int4.clone(), int4])
                    .unwrap(),
                3,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    (
        base,
        relations,
        indexes,
        constraints,
        period,
        keys,
        families,
        semantics,
    )
}

fn source_lineage<'a>(
    base: &'a PostgresSchemaSnapshotV3,
    relations: &'a RelationPartitionSnapshot,
    indexes: &'a IndexPartitionSnapshot,
    constraints: &'a IndexExclusionConstraintSnapshot,
    period: &'a IndexExclusionConstraintPeriodSnapshot,
    keys: &'a IndexExclusionConstraintKeySnapshot,
) -> IndexExclusionConstraintOperatorSourceLineage<'a> {
    IndexExclusionConstraintOperatorSourceLineage::new(
        base,
        relations,
        indexes,
        constraints,
        period,
        keys,
    )
}

fn semantics_lineage<'a>(
    families: &'a IndexOperatorFamilySnapshot,
    semantics: &'a IndexExclusionSemanticsSnapshot,
) -> IndexExclusionConstraintOperatorSemanticsLineage<'a> {
    IndexExclusionConstraintOperatorSemanticsLineage::new(families, semantics)
}

#[test]
fn ordinary_exclude_preserves_constraint_side_conexclop() {
    let (base, relations, indexes, constraints, period, keys, families, semantics) =
        predecessor_snapshots();
    let snapshot = IndexExclusionConstraintOperatorSnapshot::new(
        source_lineage(&base, &relations, &indexes, &constraints, &period, &keys),
        semantics_lineage(&families, &semantics),
        vec![
            IndexExclusionConstraintOperatorObservation::new(
                constraint_coordinate(),
                vec![qualified_operator("=")],
            )
            .unwrap(),
        ],
    )
    .expect("constraint-side conexclop agrees with the exact backing-index exclusion operator");

    assert_eq!(
        snapshot
            .source_receipt(constraint_coordinate())
            .unwrap()
            .location()
            .operators(),
        &[qualified_operator("=")]
    );
}

#[test]
fn ordinary_exclude_rejects_constraint_side_operator_drift() {
    let (base, relations, indexes, constraints, period, keys, families, semantics) =
        predecessor_snapshots();
    let error = IndexExclusionConstraintOperatorSnapshot::new(
        source_lineage(&base, &relations, &indexes, &constraints, &period, &keys),
        semantics_lineage(&families, &semantics),
        vec![
            IndexExclusionConstraintOperatorObservation::new(
                constraint_coordinate(),
                vec![qualified_operator("<")],
            )
            .unwrap(),
        ],
    )
    .expect_err(
        "pg_constraint.conexclop is independent evidence and must match conindid semantics",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_state",
        }
    );
}

#[test]
fn ordinary_exclude_constraint_operator_inventory_is_complete() {
    let (base, relations, indexes, constraints, period, keys, families, semantics) =
        predecessor_snapshots();
    let error = IndexExclusionConstraintOperatorSnapshot::new(
        source_lineage(&base, &relations, &indexes, &constraints, &period, &keys),
        semantics_lineage(&families, &semantics),
        vec![],
    )
    .expect_err("every ordinary EXCLUDE constraint must retain pg_constraint.conexclop");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_completeness",
        }
    );
}

#[test]
fn exact_conexclop_issues_domain_separated_provenance() {
    let (base, relations, indexes, constraints, period, keys, families, semantics) =
        predecessor_snapshots();
    let snapshot = IndexExclusionConstraintOperatorSnapshot::new(
        source_lineage(&base, &relations, &indexes, &constraints, &period, &keys),
        semantics_lineage(&families, &semantics),
        vec![
            IndexExclusionConstraintOperatorObservation::new(
                constraint_coordinate(),
                vec![qualified_operator("=")],
            )
            .unwrap(),
        ],
    )
    .unwrap();

    let receipt = snapshot.source_receipt(constraint_coordinate()).unwrap();
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/exclusion-operators")
    );
}
