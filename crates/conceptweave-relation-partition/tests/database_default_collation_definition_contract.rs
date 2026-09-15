use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    DatabaseDefaultCollationDefinitionObservation, PostgresDatabaseLocaleProvider,
};

fn database_default(
    locale: &str,
    recorded_version: &str,
    actual_version: &str,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        None,
        None,
        Some(locale.to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some(recorded_version.to_owned()),
        Some(actual_version.to_owned()),
    )
    .unwrap()
}

#[test]
fn same_recorded_database_version_with_changed_actual_provider_version_changes_identity() {
    let before = database_default("en-US-u-ks-level2", "153.80", "153.80");
    let provider_upgraded = database_default("en-US-u-ks-level2", "153.80", "154.10");

    assert_eq!(before.recorded_version(), provider_upgraded.recorded_version());
    assert!(!before.has_version_mismatch());
    assert!(provider_upgraded.has_version_mismatch());
    assert_ne!(before.actual_version(), provider_upgraded.actual_version());
    assert_ne!(before.canonical_digest(), provider_upgraded.canonical_digest());
}

#[test]
fn changed_database_default_locale_changes_effective_definition_identity() {
    let left = database_default("en-US-u-ks-level2", "153.80", "153.80");
    let right = database_default("ko-KR", "153.80", "153.80");

    assert_ne!(left.locale(), right.locale());
    assert_ne!(left.canonical_digest(), right.canonical_digest());
}

#[test]
fn database_locale_provider_accepts_only_postgresql18_database_provider_tokens() {
    assert_eq!(
        PostgresDatabaseLocaleProvider::try_from('b').unwrap(),
        PostgresDatabaseLocaleProvider::Builtin
    );
    assert_eq!(
        PostgresDatabaseLocaleProvider::try_from('c').unwrap(),
        PostgresDatabaseLocaleProvider::Libc
    );
    assert_eq!(
        PostgresDatabaseLocaleProvider::try_from('i').unwrap(),
        PostgresDatabaseLocaleProvider::Icu
    );

    for invalid in ['d', 'x'] {
        let error = PostgresDatabaseLocaleProvider::try_from(invalid)
            .expect_err("pg_database.datlocprovider admits only builtin/libc/icu in PostgreSQL 18");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "database_default_collation_provider",
            }
        );
    }
}
