//! PostgreSQL key-constraint parentage layered over exact index-partition evidence.
//!
//! `pg_constraint.conparentid` is independent catalog evidence from `pg_inherits` index parentage.
//! PostgreSQL sets it when a constraint-backed child index is attached below a constraint-backed
//! parent index. This successor preserves that resolved constraint edge without changing any frozen
//! predecessor digest.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, RelationKind, TableConstraintObservation,
};
use sha2::{Digest, Sha256};

use super::{IndexPartitionCoordinate, IndexPartitionSnapshot};
use crate::RelationPartitionSnapshot;

const INDEX_CONSTRAINT_PARENTAGE_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.constraint_parentage.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact resolved coordinate of one PostgreSQL primary-key or unique constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintParentageCoordinate {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
}

impl Ord for IndexConstraintParentageCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.schema_name.as_str(),
            self.relation_name.as_str(),
            self.relation_kind.token(),
            self.constraint_name.as_str(),
        )
            .cmp(&(
                other.schema_name.as_str(),
                other.relation_name.as_str(),
                other.relation_kind.token(),
                other.constraint_name.as_str(),
            ))
    }
}

impl PartialOrd for IndexConstraintParentageCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IndexConstraintParentageCoordinate {
    /// Creates one exact relation-scoped key-constraint coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        validate_nonblank(&schema_name, "index_constraint_parentage_schema_name")?;
        validate_nonblank(&relation_name, "index_constraint_parentage_relation_name")?;
        validate_nonblank(
            &constraint_name,
            "index_constraint_parentage_constraint_name",
        )?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact owning relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact owning relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns a collision-safe evidence location for this constraint-parentage fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/relations/{}/{}/constraints/{}/parentage",
            escape_json_pointer_token(&self.schema_name),
            self.relation_kind.token(),
            escape_json_pointer_token(&self.relation_name),
            escape_json_pointer_token(&self.constraint_name)
        )
    }
}

/// One exact resolved `pg_constraint.conparentid` observation for a key constraint.
///
/// [`Self::root`] records `conparentid = 0`. [`Self::partition`] records the exact parent constraint
/// reached by a nonzero `conparentid` after the adapter resolves catalog OIDs to stable coordinates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintParentageObservation {
    coordinate: IndexConstraintParentageCoordinate,
    parent_constraint: Option<IndexConstraintParentageCoordinate>,
}

impl IndexConstraintParentageObservation {
    /// Records a key constraint with no parent constraint (`conparentid = 0`).
    pub fn root(
        coordinate: IndexConstraintParentageCoordinate,
    ) -> Result<Self, ObservationError> {
        Self::new(coordinate, None)
    }

    /// Records a key constraint with one exact resolved parent constraint.
    pub fn partition(
        coordinate: IndexConstraintParentageCoordinate,
        parent_constraint: IndexConstraintParentageCoordinate,
    ) -> Result<Self, ObservationError> {
        Self::new(coordinate, Some(parent_constraint))
    }

    fn new(
        coordinate: IndexConstraintParentageCoordinate,
        parent_constraint: Option<IndexConstraintParentageCoordinate>,
    ) -> Result<Self, ObservationError> {
        if parent_constraint
            .as_ref()
            .is_some_and(|parent| parent == &coordinate)
        {
            return Err(invalid("index_constraint_parentage_parent"));
        }
        Ok(Self {
            coordinate,
            parent_constraint,
        })
    }

    /// Returns the exact child/source constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexConstraintParentageCoordinate {
        &self.coordinate
    }

    /// Returns the exact resolved parent constraint, or `None` for `conparentid = 0`.
    #[must_use]
    pub const fn parent_constraint(&self) -> Option<&IndexConstraintParentageCoordinate> {
        self.parent_constraint.as_ref()
    }
}

/// Immutable provenance receipt for one exact key-constraint parentage observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintParentageSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexConstraintParentageObservation,
}

impl IndexConstraintParentageSourceReceipt {
    /// Returns the stable source registry key, never credential material.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed parentage successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
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

    /// Returns the exact constraint-parentage observation.
    #[must_use]
    pub const fn location(&self) -> &IndexConstraintParentageObservation {
        &self.location
    }
}

/// Complete key-constraint parentage layered over one exact index-partition snapshot.
///
/// The family is complete over every observed PostgreSQL `PRIMARY KEY` and `UNIQUE` constraint.
/// For a key constraint whose same-name backing index is attached below a constraint-backed parent
/// index, the resolved `conparentid` must identify that exact parent key constraint. A constraint-
/// backed child index below a non-constraint parent index remains a local constraint with
/// `conparentid = 0`, matching PostgreSQL detach/attach semantics. The successor hashes these facts
/// under a new domain separator and does not mutate v3, relation-partition, or index-partition
/// identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintParentageSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexConstraintParentageObservation>,
}

impl IndexConstraintParentageSnapshot {
    /// Creates complete key-constraint parentage over an exact predecessor chain.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        observations: Vec<IndexConstraintParentageObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexPartitionSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != index_partition_snapshot.snapshot_digest() {
            return Err(invalid("index_constraint_parentage_predecessor_binding"));
        }

        let observations = canonicalize_parentage(
            base_snapshot,
            index_partition_snapshot,
            observations,
        )?;
        let snapshot_digest = compute_parentage_digest(
            index_partition_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: index_partition_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: index_partition_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: index_partition_snapshot.extractor_revision().to_owned(),
            observed_at_utc: index_partition_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated digest.
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

    /// Returns complete key-constraint parentage in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexConstraintParentageObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed key-constraint parentage coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexConstraintParentageCoordinate,
    ) -> Result<IndexConstraintParentageSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: coordinate.canonical_location(),
            })?;
        Ok(IndexConstraintParentageSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn canonicalize_parentage(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index_partition_snapshot: &IndexPartitionSnapshot,
    mut observations: Vec<IndexConstraintParentageObservation>,
) -> Result<Vec<IndexConstraintParentageObservation>, ObservationError> {
    observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

    let expected = base_snapshot
        .relations()
        .iter()
        .flat_map(|relation| {
            relation
                .constraints()
                .iter()
                .filter(|constraint| is_key_constraint(constraint))
                .map(move |constraint| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    let observed = observations
        .iter()
        .map(|observation| coordinate_key(observation.coordinate()))
        .collect::<BTreeSet<_>>();

    if observed.len() != observations.len() {
        return Err(invalid("index_constraint_parentage_coordinate"));
    }
    if observed != expected {
        return Err(invalid("index_constraint_parentage_completeness"));
    }

    for observation in &observations {
        let expected_parent = expected_parent_constraint(
            base_snapshot,
            index_partition_snapshot,
            observation.coordinate(),
        )?;
        if observation.parent_constraint() != expected_parent.as_ref() {
            return Err(invalid("index_constraint_parentage"));
        }
    }

    Ok(observations)
}

fn expected_parent_constraint(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index_partition_snapshot: &IndexPartitionSnapshot,
    coordinate: &IndexConstraintParentageCoordinate,
) -> Result<Option<IndexConstraintParentageCoordinate>, ObservationError> {
    let child_index = IndexPartitionCoordinate::new(
        coordinate.schema_name(),
        coordinate.relation_name(),
        coordinate.relation_kind(),
        coordinate.constraint_name(),
    )?;
    let membership = index_partition_snapshot
        .observations()
        .iter()
        .find(|membership| membership.coordinate() == &child_index)
        .ok_or_else(|| invalid("index_constraint_parentage_backing_index"))?;
    let Some(parent_index) = membership.parent_index() else {
        return Ok(None);
    };

    let parent_relation = base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == parent_index.schema_name()
                && relation.relation_name() == parent_index.relation_name()
                && relation.kind() == parent_index.relation_kind()
        })
        .ok_or_else(|| invalid("index_constraint_parentage_parent_coordinate"))?;
    let parent_is_key_constraint = parent_relation.constraints().iter().any(|constraint| {
        is_key_constraint(constraint) && constraint.constraint_name() == parent_index.index_name()
    });
    if !parent_is_key_constraint {
        return Ok(None);
    }

    Ok(Some(IndexConstraintParentageCoordinate::new(
        parent_index.schema_name(),
        parent_index.relation_name(),
        parent_index.relation_kind(),
        parent_index.index_name(),
    )?))
}

fn is_key_constraint(constraint: &TableConstraintObservation) -> bool {
    matches!(
        constraint,
        TableConstraintObservation::PrimaryKey(_) | TableConstraintObservation::Unique(_)
    )
}

fn coordinate_key(
    coordinate: &IndexConstraintParentageCoordinate,
) -> (String, String, String, String) {
    (
        coordinate.schema_name().to_owned(),
        coordinate.relation_name().to_owned(),
        coordinate.relation_kind().token().to_owned(),
        coordinate.constraint_name().to_owned(),
    )
}

fn compute_parentage_digest(
    predecessor_digest: &str,
    observations: &[IndexConstraintParentageObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_CONSTRAINT_PARENTAGE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        match observation.parent_constraint() {
            None => hasher.update([0]),
            Some(parent) => {
                hasher.update([1]);
                encode_coordinate(&mut hasher, parent);
            }
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexConstraintParentageCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
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

fn escape_json_pointer_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
