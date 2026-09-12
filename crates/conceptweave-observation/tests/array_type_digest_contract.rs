use conceptweave_observation::{
    ArrayTypeObservation, ColumnObservationV3, EnumObservation, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation, SchemaObjectLocation,
};

mod support;

fn type_name(schema: &str, name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema, name).expect("qualified type coordinate is valid")
}

fn observed_enum(name: &str) -> EnumObservation {
    EnumObservation::new(
        "public",
        name,
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid")
}

fn array_type(array_name: &str, element_name: &str) -> ArrayTypeObservation {
    ArrayTypeObservation::new(
        type_name("public", array_name),
        type_name("public", element_name),
    )
    .expect("array fixture is valid")
}

#[test]
fn array_inventory_is_order_independent_identity_bearing_and_receipt_bound() {
    let status = observed_enum("status");
    let priority = observed_enum("priority");
    let status_array = array_type("_status", "status");
    let priority_array = array_type("_priority", "priority");

    let first = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        vec![status.clone(), priority.clone()],
        vec![status_array.clone(), priority_array.clone()],
    )
    .expect("array-aware snapshot is valid");
    let reordered = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        vec![priority, status.clone()],
        vec![priority_array, status_array],
    )
    .expect("input order is not identity");
    let renamed_array = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        vec![status.clone()],
        vec![array_type("__status", "status")],
    )
    .expect("an exact renamed true-array coordinate is valid");
    let original_name = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        vec![status.clone()],
        vec![array_type("_status", "status")],
    )
    .expect("the conventional exact catalog coordinate is valid");
    let legacy_unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        vec![status],
    )
    .expect("legacy v3 construction remains valid");

    assert_eq!(first.snapshot_digest(), reordered.snapshot_digest());
    assert_ne!(renamed_array.snapshot_digest(), original_name.snapshot_digest());
    assert_ne!(original_name.snapshot_digest(), legacy_unobserved.snapshot_digest());
    assert_eq!(first.array_types().expect("array inventory was observed").len(), 2);

    let receipt = first
        .source_receipt(
            SchemaObjectLocation::enum_("public", "status")
                .expect("enum receipt coordinate is valid"),
        )
        .expect("enum exists in array-aware snapshot");
    assert_eq!(receipt.source_digest(), first.snapshot_digest());
}

#[test]
fn exact_array_binding_remains_identity_even_when_private_validation_projects_to_element() {
    let status = observed_enum("status");
    let true_array = array_type("_status", "status");
    let array_bound = RelationObservation::new(
        "public",
        "ticket",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "statuses",
            1,
            "public.status[]",
            type_name("public", "_status"),
            false,
            None,
        )
        .expect("array-bound column is valid")],
    )
    .expect("relation is valid");
    let element_bound = RelationObservation::new(
        "public",
        "ticket",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "statuses",
            1,
            "public.status[]",
            type_name("public", "status"),
            false,
            None,
        )
        .expect("element-bound control column is structurally valid")],
    )
    .expect("control relation is valid");

    let array_snapshot = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        vec![array_bound],
        Vec::new(),
        vec![status.clone()],
        vec![true_array.clone()],
    )
    .expect("array-bound snapshot is valid");
    let element_snapshot = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        vec![element_bound],
        Vec::new(),
        vec![status],
        vec![true_array],
    )
    .expect("element-bound control snapshot is valid");

    assert_ne!(array_snapshot.snapshot_digest(), element_snapshot.snapshot_digest());
}

#[test]
fn observed_empty_array_inventory_does_not_collapse_into_unobserved_state() {
    let observed_empty = PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect("an explicitly observed empty array inventory is valid");
    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:48:00Z",
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect("legacy unobserved array state remains valid");

    assert!(observed_empty.array_types().is_some());
    assert!(unobserved.array_types().is_none());
    assert_ne!(observed_empty.snapshot_digest(), unobserved.snapshot_digest());
}
