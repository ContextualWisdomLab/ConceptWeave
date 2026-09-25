use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedTypeName,
};
use sha2::{Digest, Sha256};

use crate::RelationPartitionSnapshot;

use super::{
    IndexExclusionSemanticsSnapshot, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionSnapshot, QualifiedOperatorSignature,
};

const INDEX_EXPRESSION_SEMANTICS_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.v1";
const EXPRESSION_SEMANTICS_SCHEMA_REVISION: &str = "postgresql-18-equal-v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Stable PostgreSQL function identity for canonical index-expression evidence.
///
/// PostgreSQL function OIDs are database-local join coordinates. Governed identity therefore keeps
/// the exact schema, function name, input argument types, and return type. Input argument types are
/// sufficient to disambiguate PostgreSQL overloads; the return type is retained because expression
/// node equality also carries result-type identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedFunctionSignature {
    schema_name: String,
    function_name: String,
    argument_types: Vec<QualifiedTypeName>,
    return_type: QualifiedTypeName,
}

impl QualifiedFunctionSignature {
    /// Creates one resolved function signature without retaining a catalog OID.
    pub fn new(
        schema_name: impl Into<String>,
        function_name: impl Into<String>,
        argument_types: Vec<QualifiedTypeName>,
        return_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let function_name = function_name.into();
        validate_nonblank(&schema_name, "expression_function_schema_name")?;
        validate_nonblank(&function_name, "expression_function_name")?;
        Ok(Self {
            schema_name,
            function_name,
            argument_types,
            return_type,
        })
    }

    /// Returns the exact function schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact function name.
    #[must_use]
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Returns the ordered input argument-type signature.
    #[must_use]
    pub fn argument_types(&self) -> &[QualifiedTypeName] {
        &self.argument_types
    }

    /// Returns the resolved function result type.
    #[must_use]
    pub const fn return_type(&self) -> &QualifiedTypeName {
        &self.return_type
    }
}

/// Stable unary PostgreSQL operator identity for canonical index-expression evidence.
///
/// PostgreSQL operator OIDs are database-local. A unary operator is resolved by schema, name, and
/// its single operand type before immutable framing; the absent operand side is therefore never
/// encoded as an OID sentinel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedUnaryOperatorSignature {
    schema_name: String,
    operator_name: String,
    operand_type: QualifiedTypeName,
}

impl QualifiedUnaryOperatorSignature {
    /// Creates one exact unary operator signature.
    pub fn new(
        schema_name: impl Into<String>,
        operator_name: impl Into<String>,
        operand_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let operator_name = operator_name.into();
        validate_nonblank(&schema_name, "expression_unary_operator_schema_name")?;
        validate_nonblank(&operator_name, "expression_unary_operator_name")?;
        Ok(Self {
            schema_name,
            operator_name,
            operand_type,
        })
    }

    /// Returns the exact operator schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact operator name.
    #[must_use]
    pub fn operator_name(&self) -> &str {
        &self.operator_name
    }

    /// Returns the exact unary operand type.
    #[must_use]
    pub const fn operand_type(&self) -> &QualifiedTypeName {
        &self.operand_type
    }
}

/// One named semantic field inside a canonical PostgreSQL expression node.
///
/// Field names identify normalized semantic properties, never raw `nodeToString()` fragments.
/// Node construction sorts these names and rejects duplicates so extractor field emission order
/// cannot affect governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalExpressionField {
    name: String,
    value: CanonicalExpressionValue,
}

impl CanonicalExpressionField {
    /// Creates one nonblank semantic field.
    pub fn new(
        name: impl Into<String>,
        value: CanonicalExpressionValue,
    ) -> Result<Self, ObservationError> {
        let name = name.into();
        validate_nonblank(&name, "canonical_expression_field_name")?;
        Ok(Self { name, value })
    }

    /// Returns the stable semantic field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the typed stable semantic field value.
    #[must_use]
    pub const fn value(&self) -> &CanonicalExpressionValue {
        &self.value
    }
}

/// Stable semantic field value used by [`CanonicalExpression`].
///
/// There is deliberately no raw catalog-OID or attribute-number variant. Catalog identities must
/// be resolved to qualified coordinates before entering this contract, while relation-local Vars
/// enter as [`CanonicalExpression::Column`] names. `Text` represents an already-normalized semantic
/// scalar such as a constant datum's canonical type output; it is not a carrier for rendered SQL,
/// `pg_node_tree`, or reconstructed DDL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalExpressionValue {
    /// Explicit semantic null.
    Null,
    /// Boolean semantic scalar.
    Boolean(bool),
    /// Signed integral semantic scalar, such as a type modifier or enum discriminant.
    Integer(i64),
    /// Exact normalized semantic text, not rendered SQL or raw node serialization.
    Text(String),
    /// Stable qualified PostgreSQL type identity.
    Type(QualifiedTypeName),
    /// Stable qualified PostgreSQL collation identity.
    Collation(QualifiedCollationName),
    /// Stable binary PostgreSQL operator identity.
    Operator(QualifiedOperatorSignature),
    /// Stable unary PostgreSQL operator identity.
    UnaryOperator(QualifiedUnaryOperatorSignature),
    /// Stable PostgreSQL function identity.
    Function(QualifiedFunctionSignature),
    /// One nested semantic expression.
    Expression(Box<CanonicalExpression>),
    /// Ordered nested expression list; list order remains part of PostgreSQL node equality.
    ExpressionList(Vec<CanonicalExpression>),
    /// Ordered stable semantic values for non-expression list fields.
    ValueList(Vec<CanonicalExpressionValue>),
}

/// Attribute-map-normalized semantic representation of a PostgreSQL index expression or predicate.
///
/// PostgreSQL 18 maps child Vars through the partition attribute map and then invokes internal
/// `equal()` on the resulting expression/predicate trees. This representation performs the stable
/// part of that transformation before immutable publication: relation-local user-column Vars become
/// exact column names, catalog OIDs become qualified signatures through typed field values, and
/// parse/render text is absent. A whole-row Var remains explicit because `CompareIndexInfo()` rejects
/// it when mapping the child tree without a target row type.
///
/// `Node` is a normalized semantic node tag plus the complete set of `equal()`-participating fields
/// emitted by the PostgreSQL 18 extractor contract. Parse locations and display-only rendering are
/// not semantic fields and must not be emitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalExpression {
    /// Relation-local user-column Var normalized to the exact column name.
    Column(String),
    /// Whole-row Var. It can be observed, but a direct attached child cannot preserve it.
    WholeRow,
    /// PostgreSQL semantic node with deterministic complete semantic fields.
    Node {
        /// Stable PostgreSQL semantic node tag, such as `FuncExpr` or `OpExpr`.
        node_kind: String,
        /// Deterministically sorted semantic fields.
        fields: Vec<CanonicalExpressionField>,
    },
}

impl CanonicalExpression {
    /// Creates a relation-local column reference without retaining its physical attribute number.
    pub fn column(column_name: impl Into<String>) -> Result<Self, ObservationError> {
        let column_name = column_name.into();
        validate_nonblank(&column_name, "canonical_expression_column_name")?;
        Ok(Self::Column(column_name))
    }

    /// Records a whole-row Var explicitly so attachment comparison can fail closed.
    #[must_use]
    pub const fn whole_row() -> Self {
        Self::WholeRow
    }

    /// Creates one normalized semantic node.
    ///
    /// Fields are canonicalized by exact name and duplicate field names are rejected. The extractor
    /// is responsible for supplying every field PostgreSQL 18 `equal()` considers for this node kind.
    pub fn node(
        node_kind: impl Into<String>,
        mut fields: Vec<CanonicalExpressionField>,
    ) -> Result<Self, ObservationError> {
        let node_kind = node_kind.into();
        validate_nonblank(&node_kind, "canonical_expression_node_kind")?;
        fields.sort_by(|left, right| left.name().cmp(right.name()));
        if fields
            .windows(2)
            .any(|pair| pair[0].name() == pair[1].name())
        {
            return Err(invalid("canonical_expression_field"));
        }
        Ok(Self::Node { node_kind, fields })
    }

    /// Returns whether this tree contains a whole-row relation Var at any depth.
    #[must_use]
    pub fn contains_whole_row(&self) -> bool {
        match self {
            Self::Column(_) => false,
            Self::WholeRow => true,
            Self::Node { fields, .. } => fields
                .iter()
                .any(|field| value_contains_whole_row(field.value())),
        }
    }

    fn collect_column_names<'a>(&'a self, names: &mut BTreeSet<&'a str>) {
        match self {
            Self::Column(column_name) => {
                names.insert(column_name.as_str());
            }
            Self::WholeRow => {}
            Self::Node { fields, .. } => {
                for field in fields {
                    collect_value_column_names(field.value(), names);
                }
            }
        }
    }
}

/// Canonical expression semantics for one zero-`indkey` index-key position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionSemanticsObservation {
    index: IndexPartitionCoordinate,
    key_position: u32,
    expression: CanonicalExpression,
}

impl IndexExpressionSemanticsObservation {
    /// Creates semantic evidence for one exact expression-key position.
    pub fn new(
        index: IndexPartitionCoordinate,
        key_position: u32,
        expression: CanonicalExpression,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            index,
            key_position,
            expression,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the one-based structural key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the stable canonical expression tree.
    #[must_use]
    pub const fn expression(&self) -> &CanonicalExpression {
        &self.expression
    }

    /// Returns the collision-safe evidence location.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "{}/keys/{}/expression-semantics",
            self.index.canonical_location(),
            self.key_position
        )
    }
}

/// Canonical partial-index predicate semantics for one relation-scoped index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPredicateSemanticsObservation {
    index: IndexPartitionCoordinate,
    predicate: CanonicalExpression,
}

impl IndexPredicateSemanticsObservation {
    /// Creates semantic evidence for one exact partial-index predicate.
    pub fn new(
        index: IndexPartitionCoordinate,
        predicate: CanonicalExpression,
    ) -> Result<Self, ObservationError> {
        Ok(Self { index, predicate })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the stable canonical predicate tree.
    #[must_use]
    pub const fn predicate(&self) -> &CanonicalExpression {
        &self.predicate
    }

    /// Returns the collision-safe evidence location.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!("{}/predicate-semantics", self.index.canonical_location())
    }
}

/// Exact location for an expression or predicate semantic receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IndexExpressionSemanticsLocation {
    /// One zero-`indkey` expression key.
    Expression {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
        /// One-based expression-key position.
        key_position: u32,
    },
    /// One partial-index predicate.
    Predicate {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
    },
}

impl IndexExpressionSemanticsLocation {
    fn canonical_location(&self) -> String {
        match self {
            Self::Expression {
                index,
                key_position,
            } => format!(
                "{}/keys/{key_position}/expression-semantics",
                index.canonical_location()
            ),
            Self::Predicate { index } => {
                format!("{}/predicate-semantics", index.canonical_location())
            }
        }
    }
}

/// Immutable provenance receipt for one exact expression/predicate semantic observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionSemanticsSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExpressionSemanticsLocation,
}

impl IndexExpressionSemanticsSourceReceipt {
    /// Returns the stable source registry key, never credentials.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed expression-semantics digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the canonical source observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact semantic evidence location.
    #[must_use]
    pub const fn location(&self) -> &IndexExpressionSemanticsLocation {
        &self.location
    }
}

/// Complete canonical expression/predicate evidence layered over exclusion semantics.
///
/// Every zero-`indkey` expression and every non-null `indpred` in the bounded frozen v3 predecessor
/// requires exactly one semantic observation. Relation-local column references are validated against
/// their owning relation. For every direct attached-index edge, child whole-row Vars fail closed and
/// parent/child canonical expression and predicate trees must be equal after normalization. The
/// digest frames the exact exclusion-semantics predecessor under a new domain separator, preserving
/// every earlier immutable identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionSemanticsSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    expression_observations: Vec<IndexExpressionSemanticsObservation>,
    predicate_observations: Vec<IndexPredicateSemanticsObservation>,
}

impl IndexExpressionSemanticsSnapshot {
    /// Creates complete expression/predicate semantics over one exact predecessor chain.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        operator_family_snapshot: &IndexOperatorFamilySnapshot,
        exclusion_snapshot: &IndexExclusionSemanticsSnapshot,
        expression_observations: Vec<IndexExpressionSemanticsObservation>,
        predicate_observations: Vec<IndexPredicateSemanticsObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexExclusionSemanticsSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot,
            exclusion_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != exclusion_snapshot.snapshot_digest() {
            return Err(invalid("index_expression_semantics_predecessor_binding"));
        }

        let expression_observations =
            canonicalize_expression_observations(base_snapshot, expression_observations)?;
        let predicate_observations =
            canonicalize_predicate_observations(base_snapshot, predicate_observations)?;
        validate_attached_expression_equivalence(
            index_partition_snapshot,
            &expression_observations,
            &predicate_observations,
        )?;

        let snapshot_digest = compute_expression_semantics_digest(
            exclusion_snapshot.snapshot_digest(),
            &expression_observations,
            &predicate_observations,
        );
        Ok(Self {
            source_connection_key: exclusion_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: exclusion_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: exclusion_snapshot.extractor_revision().to_owned(),
            observed_at_utc: exclusion_snapshot.observed_at_utc().to_owned(),
            expression_observations,
            predicate_observations,
        })
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated expression-semantics digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete expression observations in deterministic index/position order.
    #[must_use]
    pub fn expression_observations(&self) -> &[IndexExpressionSemanticsObservation] {
        &self.expression_observations
    }

    /// Returns complete predicate observations in deterministic index order.
    #[must_use]
    pub fn predicate_observations(&self) -> &[IndexPredicateSemanticsObservation] {
        &self.predicate_observations
    }

    /// Issues provenance for one exact semantic evidence location.
    pub fn source_receipt(
        &self,
        location: IndexExpressionSemanticsLocation,
    ) -> Result<IndexExpressionSemanticsSourceReceipt, ObservationError> {
        let exists = match &location {
            IndexExpressionSemanticsLocation::Expression {
                index,
                key_position,
            } => self.expression_observations.iter().any(|observation| {
                observation.index() == index && observation.key_position() == *key_position
            }),
            IndexExpressionSemanticsLocation::Predicate { index } => self
                .predicate_observations
                .iter()
                .any(|observation| observation.index() == index),
        };
        if !exists {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(IndexExpressionSemanticsSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }
}

fn canonicalize_expression_observations(
    base_snapshot: &PostgresSchemaSnapshotV3,
    mut observations: Vec<IndexExpressionSemanticsObservation>,
) -> Result<Vec<IndexExpressionSemanticsObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        left.index()
            .cmp(right.index())
            .then_with(|| left.key_position().cmp(&right.key_position()))
    });

    let expected = base_snapshot
        .relations()
        .iter()
        .flat_map(|relation| {
            relation.indexes().iter().flat_map(move |index| {
                index
                    .key_attributes()
                    .iter()
                    .filter(|attribute| attribute.expression_text().is_some())
                    .map(move |attribute| {
                        (
                            relation.schema_name().to_owned(),
                            relation.relation_name().to_owned(),
                            relation.kind().token().to_owned(),
                            index.index_name().to_owned(),
                            attribute.position(),
                        )
                    })
            })
        })
        .collect::<BTreeSet<_>>();
    let observed = observations
        .iter()
        .map(|observation| {
            (
                observation.index().schema_name().to_owned(),
                observation.index().relation_name().to_owned(),
                observation.index().relation_kind().token().to_owned(),
                observation.index().index_name().to_owned(),
                observation.key_position(),
            )
        })
        .collect::<BTreeSet<_>>();
    if observed.len() != observations.len() {
        return Err(invalid("index_expression_semantics_coordinate"));
    }
    if observed != expected {
        return Err(invalid("index_expression_semantics_completeness"));
    }

    for observation in &observations {
        validate_expression_columns(base_snapshot, observation.index(), observation.expression())?;
    }

    Ok(observations)
}

fn canonicalize_predicate_observations(
    base_snapshot: &PostgresSchemaSnapshotV3,
    mut observations: Vec<IndexPredicateSemanticsObservation>,
) -> Result<Vec<IndexPredicateSemanticsObservation>, ObservationError> {
    observations.sort_by(|left, right| left.index().cmp(right.index()));

    let expected = base_snapshot
        .relations()
        .iter()
        .flat_map(|relation| {
            relation
                .indexes()
                .iter()
                .filter(|index| index.predicate().is_some())
                .map(move |index| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        index.index_name().to_owned(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    let observed = observations
        .iter()
        .map(|observation| {
            (
                observation.index().schema_name().to_owned(),
                observation.index().relation_name().to_owned(),
                observation.index().relation_kind().token().to_owned(),
                observation.index().index_name().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    if observed.len() != observations.len() {
        return Err(invalid("index_predicate_semantics_coordinate"));
    }
    if observed != expected {
        return Err(invalid("index_predicate_semantics_completeness"));
    }

    for observation in &observations {
        validate_expression_columns(base_snapshot, observation.index(), observation.predicate())?;
    }

    Ok(observations)
}

fn validate_expression_columns(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index: &IndexPartitionCoordinate,
    expression: &CanonicalExpression,
) -> Result<(), ObservationError> {
    let relation = base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == index.schema_name()
                && relation.relation_name() == index.relation_name()
                && relation.kind() == index.relation_kind()
        })
        .ok_or_else(|| invalid("index_expression_semantics_index_binding"))?;

    if !relation
        .indexes()
        .iter()
        .any(|candidate| candidate.index_name() == index.index_name())
    {
        return Err(invalid("index_expression_semantics_index_binding"));
    }

    let known_columns = relation
        .columns()
        .iter()
        .map(|column| column.column_name())
        .collect::<BTreeSet<_>>();
    let mut referenced_columns = BTreeSet::new();
    expression.collect_column_names(&mut referenced_columns);
    if !referenced_columns.is_subset(&known_columns) {
        return Err(invalid("index_expression_semantics_column_binding"));
    }
    Ok(())
}

fn validate_attached_expression_equivalence(
    index_partition_snapshot: &IndexPartitionSnapshot,
    expression_observations: &[IndexExpressionSemanticsObservation],
    predicate_observations: &[IndexPredicateSemanticsObservation],
) -> Result<(), ObservationError> {
    let expressions = expression_observations
        .iter()
        .map(|observation| {
            (
                (observation.index().clone(), observation.key_position()),
                observation.expression(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let predicates = predicate_observations
        .iter()
        .map(|observation| (observation.index().clone(), observation.predicate()))
        .collect::<BTreeMap<_, _>>();

    for membership in index_partition_snapshot.observations() {
        let Some(parent) = membership.parent_index() else {
            continue;
        };

        for observation in expression_observations
            .iter()
            .filter(|observation| observation.index() == membership.coordinate())
        {
            let child_expression = observation.expression();
            let parent_expression = expressions
                .get(&(parent.clone(), observation.key_position()))
                .ok_or_else(|| invalid("index_expression_semantics_completeness"))?;
            if child_expression.contains_whole_row() {
                return Err(invalid("index_partition_definition_expression_whole_row"));
            }
            if child_expression != *parent_expression {
                return Err(invalid("index_partition_definition_expression"));
            }
        }

        let child_predicate = predicates.get(membership.coordinate()).copied();
        let parent_predicate = predicates.get(parent).copied();
        if child_predicate.is_some_and(CanonicalExpression::contains_whole_row) {
            return Err(invalid("index_partition_definition_predicate_whole_row"));
        }
        if child_predicate != parent_predicate {
            return Err(invalid("index_partition_definition_predicate"));
        }
    }

    Ok(())
}

fn value_contains_whole_row(value: &CanonicalExpressionValue) -> bool {
    match value {
        CanonicalExpressionValue::Expression(expression) => expression.contains_whole_row(),
        CanonicalExpressionValue::ExpressionList(expressions) => expressions
            .iter()
            .any(CanonicalExpression::contains_whole_row),
        CanonicalExpressionValue::ValueList(values) => values.iter().any(value_contains_whole_row),
        CanonicalExpressionValue::Null
        | CanonicalExpressionValue::Boolean(_)
        | CanonicalExpressionValue::Integer(_)
        | CanonicalExpressionValue::Text(_)
        | CanonicalExpressionValue::Type(_)
        | CanonicalExpressionValue::Collation(_)
        | CanonicalExpressionValue::Operator(_)
        | CanonicalExpressionValue::UnaryOperator(_)
        | CanonicalExpressionValue::Function(_) => false,
    }
}

fn collect_value_column_names<'a>(
    value: &'a CanonicalExpressionValue,
    names: &mut BTreeSet<&'a str>,
) {
    match value {
        CanonicalExpressionValue::Expression(expression) => expression.collect_column_names(names),
        CanonicalExpressionValue::ExpressionList(expressions) => {
            for expression in expressions {
                expression.collect_column_names(names);
            }
        }
        CanonicalExpressionValue::ValueList(values) => {
            for value in values {
                collect_value_column_names(value, names);
            }
        }
        CanonicalExpressionValue::Null
        | CanonicalExpressionValue::Boolean(_)
        | CanonicalExpressionValue::Integer(_)
        | CanonicalExpressionValue::Text(_)
        | CanonicalExpressionValue::Type(_)
        | CanonicalExpressionValue::Collation(_)
        | CanonicalExpressionValue::Operator(_)
        | CanonicalExpressionValue::UnaryOperator(_)
        | CanonicalExpressionValue::Function(_) => {}
    }
}

fn compute_expression_semantics_digest(
    predecessor_digest: &str,
    expressions: &[IndexExpressionSemanticsObservation],
    predicates: &[IndexPredicateSemanticsObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXPRESSION_SEMANTICS_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, EXPRESSION_SEMANTICS_SCHEMA_REVISION);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, expressions.len());
    for observation in expressions {
        encode_index_coordinate(&mut hasher, observation.index());
        hasher.update(observation.key_position().to_be_bytes());
        encode_expression(&mut hasher, observation.expression());
    }
    encode_len(&mut hasher, predicates.len());
    for observation in predicates {
        encode_index_coordinate(&mut hasher, observation.index());
        encode_expression(&mut hasher, observation.predicate());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_index_coordinate(hasher: &mut Sha256, index: &IndexPartitionCoordinate) {
    encode_str(hasher, index.schema_name());
    encode_str(hasher, index.relation_name());
    encode_str(hasher, index.relation_kind().token());
    encode_str(hasher, index.index_name());
}

fn encode_expression(hasher: &mut Sha256, expression: &CanonicalExpression) {
    match expression {
        CanonicalExpression::Column(column_name) => {
            hasher.update([0]);
            encode_str(hasher, column_name);
        }
        CanonicalExpression::WholeRow => hasher.update([1]),
        CanonicalExpression::Node { node_kind, fields } => {
            hasher.update([2]);
            encode_str(hasher, node_kind);
            encode_len(hasher, fields.len());
            for field in fields {
                encode_str(hasher, field.name());
                encode_expression_value(hasher, field.value());
            }
        }
    }
}

fn encode_expression_value(hasher: &mut Sha256, value: &CanonicalExpressionValue) {
    match value {
        CanonicalExpressionValue::Null => hasher.update([0]),
        CanonicalExpressionValue::Boolean(value) => {
            hasher.update([1]);
            hasher.update([u8::from(*value)]);
        }
        CanonicalExpressionValue::Integer(value) => {
            hasher.update([2]);
            hasher.update(value.to_be_bytes());
        }
        CanonicalExpressionValue::Text(value) => {
            hasher.update([3]);
            encode_str(hasher, value);
        }
        CanonicalExpressionValue::Type(value) => {
            hasher.update([4]);
            encode_type(hasher, value);
        }
        CanonicalExpressionValue::Collation(value) => {
            hasher.update([5]);
            encode_str(hasher, value.schema_name());
            encode_str(hasher, value.collation_name());
        }
        CanonicalExpressionValue::Operator(value) => {
            hasher.update([6]);
            encode_str(hasher, value.schema_name());
            encode_str(hasher, value.operator_name());
            encode_type(hasher, value.left_type());
            encode_type(hasher, value.right_type());
        }
        CanonicalExpressionValue::UnaryOperator(value) => {
            hasher.update([7]);
            encode_str(hasher, value.schema_name());
            encode_str(hasher, value.operator_name());
            encode_type(hasher, value.operand_type());
        }
        CanonicalExpressionValue::Function(value) => {
            hasher.update([8]);
            encode_str(hasher, value.schema_name());
            encode_str(hasher, value.function_name());
            encode_len(hasher, value.argument_types().len());
            for argument in value.argument_types() {
                encode_type(hasher, argument);
            }
            encode_type(hasher, value.return_type());
        }
        CanonicalExpressionValue::Expression(value) => {
            hasher.update([9]);
            encode_expression(hasher, value);
        }
        CanonicalExpressionValue::ExpressionList(values) => {
            hasher.update([10]);
            encode_len(hasher, values.len());
            for value in values {
                encode_expression(hasher, value);
            }
        }
        CanonicalExpressionValue::ValueList(values) => {
            hasher.update([11]);
            encode_len(hasher, values.len());
            for value in values {
                encode_expression_value(hasher, value);
            }
        }
    }
}

fn encode_type(hasher: &mut Sha256, value: &QualifiedTypeName) {
    encode_str(hasher, value.schema_name());
    encode_str(hasher, value.type_name());
}

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}
