//! Exact PostgreSQL `pg_type` kind evidence used by successor schema admission.

use crate::model::ObservationError;
use crate::representation_v3::QualifiedTypeName;

/// Direct PostgreSQL type class reported by `pg_type.typtype`.
///
/// The enum preserves PostgreSQL's catalog distinction instead of collapsing range-capable types
/// into a boolean. In particular, domains stay distinct because PostgreSQL 18.4+ permits temporal
/// constraints on a domain whose observed base type resolves to a range or multirange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostgresTypeKind {
    /// Base type (`typtype = 'b'`).
    Base,
    /// Composite type (`typtype = 'c'`).
    Composite,
    /// Domain (`typtype = 'd'`).
    Domain,
    /// Enum type (`typtype = 'e'`).
    Enum,
    /// Pseudo-type (`typtype = 'p'`).
    Pseudo,
    /// Range type (`typtype = 'r'`).
    Range,
    /// Multirange type (`typtype = 'm'`).
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

/// One exact qualified PostgreSQL type and its directly observed catalog kind.
///
/// Domain observations additionally preserve the exact qualified `pg_type.typbasetype` target.
/// Catalog OIDs are deliberately absent: an adapter may use them to join `pg_type`, `pg_namespace`,
/// and `pg_range` within one capture transaction, but governed identity is the resolved qualified
/// coordinate plus catalog kind/base relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeKindObservation {
    type_name: QualifiedTypeName,
    kind: PostgresTypeKind,
    domain_base_type: Option<QualifiedTypeName>,
}

impl TypeKindObservation {
    /// Creates a non-domain type-kind observation.
    ///
    /// Domains require [`Self::domain`] so their base coordinate can never be silently omitted.
    pub fn new(
        type_name: QualifiedTypeName,
        kind: PostgresTypeKind,
    ) -> Result<Self, ObservationError> {
        if kind == PostgresTypeKind::Domain {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_domain_base",
            });
        }
        Ok(Self {
            type_name,
            kind,
            domain_base_type: None,
        })
    }

    /// Creates a domain observation with its exact qualified base-type coordinate.
    pub fn domain(
        type_name: QualifiedTypeName,
        domain_base_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        if type_name.schema_name() == domain_base_type.schema_name()
            && type_name.type_name() == domain_base_type.type_name()
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_domain_base",
            });
        }
        Ok(Self {
            type_name,
            kind: PostgresTypeKind::Domain,
            domain_base_type: Some(domain_base_type),
        })
    }

    /// Returns the exact qualified observed type coordinate.
    #[must_use]
    pub const fn type_name(&self) -> &QualifiedTypeName {
        &self.type_name
    }

    /// Returns the directly observed `pg_type.typtype` class.
    #[must_use]
    pub const fn kind(&self) -> PostgresTypeKind {
        self.kind
    }

    /// Returns the exact domain base coordinate, or `None` for non-domain types.
    #[must_use]
    pub const fn domain_base_type(&self) -> Option<&QualifiedTypeName> {
        self.domain_base_type.as_ref()
    }
}
