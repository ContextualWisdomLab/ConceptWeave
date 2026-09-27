//! Exact source ACL items, distinct from application authorization or semantic authority.

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, ReferencedProcedureDefinitionObservation, ReferencedProcedureLocation,
    encode_bytes, encode_len, encode_sha256, encode_str,
};
use sha2::{Digest, Sha256};
use std::fmt;

/// One original function ACL item, including an item with no privileges.
/// Role OIDs are source identity; optional names only enrich that identity.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcedureAclItem {
    grantee_oid: u32,
    grantee_name: Option<String>,
    grantor_oid: u32,
    grantor_name: Option<String>,
    execute: bool,
    grant_option: bool,
}

impl fmt::Debug for ProcedureAclItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcedureAclItem").finish_non_exhaustive()
    }
}

impl ProcedureAclItem {
    /// Records raw role identities and EXECUTE bits; grantee zero is PUBLIC, not a named role.
    pub fn new(
        grantee_oid: u32,
        grantee_name: Option<String>,
        grantor_oid: u32,
        grantor_name: Option<String>,
        execute: bool,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        if grantor_oid == 0
            || (grantee_oid == 0 && grantee_name.is_some())
            || (grant_option && !execute)
        {
            return Err(invalid("procedure_acl_item"));
        }
        for name in [&grantee_name, &grantor_name].into_iter().flatten() {
            validate_postgresql_identifier(name, "procedure_acl_role_name")?;
        }
        Ok(Self {
            grantee_oid,
            grantee_name,
            grantor_oid,
            grantor_name,
            execute,
            grant_option,
        })
    }
    /// Returns the original grantee OID, with zero reserved for PUBLIC.
    #[must_use]
    pub const fn grantee_oid(&self) -> u32 {
        self.grantee_oid
    }
    /// Returns a resolved role name, or None for PUBLIC or a dangling role OID.
    #[must_use]
    pub fn grantee_name(&self) -> Option<&str> {
        self.grantee_name.as_deref()
    }
    /// Returns the original grantor OID.
    #[must_use]
    pub const fn grantor_oid(&self) -> u32 {
        self.grantor_oid
    }
    /// Returns the resolved grantor name, or None for a dangling OID.
    #[must_use]
    pub fn grantor_name(&self) -> Option<&str> {
        self.grantor_name.as_deref()
    }
    /// Returns the observed EXECUTE privilege bit, without granting application permissions.
    #[must_use]
    pub const fn execute(&self) -> bool {
        self.execute
    }
    /// Returns the observed EXECUTE grant-option bit.
    #[must_use]
    pub const fn grant_option(&self) -> bool {
        self.grant_option
    }
}

/// Exact catalog ACL state for one qualified procedure signature.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcedureAccessControlObservation {
    location: ReferencedProcedureLocation,
    raw_acl: Option<Vec<ProcedureAclItem>>,
}

impl fmt::Debug for ProcedureAccessControlObservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcedureAccessControlObservation")
            .field("location", &self.location)
            .field("raw_acl", &"<redacted>")
            .finish()
    }
}

impl ProcedureAccessControlObservation {
    /// Records catalog NULL separately from an explicit ACL, preserving item order and duplicates.
    pub fn new(
        location: ReferencedProcedureLocation,
        raw_acl: Option<Vec<ProcedureAclItem>>,
    ) -> Self {
        Self { location, raw_acl }
    }
    /// Returns the exact overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
    /// Returns the original ACL items; None means observed catalog NULL, not unobserved evidence.
    #[must_use]
    pub fn raw_acl(&self) -> Option<&[ProcedureAclItem]> {
        self.raw_acl.as_deref()
    }
}

pub(crate) fn canonicalize(
    definitions: &[ReferencedProcedureDefinitionObservation],
    mut items: Vec<ProcedureAccessControlObservation>,
) -> Result<Vec<ProcedureAccessControlObservation>, ObservationError> {
    items.sort_by_cached_key(|item| item.location.canonical_location());
    if items.len() != definitions.len()
        || items
            .iter()
            .zip(definitions)
            .any(|(item, definition)| item.location() != definition.location())
    {
        return Err(invalid("procedure_access_control_coverage"));
    }
    Ok(items)
}

pub(crate) fn digest(base: &str, items: &[ProcedureAccessControlObservation]) -> String {
    let mut hash = Sha256::new();
    encode_bytes(
        &mut hash,
        b"conceptweave.postgres_schema_snapshot.v3.procedure_access_control.v1",
    );
    encode_str(&mut hash, base);
    encode_len(&mut hash, items.len());
    for item in items {
        encode_str(&mut hash, &item.location.canonical_location());
        hash.update([u8::from(item.raw_acl.is_some())]);
        if let Some(acl) = &item.raw_acl {
            encode_len(&mut hash, acl.len());
            for grant in acl {
                encode_role(&mut hash, grant.grantee_oid, grant.grantee_name.as_deref());
                encode_role(&mut hash, grant.grantor_oid, grant.grantor_name.as_deref());
                hash.update([u8::from(grant.execute), u8::from(grant.grant_option)]);
            }
        }
    }
    encode_sha256(hash)
}

fn encode_role(hash: &mut Sha256, oid: u32, name: Option<&str>) {
    hash.update(oid.to_be_bytes());
    hash.update([u8::from(name.is_some())]);
    if let Some(name) = name {
        encode_str(hash, name);
    }
}
fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn acl_items_keep_dangling_identities_and_reject_impossible_role_bits() {
        let public = ProcedureAclItem::new(0, None, 42, Some(" ".into()), false, false).unwrap();
        assert_eq!(public.grantee_oid(), 0);
        assert_eq!(public.grantee_name(), None);
        assert_eq!(public.grantor_name(), Some(" "));
        let dangling = ProcedureAclItem::new(100, None, 200, None, true, true).unwrap();
        assert_eq!(dangling.grantee_oid(), 100);
        assert_eq!(dangling.grantor_oid(), 200);
        assert_eq!(dangling.grantor_name(), None);
        assert!(dangling.execute() && dangling.grant_option());
        assert!(!format!("{dangling:?}").contains("100"));
        for (grantee, name, grantor, grantor_name, execute, option) in [
            (0, Some("PUBLIC".to_owned()), 42, None, true, false),
            (1, None, 0, None, true, false),
            (1, None, 42, None, false, true),
            (1, Some(String::new()), 42, None, true, false),
            (1, None, 42, Some("bad\0".into()), true, false),
        ] {
            assert!(
                ProcedureAclItem::new(grantee, name, grantor, grantor_name, execute, option)
                    .is_err()
            );
        }
        let location = ReferencedProcedureLocation::new("pg_catalog", "f", vec![]).unwrap();
        let original = vec![public.clone(), dangling, public];
        let record =
            ProcedureAccessControlObservation::new(location.clone(), Some(original.clone()));
        assert_eq!(record.raw_acl(), Some(original.as_slice()));
        assert!(!format!("{record:?}").contains("200"));
        assert_eq!(
            ProcedureAccessControlObservation::new(location.clone(), None).raw_acl(),
            None
        );
        assert_eq!(
            ProcedureAccessControlObservation::new(location, Some(vec![])).raw_acl(),
            Some([].as_slice())
        );
    }
}
