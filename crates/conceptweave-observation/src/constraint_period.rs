use crate::model::validate_nonblank;
use crate::{ObservationError, RelationKind};

/// Exact observed PostgreSQL temporal-constraint state for one table constraint.
///
/// PostgreSQL 18 exposes `pg_constraint.conperiod` directly. `true` means `WITHOUT OVERLAPS` for
/// PRIMARY KEY/UNIQUE constraints and `PERIOD` for FOREIGN KEY constraints. `false` remains material
/// observed evidence; absence of this value object means the catalog family was not observed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintPeriodObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
    has_period_semantics: bool,
}

impl ConstraintPeriodObservation {
    /// Creates exact `pg_constraint.conperiod` evidence without inferring from index shape or DDL.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
        has_period_semantics: bool,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        validate_nonblank(&schema_name, "constraint_period_schema_name")?;
        validate_nonblank(&relation_name, "constraint_period_relation_name")?;
        validate_nonblank(&constraint_name, "constraint_period_constraint_name")?;
        if !matches!(
            relation_kind,
            RelationKind::Table | RelationKind::PartitionedTable
        ) {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_relation_kind",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
            has_period_semantics,
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

    /// Returns the exact observed `pg_constraint.conperiod` value.
    #[must_use]
    pub const fn has_period_semantics(&self) -> bool {
        self.has_period_semantics
    }
}
