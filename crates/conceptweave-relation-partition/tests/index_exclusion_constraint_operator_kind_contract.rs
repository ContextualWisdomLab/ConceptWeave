use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintOperatorKindObservation, IndexExclusionConstraintOperatorKindSnapshot,
    IndexExclusionConstraintOperatorObservation, IndexExclusionConstraintOperatorProcedureObservation,
    IndexExclusionConstraintOperatorProcedureSnapshot, IndexExclusionConstraintOperatorResultObservation,
    IndexExclusionConstraintOperatorResultSnapshot,
    IndexExclusionConstraintOperatorSemanticsLineage, IndexExclusionConstraintOperatorSnapshot,
    IndexExclusionConstraintOperatorSourceLineage, IndexExclusionConstraintPeriodObservation,
    IndexExclusionConstraintPeriodSnapshot, IndexExclusionConstraintSnapshot,
    IndexExclusionSemanticsSnapshot, IndexKeyExclusionSemanticsObservation,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind, QualifiedOperatorFamilyName,
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

fn int4() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "int4").unwrap()
}

fn bool_type() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "bool").unwrap()
}

fn operator(name: &str) -> QualifiedOperatorSignature {
    QualifiedOperatorSignature::new("pg_catalog", name, int4(), int4()).unwrap()
}

fn procedure(name: &str) -> QualifiedProcedureSignature {
    QualifiedProcedureSignature::new("pg_catalog", name, vec![int4(), int4()]).unwrap()
}

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn index_coordinate() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn result_snapshot() -> IndexExclusionConstraintOperatorResultSnapshot {
    let index = IndexObservation::new(
        "bookings_no_overlap",
        false,
        Some(false),
        vec![IndexAttributeObservation::column(
            1,
            IndexAttributeKind::Key,
            "resource_id",
        )
        .unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
        0,
    )
    .unwrap()])
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
        vec![ColumnObservationV3::new(
            "resource_id",
            1,
            "integer",
            int4(),
            false,
            None,
        )
        .unwrap()],
    )
    .unwrap()
    .with_indexes(vec![index])
    .unwrap();
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-kind-v1",
        "2026-09-16T19:46:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap();
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
            coordinate(),
            index_coordinate(),
        )
        .unwrap()],
    )
    .unwrap();
    let periods = IndexExclusionConstraintPeriodSnapshot::new(
        &constraints,
        vec![IndexExclusionConstraintPeriodObservation::new(coordinate(), false).unwrap()],
    )
    .unwrap();
    let keys = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &periods,
        vec![IndexExclusionConstraintKeyObservation::new(coordinate(), vec![1]).unwrap()],
    )
    .unwrap();
    let families = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![IndexKeyOperatorFamilyObservation::new(
            index_coordinate(),
            1,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            QualifiedOperatorFamilyName::new("btree", "pg_catalog", "integer_ops").unwrap(),
        )
        .unwrap()],
    )
    .unwrap();
    let semantics = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![IndexKeyExclusionSemanticsObservation::new(
            index_coordinate(),
            1,
            operator("="),
            procedure("int4eq"),
            3,
        )
        .unwrap()],
    )
    .unwrap();
    let operators = IndexExclusionConstraintOperatorSnapshot::new(
        IndexExclusionConstraintOperatorSourceLineage::new(
            &base,
            &relations,
            &indexes,
            &constraints,
            &periods,
            &keys,
        ),
        IndexExclusionConstraintOperatorSemanticsLineage::new(&families, &semantics),
        vec![IndexExclusionConstraintOperatorObservation::new(
            coordinate(),
            vec![operator("=")],
        )
        .unwrap()],
    )
    .unwrap();
    let procedures = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![IndexExclusionConstraintOperatorProcedureObservation::new(
            coordinate(),
            1,
            operator("="),
            procedure("int4eq"),
        )
        .unwrap()],
    )
    .unwrap();
    IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![IndexExclusionConstraintOperatorResultObservation::new(
            coordinate(),
            1,
            operator("="),
            procedure("int4eq"),
            bool_type(),
            bool_type(),
        )
        .unwrap()],
    )
    .unwrap()
}

fn kind_observation(
    observed_operator: QualifiedOperatorSignature,
    operator_kind: char,
) -> IndexExclusionConstraintOperatorKindObservation {
    IndexExclusionConstraintOperatorKindObservation::new(
        coordinate(),
        1,
        observed_operator,
        operator_kind,
    )
    .unwrap()
}

fn assert_field(error: ObservationError, expected: &'static str) {
    assert_eq!(
        error,
        ObservationError::InvalidObservationField { field: expected }
    );
}

#[test]
fn ordinary_exclude_preserves_raw_binary_operator_kind() {
    let results = result_snapshot();
    let snapshot = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![kind_observation(operator("="), 'b')],
    )
    .expect("ordinary EXCLUDE must retain the raw binary pg_operator discriminator");

    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();
    assert_eq!(receipt.location().operator_kind(), 'b');
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.source_id(), results.source_connection_key());
    assert_eq!(receipt.connection_policy_binding(), results.connection_policy_binding());
    assert_eq!(receipt.extractor_revision(), results.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), results.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt.location().canonical_location().ends_with("/1/operator-kind"));
}

#[test]
fn ordinary_exclude_rejects_prefix_operator_kind() {
    let results = result_snapshot();
    let error = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![kind_observation(operator("="), 'l')],
    )
    .expect_err("normalized operand signatures cannot substitute for raw pg_operator.oprkind");
    assert_field(error, "index_exclusion_constraint_operator_kind_state");
}

#[test]
fn ordinary_exclude_rejects_unknown_operator_kind() {
    let results = result_snapshot();
    let error = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![kind_observation(operator("="), 'x')],
    )
    .expect_err("unknown raw operator discriminators must fail closed");
    assert_field(error, "index_exclusion_constraint_operator_kind_state");
}

#[test]
fn ordinary_exclude_rejects_operator_kind_binding_drift() {
    let results = result_snapshot();
    let error = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![kind_observation(operator("<>"), 'b')],
    )
    .expect_err("oprkind evidence must bind to the exact governed conexclop operator");
    assert_field(error, "index_exclusion_constraint_operator_kind_binding");
}

#[test]
fn ordinary_exclude_rejects_missing_operator_kind_evidence() {
    let results = result_snapshot();
    let error = IndexExclusionConstraintOperatorKindSnapshot::new(&results, vec![])
        .expect_err("every governed ordinary EXCLUDE operator position needs raw oprkind evidence");
    assert_field(error, "index_exclusion_constraint_operator_kind_completeness");
}

#[test]
fn ordinary_exclude_rejects_duplicate_operator_kind_coordinate() {
    let results = result_snapshot();
    let error = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![
            kind_observation(operator("="), 'b'),
            kind_observation(operator("="), 'b'),
        ],
    )
    .expect_err("duplicate raw catalog evidence must not collapse during canonicalization");
    assert_field(error, "index_exclusion_constraint_operator_kind_coordinate");
}

#[test]
fn operator_kind_observation_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorKindObservation::new(
        coordinate(),
        0,
        operator("="),
        'b',
    )
    .expect_err("operator kind positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn operator_kind_receipt_rejects_unknown_position() {
    let results = result_snapshot();
    let snapshot = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![kind_observation(operator("="), 'b')],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("receipts may only be issued for governed positions");
    assert!(matches!(error, ObservationError::UnknownObservationLocation { .. }));
}
