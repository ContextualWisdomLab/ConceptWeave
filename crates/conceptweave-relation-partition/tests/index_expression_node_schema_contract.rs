use conceptweave_observation::{ObservationError, QualifiedTypeName};
use conceptweave_relation_partition::{
    validate_postgres18_equal_schema, validate_postgres18_equal_schema_v2, CanonicalExpression,
    CanonicalExpressionField, CanonicalExpressionValue, QualifiedFunctionSignature,
    QualifiedOperatorSignature,
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

fn complete_zero_arg_func_expr() -> CanonicalExpression {
    let float8 = QualifiedTypeName::new("pg_catalog", "float8").unwrap();
    CanonicalExpression::node(
        "FuncExpr",
        vec![
            field(
                "function",
                CanonicalExpressionValue::Function(
                    QualifiedFunctionSignature::new(
                        "pg_catalog",
                        "pi",
                        vec![],
                        float8,
                    )
                    .unwrap(),
                ),
            ),
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("variadic", CanonicalExpressionValue::Boolean(false)),
            field("result_collation", CanonicalExpressionValue::Null),
            field("input_collation", CanonicalExpressionValue::Null),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![]),
            ),
        ],
    )
    .unwrap()
}

fn complete_zero_arg_op_expr() -> CanonicalExpression {
    let float8 = QualifiedTypeName::new("pg_catalog", "float8").unwrap();
    let bool_type = QualifiedTypeName::new("pg_catalog", "bool").unwrap();
    CanonicalExpression::node(
        "OpExpr",
        vec![
            field(
                "operator",
                CanonicalExpressionValue::Operator(
                    QualifiedOperatorSignature::new(
                        "pg_catalog",
                        ">",
                        float8.clone(),
                        float8,
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
                    complete_zero_arg_func_expr(),
                    complete_zero_arg_func_expr(),
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
fn v1_relation_var_admission_is_preserved_for_digest_family_stability() {
    validate_postgres18_equal_schema(&CanonicalExpression::column("account_email").unwrap())
        .expect("node_schema.v1 historically admitted a relation-local column leaf");
    validate_postgres18_equal_schema(&CanonicalExpression::whole_row())
        .expect("node_schema.v1 historically admitted a whole-row leaf");
}

#[test]
fn v2_relation_var_leaves_fail_closed_until_complete_var_semantics_are_versioned() {
    let column_error = validate_postgres18_equal_schema_v2(
        &CanonicalExpression::column("account_email").unwrap(),
    )
    .expect_err("node_schema.v2 must not certify a column-name-only Var leaf");
    assert_eq!(
        column_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema_v2",
        }
    );

    let whole_row_error = validate_postgres18_equal_schema_v2(&CanonicalExpression::whole_row())
        .expect_err("node_schema.v2 must not certify an incomplete whole-row Var leaf");
    assert_eq!(
        whole_row_error,
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema_v2",
        }
    );
}

#[test]
fn supported_func_and_op_nodes_accept_complete_equal_schemas_without_var_leaves() {
    let func = complete_zero_arg_func_expr();
    validate_postgres18_equal_schema(&func)
        .expect("historical v1 accepts the complete modeled FuncExpr field schema");
    validate_postgres18_equal_schema_v2(&func)
        .expect("v2 accepts complete supported FuncExpr trees without incomplete Var leaves");

    let op = complete_zero_arg_op_expr();
    validate_postgres18_equal_schema(&op)
        .expect("historical v1 accepts the complete modeled OpExpr field schema");
    validate_postgres18_equal_schema_v2(&op)
        .expect("v2 accepts complete supported OpExpr trees without incomplete Var leaves");
}
