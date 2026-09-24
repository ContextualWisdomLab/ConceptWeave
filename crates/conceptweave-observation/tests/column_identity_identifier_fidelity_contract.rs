use conceptweave_observation::{ColumnIdentityObservation, RelationKind};

#[test]
fn column_identity_coordinates_preserve_quoted_whitespace_identifiers() {
    let observation =
        ColumnIdentityObservation::not_identity("  ", "\t", RelationKind::Table, " \t ")
            .expect("PostgreSQL quoted identifiers may consist of whitespace");

    assert_eq!(observation.schema_name(), "  ");
    assert_eq!(observation.relation_name(), "\t");
    assert_eq!(observation.column_name(), " \t ");
}

#[test]
fn column_identity_coordinates_reject_empty_and_code_zero_identifiers() {
    assert!(
        ColumnIdentityObservation::not_identity("", "events", RelationKind::Table, "id").is_err()
    );
    assert!(
        ColumnIdentityObservation::not_identity(
            "public",
            "bad\0relation",
            RelationKind::Table,
            "id",
        )
        .is_err()
    );
    assert!(
        ColumnIdentityObservation::not_identity(
            "public",
            "events",
            RelationKind::Table,
            "bad\0column",
        )
        .is_err()
    );
}
