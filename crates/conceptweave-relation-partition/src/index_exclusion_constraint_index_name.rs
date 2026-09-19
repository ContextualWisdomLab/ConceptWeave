//! PostgreSQL ordinary exclusion-constraint backing-index naming evidence.
//!
//! PostgreSQL implements an ordinary `EXCLUDE` constraint with an index that has the same name as
//! the constraint. That name also occupies the schema-scoped `pg_class` relation namespace. The
//! constraint coordinate and resolved `conindid` index coordinate are independent source facts, so
//! this successor validates their exact-name equality and the bounded schema namespace instead of
//! deriving either fact from the other or rewriting an issued predecessor digest.

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCatalogShapeSnapshot, IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintSnapshot, IndexPartitionCoordinate, IndexPartitionSnapshot,
};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.index_name.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Validated cross-catalog name binding for one ordinary `EXCLUDE` constraint and backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNameObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
}

impl IndexExclusionConstraintIndexNameObservation {
    /// Returns the exact ordinary exclusion-constraint coordinate from `pg_constraint` evidence.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact resolved `conindid` backing-index coordinate from `pg_class` evidence.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }

    /// Returns a collision-safe location for the validated backing-index name binding.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        index_name_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one validated ordinary EXCLUDE backing-index name binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNameSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintIndexNameObservation,
}

impl IndexExclusionConstraintIndexNameSourceReceipt {
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

    /// Returns the owner-computed backing-index name-binding digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the validated constraint/index name binding.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintIndexNameObservation {
        &self.location
    }
}

/// Complete ordinary EXCLUDE backing-index naming integrity over one exact predecessor chain.
///
/// PostgreSQL keeps the constraint and index in different catalogs, but an index-backed `EXCLUDE`
/// constraint owns an index with the same name. Index relations share the schema's `pg_class`
/// relation namespace, so the exact backing-index name must occur once among observed indexes in
/// that schema and must not collide with an observed table/view/sequence/composite relation name.
/// The checks are bounded to the authorized source snapshot; no missing relation is synthesized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNameSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    source_snapshot_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintIndexNameObservation>,
}

impl IndexExclusionConstraintIndexNameSnapshot {
    /// Rebinds the exact predecessor chain and validates index-backed EXCLUDE naming integrity.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        catalog_shape_snapshot: &IndexExclusionConstraintCatalogShapeSnapshot,
    ) -> Result<Self, ObservationError> {
        let rebound_constraint = IndexExclusionConstraintSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot.observations().to_vec(),
        )?;
        if rebound_constraint.snapshot_digest() != constraint_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_index_name_predecessor_binding",
            ));
        }

        let rebound_catalog_shape = IndexExclusionConstraintCatalogShapeSnapshot::new(
            &rebound_constraint,
            catalog_shape_snapshot.observations().to_vec(),
        )?;
        if rebound_catalog_shape.snapshot_digest() != catalog_shape_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_index_name_predecessor_binding",
            ));
        }

        let mut observations = Vec::with_capacity(rebound_constraint.observations().len());
        for constraint in rebound_constraint.observations() {
            if constraint.coordinate().constraint_name() != constraint.backing_index().index_name() {
                return Err(invalid("index_exclusion_constraint_index_name_state"));
            }
            validate_schema_relation_namespace(base_snapshot, constraint.backing_index())?;
            observations.push(IndexExclusionConstraintIndexNameObservation {
                coordinate: constraint.coordinate().clone(),
                backing_index: constraint.backing_index().clone(),
            });
        }

        let snapshot_digest = compute_index_name_digest(
            rebound_catalog_shape.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: rebound_catalog_shape.source_connection_key().to_owned(),
            connection_policy_binding: rebound_catalog_shape.connection_policy_binding().to_owned(),
            source_snapshot_digest: base_snapshot.snapshot_digest().to_owned(),
            snapshot_digest,
            extractor_revision: rebound_catalog_shape.extractor_revision().to_owned(),
            observed_at_utc: rebound_catalog_shape.observed_at_utc().to_owned(),
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

    /// Returns the exact v3 source-content digest bound by this predecessor generation.
    #[must_use]
    pub fn source_snapshot_digest(&self) -> &str {
        &self.source_snapshot_digest
    }

    /// Returns the domain-separated ordinary EXCLUDE backing-index name-binding digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the catalog-shape predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns validated name bindings in deterministic predecessor order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintIndexNameObservation] {
        &self.observations
    }

    /// Issues provenance for one validated ordinary EXCLUDE backing-index name binding.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintIndexNameSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: index_name_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintIndexNameSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn validate_schema_relation_namespace(
    base_snapshot: &PostgresSchemaSnapshotV3,
    backing_index: &IndexPartitionCoordinate,
) -> Result<(), ObservationError> {
    let schema_name = backing_index.schema_name();
    let index_name = backing_index.index_name();

    if base_snapshot.relations().iter().any(|relation| {
        relation.schema_name() == schema_name && relation.relation_name() == index_name
    }) {
        return Err(invalid(
            "index_exclusion_constraint_index_name_namespace",
        ));
    }

    let mut matching_index_count = 0usize;
    let mut exact_backing_count = 0usize;
    for relation in base_snapshot
        .relations()
        .iter()
        .filter(|relation| relation.schema_name() == schema_name)
    {
        for index in relation
            .indexes()
            .iter()
            .filter(|index| index.index_name() == index_name)
        {
            matching_index_count += 1;
            if relation.relation_name() == backing_index.relation_name()
                && relation.kind() == backing_index.relation_kind()
            {
                exact_backing_count += 1;
            }
        }
    }

    if matching_index_count != 1 || exact_backing_count != 1 {
        return Err(invalid(
            "index_exclusion_constraint_index_name_namespace",
        ));
    }
    Ok(())
}

fn compute_index_name_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintIndexNameObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_INDEX_NAME_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn index_name_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/backing-index-name", coordinate.canonical_location())
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

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
