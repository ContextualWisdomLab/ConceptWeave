//! Original initial privilege evidence, separate from current ACL and application authorization.

use crate::{
    ObservationError, ProcedureAclItem, ReferencedProcedureDefinitionObservation,
    ReferencedProcedureLocation, encode_bytes, encode_len, encode_sha256, encode_str,
};
use sha2::{Digest, Sha256};
use std::fmt;

/// PostgreSQL's recorded origin of an object's initial privileges.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcedureInitialPrivilegeOrigin {
    /// Privileges recorded while initializing the database (native token i).
    Initialization,
    /// Privileges recorded while creating an extension (native token e).
    Extension,
}

/// One present initial-privilege row, preserving original ACL item order and duplicates.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcedureInitialPrivileges {
    origin: ProcedureInitialPrivilegeOrigin,
    raw_acl: Vec<ProcedureAclItem>,
}
impl fmt::Debug for ProcedureInitialPrivileges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcedureInitialPrivileges")
            .field("origin", &self.origin)
            .field("raw_acl", &"<redacted>")
            .finish()
    }
}
impl ProcedureInitialPrivileges {
    /// Records a present source row; an empty ACL remains distinct from no initial-privilege row.
    pub fn new(origin: ProcedureInitialPrivilegeOrigin, raw_acl: Vec<ProcedureAclItem>) -> Self {
        Self { origin, raw_acl }
    }
    /// Returns the recorded source origin, without treating it as a trust decision.
    #[must_use]
    pub const fn origin(&self) -> ProcedureInitialPrivilegeOrigin {
        self.origin
    }
    /// Returns the unchanged original ACL items.
    #[must_use]
    pub fn raw_acl(&self) -> &[ProcedureAclItem] {
        &self.raw_acl
    }
}

/// Complete initial-privilege evidence for one exact overloaded procedure signature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcedureInitialPrivilegesObservation {
    location: ReferencedProcedureLocation,
    initial_privileges: Option<ProcedureInitialPrivileges>,
}
impl ProcedureInitialPrivilegesObservation {
    /// Records observed row absence with None, or the exact present source material.
    pub fn new(
        location: ReferencedProcedureLocation,
        initial_privileges: Option<ProcedureInitialPrivileges>,
    ) -> Self {
        Self {
            location,
            initial_privileges,
        }
    }
    /// Returns the exact overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
    /// Returns the observed row, or None for observed catalog absence, not unobserved inventory.
    #[must_use]
    pub const fn initial_privileges(&self) -> Option<&ProcedureInitialPrivileges> {
        self.initial_privileges.as_ref()
    }
}

pub(crate) fn canonicalize(
    definitions: &[ReferencedProcedureDefinitionObservation],
    mut items: Vec<ProcedureInitialPrivilegesObservation>,
) -> Result<Vec<ProcedureInitialPrivilegesObservation>, ObservationError> {
    items.sort_by_cached_key(|item| item.location.canonical_location());
    if items.len() != definitions.len()
        || items
            .iter()
            .zip(definitions)
            .any(|(item, definition)| item.location() != definition.location())
    {
        return Err(ObservationError::InvalidObservationField {
            field: "procedure_initial_privileges_coverage",
        });
    }
    Ok(items)
}
pub(crate) fn digest(base: &str, items: &[ProcedureInitialPrivilegesObservation]) -> String {
    let mut hash = Sha256::new();
    encode_bytes(
        &mut hash,
        b"conceptweave.postgres_schema_snapshot.v3.procedure_initial_privileges.v1",
    );
    encode_str(&mut hash, base);
    encode_len(&mut hash, items.len());
    for item in items {
        encode_str(&mut hash, &item.location.canonical_location());
        hash.update([u8::from(item.initial_privileges.is_some())]);
        if let Some(material) = &item.initial_privileges {
            hash.update([match material.origin {
                ProcedureInitialPrivilegeOrigin::Initialization => b'i',
                ProcedureInitialPrivilegeOrigin::Extension => b'e',
            }]);
            crate::procedure_access_control::encode_acl(&mut hash, &material.raw_acl);
        }
    }
    encode_sha256(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initial_rows_keep_origin_absence_and_original_acl_order() {
        let location = ReferencedProcedureLocation::new("pg_catalog", "f", vec![]).unwrap();
        let grant = ProcedureAclItem::new(4000000123, None, 42, None, true, true).unwrap();
        let original = vec![grant.clone(), grant];
        let material = ProcedureInitialPrivileges::new(
            ProcedureInitialPrivilegeOrigin::Extension,
            original.clone(),
        );
        assert_eq!(
            material.origin(),
            ProcedureInitialPrivilegeOrigin::Extension
        );
        assert_eq!(material.raw_acl(), original.as_slice());
        assert!(!format!("{material:?}").contains("4000000123"));
        let present = ProcedureInitialPrivilegesObservation::new(location.clone(), Some(material));
        assert_eq!(present.location(), &location);
        assert!(present.initial_privileges().is_some());
        let absent = ProcedureInitialPrivilegesObservation::new(location.clone(), None);
        assert!(absent.initial_privileges().is_none());
        let empty = ProcedureInitialPrivilegesObservation::new(
            location.clone(),
            Some(ProcedureInitialPrivileges::new(
                ProcedureInitialPrivilegeOrigin::Extension,
                vec![],
            )),
        );
        assert_ne!(
            digest("base", &[absent]),
            digest("base", std::slice::from_ref(&empty))
        );
        let initialized = ProcedureInitialPrivilegesObservation::new(
            location.clone(),
            Some(ProcedureInitialPrivileges::new(
                ProcedureInitialPrivilegeOrigin::Initialization,
                vec![],
            )),
        );
        assert_ne!(digest("base", &[empty]), digest("base", &[initialized]));
        let definition = ReferencedProcedureDefinitionObservation::new(
            location,
            crate::QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            42,
            "owner",
            "definition",
        )
        .unwrap();
        let definitions = [definition];
        assert!(canonicalize(&definitions, vec![]).is_err());
        assert!(canonicalize(&definitions, vec![present.clone(), present.clone()]).is_err());
        let wrong = ProcedureInitialPrivilegesObservation::new(
            ReferencedProcedureLocation::new("pg_catalog", "g", vec![]).unwrap(),
            None,
        );
        assert!(canonicalize(&definitions, vec![wrong]).is_err());
        assert_eq!(
            canonicalize(&definitions, vec![present.clone()]).unwrap(),
            vec![present]
        );
        assert!(canonicalize(&[], vec![]).unwrap().is_empty());
    }
}
