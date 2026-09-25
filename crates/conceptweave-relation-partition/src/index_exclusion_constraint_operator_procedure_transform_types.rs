//! PostgreSQL ordinary exclusion-constraint implementation-function transform-type evidence.
//!
//! `pg_proc.protrftypes` is an independent nullable `oid[]` selecting the argument/result types for
//! which one function call applies language transforms declared by `TRANSFORM FOR TYPE`. Exact target
//! function identity, implementation language, body, ACL, planner support, and planner cost do not
//! determine that selection. This observational successor therefore preserves the exact NULL state or
//! the same-generation resolved qualified type set without inventing a transform-admission policy.
//!
//! This contract intentionally does not authenticate the mutable `pg_transform` converter row for a
//! selected `(type, language)` pair. Binding `trffromsql`/`trftosql` implementation identity is a
//! separate reviewed successor when that stronger runtime-conversion claim is required.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorProcedureCostSnapshot,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_types.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact same-row `pg_proc.protrftypes` observation for one governed ordinary-EXCLUDE function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformTypesObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    transform_types: Option<Vec<QualifiedTypeName>>,
}

impl IndexExclusionConstraintOperatorProcedureTransformTypesObservation {
    /// Records exact NULL or nonempty same-generation resolved transform-type evidence.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        mut transform_types: Option<Vec<QualifiedTypeName>>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if transform_types.as_ref().is_some_and(Vec::is_empty) {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_types_empty",
            ));
        }
        if let Some(types) = transform_types.as_mut() {
            types.sort_by(|left, right| {
                (left.schema_name(), left.type_name())
                    .cmp(&(right.schema_name(), right.type_name()))
            });
            if types.windows(2).any(|pair| pair[0] == pair[1]) {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_types_duplicate",
                ));
            }
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            transform_types,
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

    /// Returns the exact governed exclusion operator repeated for predecessor binding.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the exact `pg_operator.oprcode` function repeated for predecessor binding.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns NULL or the canonical resolved transform-type set from exact `pg_proc.protrftypes`.
    #[must_use]
    pub fn transform_types(&self) -> Option<&[QualifiedTypeName]> {
        self.transform_types.as_deref()
    }

    /// Returns the collision-safe evidence location for this transform-type fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_types_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-function transform-type observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformTypesObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt {
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

    /// Returns the owner-computed transform-type successor digest.
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

    /// Returns the exact validated transform-type observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformTypesObservation {
        &self.location
    }
}

/// Complete `pg_proc.protrftypes` evidence over one exact planner-cost predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformTypesObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot {
    /// Creates complete transform-type evidence over one exact planner-cost predecessor.
    pub fn new(
        cost_snapshot: &IndexExclusionConstraintOperatorProcedureCostSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureTransformTypesObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = cost_snapshot
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
                "index_exclusion_constraint_operator_procedure_transform_types_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_types_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = cost_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_types_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_types_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_transform_types_digest(
            cost_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: cost_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: cost_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: cost_snapshot.extractor_revision().to_owned(),
            observed_at_utc: cost_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-function transform-type successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete transform-type observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformTypesObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed implementation-function transform-type identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt,
        ObservationError,
    > {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_transform_types_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureTransformTypesSourceReceipt {
                source_id: self.source_connection_key.clone(),
                connection_policy_binding: self.connection_policy_binding.clone(),
                source_digest: self.snapshot_digest.clone(),
                extractor_revision: self.extractor_revision.clone(),
                observed_at_utc: self.observed_at_utc.clone(),
                location: observation.clone(),
            },
        )
    }
}

fn compute_procedure_transform_types_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformTypesObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_TYPES_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        match observation.transform_types() {
            None => hasher.update([0]),
            Some(transform_types) => {
                hasher.update([1]);
                encode_len(&mut hasher, transform_types.len());
                for transform_type in transform_types {
                    encode_type(&mut hasher, transform_type);
                }
            }
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_transform_types_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-types",
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
