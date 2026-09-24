use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, QualifiedOperatorClassName,
};
use sha2::{Digest, Sha256};

use super::{IndexPartitionCoordinate, IndexPartitionSnapshot};
use crate::RelationPartitionSnapshot;

const INDEX_OPERATOR_FAMILY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact resolved PostgreSQL operator-family coordinate for one index key.
///
/// `pg_opfamily` identity is access-method-relative, so governed identity includes the exact access
/// method, schema, and family name rather than a capture-time catalog OID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedOperatorFamilyName {
    access_method_name: String,
    schema_name: String,
    operator_family_name: String,
}

impl QualifiedOperatorFamilyName {
    /// Creates one resolved operator-family coordinate from exact catalog identifiers.
    pub fn new(
        access_method_name: impl Into<String>,
        schema_name: impl Into<String>,
        operator_family_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let access_method_name = access_method_name.into();
        let schema_name = schema_name.into();
        let operator_family_name = operator_family_name.into();
        validate_identifier(&access_method_name, "operator_family_access_method_name")?;
        validate_identifier(&schema_name, "operator_family_schema_name")?;
        validate_identifier(&operator_family_name, "operator_family_name")?;
        Ok(Self {
            access_method_name,
            schema_name,
            operator_family_name,
        })
    }

    /// Returns the exact access-method name that owns this family.
    #[must_use]
    pub fn access_method_name(&self) -> &str {
        &self.access_method_name
    }

    /// Returns the exact schema containing this operator family.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact operator-family name.
    #[must_use]
    pub fn operator_family_name(&self) -> &str {
        &self.operator_family_name
    }
}

/// Exact `pg_opclass` to `pg_opfamily` resolution for one observed index-key position.
///
/// The operator-class coordinate is repeated deliberately: snapshot construction verifies it against
/// the frozen v3 key semantics, preventing a caller from attaching a family to a different class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexKeyOperatorFamilyObservation {
    index: IndexPartitionCoordinate,
    key_position: u32,
    operator_class: QualifiedOperatorClassName,
    operator_family: QualifiedOperatorFamilyName,
}

impl IndexKeyOperatorFamilyObservation {
    /// Creates one exact key-level operator-family resolution.
    pub fn new(
        index: IndexPartitionCoordinate,
        key_position: u32,
        operator_class: QualifiedOperatorClassName,
        operator_family: QualifiedOperatorFamilyName,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            index,
            key_position,
            operator_class,
            operator_family,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the one-based index-key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the exact operator-class coordinate resolved through `pg_opclass`.
    #[must_use]
    pub const fn operator_class(&self) -> &QualifiedOperatorClassName {
        &self.operator_class
    }

    /// Returns the exact resolved operator-family coordinate.
    #[must_use]
    pub const fn operator_family(&self) -> &QualifiedOperatorFamilyName {
        &self.operator_family
    }

    /// Returns the collision-safe evidence location for this key-family fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "{}/keys/{}/operator-family",
            self.index.canonical_location(),
            self.key_position
        )
    }
}

/// Immutable receipt for one exact key-level operator-family observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexOperatorFamilySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexKeyOperatorFamilyObservation,
}

impl IndexOperatorFamilySourceReceipt {
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

    /// Returns the governed operator-family successor digest.
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

    /// Returns the exact verified key-family observation.
    #[must_use]
    pub const fn location(&self) -> &IndexKeyOperatorFamilyObservation {
        &self.location
    }
}

/// Complete operator-family evidence layered over one exact index-partition snapshot.
///
/// PostgreSQL 18 `CompareIndexInfo()` compares operator-family OIDs for corresponding key positions,
/// not operator-class names. This successor resolves each `pg_index.indclass` through `pg_opclass`
/// to its exact access-method/schema/family coordinate, binds the repeated class coordinate to the
/// frozen v3 key semantics, requires complete evidence for every bounded index key, and rejects a
/// direct parent/child index edge whose corresponding operator families differ. Its digest frames the
/// exact predecessor index-partition digest, so the frozen v3 and index-partition identities keep
/// their existing meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexOperatorFamilySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexKeyOperatorFamilyObservation>,
}

impl IndexOperatorFamilySnapshot {
    /// Creates complete key-level operator-family evidence over an exact predecessor stack.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        observations: Vec<IndexKeyOperatorFamilyObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexPartitionSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != index_partition_snapshot.snapshot_digest() {
            return Err(invalid("index_operator_family_predecessor_binding"));
        }

        let observations =
            canonicalize_operator_families(base_snapshot, index_partition_snapshot, observations)?;
        let snapshot_digest = compute_operator_family_digest(
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

    /// Returns the owner-computed domain-separated operator-family digest.
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

    /// Returns complete key-family observations in deterministic index/position order.
    #[must_use]
    pub fn observations(&self) -> &[IndexKeyOperatorFamilyObservation] {
        &self.observations
    }

    /// Issues provenance for one exact observed key-family coordinate.
    pub fn source_receipt(
        &self,
        index: &IndexPartitionCoordinate,
        key_position: u32,
    ) -> Result<IndexOperatorFamilySourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.index() == index && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: format!(
                    "{}/keys/{key_position}/operator-family",
                    index.canonical_location()
                ),
            })?;
        Ok(IndexOperatorFamilySourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn canonicalize_operator_families(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index_partition_snapshot: &IndexPartitionSnapshot,
    mut observations: Vec<IndexKeyOperatorFamilyObservation>,
) -> Result<Vec<IndexKeyOperatorFamilyObservation>, ObservationError> {
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
                index.key_attributes().iter().map(move |attribute| {
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
        return Err(invalid("index_operator_family_coordinate"));
    }
    if observed != expected {
        return Err(invalid("index_operator_family_completeness"));
    }

    let by_key = observations
        .iter()
        .map(|observation| {
            (
                (observation.index().clone(), observation.key_position()),
                observation,
            )
        })
        .collect::<BTreeMap<_, _>>();

    for observation in &observations {
        let index = find_base_index(base_snapshot, observation.index())
            .ok_or_else(|| invalid("index_operator_family_index_binding"))?;
        let access_method = index
            .access_method()
            .ok_or_else(|| invalid("index_operator_family_access_method_binding"))?;
        if access_method != observation.operator_family().access_method_name() {
            return Err(invalid("index_operator_family_access_method_binding"));
        }
        let semantics = index
            .key_semantics()
            .ok_or_else(|| invalid("index_operator_family_class_binding"))?
            .iter()
            .find(|semantics| semantics.position() == observation.key_position())
            .ok_or_else(|| invalid("index_operator_family_class_binding"))?;
        if semantics.operator_class() != observation.operator_class() {
            return Err(invalid("index_operator_family_class_binding"));
        }
    }

    for membership in index_partition_snapshot.observations() {
        let Some(parent) = membership.parent_index() else {
            continue;
        };
        let child_index = find_base_index(base_snapshot, membership.coordinate())
            .ok_or_else(|| invalid("index_operator_family_index_binding"))?;
        for key_attribute in child_index.key_attributes() {
            let position = key_attribute.position();
            let child_family = by_key
                .get(&(membership.coordinate().clone(), position))
                .ok_or_else(|| invalid("index_operator_family_completeness"))?;
            let parent_family = by_key
                .get(&(parent.clone(), position))
                .ok_or_else(|| invalid("index_operator_family_completeness"))?;
            if child_family.operator_family() != parent_family.operator_family() {
                return Err(invalid("index_partition_definition_operator_family"));
            }
        }
    }

    Ok(observations)
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

fn compute_operator_family_digest(
    predecessor_digest: &str,
    observations: &[IndexKeyOperatorFamilyObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_OPERATOR_FAMILY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        let index = observation.index();
        encode_str(&mut hasher, index.schema_name());
        encode_str(&mut hasher, index.relation_name());
        encode_str(&mut hasher, index.relation_kind().token());
        encode_str(&mut hasher, index.index_name());
        hasher.update(observation.key_position().to_be_bytes());
        encode_str(&mut hasher, observation.operator_class().schema_name());
        encode_str(
            &mut hasher,
            observation.operator_class().operator_class_name(),
        );
        encode_str(
            &mut hasher,
            observation.operator_family().access_method_name(),
        );
        encode_str(&mut hasher, observation.operator_family().schema_name());
        encode_str(
            &mut hasher,
            observation.operator_family().operator_family_name(),
        );
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn validate_identifier(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.is_empty() || value.contains('\0') {
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
