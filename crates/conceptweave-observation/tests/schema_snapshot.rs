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
fn snapshot_preserves_evidence_and_qualified_identifiers_without_normalization() {
    let snapshot = PostgresSchemaSnapshot::new(
        "warehouse_primary",
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

    assert_eq!(snapshot.source_connection_key(), "warehouse_primary");
    assert_eq!(
        snapshot.snapshot_digest(),
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(snapshot.extractor_revision(), "postgres-introspector/1");
    assert_eq!(snapshot.observed_at_utc(), "2026-09-02T00:00:00Z");

    let coordinates: Vec<_> = snapshot
        .tables()
        .iter()
        .map(|table| (table.schema_name(), table.table_name()))
        .collect();
    assert_eq!(coordinates, vec![("audit", "Order"), ("public", "Order")]);

    let observed_column = &snapshot.tables()[0].columns()[0];
    assert_eq!(observed_column.column_name(), "Line Item");
    assert_eq!(observed_column.ordinal_position(), 1);
    assert_eq!(observed_column.data_type(), "text");
    assert!(observed_column.nullable());
    assert_eq!(observed_column.source_comment(), Some("source comment"));
}

#[test]
fn snapshot_rejects_duplicate_qualified_tables() {
    let duplicate = TableObservation::new("public", "events", vec![column("event_key", 1)])
        .expect("table is valid");
    let error = PostgresSchemaSnapshot::new(
        "warehouse_primary",
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
    assert_eq!(
        error.to_string(),
        "duplicate table observation: public.events"
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
    assert_eq!(
        duplicate_name.to_string(),
        "duplicate column observation: public.events.event_key"
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
    assert_eq!(
        duplicate_ordinal.to_string(),
        "duplicate column ordinal in public.events: 1"
    );
}

#[test]
fn source_identifiers_and_evidence_reject_unicode_whitespace_only_values() {
    let column_error = ColumnObservation::new("\t\n", 1, "text", false, None)
        .expect_err("blank column names must fail closed");
    assert_eq!(
        column_error,
        ObservationError::InvalidObservationField {
            field: "column_name"
        }
    );
    assert_eq!(
        column_error.to_string(),
        "invalid observation field: column_name"
    );

    let data_type_error = ColumnObservation::new("event_key", 1, "\u{2003}", false, None)
        .expect_err("blank data types must fail closed");
    assert_eq!(
        data_type_error,
        ObservationError::InvalidObservationField { field: "data_type" }
    );

    let schema_error = TableObservation::new(" ", "events", Vec::new())
        .expect_err("blank schema names must fail closed");
    assert_eq!(
        schema_error,
        ObservationError::InvalidObservationField {
            field: "schema_name"
        }
    );

    let table_error = TableObservation::new("public", "\n", Vec::new())
        .expect_err("blank table names must fail closed");
    assert_eq!(
        table_error,
        ObservationError::InvalidObservationField {
            field: "table_name"
        }
    );

    for (source_connection_key, snapshot_digest, extractor_revision, observed_at_utc, field) in [
        (
            "\t",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "extractor",
            "time",
            "source_connection_key",
        ),
        ("source", "\u{2003}", "extractor", "time", "snapshot_digest"),
        (
            "source",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "\n",
            "time",
            "extractor_revision",
        ),
        (
            "source",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "extractor",
            " ",
            "observed_at_utc",
        ),
    ] {
        let error = PostgresSchemaSnapshot::new(
            source_connection_key,
            snapshot_digest,
            extractor_revision,
            observed_at_utc,
            Vec::new(),
        )
        .expect_err("blank snapshot evidence must fail closed");
        assert_eq!(error, ObservationError::InvalidObservationField { field });
    }
}

#[test]
fn snapshot_digest_requires_canonical_sha256_identity() {
    for digest in [
        "digest",
        "sha256:abc",
        "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "sha512:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ] {
        let error = PostgresSchemaSnapshot::new(
            "warehouse_primary",
            digest,
            "postgres-introspector/1",
            "2026-09-02T00:00:00Z",
            Vec::new(),
        )
        .expect_err("snapshot digests must be canonical lowercase SHA-256 identities");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "snapshot_digest"
            }
        );
    }
}

#[test]
fn column_rejects_zero_ordinal_and_preserves_missing_comment() {
    let error = ColumnObservation::new("event_key", 0, "uuid", false, None)
        .expect_err("zero ordinal positions must fail closed");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
    assert_eq!(
        error.to_string(),
        "column ordinal position must be positive"
    );

    let observed =
        ColumnObservation::new("event_key", 1, "uuid", false, None).expect("column is valid");
    assert!(!observed.nullable());
    assert_eq!(observed.source_comment(), None);
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
