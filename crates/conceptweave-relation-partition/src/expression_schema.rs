use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    IndexExpressionSemanticsSnapshot,
};

const EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.node_schema.v1";
const EXPRESSION_NODE_SCHEMA_REVISION_V1: &str = "postgresql-18-equal-supported-v1";
const EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V2: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.node_schema.v2";
const EXPRESSION_NODE_SCHEMA_REVISION_V2: &str = "postgresql-18-equal-supported-v2-var-safe";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Validates the historical PostgreSQL 18 equality-schema v1 contract.
///
/// This function deliberately preserves the issued v1 admission semantics. The v1 family validates
/// the complete modeled field sets for `FuncExpr` and `OpExpr`, rejects `Const` and unknown node
/// kinds, but historically admits relation-local [`CanonicalExpression::Column`] and
/// [`CanonicalExpression::WholeRow`] leaves without carrying the complete PostgreSQL `Var` equality
/// state. That limitation is retained here only so an existing v1 digest can still be rebound under
/// exactly the contract that produced it.
///
/// New authoritative equality claims must use [`validate_postgres18_equal_schema_v2`] and
/// [`IndexExpressionNodeSchemaSnapshotV2`].
pub fn validate_postgres18_equal_schema(
    expression: &CanonicalExpression,
) -> Result<(), ObservationError> {
    match expression {
        CanonicalExpression::Column(_) | CanonicalExpression::WholeRow => Ok(()),
        CanonicalExpression::Node { node_kind, fields } => {
            match node_kind.as_str() {
                "FuncExpr" => validate_func_expr(fields)?,
                "OpExpr" => validate_op_expr(fields)?,
                _ => return Err(invalid_v1()),
            }
            validate_nested_fields_v1(fields)
        }
    }
}

/// Validates the PostgreSQL 18 equality-schema v2 contract.
///
/// V2 first requires the complete historical v1 node-field schema and then fails closed on the v1
/// relation-`Var` leaves. PostgreSQL `CompareIndexInfo()` maps child `varattno` values and compares
/// the resulting complete node tree with `equal()`. A column name alone therefore cannot prove the
/// equality state of `vartype`, `vartypmod`, `varcollid`, `varnullingrels`, `varlevelsup`, or
/// `varreturningtype`; a whole-row marker is likewise incomplete. Those leaves remain inadmissible
/// until a later domain-separated Var-semantics successor represents or proves every material field.
pub fn validate_postgres18_equal_schema_v2(
    expression: &CanonicalExpression,
) -> Result<(), ObservationError> {
    validate_postgres18_equal_schema(expression)?;
    reject_incomplete_var_leaves(expression)
}

/// Historical v1 validation successor for one exact expression-semantics predecessor.
///
/// This type is retained for digest-family stability. It is not sufficient for new authoritative
/// PostgreSQL expression-equality claims when its predecessor contains relation `Var` leaves; use
/// [`IndexExpressionNodeSchemaSnapshotV2`] for the current fail-closed contract.
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
    /// Validates and frames one exact expression-semantics predecessor under historical v1 rules.
    pub fn new(predecessor: &IndexExpressionSemanticsSnapshot) -> Result<Self, ObservationError> {
        for observation in predecessor.expression_observations() {
            validate_postgres18_equal_schema(observation.expression())?;
        }
        for observation in predecessor.predicate_observations() {
            validate_postgres18_equal_schema(observation.predicate())?;
        }

        let predecessor_digest = predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest_v1(&predecessor_digest);
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

    /// Returns the exact expression-semantics predecessor digest that v1 validated.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated historical v1 node-schema digest.
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

/// Domain-separated v2 successor proving that an exact v1 node-schema predecessor contains no
/// incompletely represented relation `Var` leaves.
///
/// The constructor rebound-validates the supplied v1 snapshot from the exact expression-semantics
/// predecessor before applying v2 validation. Its digest is derived from the exact v1 digest under
/// a new domain and revision, so tightening the `Var` boundary does not redefine issued v1 identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionNodeSchemaSnapshotV2 {
    source_connection_key: String,
    connection_policy_binding: String,
    predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
}

impl IndexExpressionNodeSchemaSnapshotV2 {
    /// Rebound-validates v1 and frames the exact predecessor under the v2 fail-closed `Var` rule.
    pub fn new(
        expression_predecessor: &IndexExpressionSemanticsSnapshot,
        v1_predecessor: &IndexExpressionNodeSchemaSnapshot,
    ) -> Result<Self, ObservationError> {
        validate_v1_predecessor(expression_predecessor, v1_predecessor)?;

        for observation in expression_predecessor.expression_observations() {
            validate_postgres18_equal_schema_v2(observation.expression())?;
        }
        for observation in expression_predecessor.predicate_observations() {
            validate_postgres18_equal_schema_v2(observation.predicate())?;
        }

        let predecessor_digest = v1_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest_v2(&predecessor_digest);
        Ok(Self {
            source_connection_key: v1_predecessor.source_connection_key().to_owned(),
            connection_policy_binding: v1_predecessor.connection_policy_binding().to_owned(),
            predecessor_digest,
            snapshot_digest,
            extractor_revision: v1_predecessor.extractor_revision().to_owned(),
            observed_at_utc: v1_predecessor.observed_at_utc().to_owned(),
        })
    }

    /// Returns the stable source registry key inherited from the exact v1 predecessor.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding inherited from the exact v1 predecessor.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact v1 node-schema digest rebound-validated by this successor.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated v2 node-schema digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the bounded predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the bounded predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
}

fn validate_v1_predecessor(
    expression_predecessor: &IndexExpressionSemanticsSnapshot,
    v1_predecessor: &IndexExpressionNodeSchemaSnapshot,
) -> Result<(), ObservationError> {
    if expression_predecessor.source_connection_key() != v1_predecessor.source_connection_key()
        || expression_predecessor.connection_policy_binding()
            != v1_predecessor.connection_policy_binding()
        || expression_predecessor.extractor_revision() != v1_predecessor.extractor_revision()
        || expression_predecessor.observed_at_utc() != v1_predecessor.observed_at_utc()
        || expression_predecessor.snapshot_digest() != v1_predecessor.predecessor_digest()
    {
        return Err(invalid_v2());
    }

    let rebound = IndexExpressionNodeSchemaSnapshot::new(expression_predecessor)?;
    if rebound.snapshot_digest() != v1_predecessor.snapshot_digest() {
        return Err(invalid_v2());
    }
    Ok(())
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
        return Err(invalid_v1());
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
        return Err(invalid_v1());
    };
    if !predicate(value) {
        return Err(invalid_v1());
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

fn validate_nested_fields_v1(fields: &[CanonicalExpressionField]) -> Result<(), ObservationError> {
    for field in fields {
        validate_nested_value_v1(field.value())?;
    }
    Ok(())
}

fn validate_nested_value_v1(value: &CanonicalExpressionValue) -> Result<(), ObservationError> {
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
                validate_nested_value_v1(value)?;
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

fn reject_incomplete_var_leaves(expression: &CanonicalExpression) -> Result<(), ObservationError> {
    match expression {
        CanonicalExpression::Column(_) | CanonicalExpression::WholeRow => Err(invalid_v2()),
        CanonicalExpression::Node { fields, .. } => {
            for field in fields {
                reject_incomplete_var_value(field.value())?;
            }
            Ok(())
        }
    }
}

fn reject_incomplete_var_value(value: &CanonicalExpressionValue) -> Result<(), ObservationError> {
    match value {
        CanonicalExpressionValue::Expression(expression) => {
            reject_incomplete_var_leaves(expression)
        }
        CanonicalExpressionValue::ExpressionList(expressions) => {
            for expression in expressions {
                reject_incomplete_var_leaves(expression)?;
            }
            Ok(())
        }
        CanonicalExpressionValue::ValueList(values) => {
            for value in values {
                reject_incomplete_var_value(value)?;
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

fn compute_digest_v1(predecessor_digest: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, EXPRESSION_NODE_SCHEMA_REVISION_V1);
    encode_str(&mut hasher, predecessor_digest);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn compute_digest_v2(predecessor_digest: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(EXPRESSION_NODE_SCHEMA_DIGEST_DOMAIN_V2);
    encode_str(&mut hasher, EXPRESSION_NODE_SCHEMA_REVISION_V2);
    encode_str(&mut hasher, predecessor_digest);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let len = u64::try_from(value.len()).expect("Rust target usize must fit canonical u64 length");
    hasher.update(len.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn invalid_v1() -> ObservationError {
    ObservationError::InvalidObservationField {
        field: "canonical_expression_node_schema",
    }
}

fn invalid_v2() -> ObservationError {
    ObservationError::InvalidObservationField {
        field: "canonical_expression_node_schema_v2",
    }
}
