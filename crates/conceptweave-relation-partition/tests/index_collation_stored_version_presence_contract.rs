use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation, PostgresCollationProvider,
};

#[test]
fn ordinary_builtin_collation_requires_stored_version() {
    let missing = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "direct_c", -1).unwrap(),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("C".to_owned()),
        None,
        None,
        Some("1".to_owned()),
    );
    assert!(missing.is_err());

    let explicit_upgrade_version = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "direct_c", -1).unwrap(),
        PostgresCollationProvider::Builtin,
        true,
        None,
        None,
        Some("C".to_owned()),
        None,
        Some("legacy-version".to_owned()),
        Some("1".to_owned()),
    )
    .unwrap();
    assert!(explicit_upgrade_version.has_version_mismatch());
}

#[test]
fn ordinary_icu_collation_requires_stored_version() {
    let missing = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "casefolded", -1).unwrap(),
        PostgresCollationProvider::Icu,
        false,
        None,
        None,
        Some("und-u-ks-level2".to_owned()),
        None,
        None,
        Some("154.10".to_owned()),
    );
    assert!(missing.is_err());

    let stale = CollationDefinitionObservation::new(
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
    assert!(stale.has_version_mismatch());
}

#[test]
fn libc_unversioned_locale_keeps_null_stored_and_actual_versions() {
    let observed = CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("public", "c_locale", -1).unwrap(),
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

    assert_eq!(observed.version(), None);
    assert_eq!(observed.actual_version(), None);
    assert!(!observed.has_version_mismatch());
}
