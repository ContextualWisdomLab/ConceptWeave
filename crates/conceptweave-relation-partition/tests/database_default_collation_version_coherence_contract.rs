use conceptweave_observation::ObservationError;
use conceptweave_relation_partition::{
    CollationCatalogIdentity, CollationDefinitionObservation,
    DatabaseDefaultCollationDefinitionObservation, PostgresCollationProvider,
    PostgresDatabaseLocaleProvider,
};

fn material_default(
    recorded_version: Option<&str>,
    actual_version: Option<&str>,
) -> Result<CollationDefinitionObservation, ObservationError> {
    CollationDefinitionObservation::new(
        CollationCatalogIdentity::new("pg_catalog", "default", -1).unwrap(),
        PostgresCollationProvider::DatabaseDefault,
        true,
        None,
        None,
        None,
        None,
        recorded_version.map(str::to_owned),
        actual_version.map(str::to_owned),
    )
}

fn database_default(
    actual_version: Option<&str>,
) -> DatabaseDefaultCollationDefinitionObservation {
    DatabaseDefaultCollationDefinitionObservation::new(
        PostgresDatabaseLocaleProvider::Icu,
        Some("C.UTF-8".to_owned()),
        Some("C.UTF-8".to_owned()),
        Some("und".to_owned()),
        None,
        Some("153.80".to_owned()),
        actual_version.map(str::to_owned),
    )
    .unwrap()
}

#[test]
fn postgres18_default_bootstrap_row_rejects_stored_pg_collation_version() {
    let error = material_default(Some("153.80"), Some("153.80"))
        .expect_err("PostgreSQL 18 pg_catalog.default has no stored pg_collation.collversion");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_collation_definition_default_stored_version",
        }
    );

    material_default(None, Some("153.80"))
        .expect("capture-time actual version remains valid for the default bootstrap row");
    material_default(None, None)
        .expect("providers without an actual version remain representable without synthesis");
}

#[test]
fn default_collation_actual_version_matches_database_default_actual_version() {
    let material = material_default(None, Some("153.80")).unwrap();
    let same = database_default(Some("153.80"));
    same.validate_material_default_collation(&material)
        .expect("both PostgreSQL 18 actual-version functions resolve the same current database locale");

    let contradictory = database_default(Some("154.10"));
    let error = contradictory
        .validate_material_default_collation(&material)
        .expect_err("one bounded observation cannot contain two actual versions for pg_catalog.default");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "database_default_collation_actual_version_coherence",
        }
    );

    let unavailable_material = material_default(None, None).unwrap();
    database_default(None)
        .validate_material_default_collation(&unavailable_material)
        .expect("matching unavailable actual-version evidence remains coherent");
}

#[test]
fn default_material_version_mismatch_delegates_to_database_definition() {
    let material = material_default(None, Some("153.80")).unwrap();
    assert!(
        !material.has_version_mismatch(),
        "pg_catalog.default has no stored collversion; database-level evidence owns version drift"
    );

    let database = database_default(Some("154.10"));
    assert!(
        database.has_version_mismatch(),
        "pg_database.datcollversion remains the recorded baseline for the delegated default"
    );
}
