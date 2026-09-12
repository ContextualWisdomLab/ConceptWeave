use conceptweave_observation::{
    ConstraintDeferrability, ConstraintPeriodObservation, ConstraintTimingObservation,
    ObservationError, RelationKind,
};

const IMPOSSIBLE_TABLE_CONSTRAINT_RELATION_KINDS: [RelationKind; 5] = [
    RelationKind::View,
    RelationKind::MaterializedView,
    RelationKind::ForeignTable,
    RelationKind::Sequence,
    RelationKind::CompositeType,
];

#[test]
fn constraint_period_coordinates_reject_non_table_relation_kinds() {
    for relation_kind in IMPOSSIBLE_TABLE_CONSTRAINT_RELATION_KINDS {
        let error = ConstraintPeriodObservation::new(
            "public",
            "event_record",
            relation_kind,
            "event_period_uq",
            true,
        )
        .expect_err("conperiod evidence cannot belong to a non-table relation kind");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "constraint_period_relation_kind",
            }
        );
    }
}

#[test]
fn constraint_timing_coordinates_reject_non_table_relation_kinds() {
    for relation_kind in IMPOSSIBLE_TABLE_CONSTRAINT_RELATION_KINDS {
        let error = ConstraintTimingObservation::new(
            "public",
            "event_record",
            relation_kind,
            "event_parent_uq",
            ConstraintDeferrability::NotDeferrable,
        )
        .expect_err("PK/UNIQUE timing evidence cannot belong to a non-table relation kind");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "constraint_timing_relation_kind",
            }
        );
    }
}

#[test]
fn constraint_catalog_coordinates_accept_table_and_partitioned_table_kinds() {
    for relation_kind in [RelationKind::Table, RelationKind::PartitionedTable] {
        ConstraintPeriodObservation::new(
            "public",
            "event_record",
            relation_kind,
            "event_period_uq",
            true,
        )
        .expect("table-backed conperiod coordinate is structurally valid");

        ConstraintTimingObservation::new(
            "public",
            "event_record",
            relation_kind,
            "event_parent_uq",
            ConstraintDeferrability::NotDeferrable,
        )
        .expect("table-backed PK/UNIQUE timing coordinate is structurally valid");
    }
}
