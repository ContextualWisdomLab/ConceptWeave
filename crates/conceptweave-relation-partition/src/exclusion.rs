use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{IndexOperatorFamilySnapshot, IndexPartitionCoordinate, IndexPartitionSnapshot};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Stable PostgreSQL operator identity resolved from one exclusion operator OID.
///
/// Operators are overloaded, so schema and name alone are insufficient. The exact binary operand
/// types are part of governed identity; catalog OIDs remain capture-time join coordinates only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedOperatorSignature {
    schema_name: String,
    operator_name: String,
    left_type: QualifiedTypeName,
    right_type: QualifiedTypeName,
}

impl QualifiedOperatorSignature {
    /// Creates one exact binary operator signature.
    pub fn new(
        schema_name: impl Into<String>,
        operator_name: impl Into<String>,
        left_type: QualifiedTypeName,
        right_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let operator_name = operator_name.into();
        validate_identifier(&schema_name, "exclusion_operator_schema_name")?;
        validate_nonblank(&operator_name, "exclusion_operator_name")?;
        Ok(Self {
            schema_name,
            operator_name,
            left_type,
            right_type,
        })
    }

    /// Returns the exact operator schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact operator name.
    #[must_use]
    pub fn operator_name(&self) -> &str {
        &self.operator_name
    }

    /// Returns the exact left operand type.
    #[must_use]
    pub const fn left_type(&self) -> &QualifiedTypeName {
        &self.left_type
    }

    /// Returns the exact right operand type.
    #[must_use]
    pub const fn right_type(&self) -> &QualifiedTypeName {
        &self.right_type
    }
}

/// Stable PostgreSQL procedure identity for an exclusion operator or planner-support function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedProcedureSignature {
    schema_name: String,
    procedure_name: String,
    argument_types: Vec<QualifiedTypeName>,
}

impl QualifiedProcedureSignature {
    /// Creates an exact operator or planner-support procedure signature.
    pub fn new(
        schema_name: impl Into<String>,
        procedure_name: impl Into<String>,
        argument_types: Vec<QualifiedTypeName>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let procedure_name = procedure_name.into();
        validate_identifier(&schema_name, "exclusion_procedure_schema_name")?;
        validate_identifier(&procedure_name, "exclusion_procedure_name")?;
        if !(1..=2).contains(&argument_types.len()) {
            return Err(invalid("exclusion_procedure_argument_types"));
        }
        Ok(Self {
            schema_name,
            procedure_name,
            argument_types,
        })
    }

    /// Returns the exact procedure schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact procedure name.
    #[must_use]
    pub fn procedure_name(&self) -> &str {
        &self.procedure_name
    }

    /// Returns the exact binary argument-type signature.
    #[must_use]
    pub fn argument_types(&self) -> &[QualifiedTypeName] {
        &self.argument_types
    }
}

/// Exact PostgreSQL exclusion operator/function/strategy evidence for one index key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexKeyExclusionSemanticsObservation {
    index: IndexPartitionCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    strategy: u16,
}

impl IndexKeyExclusionSemanticsObservation {
    /// Creates one exact exclusion-key semantic tuple.
    pub fn new(
        index: IndexPartitionCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        strategy: u16,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if strategy == 0 {
            return Err(invalid("exclusion_strategy"));
        }
        let expected_arguments = [operator.left_type().clone(), operator.right_type().clone()];
        if procedure.argument_types() != expected_arguments {
            return Err(invalid("exclusion_operator_procedure_signature"));
        }
        Ok(Self {
            index,
            key_position,
            operator,
            procedure,
            strategy,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the one-based key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the resolved exclusion operator signature.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the resolved underlying procedure signature.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns the exact operator-family strategy number.
    #[must_use]
    pub const fn strategy(&self) -> u16 {
        self.strategy
    }

    /// Returns the collision-safe evidence location for this exclusion-key fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "{}/keys/{}/exclusion",
            self.index.canonical_location(),
            self.key_position
        )
    }
}

/// Immutable receipt for one exact exclusion-key observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexKeyExclusionSemanticsObservation,
}

impl IndexExclusionSourceReceipt {
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

    /// Returns the owner-computed exclusion-successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact exclusion-key observation.
    #[must_use]
    pub const fn location(&self) -> &IndexKeyExclusionSemanticsObservation {
        &self.location
    }
}

/// Complete PostgreSQL exclusion semantics layered over the exact operator-family predecessor.
///
/// PostgreSQL 18 `CompareIndexInfo()` requires exclusion presence to agree and, when present,
/// compares `ii_ExclusionOps`, `ii_ExclusionProcs`, and `ii_ExclusionStrats` at every key position.
/// This successor resolves the OID arrays to stable signatures plus strategy numbers and hashes them
/// under a new domain separator without changing the predecessor identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionSemanticsSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexKeyExclusionSemanticsObservation>,
}

impl IndexExclusionSemanticsSnapshot {
    /// Creates complete exclusion evidence over one exact predecessor chain.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        operator_family_snapshot: &IndexOperatorFamilySnapshot,
        observations: Vec<IndexKeyExclusionSemanticsObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexOperatorFamilySnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot.observations().to_vec(),
        )?;
        if rebound.snapshot_digest() != operator_family_snapshot.snapshot_digest() {
            return Err(invalid("index_exclusion_semantics_predecessor_binding"));
        }

        validate_exclusion_presence(base_snapshot, index_partition_snapshot)?;
        let observations = canonicalize_exclusion_semantics(
            base_snapshot,
            index_partition_snapshot,
            observations,
        )?;
        let snapshot_digest =
            compute_exclusion_digest(operator_family_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: operator_family_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: operator_family_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: operator_family_snapshot.extractor_revision().to_owned(),
            observed_at_utc: operator_family_snapshot.observed_at_utc().to_owned(),
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

    /// Returns exclusion observations in deterministic index/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexKeyExclusionSemanticsObservation] {
        &self.observations
    }

    /// Issues exact provenance for one exclusion key.
    pub fn source_receipt(
        &self,
        index: &IndexPartitionCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.index() == index && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: format!(
                    "{}/keys/{key_position}/exclusion",
                    index.canonical_location()
                ),
            })?;
        Ok(IndexExclusionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn validate_exclusion_presence(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index_partition_snapshot: &IndexPartitionSnapshot,
) -> Result<(), ObservationError> {
    for membership in index_partition_snapshot.observations() {
        let Some(parent) = membership.parent_index() else {
            continue;
        };
        let child = find_base_index(base_snapshot, membership.coordinate())
            .ok_or_else(|| invalid("index_exclusion_semantics_index_binding"))?;
        let parent = find_base_index(base_snapshot, parent)
            .ok_or_else(|| invalid("index_exclusion_semantics_index_binding"))?;
        if exclusion_flag(child)? != exclusion_flag(parent)? {
            return Err(invalid("index_partition_definition_exclusion_presence"));
        }
    }
    Ok(())
}

fn canonicalize_exclusion_semantics(
    base_snapshot: &PostgresSchemaSnapshotV3,
    index_partition_snapshot: &IndexPartitionSnapshot,
    mut observations: Vec<IndexKeyExclusionSemanticsObservation>,
) -> Result<Vec<IndexKeyExclusionSemanticsObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        left.index()
            .cmp(right.index())
            .then_with(|| left.key_position().cmp(&right.key_position()))
    });

    let mut expected = BTreeSet::new();
    for relation in base_snapshot.relations() {
        for index in relation.indexes() {
            let is_exclusion = exclusion_flag(index)?;
            if is_exclusion {
                for attribute in index.key_attributes() {
                    expected.insert((
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        index.index_name().to_owned(),
                        attribute.position(),
                    ));
                }
            }
        }
    }
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
        return Err(invalid("index_exclusion_semantics_coordinate"));
    }
    if observed != expected {
        return Err(invalid("index_exclusion_semantics_completeness"));
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

    for membership in index_partition_snapshot.observations() {
        let Some(parent) = membership.parent_index() else {
            continue;
        };
        let child_index = find_base_index(base_snapshot, membership.coordinate())
            .ok_or_else(|| invalid("index_exclusion_semantics_index_binding"))?;
        if !exclusion_flag(child_index)? {
            continue;
        }
        for key in child_index.key_attributes() {
            let position = key.position();
            let child = by_key
                .get(&(membership.coordinate().clone(), position))
                .ok_or_else(|| invalid("index_exclusion_semantics_completeness"))?;
            let parent = by_key
                .get(&(parent.clone(), position))
                .ok_or_else(|| invalid("index_exclusion_semantics_completeness"))?;
            if child.operator() != parent.operator() {
                return Err(invalid("index_partition_definition_exclusion_operator"));
            }
            if child.procedure() != parent.procedure() {
                return Err(invalid("index_partition_definition_exclusion_procedure"));
            }
            if child.strategy() != parent.strategy() {
                return Err(invalid("index_partition_definition_exclusion_strategy"));
            }
        }
    }

    Ok(observations)
}

fn exclusion_flag(
    index: &conceptweave_observation::IndexObservation,
) -> Result<bool, ObservationError> {
    index
        .catalog_flags()
        .map(|flags| flags.exclusion())
        .ok_or_else(|| invalid("index_exclusion_semantics_catalog_flags"))
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

fn compute_exclusion_digest(
    predecessor_digest: &str,
    observations: &[IndexKeyExclusionSemanticsObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        let index = observation.index();
        encode_str(&mut hasher, index.schema_name());
        encode_str(&mut hasher, index.relation_name());
        encode_str(&mut hasher, index.relation_kind().token());
        encode_str(&mut hasher, index.index_name());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update(observation.strategy().to_be_bytes());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_operator(hasher: &mut Sha256, operator: &QualifiedOperatorSignature) {
    encode_str(hasher, operator.schema_name());
    encode_str(hasher, operator.operator_name());
    encode_type(hasher, operator.left_type());
    encode_type(hasher, operator.right_type());
}

fn encode_procedure(hasher: &mut Sha256, procedure: &QualifiedProcedureSignature) {
    encode_str(hasher, procedure.schema_name());
    encode_str(hasher, procedure.procedure_name());
    encode_len(hasher, procedure.argument_types().len());
    for argument in procedure.argument_types() {
        encode_type(hasher, argument);
    }
}

fn encode_type(hasher: &mut Sha256, qualified_type: &QualifiedTypeName) {
    encode_str(hasher, qualified_type.schema_name());
    encode_str(hasher, qualified_type.type_name());
}

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(invalid(field));
    }
    Ok(())
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
