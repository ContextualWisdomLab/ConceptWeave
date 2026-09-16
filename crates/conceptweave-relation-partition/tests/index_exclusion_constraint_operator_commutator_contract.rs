use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintOperatorCommutatorObservation,
    IndexExclusionConstraintOperatorCommutatorSnapshot,
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

fn operator(name: &str) -> QualifiedOperatorSignature {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    QualifiedOperatorSignature::new("pg_catalog", name, int4.clone(), int4).unwrap()
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
            QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
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
        "extractor-index-exclusion-commutator-v1",
        "2026-09-16T15:20:00Z",
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
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let semantics = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![IndexKeyExclusionSemanticsObservation::new(
            index_coordinate(),
            1,
            operator("="),
            QualifiedProcedureSignature::new(
                "pg_catalog",
                "int4eq",
                vec![int4.clone(), int4],
            )
            .unwrap(),
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

fn observation(
    observed_operator: QualifiedOperatorSignature,
    commutator: QualifiedOperatorSignature,
) -> IndexExclusionConstraintOperatorCommutatorObservation {
    IndexExclusionConstraintOperatorCommutatorObservation::new(
        coordinate(),
        1,
        observed_operator,
        commutator,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_preserves_self_commutator_as_independent_evidence() {
    let operators = operator_snapshot();
    let snapshot = IndexExclusionConstraintOperatorCommutatorSnapshot::new(
        &operators,
        vec![observation(operator("="), operator("="))],
    )
    .expect("PostgreSQL EXCLUDE accepts an operator whose observed commutator is itself");

    let receipt = snapshot
        .source_receipt(coordinate(), 1)
        .expect("validated commutator evidence must issue provenance");
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().commutator(), &operator("="));
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt.location().canonical_location().ends_with("/1/commutator"));
}

#[test]
fn ordinary_exclude_rejects_non_self_commutator() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorCommutatorSnapshot::new(
        &operators,
        vec![observation(operator("="), operator("<>"))],
    )
    .expect_err("EXCLUDE operators must be self-commutative in PostgreSQL");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_commutator_state",
        }
    );
}

#[test]
fn ordinary_exclude_rejects_commutator_operator_binding_drift() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorCommutatorSnapshot::new(
        &operators,
        vec![observation(operator("<"), operator("<"))],
    )
    .expect_err("commutator evidence must bind to the exact governed conexclop operator");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_commutator_binding",
        }
    );
}

#[test]
fn ordinary_exclude_requires_complete_commutator_evidence() {
    let operators = operator_snapshot();
    let error = IndexExclusionConstraintOperatorCommutatorSnapshot::new(&operators, vec![])
        .expect_err("every ordinary EXCLUDE operator needs an explicit commutator observation");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_commutator_completeness",
        }
    );
}

#[test]
fn ordinary_exclude_rejects_duplicate_commutator_coordinates() {
    let operators = operator_snapshot();
    let entry = observation(operator("="), operator("="));
    let error = IndexExclusionConstraintOperatorCommutatorSnapshot::new(
        &operators,
        vec![entry.clone(), entry],
    )
    .expect_err("duplicate constraint/key commutator evidence must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_commutator_coordinate",
        }
    );
}

#[test]
fn commutator_observation_rejects_zero_key_position() {
    let error = IndexExclusionConstraintOperatorCommutatorObservation::new(
        coordinate(),
        0,
        operator("="),
        operator("="),
    )
    .expect_err("key positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn commutator_receipt_rejects_unknown_key_position() {
    let operators = operator_snapshot();
    let snapshot = IndexExclusionConstraintOperatorCommutatorSnapshot::new(
        &operators,
        vec![observation(operator("="), operator("="))],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("unobserved commutator coordinates cannot issue provenance");
    assert!(matches!(error, ObservationError::UnknownObservationLocation { .. }));
}
