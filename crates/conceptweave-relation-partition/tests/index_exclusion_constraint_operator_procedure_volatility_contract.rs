use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeyObservation,
    IndexExclusionConstraintKeySnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintOperatorKindObservation, IndexExclusionConstraintOperatorKindSnapshot,
    IndexExclusionConstraintOperatorObservation,
    IndexExclusionConstraintOperatorProcedureObservation,
    IndexExclusionConstraintOperatorProcedureScalarObservation,
    IndexExclusionConstraintOperatorProcedureScalarSnapshot,
    IndexExclusionConstraintOperatorProcedureSnapshot,
    IndexExclusionConstraintOperatorProcedureStrictnessObservation,
    IndexExclusionConstraintOperatorProcedureStrictnessSnapshot,
    IndexExclusionConstraintOperatorProcedureVolatilityObservation,
    IndexExclusionConstraintOperatorProcedureVolatilitySnapshot,
    IndexExclusionConstraintOperatorResultObservation,
    IndexExclusionConstraintOperatorResultSnapshot,
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

fn strictness_snapshot() -> IndexExclusionConstraintOperatorProcedureStrictnessSnapshot {
    let index = IndexObservation::new(
        "bookings_no_overlap",
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
        vec![ColumnObservationV3::new("resource_id", 1, "integer", int4(), false, None).unwrap()],
    )
    .unwrap()
    .with_indexes(vec![index])
    .unwrap();
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-procedure-volatility-v1",
        "2026-09-16T22:43:31Z",
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
    let semantics = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![
            IndexKeyExclusionSemanticsObservation::new(
                index_coordinate(),
                1,
                operator("="),
                procedure("int4eq"),
                3,
            )
            .unwrap(),
        ],
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
        vec![
            IndexExclusionConstraintOperatorObservation::new(coordinate(), vec![operator("=")])
                .unwrap(),
        ],
    )
    .unwrap();
    let procedures = IndexExclusionConstraintOperatorProcedureSnapshot::new(
        &operators,
        vec![
            IndexExclusionConstraintOperatorProcedureObservation::new(
                coordinate(),
                1,
                operator("="),
                procedure("int4eq"),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let results = IndexExclusionConstraintOperatorResultSnapshot::new(
        &procedures,
        vec![
            IndexExclusionConstraintOperatorResultObservation::new(
                coordinate(),
                1,
                operator("="),
                procedure("int4eq"),
                bool_type(),
                bool_type(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let kinds = IndexExclusionConstraintOperatorKindSnapshot::new(
        &results,
        vec![
            IndexExclusionConstraintOperatorKindObservation::new(
                coordinate(),
                1,
                operator("="),
                'b',
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let scalar = IndexExclusionConstraintOperatorProcedureScalarSnapshot::new(
        &results,
        &kinds,
        vec![
            IndexExclusionConstraintOperatorProcedureScalarObservation::new(
                coordinate(),
                1,
                operator("="),
                procedure("int4eq"),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    IndexExclusionConstraintOperatorProcedureStrictnessSnapshot::new(
        &scalar,
        vec![
            IndexExclusionConstraintOperatorProcedureStrictnessObservation::new(
                coordinate(),
                1,
                operator("="),
                procedure("int4eq"),
                true,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn volatility_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    volatility: char,
) -> IndexExclusionConstraintOperatorProcedureVolatilityObservation {
    IndexExclusionConstraintOperatorProcedureVolatilityObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        volatility,
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
fn ordinary_exclude_preserves_raw_procedure_volatility_and_provenance() {
    let strictness = strictness_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4eq"),
            'i',
        )],
    )
    .unwrap();

    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();
    assert_eq!(receipt.location().volatility(), 'i');
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), strictness.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        strictness.connection_policy_binding()
    );
    assert_eq!(
        receipt.extractor_revision(),
        strictness.extractor_revision()
    );
    assert_eq!(receipt.observed_at_utc(), strictness.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-volatility")
    );
}

#[test]
fn ordinary_exclude_domain_separates_all_postgres_volatility_states() {
    let strictness = strictness_snapshot();
    let immutable = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4eq"),
            'i',
        )],
    )
    .unwrap();
    let stable = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4eq"),
            's',
        )],
    )
    .unwrap();
    let volatile = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4eq"),
            'v',
        )],
    )
    .unwrap();

    assert_ne!(immutable.snapshot_digest(), stable.snapshot_digest());
    assert_ne!(immutable.snapshot_digest(), volatile.snapshot_digest());
    assert_ne!(stable.snapshot_digest(), volatile.snapshot_digest());
}

#[test]
fn procedure_volatility_observation_rejects_unknown_catalog_discriminator() {
    let error = IndexExclusionConstraintOperatorProcedureVolatilityObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        'x',
    )
    .expect_err("only PostgreSQL i/s/v provolatile values are admissible");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_volatility",
    );
}

#[test]
fn ordinary_exclude_rejects_procedure_volatility_binding_drift() {
    let strictness = strictness_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4ne"),
            'i',
        )],
    )
    .expect_err("provolatile evidence must bind to the exact pg_operator.oprcode procedure");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_volatility_binding",
    );
}

#[test]
fn ordinary_exclude_rejects_operator_volatility_binding_drift() {
    let strictness = strictness_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("<>"),
            procedure("int4eq"),
            'i',
        )],
    )
    .expect_err("provolatile evidence must remain on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_volatility_binding",
    );
}

#[test]
fn ordinary_exclude_rejects_missing_procedure_volatility_evidence() {
    let strictness = strictness_snapshot();
    let error =
        IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(&strictness, vec![])
            .expect_err("every governed ordinary EXCLUDE procedure needs raw provolatile evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_volatility_completeness",
    );
}

#[test]
fn ordinary_exclude_rejects_duplicate_procedure_volatility_coordinate() {
    let strictness = strictness_snapshot();
    let observation = volatility_observation(operator("="), procedure("int4eq"), 'i');
    let error = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate raw procedure volatility evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_volatility_coordinate",
    );
}

#[test]
fn procedure_volatility_observation_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureVolatilityObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        'i',
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn procedure_volatility_receipt_rejects_unknown_position() {
    let strictness = strictness_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureVolatilitySnapshot::new(
        &strictness,
        vec![volatility_observation(
            operator("="),
            procedure("int4eq"),
            'i',
        )],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("receipts may only be issued for governed positions");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}
