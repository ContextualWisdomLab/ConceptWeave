use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

fn identity() -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("public", "casefolded", -1).unwrap()
}

fn icu_definition(recorded_version: &str, actual_version: &str) -> CollationDefinitionObservation {
    CollationDefinitionObservation::new(
        identity(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some(recorded_version.to_owned()),
        Some(actual_version.to_owned()),
    )
    .unwrap()
}

#[test]
fn same_catalog_coordinate_with_a_different_recorded_version_is_not_the_same_definition() {
    let before = icu_definition("153.80", "153.80");
    let after = icu_definition("154.10", "154.10");

    assert_eq!(before.identity(), after.identity());
    assert_ne!(before, after);
    assert_ne!(before.canonical_digest(), after.canonical_digest());
}

#[test]
fn provider_upgrade_before_refresh_changes_governed_definition_identity() {
    let before = icu_definition("153.80", "153.80");
    let provider_upgraded = icu_definition("153.80", "154.10");

    assert_eq!(before.identity(), provider_upgraded.identity());
    assert_eq!(before.version(), provider_upgraded.version());
    assert!(!before.has_version_mismatch());
    assert!(provider_upgraded.has_version_mismatch());
    assert_ne!(before.actual_version(), provider_upgraded.actual_version());
    assert_ne!(before.canonical_digest(), provider_upgraded.canonical_digest());
}

#[test]
fn same_catalog_coordinate_with_different_provider_semantics_is_not_the_same_definition() {
    let icu = icu_definition("153.80", "153.80");
    let libc = CollationDefinitionObservation::new(
        identity(),
        PostgresCollationProvider::Libc,
        true,
        Some("C".to_owned()),
        Some("C".to_owned()),
        None,
        None,
        None,
        None,
    )
    .unwrap();

    assert_eq!(icu.identity(), libc.identity());
    assert_ne!(icu.canonical_digest(), libc.canonical_digest());
}

#[test]
fn postgres18_libc_c_family_has_no_capture_time_actual_version() {
    for locale in ["C", "c", "C.UTF-8", "c.utf8", "POSIX", "posix"] {
        let error = CollationDefinitionObservation::new(
            identity(),
            PostgresCollationProvider::Libc,
            true,
            Some(locale.to_owned()),
            Some(locale.to_owned()),
            None,
            None,
            Some("operator-supplied-recorded-version".to_owned()),
            Some("fabricated-provider-version".to_owned()),
        )
        .expect_err(
            "PostgreSQL 18 returns NULL actual version for libc C, C.*, and POSIX locales",
        );
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "index_collation_definition_actual_version",
            }
        );

        let observation = CollationDefinitionObservation::new(
            identity(),
            PostgresCollationProvider::Libc,
            true,
            Some(locale.to_owned()),
            Some(locale.to_owned()),
            None,
            None,
            Some("operator-supplied-recorded-version".to_owned()),
            None,
        )
        .expect("stored version drift remains representable when actual provider version is NULL");
        assert_eq!(observation.version(), Some("operator-supplied-recorded-version"));
        assert_eq!(observation.actual_version(), None);
        assert!(observation.has_version_mismatch());
    }
}

#[test]
fn postgres18_collation_provider_tokens_fail_closed() {
    assert_eq!(
        PostgresCollationProvider::try_from('d').unwrap(),
        PostgresCollationProvider::DatabaseDefault
    );
    assert_eq!(
        PostgresCollationProvider::try_from('b').unwrap(),
        PostgresCollationProvider::Builtin
    );
    assert_eq!(
        PostgresCollationProvider::try_from('c').unwrap(),
        PostgresCollationProvider::Libc
    );
    assert_eq!(
        PostgresCollationProvider::try_from('i').unwrap(),
        PostgresCollationProvider::Icu
    );

    let error = PostgresCollationProvider::try_from('x')
        .expect_err("unknown pg_collation.collprovider values must not enter governed evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_provider",
        }
    );
}
