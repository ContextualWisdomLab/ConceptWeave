use conceptweave_observation::{ObservationError, QualifiedTypeName, RelationKind};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorProcedureKindObservation,
    IndexExclusionConstraintOperatorProcedureKindSnapshot, QualifiedOperatorSignature,
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
fn ordinary_exclude_operator_procedure_kind_accepts_only_normal_function() {
    let observation = IndexExclusionConstraintOperatorProcedureKindObservation::new(
        coordinate(),
        1,
        operator(),
        procedure(),
        'f',
    )
    .unwrap();
    assert_eq!(observation.procedure_kind(), 'f');
    assert_eq!(observation.operator(), &operator());
    assert_eq!(observation.procedure(), &procedure());
    assert!(observation
        .canonical_location()
        .ends_with("/1/procedure-kind"));
}

#[test]
fn ordinary_exclude_operator_procedure_kind_rejects_non_function_routines() {
    for invalid_kind in ['p', 'a', 'w', 'x'] {
        let error = IndexExclusionConstraintOperatorProcedureKindObservation::new(
            coordinate(),
            1,
            operator(),
            procedure(),
            invalid_kind,
        )
        .expect_err("CREATE OPERATOR implementation must resolve to a normal function");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "index_exclusion_constraint_operator_procedure_kind"
            }
        );
    }
}

#[test]
fn ordinary_exclude_operator_procedure_kind_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureKindObservation::new(
        coordinate(),
        0,
        operator(),
        procedure(),
        'f',
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_kind_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureKindSnapshot>() > 0);
}
