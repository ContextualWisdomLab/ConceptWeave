//! Immutable PostgreSQL index-partition topology layered over relation-partition evidence.
//!
//! PostgreSQL stores index relations in `pg_class`: ordinary indexes use `relkind = 'i'`,
//! partitioned indexes use `relkind = 'I'`, and `relispartition` is material for indexes as well as
//! tables. Direct index-parent relationships and detach state are recorded in `pg_inherits`. This
//! module preserves those facts without changing the predecessor relation-partition digest.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, RelationKind,
};
use sha2::{Digest, Sha256};

use crate::{RelationPartitionObservation, RelationPartitionSnapshot};

const INDEX_PARTITION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// PostgreSQL `pg_class.relkind` for one observed index relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexRelationKind {
    /// Ordinary index (`pg_class.relkind = 'i'`).
    Index,
    /// Partitioned index (`pg_class.relkind = 'I'`).
    PartitionedIndex,
}

impl IndexRelationKind {
    fn token(self) -> &'static str {
        match self {
            Self::Index => "index",
            Self::PartitionedIndex => "partitioned_index",
        }
    }
}

/// Exact source coordinate of one index nested under its owning relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPartitionCoordinate {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    index_name: String,
}

impl Ord for IndexPartitionCoordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.schema_name.as_str(),
            self.relation_name.as_str(),
            self.relation_kind.token(),
            self.index_name.as_str(),
        )
            .cmp(&(
                other.schema_name.as_str(),
                other.relation_name.as_str(),
                other.relation_kind.token(),
                other.index_name.as_str(),
            ))
    }
}

impl PartialOrd for IndexPartitionCoordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IndexPartitionCoordinate {
    /// Creates an exact relation-scoped index coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        index_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let index_name = index_name.into();
        validate_nonblank(&schema_name, "index_partition_schema_name")?;
        validate_nonblank(&relation_name, "index_partition_relation_name")?;
        validate_nonblank(&index_name, "index_partition_index_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            index_name,
        })
    }

    /// Returns the exact owning schema name.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact owning relation name.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact owning relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact index name.
    #[must_use]
    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    /// Returns a collision-safe evidence location for this index-partition fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/relations/{}/{}/indexes/{}/partition-membership",
            escape_json_pointer_token(&self.schema_name),
            self.relation_kind.token(),
            escape_json_pointer_token(&self.relation_name),
            escape_json_pointer_token(&self.index_name)
        )
    }
}

/// One exact index `relkind`/`relispartition` observation plus direct index parent when present.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPartitionObservation {
    coordinate: IndexPartitionCoordinate,
    index_relation_kind: IndexRelationKind,
    is_partition: bool,
    parent_index: Option<IndexPartitionCoordinate>,
    detach_pending: bool,
}

impl IndexPartitionObservation {
    /// Records one index that is not itself attached as an index partition.
    pub fn non_partition(
        coordinate: IndexPartitionCoordinate,
        index_relation_kind: IndexRelationKind,
    ) -> Result<Self, ObservationError> {
        Self::new(coordinate, index_relation_kind, false, None, false)
    }

    /// Records one direct index-partition edge from `pg_inherits`.
    pub fn partition(
        coordinate: IndexPartitionCoordinate,
        index_relation_kind: IndexRelationKind,
        parent_index: IndexPartitionCoordinate,
        detach_pending: bool,
    ) -> Result<Self, ObservationError> {
        Self::new(
            coordinate,
            index_relation_kind,
            true,
            Some(parent_index),
            detach_pending,
        )
    }

    fn new(
        coordinate: IndexPartitionCoordinate,
        index_relation_kind: IndexRelationKind,
        is_partition: bool,
        parent_index: Option<IndexPartitionCoordinate>,
        detach_pending: bool,
    ) -> Result<Self, ObservationError> {
        if detach_pending {
            return Err(invalid("index_partition_detach_pending"));
        }
        if is_partition != parent_index.is_some() {
            return Err(invalid("index_partition_parent_presence"));
        }
        if parent_index
            .as_ref()
            .is_some_and(|parent| parent == &coordinate)
        {
            return Err(invalid("index_partition_parent"));
        }
        Ok(Self {
            coordinate,
            index_relation_kind,
            is_partition,
            parent_index,
            detach_pending,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexPartitionCoordinate {
        &self.coordinate
    }

    /// Returns the exact observed index `pg_class.relkind`.
    #[must_use]
    pub const fn index_relation_kind(&self) -> IndexRelationKind {
        self.index_relation_kind
    }

    /// Returns observed `pg_class.relispartition` for this index.
    #[must_use]
    pub const fn is_partition(&self) -> bool {
        self.is_partition
    }

    /// Returns the exact resolved direct parent index coordinate when attached.
    #[must_use]
    pub const fn parent_index(&self) -> Option<&IndexPartitionCoordinate> {
        self.parent_index.as_ref()
    }

    /// Returns observed `pg_inherits.inhdetachpending`.
    ///
    /// Immutable admitted observations always return `false`; a transitional true value fails at
    /// construction so detach state cannot silently become stable governed evidence.
    #[must_use]
    pub const fn detach_pending(&self) -> bool {
        self.detach_pending
    }
}

/// Immutable receipt for one exact index-partition evidence coordinate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPartitionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexPartitionCoordinate,
}

impl IndexPartitionSourceReceipt {
    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the governed index-partition source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact verified index coordinate.
    #[must_use]
    pub const fn location(&self) -> &IndexPartitionCoordinate {
        &self.location
    }
}

/// Complete index-partition topology layered over one exact relation-partition snapshot.
///
/// The family is complete over every index in the bounded v3 snapshot. It preserves index versus
/// partitioned-index `relkind`, index `relispartition`, exact direct index parent, and detach state.
/// Every attached index must belong to an attached relation and its parent index must be owned by the
/// same direct parent relation. A valid partitioned index must cover every direct local table
/// partition with an attached valid child index; PostgreSQL skips foreign-table partitions for
/// regular indexes and rejects valid unique partitioned indexes over such a foreign child. Attached
/// indexes also preserve PostgreSQL's constraint-parent rule: if the parent index backs an observed
/// key constraint, the child index must back its own observed key constraint before attachment. This
/// keeps table and index inheritance graphs coherent without changing the frozen v3 or
/// relation-partition predecessor identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPartitionSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexPartitionObservation>,
}

impl IndexPartitionSnapshot {
    /// Creates complete index-partition evidence over an exact v3/relation-partition pair.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        observations: Vec<IndexPartitionObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = RelationPartitionSnapshot::new(
            base_snapshot,
            relation_partition_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != relation_partition_snapshot.snapshot_digest() {
            return Err(invalid("index_partition_relation_snapshot_binding"));
        }

        let observations = canonicalize_index_partitions(
            base_snapshot,
            relation_partition_snapshot.observations(),
            observations,
        )?;
        let snapshot_digest = compute_index_partition_digest(
            relation_partition_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: relation_partition_snapshot
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: relation_partition_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: relation_partition_snapshot.extractor_revision().to_owned(),
            observed_at_utc: relation_partition_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated index-partition digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete index-partition observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexPartitionObservation] {
        &self.observations
    }

    /// Issues provenance for one exact observed index-partition coordinate.
    pub fn source_receipt(
        &self,
        location: IndexPartitionCoordinate,
    ) -> Result<IndexPartitionSourceReceipt, ObservationError> {
        if !self
            .observations
            .iter()
            .any(|observation| observation.coordinate() == &location)
        {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(IndexPartitionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }
}

fn canonicalize_index_partitions(
    base_snapshot: &PostgresSchemaSnapshotV3,
    relation_partitions: &[RelationPartitionObservation],
    mut observations: Vec<IndexPartitionObservation>,
) -> Result<Vec<IndexPartitionObservation>, ObservationError> {
    observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

    let expected_coordinates = base_snapshot
        .relations()
        .iter()
        .flat_map(|relation| {
            relation.indexes().iter().map(move |index| {
                (
                    relation.schema_name().to_owned(),
                    relation.relation_name().to_owned(),
                    relation.kind().token().to_owned(),
                    index.index_name().to_owned(),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let observed_coordinates = observations
        .iter()
        .map(|observation| coordinate_key(observation.coordinate()))
        .collect::<BTreeSet<_>>();

    if observed_coordinates.len() != observations.len() {
        return Err(invalid("index_partition_coordinate"));
    }
    if observed_coordinates != expected_coordinates {
        return Err(invalid("index_partition_completeness"));
    }

    let by_coordinate = observations
        .iter()
        .map(|observation| (observation.coordinate().clone(), observation))
        .collect::<BTreeMap<_, _>>();

    for observation in &observations {
        let coordinate = observation.coordinate();
        let owner = base_snapshot
            .relations()
            .iter()
            .find(|relation| {
                relation.schema_name() == coordinate.schema_name()
                    && relation.relation_name() == coordinate.relation_name()
                    && relation.kind() == coordinate.relation_kind()
            })
            .ok_or_else(|| invalid("index_partition_owner_coordinate"))?;

        let expected_kind = if owner.kind() == RelationKind::PartitionedTable {
            IndexRelationKind::PartitionedIndex
        } else {
            IndexRelationKind::Index
        };
        if observation.index_relation_kind() != expected_kind {
            return Err(invalid("index_partition_relation_kind"));
        }

        let owner_membership = relation_partitions
            .iter()
            .find(|membership| {
                membership.schema_name() == coordinate.schema_name()
                    && membership.relation_name() == coordinate.relation_name()
                    && membership.relation_kind() == coordinate.relation_kind()
            })
            .ok_or_else(|| invalid("index_partition_owner_membership"))?;

        if let Some(parent_index) = observation.parent_index() {
            if !owner_membership.is_partition() {
                return Err(invalid("index_partition_owner_relation"));
            }
            let parent_relation = owner_membership
                .parent_relation()
                .ok_or_else(|| invalid("index_partition_owner_relation"))?;
            if parent_relation.schema_name() != parent_index.schema_name()
                || parent_relation.relation_name() != parent_index.relation_name()
                || parent_index.relation_kind() != RelationKind::PartitionedTable
            {
                return Err(invalid("index_partition_relation_parent"));
            }

            let parent_observation = by_coordinate
                .get(parent_index)
                .ok_or_else(|| invalid("index_partition_parent_coordinate"))?;
            if parent_observation.index_relation_kind() != IndexRelationKind::PartitionedIndex {
                return Err(invalid("index_partition_parent_kind"));
            }

            let child_definition = find_base_index(base_snapshot, coordinate)
                .ok_or_else(|| invalid("index_partition_owner_coordinate"))?;
            let parent_definition = find_base_index(base_snapshot, parent_index)
                .ok_or_else(|| invalid("index_partition_parent_coordinate"))?;
            if child_definition.is_unique() != parent_definition.is_unique() {
                return Err(invalid("index_partition_definition_uniqueness"));
            }
            if child_definition.nulls_not_distinct() != parent_definition.nulls_not_distinct() {
                return Err(invalid("index_partition_definition_nulls_not_distinct"));
            }
            if child_definition.access_method() != parent_definition.access_method() {
                return Err(invalid("index_partition_definition_access_method"));
            }
            validate_modeled_index_attribute_mapping(child_definition, parent_definition)?;
            validate_modeled_index_key_collations(child_definition, parent_definition)?;
            validate_attached_constraint_backing(base_snapshot, coordinate, parent_index)?;
        }
    }

    validate_valid_partitioned_index_children(base_snapshot, relation_partitions, &by_coordinate)?;
    validate_index_parent_graph(&by_coordinate)?;
    Ok(observations)
}

fn validate_modeled_index_attribute_mapping(
    child_definition: &IndexObservation,
    parent_definition: &IndexObservation,
) -> Result<(), ObservationError> {
    if child_definition.key_attributes().len() != parent_definition.key_attributes().len()
        || child_definition.include_attributes().len()
            != parent_definition.include_attributes().len()
    {
        return Err(invalid("index_partition_definition_attribute_mapping"));
    }

    for (child_attribute, parent_attribute) in child_definition
        .key_attributes()
        .iter()
        .chain(child_definition.include_attributes())
        .zip(
            parent_definition
                .key_attributes()
                .iter()
                .chain(parent_definition.include_attributes()),
        )
    {
        match (
            child_attribute.attribute_name(),
            parent_attribute.attribute_name(),
        ) {
            (Some(child_name), Some(parent_name)) if child_name == parent_name => {}
            (None, None) => {
                // Expression-tree equality requires canonical expression evidence beyond this repair.
            }
            _ => return Err(invalid("index_partition_definition_attribute_mapping")),
        }
    }

    Ok(())
}

fn validate_modeled_index_key_collations(
    child_definition: &IndexObservation,
    parent_definition: &IndexObservation,
) -> Result<(), ObservationError> {
    let child_semantics = child_definition
        .key_semantics()
        .ok_or_else(|| invalid("index_partition_definition_collation"))?;
    let parent_semantics = parent_definition
        .key_semantics()
        .ok_or_else(|| invalid("index_partition_definition_collation"))?;

    if child_semantics.len() != parent_semantics.len() {
        return Err(invalid("index_partition_definition_collation"));
    }
    for (child_key, parent_key) in child_semantics.iter().zip(parent_semantics) {
        if child_key.collation() != parent_key.collation() {
            return Err(invalid("index_partition_definition_collation"));
        }
    }
    Ok(())
}

fn validate_attached_constraint_backing(
    base_snapshot: &PostgresSchemaSnapshotV3,
    child_coordinate: &IndexPartitionCoordinate,
    parent_coordinate: &IndexPartitionCoordinate,
) -> Result<(), ObservationError> {
    if relation_has_key_constraint_for_index(base_snapshot, parent_coordinate)?
        && !relation_has_key_constraint_for_index(base_snapshot, child_coordinate)?
    {
        return Err(invalid("index_partition_child_constraint"));
    }
    Ok(())
}

fn relation_has_key_constraint_for_index(
    base_snapshot: &PostgresSchemaSnapshotV3,
    coordinate: &IndexPartitionCoordinate,
) -> Result<bool, ObservationError> {
    let relation = base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == coordinate.schema_name()
                && relation.relation_name() == coordinate.relation_name()
                && relation.kind() == coordinate.relation_kind()
        })
        .ok_or_else(|| invalid("index_partition_owner_coordinate"))?;

    Ok(relation.constraints().iter().any(|constraint| {
        matches!(
            constraint,
            conceptweave_observation::TableConstraintObservation::PrimaryKey(_)
                | conceptweave_observation::TableConstraintObservation::Unique(_)
        ) && constraint.constraint_name() == coordinate.index_name()
    }))
}

fn validate_valid_partitioned_index_children(
    base_snapshot: &PostgresSchemaSnapshotV3,
    relation_partitions: &[RelationPartitionObservation],
    by_coordinate: &BTreeMap<IndexPartitionCoordinate, &IndexPartitionObservation>,
) -> Result<(), ObservationError> {
    for (parent_coordinate, parent_observation) in by_coordinate {
        if parent_observation.index_relation_kind() != IndexRelationKind::PartitionedIndex {
            continue;
        }
        let parent_index = find_base_index(base_snapshot, parent_coordinate)
            .ok_or_else(|| invalid("index_partition_owner_coordinate"))?;
        if parent_index.valid() != Some(true) {
            continue;
        }

        for child_relation in relation_partitions.iter().filter(|membership| {
            membership.is_partition()
                && membership.parent_relation().is_some_and(|parent_relation| {
                    parent_relation.schema_name() == parent_coordinate.schema_name()
                        && parent_relation.relation_name() == parent_coordinate.relation_name()
                })
        }) {
            if child_relation.relation_kind() == RelationKind::ForeignTable {
                if parent_index.is_unique() {
                    return Err(invalid("index_partition_foreign_partition_unique"));
                }
                continue;
            }

            let Some(attached_child) = by_coordinate.values().find(|child_index| {
                child_index.coordinate().schema_name() == child_relation.schema_name()
                    && child_index.coordinate().relation_name() == child_relation.relation_name()
                    && child_index.coordinate().relation_kind() == child_relation.relation_kind()
                    && child_index
                        .parent_index()
                        .is_some_and(|parent| parent == parent_coordinate)
            }) else {
                return Err(invalid("index_partition_parent_validity"));
            };

            let child_index = find_base_index(base_snapshot, attached_child.coordinate())
                .ok_or_else(|| invalid("index_partition_owner_coordinate"))?;
            if child_index.valid() != Some(true) {
                return Err(invalid("index_partition_child_validity"));
            }
        }
    }
    Ok(())
}

fn find_base_index<'a>(
    base_snapshot: &'a PostgresSchemaSnapshotV3,
    coordinate: &IndexPartitionCoordinate,
) -> Option<&'a IndexObservation> {
    base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == coordinate.schema_name()
                && relation.relation_name() == coordinate.relation_name()
                && relation.kind() == coordinate.relation_kind()
        })
        .and_then(|relation| {
            relation
                .indexes()
                .iter()
                .find(|index| index.index_name() == coordinate.index_name())
        })
}

fn validate_index_parent_graph(
    by_coordinate: &BTreeMap<IndexPartitionCoordinate, &IndexPartitionObservation>,
) -> Result<(), ObservationError> {
    for root in by_coordinate.keys() {
        let mut path = BTreeSet::new();
        let mut current = root.clone();
        loop {
            if !path.insert(current.clone()) {
                return Err(invalid("index_partition_cycle"));
            }
            let Some(observation) = by_coordinate.get(&current) else {
                break;
            };
            let Some(parent) = observation.parent_index() else {
                break;
            };
            current = parent.clone();
        }
    }
    Ok(())
}

fn compute_index_partition_digest(
    predecessor_digest: &str,
    observations: &[IndexPartitionObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_PARTITION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        let coordinate = observation.coordinate();
        encode_str(&mut hasher, coordinate.schema_name());
        encode_str(&mut hasher, coordinate.relation_name());
        encode_str(&mut hasher, coordinate.relation_kind().token());
        encode_str(&mut hasher, coordinate.index_name());
        encode_str(&mut hasher, observation.index_relation_kind().token());
        encode_bool(&mut hasher, observation.is_partition());
        match observation.parent_index() {
            None => hasher.update([0]),
            Some(parent) => {
                hasher.update([1]);
                encode_str(&mut hasher, parent.schema_name());
                encode_str(&mut hasher, parent.relation_name());
                encode_str(&mut hasher, parent.relation_kind().token());
                encode_str(&mut hasher, parent.index_name());
            }
        }
        encode_bool(&mut hasher, observation.detach_pending());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn coordinate_key(coordinate: &IndexPartitionCoordinate) -> (String, String, String, String) {
    (
        coordinate.schema_name().to_owned(),
        coordinate.relation_name().to_owned(),
        coordinate.relation_kind().token().to_owned(),
        coordinate.index_name().to_owned(),
    )
}

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.is_empty() || value.contains('\0') {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}

fn escape_json_pointer_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}

fn encode_bool(hasher: &mut Sha256, value: bool) {
    hasher.update([u8::from(value)]);
}
