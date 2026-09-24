//! Complete PostgreSQL 18 relation-`Var` equality evidence for index expressions and predicates.
//!
//! Historical node-schema v1 admitted column-name-only `Var` leaves, while v2 intentionally rejects
//! every relation `Var` until its full `equal()` state is modeled. A relation-`Var` successor cannot
//! therefore use a successful v2 snapshot as a concrete predecessor for the very trees it repairs.
//! This family is a separate, domain-separated branch over the exact expression-semantics snapshot.
//! It preserves v1/v2 unchanged while proving the stable source facts that PostgreSQL 18 keeps when
//! `map_variable_attnos()` remaps a child index expression before `equal()` comparison.

use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedTypeName,
};
use sha2::{Digest, Sha256};

use crate::RelationPartitionSnapshot;

use super::{
    CanonicalExpression, CanonicalExpressionValue, IndexExclusionSemanticsSnapshot,
    IndexExpressionSemanticsSnapshot, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionSnapshot, RelationPartitionTypeModifierSnapshot,
};

const RELATION_VAR_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.relation_var.v1";
const RELATION_VAR_SCHEMA_REVISION: &str = "postgresql-18-equal-relation-var-v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Stable semantic role of `Var.varno` inside the stored index-expression boundary.
///
/// A raw range-table index is statement-local and cannot enter immutable identity. PostgreSQL index
/// expressions and predicates are relation-local here, so only the indexed relation role is
/// admissible. `Other` exists solely so an extractor can fail closed instead of coercing a foreign
/// range-table role into the governed representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationVarRelationRole {
    /// The `Var` targets the relation whose index definition is being observed.
    IndexRelation,
    /// Any other range-table role, which is invalid for this bounded source contract.
    Other,
}

impl RelationVarRelationRole {
    fn token(self) -> &'static str {
        match self {
            Self::IndexRelation => "index_relation",
            Self::Other => "other",
        }
    }
}

/// Stable PostgreSQL `VarReturningType` state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationVarReturningType {
    /// Ordinary `Var` (`VAR_RETURNING_DEFAULT`).
    Default,
    /// `RETURNING OLD` reference (`VAR_RETURNING_OLD`).
    Old,
    /// `RETURNING NEW` reference (`VAR_RETURNING_NEW`).
    New,
}

impl RelationVarReturningType {
    fn token(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Old => "old",
            Self::New => "new",
        }
    }
}

/// Exact coordinate of one relation-`Var` leaf inside canonical expression/predicate evidence.
///
/// `leaf_position` is one-based preorder among relation-column leaves in the already canonicalized
/// expression tree. It is stable for equal trees and avoids retaining physical PostgreSQL `attnum`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IndexExpressionRelationVarLocation {
    /// A relation `Var` inside one expression index key.
    Expression {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
        /// One-based index key position.
        key_position: u32,
        /// One-based canonical relation-`Var` leaf position inside that expression.
        leaf_position: u32,
    },
    /// A relation `Var` inside one partial-index predicate.
    Predicate {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
        /// One-based canonical relation-`Var` leaf position inside that predicate.
        leaf_position: u32,
    },
}

impl IndexExpressionRelationVarLocation {
    /// Creates an expression-key relation-`Var` coordinate.
    pub fn expression(
        index: IndexPartitionCoordinate,
        key_position: u32,
        leaf_position: u32,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 || leaf_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self::Expression {
            index,
            key_position,
            leaf_position,
        })
    }

    /// Creates a predicate relation-`Var` coordinate.
    pub fn predicate(
        index: IndexPartitionCoordinate,
        leaf_position: u32,
    ) -> Result<Self, ObservationError> {
        if leaf_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self::Predicate {
            index,
            leaf_position,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        match self {
            Self::Expression { index, .. } | Self::Predicate { index, .. } => index,
        }
    }

    /// Returns the one-based relation-`Var` leaf position.
    #[must_use]
    pub const fn leaf_position(&self) -> u32 {
        match self {
            Self::Expression { leaf_position, .. } | Self::Predicate { leaf_position, .. } => {
                *leaf_position
            }
        }
    }

    /// Returns the expression key position when this location belongs to an expression key.
    #[must_use]
    pub const fn key_position(&self) -> Option<u32> {
        match self {
            Self::Expression { key_position, .. } => Some(*key_position),
            Self::Predicate { .. } => None,
        }
    }

    /// Returns a collision-safe governed evidence location.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        match self {
            Self::Expression {
                index,
                key_position,
                leaf_position,
            } => format!(
                "{}/keys/{key_position}/expression-semantics/relation-vars/{leaf_position}",
                index.canonical_location()
            ),
            Self::Predicate {
                index,
                leaf_position,
            } => format!(
                "{}/predicate-semantics/relation-vars/{leaf_position}",
                index.canonical_location()
            ),
        }
    }

    fn with_index(&self, index: IndexPartitionCoordinate) -> Self {
        match self {
            Self::Expression {
                key_position,
                leaf_position,
                ..
            } => Self::Expression {
                index,
                key_position: *key_position,
                leaf_position: *leaf_position,
            },
            Self::Predicate { leaf_position, .. } => Self::Predicate {
                index,
                leaf_position: *leaf_position,
            },
        }
    }
}

/// Complete stable equality state for one PostgreSQL relation `Var` leaf.
///
/// Raw `varno`, physical `varattno`, database-local type/collation OIDs, syntactic `varnosyn` /
/// `varattnosyn`, and parse location are deliberately absent. The first two are normalized to the
/// indexed-relation role and exact column name; type/collation OIDs are resolved to qualified stable
/// coordinates. Raw `vartypmod` remains a signed `int4` because PostgreSQL compares it exactly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionRelationVarObservation {
    location: IndexExpressionRelationVarLocation,
    column_name: String,
    value_type: QualifiedTypeName,
    type_modifier: i32,
    collation: Option<QualifiedCollationName>,
    relation_role: RelationVarRelationRole,
    nulling_relations_empty: bool,
    levels_up: u32,
    returning_type: RelationVarReturningType,
}

impl IndexExpressionRelationVarObservation {
    /// Creates complete relation-`Var` equality evidence for the stored index-expression boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        location: IndexExpressionRelationVarLocation,
        column_name: impl Into<String>,
        value_type: QualifiedTypeName,
        type_modifier: i32,
        collation: Option<QualifiedCollationName>,
        relation_role: RelationVarRelationRole,
        nulling_relations_empty: bool,
        levels_up: u32,
        returning_type: RelationVarReturningType,
    ) -> Result<Self, ObservationError> {
        let column_name = column_name.into();
        validate_nonblank(&column_name, "index_expression_relation_var_column_name")?;
        if relation_role != RelationVarRelationRole::IndexRelation {
            return Err(invalid("index_expression_relation_var_role"));
        }
        if !nulling_relations_empty {
            return Err(invalid("index_expression_relation_var_nulling_relations"));
        }
        if levels_up != 0 {
            return Err(invalid("index_expression_relation_var_levels_up"));
        }
        if returning_type != RelationVarReturningType::Default {
            return Err(invalid("index_expression_relation_var_returning_type"));
        }
        Ok(Self {
            location,
            column_name,
            value_type,
            type_modifier,
            collation,
            relation_role,
            nulling_relations_empty,
            levels_up,
            returning_type,
        })
    }

    /// Returns the exact canonical relation-`Var` leaf location.
    #[must_use]
    pub const fn location(&self) -> &IndexExpressionRelationVarLocation {
        &self.location
    }

    /// Returns the attribute-map-normalized exact column identity.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns stable qualified `vartype` identity.
    #[must_use]
    pub const fn value_type(&self) -> &QualifiedTypeName {
        &self.value_type
    }

    /// Returns exact raw signed `vartypmod`.
    #[must_use]
    pub const fn type_modifier(&self) -> i32 {
        self.type_modifier
    }

    /// Returns stable qualified `varcollid`, or `None` for `InvalidOid`.
    #[must_use]
    pub fn collation(&self) -> Option<&QualifiedCollationName> {
        self.collation.as_ref()
    }

    /// Returns the normalized `varno` role.
    #[must_use]
    pub const fn relation_role(&self) -> RelationVarRelationRole {
        self.relation_role
    }

    /// Returns whether `varnullingrels` was explicitly observed empty.
    #[must_use]
    pub const fn nulling_relations_empty(&self) -> bool {
        self.nulling_relations_empty
    }

    /// Returns exact `varlevelsup`.
    #[must_use]
    pub const fn levels_up(&self) -> u32 {
        self.levels_up
    }

    /// Returns exact `varreturningtype`.
    #[must_use]
    pub const fn returning_type(&self) -> RelationVarReturningType {
        self.returning_type
    }

    fn semantic_equal(&self, other: &Self) -> bool {
        self.column_name == other.column_name
            && self.value_type == other.value_type
            && self.type_modifier == other.type_modifier
            && self.collation == other.collation
            && self.relation_role == other.relation_role
            && self.nulling_relations_empty == other.nulling_relations_empty
            && self.levels_up == other.levels_up
            && self.returning_type == other.returning_type
    }
}

/// Immutable provenance receipt for one exact relation-`Var` equality observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionRelationVarSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExpressionRelationVarLocation,
}

impl IndexExpressionRelationVarSourceReceipt {
    /// Returns the stable source registry key, never credentials.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the governed relation-`Var` successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact verified relation-`Var` coordinate.
    #[must_use]
    pub const fn location(&self) -> &IndexExpressionRelationVarLocation {
        &self.location
    }
}

/// Complete PostgreSQL 18 relation-`Var` equality evidence over canonical expression semantics.
///
/// This successor deliberately does not consume `IndexExpressionNodeSchemaSnapshotV2`: v2 rejects
/// every incomplete relation-`Var` leaf and therefore cannot exist for the trees this family repairs.
/// Instead the constructor rebound-validates the exact expression-semantics predecessor, requires a
/// complete one-per-column-leaf relation-`Var` family, binds each leaf to source-authoritative column
/// type, raw `atttypmod`, and collation evidence, and checks direct attached parent/child leaves for
/// equality after the already-governed attribute-map column normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionRelationVarSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExpressionRelationVarObservation>,
}

impl IndexExpressionRelationVarSnapshot {
    /// Creates complete relation-`Var` equality evidence over exact predecessor snapshots.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        operator_family_snapshot: &IndexOperatorFamilySnapshot,
        exclusion_snapshot: &IndexExclusionSemanticsSnapshot,
        expression_snapshot: &IndexExpressionSemanticsSnapshot,
        type_modifier_snapshot: &RelationPartitionTypeModifierSnapshot,
        observations: Vec<IndexExpressionRelationVarObservation>,
    ) -> Result<Self, ObservationError> {
        validate_predecessors(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot,
            exclusion_snapshot,
            expression_snapshot,
            type_modifier_snapshot,
        )?;

        let observations = canonicalize_and_validate(
            base_snapshot,
            expression_snapshot,
            type_modifier_snapshot,
            observations,
        )?;
        validate_attached_var_equivalence(index_partition_snapshot, &observations)?;

        let snapshot_digest = compute_digest(
            expression_snapshot.snapshot_digest(),
            type_modifier_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: expression_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: expression_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: expression_snapshot.extractor_revision().to_owned(),
            observed_at_utc: expression_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated relation-`Var` digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the expression predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete relation-`Var` observations in deterministic location order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExpressionRelationVarObservation] {
        &self.observations
    }

    /// Issues provenance for one exact relation-`Var` coordinate when it exists.
    pub fn source_receipt(
        &self,
        location: IndexExpressionRelationVarLocation,
    ) -> Result<IndexExpressionRelationVarSourceReceipt, ObservationError> {
        if !self
            .observations
            .iter()
            .any(|observation| observation.location() == &location)
        {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(IndexExpressionRelationVarSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_predecessors(
    base_snapshot: &PostgresSchemaSnapshotV3,
    relation_partition_snapshot: &RelationPartitionSnapshot,
    index_partition_snapshot: &IndexPartitionSnapshot,
    operator_family_snapshot: &IndexOperatorFamilySnapshot,
    exclusion_snapshot: &IndexExclusionSemanticsSnapshot,
    expression_snapshot: &IndexExpressionSemanticsSnapshot,
    type_modifier_snapshot: &RelationPartitionTypeModifierSnapshot,
) -> Result<(), ObservationError> {
    let rebound_expression = IndexExpressionSemanticsSnapshot::new(
        base_snapshot,
        relation_partition_snapshot,
        index_partition_snapshot,
        operator_family_snapshot,
        exclusion_snapshot,
        expression_snapshot.expression_observations().to_vec(),
        expression_snapshot.predicate_observations().to_vec(),
    )?;
    if rebound_expression.snapshot_digest() != expression_snapshot.snapshot_digest() {
        return Err(invalid(
            "index_expression_relation_var_expression_predecessor",
        ));
    }

    let rebound_type_modifier = RelationPartitionTypeModifierSnapshot::new(
        base_snapshot,
        relation_partition_snapshot,
        type_modifier_snapshot.observations().to_vec(),
    )?;
    if rebound_type_modifier.snapshot_digest() != type_modifier_snapshot.snapshot_digest() {
        return Err(invalid(
            "index_expression_relation_var_type_modifier_predecessor",
        ));
    }

    if expression_snapshot.source_connection_key() != type_modifier_snapshot.source_connection_key()
        || expression_snapshot.connection_policy_binding()
            != type_modifier_snapshot.connection_policy_binding()
        || expression_snapshot.extractor_revision() != type_modifier_snapshot.extractor_revision()
        || expression_snapshot.observed_at_utc() != type_modifier_snapshot.observed_at_utc()
    {
        return Err(invalid(
            "index_expression_relation_var_predecessor_provenance",
        ));
    }
    Ok(())
}

fn canonicalize_and_validate(
    base_snapshot: &PostgresSchemaSnapshotV3,
    expression_snapshot: &IndexExpressionSemanticsSnapshot,
    type_modifier_snapshot: &RelationPartitionTypeModifierSnapshot,
    mut observations: Vec<IndexExpressionRelationVarObservation>,
) -> Result<Vec<IndexExpressionRelationVarObservation>, ObservationError> {
    let expected = expected_var_leaves(expression_snapshot)?;
    if expected.is_empty() {
        return Err(invalid("index_expression_relation_var_empty"));
    }

    observations.sort_by_key(|observation| observation.location().canonical_location());
    if observations.windows(2).any(|pair| {
        pair[0].location().canonical_location() == pair[1].location().canonical_location()
    }) {
        return Err(invalid("index_expression_relation_var_coordinate"));
    }

    let expected_locations = expected.keys().cloned().collect::<BTreeSet<_>>();
    let observed_locations = observations
        .iter()
        .map(|observation| observation.location().canonical_location())
        .collect::<BTreeSet<_>>();
    if observed_locations != expected_locations {
        return Err(invalid("index_expression_relation_var_completeness"));
    }

    let column_collations = base_snapshot
        .column_collations()
        .ok_or_else(|| invalid("index_expression_relation_var_column_collation_evidence"))?;

    for observation in &observations {
        let location_key = observation.location().canonical_location();
        let expected_column = expected
            .get(&location_key)
            .ok_or_else(|| invalid("index_expression_relation_var_coordinate"))?;
        if observation.column_name() != expected_column {
            return Err(invalid("index_expression_relation_var_column"));
        }

        let index = observation.location().index();
        let relation = base_snapshot
            .relations()
            .iter()
            .find(|relation| {
                relation.schema_name() == index.schema_name()
                    && relation.relation_name() == index.relation_name()
                    && relation.kind() == index.relation_kind()
            })
            .ok_or_else(|| invalid("index_expression_relation_var_owner"))?;
        let column = relation
            .columns()
            .iter()
            .find(|column| column.column_name() == observation.column_name())
            .ok_or_else(|| invalid("index_expression_relation_var_column"))?;
        if column.type_binding() != observation.value_type() {
            return Err(invalid("index_expression_relation_var_value_type"));
        }

        let type_modifier = type_modifier_snapshot
            .observations()
            .iter()
            .find(|candidate| {
                candidate.schema_name() == index.schema_name()
                    && candidate.relation_name() == index.relation_name()
                    && candidate.relation_kind() == index.relation_kind()
                    && candidate.column_name() == observation.column_name()
            })
            .ok_or_else(|| invalid("index_expression_relation_var_type_modifier_evidence"))?;
        if type_modifier.type_modifier() != observation.type_modifier() {
            return Err(invalid("index_expression_relation_var_type_modifier"));
        }

        let column_collation = column_collations
            .iter()
            .find(|candidate| {
                candidate.schema_name() == index.schema_name()
                    && candidate.relation_name() == index.relation_name()
                    && candidate.relation_kind() == index.relation_kind()
                    && candidate.column_name() == observation.column_name()
            })
            .ok_or_else(|| invalid("index_expression_relation_var_column_collation_evidence"))?;
        if column_collation.collation() != observation.collation() {
            return Err(invalid("index_expression_relation_var_collation"));
        }
    }

    Ok(observations)
}

fn expected_var_leaves(
    expression_snapshot: &IndexExpressionSemanticsSnapshot,
) -> Result<BTreeMap<String, String>, ObservationError> {
    let mut expected = BTreeMap::new();

    for observation in expression_snapshot.expression_observations() {
        let mut leaves = Vec::new();
        collect_var_leaves(observation.expression(), &mut leaves)?;
        for (offset, column_name) in leaves.into_iter().enumerate() {
            let leaf_position = u32::try_from(offset + 1)
                .map_err(|_| invalid("index_expression_relation_var_leaf_position"))?;
            let location = IndexExpressionRelationVarLocation::expression(
                observation.index().clone(),
                observation.key_position(),
                leaf_position,
            )?;
            expected.insert(location.canonical_location(), column_name);
        }
    }

    for observation in expression_snapshot.predicate_observations() {
        let mut leaves = Vec::new();
        collect_var_leaves(observation.predicate(), &mut leaves)?;
        for (offset, column_name) in leaves.into_iter().enumerate() {
            let leaf_position = u32::try_from(offset + 1)
                .map_err(|_| invalid("index_expression_relation_var_leaf_position"))?;
            let location = IndexExpressionRelationVarLocation::predicate(
                observation.index().clone(),
                leaf_position,
            )?;
            expected.insert(location.canonical_location(), column_name);
        }
    }

    Ok(expected)
}

fn collect_var_leaves(
    expression: &CanonicalExpression,
    leaves: &mut Vec<String>,
) -> Result<(), ObservationError> {
    match expression {
        CanonicalExpression::Column(column_name) => leaves.push(column_name.clone()),
        CanonicalExpression::WholeRow => {
            return Err(invalid("index_expression_relation_var_whole_row"));
        }
        CanonicalExpression::Node { fields, .. } => {
            for field in fields {
                collect_value_var_leaves(field.value(), leaves)?;
            }
        }
    }
    Ok(())
}

fn collect_value_var_leaves(
    value: &CanonicalExpressionValue,
    leaves: &mut Vec<String>,
) -> Result<(), ObservationError> {
    match value {
        CanonicalExpressionValue::Expression(expression) => collect_var_leaves(expression, leaves)?,
        CanonicalExpressionValue::ExpressionList(expressions) => {
            for expression in expressions {
                collect_var_leaves(expression, leaves)?;
            }
        }
        CanonicalExpressionValue::ValueList(values) => {
            for nested in values {
                collect_value_var_leaves(nested, leaves)?;
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
    Ok(())
}

fn validate_attached_var_equivalence(
    index_partition_snapshot: &IndexPartitionSnapshot,
    observations: &[IndexExpressionRelationVarObservation],
) -> Result<(), ObservationError> {
    let by_location = observations
        .iter()
        .map(|observation| (observation.location().canonical_location(), observation))
        .collect::<BTreeMap<_, _>>();

    for membership in index_partition_snapshot.observations() {
        let Some(parent_index) = membership.parent_index() else {
            continue;
        };
        let child_index = membership.coordinate();

        for child in observations
            .iter()
            .filter(|observation| observation.location().index() == child_index)
        {
            let parent_location = child.location().with_index(parent_index.clone());
            let parent = by_location
                .get(&parent_location.canonical_location())
                .ok_or_else(|| invalid("index_expression_relation_var_parent_evidence"))?;
            if !child.semantic_equal(parent) {
                return Err(invalid(
                    "index_expression_relation_var_attached_equivalence",
                ));
            }
        }
    }
    Ok(())
}

fn compute_digest(
    expression_digest: &str,
    type_modifier_digest: &str,
    observations: &[IndexExpressionRelationVarObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, RELATION_VAR_DIGEST_DOMAIN_V1);
    encode_bytes(&mut hasher, RELATION_VAR_SCHEMA_REVISION.as_bytes());
    encode_bytes(&mut hasher, expression_digest.as_bytes());
    encode_bytes(&mut hasher, type_modifier_digest.as_bytes());
    encode_u64(&mut hasher, observations.len() as u64);
    for observation in observations {
        encode_bytes(
            &mut hasher,
            observation.location().canonical_location().as_bytes(),
        );
        encode_bytes(&mut hasher, observation.column_name().as_bytes());
        encode_bytes(
            &mut hasher,
            observation.value_type().schema_name().as_bytes(),
        );
        encode_bytes(&mut hasher, observation.value_type().type_name().as_bytes());
        hasher.update(observation.type_modifier().to_be_bytes());
        match observation.collation() {
            Some(collation) => {
                hasher.update([1]);
                encode_bytes(&mut hasher, collation.schema_name().as_bytes());
                encode_bytes(&mut hasher, collation.collation_name().as_bytes());
            }
            None => hasher.update([0]),
        }
        encode_bytes(&mut hasher, observation.relation_role().token().as_bytes());
        hasher.update([u8::from(observation.nulling_relations_empty())]);
        hasher.update(observation.levels_up().to_be_bytes());
        encode_bytes(&mut hasher, observation.returning_type().token().as_bytes());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    encode_u64(hasher, bytes.len() as u64);
    hasher.update(bytes);
}

fn encode_u64(hasher: &mut Sha256, value: u64) {
    hasher.update(value.to_be_bytes());
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
