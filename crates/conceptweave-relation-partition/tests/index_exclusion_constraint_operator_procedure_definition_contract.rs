include!("index_exclusion_constraint_operator_procedure_leakproof_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureDefinitionObservation,
    IndexExclusionConstraintOperatorProcedureDefinitionSnapshot,
};

fn leakproof_snapshot() -> IndexExclusionConstraintOperatorProcedureLeakproofSnapshot {
    let security_definer = security_definer_snapshot();
    IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap()
}

fn definition_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    language_name: &str,
    prosrc: &str,
    probin: Option<&str>,
    prosqlbody: Option<&str>,
) -> IndexExclusionConstraintOperatorProcedureDefinitionObservation {
    IndexExclusionConstraintOperatorProcedureDefinitionObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        language_name,
        prosrc,
        probin.map(str::to_owned),
        prosqlbody.map(str::to_owned),
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_definition_preserves_content_bound_provenance() {
    let leakproof = leakproof_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            "int4eq",
            None,
            None,
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().language_name(), "internal");
    assert!(receipt.location().definition_digest().starts_with("sha256:"));
    assert_eq!(receipt.location().definition_digest().len(), 71);
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), leakproof.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        leakproof.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), leakproof.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), leakproof.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/procedure-definition"));
}

#[test]
fn ordinary_exclude_operator_procedure_definition_distinguishes_prosrc_changes() {
    let leakproof = leakproof_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "sql",
            "SELECT $1 = $2",
            None,
            None,
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "sql",
            "SELECT $1 <> $2",
            None,
            None,
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
    assert_ne!(
        first.source_receipt(coordinate(), 1).unwrap().location().definition_digest(),
        second.source_receipt(coordinate(), 1).unwrap().location().definition_digest()
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_distinguishes_binary_reference_changes() {
    let leakproof = leakproof_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "c",
            "int4eq",
            Some("$libdir/first"),
            None,
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "c",
            "int4eq",
            Some("$libdir/second"),
            None,
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_definition_distinguishes_preparsed_sql_body_changes() {
    let leakproof = leakproof_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "sql",
            "",
            None,
            Some("{QUERY :commandType 1 :querySource 0}"),
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "sql",
            "",
            None,
            Some("{QUERY :commandType 1 :querySource 1}"),
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_definition_distinguishes_language_changes() {
    let leakproof = leakproof_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "sql",
            "same text",
            None,
            None,
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "plpgsql",
            "same text",
            None,
            None,
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_blank_language() {
    let error = IndexExclusionConstraintOperatorProcedureDefinitionObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        "   ",
        "int4eq",
        None,
        None,
    )
    .expect_err("resolved pg_language identity must not be blank");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_definition_language",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_procedure_binding_drift() {
    let leakproof = leakproof_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4ne"),
            "internal",
            "int4ne",
            None,
            None,
        )],
    )
    .expect_err("definition evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_definition_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_operator_binding_drift() {
    let leakproof = leakproof_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("<>"),
            procedure("int4eq"),
            "internal",
            "int4eq",
            None,
            None,
        )],
    )
    .expect_err("definition evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_definition_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_missing_evidence() {
    let leakproof = leakproof_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(&leakproof, vec![])
        .expect_err("every governed operator function needs definition evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_definition_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_duplicate_coordinate() {
    let leakproof = leakproof_snapshot();
    let observation = definition_observation(
        operator("="),
        procedure("int4eq"),
        "internal",
        "int4eq",
        None,
        None,
    );
    let error = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate definition evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_definition_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureDefinitionObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        "internal",
        "int4eq",
        None,
        None,
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_definition_rejects_unknown_receipt_coordinate() {
    let leakproof = leakproof_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            "int4eq",
            None,
            None,
        )],
    )
    .unwrap();
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("receipt lookup must remain exact-coordinate and exact-position bound");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_operator_procedure_definition_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureDefinitionSnapshot>() > 0);
}
