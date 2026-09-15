use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn identity(name: &str, encoding: i32) -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("pg_catalog", name, encoding).unwrap()
}

fn builtin(locale: &str, encoding: i32) -> Result<CollationDefinitionObservation, ObservationError> {
    CollationDefinitionObservation::new(
        identity("builtin", encoding),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some(locale.to_owned()),
        None,
        Some("1".to_owned()),
        Some("1".to_owned()),
    )
}

fn icu(encoding: i32) -> Result<CollationDefinitionObservation, ObservationError> {
    CollationDefinitionObservation::new(
        identity("unicode", encoding),
        PostgresCollationProvider::Icu,
        true,
        None,
        None,
        Some("und".to_owned()),
        None,
        Some("153.80".to_owned()),
        Some("153.80".to_owned()),
    )
}

fn assert_provider_encoding_error(
    result: Result<CollationDefinitionObservation, ObservationError>,
) {
    assert_eq!(
        result.expect_err("impossible PostgreSQL 18 provider/catalog-encoding tuple must fail closed"),
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_provider_encoding",
        }
    );
}

#[test]
fn builtin_catalog_encoding_is_derived_from_the_builtin_locale() {
    builtin("C", -1).expect("PostgreSQL 18 built-in C is encoding-independent");
    builtin("C.UTF-8", 6).expect("PostgreSQL 18 built-in C.UTF-8 is UTF8-only");
    builtin("PG_UNICODE_FAST", 6)
        .expect("PostgreSQL 18 built-in PG_UNICODE_FAST is UTF8-only");

    assert_provider_encoding_error(builtin("C", 6));
    assert_provider_encoding_error(builtin("C.UTF-8", -1));
    assert_provider_encoding_error(builtin("PG_UNICODE_FAST", -1));
}

#[test]
fn postgresql18_ucs_basic_bootstrap_collation_is_utf8_builtin_c() {
    CollationDefinitionObservation::new(
        identity("ucs_basic", 6),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("C".to_owned()),
        None,
        Some("1".to_owned()),
        Some("1".to_owned()),
    )
    .expect("PostgreSQL 18 pg_catalog.ucs_basic is the canonical UTF8 built-in C bootstrap row");

    assert_provider_encoding_error(builtin("C", 6));
}

#[test]
fn icu_catalog_encoding_is_always_encoding_independent() {
    icu(-1).expect("PostgreSQL 18 ICU collations use collencoding -1");
    assert_provider_encoding_error(icu(6));
}
