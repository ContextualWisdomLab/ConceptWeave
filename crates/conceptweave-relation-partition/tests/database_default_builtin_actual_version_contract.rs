use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    DatabaseDefaultCollationDefinitionObservation, PostgresDatabaseLocaleProvider,
};

fn builtin_database_default(
    locale: &str,
    recorded_version: Option<&str>,
    actual_version: Option<&str>,
) -> Result<DatabaseDefaultCollationDefinitionObservation, ObservationError> {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Builtin,
        Some(locale.to_owned()),
        Some(locale.to_owned()),
        Some(locale.to_owned()),
        None,
        recorded_version.map(str::to_owned),
        actual_version.map(str::to_owned),
    )
}

#[test]
fn postgresql18_builtin_database_defaults_require_fixed_actual_version_one() {
    for locale in ["C", "C.UTF-8", "PG_UNICODE_FAST"] {
        let definition = builtin_database_default(locale, Some("1"), Some("1"))
            .expect("every PostgreSQL 18 built-in database locale reports actual version 1");
        assert_eq!(definition.actual_version(), Some("1"));
    }

    for actual_version in [None, Some("18"), Some("2")] {
        let error = builtin_database_default("C", Some("1"), actual_version).expect_err(
            "PostgreSQL 18 built-in database actual version cannot be absent or fabricated",
        );
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "database_default_collation_actual_version",
            }
        );
    }
}

#[test]
fn recorded_builtin_database_version_remains_independent_drift_evidence() {
    let missing_recorded = builtin_database_default("C", None, Some("1"))
        .expect("missing recorded datcollversion does not erase the fixed current version");
    assert!(missing_recorded.has_version_mismatch());

    let stale_recorded = builtin_database_default("C", Some("0"), Some("1"))
        .expect("a stale recorded datcollversion remains observable against current version 1");
    assert!(stale_recorded.has_version_mismatch());

    let current = builtin_database_default("C", Some("1"), Some("1"))
        .expect("matching built-in database versions remain valid");
    assert!(!current.has_version_mismatch());
}
