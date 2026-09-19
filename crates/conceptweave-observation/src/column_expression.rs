//! Source-authoritative PostgreSQL column default and generation-expression evidence.
//!
//! `pg_attrdef` owns the defining expression for both ordinary column defaults and generated
//! columns. `pg_attribute.atthasdef` only establishes that such a row exists and
//! `pg_attribute.attgenerated` only distinguishes ordinary, stored-generated, and
//! virtual-generated declaration modes. This family therefore preserves the exact server-rendered
//! `pg_get_expr(adbin, adrelid)` text separately from the frozen v3 column representation and the
//! generated-column declaration family.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{ColumnGenerationObservation, ObservationError, RelationKind, RelationObservation};

const SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_EXPRESSION_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.column_expression.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ColumnExpressionKind {
    None,
    Default,
    Generation,
}

impl ColumnExpressionKind {
    const fn tag(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Default => 1,
            Self::Generation => 2,
        }
    }
}

/// One exact observed `pg_attrdef` expression state for a bounded relation column.
///
/// A column with no matching `pg_attrdef` row is represented explicitly by
/// [`Self::no_expression`]. Present expressions retain the exact text returned by
/// `pg_get_expr(adbin, adrelid)` from the same bounded catalog snapshot. Catalog OIDs and the
/// internal `pg_node_tree` serialization are capture-time details and are deliberately excluded
/// from governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnExpressionObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
    kind: ColumnExpressionKind,
    expression: Option<String>,
}

impl ColumnExpressionObservation {
    /// Records an explicitly observed column with no `pg_attrdef` expression row.
    pub fn no_expression(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnExpressionKind::None,
            None,
        )
    }

    /// Records an ordinary-column default expression rendered by PostgreSQL.
    pub fn default_expression(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        expression: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnExpressionKind::Default,
            Some(expression.into()),
        )
    }

    /// Records a generated-column expression rendered by PostgreSQL.
    pub fn generation_expression(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        expression: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnExpressionKind::Generation,
            Some(expression.into()),
        )
    }

    fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        kind: ColumnExpressionKind,
        expression: Option<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_postgresql_identifier(&schema_name, "schema_name")?;
        validate_postgresql_identifier(&relation_name, "relation_name")?;
        validate_postgresql_identifier(&column_name, "column_name")?;
        if let Some(value) = expression.as_deref() {
            crate::model::validate_nonblank(value, "column_expression")?;
        }
        if matches!(kind, ColumnExpressionKind::None) != expression.is_none() {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            kind,
            expression,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact observed relation kind for the owning column.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns whether the source explicitly reported no default or generation expression.
    #[must_use]
    pub const fn is_no_expression(&self) -> bool {
        matches!(self.kind, ColumnExpressionKind::None)
    }

    /// Returns whether this observation is an ordinary-column default expression.
    #[must_use]
    pub const fn is_default_expression(&self) -> bool {
        matches!(self.kind, ColumnExpressionKind::Default)
    }

    /// Returns whether this observation is a generated-column expression.
    #[must_use]
    pub const fn is_generation_expression(&self) -> bool {
        matches!(self.kind, ColumnExpressionKind::Generation)
    }

    /// Returns the exact server-rendered expression, or `None` for explicit no-expression state.
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }
}

pub(crate) fn canonicalize_column_expressions(
    relations: &[RelationObservation],
    column_generations: &[ColumnGenerationObservation],
    mut column_expressions: Vec<ColumnExpressionObservation>,
) -> Result<Vec<ColumnExpressionObservation>, ObservationError> {
    column_expressions.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.column_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.column_name(),
            ))
    });

    for pair in column_expressions.windows(2) {
        if same_column_coordinate(&pair[0], &pair[1]) {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_coordinate",
            });
        }
    }

    let expected_coordinates = relations
        .iter()
        .flat_map(|relation| {
            relation.columns().iter().map(move |column| {
                (
                    relation.schema_name().to_owned(),
                    relation.relation_name().to_owned(),
                    relation.kind().token().to_owned(),
                    column.column_name().to_owned(),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_coordinates = BTreeSet::new();

    for observation in &column_expressions {
        let coordinate_exists = relations.iter().any(|relation| {
            relation.schema_name() == observation.schema_name()
                && relation.relation_name() == observation.relation_name()
                && relation.kind() == observation.relation_kind()
                && relation
                    .columns()
                    .iter()
                    .any(|column| column.column_name() == observation.column_name())
        });
        if !coordinate_exists {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_coordinate",
            });
        }

        let Some(generation) = column_generations.iter().find(|generation| {
            generation.schema_name() == observation.schema_name()
                && generation.relation_name() == observation.relation_name()
                && generation.relation_kind() == observation.relation_kind()
                && generation.column_name() == observation.column_name()
        }) else {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_generation",
            });
        };

        let expression_matches_generation = if generation.is_not_generated() {
            !observation.is_generation_expression()
        } else {
            observation.is_generation_expression()
        };
        if !expression_matches_generation {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_generation",
            });
        }

        observed_coordinates.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.column_name().to_owned(),
        ));
    }

    if observed_coordinates != expected_coordinates {
        return Err(ObservationError::InvalidObservationField {
            field: "column_expression_completeness",
        });
    }

    Ok(column_expressions)
}

fn same_column_coordinate(
    left: &ColumnExpressionObservation,
    right: &ColumnExpressionObservation,
) -> bool {
    left.schema_name() == right.schema_name()
        && left.relation_name() == right.relation_name()
        && left.relation_kind() == right.relation_kind()
        && left.column_name() == right.column_name()
}

pub(crate) fn compute_column_expression_digest(
    base_snapshot_digest: &str,
    column_expressions: &[ColumnExpressionObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_EXPRESSION_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, column_expressions.len());
    for observation in column_expressions {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.column_name());
        hasher.update([observation.kind.tag()]);
        match observation.expression() {
            Some(expression) => {
                hasher.update([1]);
                encode_str(&mut hasher, expression);
            }
            None => hasher.update([0]),
        }
    }
    encode_sha256(hasher)
}

fn encode_sha256(hasher: Sha256) -> String {
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity("sha256:".len() + digest.len() * 2);
    encoded.push_str("sha256:");
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_bytes(hasher, value.as_bytes());
}

fn encode_bytes(hasher: &mut Sha256, value: &[u8]) {
    encode_len(hasher, value.len());
    hasher.update(value);
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}
