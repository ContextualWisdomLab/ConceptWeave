use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{CollationCatalogIdentity, PostgresDatabaseEncodingObservation};

#[test]
fn postgres18_database_encoding_rejects_catalog_identity_from_another_database_encoding() {
    let database_encoding = PostgresDatabaseEncodingObservation::new(6)
        .expect("UTF8 is a PostgreSQL 18 backend/database encoding");
    let incompatible = CollationCatalogIdentity::new("pg_catalog", "latin1_only", 8)
        .expect("the historical catalog identity deliberately preserves raw catalog evidence");

    let error = database_encoding
        .validate_collation_identity(&incompatible)
        .expect_err("a UTF8 database cannot issue an index collation resolved only for LATIN1");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_collation_database_encoding_binding",
        }
    );
}

#[test]
fn postgres18_database_encoding_accepts_database_specific_and_any_encoding_collations() {
    let database_encoding = PostgresDatabaseEncodingObservation::new(6).unwrap();
    let utf8 = CollationCatalogIdentity::new("pg_catalog", "ucs_basic", 6).unwrap();
    let any_encoding = CollationCatalogIdentity::new("pg_catalog", "C", -1).unwrap();

    database_encoding
        .validate_collation_identity(&utf8)
        .expect("a database-specific collation may match the exact database encoding");
    database_encoding
        .validate_collation_identity(&any_encoding)
        .expect("PostgreSQL permits encoding-independent collation rows with collencoding -1");
}

#[test]
fn postgres18_database_encoding_is_bounded_to_backend_encoding_ids() {
    let client_only = PostgresDatabaseEncodingObservation::new(35)
        .expect_err("client-only encodings cannot be a PostgreSQL database encoding");
    assert_eq!(
        client_only,
        ObservationError::InvalidObservationField {
            field: "postgres_database_encoding",
        }
    );

    let unused = PostgresDatabaseEncodingObservation::new(7)
        .expect_err("PG_UNUSED_1 is inside the enum range but is not a valid backend encoding");
    assert_eq!(
        unused,
        ObservationError::InvalidObservationField {
            field: "postgres_database_encoding",
        }
    );

    let sentinel = PostgresDatabaseEncodingObservation::new(-1)
        .expect_err("-1 is a collation sentinel, not a database encoding");
    assert_eq!(
        sentinel,
        ObservationError::InvalidObservationField {
            field: "postgres_database_encoding",
        }
    );

    PostgresDatabaseEncodingObservation::new(0)
        .expect("SQL_ASCII remains a valid PostgreSQL backend encoding ID");
    PostgresDatabaseEncodingObservation::new(34)
        .expect("KOI8U is the final valid PostgreSQL 18 backend encoding ID");
}
