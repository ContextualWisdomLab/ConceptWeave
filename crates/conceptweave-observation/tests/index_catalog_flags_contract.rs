use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn catalog_flags(
    primary: bool,
    exclusion: bool,
    immediate: bool,
    clustered: bool,
    check_xmin: bool,
    replica_identity: bool,
) -> IndexCatalogFlags {
    IndexCatalogFlags::new(
        primary,
        exclusion,
        immediate,
        clustered,
        check_xmin,
        replica_identity,
    )
}

fn complete_index(
    is_unique: bool,
    flags: Option<IndexCatalogFlags>,
) -> Result<IndexObservation, ObservationError> {
    let index = IndexObservation::new(
        "document_id_ix",
        is_unique,
        Some(false),
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
                .expect("key fixture is valid"),
        ],
        Vec::new(),
    )?
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
                .expect("operator-class fixture is valid"),
            0,
        )
        .expect("key semantics fixture is valid"),
    ])?;

    match flags {
        Some(flags) => index.with_catalog_flags(flags),
        None => Ok(index),
    }
}

fn digest(index: IndexObservation) -> String {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "document_id",
                1,
                "bigint",
                catalog_type("int8"),
                false,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("complete index fixture is valid");

    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T08:55:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("snapshot fixture is valid")
    .snapshot_digest()
    .to_owned()
}

#[test]
fn each_observed_pg_index_catalog_flag_is_material_identity() {
    let base = digest(
        complete_index(
            true,
            Some(catalog_flags(false, false, false, false, false, false)),
        )
        .expect("base index fixture is valid"),
    );
    let variants = [
        catalog_flags(true, false, false, false, false, false),
        catalog_flags(false, true, false, false, false, false),
        catalog_flags(false, false, true, false, false, false),
        catalog_flags(false, false, false, true, false, false),
        catalog_flags(false, false, false, false, true, false),
        catalog_flags(false, false, false, false, false, true),
    ];

    for (index, flags) in variants.into_iter().enumerate() {
        let variant = digest(
            complete_index(true, Some(flags)).expect("catalog-flag variant is admissible"),
        );
        assert_ne!(
            base, variant,
            "material pg_index catalog-flag variant {index} must change v3 identity"
        );
    }
}

#[test]
fn unobserved_catalog_flags_do_not_collapse_into_observed_all_false() {
    let unobserved = digest(complete_index(true, None).expect("unobserved state is admissible"));
    let observed_false = digest(
        complete_index(
            true,
            Some(catalog_flags(false, false, false, false, false, false)),
        )
        .expect("observed all-false state is admissible"),
    );

    assert_ne!(unobserved, observed_false);
}

#[test]
fn primary_catalog_flag_requires_a_unique_index() {
    let error = complete_index(
        false,
        Some(catalog_flags(true, false, true, false, false, false)),
    )
    .expect_err("PostgreSQL primary indexes must also be unique");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_catalog_flags"
        }
    );
}
