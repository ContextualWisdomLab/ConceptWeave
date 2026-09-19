//! Deterministic recovery validation for transform-converter initial privileges.
//!
//! Source observation deliberately preserves damaged `pg_init_privs` ACL role references instead of
//! dropping them or pretending that raw OIDs are role names. Publication and recovery decisions need
//! a separate fail-closed domain boundary over that immutable evidence. Validation therefore consumes
//! an owner-issued source receipt rather than detached ACL material: every verdict is bound to the
//! exact source-generation digest and canonical converter location that were validated. Raw dangling
//! role OIDs remain private inside Source Observation identity.

use super::IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt;

/// Recovery-validation evidence for one exact converter-function initial-privilege observation.
///
/// Consumers cannot construct this value directly. They must obtain it from
/// [`validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery`],
/// which consumes an owner-issued source receipt. The source digest binds the observation generation,
/// while the canonical location prevents byte-identical ACL material (or row absence) from being
/// replayed as validation evidence for another converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    source_digest: String,
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

    /// Returns the immutable source-snapshot digest whose observation was validated.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the collision-safe converter observation location validated by this verdict.
    #[must_use]
    pub fn canonical_location(&self) -> &str {
        &self.canonical_location
    }

    /// Returns the immutable source-material digest validated by this verdict, if a row was present.
    ///
    /// `None` means PostgreSQL had no `pg_init_privs` material at this exact source-bound location.
    /// Absence is still provenance-bound by [`Self::source_digest`] plus [`Self::canonical_location`].
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
}

/// Validates whether one exact owner-issued converter initial-privilege observation is recovery-ready.
///
/// PostgreSQL permits no `pg_init_privs` row for an object, so absence is not itself damage. A
/// present row is blocked whenever the same-generation role lookup left any nonzero ACL grantee or
/// grantor OID unresolved. The verdict always carries the receipt's source digest and canonical
/// observation location; present rows additionally carry the exact material digest. This keeps the
/// validation evidence replay-resistant without changing Source Observation identity.
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
        source_digest: receipt.source_digest().to_owned(),
        canonical_location: observation.canonical_location(),
        material_digest,
        unresolved_grantee_count,
        unresolved_grantor_count,
    }
}
