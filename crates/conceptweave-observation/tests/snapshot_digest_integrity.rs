use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservation, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationLocation,
    PostgresSchemaSnapshot, PrimaryKeyObservation, TableConstraintObservation, TableObservation,
    UniqueConstraintObservation,
};

mod support;

#[test]
fn unique_null_comparison_evidence_changes_observation_and_snapshot_identity() {
    let unknown =
        UniqueConstraintObservation::new("event_parent_uq", vec!["parent_key".to_owned()]).unwrap();
    assert_eq!(unknown.nulls_not_distinct(), None);
    let distinct = unknown.clone().with_nulls_not_distinct(false);
    let not_distinct = unknown.clone().with_nulls_not_distinct(true);
    assert_eq!(distinct.nulls_not_distinct(), Some(false));
    assert_eq!(not_distinct.nulls_not_distinct(), Some(true));
    assert_eq!(unknown.nulls_not_distinct(), None);
    assert_ne!(unknown, distinct);
    assert_ne!(unknown, not_distinct);
    assert_ne!(distinct, not_distinct);

    let snapshots = [unknown, distinct, not_distinct].map(|constraint| {
        PostgresSchemaSnapshot::new(
            &support::resolved_source("warehouse_primary"),
            "postgres_introspector_v1",
            "2026-09-05T03:30:00Z",
            vec![
                TableObservation::with_constraints(
                    "public",
                    "event_record",
                    vec![ColumnObservation::new("parent_key", 1, "uuid", true, None).unwrap()],
                    vec![TableConstraintObservation::Unique(constraint)],
                )
                .unwrap(),
            ],
        )
        .unwrap()
    });
    assert_ne!(
        snapshots[1].snapshot_digest(),
        snapshots[2].snapshot_digest(),
        "NULLS DISTINCT and NULLS NOT DISTINCT must not share source-content identity"
    );
    for left in 0..snapshots.len() {
        for right in left + 1..snapshots.len() {
            assert_ne!(
                snapshots[left].snapshot_digest(),
                snapshots[right].snapshot_digest(),
                "unknown, NULLS DISTINCT and NULLS NOT DISTINCT must not share content identity"
            );
        }
        let receipt = snapshots[left]
            .source_receipt(
                ObservationLocation::constraint("public", "event_record", "event_parent_uq")
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), snapshots[left].snapshot_digest());
    }
}

fn table(comment: &str) -> TableObservation {
    TableObservation::new(
        "public",
        "event_record",
        vec![
            ColumnObservation::new("event_key", 1, "uuid", false, Some(comment.to_owned()))
                .expect("fixture column is valid"),
        ],
    )
    .expect("fixture table is valid")
}

#[test]
fn snapshot_digest_changes_when_observed_metadata_changes() {
    let first = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![table("first source comment")],
    )
    .expect("first snapshot is structurally valid");
    let changed = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v1",
        "2026-09-05T03:31:00Z",
        vec![table("changed source comment")],
    )
    .expect("changed snapshot is structurally valid");

    assert_ne!(
        first.snapshot_digest(),
        changed.snapshot_digest(),
        "immutable source identity must be derived from observed metadata"
    );
}

#[test]
fn snapshot_digest_is_stable_across_table_input_order_and_provenance_coordinates() {
    let alpha = TableObservation::new("audit", "event_record", Vec::new()).unwrap();
    let beta = TableObservation::new("public", "event_record", Vec::new()).unwrap();

    let first = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![beta.clone(), alpha.clone()],
    )
    .unwrap();
    let reordered = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_secondary"),
        "postgres_introspector_v2",
        "2026-09-05T04:30:00Z",
        vec![alpha, beta],
    )
    .unwrap();

    assert_eq!(first.snapshot_digest(), reordered.snapshot_digest());
}

#[test]
fn source_receipt_exposes_the_snapshot_verified_digest() {
    let snapshot = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![table("source comment")],
    )
    .unwrap();
    let receipt = snapshot
        .source_receipt(ObservationLocation::table("public", "event_record").unwrap())
        .unwrap();

    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn canonical_digest_frames_every_observed_constraint_variant_and_optional_state() {
    let columns = vec![
        ColumnObservation::new(
            "event_key",
            1,
            "uuid",
            false,
            Some("stable identifier".to_owned()),
        )
        .unwrap(),
        ColumnObservation::new("parent_key", 2, "uuid", true, None).unwrap(),
    ];

    let primary_key =
        PrimaryKeyObservation::new("event_record_pk", vec!["event_key".to_owned()]).unwrap();
    let unique_key =
        UniqueConstraintObservation::new("event_parent_uq", vec!["parent_key".to_owned()]).unwrap();

    let no_action_behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::Restrict,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    );
    let no_action_fk = ForeignKeyObservation::with_reference_behavior(
        "event_parent_no_action_fk",
        vec!["parent_key".to_owned()],
        "identity",
        "parent_record",
        vec!["parent_key".to_owned()],
        no_action_behavior,
    )
    .unwrap();

    let set_null_behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::Cascade,
        ForeignKeyAction::SetNull,
        ForeignKeyMatchType::Full,
        ForeignKeyDeferrability::InitiallyImmediate,
    )
    .with_delete_target_columns(vec!["parent_key".to_owned()])
    .unwrap();
    let set_null_fk = ForeignKeyObservation::with_reference_behavior(
        "event_parent_set_null_fk",
        vec!["parent_key".to_owned()],
        "identity",
        "parent_record",
        vec!["parent_key".to_owned()],
        set_null_behavior,
    )
    .unwrap()
    .with_validation_and_enforcement(false, true);

    let set_default_behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::SetDefault,
        ForeignKeyAction::SetDefault,
        ForeignKeyMatchType::Partial,
        ForeignKeyDeferrability::InitiallyDeferred,
    )
    .with_delete_target_columns(vec!["parent_key".to_owned()])
    .unwrap();
    let set_default_fk = ForeignKeyObservation::with_reference_behavior(
        "event_parent_set_default_fk",
        vec!["parent_key".to_owned()],
        "identity",
        "parent_record",
        vec!["parent_key".to_owned()],
        set_default_behavior,
    )
    .unwrap()
    .with_validation_and_enforcement(true, false);

    let unknown_behavior_fk = ForeignKeyObservation::new(
        "event_parent_unknown_behavior_fk",
        vec!["parent_key".to_owned()],
        "identity",
        "parent_record",
        vec!["parent_key".to_owned()],
    )
    .unwrap();

    let check = CheckConstraintObservation::new(
        "event_key_present",
        "CHECK ((event_key IS NOT NULL))",
        true,
        false,
        true,
    )
    .unwrap();

    let observed = TableObservation::with_constraints(
        "public",
        "event_record",
        columns,
        vec![
            TableConstraintObservation::PrimaryKey(primary_key),
            TableConstraintObservation::Unique(unique_key),
            TableConstraintObservation::ForeignKey(no_action_fk),
            TableConstraintObservation::ForeignKey(set_null_fk),
            TableConstraintObservation::ForeignKey(set_default_fk),
            TableConstraintObservation::ForeignKey(unknown_behavior_fk),
            TableConstraintObservation::Check(check),
        ],
    )
    .unwrap();

    let snapshot = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![observed],
    )
    .unwrap();

    let digest = snapshot.snapshot_digest();
    assert_eq!(digest.len(), "sha256:".len() + 64);
    assert!(digest.starts_with("sha256:"));
    assert!(
        digest["sha256:".len()..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
}
