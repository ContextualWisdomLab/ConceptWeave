//! Deterministic recovery validation for transform-converter initial privileges.
//!
//! Source observation deliberately preserves damaged `pg_init_privs` ACL role references instead of
//! dropping them or pretending that raw OIDs are role names. Publication and recovery decisions need
//! a separate fail-closed domain boundary over that immutable evidence. This module consumes only the
//! privacy-preserving unresolved-reference counts already carried by the observation material; it does
//! not expose raw OIDs or mutate PostgreSQL catalog state. The validation result is non-forgeable by
//! consumers and, for a present row, is bound to the exact immutable material digest it validated.

use super::IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial;

/// Recovery-validation evidence for one observed converter-function initial-privilege baseline.
///
/// Consumers cannot construct this value directly. They must obtain it from
/// [`validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery`],
/// which binds every present-material verdict to that material's immutable digest. This prevents a
/// successful verdict from being minted without validation or replayed as evidence for a different
/// initial-privilege baseline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
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

    /// Returns the immutable source-material digest validated by this verdict, if a row was present.
    ///
    /// `None` means PostgreSQL had no `pg_init_privs` material for the converter function. A present
    /// material always returns its exact digest, including when unresolved role references block it.
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

/// Validates whether one exact converter-function initial-privilege baseline is recovery-ready.
///
/// PostgreSQL permits no `pg_init_privs` row for an object, so absence is not itself damage. A
/// present row is blocked whenever the same-generation role lookup left any nonzero ACL grantee or
/// grantor OID unresolved. The source observation remains immutable either way; this derived result
/// carries the exact material digest for evidence binding but does not participate in source identity.
#[must_use]
pub fn validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
    material: Option<
        &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial,
    >,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    match material {
        None => IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
            material_digest: None,
            unresolved_grantee_count: 0,
            unresolved_grantor_count: 0,
        },
        Some(material) => {
            IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
                material_digest: Some(material.digest().to_owned()),
                unresolved_grantee_count: material.unresolved_grantee_count(),
                unresolved_grantor_count: material.unresolved_grantor_count(),
            }
        }
    }
}
