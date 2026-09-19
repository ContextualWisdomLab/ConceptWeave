//! PostgreSQL ordinary exclusion-constraint `conexclop` evidence.
//!
//! `pg_constraint.conexclop` is independently stored catalog state for an ordinary `EXCLUDE`
//! constraint. PostgreSQL also carries exclusion operators in the backing index's `IndexInfo`, so a
//! governed snapshot must retain both facts and reject disagreement instead of inferring one from
//! the other. OIDs are capture-time join coordinates only; this successor records their resolved,
//! stable operator signatures without rewriting any predecessor digest domain.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintKeySnapshot,
    IndexExclusionConstraintPeriodSnapshot, IndexExclusionConstraintSnapshot,
    IndexExclusionSemanticsSnapshot, IndexOperatorFamilySnapshot, IndexPartitionSnapshot,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact resolved `pg_constraint.conexclop` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    operators: Vec<QualifiedOperatorSignature>,
}

impl IndexExclusionConstraintOperatorObservation {
    /// Records the ordered exclusion-operator vector resolved from `pg_constraint.conexclop`.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        operators: Vec<QualifiedOperatorSignature>,
    ) -> Result<Self, ObservationError> {
        if operators.is_empty() {
            return Err(invalid("index_exclusion_constraint_operator_state"));
        }
        Ok(Self {
            coordinate,
            operators,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the ordered stable signatures resolved from the raw `conexclop` OID array.
    #[must_use]
    pub fn operators(&self) -> &[QualifiedOperatorSignature] {
        &self.operators
    }

    /// Returns the collision-safe evidence location for this constraint-side operator vector.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!("{}/exclusion-operators", self.coordinate.canonical_location())
    }
}

/// Exact ordinary-EXCLUDE predecessor branch needed to rebind constraint-side operator evidence.
#[derive(Clone, Copy, Debug)]
pub struct IndexExclusionConstraintOperatorSourceLineage<'a> {
    base_snapshot: &'a PostgresSchemaSnapshotV3,
    relation_partition_snapshot: &'a RelationPartitionSnapshot,
    index_partition_snapshot: &'a IndexPartitionSnapshot,
    constraint_snapshot: &'a IndexExclusionConstraintSnapshot,
    period_snapshot: &'a IndexExclusionConstraintPeriodSnapshot,
    key_snapshot: &'a IndexExclusionConstraintKeySnapshot,
}

impl<'a> IndexExclusionConstraintOperatorSourceLineage<'a> {
    /// Binds the exact v3 through ordinary-EXCLUDE `conkey` predecessor chain for one composition.
    #[must_use]
    pub const fn new(
        base_snapshot: &'a PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &'a RelationPartitionSnapshot,
        index_partition_snapshot: &'a IndexPartitionSnapshot,
        constraint_snapshot: &'a IndexExclusionConstraintSnapshot,
        period_snapshot: &'a IndexExclusionConstraintPeriodSnapshot,
        key_snapshot: &'a IndexExclusionConstraintKeySnapshot,
    ) -> Self {
        Self {
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot,
            period_snapshot,
            key_snapshot,
        }
    }
}

/// Exact backing-index semantic predecessor branch for constraint-side operator comparison.
#[derive(Clone, Copy, Debug)]
pub struct IndexExclusionConstraintOperatorSemanticsLineage<'a> {
    operator_family_snapshot: &'a IndexOperatorFamilySnapshot,
    exclusion_semantics_snapshot: &'a IndexExclusionSemanticsSnapshot,
}

impl<'a> IndexExclusionConstraintOperatorSemanticsLineage<'a> {
    /// Binds the exact operator-family and exclusion-semantics predecessors for one composition.
    #[must_use]
    pub const fn new(
        operator_family_snapshot: &'a IndexOperatorFamilySnapshot,
        exclusion_semantics_snapshot: &'a IndexExclusionSemanticsSnapshot,
    ) -> Self {
        Self {
            operator_family_snapshot,
            exclusion_semantics_snapshot,
        }
    }
}

/// Immutable provenance receipt for one exact ordinary EXCLUDE `conexclop` observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorObservation,
}

impl IndexExclusionConstraintOperatorSourceReceipt {
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

    /// Returns the owner-computed ordinary EXCLUDE constraint-operator successor digest.
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

    /// Returns the exact ordinary EXCLUDE constraint-operator observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorObservation {
        &self.location
    }
}

/// Complete constraint-side exclusion-operator evidence over exact ordinary EXCLUDE predecessors.
///
/// Every ordinary `contype='x'` coordinate receives exactly one resolved `conexclop` vector. The
/// supplied source, relation-partition, index-partition, ordinary-EXCLUDE, period, key, operator-
/// family, and backing-index exclusion-semantics predecessors are rebound before use. The ordered
/// constraint-side vector must then equal the exact operators independently observed for its
/// `conindid` backing index. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` constraints remain in
/// their key-constraint owner family and are not reclassified as ordinary EXCLUDE constraints here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorObservation>,
    backing_procedures: Vec<(
        IndexExclusionConstraintCoordinate,
        u32,
        QualifiedProcedureSignature,
    )>,
}

impl IndexExclusionConstraintOperatorSnapshot {
    /// Creates complete constraint-side exclusion-operator evidence over exact predecessor stacks.
    pub fn new(
        source_lineage: IndexExclusionConstraintOperatorSourceLineage<'_>,
        semantics_lineage: IndexExclusionConstraintOperatorSemanticsLineage<'_>,
        mut observations: Vec<IndexExclusionConstraintOperatorObservation>,
    ) -> Result<Self, ObservationError> {
        let base_snapshot = source_lineage.base_snapshot;
        let relation_partition_snapshot = source_lineage.relation_partition_snapshot;
        let index_partition_snapshot = source_lineage.index_partition_snapshot;
        let constraint_snapshot = source_lineage.constraint_snapshot;
        let period_snapshot = source_lineage.period_snapshot;
        let key_snapshot = source_lineage.key_snapshot;
        let operator_family_snapshot = semantics_lineage.operator_family_snapshot;
        let exclusion_semantics_snapshot = semantics_lineage.exclusion_semantics_snapshot;

        let rebound_constraint = IndexExclusionConstraintSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot.observations().to_vec(),
        )?;
        if rebound_constraint.snapshot_digest() != constraint_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_predecessor_binding",
            ));
        }

        let rebound_period = IndexExclusionConstraintPeriodSnapshot::new(
            &rebound_constraint,
            period_snapshot.observations().to_vec(),
        )?;
        if rebound_period.snapshot_digest() != period_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_predecessor_binding",
            ));
        }

        let rebound_key = IndexExclusionConstraintKeySnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            &rebound_constraint,
            &rebound_period,
            key_snapshot.observations().to_vec(),
        )?;
        if rebound_key.snapshot_digest() != key_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_predecessor_binding",
            ));
        }

        let rebound_families = IndexOperatorFamilySnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot.observations().to_vec(),
        )?;
        if rebound_families.snapshot_digest() != operator_family_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_predecessor_binding",
            ));
        }

        let rebound_semantics = IndexExclusionSemanticsSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            &rebound_families,
            exclusion_semantics_snapshot.observations().to_vec(),
        )?;
        if rebound_semantics.snapshot_digest() != exclusion_semantics_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_predecessor_binding",
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
            return Err(invalid("index_exclusion_constraint_operator_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_completeness",
            ));
        }

        let mut backing_procedures = Vec::new();
        for observation in &observations {
            let constraint = rebound_constraint
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == observation.coordinate())
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_operator_completeness")
                })?;
            let mut backing = rebound_semantics
                .observations()
                .iter()
                .filter(|candidate| candidate.index() == constraint.backing_index())
                .collect::<Vec<_>>();
            backing.sort_by_key(|candidate| candidate.key_position());
            let expected_operators = backing
                .iter()
                .map(|candidate| candidate.operator().clone())
                .collect::<Vec<_>>();
            if expected_operators.is_empty() || observation.operators() != expected_operators {
                return Err(invalid("index_exclusion_constraint_operator_state"));
            }
            backing_procedures.extend(backing.into_iter().map(|candidate| {
                (
                    observation.coordinate().clone(),
                    candidate.key_position(),
                    candidate.procedure().clone(),
                )
            }));
        }

        let snapshot_digest = compute_operator_digest(
            rebound_key.snapshot_digest(),
            rebound_semantics.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: rebound_key.source_connection_key().to_owned(),
            connection_policy_binding: rebound_key.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: rebound_key.extractor_revision().to_owned(),
            observed_at_utc: rebound_key.observed_at_utc().to_owned(),
            observations,
            backing_procedures,
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

    /// Returns the owner-computed domain-separated ordinary EXCLUDE operator digest.
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

    /// Returns complete constraint-side operator observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorObservation] {
        &self.observations
    }

    /// Returns the exact backing exclusion procedure retained for one constraint/key position.
    ///
    /// This metadata is inherited from the already-governed backing-index exclusion semantics and
    /// is intentionally not added to the existing operator digest domain. A later successor compares
    /// it with an independently resolved `pg_operator.oprcode` observation.
    #[must_use]
    pub fn backing_procedure(
        &self,
        coordinate: &IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Option<&QualifiedProcedureSignature> {
        self.backing_procedures
            .iter()
            .find(|(candidate, position, _)| {
                candidate == coordinate && *position == key_position
            })
            .map(|(_, _, procedure)| procedure)
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE `conexclop` coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintOperatorSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: format!("{}/exclusion-operators", coordinate.canonical_location()),
            })?;
        Ok(IndexExclusionConstraintOperatorSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_operator_digest(
    key_predecessor_digest: &str,
    semantics_predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, key_predecessor_digest);
    encode_str(&mut hasher, semantics_predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        encode_len(&mut hasher, observation.operators().len());
        for operator in observation.operators() {
            encode_operator(&mut hasher, operator);
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
}

fn encode_operator(hasher: &mut Sha256, operator: &QualifiedOperatorSignature) {
    encode_str(hasher, operator.schema_name());
    encode_str(hasher, operator.operator_name());
    encode_type(hasher, operator.left_type());
    encode_type(hasher, operator.right_type());
}

fn encode_type(hasher: &mut Sha256, qualified_type: &QualifiedTypeName) {
    encode_str(hasher, qualified_type.schema_name());
    encode_str(hasher, qualified_type.type_name());
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
