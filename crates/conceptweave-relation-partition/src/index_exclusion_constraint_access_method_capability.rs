//! PostgreSQL ordinary exclusion-constraint access-method capability evidence.
//!
//! An ordinary `EXCLUDE` constraint is not valid merely because its backing index reports
//! `pg_index.indisexclusion=true`. PostgreSQL 18 rejects exclusion constraints when the selected
//! index access method does not expose tuple-at-a-time scans (`amgettuple`), surfaced to SQL as
//! `pg_indexam_has_property(am_oid, 'can_exclude')`. This successor retains that independently
//! observed capability and binds it to the exact `conindid` backing index and observed access-method
//! name instead of inferring capability from a built-in method allowlist.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot, IndexPartitionCoordinate,
    IndexPartitionSnapshot,
};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.access_method_capability.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact access-method exclusion capability observed for one ordinary `EXCLUDE` backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintAccessMethodCapabilityObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    access_method_name: String,
    can_exclude: bool,
}

impl IndexExclusionConstraintAccessMethodCapabilityObservation {
    /// Records independently resolved access-method capability for one exact `conindid` index.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        backing_index: IndexPartitionCoordinate,
        access_method_name: impl Into<String>,
        can_exclude: bool,
    ) -> Result<Self, ObservationError> {
        let access_method_name = access_method_name.into();
        validate_nonblank(
            &access_method_name,
            "index_exclusion_constraint_access_method_name",
        )?;
        Ok(Self {
            coordinate,
            backing_index,
            access_method_name,
            can_exclude,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact resolved `pg_constraint.conindid` backing-index coordinate.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }

    /// Returns the independently observed backing-index access-method name.
    #[must_use]
    pub fn access_method_name(&self) -> &str {
        &self.access_method_name
    }

    /// Returns `pg_indexam_has_property(am_oid, 'can_exclude')` for the observed access method.
    #[must_use]
    pub const fn can_exclude(&self) -> bool {
        self.can_exclude
    }

    /// Returns the collision-safe source location for this capability evidence.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        capability_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact exclusion access-method capability observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintAccessMethodCapabilitySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintAccessMethodCapabilityObservation,
}

impl IndexExclusionConstraintAccessMethodCapabilitySourceReceipt {
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

    /// Returns the owner-computed capability successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
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

    /// Returns the exact validated access-method capability observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintAccessMethodCapabilityObservation {
        &self.location
    }
}

/// Complete access-method exclusion-capability evidence over ordinary `EXCLUDE` constraints.
///
/// The exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE predecessor is rebound
/// before use. Every ordinary exclusion constraint then requires exactly one independent capability
/// observation for its exact `conindid` backing index. The observed access-method name must equal the
/// name already retained on that backing index and `can_exclude` must be true. Extension access
/// methods are admitted by observed capability rather than by a built-in name allowlist.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintAccessMethodCapabilitySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintAccessMethodCapabilityObservation>,
}

impl IndexExclusionConstraintAccessMethodCapabilitySnapshot {
    /// Creates complete access-method capability evidence over one exact ordinary EXCLUDE snapshot.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintAccessMethodCapabilityObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound_constraint = IndexExclusionConstraintSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot.observations().to_vec(),
        )?;
        if rebound_constraint.snapshot_digest() != constraint_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_access_method_capability_predecessor_binding",
            ));
        }

        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));
        let expected = rebound_constraint
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_access_method_capability_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_access_method_capability_completeness",
            ));
        }

        for observation in &observations {
            let constraint = rebound_constraint
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == observation.coordinate())
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_access_method_capability_completeness")
                })?;
            if constraint.backing_index() != observation.backing_index() {
                return Err(invalid(
                    "index_exclusion_constraint_access_method_capability_binding",
                ));
            }
            let index = find_base_index(base_snapshot, observation.backing_index()).ok_or_else(|| {
                invalid("index_exclusion_constraint_access_method_capability_binding")
            })?;
            let access_method = index.access_method().ok_or_else(|| {
                invalid("index_exclusion_constraint_access_method_capability_binding")
            })?;
            if access_method != observation.access_method_name() {
                return Err(invalid(
                    "index_exclusion_constraint_access_method_capability_binding",
                ));
            }
            if !observation.can_exclude() {
                return Err(invalid(
                    "index_exclusion_constraint_access_method_capability_state",
                ));
            }
        }

        let snapshot_digest = compute_capability_digest(
            rebound_constraint.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: rebound_constraint.source_connection_key().to_owned(),
            connection_policy_binding: rebound_constraint.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: rebound_constraint.extractor_revision().to_owned(),
            observed_at_utc: rebound_constraint.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated access-method capability successor digest.
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

    /// Returns complete capability observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintAccessMethodCapabilityObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary exclusion constraint capability.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintAccessMethodCapabilitySourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: capability_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintAccessMethodCapabilitySourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn find_base_index<'a>(
    base_snapshot: &'a PostgresSchemaSnapshotV3,
    coordinate: &IndexPartitionCoordinate,
) -> Option<&'a conceptweave_observation::IndexObservation> {
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

fn compute_capability_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintAccessMethodCapabilityObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_ACCESS_METHOD_CAPABILITY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        encode_str(&mut hasher, observation.access_method_name());
        encode_bool(&mut hasher, observation.can_exclude());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn capability_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!(
        "{}/access-method-exclusion-capability",
        coordinate.canonical_location()
    )
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

fn encode_bool(hasher: &mut Sha256, value: bool) {
    hasher.update([u8::from(value)]);
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
