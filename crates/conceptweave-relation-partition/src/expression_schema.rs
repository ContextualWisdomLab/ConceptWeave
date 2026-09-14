use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    IndexExpressionSemanticsSnapshot,
};

const EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.node_schema.v1";
const EXPRESSION_NODE_SCHEMA_REVISION: &str = "postgresql-18-equal-supported-v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Validates that a canonical expression contains the complete PostgreSQL 18 `equal()` schema for
/// every node kind currently admitted by ConceptWeave.
///
/// The current supported set is intentionally narrow: `FuncExpr` and `OpExpr` whose complete leaf
/// semantics are themselves supported. Relation-local `Var` leaves are not yet admissible here:
/// [`CanonicalExpression::Column`] preserves only the attribute-map-normalized column name, while
/// PostgreSQL `equal()` also observes material `Var` state such as type, type modifier, collation,
/// nulling relations, nesting level, and RETURNING behavior. [`CanonicalExpression::WholeRow`] is
/// likewise not a complete `Var` representation. Both therefore fail closed until a later
/// domain-separated Var-semantics successor binds every equality-participating field or proves the
/// corresponding PostgreSQL index-expression invariant.
///
/// `Const` remains unsupported because PostgreSQL `_equalConst` compares the exact Datum using type
/// length/by-value semantics; rendered SQL or type output is not an equivalent immutable
/// representation. Unknown node kinds fail closed until their full equality schema and stable OID
/// resolution are modeled.
///
/// `CoercionForm` and parse locations are deliberately absent because PostgreSQL 18 `equal()`
/// explicitly ignores them. `FuncExpr` function identity includes the resolved result type in
/// [`super::QualifiedFunctionSignature`]. `OpExpr` operator identity is resolved by its stable
/// signature while the result type remains an explicit equality field.
pub fn validate_postgres18_equal_schema(
    expression: &CanonicalExpression,
) -> Result<(), ObservationError> {
    match expression {
        CanonicalExpression::Column(_) | CanonicalExpression::WholeRow => Err(invalid()),
        CanonicalExpression::Node { node_kind, fields } => {
            match node_kind.as_str() {
                "FuncExpr" => validate_func_expr(fields)?,
                "OpExpr" => validate_op_expr(fields)?,
                _ => return Err(invalid()),
            }
            validate_nested_fields(fields)
        }
    }
}

/// Immutable validation successor proving that one expression-semantics predecessor contains only
/// supported, complete PostgreSQL 18 equality schemas.
///
/// This successor adds no source fact and does not rewrite its predecessor. Its digest frames the
/// exact expression-semantics digest under a new domain only after every expression and predicate
/// passes [`validate_postgres18_equal_schema`]. A concrete PostgreSQL adapter still has to extract
/// the equality-participating fields from server node/catalog structures and pass differential
/// attachment tests before production semantic parity can be claimed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionNodeSchemaSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
}

impl IndexExpressionNodeSchemaSnapshot {
    /// Validates and frames one exact expression-semantics predecessor.
    pub fn new(predecessor: &IndexExpressionSemanticsSnapshot) -> Result<Self, ObservationError> {
        for observation in predecessor.expression_observations() {
            validate_postgres18_equal_schema(observation.expression())?;
        }
        for observation in predecessor.predicate_observations() {
            validate_postgres18_equal_schema(observation.predicate())?;
        }

        let predecessor_digest = predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest(&predecessor_digest);
        Ok(Self {
            source_connection_key: predecessor.source_connection_key().to_owned(),
            connection_policy_binding: predecessor.connection_policy_binding().to_owned(),
            predecessor_digest,
            snapshot_digest,
            extractor_revision: predecessor.extractor_revision().to_owned(),
            observed_at_utc: predecessor.observed_at_utc().to_owned(),
        })
    }

    /// Returns the stable source registry key inherited from the predecessor.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding inherited from the predecessor.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact expression-semantics predecessor digest that was validated.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated node-schema validation digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
}

fn validate_func_expr(fields: &[CanonicalExpressionField]) -> Result<(), ObservationError> {
    require_exact_fields(
        fields,
        &[
            "arguments",
            "function",
            "input_collation",
            "result_collation",
            "returns_set",
            "variadic",
        ],
    )?;
    require_value(fields, "function", |value| {
        matches!(value, CanonicalExpressionValue::Function(_))
    })?;
    require_value(fields, "returns_set", |value| {
        matches!(value, CanonicalExpressionValue::Boolean(_))
    })?;
    require_value(fields, "variadic", |value| {
        matches!(value, CanonicalExpressionValue::Boolean(_))
    })?;
    require_optional_collation(fields, "result_collation")?;
    require_optional_collation(fields, "input_collation")?;
    require_value(fields, "arguments", |value| {
        matches!(value, CanonicalExpressionValue::ExpressionList(_))
    })
}

fn validate_op_expr(fields: &[CanonicalExpressionField]) -> Result<(), ObservationError> {
    require_exact_fields(
        fields,
        &[
            "arguments",
            "input_collation",
            "operator",
            "result_collation",
            "result_type",
            "returns_set",
        ],
    )?;
    require_value(fields, "operator", |value| {
        matches!(value, CanonicalExpressionValue::Operator(_))
    })?;
    require_value(fields, "result_type", |value| {
        matches!(value, CanonicalExpressionValue::Type(_))
    })?;
    require_value(fields, "returns_set", |value| {
        matches!(value, CanonicalExpressionValue::Boolean(_))
    })?;
    require_optional_collation(fields, "result_collation")?;
    require_optional_collation(fields, "input_collation")?;
    require_value(fields, "arguments", |value| {
        matches!(value, CanonicalExpressionValue::ExpressionList(_))
    })
}

fn require_exact_fields(
    fields: &[CanonicalExpressionField],
    expected: &[&str],
) -> Result<(), ObservationError> {
    let actual = fields
        .iter()
        .map(CanonicalExpressionField::name)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected || actual.len() != fields.len() {
        return Err(invalid());
    }
    Ok(())
}

fn require_value(
    fields: &[CanonicalExpressionField],
    name: &str,
    predicate: impl FnOnce(&CanonicalExpressionValue) -> bool,
) -> Result<(), ObservationError> {
    let Some(value) = fields
        .iter()
        .find(|field| field.name() == name)
        .map(CanonicalExpressionField::value)
    else {
        return Err(invalid());
    };
    if !predicate(value) {
        return Err(invalid());
    }
    Ok(())
}

fn require_optional_collation(
    fields: &[CanonicalExpressionField],
    name: &str,
) -> Result<(), ObservationError> {
    require_value(fields, name, |value| {
        matches!(
            value,
            CanonicalExpressionValue::Null | CanonicalExpressionValue::Collation(_)
        )
    })
}

fn validate_nested_fields(fields: &[CanonicalExpressionField]) -> Result<(), ObservationError> {
    for field in fields {
        validate_nested_value(field.value())?;
    }
    Ok(())
}

fn validate_nested_value(value: &CanonicalExpressionValue) -> Result<(), ObservationError> {
    match value {
        CanonicalExpressionValue::Expression(expression) => {
            validate_postgres18_equal_schema(expression)
        }
        CanonicalExpressionValue::ExpressionList(expressions) => {
            for expression in expressions {
                validate_postgres18_equal_schema(expression)?;
            }
            Ok(())
        }
        CanonicalExpressionValue::ValueList(values) => {
            for value in values {
                validate_nested_value(value)?;
            }
            Ok(())
        }
        CanonicalExpressionValue::Null
        | CanonicalExpressionValue::Boolean(_)
        | CanonicalExpressionValue::Integer(_)
        | CanonicalExpressionValue::Text(_)
        | CanonicalExpressionValue::Type(_)
        | CanonicalExpressionValue::Collation(_)
        | CanonicalExpressionValue::Operator(_)
        | CanonicalExpressionValue::UnaryOperator(_)
        | CanonicalExpressionValue::Function(_) => Ok(()),
    }
}

fn compute_digest(predecessor_digest: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, EXPRESSION_NODE_SCHEMA_REVISION);
    encode_str(&mut hasher, predecessor_digest);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let len = u64::try_from(value.len()).expect("Rust target usize must fit canonical u64 length");
    hasher.update(len.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn invalid() -> ObservationError {
    ObservationError::InvalidObservationField {
        field: "canonical_expression_node_schema",
    }
}
