use crate::model::validate_nonblank;
use crate::{ObservationError, RelationKind};

/// PostgreSQL deferrability and initial timing for index-backed key constraints.
///
/// These values map exactly to `pg_constraint.condeferrable` and `condeferred` for PRIMARY KEY and
/// UNIQUE constraints. Absence of a [`ConstraintTimingObservation`] means this catalog family was not
/// observed; it must not be interpreted as `NOT DEFERRABLE`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConstraintDeferrability {
    /// `NOT DEFERRABLE` (`condeferrable = false`, `condeferred = false`).
    NotDeferrable,
    /// `DEFERRABLE INITIALLY IMMEDIATE` (`condeferrable = true`, `condeferred = false`).
    InitiallyImmediate,
    /// `DEFERRABLE INITIALLY DEFERRED` (`condeferrable = true`, `condeferred = true`).
    InitiallyDeferred,
}

impl ConstraintDeferrability {
    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::NotDeferrable => 0,
            Self::InitiallyImmediate => 1,
            Self::InitiallyDeferred => 2,
        }
    }
}

/// Exact observed timing for one PRIMARY KEY or UNIQUE constraint coordinate.
///
/// The coordinate includes the owning relation kind so a receipt-backed relation identity cannot be
/// silently rebound after a PostgreSQL relation-kind change.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintTimingObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
    deferrability: ConstraintDeferrability,
}

impl ConstraintTimingObservation {
    /// Creates exact key-constraint timing evidence without deriving PostgreSQL defaults.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
        deferrability: ConstraintDeferrability,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        validate_nonblank(&schema_name, "constraint_timing_schema_name")?;
        validate_nonblank(&relation_name, "constraint_timing_relation_name")?;
        validate_nonblank(&constraint_name, "constraint_timing_constraint_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
            deferrability,
        })
    }

    /// Returns the exact owning schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact owning relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact owning relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns exact PostgreSQL deferrability and initial timing.
    #[must_use]
    pub const fn deferrability(&self) -> ConstraintDeferrability {
        self.deferrability
    }
}
