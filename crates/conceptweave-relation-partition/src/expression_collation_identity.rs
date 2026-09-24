//! Catalog-exact PostgreSQL collation identity for modeled index-expression equality.
//!
//! Historical canonical expression and relation-`Var` evidence resolves collation OIDs to qualified
//! namespace/name coordinates. PostgreSQL 18 permits distinct `pg_collation` rows with that same
//! two-part name when `collencoding` differs, while attached-index equality compares resolved OIDs.
//! This successor composes the already-issued whole-tree and key-collation proofs with explicit
//! catalog-row identity for every modeled expression/predicate collation field and relation-`Var`
//! collation leaf without changing any predecessor digest.

use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName,
};
use sha2::{Digest, Sha256};

use crate::RelationPartitionSnapshot;

use super::{
    CanonicalExpression, CanonicalExpressionValue, CollationCatalogIdentity,
    IndexExclusionSemanticsSnapshot, IndexExpressionNodeSchemaSnapshot,
    IndexExpressionRelationVarLocation, IndexExpressionRelationVarNodeSchemaSnapshot,
    IndexExpressionRelationVarSnapshot, IndexExpressionSemanticsSnapshot,
    IndexOperatorFamilySnapshot, IndexPartitionCollationIdentitySnapshot, IndexPartitionCoordinate,
    IndexPartitionSnapshot, RelationPartitionTypeModifierSnapshot,
};

const EXPRESSION_COLLATION_IDENTITY_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.expression_collation_catalog_identity.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Stable location of one collation-bearing field in canonical index expression/predicate evidence.
///
/// `occurrence_position` is one-based preorder among [`CanonicalExpressionValue::Collation`]
/// occurrences after deterministic canonical field sorting. The position is stable for trees that
/// the historical expression predecessor already considers equal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IndexExpressionCollationIdentityLocation {
    /// One collation field inside an expression index key.
    Expression {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
        /// One-based key position.
        key_position: u32,
        /// One-based collation occurrence inside the canonical expression tree.
        occurrence_position: u32,
    },
    /// One collation field inside a partial-index predicate.
    Predicate {
        /// Exact relation-scoped index coordinate.
        index: IndexPartitionCoordinate,
        /// One-based collation occurrence inside the canonical predicate tree.
        occurrence_position: u32,
    },
}

impl IndexExpressionCollationIdentityLocation {
    /// Creates one expression-key collation occurrence coordinate.
    pub fn expression(
        index: IndexPartitionCoordinate,
        key_position: u32,
        occurrence_position: u32,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 || occurrence_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self::Expression {
            index,
            key_position,
            occurrence_position,
        })
    }

    /// Creates one predicate collation occurrence coordinate.
    pub fn predicate(
        index: IndexPartitionCoordinate,
        occurrence_position: u32,
    ) -> Result<Self, ObservationError> {
        if occurrence_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self::Predicate {
            index,
            occurrence_position,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        match self {
            Self::Expression { index, .. } | Self::Predicate { index, .. } => index,
        }
    }

    /// Returns the one-based expression key position, when this is an expression-key location.
    #[must_use]
    pub const fn key_position(&self) -> Option<u32> {
        match self {
            Self::Expression { key_position, .. } => Some(*key_position),
            Self::Predicate { .. } => None,
        }
    }

    /// Returns the one-based collation occurrence position.
    #[must_use]
    pub const fn occurrence_position(&self) -> u32 {
        match self {
            Self::Expression {
                occurrence_position,
                ..
            }
            | Self::Predicate {
                occurrence_position,
                ..
            } => *occurrence_position,
        }
    }

    /// Returns the collision-safe governed evidence coordinate.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        match self {
            Self::Expression {
                index,
                key_position,
                occurrence_position,
            } => format!(
                "{}/keys/{key_position}/expression-semantics/collations/{occurrence_position}",
                index.canonical_location()
            ),
            Self::Predicate {
                index,
                occurrence_position,
            } => format!(
                "{}/predicate-semantics/collations/{occurrence_position}",
                index.canonical_location()
            ),
        }
    }

    fn with_index(&self, index: IndexPartitionCoordinate) -> Self {
        match self {
            Self::Expression {
                key_position,
                occurrence_position,
                ..
            } => Self::Expression {
                index,
                key_position: *key_position,
                occurrence_position: *occurrence_position,
            },
            Self::Predicate {
                occurrence_position,
                ..
            } => Self::Predicate {
                index,
                occurrence_position: *occurrence_position,
            },
        }
    }
}

/// Catalog-row identity for one historical expression/predicate qualified-collation occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionCollationIdentityObservation {
    location: IndexExpressionCollationIdentityLocation,
    collation: CollationCatalogIdentity,
}

impl IndexExpressionCollationIdentityObservation {
    /// Creates one catalog-exact expression/predicate collation observation.
    pub fn new(
        location: IndexExpressionCollationIdentityLocation,
        collation: CollationCatalogIdentity,
    ) -> Result<Self, ObservationError> {
        Ok(Self {
            location,
            collation,
        })
    }

    /// Returns the exact canonical expression/predicate occurrence location.
    #[must_use]
    pub const fn location(&self) -> &IndexExpressionCollationIdentityLocation {
        &self.location
    }

    /// Returns the resolved exact `pg_collation` catalog identity.
    #[must_use]
    pub const fn collation(&self) -> &CollationCatalogIdentity {
        &self.collation
    }
}

/// Catalog-row identity for one historical relation-`Var.varcollid` occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexRelationVarCollationIdentityObservation {
    location: IndexExpressionRelationVarLocation,
    collation: Option<CollationCatalogIdentity>,
}

impl IndexRelationVarCollationIdentityObservation {
    /// Creates one relation-`Var` collation observation; `None` proves `InvalidOid`/no collation.
    pub fn new(
        location: IndexExpressionRelationVarLocation,
        collation: Option<CollationCatalogIdentity>,
    ) -> Result<Self, ObservationError> {
        Ok(Self {
            location,
            collation,
        })
    }

    /// Returns the exact canonical relation-`Var` leaf location.
    #[must_use]
    pub const fn location(&self) -> &IndexExpressionRelationVarLocation {
        &self.location
    }

    /// Returns the resolved catalog identity, or `None` for an explicitly uncollated `Var`.
    #[must_use]
    pub const fn collation(&self) -> Option<&CollationCatalogIdentity> {
        self.collation.as_ref()
    }
}

/// Catalog-exact collation composition over the existing whole-tree and key-collation proofs.
///
/// The constructor rebound-validates the issued whole-tree proof and the per-key catalog-identity
/// successor, requires complete catalog identities for every canonical `Collation` value and every
/// relation-`Var` leaf, binds them back to the historical qualified names, and compares full catalog
/// identity across each direct attached-index edge. Historical expression, relation-`Var`, composed,
/// and key-collation digests remain immutable predecessors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionCollationIdentitySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    whole_tree_predecessor_digest: String,
    key_collation_predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    expression_observations: Vec<IndexExpressionCollationIdentityObservation>,
    relation_var_observations: Vec<IndexRelationVarCollationIdentityObservation>,
}

impl IndexExpressionCollationIdentitySnapshot {
    /// Creates catalog-exact collation composition over the exact bounded predecessor stack.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        operator_family_snapshot: &IndexOperatorFamilySnapshot,
        exclusion_snapshot: &IndexExclusionSemanticsSnapshot,
        expression_snapshot: &IndexExpressionSemanticsSnapshot,
        type_modifier_snapshot: &RelationPartitionTypeModifierSnapshot,
        node_schema_predecessor: &IndexExpressionNodeSchemaSnapshot,
        relation_var_predecessor: &IndexExpressionRelationVarSnapshot,
        whole_tree_predecessor: &IndexExpressionRelationVarNodeSchemaSnapshot,
        key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
        expression_observations: Vec<IndexExpressionCollationIdentityObservation>,
        relation_var_observations: Vec<IndexRelationVarCollationIdentityObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound_whole_tree = IndexExpressionRelationVarNodeSchemaSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot,
            exclusion_snapshot,
            expression_snapshot,
            type_modifier_snapshot,
            node_schema_predecessor,
            relation_var_predecessor,
        )?;
        if rebound_whole_tree.snapshot_digest() != whole_tree_predecessor.snapshot_digest() {
            return Err(invalid("index_expression_collation_whole_tree_predecessor"));
        }

        let rebound_key_collation = IndexPartitionCollationIdentitySnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            key_collation_predecessor.observations().to_vec(),
        )?;
        if rebound_key_collation.snapshot_digest() != key_collation_predecessor.snapshot_digest() {
            return Err(invalid("index_expression_collation_key_predecessor"));
        }

        if whole_tree_predecessor.source_connection_key()
            != key_collation_predecessor.source_connection_key()
            || whole_tree_predecessor.connection_policy_binding()
                != key_collation_predecessor.connection_policy_binding()
            || whole_tree_predecessor.extractor_revision()
                != key_collation_predecessor.extractor_revision()
            || whole_tree_predecessor.observed_at_utc()
                != key_collation_predecessor.observed_at_utc()
        {
            return Err(invalid("index_expression_collation_predecessor_provenance"));
        }

        let expression_observations =
            canonicalize_expression_collations(expression_snapshot, expression_observations)?;
        let relation_var_observations = canonicalize_relation_var_collations(
            relation_var_predecessor,
            relation_var_observations,
        )?;
        validate_attached_expression_collations(
            index_partition_snapshot,
            &expression_observations,
        )?;
        validate_attached_relation_var_collations(
            index_partition_snapshot,
            &relation_var_observations,
        )?;

        let whole_tree_predecessor_digest = whole_tree_predecessor.snapshot_digest().to_owned();
        let key_collation_predecessor_digest =
            key_collation_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest(
            &whole_tree_predecessor_digest,
            &key_collation_predecessor_digest,
            &expression_observations,
            &relation_var_observations,
        );

        Ok(Self {
            source_connection_key: whole_tree_predecessor.source_connection_key().to_owned(),
            connection_policy_binding: whole_tree_predecessor
                .connection_policy_binding()
                .to_owned(),
            whole_tree_predecessor_digest,
            key_collation_predecessor_digest,
            snapshot_digest,
            extractor_revision: whole_tree_predecessor.extractor_revision().to_owned(),
            observed_at_utc: whole_tree_predecessor.observed_at_utc().to_owned(),
            expression_observations,
            relation_var_observations,
        })
    }

    /// Returns the stable source registry key inherited from the predecessor stack.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding inherited from the predecessor stack.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact whole-tree equality proof digest consumed by this composition.
    #[must_use]
    pub fn whole_tree_predecessor_digest(&self) -> &str {
        &self.whole_tree_predecessor_digest
    }

    /// Returns the exact per-key catalog-collation proof digest consumed by this composition.
    #[must_use]
    pub fn key_collation_predecessor_digest(&self) -> &str {
        &self.key_collation_predecessor_digest
    }

    /// Returns the owner-computed domain-separated composition digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete expression/predicate collation observations in canonical location order.
    #[must_use]
    pub fn expression_observations(&self) -> &[IndexExpressionCollationIdentityObservation] {
        &self.expression_observations
    }

    /// Returns complete relation-`Var` collation observations in canonical location order.
    #[must_use]
    pub fn relation_var_observations(&self) -> &[IndexRelationVarCollationIdentityObservation] {
        &self.relation_var_observations
    }
}

fn canonicalize_expression_collations(
    expression_snapshot: &IndexExpressionSemanticsSnapshot,
    mut observations: Vec<IndexExpressionCollationIdentityObservation>,
) -> Result<Vec<IndexExpressionCollationIdentityObservation>, ObservationError> {
    observations.sort_by_key(|observation| observation.location().canonical_location());

    let mut expected = BTreeMap::<String, QualifiedCollationName>::new();
    for observation in expression_snapshot.expression_observations() {
        let mut collations = Vec::new();
        collect_expression_collations(observation.expression(), &mut collations);
        for (offset, collation) in collations.into_iter().enumerate() {
            let occurrence_position = u32::try_from(offset + 1)
                .map_err(|_| invalid("index_expression_collation_occurrence_position"))?;
            let location = IndexExpressionCollationIdentityLocation::expression(
                observation.index().clone(),
                observation.key_position(),
                occurrence_position,
            )?;
            expected.insert(location.canonical_location(), collation);
        }
    }
    for observation in expression_snapshot.predicate_observations() {
        let mut collations = Vec::new();
        collect_expression_collations(observation.predicate(), &mut collations);
        for (offset, collation) in collations.into_iter().enumerate() {
            let occurrence_position = u32::try_from(offset + 1)
                .map_err(|_| invalid("index_expression_collation_occurrence_position"))?;
            let location = IndexExpressionCollationIdentityLocation::predicate(
                observation.index().clone(),
                occurrence_position,
            )?;
            expected.insert(location.canonical_location(), collation);
        }
    }

    let observed_locations = observations
        .iter()
        .map(|observation| observation.location().canonical_location())
        .collect::<BTreeSet<_>>();
    if observed_locations.len() != observations.len()
        || observed_locations != expected.keys().cloned().collect::<BTreeSet<_>>()
    {
        return Err(invalid("index_expression_collation_catalog_completeness"));
    }

    for observation in &observations {
        let expected_name = expected
            .get(&observation.location().canonical_location())
            .ok_or_else(|| invalid("index_expression_collation_catalog_coordinate"))?;
        if observation.collation().schema_name() != expected_name.schema_name()
            || observation.collation().collation_name() != expected_name.collation_name()
        {
            return Err(invalid("index_expression_collation_catalog_binding"));
        }
    }
    Ok(observations)
}

fn canonicalize_relation_var_collations(
    relation_var_snapshot: &IndexExpressionRelationVarSnapshot,
    mut observations: Vec<IndexRelationVarCollationIdentityObservation>,
) -> Result<Vec<IndexRelationVarCollationIdentityObservation>, ObservationError> {
    observations.sort_by_key(|observation| observation.location().canonical_location());

    let expected = relation_var_snapshot
        .observations()
        .iter()
        .map(|observation| {
            (
                observation.location().canonical_location(),
                observation.collation().cloned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let observed_locations = observations
        .iter()
        .map(|observation| observation.location().canonical_location())
        .collect::<BTreeSet<_>>();
    if observed_locations.len() != observations.len()
        || observed_locations != expected.keys().cloned().collect::<BTreeSet<_>>()
    {
        return Err(invalid("index_relation_var_collation_catalog_completeness"));
    }

    for observation in &observations {
        let expected_name = expected
            .get(&observation.location().canonical_location())
            .ok_or_else(|| invalid("index_relation_var_collation_catalog_coordinate"))?;
        let actual_name = observation
            .collation()
            .map(|collation| (collation.schema_name(), collation.collation_name()));
        let expected_name = expected_name
            .as_ref()
            .map(|collation| (collation.schema_name(), collation.collation_name()));
        if actual_name != expected_name {
            return Err(invalid("index_relation_var_collation_catalog_binding"));
        }
    }
    Ok(observations)
}

fn collect_expression_collations(
    expression: &CanonicalExpression,
    collations: &mut Vec<QualifiedCollationName>,
) {
    match expression {
        CanonicalExpression::Column(_) | CanonicalExpression::WholeRow => {}
        CanonicalExpression::Node { fields, .. } => {
            for field in fields {
                collect_value_collations(field.value(), collations);
            }
        }
    }
}

fn collect_value_collations(
    value: &CanonicalExpressionValue,
    collations: &mut Vec<QualifiedCollationName>,
) {
    match value {
        CanonicalExpressionValue::Collation(collation) => collations.push(collation.clone()),
        CanonicalExpressionValue::Expression(expression) => {
            collect_expression_collations(expression, collations);
        }
        CanonicalExpressionValue::ExpressionList(expressions) => {
            for expression in expressions {
                collect_expression_collations(expression, collations);
            }
        }
        CanonicalExpressionValue::ValueList(values) => {
            for value in values {
                collect_value_collations(value, collations);
            }
        }
        CanonicalExpressionValue::Null
        | CanonicalExpressionValue::Boolean(_)
        | CanonicalExpressionValue::Integer(_)
        | CanonicalExpressionValue::Text(_)
        | CanonicalExpressionValue::Type(_)
        | CanonicalExpressionValue::Operator(_)
        | CanonicalExpressionValue::UnaryOperator(_)
        | CanonicalExpressionValue::Function(_) => {}
    }
}

fn validate_attached_expression_collations(
    index_partition_snapshot: &IndexPartitionSnapshot,
    observations: &[IndexExpressionCollationIdentityObservation],
) -> Result<(), ObservationError> {
    let by_location = observations
        .iter()
        .map(|observation| {
            (
                observation.location().canonical_location(),
                observation.collation(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for membership in index_partition_snapshot
        .observations()
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent_index) = membership.parent_index() else {
            continue;
        };
        for child in observations
            .iter()
            .filter(|observation| observation.location().index() == membership.coordinate())
        {
            let parent_location = child.location().with_index(parent_index.clone());
            let parent = by_location
                .get(&parent_location.canonical_location())
                .ok_or_else(|| invalid("index_expression_collation_catalog_completeness"))?;
            if *parent != child.collation() {
                return Err(invalid("index_expression_collation_catalog_identity"));
            }
        }
    }
    Ok(())
}

fn validate_attached_relation_var_collations(
    index_partition_snapshot: &IndexPartitionSnapshot,
    observations: &[IndexRelationVarCollationIdentityObservation],
) -> Result<(), ObservationError> {
    let by_location = observations
        .iter()
        .map(|observation| {
            (
                observation.location().canonical_location(),
                observation.collation(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for membership in index_partition_snapshot
        .observations()
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent_index) = membership.parent_index() else {
            continue;
        };
        for child in observations
            .iter()
            .filter(|observation| observation.location().index() == membership.coordinate())
        {
            let parent_location =
                relation_var_location_with_index(child.location(), parent_index.clone());
            let parent = by_location
                .get(&parent_location.canonical_location())
                .ok_or_else(|| invalid("index_relation_var_collation_catalog_completeness"))?;
            if *parent != child.collation() {
                return Err(invalid("index_relation_var_collation_catalog_identity"));
            }
        }
    }
    Ok(())
}

fn relation_var_location_with_index(
    location: &IndexExpressionRelationVarLocation,
    index: IndexPartitionCoordinate,
) -> IndexExpressionRelationVarLocation {
    match location {
        IndexExpressionRelationVarLocation::Expression {
            key_position,
            leaf_position,
            ..
        } => IndexExpressionRelationVarLocation::expression(index, *key_position, *leaf_position)
            .expect("an existing relation-Var location has positive positions"),
        IndexExpressionRelationVarLocation::Predicate { leaf_position, .. } => {
            IndexExpressionRelationVarLocation::predicate(index, *leaf_position)
                .expect("an existing relation-Var location has a positive position")
        }
    }
}

fn compute_digest(
    whole_tree_predecessor_digest: &str,
    key_collation_predecessor_digest: &str,
    expression_observations: &[IndexExpressionCollationIdentityObservation],
    relation_var_observations: &[IndexRelationVarCollationIdentityObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(EXPRESSION_COLLATION_IDENTITY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, whole_tree_predecessor_digest);
    encode_str(&mut hasher, key_collation_predecessor_digest);
    encode_len(&mut hasher, expression_observations.len());
    for observation in expression_observations {
        encode_str(&mut hasher, &observation.location().canonical_location());
        encode_catalog_identity(&mut hasher, observation.collation());
    }
    encode_len(&mut hasher, relation_var_observations.len());
    for observation in relation_var_observations {
        encode_str(&mut hasher, &observation.location().canonical_location());
        match observation.collation() {
            Some(collation) => {
                hasher.update([1]);
                encode_catalog_identity(&mut hasher, collation);
            }
            None => hasher.update([0]),
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_catalog_identity(hasher: &mut Sha256, collation: &CollationCatalogIdentity) {
    encode_str(hasher, collation.schema_name());
    encode_str(hasher, collation.collation_name());
    hasher.update(collation.encoding().to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let len = u64::try_from(value.len()).expect("semantic string length fits in u64");
    hasher.update(len.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize fits in u64");
    hasher.update(value.to_be_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
