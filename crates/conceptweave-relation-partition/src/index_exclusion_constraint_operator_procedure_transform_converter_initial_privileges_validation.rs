//! Deterministic recovery validation for transform-converter initial privileges.
//!
//! Source observation deliberately preserves damaged `pg_init_privs` ACL role references instead of
//! dropping them or pretending that raw OIDs are role names. Publication and recovery decisions need
//! a separate fail-closed domain boundary over that immutable evidence. Validation therefore consumes
//! an owner-issued source receipt, retains the receipt's complete non-secret provenance binding, and
//! carries exact dangling-role identities for deterministic remediation. Routine `Debug` output
//! deliberately redacts those raw catalog identifiers.

use std::fmt;

use super::IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt;

/// Recovery-validation evidence for one exact converter-function initial-privilege observation.
///
/// Consumers cannot construct this value directly. They must obtain it from
/// [`validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery`],
/// which consumes an owner-issued source receipt. Receipt identity is preserved separately from ACL
/// material identity so equal content observed under a different source/policy/extractor/time epoch
/// cannot silently reuse validation evidence. Exact unresolved role OIDs remain typed evidence for
/// repair tooling while routine debug formatting exposes only their counts.
#[derive(Clone, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation
{
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    canonical_location: String,
    material_digest: Option<String>,
    unresolved_grantee_oids: Vec<u32>,
    unresolved_grantor_oids: Vec<u32>,
}

impl fmt::Debug
    for IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(
                "IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation",
            )
            .field("source_id", &self.source_id)
            .field("connection_policy_binding", &self.connection_policy_binding)
            .field("source_digest", &self.source_digest)
            .field("extractor_revision", &self.extractor_revision)
            .field("observed_at_utc", &self.observed_at_utc)
            .field("canonical_location", &self.canonical_location)
            .field("material_digest", &self.material_digest)
            .field("unresolved_grantee_count", &self.unresolved_grantee_oids.len())
            .field("unresolved_grantor_count", &self.unresolved_grantor_oids.len())
            .finish()
    }
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    /// Returns whether this exact observed baseline is safe to advance through the recovery gate.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.unresolved_grantee_oids.is_empty() && self.unresolved_grantor_oids.is_empty()
    }

    /// Returns the stable source registry identity copied from the owner-issued receipt.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable source-policy binding copied from the owner-issued receipt.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the immutable source-snapshot content digest copied from the owner-issued receipt.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision copied from the owner-issued receipt.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time copied from the owner-issued receipt.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the collision-safe converter observation location validated by this verdict.
    #[must_use]
    pub fn canonical_location(&self) -> &str {
        &self.canonical_location
    }

    /// Returns the immutable source-material digest validated by this verdict, if a row was present.
    ///
    /// `None` means PostgreSQL had no `pg_init_privs` material at this exact receipt-bound location.
    #[must_use]
    pub fn material_digest(&self) -> Option<&str> {
        self.material_digest.as_deref()
    }

    /// Returns how many distinct dangling grantee role identities require remediation.
    #[must_use]
    pub fn unresolved_grantee_count(&self) -> usize {
        self.unresolved_grantee_oids.len()
    }

    /// Returns how many distinct dangling grantor role identities require remediation.
    #[must_use]
    pub fn unresolved_grantor_count(&self) -> usize {
        self.unresolved_grantor_oids.len()
    }

    /// Returns canonical unresolved grantee role OIDs for exact recovery remediation.
    ///
    /// These are PostgreSQL catalog identifiers, not credentials. They are intentionally omitted
    /// from routine [`fmt::Debug`] output but retained here so repair/review does not have to guess
    /// from aggregate counts or reverse an opaque material digest.
    #[must_use]
    pub fn unresolved_grantee_oids(&self) -> &[u32] {
        &self.unresolved_grantee_oids
    }

    /// Returns canonical unresolved grantor role OIDs for exact recovery remediation.
    ///
    /// These are PostgreSQL catalog identifiers, not credentials. They are intentionally omitted
    /// from routine [`fmt::Debug`] output but retained here so repair/review does not have to guess
    /// from aggregate counts or reverse an opaque material digest.
    #[must_use]
    pub fn unresolved_grantor_oids(&self) -> &[u32] {
        &self.unresolved_grantor_oids
    }

    /// Verifies that this verdict belongs to the exact owner-issued receipt presented by a caller.
    ///
    /// The comparison includes source registry identity, policy binding, source-content digest,
    /// extractor revision, observation time, canonical converter location, row absence/presence
    /// material identity, and exact dangling-role identities.
    #[must_use]
    pub fn matches_source_receipt(
        &self,
        receipt: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
    ) -> bool {
        let observation = receipt.location();
        let (material_digest, unresolved_grantee_oids, unresolved_grantor_oids) =
            match observation.initial_privileges() {
                None => (None, &[][..], &[][..]),
                Some(material) => (
                    Some(material.digest()),
                    material.unresolved_grantee_oids(),
                    material.unresolved_grantor_oids(),
                ),
            };

        self.source_id == receipt.source_id()
            && self.connection_policy_binding == receipt.connection_policy_binding()
            && self.source_digest == receipt.source_digest()
            && self.extractor_revision == receipt.extractor_revision()
            && self.observed_at_utc == receipt.observed_at_utc()
            && self.canonical_location == observation.canonical_location()
            && self.material_digest.as_deref() == material_digest
            && self.unresolved_grantee_oids.as_slice() == unresolved_grantee_oids
            && self.unresolved_grantor_oids.as_slice() == unresolved_grantor_oids
    }
}

/// Validates whether one exact owner-issued converter initial-privilege observation is recovery-ready.
///
/// PostgreSQL permits no `pg_init_privs` row for an object, so absence is not itself damage. A
/// present row is blocked whenever the same-generation role lookup left any nonzero ACL grantee or
/// grantor OID unresolved. The verdict retains the receipt's complete public provenance binding,
/// exact canonical dangling-role identifiers, and (for present rows) the exact material digest. This
/// keeps validation evidence replay-resistant and remediation-actionable without changing Source
/// Observation digest identity.
#[must_use]
pub fn validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
    receipt: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    let observation = receipt.location();
    let (material_digest, unresolved_grantee_oids, unresolved_grantor_oids) =
        match observation.initial_privileges() {
            None => (None, Vec::new(), Vec::new()),
            Some(material) => (
                Some(material.digest().to_owned()),
                material.unresolved_grantee_oids().to_vec(),
                material.unresolved_grantor_oids().to_vec(),
            ),
        };

    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
        source_id: receipt.source_id().to_owned(),
        connection_policy_binding: receipt.connection_policy_binding().to_owned(),
        source_digest: receipt.source_digest().to_owned(),
        extractor_revision: receipt.extractor_revision().to_owned(),
        observed_at_utc: receipt.observed_at_utc().to_owned(),
        canonical_location: observation.canonical_location(),
        material_digest,
        unresolved_grantee_oids,
        unresolved_grantor_oids,
    }
}
