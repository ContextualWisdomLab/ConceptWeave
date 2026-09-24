use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn identity(name: &str, encoding: i32) -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("pg_catalog", name, encoding).unwrap()
}

fn assert_provider_shape_error(result: Result<CollationDefinitionObservation, ObservationError>) {
    assert_eq!(
        result.expect_err("impossible PostgreSQL 18 provider/field shapes must fail closed"),
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_provider_shape",
        }
    );
}

#[test]
fn libc_requires_both_lc_fields_and_forbids_provider_locale() {
    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("en_us", 6),
        PostgresCollationProvider::Libc,
        true,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        Some("en-US".to_owned()),
        None,
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    ));

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("en_us", 6),
        PostgresCollationProvider::Libc,
        true,
        Some("en_US.UTF-8".to_owned()),
        None,
        None,
        None,
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    ));
}

#[test]
fn non_libc_definitions_forbid_lc_fields_and_require_provider_locale() {
    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("unicode", -1),
        PostgresCollationProvider::Icu,
        true,
        Some("C".to_owned()),
        None,
        Some("und".to_owned()),
        None,
        Some("153.80".to_owned()),
        Some("153.80".to_owned()),
    ));

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("unicode", -1),
        PostgresCollationProvider::Icu,
        true,
        None,
        None,
        None,
        None,
        Some("153.80".to_owned()),
        Some("153.80".to_owned()),
    ));
}

#[test]
fn nondeterminism_and_icu_rules_are_icu_only() {
    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("pg_c_utf8", 6),
        PostgresCollationProvider::Builtin,
        false,
        None,
        None,
        Some("C.UTF-8".to_owned()),
        None,
        Some("1".to_owned()),
        Some("1".to_owned()),
    ));

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("en_us", 6),
        PostgresCollationProvider::Libc,
        true,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        None,
        Some("&V << w <<< W".to_owned()),
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    ));

    CollationDefinitionObservation::new(
        identity("casefolded", -1),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some("153.80".to_owned()),
        Some("153.80".to_owned()),
    )
    .expect("ICU is PostgreSQL 18's supported nondeterministic/rules provider");
}

#[test]
fn builtin_provider_accepts_only_postgresql18_builtin_locales() {
    for (locale, encoding) in [("C", -1), ("C.UTF-8", 6), ("PG_UNICODE_FAST", 6)] {
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
        .expect(
            "PostgreSQL 18 built-in locale with its canonical catalog encoding must be admitted",
        );
    }

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("builtin", 6),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("en-US".to_owned()),
        None,
        Some("1".to_owned()),
        Some("1".to_owned()),
    ));
}

#[test]
fn database_default_catalog_row_uses_the_bootstrap_shape_and_coordinate() {
    CollationDefinitionObservation::new(
        identity("default", -1),
        PostgresCollationProvider::DatabaseDefault,
        true,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .expect("PostgreSQL 18 bootstrap default collation row must remain representable");

    for invalid_identity in [
        CollationCatalogIdentity::new("public", "default", -1).unwrap(),
        identity("default", 6),
        identity("not_default", -1),
    ] {
        assert_provider_shape_error(CollationDefinitionObservation::new(
            invalid_identity,
            PostgresCollationProvider::DatabaseDefault,
            true,
            None,
            None,
            None,
            None,
            None,
            None,
        ));
    }

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("default", -1),
        PostgresCollationProvider::DatabaseDefault,
        false,
        None,
        None,
        None,
        None,
        None,
        None,
    ));

    assert_provider_shape_error(CollationDefinitionObservation::new(
        identity("default", -1),
        PostgresCollationProvider::DatabaseDefault,
        true,
        None,
        None,
        Some("und".to_owned()),
        None,
        None,
        None,
    ));
}
