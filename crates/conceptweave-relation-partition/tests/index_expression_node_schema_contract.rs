use conceptweave_observation::{ObservationError, QualifiedTypeName};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    QualifiedFunctionSignature, QualifiedOperatorSignature,
};

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

#[test]
fn func_expr_rejects_omitted_postgresql_equal_fields() {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    let error = CanonicalExpression::node(
        "FuncExpr",
        vec![
            field(
                "function",
                CanonicalExpressionValue::Function(
                    QualifiedFunctionSignature::new(
                        "pg_catalog",
                        "lower",
                        vec![text.clone()],
                        text,
                    )
                    .unwrap(),
                ),
            ),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_email").unwrap(),
                ]),
            ),
        ],
    )
    .expect_err("FuncExpr evidence must not omit fields PostgreSQL 18 equal() compares");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );
}

#[test]
fn op_expr_rejects_omitted_postgresql_equal_fields() {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let error = CanonicalExpression::node(
        "OpExpr",
        vec![
            field(
                "operator",
                CanonicalExpressionValue::Operator(
                    QualifiedOperatorSignature::new(
                        "pg_catalog",
                        ">",
                        int4.clone(),
                        int4,
                    )
                    .unwrap(),
                ),
            ),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_id").unwrap(),
                    CanonicalExpression::column("account_id").unwrap(),
                ]),
            ),
        ],
    )
    .expect_err("OpExpr evidence must not omit fields PostgreSQL 18 equal() compares");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );
}

#[test]
fn const_and_unknown_nodes_fail_closed_until_their_full_equal_schema_is_modeled() {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let const_error = CanonicalExpression::node(
        "Const",
        vec![
            field("type", CanonicalExpressionValue::Type(int4)),
            field("value", CanonicalExpressionValue::Text("0".to_owned())),
        ],
    )
    .expect_err("Const Datum identity must not be approximated with rendered text");
    assert_eq!(
        const_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );

    let unknown_error = CanonicalExpression::node("FuturePostgresNode", vec![])
        .expect_err("unmodeled PostgreSQL node kinds must fail closed");
    assert_eq!(
        unknown_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );
}
