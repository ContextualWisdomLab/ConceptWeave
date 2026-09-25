use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    DatabaseDefaultCollationDefinitionObservation, PostgresDatabaseLocaleProvider,
};

fn libc_database_default(
    lc_collate: &str,
    actual_version: Option<&str>,
) -> Result<DatabaseDefaultCollationDefinitionObservation, ObservationError> {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Libc,
        Some(lc_collate.to_owned()),
        Some(lc_collate.to_owned()),
        None,
        None,
        None,
        actual_version.map(str::to_owned),
    )
}

#[test]
fn postgresql18_unversioned_libc_database_defaults_reject_fabricated_actual_versions() {
    for locale in ["C", "c.UTF-8", "POSIX"] {
        let error = libc_database_default(locale, Some("2.39")).expect_err(
            "PostgreSQL 18 reports no libc actual version for C, C.*, or POSIX database locales",
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
fn postgresql18_unversioned_libc_database_defaults_preserve_null_actual_version() {
    for locale in ["C", "C.UTF-8", "POSIX"] {
        let definition = libc_database_default(locale, None)
            .expect("PostgreSQL 18 C-family and POSIX libc database locales are unversioned");
        assert_eq!(definition.actual_version(), None);
    }
}

#[test]
fn non_c_libc_database_defaults_keep_platform_dependent_version_availability() {
    let versioned = libc_database_default("en_US.UTF-8", Some("2.39"))
        .expect("a version-reporting libc platform may expose a concrete provider version");
    assert_eq!(versioned.actual_version(), Some("2.39"));

    let unavailable = libc_database_default("en_US.UTF-8", None)
        .expect("PostgreSQL may legitimately lack libc version data on some platforms");
    assert_eq!(unavailable.actual_version(), None);
}
