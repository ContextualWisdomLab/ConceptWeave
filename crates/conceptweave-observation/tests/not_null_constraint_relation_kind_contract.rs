use conceptweave_observation::{NotNullConstraintObservation, ObservationError, RelationKind};

fn observed_not_null(
    relation_kind: RelationKind,
) -> Result<NotNullConstraintObservation, ObservationError> {
    NotNullConstraintObservation::new(
        "public",
        "metric_source",
        relation_kind,
        "metric_source_raw_value_not_null",
        "raw_value",
        true,
        true,
        true,
        0,
        false,
    )
}

#[test]
fn table_like_relations_admit_not_null_constraint_rows() {
    for relation_kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::ForeignTable,
    ] {
        observed_not_null(relation_kind)
            .expect("PostgreSQL table-like relations can own NOT NULL constraints");
    }
}

#[test]
fn non_table_relations_reject_not_null_constraint_rows() {
    for relation_kind in [
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let error = observed_not_null(relation_kind)
            .expect_err("non-table relations must not acquire governed NOT NULL identity");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "not_null_constraint_relation_kind",
            }
        );
    }
}
