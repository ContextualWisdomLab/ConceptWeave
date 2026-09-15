use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn identity() -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("public", "casefolded", 6).unwrap()
}

fn icu_definition(version: &str) -> CollationDefinitionObservation {
    CollationDefinitionObservation::new(
        identity(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some(version.to_owned()),
    )
    .unwrap()
}

#[test]
fn same_catalog_coordinate_with_a_different_recorded_version_is_not_the_same_definition() {
    let before = icu_definition("153.80");
    let after = icu_definition("154.10");

    assert_eq!(before.identity(), after.identity());
    assert_ne!(before, after);
    assert_ne!(before.canonical_digest(), after.canonical_digest());
}

#[test]
fn same_catalog_coordinate_with_different_provider_semantics_is_not_the_same_definition() {
    let icu = icu_definition("153.80");
    let libc = CollationDefinitionObservation::new(
        identity(),
        PostgresCollationProvider::Libc,
        true,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        None,
        None,
        Some("2.39".to_owned()),
    )
    .unwrap();

    assert_eq!(icu.identity(), libc.identity());
    assert_ne!(icu.canonical_digest(), libc.canonical_digest());
}

#[test]
fn postgres18_collation_provider_tokens_fail_closed() {
    assert_eq!(PostgresCollationProvider::try_from('d').unwrap(), PostgresCollationProvider::DatabaseDefault);
    assert_eq!(PostgresCollationProvider::try_from('b').unwrap(), PostgresCollationProvider::Builtin);
    assert_eq!(PostgresCollationProvider::try_from('c').unwrap(), PostgresCollationProvider::Libc);
    assert_eq!(PostgresCollationProvider::try_from('i').unwrap(), PostgresCollationProvider::Icu);

    let error = PostgresCollationProvider::try_from('x')
        .expect_err("unknown pg_collation.collprovider values must not enter governed evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_provider",
        }
    );
}
