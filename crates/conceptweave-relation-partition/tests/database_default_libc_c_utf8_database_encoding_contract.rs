use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    DatabaseDefaultCollationDefinitionObservation, PostgresDatabaseEncodingObservation,
    PostgresDatabaseLocaleProvider,
};

fn libc_database_default(
    lc_collate: &str,
    lc_ctype: &str,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Libc,
        Some(lc_collate.to_owned()),
        Some(lc_ctype.to_owned()),
        None,
        None,
        None,
        None,
    )
    .expect("C-family libc database defaults are valid representation inputs")
}

fn assert_database_encoding_error(
    result: Result<(), ObservationError>,
) {
    assert_eq!(
        result.expect_err(
            "PostgreSQL 18 must reject libc C-UTF8 locale evidence on a non-UTF8 database",
        ),
        ObservationError::InvalidObservationField {
            field: "database_default_collation_database_encoding",
        }
    );
}

#[test]
fn libc_c_utf8_locale_rejects_non_utf8_database_encoding() {
    let latin1 = PostgresDatabaseEncodingObservation::new(8)
        .expect("LATIN1 is a valid PostgreSQL 18 backend/database encoding");

    for locale in ["C.UTF-8", "C.utf8"] {
        assert_database_encoding_error(
            libc_database_default(locale, "C").validate_database_encoding(latin1),
        );
        assert_database_encoding_error(
            libc_database_default("C", locale).validate_database_encoding(latin1),
        );
        assert_database_encoding_error(
            libc_database_default(locale, locale).validate_database_encoding(latin1),
        );
    }
}

#[test]
fn libc_c_utf8_locale_remains_valid_on_utf8_database_encoding() {
    let utf8 = PostgresDatabaseEncodingObservation::new(6)
        .expect("UTF8 is a valid PostgreSQL 18 backend/database encoding");

    for locale in ["C.UTF-8", "C.utf8"] {
        libc_database_default(locale, locale)
            .validate_database_encoding(utf8)
            .expect("C-UTF8 libc locale evidence is source-compatible with a UTF8 database");
    }
}

#[test]
fn libc_c_and_posix_remain_encoding_independent_positive_controls() {
    let latin1 = PostgresDatabaseEncodingObservation::new(8)
        .expect("LATIN1 is a valid PostgreSQL 18 backend/database encoding");

    for locale in ["C", "POSIX"] {
        libc_database_default(locale, locale)
            .validate_database_encoding(latin1)
            .expect("PostgreSQL treats C/POSIX locale encodings as SQL_ASCII-compatible");
    }
}
