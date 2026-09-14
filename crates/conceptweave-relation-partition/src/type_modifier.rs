//! Structured PostgreSQL `pg_attribute.atttypmod` evidence for declarative partition rowtypes.
//!
//! The frozen v3 observation keeps adapter-rendered data-type text, while PostgreSQL rowtype
//! mapping compares the raw `atttypmod` catalog value. This domain-separated successor preserves
//! that exact signed `int4` evidence without changing the meaning of predecessor digests.

use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3, RelationKind};
use sha2::{Digest, Sha256};

use crate::{RelationPartitionObservation, RelationPartitionSnapshot};

const TYPE_MODIFIER_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.atttypmod.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact raw PostgreSQL `pg_attribute.atttypmod` observation for one bounded column.
///
/// The value is retained exactly as the signed catalog `int4`, including `-1`. ConceptWeave does
/// not decode it into datatype-specific semantics because that interpretation belongs to the
/// owning PostgreSQL type implementation and is unnecessary for exact partition rowtype equality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnTypeModifierObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
    type_modifier: i32,
}

impl ColumnTypeModifierObservation {
    /// Creates one exact raw column type-modifier observation.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        type_modifier: i32,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_nonblank(&schema_name, "relation_partition_type_modifier_schema_name")?;
        validate_nonblank(
            &relation_name,
            "relation_partition_type_modifier_relation_name",
        )?;
        validate_nonblank(
            &column_name,
            "relation_partition_type_modifier_column_name",
        )?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            type_modifier,
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

    /// Returns the exact source relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns the raw signed PostgreSQL `pg_attribute.atttypmod` value.
    #[must_use]
    pub const fn type_modifier(&self) -> i32 {
        self.type_modifier
    }
}

/// Exact receipt coordinate for one structured column type-modifier observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnTypeModifierLocation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
}

impl ColumnTypeModifierLocation {
    /// Creates one exact column type-modifier evidence coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_nonblank(
            &schema_name,
            "relation_partition_type_modifier_location_schema_name",
        )?;
        validate_nonblank(
            &relation_name,
            "relation_partition_type_modifier_location_relation_name",
        )?;
        validate_nonblank(
            &column_name,
            "relation_partition_type_modifier_location_column_name",
        )?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            column_name,
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

    /// Returns the exact source relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns the collision-safe canonical evidence location.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/relations/{}/{}/columns/{}/type-modifier",
            escape_json_pointer_token(&self.schema_name),
            self.relation_kind.token(),
            escape_json_pointer_token(&self.relation_name),
            escape_json_pointer_token(&self.column_name)
        )
    }
}

/// Immutable receipt binding one exact type-modifier coordinate to successor provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnTypeModifierSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: ColumnTypeModifierLocation,
}

impl ColumnTypeModifierSourceReceipt {
    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable source connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated structured type-modifier digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the bounded predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the bounded predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact verified column type-modifier coordinate.
    #[must_use]
    pub const fn location(&self) -> &ColumnTypeModifierLocation {
        &self.location
    }
}

/// Complete structured `atttypmod` evidence layered over one exact relation-partition snapshot.
///
/// Every bounded column receives one raw modifier observation. The successor first rebound-validates
/// that the supplied relation-partition snapshot is the exact digest produced from the supplied v3
/// predecessor, then requires direct declarative parent/child columns matched by name to have equal
/// raw `atttypmod`. This adds authoritative modifier evidence without rewriting frozen v3 or the
/// previously issued relation-partition digest family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPartitionTypeModifierSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<ColumnTypeModifierObservation>,
}

impl RelationPartitionTypeModifierSnapshot {
    /// Creates complete structured column type-modifier evidence over an exact predecessor stack.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        observations: Vec<ColumnTypeModifierObservation>,
    ) -> Result<Self, ObservationError> {
        validate_predecessor(base_snapshot, relation_partition_snapshot)?;
        let observations = canonicalize_type_modifiers(base_snapshot, observations)?;
        validate_partition_type_modifiers(relation_partition_snapshot.observations(), &observations)?;
        let snapshot_digest = compute_type_modifier_digest(
            relation_partition_snapshot.snapshot_digest(),
            &observations,
        );

        Ok(Self {
            source_connection_key: relation_partition_snapshot.source_connection_key().to_owned(),
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

    /// Returns the immutable source connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated structured type-modifier digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the bounded predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the bounded predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete structured type-modifier observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[ColumnTypeModifierObservation] {
        &self.observations
    }

    /// Issues provenance for one exact column type-modifier coordinate when it exists.
    pub fn source_receipt(
        &self,
        location: ColumnTypeModifierLocation,
    ) -> Result<ColumnTypeModifierSourceReceipt, ObservationError> {
        let exists = self.observations.iter().any(|observation| {
            observation.schema_name() == location.schema_name()
                && observation.relation_name() == location.relation_name()
                && observation.relation_kind() == location.relation_kind()
                && observation.column_name() == location.column_name()
        });
        if !exists {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }

        Ok(ColumnTypeModifierSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }
}

fn validate_predecessor(
    base_snapshot: &PostgresSchemaSnapshotV3,
    relation_partition_snapshot: &RelationPartitionSnapshot,
) -> Result<(), ObservationError> {
    if base_snapshot.source_connection_key() != relation_partition_snapshot.source_connection_key()
        || base_snapshot.connection_policy_binding()
            != relation_partition_snapshot.connection_policy_binding()
        || base_snapshot.extractor_revision() != relation_partition_snapshot.extractor_revision()
        || base_snapshot.observed_at_utc() != relation_partition_snapshot.observed_at_utc()
    {
        return Err(invalid("relation_partition_type_modifier_predecessor"));
    }

    let rebound = RelationPartitionSnapshot::new(
        base_snapshot,
        relation_partition_snapshot.observations().to_vec(),
    )?;
    if rebound.snapshot_digest() != relation_partition_snapshot.snapshot_digest() {
        return Err(invalid("relation_partition_type_modifier_predecessor"));
    }
    Ok(())
}

fn canonicalize_type_modifiers(
    base_snapshot: &PostgresSchemaSnapshotV3,
    mut observations: Vec<ColumnTypeModifierObservation>,
) -> Result<Vec<ColumnTypeModifierObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.column_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.column_name(),
            ))
    });

    let expected_coordinates = base_snapshot
        .relations()
        .iter()
        .flat_map(|relation| {
            relation.columns().iter().map(move |column| {
                (
                    relation.schema_name().to_owned(),
                    relation.relation_name().to_owned(),
                    relation.kind().token().to_owned(),
                    column.column_name().to_owned(),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let observed_coordinates = observations
        .iter()
        .map(|observation| {
            (
                observation.schema_name().to_owned(),
                observation.relation_name().to_owned(),
                observation.relation_kind().token().to_owned(),
                observation.column_name().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();

    if observed_coordinates.len() != observations.len() {
        return Err(invalid("relation_partition_column_type_modifier_coordinate"));
    }
    if observed_coordinates != expected_coordinates {
        return Err(invalid(
            "relation_partition_column_type_modifier_completeness",
        ));
    }
    Ok(observations)
}

fn validate_partition_type_modifiers(
    memberships: &[RelationPartitionObservation],
    observations: &[ColumnTypeModifierObservation],
) -> Result<(), ObservationError> {
    let by_coordinate = observations
        .iter()
        .map(|observation| {
            (
                (
                    observation.schema_name().to_owned(),
                    observation.relation_name().to_owned(),
                    observation.relation_kind().token().to_owned(),
                    observation.column_name().to_owned(),
                ),
                observation.type_modifier(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for membership in memberships.iter().filter(|membership| membership.is_partition()) {
        let Some(parent) = membership.parent_relation() else {
            continue;
        };

        for child in observations.iter().filter(|observation| {
            observation.schema_name() == membership.schema_name()
                && observation.relation_name() == membership.relation_name()
                && observation.relation_kind() == membership.relation_kind()
        }) {
            let parent_key = (
                parent.schema_name().to_owned(),
                parent.relation_name().to_owned(),
                RelationKind::PartitionedTable.token().to_owned(),
                child.column_name().to_owned(),
            );
            let parent_type_modifier = by_coordinate.get(&parent_key).ok_or_else(|| {
                invalid("relation_partition_column_type_modifier_completeness")
            })?;
            if *parent_type_modifier != child.type_modifier() {
                return Err(invalid("relation_partition_column_type_modifier"));
            }
        }
    }
    Ok(())
}

fn compute_type_modifier_digest(
    predecessor_digest: &str,
    observations: &[ColumnTypeModifierObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(TYPE_MODIFIER_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.column_name());
        hasher.update(observation.type_modifier().to_be_bytes());
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
