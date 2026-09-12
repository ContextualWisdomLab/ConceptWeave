use crate::model::validate_nonblank;
use crate::{ObservationError, QualifiedTypeName, RelationKind};

/// Stable resolved identity for one `pg_constraint.conexclop` entry.
///
/// PostgreSQL stores exclusion operators as catalog OIDs. OIDs are capture-time join coordinates,
/// not durable semantic identity, and operator names can be overloaded. ConceptWeave therefore
/// preserves the one-based constrained-column position together with the exact operator namespace,
/// name, and qualified binary operand types resolved from the same bounded catalog snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConstraintExclusionOperatorObservation {
    position: u32,
    operator_schema_name: String,
    operator_name: String,
    left_type: QualifiedTypeName,
    right_type: QualifiedTypeName,
}

impl ConstraintExclusionOperatorObservation {
    pub(crate) fn new(
        position: u32,
        operator_schema_name: impl Into<String>,
        operator_name: impl Into<String>,
        left_type: QualifiedTypeName,
        right_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        if position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let operator_schema_name = operator_schema_name.into();
        let operator_name = operator_name.into();
        validate_nonblank(
            &operator_schema_name,
            "constraint_exclusion_operator_schema_name",
        )?;
        validate_nonblank(&operator_name, "constraint_exclusion_operator_name")?;
        Ok(Self {
            position,
            operator_schema_name,
            operator_name,
            left_type,
            right_type,
        })
    }

    pub(crate) const fn position(&self) -> u32 {
        self.position
    }

    pub(crate) fn operator_schema_name(&self) -> &str {
        &self.operator_schema_name
    }

    pub(crate) fn operator_name(&self) -> &str {
        &self.operator_name
    }

    pub(crate) const fn left_type(&self) -> &QualifiedTypeName {
        &self.left_type
    }

    pub(crate) const fn right_type(&self) -> &QualifiedTypeName {
        &self.right_type
    }
}

/// Exact observed PostgreSQL temporal-constraint state for one table constraint.
///
/// PostgreSQL 18 exposes `pg_constraint.conperiod` directly. `true` means `WITHOUT OVERLAPS` for
/// PRIMARY KEY/UNIQUE constraints and `PERIOD` for FOREIGN KEY constraints. `false` remains material
/// observed evidence; absence of this value object means the catalog family was not observed.
/// `WITHOUT OVERLAPS` key observations additionally retain the exact resolved per-column
/// `pg_constraint.conexclop` vector; PERIOD foreign keys do not carry that key-only catalog field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintPeriodObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
    has_period_semantics: bool,
    exclusion_operators: Option<Vec<ConstraintExclusionOperatorObservation>>,
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
            exclusion_operators: None,
        })
    }

    pub(crate) fn with_exclusion_operators(
        mut self,
        mut exclusion_operators: Vec<ConstraintExclusionOperatorObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.has_period_semantics || exclusion_operators.is_empty() {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_exclusion_operators",
            });
        }
        exclusion_operators.sort_by_key(ConstraintExclusionOperatorObservation::position);
        for (expected_position, operator) in (1u32..).zip(&exclusion_operators) {
            if operator.position() != expected_position {
                return Err(ObservationError::InvalidObservationField {
                    field: "constraint_period_exclusion_operators",
                });
            }
        }
        self.exclusion_operators = Some(exclusion_operators);
        Ok(self)
    }

    /// Records resolved operator signatures without exposing source catalog OIDs as governed input.
    ///
    /// Each tuple is `(position, operator_schema, operator_name, left_type, right_type)`. Exact
    /// qualified operand types disambiguate overloaded PostgreSQL operator names.
    pub fn with_exclusion_operator_signatures(
        self,
        signatures: Vec<(u32, String, String, QualifiedTypeName, QualifiedTypeName)>,
    ) -> Result<Self, ObservationError> {
        let operators = signatures
            .into_iter()
            .map(
                |(position, operator_schema, operator_name, left_type, right_type)| {
                    ConstraintExclusionOperatorObservation::new(
                        position,
                        operator_schema,
                        operator_name,
                        left_type,
                        right_type,
                    )
                },
            )
            .collect::<Result<Vec<_>, _>>()?;
        self.with_exclusion_operators(operators)
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

    pub(crate) fn exclusion_operators(&self) -> Option<&[ConstraintExclusionOperatorObservation]> {
        self.exclusion_operators.as_deref()
    }

    /// Returns one resolved operator signature at its exact one-based position.
    #[must_use]
    pub fn exclusion_operator_signature(
        &self,
        position: u32,
    ) -> Option<(&str, &str, &QualifiedTypeName, &QualifiedTypeName)> {
        self.exclusion_operators
            .as_ref()?
            .iter()
            .find(|operator| operator.position() == position)
            .map(|operator| {
                (
                    operator.operator_schema_name(),
                    operator.operator_name(),
                    operator.left_type(),
                    operator.right_type(),
                )
            })
    }
}
