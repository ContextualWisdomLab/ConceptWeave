//! Exact PostgreSQL role identity for each observed relation owner.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, RelationObservation, encode_bytes, encode_len, encode_sha256, encode_str,
};

const DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_owner.v1";

/// Exact same-generation owner role OID and name for one schema-local relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationOwnerObservation {
    schema_name: String,
    relation_name: String,
    owner_oid: u32,
    owner_role_name: String,
}

impl RelationOwnerObservation {
    /// Records exact catalog role identity without inferring it from a connection user.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let owner_role_name = owner_role_name.into();
        validate_postgresql_identifier(&schema_name, "relation_owner_schema_name")?;
        validate_postgresql_identifier(&relation_name, "relation_owner_relation_name")?;
        validate_postgresql_identifier(&owner_role_name, "relation_owner_role_name")?;
        if owner_oid == 0 {
            return Err(ObservationError::InvalidObservationField {
                field: "relation_owner_oid",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            owner_oid,
            owner_role_name,
        })
    }

    /// Returns the exact source schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact relation name.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the same-generation PostgreSQL role OID.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }

    /// Returns the exact resolved role name.
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }
}

pub(crate) fn canonicalize(
    relations: &[RelationObservation],
    mut observations: Vec<RelationOwnerObservation>,
) -> Result<Vec<RelationOwnerObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (left.schema_name(), left.relation_name())
            .cmp(&(right.schema_name(), right.relation_name()))
    });
    let expected = relations
        .iter()
        .map(|relation| (relation.schema_name(), relation.relation_name()))
        .collect::<BTreeSet<_>>();
    let actual = observations
        .iter()
        .map(|item| (item.schema_name(), item.relation_name()))
        .collect::<BTreeSet<_>>();
    if actual != expected || observations.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "relation_owner_coverage",
        });
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[RelationOwnerObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        encode_str(&mut hasher, item.schema_name());
        encode_str(&mut hasher, item.relation_name());
        hasher.update(item.owner_oid().to_be_bytes());
        encode_str(&mut hasher, item.owner_role_name());
    }
    encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RelationKind;

    #[test]
    fn owner_identity_requires_complete_coverage_and_binds_oid_and_name() {
        let relations =
            [RelationObservation::new("source", "record", RelationKind::Table, vec![]).unwrap()];
        let owner = RelationOwnerObservation::new("source", "record", 42, "owner").unwrap();
        assert!(canonicalize(&relations, vec![]).is_err());
        assert!(canonicalize(&relations, vec![owner.clone(), owner.clone()]).is_err());
        assert!(RelationOwnerObservation::new("source", "record", 0, "owner").is_err());
        assert_ne!(
            digest("sha256:prior", std::slice::from_ref(&owner)),
            digest(
                "sha256:prior",
                &[RelationOwnerObservation::new("source", "record", 43, "owner").unwrap()]
            )
        );
        assert_ne!(
            digest("sha256:prior", std::slice::from_ref(&owner)),
            digest(
                "sha256:prior",
                &[RelationOwnerObservation::new("source", "record", 42, "other").unwrap()]
            )
        );
    }
}
