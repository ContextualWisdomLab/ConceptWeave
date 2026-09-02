//! Immutable PostgreSQL schema-observation contracts for ConceptWeave.
//!
//! This crate owns deterministic, provider-independent Source Observation value objects. A live
//! PostgreSQL adapter belongs outside this crate and must supply bounded, read-only metadata. The
//! contract preserves exact identifiers rather than normalizing case or quoting semantics.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Fail-closed validation errors for immutable schema observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationError {
    /// A required observation field contained only Unicode whitespace.
    InvalidObservationField {
        /// Stable field name for caller diagnostics.
        field: &'static str,
    },
    /// PostgreSQL ordinal positions are one-based and therefore cannot be zero.
    InvalidOrdinalPosition,
    /// The same exact source column name appeared more than once in a table observation.
    DuplicateColumnName {
        /// Exact source schema identifier.
        schema_name: String,
        /// Exact source table identifier.
        table_name: String,
        /// Exact duplicated source column identifier.
        column_name: String,
    },
    /// Two columns claimed the same source ordinal position.
    DuplicateColumnOrdinal {
        /// Exact source schema identifier.
        schema_name: String,
        /// Exact source table identifier.
        table_name: String,
        /// Duplicated one-based source ordinal position.
        ordinal_position: u32,
    },
    /// A key or relationship constraint did not name any source columns.
    EmptyConstraintColumns {
        /// Exact source constraint identifier.
        constraint_name: String,
    },
    /// The same exact source column appeared twice within one constraint coordinate list.
    DuplicateConstraintColumn {
        /// Exact source constraint identifier.
        constraint_name: String,
        /// Exact duplicated source column identifier.
        column_name: String,
    },
    /// The same exact source constraint name appeared more than once on one table.
    DuplicateConstraintName {
        /// Exact source schema identifier.
        schema_name: String,
        /// Exact source table identifier.
        table_name: String,
        /// Exact duplicated source constraint identifier.
        constraint_name: String,
    },
    /// A table constraint referred to a local column absent from the same observation.
    UnknownConstraintColumn {
        /// Exact source schema identifier.
        schema_name: String,
        /// Exact source table identifier.
        table_name: String,
        /// Exact source constraint identifier.
        constraint_name: String,
        /// Exact missing local source column identifier.
        column_name: String,
    },
    /// A foreign key did not provide a one-to-one local-to-referenced column coordinate mapping.
    ForeignKeyArityMismatch {
        /// Exact source constraint identifier.
        constraint_name: String,
        /// Number of local source columns in the relationship coordinate.
        local_column_count: usize,
        /// Number of referenced source columns in the relationship coordinate.
        referenced_column_count: usize,
    },
    /// The same exact `(schema_name, table_name)` observation appeared more than once.
    DuplicateTableObservation {
        /// Exact source schema identifier.
        schema_name: String,
        /// Exact source table identifier.
        table_name: String,
    },
    /// An evidence receipt requested a coordinate absent from the immutable snapshot.
    UnknownObservationLocation {
        /// Canonical escaped location requested by the caller.
        location: String,
    },
}

impl Display for ObservationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidObservationField { field } => {
                write!(formatter, "invalid observation field: {field}")
            }
            Self::InvalidOrdinalPosition => {
                write!(formatter, "column ordinal position must be positive")
            }
            Self::DuplicateColumnName {
                schema_name,
                table_name,
                column_name,
            } => write!(
                formatter,
                "duplicate column observation: {schema_name}.{table_name}.{column_name}"
            ),
            Self::DuplicateColumnOrdinal {
                schema_name,
                table_name,
                ordinal_position,
            } => write!(
                formatter,
                "duplicate column ordinal in {schema_name}.{table_name}: {ordinal_position}"
            ),
            Self::EmptyConstraintColumns { constraint_name } => {
                write!(formatter, "constraint has no columns: {constraint_name}")
            }
            Self::DuplicateConstraintColumn {
                constraint_name,
                column_name,
            } => write!(
                formatter,
                "duplicate constraint column in {constraint_name}: {column_name}"
            ),
            Self::DuplicateConstraintName {
                schema_name,
                table_name,
                constraint_name,
            } => write!(
                formatter,
                "duplicate constraint observation on {schema_name}.{table_name}: {constraint_name}"
            ),
            Self::UnknownConstraintColumn {
                schema_name,
                table_name,
                constraint_name,
                column_name,
            } => write!(
                formatter,
                "constraint {constraint_name} on {schema_name}.{table_name} references unknown local column {column_name}"
            ),
            Self::ForeignKeyArityMismatch {
                constraint_name,
                local_column_count,
                referenced_column_count,
            } => write!(
                formatter,
                "foreign key {constraint_name} has {local_column_count} local columns but {referenced_column_count} referenced columns"
            ),
            Self::DuplicateTableObservation {
                schema_name,
                table_name,
            } => write!(formatter, "duplicate table observation: {schema_name}.{table_name}"),
            Self::UnknownObservationLocation { location } => {
                write!(formatter, "unobserved source location: {location}")
            }
        }
    }
}

impl Error for ObservationError {}

/// One immutable PostgreSQL column observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnObservation {
    column_name: String,
    ordinal_position: u32,
    data_type: String,
    nullable: bool,
    source_comment: Option<String>,
}

impl ColumnObservation {
    /// Creates a column observation while preserving exact source text.
    pub fn new(
        column_name: impl Into<String>,
        ordinal_position: u32,
        data_type: impl Into<String>,
        nullable: bool,
        source_comment: Option<String>,
    ) -> Result<Self, ObservationError> {
        let column_name = column_name.into();
        let data_type = data_type.into();
        validate_nonblank(&column_name, "column_name")?;
        if ordinal_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        validate_nonblank(&data_type, "data_type")?;
        Ok(Self {
            column_name,
            ordinal_position,
            data_type,
            nullable,
            source_comment,
        })
    }

    /// Returns the exact source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns the one-based source ordinal position.
    #[must_use]
    pub const fn ordinal_position(&self) -> u32 {
        self.ordinal_position
    }

    /// Returns the exact PostgreSQL data-type text captured by the adapter.
    #[must_use]
    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    /// Returns whether the source column permits null values.
    #[must_use]
    pub const fn nullable(&self) -> bool {
        self.nullable
    }

    /// Returns the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }
}

/// Immutable observation of one PostgreSQL primary-key constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimaryKeyObservation {
    constraint_name: String,
    column_names: Vec<String>,
}

impl PrimaryKeyObservation {
    /// Creates a primary-key observation while preserving exact source column order.
    pub fn new(
        constraint_name: impl Into<String>,
        column_names: Vec<String>,
    ) -> Result<Self, ObservationError> {
        let constraint_name = constraint_name.into();
        validate_nonblank(&constraint_name, "constraint_name")?;
        validate_constraint_columns(&constraint_name, &column_names, "constraint_column_name")?;
        Ok(Self {
            constraint_name,
            column_names,
        })
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns source columns in the exact key ordinal order reported by PostgreSQL.
    #[must_use]
    pub fn column_names(&self) -> &[String] {
        &self.column_names
    }
}

/// Immutable observation of one PostgreSQL unique constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UniqueConstraintObservation {
    constraint_name: String,
    column_names: Vec<String>,
}

impl UniqueConstraintObservation {
    /// Creates a unique-constraint observation while preserving exact source column order.
    pub fn new(
        constraint_name: impl Into<String>,
        column_names: Vec<String>,
    ) -> Result<Self, ObservationError> {
        let constraint_name = constraint_name.into();
        validate_nonblank(&constraint_name, "constraint_name")?;
        validate_constraint_columns(&constraint_name, &column_names, "constraint_column_name")?;
        Ok(Self {
            constraint_name,
            column_names,
        })
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns source columns in the exact unique-key ordinal order reported by PostgreSQL.
    #[must_use]
    pub fn column_names(&self) -> &[String] {
        &self.column_names
    }
}

/// PostgreSQL referential action preserved from a foreign-key definition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForeignKeyAction {
    /// `NO ACTION`.
    NoAction,
    /// `RESTRICT`.
    Restrict,
    /// `CASCADE`.
    Cascade,
    /// `SET NULL`.
    SetNull,
    /// `SET DEFAULT`.
    SetDefault,
}

/// PostgreSQL foreign-key match type preserved from source metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForeignKeyMatchType {
    /// `MATCH SIMPLE`.
    Simple,
    /// `MATCH FULL`.
    Full,
    /// `MATCH PARTIAL` when represented by source metadata.
    Partial,
}

/// PostgreSQL foreign-key deferrability and initial timing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForeignKeyDeferrability {
    /// The constraint is not deferrable.
    NotDeferrable,
    /// The constraint is deferrable and initially immediate.
    InitiallyImmediate,
    /// The constraint is deferrable and initially deferred.
    InitiallyDeferred,
}

/// Exact PostgreSQL reference behavior for one observed foreign key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForeignKeyReferenceBehavior {
    update_action: ForeignKeyAction,
    delete_action: ForeignKeyAction,
    match_type: ForeignKeyMatchType,
    deferrability: ForeignKeyDeferrability,
}

impl ForeignKeyReferenceBehavior {
    /// Creates exact source behavior without deriving or filling defaults.
    #[must_use]
    pub const fn new(
        update_action: ForeignKeyAction,
        delete_action: ForeignKeyAction,
        match_type: ForeignKeyMatchType,
        deferrability: ForeignKeyDeferrability,
    ) -> Self {
        Self {
            update_action,
            delete_action,
            match_type,
            deferrability,
        }
    }

    /// Returns the exact `ON UPDATE` action.
    #[must_use]
    pub const fn update_action(&self) -> ForeignKeyAction {
        self.update_action
    }

    /// Returns the exact `ON DELETE` action.
    #[must_use]
    pub const fn delete_action(&self) -> ForeignKeyAction {
        self.delete_action
    }

    /// Returns the exact foreign-key match type.
    #[must_use]
    pub const fn match_type(&self) -> ForeignKeyMatchType {
        self.match_type
    }

    /// Returns the exact deferrability and initial timing.
    #[must_use]
    pub const fn deferrability(&self) -> ForeignKeyDeferrability {
        self.deferrability
    }
}

/// Immutable observation of one PostgreSQL foreign-key relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForeignKeyObservation {
    constraint_name: String,
    column_names: Vec<String>,
    referenced_schema_name: String,
    referenced_table_name: String,
    referenced_column_names: Vec<String>,
    reference_behavior: Option<ForeignKeyReferenceBehavior>,
}

impl ForeignKeyObservation {
    /// Creates a foreign-key observation when reference behavior was not observed.
    pub fn new(
        constraint_name: impl Into<String>,
        column_names: Vec<String>,
        referenced_schema_name: impl Into<String>,
        referenced_table_name: impl Into<String>,
        referenced_column_names: Vec<String>,
    ) -> Result<Self, ObservationError> {
        Self::build(
            constraint_name,
            column_names,
            referenced_schema_name,
            referenced_table_name,
            referenced_column_names,
            None,
        )
    }

    /// Creates a foreign-key observation with exact source reference behavior.
    pub fn with_reference_behavior(
        constraint_name: impl Into<String>,
        column_names: Vec<String>,
        referenced_schema_name: impl Into<String>,
        referenced_table_name: impl Into<String>,
        referenced_column_names: Vec<String>,
        reference_behavior: ForeignKeyReferenceBehavior,
    ) -> Result<Self, ObservationError> {
        Self::build(
            constraint_name,
            column_names,
            referenced_schema_name,
            referenced_table_name,
            referenced_column_names,
            Some(reference_behavior),
        )
    }

    fn build(
        constraint_name: impl Into<String>,
        column_names: Vec<String>,
        referenced_schema_name: impl Into<String>,
        referenced_table_name: impl Into<String>,
        referenced_column_names: Vec<String>,
        reference_behavior: Option<ForeignKeyReferenceBehavior>,
    ) -> Result<Self, ObservationError> {
        let constraint_name = constraint_name.into();
        let referenced_schema_name = referenced_schema_name.into();
        let referenced_table_name = referenced_table_name.into();
        validate_nonblank(&constraint_name, "constraint_name")?;
        validate_nonblank(&referenced_schema_name, "referenced_schema_name")?;
        validate_nonblank(&referenced_table_name, "referenced_table_name")?;
        validate_constraint_columns(&constraint_name, &column_names, "constraint_column_name")?;
        validate_constraint_columns(
            &constraint_name,
            &referenced_column_names,
            "referenced_column_name",
        )?;
        if column_names.len() != referenced_column_names.len() {
            return Err(ObservationError::ForeignKeyArityMismatch {
                constraint_name,
                local_column_count: column_names.len(),
                referenced_column_count: referenced_column_names.len(),
            });
        }
        Ok(Self {
            constraint_name,
            column_names,
            referenced_schema_name,
            referenced_table_name,
            referenced_column_names,
            reference_behavior,
        })
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns local source columns in the exact relationship ordinal order.
    #[must_use]
    pub fn column_names(&self) -> &[String] {
        &self.column_names
    }

    /// Returns the exact referenced schema identifier.
    #[must_use]
    pub fn referenced_schema_name(&self) -> &str {
        &self.referenced_schema_name
    }

    /// Returns the exact referenced table identifier.
    #[must_use]
    pub fn referenced_table_name(&self) -> &str {
        &self.referenced_table_name
    }

    /// Returns referenced source columns in the exact relationship ordinal order.
    #[must_use]
    pub fn referenced_column_names(&self) -> &[String] {
        &self.referenced_column_names
    }

    /// Returns exact reference behavior when it was observed, or `None` when it was not observed.
    #[must_use]
    pub const fn reference_behavior(&self) -> Option<&ForeignKeyReferenceBehavior> {
        self.reference_behavior.as_ref()
    }
}

/// Immutable table-level key or relationship evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TableConstraintObservation {
    /// Primary-key evidence.
    PrimaryKey(PrimaryKeyObservation),
    /// Unique-constraint evidence.
    Unique(UniqueConstraintObservation),
    /// Foreign-key relationship evidence.
    ForeignKey(ForeignKeyObservation),
}

impl TableConstraintObservation {
    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        match self {
            Self::PrimaryKey(observation) => observation.constraint_name(),
            Self::Unique(observation) => observation.constraint_name(),
            Self::ForeignKey(observation) => observation.constraint_name(),
        }
    }

    /// Returns local source columns in the exact constraint ordinal order.
    #[must_use]
    pub fn column_names(&self) -> &[String] {
        match self {
            Self::PrimaryKey(observation) => observation.column_names(),
            Self::Unique(observation) => observation.column_names(),
            Self::ForeignKey(observation) => observation.column_names(),
        }
    }
}

/// Immutable observation of one qualified PostgreSQL table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableObservation {
    schema_name: String,
    table_name: String,
    columns: Vec<ColumnObservation>,
    constraints: Vec<TableConstraintObservation>,
}

impl TableObservation {
    /// Creates one table observation without key or relationship evidence.
    pub fn new(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
        columns: Vec<ColumnObservation>,
    ) -> Result<Self, ObservationError> {
        Self::with_constraints(schema_name, table_name, columns, Vec::new())
    }

    /// Creates one table observation with deterministic key and relationship evidence.
    ///
    /// Collection order is canonicalized, exact identifiers are never normalized, and every local
    /// constraint column must be present in the same table observation.
    pub fn with_constraints(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
        mut columns: Vec<ColumnObservation>,
        mut constraints: Vec<TableConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let table_name = table_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&table_name, "table_name")?;

        let mut column_names = BTreeSet::new();
        let mut ordinal_positions = BTreeSet::new();
        for column in &columns {
            if !column_names.insert(column.column_name.clone()) {
                return Err(ObservationError::DuplicateColumnName {
                    schema_name,
                    table_name,
                    column_name: column.column_name.clone(),
                });
            }
            if !ordinal_positions.insert(column.ordinal_position) {
                return Err(ObservationError::DuplicateColumnOrdinal {
                    schema_name,
                    table_name,
                    ordinal_position: column.ordinal_position,
                });
            }
        }

        let mut constraint_names = BTreeSet::new();
        for constraint in &constraints {
            let constraint_name = constraint.constraint_name();
            if !constraint_names.insert(constraint_name.to_owned()) {
                return Err(ObservationError::DuplicateConstraintName {
                    schema_name,
                    table_name,
                    constraint_name: constraint_name.to_owned(),
                });
            }
            for column_name in constraint.column_names() {
                if !column_names.contains(column_name) {
                    return Err(ObservationError::UnknownConstraintColumn {
                        schema_name,
                        table_name,
                        constraint_name: constraint_name.to_owned(),
                        column_name: column_name.clone(),
                    });
                }
            }
        }

        columns.sort_by(|left, right| {
            (left.ordinal_position, left.column_name.as_str())
                .cmp(&(right.ordinal_position, right.column_name.as_str()))
        });
        constraints.sort_by(|left, right| left.constraint_name().cmp(right.constraint_name()));
        Ok(Self {
            schema_name,
            table_name,
            columns,
            constraints,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source table identifier.
    #[must_use]
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Returns columns in deterministic source ordinal order.
    #[must_use]
    pub fn columns(&self) -> &[ColumnObservation] {
        &self.columns
    }

    /// Returns constraints in deterministic exact source-name order.
    #[must_use]
    pub fn constraints(&self) -> &[TableConstraintObservation] {
        &self.constraints
    }
}

/// Stable type discriminator for an exact observed relational evidence coordinate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationLocationKind {
    /// A qualified table observation.
    Table,
    /// A qualified column observation.
    Column,
    /// A qualified table-constraint observation.
    Constraint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ObservationElement {
    Table,
    Column(String),
    Constraint(String),
}

/// Exact structured location inside an immutable PostgreSQL schema snapshot.
///
/// Exact identifiers are retained separately instead of being parsed from dotted SQL names. The
/// canonical string form applies RFC 6901 reference-token escaping (`~` -> `~0`, `/` -> `~1`) so
/// quoted source identifiers containing path delimiters remain collision-safe without case or
/// Unicode normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationLocation {
    schema_name: String,
    table_name: String,
    element: ObservationElement,
}

impl ObservationLocation {
    /// Creates a location for an exact qualified table.
    pub fn table(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(schema_name, table_name, ObservationElement::Table)
    }

    /// Creates a location for an exact qualified column.
    pub fn column(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let column_name = column_name.into();
        validate_nonblank(&column_name, "column_name")?;
        Self::new(
            schema_name,
            table_name,
            ObservationElement::Column(column_name),
        )
    }

    /// Creates a location for an exact qualified table constraint.
    pub fn constraint(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let constraint_name = constraint_name.into();
        validate_nonblank(&constraint_name, "constraint_name")?;
        Self::new(
            schema_name,
            table_name,
            ObservationElement::Constraint(constraint_name),
        )
    }

    fn new(
        schema_name: impl Into<String>,
        table_name: impl Into<String>,
        element: ObservationElement,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let table_name = table_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&table_name, "table_name")?;
        Ok(Self {
            schema_name,
            table_name,
            element,
        })
    }

    /// Returns the coordinate kind without exposing mutable representation details.
    #[must_use]
    pub fn kind(&self) -> ObservationLocationKind {
        match self.element {
            ObservationElement::Table => ObservationLocationKind::Table,
            ObservationElement::Column(_) => ObservationLocationKind::Column,
            ObservationElement::Constraint(_) => ObservationLocationKind::Constraint,
        }
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source table identifier.
    #[must_use]
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Returns the exact source column identifier for a column coordinate.
    #[must_use]
    pub fn column_name(&self) -> Option<&str> {
        match &self.element {
            ObservationElement::Column(column_name) => Some(column_name),
            ObservationElement::Table | ObservationElement::Constraint(_) => None,
        }
    }

    /// Returns the exact source constraint identifier for a constraint coordinate.
    #[must_use]
    pub fn constraint_name(&self) -> Option<&str> {
        match &self.element {
            ObservationElement::Constraint(constraint_name) => Some(constraint_name),
            ObservationElement::Table | ObservationElement::Column(_) => None,
        }
    }

    /// Returns a deterministic collision-safe evidence location string.
    ///
    /// The vocabulary segments (`schemas`, `tables`, `columns`, `constraints`) are ConceptWeave
    /// coordinate labels; identifier tokens use RFC 6901 escaping and retain exact case/text.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        let mut location = format!(
            "/schemas/{}/tables/{}",
            escape_json_pointer_token(&self.schema_name),
            escape_json_pointer_token(&self.table_name)
        );
        match &self.element {
            ObservationElement::Table => {}
            ObservationElement::Column(column_name) => {
                location.push_str("/columns/");
                location.push_str(&escape_json_pointer_token(column_name));
            }
            ObservationElement::Constraint(constraint_name) => {
                location.push_str("/constraints/");
                location.push_str(&escape_json_pointer_token(constraint_name));
            }
        }
        location
    }
}

/// Immutable receipt binding one exact observed source coordinate to snapshot provenance.
///
/// Receipts are issued only by [`PostgresSchemaSnapshot::source_receipt`], which verifies that the
/// requested coordinate actually exists in that snapshot. `source_id` is the stable source
/// connection reference supplied to the snapshot, never a credential.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceObservationReceipt {
    source_id: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: ObservationLocation,
}

impl SourceObservationReceipt {
    /// Returns the stable source reference used by candidate evidence binding.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable canonical snapshot digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the verified exact source coordinate inside the snapshot.
    #[must_use]
    pub const fn location(&self) -> &ObservationLocation {
        &self.location
    }
}

/// Immutable evidence that one bounded PostgreSQL schema snapshot was observed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshot {
    source_connection_key: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    tables: Vec<TableObservation>,
}

impl PostgresSchemaSnapshot {
    /// Creates a deterministic snapshot contract from already-bounded source metadata.
    ///
    /// Collection order is canonicalized by exact qualified table identifier. Exact source text is
    /// preserved, including case and characters that would require quoting in PostgreSQL.
    pub fn new(
        source_connection_key: impl Into<String>,
        snapshot_digest: impl Into<String>,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        mut tables: Vec<TableObservation>,
    ) -> Result<Self, ObservationError> {
        let source_connection_key = source_connection_key.into();
        let snapshot_digest = snapshot_digest.into();
        let extractor_revision = extractor_revision.into();
        let observed_at_utc = observed_at_utc.into();
        validate_nonblank(&source_connection_key, "source_connection_key")?;
        validate_snapshot_digest(&snapshot_digest)?;
        validate_nonblank(&extractor_revision, "extractor_revision")?;
        validate_nonblank(&observed_at_utc, "observed_at_utc")?;

        let mut table_coordinates = BTreeSet::new();
        for table in &tables {
            let coordinate = (table.schema_name.clone(), table.table_name.clone());
            if !table_coordinates.insert(coordinate) {
                return Err(ObservationError::DuplicateTableObservation {
                    schema_name: table.schema_name.clone(),
                    table_name: table.table_name.clone(),
                });
            }
        }
        tables.sort_by(|left, right| {
            (left.schema_name.as_str(), left.table_name.as_str())
                .cmp(&(right.schema_name.as_str(), right.table_name.as_str()))
        });
        Ok(Self {
            source_connection_key,
            snapshot_digest,
            extractor_revision,
            observed_at_utc,
            tables,
        })
    }

    /// Returns the stable source-connection reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the caller-supplied immutable snapshot digest identity.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns qualified tables in deterministic exact-identifier order.
    #[must_use]
    pub fn tables(&self) -> &[TableObservation] {
        &self.tables
    }

    /// Issues provenance for an exact coordinate only when that coordinate exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: ObservationLocation,
    ) -> Result<SourceObservationReceipt, ObservationError> {
        if !self.contains_location(&location) {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(SourceObservationReceipt {
            source_id: self.source_connection_key.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }

    fn contains_location(&self, location: &ObservationLocation) -> bool {
        let Some(table) = self.tables.iter().find(|table| {
            table.schema_name == location.schema_name && table.table_name == location.table_name
        }) else {
            return false;
        };

        match &location.element {
            ObservationElement::Table => true,
            ObservationElement::Column(column_name) => table
                .columns
                .iter()
                .any(|column| column.column_name == *column_name),
            ObservationElement::Constraint(constraint_name) => table
                .constraints
                .iter()
                .any(|constraint| constraint.constraint_name() == constraint_name),
        }
    }
}

fn validate_constraint_columns(
    constraint_name: &str,
    column_names: &[String],
    field: &'static str,
) -> Result<(), ObservationError> {
    if column_names.is_empty() {
        return Err(ObservationError::EmptyConstraintColumns {
            constraint_name: constraint_name.to_owned(),
        });
    }
    let mut seen_columns = BTreeSet::new();
    for column_name in column_names {
        validate_nonblank(column_name, field)?;
        if !seen_columns.insert(column_name.as_str()) {
            return Err(ObservationError::DuplicateConstraintColumn {
                constraint_name: constraint_name.to_owned(),
                column_name: column_name.clone(),
            });
        }
    }
    Ok(())
}

fn escape_json_pointer_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn validate_snapshot_digest(value: &str) -> Result<(), ObservationError> {
    let value_bytes = value.as_bytes();
    let is_canonical = value_bytes.len() == SHA256_DIGEST_PREFIX.len() + 64
        && value_bytes.starts_with(SHA256_DIGEST_PREFIX.as_bytes())
        && value_bytes[SHA256_DIGEST_PREFIX.len()..]
            .iter()
            .all(|byte| matches!(*byte, b'0'..=b'9' | b'a'..=b'f'));
    if !is_canonical {
        return Err(ObservationError::InvalidObservationField {
            field: "snapshot_digest",
        });
    }
    Ok(())
}

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(ObservationError::InvalidObservationField { field });
    }
    Ok(())
}
