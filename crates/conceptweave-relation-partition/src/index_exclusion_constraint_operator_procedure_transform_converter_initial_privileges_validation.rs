//! Deterministic recovery validation for transform-converter initial privileges.
//!
//! Source observation deliberately preserves damaged `pg_init_privs` ACL role references instead of
//! dropping them or pretending that raw OIDs are role names. Publication and recovery decisions need
//! a separate fail-closed domain boundary over that immutable evidence. Validation therefore consumes
//! an owner-issued source receipt rather than detached ACL material and retains the receipt's complete
//! non-secret provenance binding. Raw dangling role OIDs remain private inside Source Observation
//! identity.

use super::IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt;

/// Recovery-validation evidence for one exact converter-function initial-privilege observation.
///
/// Consumers cannot construct this value directly. They must obtain it from
/// [`validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery`],
/// which consumes an owner-issued source receipt. Receipt identity is preserved separately from ACL
/// material identity so equal content observed under a different source/policy/extractor/time epoch
/// cannot silently reuse validation evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    canonical_location: String,
    material_digest: Option<String>,
    unresolved_grantee_count: usize,
    unresolved_grantor_count: usize,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    /// Returns whether this exact observed baseline is safe to advance through the recovery gate.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.unresolved_grantee_count == 0 && self.unresolved_grantor_count == 0
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

    /// Returns the aggregate unresolved-grantee diagnostic without exposing raw role OIDs.
    #[must_use]
    pub const fn unresolved_grantee_count(&self) -> usize {
        self.unresolved_grantee_count
    }

    /// Returns the aggregate unresolved-grantor diagnostic without exposing raw role OIDs.
    #[must_use]
    pub const fn unresolved_grantor_count(&self) -> usize {
        self.unresolved_grantor_count
    }

    /// Verifies that this verdict belongs to the exact owner-issued receipt presented by a caller.
    ///
    /// The comparison includes source registry identity, policy binding, source-content digest,
    /// extractor revision, observation time, canonical converter location, and row absence/presence
    /// material identity. It deliberately does not expose the raw dangling role OIDs committed inside
    /// the material digest.
    #[must_use]
    pub fn matches_source_receipt(
        &self,
        receipt: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
    ) -> bool {
        let observation = receipt.location();
        let material_digest = observation
            .initial_privileges()
            .map(|material| material.digest());

        self.source_id == receipt.source_id()
            && self.connection_policy_binding == receipt.connection_policy_binding()
            && self.source_digest == receipt.source_digest()
            && self.extractor_revision == receipt.extractor_revision()
            && self.observed_at_utc == receipt.observed_at_utc()
            && self.canonical_location == observation.canonical_location()
            && self.material_digest.as_deref() == material_digest
    }
}

/// Validates whether one exact owner-issued converter initial-privilege observation is recovery-ready.
///
/// PostgreSQL permits no `pg_init_privs` row for an object, so absence is not itself damage. A
/// present row is blocked whenever the same-generation role lookup left any nonzero ACL grantee or
/// grantor OID unresolved. The verdict retains the receipt's complete public provenance binding;
/// present rows additionally carry the exact material digest. This keeps validation evidence
/// replay-resistant without changing Source Observation identity.
#[must_use]
pub fn validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
    receipt: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    let observation = receipt.location();
    let (material_digest, unresolved_grantee_count, unresolved_grantor_count) =
        match observation.initial_privileges() {
            None => (None, 0, 0),
            Some(material) => (
                Some(material.digest().to_owned()),
                material.unresolved_grantee_count(),
                material.unresolved_grantor_count(),
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
        unresolved_grantee_count,
        unresolved_grantor_count,
    }
}
