use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation,
    DatabaseDefaultCollationDefinitionObservation, PostgresCollationProvider,
    PostgresDatabaseLocaleProvider,
};

#[test]
fn ordinary_icu_collation_requires_capture_time_actual_version() {
    let missing_actual_version = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "casefolded", -1).unwrap(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        None,
        Some("153.80".to_owned()),
        None,
    );

    assert!(missing_actual_version.is_err());

    let observed = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "casefolded", -1).unwrap(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        None,
        Some("153.80".to_owned()),
        Some("154.10".to_owned()),
    )
    .unwrap();

    assert_eq!(observed.actual_version(), Some("154.10"));
    assert!(observed.has_version_mismatch());
}

#[test]
fn database_default_icu_requires_capture_time_actual_version() {
    let missing_actual_version = DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        None,
        Some("153.80".to_owned()),
        None,
    );

    assert!(missing_actual_version.is_err());

    let observed = DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        None,
        Some("153.80".to_owned()),
        Some("154.10".to_owned()),
    )
    .unwrap();

    assert_eq!(observed.actual_version(), Some("154.10"));
    assert!(observed.has_version_mismatch());
}
