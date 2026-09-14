use conceptweave_observation::{
    ObservationError, QualifiedCollationName, QualifiedTypeName,
};
use conceptweave_relation_partition::{
    validate_postgres18_equal_schema, CanonicalExpression, CanonicalExpressionField,
    CanonicalExpressionValue, QualifiedFunctionSignature, QualifiedOperatorSignature,
};

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

fn incomplete_func_expr() -> CanonicalExpression {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    CanonicalExpression::node(
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
    .unwrap()
}

fn incomplete_op_expr() -> CanonicalExpression {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    CanonicalExpression::node(
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
    .unwrap()
}

#[test]
fn func_expr_rejects_omitted_postgresql_equal_fields() {
    let error = validate_postgres18_equal_schema(&incomplete_func_expr())
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
    let error = validate_postgres18_equal_schema(&incomplete_op_expr())
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
    let constant = CanonicalExpression::node(
        "Const",
        vec![
            field("type", CanonicalExpressionValue::Type(int4)),
            field("value", CanonicalExpressionValue::Text("0".to_owned())),
        ],
    )
    .unwrap();
    let const_error = validate_postgres18_equal_schema(&constant)
        .expect_err("Const Datum identity must not be approximated with rendered text");
    assert_eq!(
        const_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );

    let unknown = CanonicalExpression::node("FuturePostgresNode", vec![]).unwrap();
    let unknown_error = validate_postgres18_equal_schema(&unknown)
        .expect_err("unmodeled PostgreSQL node kinds must fail closed");
    assert_eq!(
        unknown_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );
}

#[test]
fn relation_var_leaves_fail_closed_until_their_full_equal_schema_is_modeled() {
    let column = CanonicalExpression::column("account_email").unwrap();
    let column_error = validate_postgres18_equal_schema(&column)
        .expect_err("column name alone cannot prove PostgreSQL Var equal() semantics");
    assert_eq!(
        column_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );

    let whole_row_error = validate_postgres18_equal_schema(&CanonicalExpression::whole_row())
        .expect_err("whole-row Var evidence must also retain complete PostgreSQL equality state");
    assert_eq!(
        whole_row_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema",
        }
    );
}

#[test]
fn supported_func_and_op_nodes_accept_complete_equal_schemas() {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    let default_collation =
        QualifiedCollationName::new("pg_catalog", "default").unwrap();
    let func = CanonicalExpression::node(
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
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("variadic", CanonicalExpressionValue::Boolean(false)),
            field(
                "result_collation",
                CanonicalExpressionValue::Collation(default_collation.clone()),
            ),
            field(
                "input_collation",
                CanonicalExpressionValue::Collation(default_collation),
            ),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_email").unwrap(),
                ]),
            ),
        ],
    )
    .unwrap();
    validate_postgres18_equal_schema(&func)
        .expect("all PostgreSQL 18 FuncExpr equality fields are represented");

    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let bool_type = QualifiedTypeName::new("pg_catalog", "bool").unwrap();
    let op = CanonicalExpression::node(
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
            field("result_type", CanonicalExpressionValue::Type(bool_type)),
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("result_collation", CanonicalExpressionValue::Null),
            field("input_collation", CanonicalExpressionValue::Null),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_id").unwrap(),
                    CanonicalExpression::column("account_id").unwrap(),
                ]),
            ),
        ],
    )
    .unwrap();
    validate_postgres18_equal_schema(&op)
        .expect("all PostgreSQL 18 OpExpr equality fields are represented");
}
