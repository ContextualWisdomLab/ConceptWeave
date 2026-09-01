use conceptweave_observation::{
    ColumnObservation, ObservationError, PostgresSchemaSnapshot, TableObservation,
};

fn column(name: &str, ordinal_position: u32) -> ColumnObservation {
    ColumnObservation::new(
        name,
        ordinal_position,
        "text",
        true,
        Some("source comment".to_owned()),
    )
    .expect("fixture column is valid")
}

#[test]
fn snapshot_preserves_qualified_identifiers_without_normalization() {
    let snapshot = PostgresSchemaSnapshot::new(
        "warehouse-primary",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "postgres-introspector/1",
        "2026-09-02T00:00:00Z",
        vec![
            TableObservation::new("public", "Order", vec![column("Line Item", 1)])
                .expect("table is valid"),
            TableObservation::new("audit", "Order", vec![column("Line Item", 1)])
                .expect("table is valid"),
        ],
    )
    .expect("snapshot is valid");

    let coordinates: Vec<_> = snapshot
        .tables()
        .iter()
        .map(|table| (table.schema_name(), table.table_name()))
        .collect();
    assert_eq!(coordinates, vec![("audit", "Order"), ("public", "Order")]);
    assert_eq!(snapshot.tables()[0].columns()[0].column_name(), "Line Item");
}

#[test]
fn snapshot_rejects_duplicate_qualified_tables() {
    let duplicate = TableObservation::new("public", "events", vec![column("event_key", 1)])
        .expect("table is valid");
    let error = PostgresSchemaSnapshot::new(
        "warehouse-primary",
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "postgres-introspector/1",
        "2026-09-02T00:00:00Z",
        vec![duplicate.clone(), duplicate],
    )
    .expect_err("duplicate qualified tables must fail closed");

    assert_eq!(
        error,
        ObservationError::DuplicateTableObservation {
            schema_name: "public".to_owned(),
            table_name: "events".to_owned(),
        }
    );
}

#[test]
fn table_rejects_duplicate_column_name_or_ordinal() {
    let duplicate_name = TableObservation::new(
        "public",
        "events",
        vec![column("event_key", 1), column("event_key", 2)],
    )
    .expect_err("duplicate source column names must fail closed");
    assert_eq!(
        duplicate_name,
        ObservationError::DuplicateColumnName {
            schema_name: "public".to_owned(),
            table_name: "events".to_owned(),
            column_name: "event_key".to_owned(),
        }
    );

    let duplicate_ordinal = TableObservation::new(
        "public",
        "events",
        vec![column("event_key", 1), column("event_label", 1)],
    )
    .expect_err("duplicate source ordinals must fail closed");
    assert_eq!(
        duplicate_ordinal,
        ObservationError::DuplicateColumnOrdinal {
            schema_name: "public".to_owned(),
            table_name: "events".to_owned(),
            ordinal_position: 1,
        }
    );
}

#[test]
fn source_identifiers_and_evidence_reject_unicode_whitespace_only_values() {
    let error = ColumnObservation::new("\t\n", 1, "text", false, None)
        .expect_err("blank column names must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_name"
        }
    );

    let error = PostgresSchemaSnapshot::new(
        "warehouse-primary",
        "\u{2003}",
        "postgres-introspector/1",
        "2026-09-02T00:00:00Z",
        Vec::new(),
    )
    .expect_err("blank snapshot evidence must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "snapshot_digest"
        }
    );
}

#[test]
fn columns_are_exposed_in_source_ordinal_order() {
    let table = TableObservation::new(
        "public",
        "events",
        vec![column("event_label", 2), column("event_key", 1)],
    )
    .expect("table is valid");

    let columns: Vec<_> = table
        .columns()
        .iter()
        .map(|column| (column.ordinal_position(), column.column_name()))
        .collect();
    assert_eq!(columns, vec![(1, "event_key"), (2, "event_label")]);
}
