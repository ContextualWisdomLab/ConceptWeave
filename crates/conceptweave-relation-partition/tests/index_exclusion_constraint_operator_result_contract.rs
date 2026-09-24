use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
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

fn operator_snapshot() -> IndexExclusionConstraintOperatorSnapshot {
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
        "extractor-index-exclusion-result-v1",
        "2026-09-16T19:20:00Z",
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
    let period = IndexExclusionConstraintPeriodSnapshot::new(
        &constraints,
        vec![IndexExclusionConstraintPeriodObservation::new(coordinate(), false).unwrap()],
    )
    .unwrap();
    let keys = IndexExclusionConstraintKeySnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &period,
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
    IndexExclusionConstraintOperatorSnapshot::new(
        IndexExclusionConstraintOperatorSourceLineage::new(
            &base,
            &relations,
            &indexes,
            &constraints,
            &period,
            &keys,
        ),
        IndexExclusionConstraintOperatorSemanticsLineage::new(&families, &semantics),
        vec![IndexExclusionConstraintOperatorObservation::new(
            coordinate(),
            vec![operator("=")],
        )
        .unwrap()],
    )
    .unwrap()
}

fn procedure_snapshot() -> IndexExclusionConstraintOperatorProcedureSnapshot {
    let operators = operator_snapshot();
    IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![IndexExclusionConstraintOperatorProcedureObservation::new(
            coordinate(),
            1,
            operator("="),
            procedure("int4eq"),
        )
        .unwrap()],
    )
    .unwrap()
}

fn result_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    operator_result_type: QualifiedTypeName,
    procedure_result_type: QualifiedTypeName,
) -> IndexExclusionConstraintOperatorResultObservation {
    IndexExclusionConstraintOperatorResultObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        operator_result_type,
        procedure_result_type,
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
fn ordinary_exclude_preserves_independent_boolean_result_contract() {
    let procedures = procedure_snapshot();
    let snapshot = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            bool_type(),
            bool_type(),
        )],
    )
    .expect("ordinary EXCLUDE search operators and their implementation functions must return bool");

    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();
    assert_eq!(receipt.location().operator_result_type(), &bool_type());
    assert_eq!(receipt.location().procedure_result_type(), &bool_type());
    assert_eq!(receipt.source_id(), procedures.source_connection_key());
    assert_eq!(receipt.connection_policy_binding(), procedures.connection_policy_binding());
    assert_eq!(receipt.extractor_revision(), procedures.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), procedures.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/result-contract"));
}

#[test]
fn ordinary_exclude_rejects_non_boolean_operator_result() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            int4(),
            bool_type(),
        )],
    )
    .expect_err("oprresult is independent source state and cannot be inferred from procedure identity");
    assert_field(error, "index_exclusion_constraint_operator_result_state");
}

#[test]
fn ordinary_exclude_rejects_non_boolean_procedure_result() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            bool_type(),
            int4(),
        )],
    )
    .expect_err("prorettype must independently agree with the operator Boolean result contract");
    assert_field(error, "index_exclusion_constraint_operator_result_state");
}

#[test]
fn ordinary_exclude_rejects_matching_non_boolean_results() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            int4(),
            int4(),
        )],
    )
    .expect_err("agreement alone is insufficient because index search operators must return bool");
    assert_field(error, "index_exclusion_constraint_operator_result_state");
}

#[test]
fn ordinary_exclude_rejects_boolean_type_from_wrong_schema() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            QualifiedTypeName::new("public", "bool").unwrap(),
            QualifiedTypeName::new("public", "bool").unwrap(),
        )],
    )
    .expect_err("type names must resolve to the canonical pg_catalog bool identity");
    assert_field(error, "index_exclusion_constraint_operator_result_state");
}

#[test]
fn ordinary_exclude_rejects_operator_result_binding_drift() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("<>"),
            procedure("int4eq"),
            bool_type(),
            bool_type(),
        )],
    )
    .expect_err("result evidence must bind to the exact governed conexclop operator");
    assert_field(error, "index_exclusion_constraint_operator_result_binding");
}

#[test]
fn ordinary_exclude_rejects_procedure_result_binding_drift() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4ne"),
            bool_type(),
            bool_type(),
        )],
    )
    .expect_err("result evidence must bind to the exact governed oprcode procedure");
    assert_field(error, "index_exclusion_constraint_operator_result_binding");
}

#[test]
fn ordinary_exclude_requires_complete_operator_result_evidence() {
    let procedures = procedure_snapshot();
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(&procedures, vec![])
        .expect_err("every governed operator position needs explicit oprresult/prorettype evidence");
    assert_field(error, "index_exclusion_constraint_operator_result_completeness");
}

#[test]
fn ordinary_exclude_rejects_duplicate_operator_result_coordinates() {
    let procedures = procedure_snapshot();
    let entry = result_observation(
        operator("="),
        procedure("int4eq"),
        bool_type(),
        bool_type(),
    );
    let error = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![entry.clone(), entry],
    )
    .expect_err("duplicate result-contract evidence must fail closed");
    assert_field(error, "index_exclusion_constraint_operator_result_coordinate");
}

#[test]
fn operator_result_observation_rejects_zero_key_position() {
    let error = IndexExclusionConstraintOperatorResultObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        bool_type(),
        bool_type(),
    )
    .expect_err("key positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn operator_result_receipt_rejects_unknown_key_position() {
    let procedures = procedure_snapshot();
    let snapshot = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![result_observation(
            operator("="),
            procedure("int4eq"),
            bool_type(),
            bool_type(),
        )],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("unobserved result coordinates cannot issue provenance");
    assert!(matches!(error, ObservationError::UnknownObservationLocation { .. }));
}
