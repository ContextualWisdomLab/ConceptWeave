use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    DatabaseDefaultCollationDefinitionObservation, PostgresDatabaseEncodingObservation,
    PostgresDatabaseLocaleProvider,
};

fn database_default(
    locale: &str,
    recorded_version: &str,
    actual_version: &str,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some(locale.to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some(recorded_version.to_owned()),
        Some(actual_version.to_owned()),
    )
    .unwrap()
}

fn builtin_database_default(locale: &str) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Builtin,
        Some(locale.to_owned()),
        Some(locale.to_owned()),
        Some(locale.to_owned()),
        None,
        Some("1".to_owned()),
        Some("1".to_owned()),
    )
    .unwrap()
}

fn assert_provider_shape_error(
    result: Result<DatabaseDefaultCollationDefinitionObservation, ObservationError>,
) {
    assert_eq!(
        result.expect_err("impossible PostgreSQL 18 pg_database locale shapes must fail closed"),
        ObservationError::InvalidObservationField {
            field: "database_default_collation_provider_shape",
        }
    );
}

#[test]
fn same_recorded_database_version_with_changed_actual_provider_version_changes_identity() {
    let before = database_default("en-US-u-ks-level2", "153.80", "153.80");
    let provider_upgraded = database_default("en-US-u-ks-level2", "153.80", "154.10");

    assert_eq!(
        before.recorded_version(),
        provider_upgraded.recorded_version()
    );
    assert!(!before.has_version_mismatch());
    assert!(provider_upgraded.has_version_mismatch());
    assert_ne!(before.actual_version(), provider_upgraded.actual_version());
    assert_ne!(
        before.canonical_digest(),
        provider_upgraded.canonical_digest()
    );
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

#[test]
fn database_catalog_requires_lc_collate_and_lc_ctype_for_every_provider() {
    for provider in [
        PostgresDatabaseLocaleProvider::Builtin,
        PostgresDatabaseLocaleProvider::Libc,
        PostgresDatabaseLocaleProvider::Icu,
    ] {
        let locale = if provider == PostgresDatabaseLocaleProvider::Libc {
            None
        } else {
            Some("und".to_owned())
        };
        assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
            provider,
            None,
            Some("C.UTF-8".to_owned()),
            locale.clone(),
            None,
            None,
            None,
        ));
        assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
            provider,
            Some("C.UTF-8".to_owned()),
            None,
            locale,
            None,
            None,
            None,
        ));
    }
}

#[test]
fn libc_forbids_datlocale_and_icu_rules() {
    assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Libc,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        Some("en-US".to_owned()),
        None,
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    ));
    assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Libc,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        None,
        Some("&V << w <<< W".to_owned()),
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    ));

    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Libc,
        Some("en_US.UTF-8".to_owned()),
        Some("en_US.UTF-8".to_owned()),
        None,
        None,
        Some("2.39".to_owned()),
        Some("2.39".to_owned()),
    )
    .expect("libc database default uses datcollate/datctype and no datlocale");
}

#[test]
fn non_libc_requires_datlocale_and_icu_rules_are_icu_only() {
    for provider in [
        PostgresDatabaseLocaleProvider::Builtin,
        PostgresDatabaseLocaleProvider::Icu,
    ] {
        assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
            provider,
            Some("C.UTF-8".to_owned()),
            Some("C.UTF-8".to_owned()),
            None,
            None,
            None,
            None,
        ));
    }

    assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Builtin,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some("1".to_owned()),
        Some("1".to_owned()),
    ));

    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        Some("&V << w <<< W".to_owned()),
        Some("153.80".to_owned()),
        Some("153.80".to_owned()),
    )
    .expect("ICU database default may carry ICU rules");
}

#[test]
fn builtin_database_default_accepts_only_postgresql18_builtin_locales() {
    for locale in ["C", "C.UTF-8", "PG_UNICODE_FAST"] {
        builtin_database_default(locale);
    }

    for locale in ["und", "en-US", "ko-KR"] {
        assert_provider_shape_error(DatabaseDefaultCollationDefinitionObservation::new(
            PostgresDatabaseLocaleProvider::Builtin,
            Some(locale.to_owned()),
            Some(locale.to_owned()),
            Some(locale.to_owned()),
            None,
            Some("1".to_owned()),
            Some("1".to_owned()),
        ));
    }
}

#[test]
fn builtin_utf8_only_database_locales_reject_non_utf8_database_encoding() {
    let latin1 = PostgresDatabaseEncodingObservation::new(8)
        .expect("LATIN1 remains a valid PostgreSQL 18 backend/database encoding");

    for locale in ["C.UTF-8", "PG_UNICODE_FAST"] {
        let error = builtin_database_default(locale)
            .validate_database_encoding(latin1)
            .expect_err("PostgreSQL 18 UTF8-only built-in database locales must fail on LATIN1");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "database_default_collation_database_encoding",
            }
        );
    }
}

#[test]
fn builtin_database_locale_encoding_positive_controls_remain_valid() {
    let latin1 = PostgresDatabaseEncodingObservation::new(8).unwrap();
    builtin_database_default("C")
        .validate_database_encoding(latin1)
        .expect("the built-in C locale remains valid across PostgreSQL backend encodings");

    let utf8 = PostgresDatabaseEncodingObservation::new(6).unwrap();
    for locale in ["C.UTF-8", "PG_UNICODE_FAST"] {
        builtin_database_default(locale)
            .validate_database_encoding(utf8)
            .expect("PostgreSQL 18 UTF8-only built-in locale must remain valid on UTF8");
    }
}

#[test]
fn icu_database_default_rejects_postgresql18_icu_unsupported_database_encodings() {
    let definition = database_default("und", "153.80", "153.80");

    for encoding_id in [0, 5, 7, 17, 21] {
        let encoding = PostgresDatabaseEncodingObservation::new(encoding_id)
            .expect("the ICU-negative witnesses are valid PostgreSQL 18 backend encodings");
        let error = definition
            .validate_database_encoding(encoding)
            .expect_err("PostgreSQL 18 must reject database encodings absent from pg_enc2icu_tbl");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "database_default_collation_database_encoding",
            }
        );
    }
}

#[test]
fn icu_database_default_accepts_exact_postgresql18_icu_supported_encoding_set() {
    let definition = database_default("und", "153.80", "153.80");

    for encoding_id in [
        1, 2, 3, 4, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 22, 23, 24, 25, 26, 27, 28,
        29, 30, 31, 32, 33, 34,
    ] {
        let encoding = PostgresDatabaseEncodingObservation::new(encoding_id)
            .expect("the ICU-positive witnesses are valid PostgreSQL 18 backend encodings");
        definition
            .validate_database_encoding(encoding)
            .expect("PostgreSQL 18 pg_enc2icu_tbl marks this backend encoding ICU-capable");
    }
}
