//! PostgreSQL ordinary exclusion-constraint operator scalar-procedure evidence.
//!
//! Ordinary EXCLUDE enforcement invokes each resolved exclusion procedure as a scalar two-argument
//! function and immediately consumes one Boolean datum. `pg_proc.proretset` is independent catalog
//! state, so a stable procedure signature plus `prorettype = pg_catalog.bool` does not prove scalar
//! cardinality. This successor binds raw `proretset` evidence to each exact `oprcode` procedure and
//! fails closed on set-returning implementations without rewriting predecessor digest domains.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorKindSnapshot,
    IndexExclusionConstraintOperatorResultSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.scalar.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw `pg_proc.proretset` evidence for one exact ordinary-EXCLUDE implementation procedure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureScalarObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    returns_set: bool,
}

impl IndexExclusionConstraintOperatorProcedureScalarObservation {
    /// Records one exact governed operator/procedure binding and raw `pg_proc.proretset` state.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        returns_set: bool,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            returns_set,
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

    /// Returns the exact governed operator repeated for predecessor binding.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the exact `pg_operator.oprcode` procedure repeated for predecessor binding.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns raw `pg_proc.proretset` state for the exact implementation procedure.
    #[must_use]
    pub const fn returns_set(&self) -> bool {
        self.returns_set
    }

    /// Returns the collision-safe evidence location for this scalar-cardinality fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_scalar_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact operator implementation scalar-state observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureScalarSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureScalarObservation,
}

impl IndexExclusionConstraintOperatorProcedureScalarSourceReceipt {
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

    /// Returns the owner-computed scalar-procedure successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact validated scalar-procedure observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedureScalarObservation {
        &self.location
    }
}

/// Complete raw `pg_proc.proretset` evidence over one exact ordinary-EXCLUDE operator-kind snapshot.
///
/// The result predecessor supplies the exact operator/procedure position while the kind predecessor
/// proves the current raw operator-row shape. The kind snapshot is rebound to the supplied result
/// snapshot before use. Each exact `oprcode` procedure must then have one independently observed
/// `proretset=false` fact because EXCLUDE enforcement consumes a single Boolean datum per invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureScalarSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureScalarObservation>,
}

impl IndexExclusionConstraintOperatorProcedureScalarSnapshot {
    /// Creates complete scalar-procedure evidence over one exact result/kind predecessor chain.
    pub fn new(
        result_snapshot: &IndexExclusionConstraintOperatorResultSnapshot,
        kind_snapshot: &IndexExclusionConstraintOperatorKindSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureScalarObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound_kind = IndexExclusionConstraintOperatorKindSnapshot::new(
            result_snapshot,
            kind_snapshot.observations().to_vec(),
        )?;
        if rebound_kind.snapshot_digest() != kind_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_scalar_predecessor_binding",
            ));
        }

        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = result_snapshot
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
                "index_exclusion_constraint_operator_procedure_scalar_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_scalar_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = result_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_scalar_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_scalar_binding",
                ));
            }
            if observation.returns_set() {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_scalar_state",
                ));
            }
        }

        let snapshot_digest = compute_procedure_scalar_digest(
            kind_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: kind_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: kind_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: kind_snapshot.extractor_revision().to_owned(),
            observed_at_utc: kind_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated scalar-procedure successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete scalar-procedure observations in deterministic coordinate/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedureScalarObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE procedure position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureScalarSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_scalar_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureScalarSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_procedure_scalar_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureScalarObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update([u8::from(observation.returns_set())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_scalar_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-scalar",
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

fn encode_type(
    hasher: &mut Sha256,
    qualified_type: &conceptweave_observation::QualifiedTypeName,
) {
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
