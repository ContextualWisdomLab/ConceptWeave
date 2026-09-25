use conceptweave_observation::{ObservationError, QualifiedTypeName, RelationKind};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureParallelSafetyObservation,
    IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

fn int4() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "int4").unwrap()
}

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn operator() -> QualifiedOperatorSignature {
    QualifiedOperatorSignature::new("pg_catalog", "=", int4(), int4()).unwrap()
}

fn procedure() -> QualifiedProcedureSignature {
    QualifiedProcedureSignature::new("pg_catalog", "int4eq", vec![int4(), int4()]).unwrap()
}

#[test]
fn ordinary_exclude_parallel_safety_preserves_every_postgres_catalog_state() {
    for expected in ['s', 'r', 'u'] {
        let observation = IndexExclusionConstraintOperatorProcedureParallelSafetyObservation::new(
            coordinate(),
            1,
            operator(),
            procedure(),
            expected,
        )
        .unwrap();
        assert_eq!(observation.parallel_safety(), expected);
        assert_eq!(observation.operator(), &operator());
        assert_eq!(observation.procedure(), &procedure());
        assert!(
            observation
                .canonical_location()
                .ends_with("/1/procedure-parallel-safety")
        );
    }
}

#[test]
fn ordinary_exclude_parallel_safety_rejects_unknown_catalog_state() {
    let error = IndexExclusionConstraintOperatorProcedureParallelSafetyObservation::new(
        coordinate(),
        1,
        operator(),
        procedure(),
        'x',
    )
    .expect_err("only PostgreSQL s/r/u proparallel states are admissible");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_operator_procedure_parallel_safety"
        }
    );
}

#[test]
fn ordinary_exclude_parallel_safety_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureParallelSafetyObservation::new(
        coordinate(),
        0,
        operator(),
        procedure(),
        's',
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_parallel_safety_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot>() > 0
    );
}
