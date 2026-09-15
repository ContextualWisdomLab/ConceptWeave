use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation,
    DatabaseDefaultCollationDefinitionObservation, PostgresCollationProvider,
    PostgresDatabaseLocaleProvider,
};

fn ordinary(
    recorded_version: Option<&str>,
    actual_version: Option<&str>,
) -> CollationDefinitionObservation {
    CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "casefolded", -1).unwrap(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        None,
        recorded_version.map(str::to_owned),
        actual_version.map(str::to_owned),
    )
    .unwrap()
}

fn database_default(
    recorded_version: Option<&str>,
    actual_version: Option<&str>,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        None,
        recorded_version.map(str::to_owned),
        actual_version.map(str::to_owned),
    )
    .unwrap()
}

#[test]
fn ordinary_collation_version_availability_changes_are_mismatches() {
    assert!(!ordinary(None, None).has_version_mismatch());
    assert!(!ordinary(Some("153.80"), Some("153.80")).has_version_mismatch());
    assert!(ordinary(Some("153.80"), None).has_version_mismatch());
    assert!(ordinary(None, Some("153.80")).has_version_mismatch());
    assert!(ordinary(Some("153.80"), Some("154.10")).has_version_mismatch());
}

#[test]
fn database_default_version_availability_changes_are_mismatches() {
    assert!(!database_default(None, None).has_version_mismatch());
    assert!(!database_default(Some("153.80"), Some("153.80")).has_version_mismatch());
    assert!(database_default(Some("153.80"), None).has_version_mismatch());
    assert!(database_default(None, Some("153.80")).has_version_mismatch());
    assert!(database_default(Some("153.80"), Some("154.10")).has_version_mismatch());
}
