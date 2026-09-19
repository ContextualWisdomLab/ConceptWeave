//! PostgreSQL ordinary exclusion-constraint transform-converter initial-privilege evidence.
//!
//! Current `pg_proc.proacl` does not preserve the privilege baseline used by PostgreSQL extension
//! dump/restore. `pg_init_privs` records non-default initial privileges set by `initdb` or an
//! extension script. PostgreSQL `pg_dump` compares current privileges with that baseline when it
//! emits the GRANT/REVOKE state needed to reconstruct extension objects. PostgreSQL also treats ACL
//! array order as recovery-significant because grants with grant option must precede dependent
//! grants, so this successor preserves the exact source-array order instead of normalizing it.
//! This successor therefore preserves row absence versus presence, exact `privtype`, and the complete
//! object-level EXECUTE ACL, including repeated source entries, for every exact converter-function
//! direction without inferring the baseline from current ACL, extension membership, package state,
//! or names. Existing catalog damage can leave ACL grantor or grantee OIDs dangling after a role
//! disappears, so unresolved raw OIDs remain distinct from resolved role names instead of making the
//! source observation itself impossible.

use std::{collections::BTreeSet, fmt};

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_INITIAL_PRIVILEGE_MATERIAL_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.initial_privilege.material.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_INITIAL_PRIVILEGE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.initial_privilege.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// PostgreSQL `pg_init_privs.privtype` for a converter function.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType {
    /// Initial privileges installed by `initdb` (`privtype = 'i'`).
    Initdb,
    /// Initial privileges installed while creating an extension (`privtype = 'e'`).
    Extension,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType {
    const fn token(self) -> &'static str {
        match self {
            Self::Initdb => "i",
            Self::Extension => "e",
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum TransformConverterInitialExecuteGrantee {
    Public,
    Role(String),
    UnresolvedRoleOid(u32),
}

impl fmt::Debug for TransformConverterInitialExecuteGrantee {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => formatter.write_str("Public"),
            Self::Role(role_name) => formatter.debug_tuple("Role").field(role_name).finish(),
            Self::UnresolvedRoleOid(_) => formatter.write_str("UnresolvedRoleOid(<redacted>)"),
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum TransformConverterInitialExecuteGrantor {
    Role(String),
    UnresolvedRoleOid(u32),
}

impl fmt::Debug for TransformConverterInitialExecuteGrantor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Role(role_name) => formatter.debug_tuple("Role").field(role_name).finish(),
            Self::UnresolvedRoleOid(_) => formatter.write_str("UnresolvedRoleOid(<redacted>)"),
        }
    }
}

/// One object-level `EXECUTE` ACL entry from converter-function `pg_init_privs.initprivs`.
///
/// Routine `Debug` output preserves grant shape and resolved role names but never renders unresolved
/// raw role OIDs. Exact unresolved identifiers remain available only through the purpose-bound
/// material and recovery-validation path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant {
    grantee: TransformConverterInitialExecuteGrantee,
    grantor: TransformConverterInitialExecuteGrantor,
    grant_option: bool,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant {
    /// Records an initial `EXECUTE` grant to `PUBLIC` with its exact resolved grantor role.
    pub fn public(
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_name",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::Public,
            grantor: TransformConverterInitialExecuteGrantor::Role(grantor_role_name),
            grant_option,
        })
    }

    /// Records a `PUBLIC` grant whose raw grantor OID no longer resolves to a role.
    pub fn public_with_unresolved_grantor_oid(
        grantor_role_oid: u32,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        validate_nonzero_role_oid(
            grantor_role_oid,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_oid",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::Public,
            grantor: TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(grantor_role_oid),
            grant_option,
        })
    }

    /// Records an initial `EXECUTE` grant to one resolved role with its exact resolved grantor role.
    pub fn role(
        grantee_role_name: impl Into<String>,
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantee_role_name = grantee_role_name.into();
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantee_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_name",
        )?;
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_name",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::Role(grantee_role_name),
            grantor: TransformConverterInitialExecuteGrantor::Role(grantor_role_name),
            grant_option,
        })
    }

    /// Records a resolved grantee whose raw grantor OID no longer resolves to a role.
    pub fn role_with_unresolved_grantor_oid(
        grantee_role_name: impl Into<String>,
        grantor_role_oid: u32,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantee_role_name = grantee_role_name.into();
        validate_nonblank(
            &grantee_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_name",
        )?;
        validate_nonzero_role_oid(
            grantor_role_oid,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_oid",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::Role(grantee_role_name),
            grantor: TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(grantor_role_oid),
            grant_option,
        })
    }

    /// Records an unresolved raw grantee OID with an exact resolved grantor role.
    pub fn unresolved_grantee_oid(
        grantee_role_oid: u32,
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        validate_nonzero_role_oid(
            grantee_role_oid,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_oid",
        )?;
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_name",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(grantee_role_oid),
            grantor: TransformConverterInitialExecuteGrantor::Role(grantor_role_name),
            grant_option,
        })
    }

    /// Records unresolved raw grantee and grantor OIDs without converting either OID to a name.
    pub fn unresolved_role_oids(
        grantee_role_oid: u32,
        grantor_role_oid: u32,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        validate_nonzero_role_oid(
            grantee_role_oid,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_oid",
        )?;
        validate_nonzero_role_oid(
            grantor_role_oid,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_oid",
        )?;
        Ok(Self {
            grantee: TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(grantee_role_oid),
            grantor: TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(grantor_role_oid),
            grant_option,
        })
    }

    /// Returns whether this grant targets PostgreSQL `PUBLIC`.
    #[must_use]
    pub fn is_public_grantee(&self) -> bool {
        matches!(self.grantee, TransformConverterInitialExecuteGrantee::Public)
    }

    /// Returns the resolved grantee role name, when this is a role grant with a live role lookup.
    #[must_use]
    pub fn resolved_grantee_role_name(&self) -> Option<&str> {
        match &self.grantee {
            TransformConverterInitialExecuteGrantee::Role(role_name) => Some(role_name),
            TransformConverterInitialExecuteGrantee::Public
            | TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(_) => None,
        }
    }

    /// Returns whether this grant retains a dangling non-PUBLIC grantee role identity.
    #[must_use]
    pub fn has_unresolved_grantee(&self) -> bool {
        matches!(
            self.grantee,
            TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(_)
        )
    }

    /// Returns the resolved grantor role name when the same-generation role lookup succeeded.
    #[must_use]
    pub fn resolved_grantor_role_name(&self) -> Option<&str> {
        match &self.grantor {
            TransformConverterInitialExecuteGrantor::Role(role_name) => Some(role_name),
            TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(_) => None,
        }
    }

    /// Returns whether this grant retains a dangling grantor role identity.
    #[must_use]
    pub fn has_unresolved_grantor(&self) -> bool {
        matches!(
            self.grantor,
            TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(_)
        )
    }

    /// Returns whether this initial EXECUTE ACL entry carries grant option.
    #[must_use]
    pub const fn grant_option(&self) -> bool {
        self.grant_option
    }
}

/// Privacy-conscious identity for one present converter-function `pg_init_privs` row.
///
/// The adapter should resolve non-PUBLIC grantor/grantee OIDs against the same source generation.
/// When a raw ACL OID has no matching role, it must preserve that nonzero OID explicitly instead of
/// dropping the entry or converting the OID to a role-name string. Row absence is represented by
/// `None` at the observation boundary, not by an empty material. PostgreSQL ACL array order and
/// multiplicity are kept byte-semantically significant because source observation must remain
/// lossless even when a catalog contains repeated entries. Exact unresolved role identifiers remain
/// privately retained for the recovery-validation boundary while routine `Debug` output shows only
/// aggregate counts and the immutable material digest.
#[derive(Clone, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial {
    privilege_type: IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType,
    grants: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant>,
    grant_count: usize,
    unresolved_grantee_oids: Vec<u32>,
    unresolved_grantor_oids: Vec<u32>,
    digest: String,
}

impl fmt::Debug for IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(
                "IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial",
            )
            .field("privilege_type", &self.privilege_type)
            .field("grant_count", &self.grant_count)
            .field("unresolved_grantee_count", &self.unresolved_grantee_oids.len())
            .field("unresolved_grantor_count", &self.unresolved_grantor_oids.len())
            .field("digest", &self.digest)
            .finish()
    }
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial {
    /// Reduces exact `privtype` and source-order object-level EXECUTE grants to a stable digest.
    pub fn new(
        privilege_type: IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType,
        grants: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant>,
    ) -> Result<Self, ObservationError> {
        let unresolved_grantee_oids = grants
            .iter()
            .filter_map(|grant| match &grant.grantee {
                TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(role_oid) => {
                    Some(*role_oid)
                }
                TransformConverterInitialExecuteGrantee::Public
                | TransformConverterInitialExecuteGrantee::Role(_) => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let unresolved_grantor_oids = grants
            .iter()
            .filter_map(|grant| match &grant.grantor {
                TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(role_oid) => {
                    Some(*role_oid)
                }
                TransformConverterInitialExecuteGrantor::Role(_) => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let mut hasher = Sha256::new();
        hasher.update(
            INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_INITIAL_PRIVILEGE_MATERIAL_DIGEST_DOMAIN_V1,
        );
        encode_str(&mut hasher, privilege_type.token());
        encode_len(&mut hasher, grants.len());
        for grant in &grants {
            match &grant.grantee {
                TransformConverterInitialExecuteGrantee::Public => hasher.update([0]),
                TransformConverterInitialExecuteGrantee::Role(role_name) => {
                    hasher.update([1]);
                    encode_str(&mut hasher, role_name);
                }
                TransformConverterInitialExecuteGrantee::UnresolvedRoleOid(role_oid) => {
                    hasher.update([2]);
                    hasher.update(role_oid.to_be_bytes());
                }
            }
            match &grant.grantor {
                TransformConverterInitialExecuteGrantor::Role(role_name) => {
                    encode_str(&mut hasher, role_name);
                }
                TransformConverterInitialExecuteGrantor::UnresolvedRoleOid(role_oid) => {
                    // Keep the historical resolved-role framing byte-for-byte stable. `u64::MAX`
                    // cannot be a realizable Rust string length and therefore safely reserves the
                    // grantor namespace for an unresolved raw OID without aliasing a role name.
                    hasher.update(u64::MAX.to_be_bytes());
                    hasher.update(role_oid.to_be_bytes());
                }
            }
            hasher.update([u8::from(grant.grant_option)]);
        }
        let grant_count = grants.len();

        Ok(Self {
            privilege_type,
            grants,
            grant_count,
            unresolved_grantee_oids,
            unresolved_grantor_oids,
            digest: format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize()),
        })
    }

    /// Returns the exact PostgreSQL initial-privilege source type.
    #[must_use]
    pub const fn privilege_type(
        &self,
    ) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType {
        self.privilege_type
    }

    /// Returns the complete object-level EXECUTE ACL in exact PostgreSQL source-array order.
    ///
    /// Source order and multiplicity are intentionally not canonicalized. PostgreSQL restore clients
    /// can rely on grant-option providers appearing before dependent grants, and PostgreSQL's ACL
    /// validator does not impose a uniqueness invariant on the array. Resolved role names, PUBLIC
    /// shape and grant-option state remain directly inspectable. Raw dangling OIDs remain private
    /// here and are exposed only by receipt-bound recovery validation.
    #[must_use]
    pub fn grants(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant] {
        &self.grants
    }

    /// Returns the number of source-order object-level EXECUTE grants consumed into the digest.
    #[must_use]
    pub const fn grant_count(&self) -> usize {
        self.grant_count
    }

    /// Returns how many distinct unresolved non-PUBLIC grantee OIDs were observed.
    #[must_use]
    pub fn unresolved_grantee_count(&self) -> usize {
        self.unresolved_grantee_oids.len()
    }

    /// Returns how many distinct unresolved grantor OIDs were observed.
    #[must_use]
    pub fn unresolved_grantor_count(&self) -> usize {
        self.unresolved_grantor_oids.len()
    }

    /// Returns the exact canonical unresolved grantee OIDs for recovery validation.
    pub(crate) fn unresolved_grantee_oids(&self) -> &[u32] {
        &self.unresolved_grantee_oids
    }

    /// Returns the exact canonical unresolved grantor OIDs for recovery validation.
    pub(crate) fn unresolved_grantor_oids(&self) -> &[u32] {
        &self.unresolved_grantor_oids
    }

    /// Returns the privacy-preserving digest of exact `privtype` plus source-order initial EXECUTE ACL.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// Exact `pg_init_privs` evidence for one nonzero transform-converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    initial_privileges: Option<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation {
    /// Records exact absence or one same-generation converter-function `pg_init_privs` row.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        initial_privileges: Option<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_function_name",
        )?;
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
            initial_privileges,
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

    /// Returns the transform type whose converter function is being described.
    #[must_use]
    pub const fn transform_type(&self) -> &QualifiedTypeName {
        &self.transform_type
    }

    /// Returns whether this is the FROM-SQL or TO-SQL converter.
    #[must_use]
    pub const fn direction(
        &self,
    ) -> IndexExclusionConstraintOperatorProcedureTransformConverterDirection {
        self.direction
    }

    /// Returns the exact converter-function schema.
    #[must_use]
    pub fn converter_schema_name(&self) -> &str {
        &self.converter_schema_name
    }

    /// Returns the exact converter-function name.
    #[must_use]
    pub fn converter_function_name(&self) -> &str {
        &self.converter_function_name
    }

    /// Returns exact row absence or the privacy-preserving present-row material.
    #[must_use]
    pub const fn initial_privileges(
        &self,
    ) -> Option<&IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial> {
        self.initial_privileges.as_ref()
    }

    /// Returns the collision-safe evidence location for this converter recovery fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_initial_privilege_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            self.direction,
        )
    }
}

/// Immutable provenance receipt for one converter-function initial-privilege observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt {
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

    /// Returns the owner-computed converter initial-privilege successor digest.
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

    /// Returns the exact validated converter-function initial-privilege observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation {
        &self.location
    }
}

/// Complete converter-function initial-privilege evidence over security-label evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    converter_snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot {
    /// Creates complete same-generation `pg_init_privs` evidence for every converter direction.
    pub fn new(
        security_label_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by_key(initial_privilege_key);

        let expected = security_label_snapshot
            .observations()
            .iter()
            .map(|observation| {
                initial_privilege_coordinate_key(
                    observation.coordinate(),
                    observation.key_position(),
                    observation.transform_type(),
                    observation.direction(),
                )
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(initial_privilege_key)
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = security_label_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                        && candidate.transform_type() == observation.transform_type()
                        && candidate.direction() == observation.direction()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_completeness",
                    )
                })?;
            if predecessor.converter_schema_name() != observation.converter_schema_name()
                || predecessor.converter_function_name() != observation.converter_function_name()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_converter_initial_privilege_digest(
            security_label_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: security_label_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: security_label_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            converter_snapshot_digest: security_label_snapshot.converter_snapshot_digest().to_owned(),
            extractor_revision: security_label_snapshot.extractor_revision().to_owned(),
            observed_at_utc: security_label_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated converter initial-privilege successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the immutable raw transform-converter root digest.
    #[must_use]
    pub fn converter_snapshot_digest(&self) -> &str {
        &self.converter_snapshot_digest
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

    /// Returns complete observations in deterministic converter-direction order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation] {
        &self.observations
    }

    /// Issues exact provenance for one converter-function initial-privilege identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
        ObservationError,
    > {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
                    && observation.transform_type() == &transform_type
                    && observation.direction() == direction
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_transform_converter_initial_privilege_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    direction,
                ),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ConverterInitialPrivilegeCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
}

fn compute_transform_converter_initial_privilege_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_INITIAL_PRIVILEGE_DIGEST_DOMAIN_V1,
    );
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_type(&mut hasher, observation.transform_type());
        encode_str(&mut hasher, direction_token(observation.direction()));
        encode_str(&mut hasher, observation.converter_schema_name());
        encode_str(&mut hasher, observation.converter_function_name());
        match observation.initial_privileges() {
            None => hasher.update([0]),
            Some(material) => {
                hasher.update([1]);
                encode_str(&mut hasher, material.digest());
            }
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn initial_privilege_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation,
) -> ConverterInitialPrivilegeCoordinateKey {
    initial_privilege_coordinate_key(
        observation.coordinate(),
        observation.key_position(),
        observation.transform_type(),
        observation.direction(),
    )
}

fn initial_privilege_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> ConverterInitialPrivilegeCoordinateKey {
    ConverterInitialPrivilegeCoordinateKey {
        schema_name: coordinate.schema_name().to_owned(),
        relation_name: coordinate.relation_name().to_owned(),
        relation_kind: coordinate.relation_kind().token().to_owned(),
        constraint_name: coordinate.constraint_name().to_owned(),
        key_position,
        transform_type_schema_name: transform_type.schema_name().to_owned(),
        transform_type_name: transform_type.type_name().to_owned(),
        direction,
    }
}

fn procedure_transform_converter_initial_privilege_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters/{transform_schema}.{transform_name}/{}/function-initial-privileges",
        coordinate.canonical_location(),
        direction_token(direction),
    )
}

fn encode_location_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => {
                encoded.push(char::from(byte));
            }
            _ => {
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }
    encoded
}

const fn direction_token(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> &'static str {
    match direction {
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql => "from_sql",
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql => "to_sql",
    }
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn validate_nonzero_role_oid(role_oid: u32, field: &'static str) -> Result<(), ObservationError> {
    if role_oid == 0 {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
