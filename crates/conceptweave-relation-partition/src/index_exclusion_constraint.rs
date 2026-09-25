//! PostgreSQL exclusion-constraint identity and partition inheritance state.
//!
//! PostgreSQL 18 supports `EXCLUDE` constraints on partitioned tables. Their backing indexes carry
//! `pg_index.indisexclusion`, while the constraint itself remains independent `pg_constraint`
//! evidence through `conindid`, `conparentid`, `conislocal`, and `coninhcount`. Index exclusion
//! operators alone are therefore insufficient source identity. This successor retains the exact
//! constraint-to-index and constraint-to-parent edges without changing issued predecessor digests.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, RelationKind, TableConstraintObservation,
};
use sha2::{Digest, Sha256};

use super::{IndexPartitionCoordinate, IndexPartitionSnapshot};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact relation-scoped coordinate of one PostgreSQL `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintCoordinate {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
}

impl Ord for IndexExclusionConstraintCoordinate {
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
impl PartialOrd for IndexExclusionConstraintCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IndexExclusionConstraintCoordinate {
    /// Creates one exact exclusion-constraint coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        validate_nonblank(&schema_name, "index_exclusion_constraint_schema_name")?;
        validate_nonblank(&relation_name, "index_exclusion_constraint_relation_name")?;
        validate_nonblank(&constraint_name, "index_exclusion_constraint_name")?;
        if !matches!(
            relation_kind,
            RelationKind::Table | RelationKind::PartitionedTable
        ) {
            return Err(invalid("index_exclusion_constraint_relation_kind"));
        }
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
    /// Returns a collision-safe source location for this exclusion constraint.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/relations/{}/{}/constraints/{}/exclusion-partition-state",
            escape_json_pointer_token(&self.schema_name),
            self.relation_kind.token(),
            escape_json_pointer_token(&self.relation_name),
            escape_json_pointer_token(&self.constraint_name)
        )
    }
}

/// Exact `pg_constraint` state for one index-backed `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    parent_constraint: Option<IndexExclusionConstraintCoordinate>,
    is_local: bool,
    inheritance_count: i16,
}

impl IndexExclusionConstraintObservation {
    /// Records a root exclusion constraint and its exact resolved `conindid` backing index.
    pub fn root(
        coordinate: IndexExclusionConstraintCoordinate,
        backing_index: IndexPartitionCoordinate,
    ) -> Result<Self, ObservationError> {
        Self::new(coordinate, backing_index, None, true, 0)
    }

    /// Records an attached exclusion constraint with exact `conindid` and `conparentid` resolution.
    pub fn partition(
        coordinate: IndexExclusionConstraintCoordinate,
        backing_index: IndexPartitionCoordinate,
        parent_constraint: IndexExclusionConstraintCoordinate,
        is_local: bool,
        inheritance_count: i16,
    ) -> Result<Self, ObservationError> {
        Self::new(
            coordinate,
            backing_index,
            Some(parent_constraint),
            is_local,
            inheritance_count,
        )
    }

    fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        backing_index: IndexPartitionCoordinate,
        parent_constraint: Option<IndexExclusionConstraintCoordinate>,
        is_local: bool,
        inheritance_count: i16,
    ) -> Result<Self, ObservationError> {
        if inheritance_count < 0 {
            return Err(invalid("index_exclusion_constraint_inheritance_count"));
        }
        if coordinate.schema_name() != backing_index.schema_name()
            || coordinate.relation_name() != backing_index.relation_name()
            || coordinate.relation_kind() != backing_index.relation_kind()
        {
            return Err(invalid("index_exclusion_constraint_backing_index"));
        }
        if parent_constraint
            .as_ref()
            .is_some_and(|parent| parent == &coordinate)
        {
            return Err(invalid("index_exclusion_constraint_parent"));
        }
        Ok(Self {
            coordinate,
            backing_index,
            parent_constraint,
            is_local,
            inheritance_count,
        })
    }

    /// Returns the exact exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }
    /// Returns the exact resolved `pg_constraint.conindid` backing-index coordinate.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }
    /// Returns the exact resolved parent constraint, or `None` for `conparentid = 0`.
    #[must_use]
    pub const fn parent_constraint(&self) -> Option<&IndexExclusionConstraintCoordinate> {
        self.parent_constraint.as_ref()
    }
    /// Returns observed `pg_constraint.conislocal`.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }
    /// Returns observed `pg_constraint.coninhcount`.
    #[must_use]
    pub const fn inheritance_count(&self) -> i16 {
        self.inheritance_count
    }
}

/// Immutable provenance receipt for one exclusion-constraint catalog observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintObservation,
}
impl IndexExclusionConstraintSourceReceipt {
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
    /// Returns the owner-computed exclusion-constraint successor digest.
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
    /// Returns the exact exclusion-constraint catalog observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintObservation {
        &self.location
    }
}

/// Complete PostgreSQL `EXCLUDE` constraint catalog state over one exact index-partition snapshot.
///
/// Each observation is explicit adapter evidence for one `pg_constraint.contype = 'x'` row and
/// carries its resolved `conindid`; names are never inferred from index names. Temporal PRIMARY KEY
/// and UNIQUE `WITHOUT OVERLAPS` indexes can also report `indisexclusion`, so same-name key backing
/// indexes remain owned by the key-constraint successors and are excluded from this family. Every
/// remaining exclusion backing index must have exactly one explicit constraint observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintObservation>,
}

impl IndexExclusionConstraintSnapshot {
    /// Creates complete exclusion-constraint catalog evidence over an exact predecessor chain.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        mut observations: Vec<IndexExclusionConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexPartitionSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != index_partition_snapshot.snapshot_digest() {
            return Err(invalid("index_exclusion_constraint_predecessor_binding"));
        }

        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));
        let coordinate_set = observations
            .iter()
            .map(|o| o.coordinate().clone())
            .collect::<BTreeSet<_>>();
        if coordinate_set.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_coordinate"));
        }
        reject_relation_constraint_name_collisions(base_snapshot, &observations)?;

        let expected_backing = expected_exclusion_backing_indexes(base_snapshot)?;
        let observed_backing = observations
            .iter()
            .map(|o| o.backing_index().clone())
            .collect::<BTreeSet<_>>();
        if observed_backing.len() != observations.len() || observed_backing != expected_backing {
            return Err(invalid("index_exclusion_constraint_completeness"));
        }

        for observation in &observations {
            let expected_parent =
                expected_parent_constraint(index_partition_snapshot, &observations, observation)?;
            if observation.parent_constraint() != expected_parent.as_ref() {
                return Err(invalid("index_exclusion_constraint_parentage"));
            }
            let expected_state = if expected_parent.is_some() {
                (false, 1)
            } else {
                (true, 0)
            };
            if (observation.is_local(), observation.inheritance_count()) != expected_state {
                return Err(invalid("index_exclusion_constraint_inheritance_state"));
            }
        }

        let snapshot_digest =
            compute_digest(index_partition_snapshot.snapshot_digest(), &observations);
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
    /// Returns the domain-separated exclusion-constraint snapshot digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }
    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }
    /// Returns the exact canonical UTC observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
    /// Returns complete exclusion-constraint observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed exclusion constraint.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|o| o.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: coordinate.canonical_location(),
            })?;
        Ok(IndexExclusionConstraintSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn reject_relation_constraint_name_collisions(
    base_snapshot: &PostgresSchemaSnapshotV3,
    observations: &[IndexExclusionConstraintObservation],
) -> Result<(), ObservationError> {
    for observation in observations {
        let coordinate = observation.coordinate();
        let relation = base_snapshot
            .relations()
            .iter()
            .find(|relation| {
                relation.schema_name() == coordinate.schema_name()
                    && relation.relation_name() == coordinate.relation_name()
                    && relation.kind() == coordinate.relation_kind()
            })
            .ok_or_else(|| invalid("index_exclusion_constraint_relation"))?;
        let relation_constraint_collision = relation
            .constraints()
            .iter()
            .any(|constraint| constraint.constraint_name() == coordinate.constraint_name());
        let not_null_constraint_collision =
            base_snapshot
                .not_null_constraints()
                .is_some_and(|constraints| {
                    constraints.iter().any(|constraint| {
                        constraint.schema_name() == coordinate.schema_name()
                            && constraint.relation_name() == coordinate.relation_name()
                            && constraint.relation_kind() == coordinate.relation_kind()
                            && constraint.constraint_name() == coordinate.constraint_name()
                    })
                });
        if relation_constraint_collision || not_null_constraint_collision {
            return Err(ObservationError::DuplicateConstraintName {
                schema_name: coordinate.schema_name().to_owned(),
                table_name: coordinate.relation_name().to_owned(),
                constraint_name: coordinate.constraint_name().to_owned(),
            });
        }
    }
    Ok(())
}

fn expected_exclusion_backing_indexes(
    base_snapshot: &PostgresSchemaSnapshotV3,
) -> Result<BTreeSet<IndexPartitionCoordinate>, ObservationError> {
    let mut expected = BTreeSet::new();
    for relation in base_snapshot.relations() {
        for index in relation.indexes() {
            let flags = index
                .catalog_flags()
                .ok_or_else(|| invalid("index_exclusion_constraint_catalog_flags"))?;
            let temporal_key_backing = relation.constraints().iter().any(|constraint| {
                is_key_constraint(constraint) && constraint.constraint_name() == index.index_name()
            });
            if flags.exclusion() && !temporal_key_backing {
                expected.insert(IndexPartitionCoordinate::new(
                    relation.schema_name(),
                    relation.relation_name(),
                    relation.kind(),
                    index.index_name(),
                )?);
            }
        }
    }
    Ok(expected)
}

fn expected_parent_constraint(
    index_partition_snapshot: &IndexPartitionSnapshot,
    observations: &[IndexExclusionConstraintObservation],
    observation: &IndexExclusionConstraintObservation,
) -> Result<Option<IndexExclusionConstraintCoordinate>, ObservationError> {
    let membership = index_partition_snapshot
        .observations()
        .iter()
        .find(|candidate| candidate.coordinate() == observation.backing_index())
        .ok_or_else(|| invalid("index_exclusion_constraint_backing_index"))?;
    let Some(parent_index) = membership.parent_index() else {
        return Ok(None);
    };
    let parent = observations
        .iter()
        .find(|candidate| candidate.backing_index() == parent_index)
        .ok_or_else(|| invalid("index_exclusion_constraint_parent_index"))?;
    Ok(Some(parent.coordinate().clone()))
}

fn is_key_constraint(constraint: &TableConstraintObservation) -> bool {
    matches!(
        constraint,
        TableConstraintObservation::PrimaryKey(_) | TableConstraintObservation::Unique(_)
    )
}

fn compute_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        match observation.parent_constraint() {
            None => hasher.update([0]),
            Some(parent) => {
                hasher.update([1]);
                encode_constraint_coordinate(&mut hasher, parent);
            }
        }
        hasher.update([u8::from(observation.is_local())]);
        hasher.update(observation.inheritance_count().to_be_bytes());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_constraint_coordinate(
    hasher: &mut Sha256,
    coordinate: &IndexExclusionConstraintCoordinate,
) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
}
fn encode_index_coordinate(hasher: &mut Sha256, coordinate: &IndexPartitionCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.index_name());
}
fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}
fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}
fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}
fn escape_json_pointer_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
