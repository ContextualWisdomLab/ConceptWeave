use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, QualifiedTypeName, encode_bytes, encode_len, encode_sha256, encode_str,
};

const OWNER_DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.type_owner.v1";

/// Exact same-generation owner role for one schema-local PostgreSQL type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeOwnerObservation {
    type_name: QualifiedTypeName,
    owner_oid: u32,
    owner_role_name: String,
}

impl TypeOwnerObservation {
    /// Records the catalog role OID and resolved role name for an exact type coordinate.
    pub fn new(
        type_name: QualifiedTypeName,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let owner_role_name = owner_role_name.into();
        validate_postgresql_identifier(&owner_role_name, "type_owner_role_name")?;
        if owner_oid == 0 {
            return Err(ObservationError::InvalidObservationField {
                field: "type_owner_oid",
            });
        }
        Ok(Self {
            type_name,
            owner_oid,
            owner_role_name,
        })
    }

    /// Returns the exact type coordinate.
    #[must_use]
    pub const fn type_name(&self) -> &QualifiedTypeName {
        &self.type_name
    }

    /// Returns the catalog role OID.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }

    /// Returns the resolved role name.
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }
}

pub(crate) fn canonicalize_owners(
    type_kinds: &[TypeKindObservation],
    mut owners: Vec<TypeOwnerObservation>,
) -> Result<Vec<TypeOwnerObservation>, ObservationError> {
    owners.sort_by(|left, right| {
        (left.type_name.schema_name(), left.type_name.type_name())
            .cmp(&(right.type_name.schema_name(), right.type_name.type_name()))
    });
    let expected = type_kinds
        .iter()
        .map(|item| (item.type_name().schema_name(), item.type_name().type_name()))
        .collect::<BTreeSet<_>>();
    let actual = owners
        .iter()
        .map(|item| (item.type_name.schema_name(), item.type_name.type_name()))
        .collect::<BTreeSet<_>>();
    if expected != actual || owners.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "type_owner_coverage",
        });
    }
    Ok(owners)
}

pub(crate) fn owner_digest(base: &str, owners: &[TypeOwnerObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, OWNER_DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, owners.len());
    for item in owners {
        encode_str(&mut hasher, item.type_name.schema_name());
        encode_str(&mut hasher, item.type_name.type_name());
        hasher.update(item.owner_oid.to_be_bytes());
        encode_str(&mut hasher, &item.owner_role_name);
    }
    encode_sha256(hasher)
}

/// Source-authoritative PostgreSQL `pg_type.typtype` classification.
///
/// The variants preserve PostgreSQL's catalog distinction without deriving semantics from rendered
/// type names. Array types remain `Base` in PostgreSQL and are modeled separately by the existing
/// true-array evidence family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostgresTypeKind {
    /// `typtype = 'b'`: base type.
    Base,
    /// `typtype = 'c'`: composite type.
    Composite,
    /// `typtype = 'd'`: domain.
    Domain,
    /// `typtype = 'e'`: enum type.
    Enum,
    /// `typtype = 'p'`: pseudo-type.
    Pseudo,
    /// `typtype = 'r'`: range type.
    Range,
    /// `typtype = 'm'`: multirange type.
    Multirange,
}

impl PostgresTypeKind {
    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::Composite => 1,
            Self::Domain => 2,
            Self::Enum => 3,
            Self::Pseudo => 4,
            Self::Range => 5,
            Self::Multirange => 6,
        }
    }
}

/// Exact `pg_type`/`pg_range` evidence for one qualified PostgreSQL type coordinate.
///
/// Domains carry the exact qualified `typbasetype`. Range and multirange observations carry the
/// exact reciprocal type coordinate obtained from the same `pg_range` row. Catalog OIDs are capture
/// joins only and never participate in governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeKindObservation {
    type_name: QualifiedTypeName,
    kind: PostgresTypeKind,
    domain_base_type: Option<QualifiedTypeName>,
    range_counterpart: Option<QualifiedTypeName>,
}

impl TypeKindObservation {
    /// Creates a base, composite, enum, or pseudo-type observation.
    pub fn plain(
        type_name: QualifiedTypeName,
        kind: PostgresTypeKind,
    ) -> Result<Self, ObservationError> {
        if matches!(
            kind,
            PostgresTypeKind::Domain | PostgresTypeKind::Range | PostgresTypeKind::Multirange
        ) {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_shape",
            });
        }
        Ok(Self {
            type_name,
            kind,
            domain_base_type: None,
            range_counterpart: None,
        })
    }

    /// Creates `typtype = 'd'` evidence with the exact qualified `typbasetype` coordinate.
    pub fn domain(type_name: QualifiedTypeName, base_type: QualifiedTypeName) -> Self {
        Self {
            type_name,
            kind: PostgresTypeKind::Domain,
            domain_base_type: Some(base_type),
            range_counterpart: None,
        }
    }

    /// Creates `typtype = 'r'` evidence with its exact `pg_range.rngmultitypid` coordinate.
    pub fn range(type_name: QualifiedTypeName, multirange_type: QualifiedTypeName) -> Self {
        Self {
            type_name,
            kind: PostgresTypeKind::Range,
            domain_base_type: None,
            range_counterpart: Some(multirange_type),
        }
    }

    /// Creates `typtype = 'm'` evidence with the range coordinate from the same `pg_range` row.
    pub fn multirange(type_name: QualifiedTypeName, range_type: QualifiedTypeName) -> Self {
        Self {
            type_name,
            kind: PostgresTypeKind::Multirange,
            domain_base_type: None,
            range_counterpart: Some(range_type),
        }
    }

    /// Returns the exact qualified type coordinate.
    #[must_use]
    pub const fn type_name(&self) -> &QualifiedTypeName {
        &self.type_name
    }

    /// Returns the exact observed PostgreSQL type kind.
    #[must_use]
    pub const fn kind(&self) -> PostgresTypeKind {
        self.kind
    }

    /// Returns the exact qualified domain base type when this is a domain.
    #[must_use]
    pub fn domain_base_type(&self) -> Option<&QualifiedTypeName> {
        self.domain_base_type.as_ref()
    }

    /// Returns the exact reciprocal range/multirange coordinate when applicable.
    #[must_use]
    pub fn range_counterpart(&self) -> Option<&QualifiedTypeName> {
        self.range_counterpart.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_owner_requires_complete_coverage_and_binds_role_identity() {
        let coordinate = QualifiedTypeName::new("source", "stage").unwrap();
        let kinds =
            [TypeKindObservation::plain(coordinate.clone(), PostgresTypeKind::Enum).unwrap()];
        let owner = TypeOwnerObservation::new(coordinate.clone(), 42, "owner").unwrap();
        assert!(canonicalize_owners(&kinds, vec![]).is_err());
        assert!(canonicalize_owners(&kinds, vec![owner.clone(), owner.clone()]).is_err());
        assert!(TypeOwnerObservation::new(coordinate.clone(), 0, "owner").is_err());
        assert_ne!(
            owner_digest("sha256:prior", std::slice::from_ref(&owner)),
            owner_digest(
                "sha256:prior",
                &[TypeOwnerObservation::new(coordinate.clone(), 43, "owner").unwrap()]
            )
        );
        assert_ne!(
            owner_digest("sha256:prior", std::slice::from_ref(&owner)),
            owner_digest(
                "sha256:prior",
                &[TypeOwnerObservation::new(coordinate, 42, "other").unwrap()]
            )
        );
    }
}
