include!("index_exclusion_constraint_operator_procedure_transform_extension_membership_contract.rs");

#[test]
fn ordinary_exclude_transform_extension_membership_preserves_quoted_whitespace_identifiers() {
    let language = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        " ",
        None,
    )
    .expect("PostgreSQL quoted language identifiers may consist of whitespace");
    assert_eq!(language.target_language_name(), " ");

    let extension = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        "internal",
        Some("\t".to_owned()),
    )
    .expect("PostgreSQL quoted extension identifiers may consist of whitespace");
    assert_eq!(extension.extension_name(), Some("\t"));
}
