use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation,
    DatabaseDefaultCollationDefinitionObservation, PostgresCollationProvider,
    PostgresDatabaseLocaleProvider,
};

fn ordinary(
    recorded_version: Option<&str>,
    actual_version: &str,
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
        Some(actual_version.to_owned()),
    )
    .unwrap()
}

fn database_default(
    recorded_version: Option<&str>,
    actual_version: &str,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        None,
        recorded_version.map(str::to_owned),
        Some(actual_version.to_owned()),
    )
    .unwrap()
}

#[test]
fn ordinary_icu_collation_recorded_version_drift_remains_observable() {
    assert!(!ordinary(Some("153.80"), "153.80").has_version_mismatch());
    assert!(ordinary(None, "153.80").has_version_mismatch());
    assert!(ordinary(Some("153.80"), "154.10").has_version_mismatch());
}

#[test]
fn database_default_icu_recorded_version_drift_remains_observable() {
    assert!(!database_default(Some("153.80"), "153.80").has_version_mismatch());
    assert!(database_default(None, "153.80").has_version_mismatch());
    assert!(database_default(Some("153.80"), "154.10").has_version_mismatch());
}
