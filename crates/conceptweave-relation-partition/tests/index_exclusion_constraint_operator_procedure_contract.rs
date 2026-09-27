use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintOperatorObservation,
    IndexExclusionConstraintOperatorProcedureObservation,
    IndexExclusionConstraintOperatorProcedureSnapshot,
    IndexExclusionConstraintOperatorSemanticsLineage, IndexExclusionConstraintOperatorSnapshot,
    IndexExclusionConstraintOperatorSourceLineage, IndexExclusionConstraintPeriodObservation,
    IndexExclusionConstraintPeriodSnapshot, IndexExclusionConstraintSnapshot,
    IndexExclusionSemanticsSnapshot, IndexKeyExclusionSemanticsObservation,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    QualifiedOperatorFamilyName, QualifiedOperatorSignature, QualifiedProcedureSignature,
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

fn operator(name: &str) -> QualifiedOperatorSignature {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    QualifiedOperatorSignature::new("pg_catalog", name, int4.clone(), int4).unwrap()
}

fn procedure(name: &str) -> QualifiedProcedureSignature {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    QualifiedProcedureSignature::new("pg_catalog", name, vec![int4.clone(), int4]).unwrap()
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
    operator_snapshot_for_types(&["int4"])
}

fn operator_snapshot_for_types(types: &[&str]) -> IndexExclusionConstraintOperatorSnapshot {
    let columns = types
        .iter()
        .enumerate()
        .map(|(offset, type_name)| {
            let position = u32::try_from(offset + 1).unwrap();
            ColumnObservationV3::new(
                if position == 1 {
                    "resource_id".to_owned()
                } else {
                    format!("resource_id_{position}")
                },
                position,
                if *type_name == "int4" {
                    "integer"
                } else {
                    "bigint"
                },
                QualifiedTypeName::new("pg_catalog", *type_name).unwrap(),
                false,
                None,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let operators = columns
        .iter()
        .map(|column| {
            QualifiedOperatorSignature::new(
                "pg_catalog",
                "=",
                column.type_binding().clone(),
                column.type_binding().clone(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let procedures = columns
        .iter()
        .map(|column| {
            QualifiedProcedureSignature::new(
                "pg_catalog",
                format!("{}eq", column.type_binding().type_name()),
                vec![column.type_binding().clone(), column.type_binding().clone()],
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let index = IndexObservation::new(
        "bookings_no_overlap",
        false,
        Some(false),
        columns
            .iter()
            .map(|column| {
                IndexAttributeObservation::column(
                    column.ordinal_position(),
                    IndexAttributeKind::Key,
                    column.column_name(),
                )
                .unwrap()
            })
            .collect(),
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(
        columns
            .iter()
            .map(|column| {
                IndexKeySemantics::new(
                    column.ordinal_position(),
                    None,
                    QualifiedOperatorClassName::new(
                        "pg_catalog",
                        format!("{}_ops", column.type_binding().type_name()),
                    )
                    .unwrap(),
                    0,
                )
                .unwrap()
            })
            .collect(),
    )
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(
        false, true, true, false, false, false,
    ))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true);
    let relation =
        RelationObservation::new("public", "bookings", RelationKind::Table, columns.clone())
            .unwrap()
            .with_indexes(vec![index])
            .unwrap();
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-procedure-v1",
        "2026-09-16T18:45:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap();
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
        vec![IndexExclusionConstraintObservation::root(coordinate(), index_coordinate()).unwrap()],
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
        vec![
            IndexExclusionConstraintKeyObservation::new(
                coordinate(),
                columns
                    .iter()
                    .map(|column| i16::try_from(column.ordinal_position()).unwrap())
                    .collect(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let families = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        columns
            .iter()
            .map(|column| {
                IndexKeyOperatorFamilyObservation::new(
                    index_coordinate(),
                    column.ordinal_position(),
                    QualifiedOperatorClassName::new(
                        "pg_catalog",
                        format!("{}_ops", column.type_binding().type_name()),
                    )
                    .unwrap(),
                    QualifiedOperatorFamilyName::new("btree", "pg_catalog", "integer_ops").unwrap(),
                )
                .unwrap()
            })
            .collect(),
    )
    .unwrap();
    let semantics = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        columns
            .iter()
            .enumerate()
            .map(|(offset, column)| {
                IndexKeyExclusionSemanticsObservation::new(
                    index_coordinate(),
                    column.ordinal_position(),
                    operators[offset].clone(),
                    procedures[offset].clone(),
                    3,
                )
                .unwrap()
            })
            .collect(),
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
        vec![IndexExclusionConstraintOperatorObservation::new(coordinate(), operators).unwrap()],
    )
    .unwrap()
}

fn observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
) -> IndexExclusionConstraintOperatorProcedureObservation {
    IndexExclusionConstraintOperatorProcedureObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_preserves_independent_operator_procedure_binding() {
    let operators = operator_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![observation(operator("="), procedure("int4eq"))],
    )
    .expect(
        "the independently resolved pg_operator.oprcode must match backing exclusion semantics",
    );
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.source_id(), operators.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        operators.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), operators.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), operators.observed_at_utc());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure")
    );
}

#[test]
fn ordinary_exclude_rejects_same_typed_wrong_operator_procedure() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![observation(operator("="), procedure("int4ne"))],
    )
    .expect_err("matching operand types do not prove that oprcode is the operator implementation");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_state",
        }
    );
}

#[test]
fn ordinary_exclude_rejects_operator_procedure_binding_drift() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![observation(operator("<>"), procedure("int4eq"))],
    )
    .expect_err("oprcode evidence must bind to the exact governed conexclop operator");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_binding",
        }
    );
}

#[test]
fn ordinary_exclude_requires_complete_operator_procedure_evidence() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSnapshot::new(&operators, vec![])
        .expect_err("every ordinary EXCLUDE operator needs an explicit oprcode observation");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_completeness",
        }
    );
}

#[test]
fn ordinary_exclude_rejects_duplicate_operator_procedure_coordinates() {
    let operators = operator_snapshot();
    let entry = observation(operator("="), procedure("int4eq"));
    let error = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![entry.clone(), entry],
    )
    .expect_err("duplicate constraint/key procedure evidence must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_coordinate",
        }
    );
}

#[test]
fn operator_procedure_observation_rejects_zero_key_position() {
    let error = IndexExclusionConstraintOperatorProcedureObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
    )
    .expect_err("key positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn operator_procedure_receipt_rejects_unknown_key_position() {
    let operators = operator_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![observation(operator("="), procedure("int4eq"))],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("unobserved procedure coordinates cannot issue provenance");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn differently_typed_keys_keep_exact_procedure_positions_under_permutation() {
    let operators = operator_snapshot_for_types(&["int4", "int8"]);
    let int8 = QualifiedTypeName::new("pg_catalog", "int8").unwrap();
    let second_operator =
        QualifiedOperatorSignature::new("pg_catalog", "=", int8.clone(), int8.clone()).unwrap();
    let second_procedure =
        QualifiedProcedureSignature::new("pg_catalog", "int8eq", vec![int8.clone(), int8]).unwrap();
    let observations = vec![
        observation(operator("="), procedure("int4eq")),
        IndexExclusionConstraintOperatorProcedureObservation::new(
            coordinate(),
            2,
            second_operator,
            second_procedure.clone(),
        )
        .unwrap(),
    ];
    let snapshot =
        IndexExclusionConstraintOperatorProcedureSnapshot::new(&operators, observations.clone())
            .unwrap();
    let mut reversed = observations.clone();
    reversed.reverse();
    assert_eq!(
        snapshot,
        IndexExclusionConstraintOperatorProcedureSnapshot::new(&operators, reversed).unwrap()
    );
    assert_eq!(
        snapshot
            .source_receipt(coordinate(), 2)
            .unwrap()
            .location()
            .procedure(),
        &second_procedure
    );
    let wrong = vec![
        observations[0].clone(),
        IndexExclusionConstraintOperatorProcedureObservation::new(
            coordinate(),
            2,
            operator("="),
            procedure("int4eq"),
        )
        .unwrap(),
    ];
    assert_eq!(
        IndexExclusionConstraintOperatorProcedureSnapshot::new(&operators, wrong).unwrap_err(),
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_binding"
        }
    );
}
