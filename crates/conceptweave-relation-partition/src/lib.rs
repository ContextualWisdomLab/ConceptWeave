//! Immutable PostgreSQL declarative-partition membership evidence for ConceptWeave.
//!
//! PostgreSQL stores relation kind and declarative-partition membership as separate catalog facts:
//! `pg_class.relkind` describes the relation kind, while `pg_class.relispartition` states whether the
//! relation is itself a partition. Direct parentage and detach transition state live in `pg_inherits`.
//! This crate binds those facts to an existing [`PostgresSchemaSnapshotV3`] without changing the
//! frozen meaning of that predecessor digest.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    ColumnGenerationObservation, ColumnIdentityObservation, ObservationError,
    PostgresSchemaSnapshotV3, RelationKind,
};
use sha2::{Digest, Sha256};

mod index_partition;
pub use index_partition::*;

const RELATION_PARTITION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact resolved direct parent coordinate for one declarative partition.
///
/// PostgreSQL exposes `pg_inherits.inhparent` as an OID. The adapter resolves that capture-time join
/// coordinate to exact schema/relation identifiers before entering this contract. Parent relation
/// kind is validated against the bounded predecessor snapshot rather than supplied redundantly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionParentRelationCoordinate {
    schema_name: String,
    relation_name: String,
}

impl PartitionParentRelationCoordinate {
    /// Creates one exact resolved declarative-partition parent coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        validate_nonblank(&schema_name, "relation_partition_parent_schema_name")?;
        validate_nonblank(&relation_name, "relation_partition_parent_relation_name")?;
        Ok(Self {
            schema_name,
            relation_name,
        })
    }

    /// Returns the exact source schema identifier of the direct parent.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation identifier of the direct parent.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }
}

/// One exact `pg_class.relispartition` observation plus its direct declarative parent when present.
///
/// Generic table inheritance is deliberately outside this value object. A [`Self::non_partition`]
/// observation means only `relispartition=false`; it does not assert that `pg_inherits` contains no
/// generic-inheritance edge. A positive observation carries exactly one direct declarative parent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPartitionObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    is_partition: bool,
    parent_relation: Option<PartitionParentRelationCoordinate>,
    detach_pending: bool,
}

impl RelationPartitionObservation {
    /// Records explicit `pg_class.relispartition=false` for one bounded relation.
    pub fn non_partition(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            false,
            None,
            false,
        )
    }

    /// Records explicit positive declarative-partition membership and its direct `pg_inherits` edge.
    ///
    /// `detach_pending` must come from `pg_inherits.inhdetachpending`; a pending detach is transitional
    /// topology and therefore fails closed rather than being normalized to stable attached state.
    pub fn partition(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        parent_relation: PartitionParentRelationCoordinate,
        detach_pending: bool,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            true,
            Some(parent_relation),
            detach_pending,
        )
    }

    fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        is_partition: bool,
        parent_relation: Option<PartitionParentRelationCoordinate>,
        detach_pending: bool,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        validate_nonblank(&schema_name, "relation_partition_schema_name")?;
        validate_nonblank(&relation_name, "relation_partition_relation_name")?;

        if is_partition
            && !matches!(
                relation_kind,
                RelationKind::Table | RelationKind::PartitionedTable | RelationKind::ForeignTable
            )
        {
            return Err(invalid("relation_partition_relation_kind"));
        }
        if detach_pending {
            return Err(invalid("relation_partition_detach_pending"));
        }
        if is_partition != parent_relation.is_some() {
            return Err(invalid("relation_partition_parent_presence"));
        }
        if parent_relation.as_ref().is_some_and(|parent| {
            parent.schema_name() == schema_name && parent.relation_name() == relation_name
        }) {
            return Err(invalid("relation_partition_parent"));
        }

        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            is_partition,
            parent_relation,
            detach_pending,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact observed `pg_class.relkind` already bound by the predecessor snapshot.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the explicit `pg_class.relispartition` value.
    #[must_use]
    pub const fn is_partition(&self) -> bool {
        self.is_partition
    }

    /// Returns the resolved direct partition parent when `relispartition=true`.
    #[must_use]
    pub const fn parent_relation(&self) -> Option<&PartitionParentRelationCoordinate> {
        self.parent_relation.as_ref()
    }

    /// Returns the observed `pg_inherits.inhdetachpending` value.
    ///
    /// Admitted immutable observations are always `false`; callers still supply the catalog bit so a
    /// transitional state cannot silently become stable evidence.
    #[must_use]
    pub const fn detach_pending(&self) -> bool {
        self.detach_pending
    }
}

/// Exact receipt coordinate for relation-partition membership evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPartitionLocation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
}

impl RelationPartitionLocation {
    /// Creates one exact relation-partition evidence coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        validate_nonblank(&schema_name, "relation_partition_location_schema_name")?;
        validate_nonblank(&relation_name, "relation_partition_location_relation_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the collision-safe canonical evidence location.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/relations/{}/{}/partition-membership",
            escape_json_pointer_token(&self.schema_name),
            self.relation_kind.token(),
            escape_json_pointer_token(&self.relation_name)
        )
    }
}

/// Immutable receipt binding exact partition-membership evidence to governed digest and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPartitionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: RelationPartitionLocation,
}

impl RelationPartitionSourceReceipt {
    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated relation-partition source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor snapshot.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact verified relation-partition evidence coordinate.
    #[must_use]
    pub const fn location(&self) -> &RelationPartitionLocation {
        &self.location
    }
}

/// Complete immutable declarative-partition evidence layered over one exact v3 observation snapshot.
///
/// Every bounded relation must receive one explicit `relispartition` observation, making observed
/// false distinct from unobserved family absence. Positive membership resolves to an observed
/// partitioned-table parent, requires a valid PostgreSQL name/type rowtype map, preserves observed
/// column collation, identity, and generated-column mode coherence across each direct partition edge,
/// detach-pending topology fails closed, the parent graph must be acyclic, and PostgreSQL 18 NOT NULL
/// evidence must agree bidirectionally with the same direct relation edge. The successor digest frames
/// the predecessor digest in a new domain, preserving the frozen v3 identity contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPartitionSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<RelationPartitionObservation>,
}

impl RelationPartitionSnapshot {
    /// Creates complete relation-partition evidence over one exact bounded predecessor snapshot.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        observations: Vec<RelationPartitionObservation>,
    ) -> Result<Self, ObservationError> {
        let observations = canonicalize_relation_partitions(base_snapshot, observations)?;
        let snapshot_digest =
            compute_relation_partition_digest(base_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: base_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: base_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: base_snapshot.extractor_revision().to_owned(),
            observed_at_utc: base_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated relation-partition digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the predecessor snapshot.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete partition-membership observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[RelationPartitionObservation] {
        &self.observations
    }

    /// Issues provenance for one exact relation-partition coordinate when it exists in this family.
    pub fn source_receipt(
        &self,
        location: RelationPartitionLocation,
    ) -> Result<RelationPartitionSourceReceipt, ObservationError> {
        let exists = self.observations.iter().any(|observation| {
            observation.schema_name() == location.schema_name()
                && observation.relation_name() == location.relation_name()
                && observation.relation_kind() == location.relation_kind()
        });
        if !exists {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(RelationPartitionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }
}

fn canonicalize_relation_partitions(
    base_snapshot: &PostgresSchemaSnapshotV3,
    mut observations: Vec<RelationPartitionObservation>,
) -> Result<Vec<RelationPartitionObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
            ))
    });

    let expected_coordinates = base_snapshot
        .relations()
        .iter()
        .map(|relation| {
            (
                relation.schema_name().to_owned(),
                relation.relation_name().to_owned(),
                relation.kind().token().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let observed_coordinates = observations
        .iter()
        .map(|observation| {
            (
                observation.schema_name().to_owned(),
                observation.relation_name().to_owned(),
                observation.relation_kind().token().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();

    if observed_coordinates.len() != observations.len() {
        return Err(invalid("relation_partition_coordinate"));
    }
    if observed_coordinates != expected_coordinates {
        return Err(invalid("relation_partition_completeness"));
    }

    for observation in &observations {
        if let Some(parent) = observation.parent_relation() {
            let Some(parent_relation) = base_snapshot.relations().iter().find(|relation| {
                relation.schema_name() == parent.schema_name()
                    && relation.relation_name() == parent.relation_name()
            }) else {
                return Err(invalid("relation_partition_parent_coordinate"));
            };
            if parent_relation.kind() != RelationKind::PartitionedTable {
                return Err(invalid("relation_partition_parent_kind"));
            }
        }
    }

    validate_partition_rowtypes(base_snapshot, &observations)?;
    validate_partition_column_collations(base_snapshot, &observations)?;
    validate_partition_column_declarations(base_snapshot, &observations)?;
    validate_parent_graph(&observations)?;
    validate_not_null_partition_witnesses(base_snapshot, &observations)?;
    Ok(observations)
}

fn validate_partition_rowtypes(
    base_snapshot: &PostgresSchemaSnapshotV3,
    observations: &[RelationPartitionObservation],
) -> Result<(), ObservationError> {
    for membership in observations
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent_coordinate) = membership.parent_relation() else {
            continue;
        };
        let child_relation = base_snapshot
            .relations()
            .iter()
            .find(|relation| {
                relation.schema_name() == membership.schema_name()
                    && relation.relation_name() == membership.relation_name()
                    && relation.kind() == membership.relation_kind()
            })
            .ok_or_else(|| invalid("relation_partition_child_coordinate"))?;
        let parent_relation = base_snapshot
            .relations()
            .iter()
            .find(|relation| {
                relation.schema_name() == parent_coordinate.schema_name()
                    && relation.relation_name() == parent_coordinate.relation_name()
                    && relation.kind() == RelationKind::PartitionedTable
            })
            .ok_or_else(|| invalid("relation_partition_parent_coordinate"))?;

        let parent_columns = parent_relation
            .columns()
            .iter()
            .map(|column| (column.column_name(), column))
            .collect::<BTreeMap<_, _>>();
        let child_columns = child_relation
            .columns()
            .iter()
            .map(|column| (column.column_name(), column))
            .collect::<BTreeMap<_, _>>();

        if parent_columns.keys().copied().collect::<BTreeSet<_>>()
            != child_columns.keys().copied().collect::<BTreeSet<_>>()
        {
            return Err(invalid("relation_partition_column_mapping"));
        }

        for (column_name, parent_column) in parent_columns {
            let child_column = child_columns
                .get(column_name)
                .expect("column-name sets were verified equal");
            if parent_column.type_binding() != child_column.type_binding() {
                return Err(invalid("relation_partition_column_type"));
            }
            if parent_column.data_type() != child_column.data_type() {
                // Frozen v3 does not yet expose pg_attribute.atttypmod structurally. Its exact
                // adapter-rendered data-type text is therefore the retained typmod witness until a
                // future successor can carry a typed modifier without rewriting v3 identity.
                return Err(invalid("relation_partition_column_type_modifier"));
            }
        }
    }
    Ok(())
}

fn validate_partition_column_collations(
    base_snapshot: &PostgresSchemaSnapshotV3,
    observations: &[RelationPartitionObservation],
) -> Result<(), ObservationError> {
    let Some(collations) = base_snapshot.column_collations() else {
        return Ok(());
    };

    for membership in observations
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent) = membership.parent_relation() else {
            continue;
        };
        for parent_collation in collations.iter().filter(|collation| {
            collation.schema_name() == parent.schema_name()
                && collation.relation_name() == parent.relation_name()
                && collation.relation_kind() == RelationKind::PartitionedTable
        }) {
            let child_collation = collations
                .iter()
                .find(|collation| {
                    collation.schema_name() == membership.schema_name()
                        && collation.relation_name() == membership.relation_name()
                        && collation.relation_kind() == membership.relation_kind()
                        && collation.column_name() == parent_collation.column_name()
                })
                .ok_or_else(|| invalid("relation_partition_column_collation"))?;

            if parent_collation.collation() != child_collation.collation()
                || parent_collation.deterministic() != child_collation.deterministic()
            {
                return Err(invalid("relation_partition_column_collation"));
            }
        }
    }

    Ok(())
}

fn validate_partition_column_declarations(
    base_snapshot: &PostgresSchemaSnapshotV3,
    observations: &[RelationPartitionObservation],
) -> Result<(), ObservationError> {
    if let Some(identities) = base_snapshot.column_identities() {
        for membership in observations
            .iter()
            .filter(|observation| observation.is_partition())
        {
            let Some(parent) = membership.parent_relation() else {
                continue;
            };
            for parent_identity in identities.iter().filter(|identity| {
                identity.schema_name() == parent.schema_name()
                    && identity.relation_name() == parent.relation_name()
                    && identity.relation_kind() == RelationKind::PartitionedTable
            }) {
                let child_identity = identities
                    .iter()
                    .find(|identity| {
                        identity.schema_name() == membership.schema_name()
                            && identity.relation_name() == membership.relation_name()
                            && identity.relation_kind() == membership.relation_kind()
                            && identity.column_name() == parent_identity.column_name()
                    })
                    .ok_or_else(|| invalid("relation_partition_column_identity"))?;
                if !same_identity_mode(parent_identity, child_identity) {
                    return Err(invalid("relation_partition_column_identity"));
                }
            }
        }
    }

    if let Some(generations) = base_snapshot.column_generations() {
        for membership in observations
            .iter()
            .filter(|observation| observation.is_partition())
        {
            let Some(parent) = membership.parent_relation() else {
                continue;
            };
            for parent_generation in generations.iter().filter(|generation| {
                generation.schema_name() == parent.schema_name()
                    && generation.relation_name() == parent.relation_name()
                    && generation.relation_kind() == RelationKind::PartitionedTable
            }) {
                let child_generation = generations
                    .iter()
                    .find(|generation| {
                        generation.schema_name() == membership.schema_name()
                            && generation.relation_name() == membership.relation_name()
                            && generation.relation_kind() == membership.relation_kind()
                            && generation.column_name() == parent_generation.column_name()
                    })
                    .ok_or_else(|| invalid("relation_partition_column_generation"))?;
                if !same_generation_mode(parent_generation, child_generation) {
                    return Err(invalid("relation_partition_column_generation"));
                }
            }
        }
    }

    Ok(())
}

const fn same_identity_mode(
    left: &ColumnIdentityObservation,
    right: &ColumnIdentityObservation,
) -> bool {
    (left.is_not_identity() && right.is_not_identity())
        || (left.is_generated_always() && right.is_generated_always())
        || (left.is_generated_by_default() && right.is_generated_by_default())
}

const fn same_generation_mode(
    left: &ColumnGenerationObservation,
    right: &ColumnGenerationObservation,
) -> bool {
    (left.is_not_generated() && right.is_not_generated())
        || (left.is_stored() && right.is_stored())
        || (left.is_virtual_generated() && right.is_virtual_generated())
}

fn validate_parent_graph(
    observations: &[RelationPartitionObservation],
) -> Result<(), ObservationError> {
    let by_coordinate = observations
        .iter()
        .map(|observation| {
            (
                (
                    observation.schema_name().to_owned(),
                    observation.relation_name().to_owned(),
                    observation.relation_kind().token().to_owned(),
                ),
                observation,
            )
        })
        .collect::<BTreeMap<_, _>>();

    for root in by_coordinate.keys() {
        let mut path = BTreeSet::new();
        let mut current = root.clone();
        loop {
            if !path.insert(current.clone()) {
                return Err(invalid("relation_partition_cycle"));
            }
            let Some(observation) = by_coordinate.get(&current) else {
                break;
            };
            let Some(parent) = observation.parent_relation() else {
                break;
            };
            current = (
                parent.schema_name().to_owned(),
                parent.relation_name().to_owned(),
                RelationKind::PartitionedTable.token().to_owned(),
            );
        }
    }
    Ok(())
}

fn validate_not_null_partition_witnesses(
    base_snapshot: &PostgresSchemaSnapshotV3,
    observations: &[RelationPartitionObservation],
) -> Result<(), ObservationError> {
    let Some(not_null_constraints) = base_snapshot.not_null_constraints() else {
        return Ok(());
    };

    // First preserve the existing direction: every explicit conparentid witness must agree with the
    // independently observed direct declarative-partition relation edge.
    for not_null in not_null_constraints {
        let Some(parent_constraint) = not_null.parent_constraint() else {
            continue;
        };
        let membership = observations.iter().find(|observation| {
            observation.schema_name() == not_null.schema_name()
                && observation.relation_name() == not_null.relation_name()
                && observation.relation_kind() == not_null.relation_kind()
        });
        let agrees = membership
            .filter(|observation| observation.is_partition())
            .and_then(RelationPartitionObservation::parent_relation)
            .is_some_and(|parent| {
                parent.schema_name() == parent_constraint.schema_name()
                    && parent.relation_name() == parent_constraint.relation_name()
            });
        if !agrees {
            return Err(invalid("relation_partition_not_null_parent"));
        }
    }

    // PostgreSQL declarative partitions inherit every NOT NULL constraint from their direct
    // partitioned-table parent. Relation membership therefore also constrains the NOT NULL family:
    // each observed parent row must have the corresponding child row linked back to that exact
    // parent constraint. Child-local constraints that do not correspond to a parent row remain legal.
    for membership in observations
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent_relation) = membership.parent_relation() else {
            continue;
        };

        for parent_not_null in not_null_constraints.iter().filter(|constraint| {
            constraint.schema_name() == parent_relation.schema_name()
                && constraint.relation_name() == parent_relation.relation_name()
                && constraint.relation_kind() == RelationKind::PartitionedTable
        }) {
            let inherited = not_null_constraints.iter().any(|child_not_null| {
                child_not_null.schema_name() == membership.schema_name()
                    && child_not_null.relation_name() == membership.relation_name()
                    && child_not_null.relation_kind() == membership.relation_kind()
                    && child_not_null.column_name() == parent_not_null.column_name()
                    && child_not_null
                        .parent_constraint()
                        .is_some_and(|parent_constraint| {
                            parent_constraint.schema_name() == parent_not_null.schema_name()
                                && parent_constraint.relation_name()
                                    == parent_not_null.relation_name()
                                && parent_constraint.relation_kind()
                                    == parent_not_null.relation_kind()
                                && parent_constraint.constraint_name()
                                    == parent_not_null.constraint_name()
                        })
            });
            if !inherited {
                return Err(invalid("relation_partition_not_null_inheritance"));
            }
        }
    }

    Ok(())
}

fn compute_relation_partition_digest(
    predecessor_digest: &str,
    observations: &[RelationPartitionObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(RELATION_PARTITION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_bool(&mut hasher, observation.is_partition());
        match observation.parent_relation() {
            None => hasher.update([0]),
            Some(parent) => {
                hasher.update([1]);
                encode_str(&mut hasher, parent.schema_name());
                encode_str(&mut hasher, parent.relation_name());
            }
        }
        encode_bool(&mut hasher, observation.detach_pending());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
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
