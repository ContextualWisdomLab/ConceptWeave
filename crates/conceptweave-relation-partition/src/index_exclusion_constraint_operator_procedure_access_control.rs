//! PostgreSQL ordinary exclusion-constraint implementation-function access-control evidence.
//!
//! `pg_proc.proacl` stores routine access privileges independently from the implementation
//! function's stable identity, owner, executable definition, execution flags, and local run-time
//! configuration. PostgreSQL's function privilege is `EXECUTE`, and that privilege also governs
//! use of operators implemented by the function. This layer therefore binds the exact same-row ACL
//! encoding state plus the effective EXECUTE grant set while reducing role-bearing grant material
//! immediately to a domain-separated digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureConfigurationSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_MATERIAL_DIGEST_DOMAIN_V1:
    &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.access_control.material.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.access_control.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ExecuteGrantee {
    Public,
    Role(String),
}

/// One effective PostgreSQL function `EXECUTE` grant resolved in the same source generation.
///
/// The adapter resolves ACL role OIDs to role names before this boundary and represents grantee OID
/// zero as [`Self::public`]. Grant order is not semantic and is canonicalized by the material layer.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IndexExclusionConstraintOperatorProcedureExecuteGrant {
    grantee: ExecuteGrantee,
    grantor_role_name: String,
    grant_option: bool,
}

impl IndexExclusionConstraintOperatorProcedureExecuteGrant {
    /// Records an `EXECUTE` grant to `PUBLIC` with the exact resolved grantor role.
    pub fn public(
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_access_control_grantor_role_name",
        )?;
        Ok(Self {
            grantee: ExecuteGrantee::Public,
            grantor_role_name,
            grant_option,
        })
    }

    /// Records an `EXECUTE` grant to one exact resolved role with the exact resolved grantor role.
    pub fn role(
        grantee_role_name: impl Into<String>,
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantee_role_name = grantee_role_name.into();
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantee_role_name,
            "index_exclusion_constraint_operator_procedure_access_control_grantee_role_name",
        )?;
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_access_control_grantor_role_name",
        )?;
        Ok(Self {
            grantee: ExecuteGrantee::Role(grantee_role_name),
            grantor_role_name,
            grant_option,
        })
    }
}

/// Privacy-preserving identity for one exact `pg_proc.proacl` state and effective EXECUTE grant set.
///
/// `proacl=NULL` is preserved separately from an explicit ACL with the same effective grants. The
/// effective grant set should come from the exact same-row ACL using PostgreSQL ACL semantics, with
/// default function privileges materialized when `proacl` is null. Grant array order is ignored,
/// because access privileges are a set; duplicate effective grant evidence fails closed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureAccessControlMaterial {
    proacl_was_null: bool,
    grant_count: usize,
    digest: String,
}

impl IndexExclusionConstraintOperatorProcedureAccessControlMaterial {
    /// Reduces exact same-generation ACL state and effective EXECUTE grants to a stable digest.
    pub fn new(
        proacl_was_null: bool,
        mut grants: Vec<IndexExclusionConstraintOperatorProcedureExecuteGrant>,
    ) -> Result<Self, ObservationError> {
        grants.sort();
        if grants.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_access_control_grant",
            ));
        }

        let mut hasher = Sha256::new();
        hasher.update(
            INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_MATERIAL_DIGEST_DOMAIN_V1,
        );
        hasher.update([u8::from(proacl_was_null)]);
        encode_len(&mut hasher, grants.len());
        for grant in &grants {
            match &grant.grantee {
                ExecuteGrantee::Public => hasher.update([0]),
                ExecuteGrantee::Role(role_name) => {
                    hasher.update([1]);
                    encode_str(&mut hasher, role_name);
                }
            }
            encode_str(&mut hasher, &grant.grantor_role_name);
            hasher.update([u8::from(grant.grant_option)]);
        }

        Ok(Self {
            proacl_was_null,
            grant_count: grants.len(),
            digest: format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize()),
        })
    }

    /// Returns whether the exact `pg_proc.proacl` catalog value was null.
    #[must_use]
    pub const fn proacl_was_null(&self) -> bool {
        self.proacl_was_null
    }

    /// Returns the number of canonical effective EXECUTE grants consumed into the digest.
    #[must_use]
    pub const fn grant_count(&self) -> usize {
        self.grant_count
    }

    /// Returns the privacy-preserving digest for the ACL encoding state and effective grant set.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// Exact access-control evidence for one governed ordinary-EXCLUDE implementation function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureAccessControlObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    access_control: IndexExclusionConstraintOperatorProcedureAccessControlMaterial,
}

impl IndexExclusionConstraintOperatorProcedureAccessControlObservation {
    /// Records one exact operator/function binding and its same-row function access-control identity.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        access_control: IndexExclusionConstraintOperatorProcedureAccessControlMaterial,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            access_control,
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

    /// Returns the privacy-preserving access-control material for the exact implementation function.
    #[must_use]
    pub const fn access_control(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureAccessControlMaterial {
        &self.access_control
    }

    /// Returns the collision-safe evidence location for this function access-control fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_access_control_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-function access-control observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureAccessControlSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureAccessControlObservation,
}

impl IndexExclusionConstraintOperatorProcedureAccessControlSourceReceipt {
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

    /// Returns the owner-computed access-control successor digest.
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

    /// Returns the exact validated access-control observation without exposing role ACL plaintext.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureAccessControlObservation {
        &self.location
    }
}

/// Complete `pg_proc.proacl` evidence over one exact function-configuration predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureAccessControlSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureAccessControlObservation>,
}

impl IndexExclusionConstraintOperatorProcedureAccessControlSnapshot {
    /// Creates complete function access-control evidence over one exact configuration predecessor.
    pub fn new(
        configuration_snapshot: &IndexExclusionConstraintOperatorProcedureConfigurationSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureAccessControlObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = configuration_snapshot
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
                "index_exclusion_constraint_operator_procedure_access_control_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_access_control_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = configuration_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_access_control_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_access_control_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_access_control_digest(
            configuration_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: configuration_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: configuration_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: configuration_snapshot.extractor_revision().to_owned(),
            observed_at_utc: configuration_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-function access-control successor digest.
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

    /// Returns complete access-control observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureAccessControlObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed implementation-function access-control identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureAccessControlSourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_access_control_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureAccessControlSourceReceipt {
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

fn compute_procedure_access_control_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureAccessControlObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        encode_str(&mut hasher, observation.access_control().digest());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_access_control_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-access-control",
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

fn encode_type(hasher: &mut Sha256, qualified_type: &conceptweave_observation::QualifiedTypeName) {
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
