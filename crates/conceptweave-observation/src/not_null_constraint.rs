//! Source-authoritative PostgreSQL 18 `NOT NULL` constraint evidence.
//!
//! PostgreSQL 18 stores explicit column `NOT NULL` specifications in `pg_constraint`. The column
//! `pg_attribute.attnotnull` flag is only a summary and can describe a not-yet-validated constraint,
//! so this family retains the first-class constraint row separately from the frozen v3 column
//! representation. Catalog OIDs remain capture-time join coordinates and never enter governed
//! identity.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::{ObservationError, RelationKind, RelationObservation};

const SNAPSHOT_DIGEST_DOMAIN_V3_NOT_NULL_CONSTRAINT_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.not_null_constraint.v1";

/// Stable source coordinate for the parent partition constraint represented by `conparentid`.
///
/// PostgreSQL exposes the relationship as a catalog OID. The adapter resolves that OID inside the
/// same catalog snapshot and supplies this source coordinate instead; the OID itself is deliberately
/// excluded from governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParentNotNullConstraintCoordinate {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
}

impl ParentNotNullConstraintCoordinate {
    /// Creates one resolved parent-constraint coordinate from exact source identifiers.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        crate::model::validate_nonblank(&schema_name, "not_null_parent_schema_name")?;
        crate::model::validate_nonblank(&relation_name, "not_null_parent_relation_name")?;
        crate::model::validate_nonblank(&constraint_name, "not_null_parent_constraint_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
        })
    }

    /// Returns the exact parent schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact parent relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact observed parent relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact parent constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }
}

/// One exact PostgreSQL 18 relation-scoped `NOT NULL` constraint observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotNullConstraintObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
    column_name: String,
    validated: bool,
    enforced: bool,
    is_local: bool,
    inheritance_ancestor_count: u16,
    no_inherit: bool,
    parent_constraint: Option<ParentNotNullConstraintCoordinate>,
}

impl NotNullConstraintObservation {
    /// Creates one source-authoritative `pg_constraint.contype = 'n'` observation.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
        column_name: impl Into<String>,
        validated: bool,
        enforced: bool,
        is_local: bool,
        inheritance_ancestor_count: u16,
        no_inherit: bool,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        let column_name = column_name.into();
        crate::model::validate_nonblank(&schema_name, "schema_name")?;
        crate::model::validate_nonblank(&relation_name, "relation_name")?;
        crate::model::validate_nonblank(&constraint_name, "not_null_constraint_name")?;
        crate::model::validate_nonblank(&column_name, "not_null_constraint_column_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
            column_name,
            validated,
            enforced,
            is_local,
            inheritance_ancestor_count,
            no_inherit,
            parent_constraint: None,
        })
    }

    /// Attaches the resolved source coordinate represented by a nonzero `conparentid`.
    pub fn with_parent_constraint(
        mut self,
        parent_constraint: ParentNotNullConstraintCoordinate,
    ) -> Result<Self, ObservationError> {
        if self.schema_name == parent_constraint.schema_name
            && self.relation_name == parent_constraint.relation_name
            && self.relation_kind == parent_constraint.relation_kind
            && self.constraint_name == parent_constraint.constraint_name
        {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent",
            });
        }
        self.parent_constraint = Some(parent_constraint);
        Ok(self)
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

    /// Returns the exact observed relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns the exact constrained source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns PostgreSQL `convalidated` state.
    #[must_use]
    pub const fn validated(&self) -> bool {
        self.validated
    }

    /// Returns PostgreSQL `conenforced` state.
    #[must_use]
    pub const fn enforced(&self) -> bool {
        self.enforced
    }

    /// Returns PostgreSQL `conislocal` state.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }

    /// Returns PostgreSQL `coninhcount` as a nonnegative ancestor count.
    #[must_use]
    pub const fn inheritance_ancestor_count(&self) -> u16 {
        self.inheritance_ancestor_count
    }

    /// Returns PostgreSQL `connoinherit` state.
    #[must_use]
    pub const fn no_inherit(&self) -> bool {
        self.no_inherit
    }

    /// Returns the resolved partition-parent constraint coordinate, when PostgreSQL reports one.
    #[must_use]
    pub const fn parent_constraint(&self) -> Option<&ParentNotNullConstraintCoordinate> {
        self.parent_constraint.as_ref()
    }
}

pub(crate) fn canonicalize_not_null_constraints(
    relations: &[RelationObservation],
    mut constraints: Vec<NotNullConstraintObservation>,
) -> Result<Vec<NotNullConstraintObservation>, ObservationError> {
    constraints.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.column_name(),
            left.constraint_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.column_name(),
                right.constraint_name(),
            ))
    });

    for pair in constraints.windows(2) {
        if same_column_coordinate(&pair[0], &pair[1]) {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_column",
            });
        }
    }

    let mut relation_constraint_names = BTreeSet::new();
    for observation in &constraints {
        if !relation_constraint_names.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.constraint_name().to_owned(),
        )) {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_name",
            });
        }
    }

    let expected_columns = relations
        .iter()
        .flat_map(|relation| {
            relation
                .columns()
                .iter()
                .filter(|column| !column.nullable())
                .map(move |column| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        column.column_name().to_owned(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_columns = BTreeSet::new();

    for observation in &constraints {
        let relation = relations.iter().find(|relation| {
            relation.schema_name() == observation.schema_name()
                && relation.relation_name() == observation.relation_name()
                && relation.kind() == observation.relation_kind()
        });
        let Some(relation) = relation else {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_coordinate",
            });
        };
        let column = relation
            .columns()
            .iter()
            .find(|column| column.column_name() == observation.column_name());
        let Some(column) = column else {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_coordinate",
            });
        };
        if column.nullable() {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_nullability",
            });
        }

        observed_columns.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.column_name().to_owned(),
        ));
    }

    if observed_columns != expected_columns {
        return Err(ObservationError::InvalidObservationField {
            field: "not_null_constraint_completeness",
        });
    }

    Ok(constraints)
}

fn same_column_coordinate(
    left: &NotNullConstraintObservation,
    right: &NotNullConstraintObservation,
) -> bool {
    left.schema_name() == right.schema_name()
        && left.relation_name() == right.relation_name()
        && left.relation_kind() == right.relation_kind()
        && left.column_name() == right.column_name()
}

pub(crate) fn compute_not_null_constraint_digest(
    base_snapshot_digest: &str,
    constraints: &[NotNullConstraintObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_NOT_NULL_CONSTRAINT_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, constraints.len());
    for observation in constraints {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.constraint_name());
        encode_str(&mut hasher, observation.column_name());
        hasher.update([
            u8::from(observation.validated()),
            u8::from(observation.enforced()),
            u8::from(observation.is_local()),
        ]);
        hasher.update(observation.inheritance_ancestor_count().to_be_bytes());
        hasher.update([u8::from(observation.no_inherit())]);
        match observation.parent_constraint() {
            Some(parent) => {
                hasher.update([1]);
                encode_str(&mut hasher, parent.schema_name());
                encode_str(&mut hasher, parent.relation_name());
                encode_str(&mut hasher, parent.relation_kind().token());
                encode_str(&mut hasher, parent.constraint_name());
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
