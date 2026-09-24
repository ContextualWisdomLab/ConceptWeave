//! PostgreSQL ordinary exclusion-constraint implementation-function planner-support evidence.
//!
//! `pg_proc.prosupport` is an independent catalog reference from a target function to an optional
//! planner support function. PostgreSQL can invoke that support function while planning calls to the
//! target function, including operators implemented by it. This observational successor therefore
//! preserves exact same-row support absence or the independently resolved support-function signature
//! without imposing a new ordinary-EXCLUDE admission rule.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureAccessControlSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.planner_support.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_proc.prosupport` state for one governed ordinary-EXCLUDE implementation function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedurePlannerSupportObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    planner_support: Option<QualifiedProcedureSignature>,
}

impl IndexExclusionConstraintOperatorProcedurePlannerSupportObservation {
    /// Records exact support absence or the support function independently resolved from `prosupport`.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        planner_support: Option<QualifiedProcedureSignature>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if planner_support.as_ref().is_some_and(|support| {
            !matches!(support.argument_types(), [argument]
                if argument.schema_name() == "pg_catalog" && argument.type_name() == "internal")
        }) {
            return Err(ObservationError::InvalidObservationField {
                field: "exclusion_planner_support_signature",
            });
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            planner_support,
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

    /// Returns the independently resolved planner support function, or `None` for `prosupport = 0`.
    #[must_use]
    pub const fn planner_support(&self) -> Option<&QualifiedProcedureSignature> {
        self.planner_support.as_ref()
    }

    /// Returns the collision-safe evidence location for this planner-support fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_planner_support_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-function planner-support observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedurePlannerSupportSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedurePlannerSupportObservation,
}

impl IndexExclusionConstraintOperatorProcedurePlannerSupportSourceReceipt {
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

    /// Returns the owner-computed planner-support successor digest.
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

    /// Returns the exact validated planner-support observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedurePlannerSupportObservation {
        &self.location
    }
}

/// Complete `pg_proc.prosupport` evidence over one exact access-control predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedurePlannerSupportObservation>,
}

impl IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot {
    /// Creates complete planner-support evidence over one exact access-control predecessor.
    pub fn new(
        access_control_snapshot: &IndexExclusionConstraintOperatorProcedureAccessControlSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedurePlannerSupportObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = access_control_snapshot
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
                "index_exclusion_constraint_operator_procedure_planner_support_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_planner_support_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = access_control_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_planner_support_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_planner_support_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_planner_support_digest(
            access_control_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: access_control_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: access_control_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: access_control_snapshot.extractor_revision().to_owned(),
            observed_at_utc: access_control_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-function planner-support successor digest.
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

    /// Returns complete planner-support observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedurePlannerSupportObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed implementation-function planner-support identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedurePlannerSupportSourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_planner_support_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedurePlannerSupportSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_procedure_planner_support_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedurePlannerSupportObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        match observation.planner_support() {
            None => hasher.update([0]),
            Some(planner_support) => {
                hasher.update([1]);
                encode_procedure(&mut hasher, planner_support);
            }
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_planner_support_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-planner-support",
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
