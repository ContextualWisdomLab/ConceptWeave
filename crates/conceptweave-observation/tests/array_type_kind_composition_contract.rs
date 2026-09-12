use conceptweave_observation::{
    ArrayTypeObservation, ColumnObservationV3, EnumObservation, PostgresSchemaSnapshotV3,
    PostgresTypeKind, QualifiedTypeName, RelationKind, RelationObservation, TypeKindObservation,
};

mod support;

fn type_name(schema_name: &str, type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema_name, type_name).expect("qualified type coordinate is valid")
}

fn status_enum() -> EnumObservation {
    EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid")
}

fn ticket_with_status_array_binding() -> RelationObservation {
    RelationObservation::new(
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
    .expect("relation fixture is valid")
}

fn ticket_with_custom_base_binding() -> RelationObservation {
    RelationObservation::new(
        "public",
        "vector_ticket",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "embedding",
            1,
            "public.vector3",
            type_name("public", "vector3"),
            false,
            None,
        )
        .expect("custom base-bound column fixture is valid")],
    )
    .expect("relation fixture is valid")
}

fn status_type_kinds() -> Vec<TypeKindObservation> {
    vec![
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
    ]
}

#[test]
fn observed_type_kinds_preserve_an_already_observed_custom_true_array_binding() {
    let status_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("true-array fixture is valid");
    let snapshot = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T05:10:00Z",
        vec![ticket_with_status_array_binding()],
        Vec::new(),
        vec![status_enum()],
        vec![status_array],
    )
    .expect("the exact custom true-array coordinate is already valid source evidence");
    let array_aware_digest = snapshot.snapshot_digest().to_owned();

    let composed = snapshot
        .with_observed_type_kinds(status_type_kinds())
        .expect("independent array and type-kind catalog families must compose");

    assert!(composed.array_types().is_some());
    assert!(composed.type_kinds().is_some());
    assert_ne!(array_aware_digest, composed.snapshot_digest());
}

#[test]
fn base_type_kind_resolves_exact_binding_without_inventing_true_array_identity() {
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T05:11:00Z",
        vec![ticket_with_status_array_binding()],
        Vec::new(),
        vec![status_enum()],
        status_type_kinds(),
    )
    .expect("Base kind proves the exact pg_type coordinate, not true-array identity");

    assert!(snapshot.type_kinds().is_some());
    assert!(snapshot.array_types().is_none());
}

#[test]
fn ordinary_user_defined_base_type_kind_resolves_its_exact_binding() {
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T05:12:00Z",
        vec![ticket_with_custom_base_binding()],
        Vec::new(),
        Vec::new(),
        vec![TypeKindObservation::plain(
            type_name("public", "vector3"),
            PostgresTypeKind::Base,
        )
        .expect("custom base type-kind fixture is valid")],
    )
    .expect("an exact user-defined base type coordinate must be a resolvable binding");

    assert!(snapshot.type_kinds().is_some());
    assert!(snapshot.array_types().is_none());
}
