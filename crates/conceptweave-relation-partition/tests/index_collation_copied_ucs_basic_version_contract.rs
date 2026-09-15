use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn copied_ucs_basic(
    version: Option<&str>,
) -> Result<CollationDefinitionObservation, ObservationError> {
    CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "ucs_basic_copy", 6).unwrap(),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("C".to_owned()),
        None,
        version.map(str::to_owned),
        Some("1".to_owned()),
    )
}

fn assert_copied_ucs_basic_version_error(
    result: Result<CollationDefinitionObservation, ObservationError>,
) {
    assert_eq!(
        result.expect_err(
            "PostgreSQL 18 UTF8 built-in C rows descend from ucs_basic and store version 1",
        ),
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_copied_ucs_basic_version",
        }
    );
}

#[test]
fn copied_ucs_basic_requires_the_postgresql18_stored_version() {
    copied_ucs_basic(Some("1"))
        .expect("CREATE COLLATION ... FROM pg_catalog.ucs_basic recomputes stored version 1");

    assert_copied_ucs_basic_version_error(copied_ucs_basic(None));
    assert_copied_ucs_basic_version_error(copied_ucs_basic(Some("18")));
}

#[test]
fn direct_builtin_c_version_override_is_not_mistaken_for_ucs_basic_lineage() {
    CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "direct_builtin_c", -1).unwrap(),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("C".to_owned()),
        None,
        Some("operator-supplied-version".to_owned()),
        Some("1".to_owned()),
    )
    .expect("direct built-in C can carry an explicit VERSION while remaining encoding-independent");
}
