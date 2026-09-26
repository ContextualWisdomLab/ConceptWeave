//! Exact PostgreSQL role identity for each observed source schema.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{ObservationError, encode_bytes, encode_len, encode_sha256, encode_str};

const DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.schema_owner.v1";

/// Exact same-generation owner role OID and name for one observed schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaOwnerObservation {
    schema_name: String,
    owner_oid: u32,
    owner_role_name: String,
}

/// Exact receipt coordinate for an observed source schema and its owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaOwnerLocation {
    schema_name: String,
}

impl SchemaOwnerLocation {
    /// Creates an exact schema-owner coordinate without changing source identifier text.
    pub fn new(schema_name: impl Into<String>) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        validate_postgresql_identifier(&schema_name, "schema_owner_schema_name")?;
        Ok(Self { schema_name })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the collision-safe receipt path with RFC 6901 escaping.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}",
            self.schema_name.replace('~', "~0").replace('/', "~1")
        )
    }
}

/// Immutable provenance receipt for one verified source-schema owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaOwnerSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: SchemaOwnerLocation,
}

impl SchemaOwnerSourceReceipt {
    pub(crate) fn new(
        source_id: String,
        connection_policy_binding: String,
        source_digest: String,
        extractor_revision: String,
        observed_at_utc: String,
        location: SchemaOwnerLocation,
    ) -> Self {
        Self {
            source_id,
            connection_policy_binding,
            source_digest,
            extractor_revision,
            observed_at_utc,
            location,
        }
    }

    /// Returns the stable source registry reference.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the schema-owner-aware governed source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the verified source-schema owner coordinate.
    #[must_use]
    pub const fn location(&self) -> &SchemaOwnerLocation {
        &self.location
    }
}

impl SchemaOwnerObservation {
    /// Records exact catalog role identity without inferring it from a connection user.
    pub fn new(
        schema_name: impl Into<String>,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let owner_role_name = owner_role_name.into();
        validate_postgresql_identifier(&schema_name, "schema_owner_schema_name")?;
        validate_postgresql_identifier(&owner_role_name, "schema_owner_role_name")?;
        if owner_oid == 0 {
            return Err(ObservationError::InvalidObservationField {
                field: "schema_owner_oid",
            });
        }
        Ok(Self {
            schema_name,
            owner_oid,
            owner_role_name,
        })
    }

    /// Returns the exact source schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
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
    authorized_schema_names: &[String],
    mut observations: Vec<SchemaOwnerObservation>,
) -> Result<Vec<SchemaOwnerObservation>, ObservationError> {
    observations.sort_by(|left, right| left.schema_name().cmp(right.schema_name()));
    let actual = observations
        .iter()
        .map(SchemaOwnerObservation::schema_name)
        .collect::<BTreeSet<_>>();
    let expected = authorized_schema_names
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if observations.len() != actual.len() || actual != expected {
        return Err(ObservationError::InvalidObservationField {
            field: "schema_owner_coverage",
        });
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[SchemaOwnerObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        encode_str(&mut hasher, item.schema_name());
        hasher.update(item.owner_oid().to_be_bytes());
        encode_str(&mut hasher, item.owner_role_name());
    }
    encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_owner_requires_coverage_and_binds_role_identity() {
        let schemas = ["source".to_owned(), "empty".to_owned()];
        let owner = SchemaOwnerObservation::new("source", 42, "owner").unwrap();
        assert!(canonicalize(&schemas, vec![owner.clone()]).is_err());
        assert!(canonicalize(&schemas, vec![owner.clone(), owner.clone()]).is_err());
        assert!(
            canonicalize(
                &["source".to_owned()],
                vec![
                    owner.clone(),
                    SchemaOwnerObservation::new("extra", 44, "owner").unwrap()
                ]
            )
            .is_err()
        );
        assert!(
            canonicalize(
                &schemas,
                vec![
                    owner.clone(),
                    SchemaOwnerObservation::new("empty", 44, "owner").unwrap()
                ]
            )
            .is_ok()
        );
        assert!(SchemaOwnerObservation::new("source", 0, "owner").is_err());
        assert_ne!(
            digest("sha256:prior", std::slice::from_ref(&owner)),
            digest(
                "sha256:prior",
                &[SchemaOwnerObservation::new("source", 43, "owner").unwrap()]
            )
        );
        assert_ne!(
            digest("sha256:prior", std::slice::from_ref(&owner)),
            digest(
                "sha256:prior",
                &[SchemaOwnerObservation::new("source", 42, "renamed").unwrap()]
            )
        );
    }
}
