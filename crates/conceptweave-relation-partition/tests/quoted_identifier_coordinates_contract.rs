use conceptweave_observation::{QualifiedTypeName, RelationKind};
use conceptweave_relation_partition::{
    CanonicalExpression, CollationCatalogIdentity, ColumnTypeModifierObservation,
    IndexConstraintParentageCoordinate, IndexExclusionConstraintAccessMethodCapabilityObservation,
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintIndexNamespaceObservation,
    IndexExclusionConstraintNamespaceObservation,
    IndexExclusionConstraintOperatorProcedureExecuteGrant, IndexPartitionCoordinate,
    PartitionParentRelationCoordinate, QualifiedFunctionSignature, QualifiedOperatorSignature,
    QualifiedProcedureSignature, QualifiedUnaryOperatorSignature, RelationPartitionObservation,
};

#[test]
fn quoted_catalog_identifiers_survive_independent_partition_and_exclusion_coordinates() {
    let parent = PartitionParentRelationCoordinate::new(" ", " ").unwrap();
    assert_eq!(parent.relation_name(), " ");
    let relation =
        RelationPartitionObservation::non_partition(" ", " ", RelationKind::Table).unwrap();
    assert_eq!(relation.schema_name(), " ");
    let index = IndexPartitionCoordinate::new(" ", " ", RelationKind::Table, " ").unwrap();
    assert_eq!(index.index_name(), " ");
    let constraint =
        IndexExclusionConstraintCoordinate::new(" ", " ", RelationKind::Table, " ").unwrap();
    assert_eq!(constraint.constraint_name(), " ");
    let parentage =
        IndexConstraintParentageCoordinate::new(" ", " ", RelationKind::Table, " ").unwrap();
    assert_eq!(parentage.constraint_name(), " ");
    let modifier =
        ColumnTypeModifierObservation::new(" ", " ", RelationKind::Table, " ", -1).unwrap();
    assert_eq!(modifier.column_name(), " ");
    assert_eq!(
        CollationCatalogIdentity::new(" ", " ", -1)
            .unwrap()
            .collation_name(),
        " "
    );

    assert_eq!(
        IndexExclusionConstraintNamespaceObservation::new(constraint.clone(), " ")
            .unwrap()
            .constraint_schema_name(),
        " "
    );
    assert_eq!(
        IndexExclusionConstraintIndexNamespaceObservation::new(
            constraint.clone(),
            index.clone(),
            " ",
        )
        .unwrap()
        .index_schema_name(),
        " "
    );
    assert_eq!(
        IndexExclusionConstraintAccessMethodCapabilityObservation::new(
            constraint, index, " ", true,
        )
        .unwrap()
        .access_method_name(),
        " "
    );
    let type_name = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    assert_eq!(
        QualifiedOperatorSignature::new(" ", "=", type_name.clone(), type_name.clone())
            .unwrap()
            .schema_name(),
        " "
    );
    assert_eq!(
        QualifiedProcedureSignature::new(" ", " ", vec![type_name.clone()])
            .unwrap()
            .procedure_name(),
        " "
    );
    IndexExclusionConstraintOperatorProcedureExecuteGrant::role(" ", " ", false).unwrap();
    assert_eq!(
        QualifiedFunctionSignature::new(" ", " ", vec![], type_name.clone())
            .unwrap()
            .function_name(),
        " "
    );
    assert_eq!(
        QualifiedUnaryOperatorSignature::new(" ", "+", type_name.clone())
            .unwrap()
            .schema_name(),
        " "
    );
    CanonicalExpression::column(" ").unwrap();
    assert!(QualifiedUnaryOperatorSignature::new(" ", " ", type_name.clone()).is_err());
    assert!(QualifiedOperatorSignature::new(" ", " ", type_name.clone(), type_name).is_err());
}

#[test]
fn impossible_catalog_identifiers_fail_before_immutable_evidence() {
    assert!(PartitionParentRelationCoordinate::new("", "name").is_err());
    assert!(
        RelationPartitionObservation::non_partition("bad\0schema", "name", RelationKind::Table,)
            .is_err()
    );
    assert!(
        IndexPartitionCoordinate::new("public", "table", RelationKind::Table, "bad\0index")
            .is_err()
    );
    assert!(CollationCatalogIdentity::new("pg_catalog", "bad\0collation", -1).is_err());
    assert!(
        IndexExclusionConstraintOperatorProcedureExecuteGrant::public("bad\0role", false).is_err()
    );
}
