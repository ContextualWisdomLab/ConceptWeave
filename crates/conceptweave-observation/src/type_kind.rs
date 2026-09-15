use crate::{ObservationError, QualifiedTypeName};

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
