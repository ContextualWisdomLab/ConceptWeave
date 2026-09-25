//! PostgreSQL ordinary exclusion-constraint operator result-contract evidence.
//!
//! `pg_operator.oprresult` and the `pg_proc.prorettype` of the function referenced by `oprcode`
//! are independently stored catalog facts. Ordinary exclusion operators are index search operators,
//! so PostgreSQL requires the operator result to be `pg_catalog.bool`. This successor preserves both
//! independently resolved result types, proves they agree with the exact operator/procedure predecessor,
//! and fails closed on non-Boolean or mismatched result contracts without rewriting predecessor digests.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorProcedureSnapshot,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.result.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Independently resolved result types for one exact ordinary-EXCLUDE operator position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorResultObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    operator_result_type: QualifiedTypeName,
    procedure_result_type: QualifiedTypeName,
}

impl IndexExclusionConstraintOperatorResultObservation {
    /// Records `pg_operator.oprresult` and `pg_proc.prorettype` for one exact governed position.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        operator_result_type: QualifiedTypeName,
        procedure_result_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            operator_result_type,
            procedure_result_type,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the one-based exclusion-key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the exact governed operator repeated for binding verification.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the exact governed implementation function repeated for binding verification.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns the stable type independently resolved from `pg_operator.oprresult`.
    #[must_use]
    pub const fn operator_result_type(&self) -> &QualifiedTypeName {
        &self.operator_result_type
    }

    /// Returns the stable type independently resolved from `pg_proc.prorettype`.
    #[must_use]
    pub const fn procedure_result_type(&self) -> &QualifiedTypeName {
        &self.procedure_result_type
    }

    /// Returns the collision-safe evidence location for this result contract.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        result_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact exclusion operator result-contract observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorResultSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorResultObservation,
}

impl IndexExclusionConstraintOperatorResultSourceReceipt {
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

    /// Returns the owner-computed operator-result successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the procedure predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the procedure predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact validated result-contract observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorResultObservation {
        &self.location
    }
}

/// Complete result-contract evidence over one exact ordinary-EXCLUDE operator-procedure snapshot.
///
/// Every governed `conexclop` position receives exactly one independently resolved `oprresult` and
/// `prorettype`. The repeated operator and implementation function must equal the exact predecessor
/// position, the two result types must agree, and both must resolve to `pg_catalog.bool`. This keeps
/// capture-time OIDs outside semantic identity while preventing a non-Boolean or shell-like catalog
/// state from being normalized into valid exclusion semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorResultSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorResultObservation>,
}

impl IndexExclusionConstraintOperatorResultSnapshot {
    /// Creates complete Boolean result-contract evidence over one exact procedure predecessor.
    pub fn new(
        procedure_snapshot: &IndexExclusionConstraintOperatorProcedureSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorResultObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = procedure_snapshot
            .observations()
            .iter()
            .map(|observation| (observation.coordinate().clone(), observation.key_position()))
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| (observation.coordinate().clone(), observation.key_position()))
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_result_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_result_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = procedure_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_operator_result_completeness")
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_result_binding",
                ));
            }
            if observation.operator_result_type() != observation.procedure_result_type()
                || !is_pg_catalog_bool(observation.operator_result_type())
            {
                return Err(invalid("index_exclusion_constraint_operator_result_state"));
            }
        }

        let snapshot_digest =
            compute_result_digest(procedure_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: procedure_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: procedure_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: procedure_snapshot.extractor_revision().to_owned(),
            observed_at_utc: procedure_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated operator-result successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the procedure predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the procedure predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete result-contract observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorResultObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE operator result contract.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorResultSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: result_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorResultSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn is_pg_catalog_bool(qualified_type: &QualifiedTypeName) -> bool {
    qualified_type.schema_name() == "pg_catalog" && qualified_type.type_name() == "bool"
}

fn compute_result_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorResultObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        encode_type(&mut hasher, observation.operator_result_type());
        encode_type(&mut hasher, observation.procedure_result_type());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn result_location(coordinate: &IndexExclusionConstraintCoordinate, key_position: u32) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/result-contract",
        coordinate.canonical_location()
    )
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

fn encode_procedure(hasher: &mut Sha256, procedure: &QualifiedProcedureSignature) {
    encode_str(hasher, procedure.schema_name());
    encode_str(hasher, procedure.procedure_name());
    encode_len(hasher, procedure.argument_types().len());
    for argument_type in procedure.argument_types() {
        encode_type(hasher, argument_type);
    }
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
