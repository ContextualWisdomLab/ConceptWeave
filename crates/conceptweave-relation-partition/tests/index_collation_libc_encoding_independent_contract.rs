use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn libc(
    schema: &str,
    name: &str,
    encoding: i32,
    lc_collate: &str,
    lc_ctype: &str,
    version: Option<&str>,
    actual_version: Option<&str>,
) -> Result<CollationDefinitionObservation, ObservationError> {
    CollationDefinitionObservation::new(
        CollationCatalogIdentity::new(schema, name, encoding).unwrap(),
        PostgresCollationProvider::Libc,
        true,
        Some(lc_collate.to_owned()),
        Some(lc_ctype.to_owned()),
        None,
        None,
        version.map(str::to_owned),
        actual_version.map(str::to_owned),
    )
}

fn assert_provider_encoding_error(
    result: Result<CollationDefinitionObservation, ObservationError>,
) {
    assert_eq!(
        result.expect_err(
            "encoding-independent libc evidence must come from the PostgreSQL 18 C/POSIX lineage",
        ),
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_provider_encoding",
        }
    );
}

#[test]
fn encoding_independent_libc_rows_preserve_c_or_posix_locale_pairs() {
    libc("public", "copied_c", -1, "C", "C", None, None)
        .expect("CREATE COLLATION ... FROM pg_catalog.C may use any target identity");
    libc("tenant", "copied_posix", -1, "POSIX", "POSIX", None, None)
        .expect("CREATE COLLATION ... FROM pg_catalog.POSIX may use any target identity");

    assert_provider_encoding_error(libc(
        "public",
        "fabricated_encoding_independent_locale",
        -1,
        "en_US.UTF-8",
        "en_US.UTF-8",
        Some("2.39"),
        Some("2.39"),
    ));
    assert_provider_encoding_error(libc(
        "public",
        "fabricated_mixed_bootstrap_locales",
        -1,
        "C",
        "POSIX",
        None,
        None,
    ));
}

#[test]
fn database_specific_libc_rows_keep_their_concrete_backend_encoding() {
    libc(
        "public",
        "english_utf8",
        6,
        "en_US.UTF-8",
        "en_US.UTF-8",
        Some("2.39"),
        Some("2.39"),
    )
    .expect("ordinary libc creation/import uses a concrete backend encoding");
}
