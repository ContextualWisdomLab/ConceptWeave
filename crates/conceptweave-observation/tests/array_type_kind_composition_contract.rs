use conceptweave_observation::{
    ArrayTypeObservation, ColumnObservationV3, EnumObservation, PostgresSchemaSnapshotV3,
    PostgresTypeKind, QualifiedTypeName, RelationKind, RelationObservation, TypeKindObservation,
};

mod support;

fn type_name(schema_name: &str, type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema_name, type_name).expect("qualified type coordinate is valid")
}

#[test]
fn observed_type_kinds_preserve_an_already_observed_custom_true_array_binding() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let status_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("true-array fixture is valid");
    let ticket = RelationObservation::new(
        "public",
        "ticket",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "statuses",
            1,
            "public.status[]",
            type_name("public", "_status"),
            true,
            None,
        )
        .expect("array-bound column fixture is valid")],
    )
    .expect("relation fixture is valid");

    let snapshot = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T05:10:00Z",
        vec![ticket],
        Vec::new(),
        vec![status],
        vec![status_array],
    )
    .expect("the exact custom true-array coordinate is already valid source evidence");

    let result = snapshot.with_observed_type_kinds(vec![
        TypeKindObservation::plain(
            type_name("public", "_status"),
            PostgresTypeKind::Base,
        )
        .expect("PostgreSQL true arrays are base-kind pg_type rows"),
        TypeKindObservation::plain(
            type_name("public", "status"),
            PostgresTypeKind::Enum,
        )
        .expect("enum type-kind fixture is valid"),
    ]);

    assert!(
        result.is_ok(),
        "adding an independent type-kind family must not invalidate a custom true-array binding that was already admitted by exact pg_type.typarray/typelem evidence"
    );
}
