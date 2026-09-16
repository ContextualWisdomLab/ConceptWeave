use conceptweave_observation::{QualifiedTypeName, RelationKind};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorResultObservation,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn int4() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "int4").unwrap()
}

fn bool_type() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "bool").unwrap()
}

fn operator() -> QualifiedOperatorSignature {
    QualifiedOperatorSignature::new("pg_catalog", "=", int4(), int4()).unwrap()
}

fn procedure() -> QualifiedProcedureSignature {
    QualifiedProcedureSignature::new("pg_catalog", "int4eq", vec![int4(), int4()]).unwrap()
}

#[test]
fn ordinary_exclude_records_independent_operator_and_procedure_result_types() {
    let observation = IndexExclusionConstraintOperatorResultObservation::new(
        coordinate(),
        1,
        operator(),
        procedure(),
        bool_type(),
        bool_type(),
    )
    .unwrap();

    assert_eq!(observation.operator_result_type(), &bool_type());
    assert_eq!(observation.procedure_result_type(), &bool_type());
}
