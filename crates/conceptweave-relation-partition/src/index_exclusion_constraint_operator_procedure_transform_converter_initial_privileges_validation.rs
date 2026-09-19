//! Deterministic recovery validation for transform-converter initial privileges.
//!
//! Source observation deliberately preserves damaged `pg_init_privs` ACL role references instead of
//! dropping them or pretending that raw OIDs are role names. Publication and recovery decisions need
//! a separate fail-closed domain boundary over that immutable evidence. This module consumes only the
//! privacy-preserving unresolved-reference counts already carried by the observation material; it does
//! not reinterpret the digest, expose raw OIDs, or mutate PostgreSQL catalog state.

use super::IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial;

/// Recovery-readiness verdict for one observed converter-function initial-privilege baseline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    /// Row absence or a present baseline whose role references all resolve in the source generation.
    Ready,
    /// The immutable baseline contains unresolved role references and must not be published as ready.
    Blocked {
        /// Number of initial EXECUTE grants whose non-PUBLIC grantee OID did not resolve.
        unresolved_grantee_count: usize,
        /// Number of initial EXECUTE grants whose grantor OID did not resolve.
        unresolved_grantor_count: usize,
    },
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    /// Returns whether this observation is safe to advance through the recovery-publication gate.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns the aggregate unresolved-grantee diagnostic without exposing raw role OIDs.
    #[must_use]
    pub const fn unresolved_grantee_count(self) -> usize {
        match self {
            Self::Ready => 0,
            Self::Blocked {
                unresolved_grantee_count,
                ..
            } => unresolved_grantee_count,
        }
    }

    /// Returns the aggregate unresolved-grantor diagnostic without exposing raw role OIDs.
    #[must_use]
    pub const fn unresolved_grantor_count(self) -> usize {
        match self {
            Self::Ready => 0,
            Self::Blocked {
                unresolved_grantor_count,
                ..
            } => unresolved_grantor_count,
        }
    }
}

/// Validates whether one exact converter-function initial-privilege baseline is recovery-ready.
///
/// PostgreSQL permits no `pg_init_privs` row for an object, so absence is not itself damage. A
/// present row is blocked whenever the same-generation role lookup left any nonzero ACL grantee or
/// grantor OID unresolved. The source observation remains immutable either way; this verdict is a
/// derived validation result and therefore does not participate in source identity.
#[must_use]
pub fn validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
    material: Option<
        &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial,
    >,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation {
    let Some(material) = material else {
        return IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Ready;
    };

    let unresolved_grantee_count = material.unresolved_grantee_count();
    let unresolved_grantor_count = material.unresolved_grantor_count();
    if unresolved_grantee_count == 0 && unresolved_grantor_count == 0 {
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Ready
    } else {
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Blocked {
            unresolved_grantee_count,
            unresolved_grantor_count,
        }
    }
}
