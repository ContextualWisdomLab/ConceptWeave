//! Versioned successor observation value objects for PostgreSQL schema evidence.
//!
//! These contracts form the successor evidence family to the frozen v2 representation: relation
//! kind and relation comments, relation-scoped index evidence, schema-scoped domains and enums,
//! qualified column type bindings, domain semantics, and enum label order. The successor framing is
//! domain-separated from v2, so no v2 digest, receipt, or coordinate changes meaning because of this
//! module.

use std::collections::BTreeSet;

use conceptweave_source_port::AuthorizedObservationRequest;
use sha2::{Digest, Sha256};

use crate::model::{
    ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType, ForeignKeyReferenceBehavior,
    ObservationError, TableConstraintObservation, escape_json_pointer_token, validate_nonblank,
    validate_observed_at_utc,
};

/// Owner-computed successor snapshot digest domain separator.
const SNAPSHOT_DIGEST_DOMAIN_V3: &[u8] = b"conceptweave.postgres_schema_snapshot.v3";

/// Exact PostgreSQL schema that owns the immutable built-in type namespace.
const POSTGRES_CATALOG_SCHEMA_NAME: &str = "pg_catalog";

/// Exact schema-qualified PostgreSQL type coordinate.
///
/// The coordinate identifies a built-in, domain, or enum type without relying on `search_path`
/// resolution. Exact source text is preserved, including case and characters that would require
/// quoting in PostgreSQL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedTypeName {
    schema_name: String,
    type_name: String,
}

impl QualifiedTypeName {
    /// Creates a qualified type coordinate while preserving exact source text.
    pub fn new(
        schema_name: impl Into<String>,
        type_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let type_name = type_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&type_name, "type_name")?;
        Ok(Self {
            schema_name,
            type_name,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source type identifier.
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

/// Exact schema-qualified PostgreSQL collation coordinate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedCollationName {
    schema_name: String,
    collation_name: String,
}

impl QualifiedCollationName {
    /// Creates a qualified collation coordinate while preserving exact source text.
    pub fn new(
        schema_name: impl Into<String>,
        collation_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let collation_name = collation_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&collation_name, "collation_name")?;
        Ok(Self {
            schema_name,
            collation_name,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source collation identifier.
    #[must_use]
    pub fn collation_name(&self) -> &str {
        &self.collation_name
    }
}

/// PostgreSQL relation kind reported by `pg_class.relkind`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationKind {
    /// An ordinary table.
    Table,
    /// A partitioned table.
    PartitionedTable,
    /// A view.
    View,
    /// A materialized view.
    MaterializedView,
    /// A foreign table.
    ForeignTable,
    /// A sequence.
    Sequence,
    /// A standalone composite type relation.
    CompositeType,
}

impl RelationKind {
    fn tag(self) -> u8 {
        match self {
            Self::Table => 0,
            Self::PartitionedTable => 1,
            Self::View => 2,
            Self::MaterializedView => 3,
            Self::ForeignTable => 4,
            Self::Sequence => 5,
            Self::CompositeType => 6,
        }
    }

    /// Exact stable coordinate token preserving the relation kind without loss.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::PartitionedTable => "partitioned_table",
            Self::View => "view",
            Self::MaterializedView => "materialized_view",
            Self::ForeignTable => "foreign_table",
            Self::Sequence => "sequence",
            Self::CompositeType => "composite_type",
        }
    }
}

/// One immutable successor PostgreSQL column observation.
///
/// Display text and identity stay separate: [`Self::data_type`] preserves the exact source
/// rendering, while [`Self::type_binding`] carries the qualified type coordinate that semantic
/// identity depends on.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnObservationV3 {
    column_name: String,
    ordinal_position: u32,
    data_type: String,
    type_binding: QualifiedTypeName,
    nullable: bool,
    source_comment: Option<String>,
}

impl ColumnObservationV3 {
    /// Creates a column observation while preserving exact source text and qualified type identity.
    pub fn new(
        column_name: impl Into<String>,
        ordinal_position: u32,
        data_type: impl Into<String>,
        type_binding: QualifiedTypeName,
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
            type_binding,
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

    /// Returns the exact qualified type coordinate this column binds to.
    #[must_use]
    pub fn type_binding(&self) -> &QualifiedTypeName {
        &self.type_binding
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

/// PostgreSQL index attribute role within one index observation.
///
/// Key attributes participate in the index key and uniqueness, while INCLUDE attributes are
/// payload-only. The role is material identity: the same exact attribute in a different role is a
/// different observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexAttributeKind {
    /// A key attribute of the index.
    Key,
    /// A non-key payload attribute carried by `INCLUDE`.
    Include,
}

impl IndexAttributeKind {
    fn tag(self) -> u8 {
        match self {
            Self::Key => 0,
            Self::Include => 1,
        }
    }
}

/// One immutable index attribute in exact `indkey` position order.
///
/// A zero `indkey` position denotes a server-rendered expression rather than a simple column
/// reference, so the two forms are structurally distinct and never collapse into one string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexAttributeObservation {
    position: u32,
    kind: IndexAttributeKind,
    source: IndexAttributeSource,
}

/// Exact source of one index attribute position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IndexAttributeSource {
    /// A simple column reference addressed by exact attribute name.
    Column(String),
    /// An exact server-rendered expression occupying a zero `indkey` position.
    Expression(String),
}

impl IndexAttributeObservation {
    /// Creates a simple column index attribute from exact source attribute text.
    pub fn new(
        position: u32,
        kind: IndexAttributeKind,
        attribute_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::column(position, kind, attribute_name)
    }

    /// Creates a simple column index attribute from exact source attribute text.
    pub fn column(
        position: u32,
        kind: IndexAttributeKind,
        attribute_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let attribute_name = attribute_name.into();
        if position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        validate_nonblank(&attribute_name, "attribute_name")?;
        Ok(Self {
            position,
            kind,
            source: IndexAttributeSource::Column(attribute_name),
        })
    }

    /// Creates an expression index attribute from exact server-rendered expression text.
    pub fn expression(
        position: u32,
        kind: IndexAttributeKind,
        expression: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let expression = expression.into();
        if position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        validate_nonblank(&expression, "expression")?;
        Ok(Self {
            position,
            kind,
            source: IndexAttributeSource::Expression(expression),
        })
    }

    /// Returns the one-based source attribute position.
    #[must_use]
    pub const fn position(&self) -> u32 {
        self.position
    }

    /// Returns the key or INCLUDE role of this attribute.
    #[must_use]
    pub const fn kind(&self) -> IndexAttributeKind {
        self.kind
    }

    /// Returns the exact source column name for a simple column attribute.
    #[must_use]
    pub fn attribute_name(&self) -> Option<&str> {
        match &self.source {
            IndexAttributeSource::Column(attribute_name) => Some(attribute_name),
            IndexAttributeSource::Expression(_) => None,
        }
    }

    /// Returns the exact server-rendered expression for an expression attribute.
    #[must_use]
    pub fn expression_text(&self) -> Option<&str> {
        match &self.source {
            IndexAttributeSource::Expression(expression) => Some(expression),
            IndexAttributeSource::Column(_) => None,
        }
    }
}

/// Immutable observation of one relation-scoped PostgreSQL index.
///
/// Key attributes are held separately from INCLUDE payload attributes because the two roles are
/// materially distinct. A partial index carries its exact server-rendered predicate, while a
/// non-partial index carries no predicate so the two never collapse. Readiness, validity, and
/// liveness remain independently optional because the adapter may not observe every catalog flag.
/// Reconstructed definition text is exact server output (`pg_get_indexdef`), never original DDL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexObservation {
    index_name: String,
    is_unique: bool,
    nulls_not_distinct: Option<bool>,
    access_method: Option<String>,
    key_attributes: Vec<IndexAttributeObservation>,
    include_attributes: Vec<IndexAttributeObservation>,
    predicate: Option<String>,
    ready: Option<bool>,
    valid: Option<bool>,
    live: Option<bool>,
    index_definition: Option<String>,
    source_comment: Option<String>,
}

impl IndexObservation {
    /// Creates an index observation from exactly the observed material index evidence.
    ///
    /// Key and INCLUDE attributes are canonicalized into deterministic position order. Attribute
    /// roles must agree with their collection (`key_attributes` holds only key positions and
    /// `include_attributes` holds only payload positions), the combined one-based positions must be
    /// contiguous with every key position preceding every INCLUDE position, and INCLUDE attributes
    /// must be simple columns because PostgreSQL `INCLUDE` does not accept expressions. Every simple
    /// column attribute is validated against the owning relation columns when the index is attached
    /// to a relation observation.
    pub fn new(
        index_name: impl Into<String>,
        is_unique: bool,
        nulls_not_distinct: Option<bool>,
        mut key_attributes: Vec<IndexAttributeObservation>,
        mut include_attributes: Vec<IndexAttributeObservation>,
    ) -> Result<Self, ObservationError> {
        let index_name = index_name.into();
        validate_nonblank(&index_name, "index_name")?;
        key_attributes.sort_by_key(IndexAttributeObservation::position);
        include_attributes.sort_by_key(IndexAttributeObservation::position);
        Self::validate_attribute_layout(&key_attributes, &include_attributes)?;
        Ok(Self {
            index_name,
            is_unique,
            nulls_not_distinct,
            access_method: None,
            key_attributes,
            include_attributes,
            predicate: None,
            ready: None,
            valid: None,
            live: None,
            index_definition: None,
            source_comment: None,
        })
    }

    fn validate_attribute_layout(
        key_attributes: &[IndexAttributeObservation],
        include_attributes: &[IndexAttributeObservation],
    ) -> Result<(), ObservationError> {
        let layout_error = || ObservationError::InvalidObservationField {
            field: "index_attribute_layout",
        };
        for (expected_position, attribute) in (1u32..).zip(key_attributes) {
            if attribute.kind() != IndexAttributeKind::Key
                || attribute.position() != expected_position
            {
                return Err(layout_error());
            }
        }
        let key_count = key_attributes.len() as u32;
        for (offset, attribute) in include_attributes.iter().enumerate() {
            if attribute.kind() != IndexAttributeKind::Include
                || attribute.position() != key_count + offset as u32 + 1
                || attribute.attribute_name().is_none()
            {
                return Err(layout_error());
            }
        }
        Ok(())
    }

    /// Records the exact observed access method name, such as `btree` or `gin`.
    #[must_use]
    pub fn with_access_method(mut self, access_method: impl Into<String>) -> Self {
        self.access_method = Some(access_method.into());
        self
    }

    /// Records an exact server-rendered partial predicate, never original DDL.
    #[must_use]
    pub fn with_predicate(mut self, predicate: impl Into<String>) -> Self {
        self.predicate = Some(predicate.into());
        self
    }

    /// Records observed `pg_index.indisready` state, or leaves `None` when it was not captured.
    #[must_use]
    pub const fn with_ready(mut self, ready: bool) -> Self {
        self.ready = Some(ready);
        self
    }

    /// Records observed `pg_index.indisvalid` state, or leaves `None` when it was not captured.
    #[must_use]
    pub const fn with_valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }

    /// Records observed `pg_index.indislive` state, or leaves `None` when it was not captured.
    #[must_use]
    pub const fn with_live(mut self, live: bool) -> Self {
        self.live = Some(live);
        self
    }

    /// Records exact server-rendered `pg_get_indexdef` text, never original DDL.
    #[must_use]
    pub fn with_index_definition(mut self, index_definition: impl Into<String>) -> Self {
        self.index_definition = Some(index_definition.into());
        self
    }

    /// Records the exact optional index comment without inventing missing metadata.
    #[must_use]
    pub fn with_source_comment(mut self, source_comment: impl Into<String>) -> Self {
        self.source_comment = Some(source_comment.into());
        self
    }

    /// Returns the exact source index identifier.
    #[must_use]
    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    /// Returns whether the index enforces uniqueness.
    #[must_use]
    pub const fn is_unique(&self) -> bool {
        self.is_unique
    }

    /// Returns observed `NULLS NOT DISTINCT` state, or `None` when it was not captured.
    #[must_use]
    pub const fn nulls_not_distinct(&self) -> Option<bool> {
        self.nulls_not_distinct
    }

    /// Returns the exact observed access method name, or `None` when it was not captured.
    #[must_use]
    pub fn access_method(&self) -> Option<&str> {
        self.access_method.as_deref()
    }

    /// Returns key attributes in deterministic source position order.
    #[must_use]
    pub fn key_attributes(&self) -> &[IndexAttributeObservation] {
        &self.key_attributes
    }

    /// Returns INCLUDE payload attributes in deterministic source position order.
    #[must_use]
    pub fn include_attributes(&self) -> &[IndexAttributeObservation] {
        &self.include_attributes
    }

    /// Returns the exact server-rendered partial predicate, or `None` for a full index.
    #[must_use]
    pub fn predicate(&self) -> Option<&str> {
        self.predicate.as_deref()
    }

    /// Returns observed `pg_index.indisready` state, or `None` when it was not captured.
    #[must_use]
    pub const fn ready(&self) -> Option<bool> {
        self.ready
    }

    /// Returns observed `pg_index.indisvalid` state, or `None` when it was not captured.
    #[must_use]
    pub const fn valid(&self) -> Option<bool> {
        self.valid
    }

    /// Returns observed `pg_index.indislive` state, or `None` when it was not captured.
    #[must_use]
    pub const fn live(&self) -> Option<bool> {
        self.live
    }

    /// Returns exact server-rendered `pg_get_indexdef` text, or `None` when it was not captured.
    #[must_use]
    pub fn index_definition(&self) -> Option<&str> {
        self.index_definition.as_deref()
    }

    /// Returns the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }
}

/// One immutable domain `CHECK`-constraint observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainCheckConstraintObservation {
    constraint_name: String,
    check_definition: String,
    validated: bool,
    enforced: bool,
}

impl DomainCheckConstraintObservation {
    /// Creates a domain check-constraint observation from exact source definition and status.
    pub fn new(
        constraint_name: impl Into<String>,
        check_definition: impl Into<String>,
        validated: bool,
        enforced: bool,
    ) -> Result<Self, ObservationError> {
        let constraint_name = constraint_name.into();
        let check_definition = check_definition.into();
        validate_nonblank(&constraint_name, "constraint_name")?;
        validate_nonblank(&check_definition, "check_definition")?;
        Ok(Self {
            constraint_name,
            check_definition,
            validated,
            enforced,
        })
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns the exact server-rendered check definition, never original DDL.
    #[must_use]
    pub fn check_definition(&self) -> &str {
        &self.check_definition
    }

    /// Returns whether PostgreSQL reports the constraint as validated.
    #[must_use]
    pub const fn validated(&self) -> bool {
        self.validated
    }

    /// Returns whether PostgreSQL reports the constraint as enforced.
    #[must_use]
    pub const fn enforced(&self) -> bool {
        self.enforced
    }
}

/// Immutable observation of one schema-scoped PostgreSQL domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainObservation {
    schema_name: String,
    domain_name: String,
    base_type: QualifiedTypeName,
    type_modifier: Option<i32>,
    array_dimensions: Option<u32>,
    collation: Option<QualifiedCollationName>,
    not_null: Option<bool>,
    default_expression: Option<String>,
    check_constraints: Vec<DomainCheckConstraintObservation>,
    source_comment: Option<String>,
}

impl DomainObservation {
    /// Creates a domain observation from its exact qualified base type.
    ///
    /// Type modifier, array dimensions, collation, NOT NULL state, default expression, check
    /// constraints, and source comment stay unobserved until the matching builder records them.
    pub fn new(
        schema_name: impl Into<String>,
        domain_name: impl Into<String>,
        base_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let domain_name = domain_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&domain_name, "domain_name")?;
        Ok(Self {
            schema_name,
            domain_name,
            base_type,
            type_modifier: None,
            array_dimensions: None,
            collation: None,
            not_null: None,
            default_expression: None,
            check_constraints: Vec::new(),
            source_comment: None,
        })
    }

    /// Records the exact PostgreSQL type modifier, preserving provider semantics such as `-1`.
    #[must_use]
    pub const fn with_type_modifier(mut self, type_modifier: i32) -> Self {
        self.type_modifier = Some(type_modifier);
        self
    }

    /// Records the exact observed array dimension count.
    #[must_use]
    pub const fn with_array_dimensions(mut self, array_dimensions: u32) -> Self {
        self.array_dimensions = Some(array_dimensions);
        self
    }

    /// Records the exact qualified source collation.
    #[must_use]
    pub fn with_collation(mut self, collation: QualifiedCollationName) -> Self {
        self.collation = Some(collation);
        self
    }

    /// Records observed domain NOT NULL state without inferring a provider default.
    #[must_use]
    pub const fn with_not_null(mut self, not_null: bool) -> Self {
        self.not_null = Some(not_null);
        self
    }

    /// Records the exact server-rendered default expression, never original DDL.
    #[must_use]
    pub fn with_default_expression(mut self, default_expression: impl Into<String>) -> Self {
        self.default_expression = Some(default_expression.into());
        self
    }

    /// Records exact domain check constraints in deterministic source-name order.
    pub fn with_check_constraints(
        mut self,
        mut check_constraints: Vec<DomainCheckConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        let mut constraint_names = BTreeSet::new();
        for check_constraint in &check_constraints {
            let constraint_name = check_constraint.constraint_name();
            if !constraint_names.insert(constraint_name.to_owned()) {
                return Err(ObservationError::DuplicateDomainCheckConstraint {
                    schema_name: self.schema_name.clone(),
                    domain_name: self.domain_name.clone(),
                    constraint_name: constraint_name.to_owned(),
                });
            }
        }
        check_constraints
            .sort_by(|left, right| left.constraint_name().cmp(right.constraint_name()));
        self.check_constraints = check_constraints;
        Ok(self)
    }

    /// Records the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn with_source_comment(mut self, source_comment: impl Into<String>) -> Self {
        self.source_comment = Some(source_comment.into());
        self
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source domain identifier.
    #[must_use]
    pub fn domain_name(&self) -> &str {
        &self.domain_name
    }

    /// Returns the exact qualified base type coordinate.
    #[must_use]
    pub fn base_type(&self) -> &QualifiedTypeName {
        &self.base_type
    }

    /// Returns the observed type modifier, or `None` when it was not captured.
    #[must_use]
    pub const fn type_modifier(&self) -> Option<i32> {
        self.type_modifier
    }

    /// Returns the observed array dimension count, or `None` when it was not captured.
    #[must_use]
    pub const fn array_dimensions(&self) -> Option<u32> {
        self.array_dimensions
    }

    /// Returns the observed qualified collation, or `None` when it was not captured.
    #[must_use]
    pub const fn collation(&self) -> Option<&QualifiedCollationName> {
        self.collation.as_ref()
    }

    /// Returns observed NOT NULL state, or `None` when it was not captured.
    #[must_use]
    pub const fn not_null(&self) -> Option<bool> {
        self.not_null
    }

    /// Returns the exact server-rendered default expression, or `None` when absent or unobserved.
    #[must_use]
    pub fn default_expression(&self) -> Option<&str> {
        self.default_expression.as_deref()
    }

    /// Returns check constraints in deterministic exact source-name order.
    #[must_use]
    pub fn check_constraints(&self) -> &[DomainCheckConstraintObservation] {
        &self.check_constraints
    }

    /// Returns the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }
}

/// Immutable observation of one schema-scoped PostgreSQL enum.
///
/// Label order is preserved exactly because PostgreSQL compares enum values by `enumsortorder`.
/// Catalog row identifiers are join coordinates and are never part of this governed evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumObservation {
    schema_name: String,
    enum_name: String,
    labels: Vec<String>,
    source_comment: Option<String>,
}

impl EnumObservation {
    /// Creates an enum observation while preserving exact label text and label order.
    ///
    /// An empty label is exact source text and stays distinguishable from a missing value.
    pub fn new(
        schema_name: impl Into<String>,
        enum_name: impl Into<String>,
        labels: Vec<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let enum_name = enum_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&enum_name, "enum_name")?;
        let mut seen_labels = BTreeSet::new();
        for label in &labels {
            if !seen_labels.insert(label.as_str()) {
                return Err(ObservationError::DuplicateEnumLabel {
                    schema_name: schema_name.clone(),
                    enum_name: enum_name.clone(),
                    label: label.clone(),
                });
            }
        }
        Ok(Self {
            schema_name,
            enum_name,
            labels,
            source_comment: None,
        })
    }

    /// Records the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn with_source_comment(mut self, source_comment: impl Into<String>) -> Self {
        self.source_comment = Some(source_comment.into());
        self
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source enum identifier.
    #[must_use]
    pub fn enum_name(&self) -> &str {
        &self.enum_name
    }

    /// Returns labels in exact `enumsortorder` order without normalization.
    #[must_use]
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Returns the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }
}

/// Immutable observation of one schema-scoped PostgreSQL relation.
///
/// The relation carries its exact `pg_class.relkind`, optional relation comment, columns bound to
/// qualified type coordinates, relation-scoped index evidence, and the same deterministic
/// constraint vocabulary used by v2.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationObservation {
    schema_name: String,
    relation_name: String,
    kind: RelationKind,
    columns: Vec<ColumnObservationV3>,
    constraints: Vec<TableConstraintObservation>,
    indexes: Vec<IndexObservation>,
    source_comment: Option<String>,
}

impl RelationObservation {
    /// Creates a relation observation without constraint or comment evidence.
    ///
    /// Columns are canonicalized into deterministic ordinal order and exact identifiers are never
    /// normalized.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        kind: RelationKind,
        mut columns: Vec<ColumnObservationV3>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        validate_nonblank(&relation_name, "relation_name")?;

        let mut column_names = BTreeSet::new();
        let mut ordinal_positions = BTreeSet::new();
        for column in &columns {
            if !column_names.insert(column.column_name().to_owned()) {
                return Err(ObservationError::DuplicateColumnName {
                    schema_name: schema_name.clone(),
                    table_name: relation_name.clone(),
                    column_name: column.column_name().to_owned(),
                });
            }
            if !ordinal_positions.insert(column.ordinal_position()) {
                return Err(ObservationError::DuplicateColumnOrdinal {
                    schema_name: schema_name.clone(),
                    table_name: relation_name.clone(),
                    ordinal_position: column.ordinal_position(),
                });
            }
        }

        columns.sort_by(|left, right| {
            (left.ordinal_position(), left.column_name())
                .cmp(&(right.ordinal_position(), right.column_name()))
        });
        Ok(Self {
            schema_name,
            relation_name,
            kind,
            columns,
            constraints: Vec::new(),
            indexes: Vec::new(),
            source_comment: None,
        })
    }

    /// Replaces constraint evidence, preserving exact local-column coordinates.
    ///
    /// Constraints are canonicalized into deterministic source-name order. Constraints that expose
    /// local-column coordinates must refer to columns in this same relation observation.
    pub fn with_constraints(
        mut self,
        mut constraints: Vec<TableConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        let mut constraint_names = BTreeSet::new();
        for constraint in &constraints {
            let constraint_name = constraint.constraint_name();
            if !constraint_names.insert(constraint_name.to_owned()) {
                return Err(ObservationError::DuplicateConstraintName {
                    schema_name: self.schema_name.clone(),
                    table_name: self.relation_name.clone(),
                    constraint_name: constraint_name.to_owned(),
                });
            }
            for column_name in constraint.column_names() {
                if !self
                    .columns
                    .iter()
                    .any(|column| column.column_name() == column_name)
                {
                    return Err(ObservationError::UnknownConstraintColumn {
                        schema_name: self.schema_name.clone(),
                        table_name: self.relation_name.clone(),
                        constraint_name: constraint_name.to_owned(),
                        column_name: column_name.clone(),
                    });
                }
            }
        }
        constraints.sort_by(|left, right| left.constraint_name().cmp(right.constraint_name()));
        self.constraints = constraints;
        Ok(self)
    }

    /// Records the exact optional relation comment without inventing missing metadata.
    #[must_use]
    pub fn with_source_comment(mut self, source_comment: impl Into<String>) -> Self {
        self.source_comment = Some(source_comment.into());
        self
    }

    /// Replaces relation-scoped index evidence, preserving exact local-column coordinates.
    ///
    /// Indexes are canonicalized into deterministic exact source-name order. Every simple column
    /// key or INCLUDE attribute must resolve to a column on this same relation observation, while
    /// expression attributes stay structurally separate. Index names must be unique; attribute
    /// layout validity is enforced by [`IndexObservation::new`] before an index reaches a relation.
    pub fn with_indexes(
        mut self,
        mut indexes: Vec<IndexObservation>,
    ) -> Result<Self, ObservationError> {
        let mut index_names = BTreeSet::new();
        for index in &indexes {
            let index_name = index.index_name();
            if !index_names.insert(index_name.to_owned()) {
                return Err(ObservationError::DuplicateIndexObservation {
                    schema_name: self.schema_name.clone(),
                    relation_name: self.relation_name.clone(),
                    index_name: index_name.to_owned(),
                });
            }
            for attribute in index
                .key_attributes()
                .iter()
                .chain(index.include_attributes())
            {
                let Some(attribute_name) = attribute.attribute_name() else {
                    continue;
                };
                if !self
                    .columns
                    .iter()
                    .any(|column| column.column_name() == attribute_name)
                {
                    return Err(ObservationError::UnknownIndexAttribute {
                        schema_name: self.schema_name.clone(),
                        relation_name: self.relation_name.clone(),
                        index_name: index_name.to_owned(),
                        attribute_name: attribute_name.to_owned(),
                    });
                }
            }
        }
        indexes.sort_by(|left, right| left.index_name().cmp(right.index_name()));
        self.indexes = indexes;
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

    /// Returns the exact observed PostgreSQL relation kind.
    #[must_use]
    pub const fn kind(&self) -> RelationKind {
        self.kind
    }

    /// Returns columns in deterministic source ordinal order.
    #[must_use]
    pub fn columns(&self) -> &[ColumnObservationV3] {
        &self.columns
    }

    /// Returns constraints in deterministic exact source-name order.
    #[must_use]
    pub fn constraints(&self) -> &[TableConstraintObservation] {
        &self.constraints
    }

    /// Returns relation-scoped indexes in deterministic exact source-name order.
    #[must_use]
    pub fn indexes(&self) -> &[IndexObservation] {
        &self.indexes
    }

    /// Returns the exact optional source comment without inventing missing metadata.
    #[must_use]
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }
}

/// Stable kind discriminator for one schema-scoped evidence coordinate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaObjectLocationKind {
    /// A schema-scoped relation observation of any `pg_class.relkind`.
    Relation,
    /// A relation column observation.
    Column,
    /// A relation table-constraint observation.
    Constraint,
    /// A relation-scoped index observation.
    Index,
    /// A schema-scoped domain observation.
    Domain,
    /// A schema-scoped enum observation.
    Enum,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SchemaObjectElement {
    Relation {
        relation_name: String,
        kind: RelationKind,
    },
    Column {
        relation_name: String,
        kind: RelationKind,
        column_name: String,
    },
    Constraint {
        relation_name: String,
        kind: RelationKind,
        constraint_name: String,
    },
    Index {
        relation_name: String,
        kind: RelationKind,
        index_name: String,
    },
    Domain(String),
    Enum(String),
}

/// Exact structured location inside an immutable successor schema snapshot.
///
/// Relation-level coordinates use the successor vocabulary
/// `/schemas/{schema}/relations/{kind}/{name}` so the exact `pg_class.relkind` is carried in the
/// coordinate. Relation children append `/columns/{name}`, `/constraints/{name}`, or
/// `/indexes/{name}` under that kind-aware relation segment. Domain and enum coordinates use the
/// schema-scoped vocabulary `/schemas/{schema}/domains/{name}` and `/schemas/{schema}/enums/{name}`.
/// Every identifier token applies RFC 6901 escaping (`~` -> `~0`, `/` -> `~1`) without case or
/// Unicode normalization. Frozen v2 `/schemas/{schema}/tables/{table}` meaning is unchanged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaObjectLocation {
    schema_name: String,
    element: SchemaObjectElement,
}

impl SchemaObjectLocation {
    /// Creates a location for an exact schema-qualified relation of any observed kind.
    pub fn relation(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        kind: RelationKind,
    ) -> Result<Self, ObservationError> {
        let relation_name = relation_name.into();
        validate_nonblank(&relation_name, "relation_name")?;
        Self::new(
            schema_name,
            SchemaObjectElement::Relation {
                relation_name,
                kind,
            },
        )
    }

    /// Creates a location for an exact schema-qualified relation column.
    pub fn column(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_nonblank(&relation_name, "relation_name")?;
        validate_nonblank(&column_name, "column_name")?;
        Self::new(
            schema_name,
            SchemaObjectElement::Column {
                relation_name,
                kind,
                column_name,
            },
        )
    }

    /// Creates a location for an exact schema-qualified relation table constraint.
    pub fn constraint(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        kind: RelationKind,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        validate_nonblank(&relation_name, "relation_name")?;
        validate_nonblank(&constraint_name, "constraint_name")?;
        Self::new(
            schema_name,
            SchemaObjectElement::Constraint {
                relation_name,
                kind,
                constraint_name,
            },
        )
    }

    /// Creates a location for an exact relation-scoped index.
    pub fn index(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        kind: RelationKind,
        index_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let relation_name = relation_name.into();
        let index_name = index_name.into();
        validate_nonblank(&relation_name, "relation_name")?;
        validate_nonblank(&index_name, "index_name")?;
        Self::new(
            schema_name,
            SchemaObjectElement::Index {
                relation_name,
                kind,
                index_name,
            },
        )
    }

    /// Creates a location for an exact schema-scoped domain.
    pub fn domain(
        schema_name: impl Into<String>,
        domain_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let domain_name = domain_name.into();
        validate_nonblank(&domain_name, "domain_name")?;
        Self::new(schema_name, SchemaObjectElement::Domain(domain_name))
    }

    /// Creates a location for an exact schema-scoped enum.
    ///
    /// The trailing underscore keeps the constructor name legal Rust; the coordinate semantics are
    /// still the PostgreSQL `ENUM` type at `/schemas/{schema}/enums/{name}`.
    pub fn enum_(
        schema_name: impl Into<String>,
        enum_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let enum_name = enum_name.into();
        validate_nonblank(&enum_name, "enum_name")?;
        Self::new(schema_name, SchemaObjectElement::Enum(enum_name))
    }

    fn new(
        schema_name: impl Into<String>,
        element: SchemaObjectElement,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        validate_nonblank(&schema_name, "schema_name")?;
        Ok(Self {
            schema_name,
            element,
        })
    }

    /// Returns the coordinate kind without exposing mutable representation details.
    #[must_use]
    pub fn kind(&self) -> SchemaObjectLocationKind {
        match self.element {
            SchemaObjectElement::Relation { .. } => SchemaObjectLocationKind::Relation,
            SchemaObjectElement::Column { .. } => SchemaObjectLocationKind::Column,
            SchemaObjectElement::Constraint { .. } => SchemaObjectLocationKind::Constraint,
            SchemaObjectElement::Index { .. } => SchemaObjectLocationKind::Index,
            SchemaObjectElement::Domain(_) => SchemaObjectLocationKind::Domain,
            SchemaObjectElement::Enum(_) => SchemaObjectLocationKind::Enum,
        }
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact owning relation kind for relation-scoped coordinates.
    ///
    /// The kind is carried by every relation-level coordinate so a view, materialized view,
    /// sequence, composite type, or foreign table is never addressed with table vocabulary.
    #[must_use]
    pub fn relation_kind(&self) -> Option<RelationKind> {
        match &self.element {
            SchemaObjectElement::Relation { kind, .. }
            | SchemaObjectElement::Column { kind, .. }
            | SchemaObjectElement::Constraint { kind, .. }
            | SchemaObjectElement::Index { kind, .. } => Some(*kind),
            SchemaObjectElement::Domain(_) | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the owning relation identifier when this coordinate has one.
    #[must_use]
    pub fn relation_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Relation { relation_name, .. } => Some(relation_name),
            SchemaObjectElement::Column { relation_name, .. }
            | SchemaObjectElement::Constraint { relation_name, .. }
            | SchemaObjectElement::Index { relation_name, .. } => Some(relation_name),
            SchemaObjectElement::Domain(_) | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the exact source column identifier for a column coordinate.
    #[must_use]
    pub fn column_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Column { column_name, .. } => Some(column_name),
            SchemaObjectElement::Relation { .. }
            | SchemaObjectElement::Constraint { .. }
            | SchemaObjectElement::Index { .. }
            | SchemaObjectElement::Domain(_)
            | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the exact source constraint identifier for a constraint coordinate.
    #[must_use]
    pub fn constraint_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Constraint {
                constraint_name, ..
            } => Some(constraint_name),
            SchemaObjectElement::Relation { .. }
            | SchemaObjectElement::Column { .. }
            | SchemaObjectElement::Index { .. }
            | SchemaObjectElement::Domain(_)
            | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the exact source index identifier for an index coordinate.
    #[must_use]
    pub fn index_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Index { index_name, .. } => Some(index_name),
            SchemaObjectElement::Relation { .. }
            | SchemaObjectElement::Column { .. }
            | SchemaObjectElement::Constraint { .. }
            | SchemaObjectElement::Domain(_)
            | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the exact source domain identifier for a domain coordinate.
    #[must_use]
    pub fn domain_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Domain(domain_name) => Some(domain_name),
            SchemaObjectElement::Relation { .. }
            | SchemaObjectElement::Column { .. }
            | SchemaObjectElement::Constraint { .. }
            | SchemaObjectElement::Index { .. }
            | SchemaObjectElement::Enum(_) => None,
        }
    }

    /// Returns the exact source enum identifier for an enum coordinate.
    #[must_use]
    pub fn enum_name(&self) -> Option<&str> {
        match &self.element {
            SchemaObjectElement::Enum(enum_name) => Some(enum_name),
            SchemaObjectElement::Relation { .. }
            | SchemaObjectElement::Column { .. }
            | SchemaObjectElement::Constraint { .. }
            | SchemaObjectElement::Index { .. }
            | SchemaObjectElement::Domain(_) => None,
        }
    }

    /// Returns a deterministic collision-safe successor evidence location string.
    ///
    /// The vocabulary segments are ConceptWeave coordinate labels; identifier tokens use RFC 6901
    /// escaping and retain exact case and text. Relation-level coordinates carry the exact relation
    /// kind so non-table relations never receive table vocabulary.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        match &self.element {
            SchemaObjectElement::Relation {
                relation_name,
                kind,
            } => format!(
                "/schemas/{}/relations/{}/{}",
                escape_json_pointer_token(&self.schema_name),
                kind.token(),
                escape_json_pointer_token(relation_name)
            ),
            SchemaObjectElement::Column {
                relation_name,
                kind,
                column_name,
            } => format!(
                "/schemas/{}/relations/{}/{}/columns/{}",
                escape_json_pointer_token(&self.schema_name),
                kind.token(),
                escape_json_pointer_token(relation_name),
                escape_json_pointer_token(column_name)
            ),
            SchemaObjectElement::Constraint {
                relation_name,
                kind,
                constraint_name,
            } => format!(
                "/schemas/{}/relations/{}/{}/constraints/{}",
                escape_json_pointer_token(&self.schema_name),
                kind.token(),
                escape_json_pointer_token(relation_name),
                escape_json_pointer_token(constraint_name)
            ),
            SchemaObjectElement::Index {
                relation_name,
                kind,
                index_name,
            } => format!(
                "/schemas/{}/relations/{}/{}/indexes/{}",
                escape_json_pointer_token(&self.schema_name),
                kind.token(),
                escape_json_pointer_token(relation_name),
                escape_json_pointer_token(index_name)
            ),
            SchemaObjectElement::Domain(domain_name) => format!(
                "/schemas/{}/domains/{}",
                escape_json_pointer_token(&self.schema_name),
                escape_json_pointer_token(domain_name)
            ),
            SchemaObjectElement::Enum(enum_name) => format!(
                "/schemas/{}/enums/{}",
                escape_json_pointer_token(&self.schema_name),
                escape_json_pointer_token(enum_name)
            ),
        }
    }
}

/// Canonical successor object collections after deterministic ordering and invariant checks.
struct CanonicalSnapshotObjects {
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
}

/// Canonicalizes successor collections and enforces every cross-object invariant.
///
/// Collections are sorted by exact qualified identifier, duplicates fail closed, domain and enum
/// type coordinates must not collide, and every column type binding and domain base type must
/// resolve to the PostgreSQL built-in namespace or to a domain or enum observed in the same
/// snapshot. Type resolution never consults `search_path`.
fn canonicalize_snapshot_objects(
    mut relations: Vec<RelationObservation>,
    mut domains: Vec<DomainObservation>,
    mut enums: Vec<EnumObservation>,
) -> Result<CanonicalSnapshotObjects, ObservationError> {
    relations.sort_by(|left, right| {
        (left.schema_name(), left.relation_name())
            .cmp(&(right.schema_name(), right.relation_name()))
    });
    for pair in relations.windows(2) {
        let (left, right) = (&pair[0], &pair[1]);
        if left.schema_name == right.schema_name && left.relation_name == right.relation_name {
            return Err(ObservationError::DuplicateRelationObservation {
                schema_name: left.schema_name.clone(),
                relation_name: left.relation_name.clone(),
            });
        }
    }

    domains.sort_by(|left, right| {
        (left.schema_name(), left.domain_name()).cmp(&(right.schema_name(), right.domain_name()))
    });
    for pair in domains.windows(2) {
        let (left, right) = (&pair[0], &pair[1]);
        if left.schema_name == right.schema_name && left.domain_name == right.domain_name {
            return Err(ObservationError::DuplicateDomainObservation {
                schema_name: left.schema_name.clone(),
                domain_name: left.domain_name.clone(),
            });
        }
    }

    enums.sort_by(|left, right| {
        (left.schema_name(), left.enum_name()).cmp(&(right.schema_name(), right.enum_name()))
    });
    for pair in enums.windows(2) {
        let (left, right) = (&pair[0], &pair[1]);
        if left.schema_name == right.schema_name && left.enum_name == right.enum_name {
            return Err(ObservationError::DuplicateEnumObservation {
                schema_name: left.schema_name.clone(),
                enum_name: left.enum_name.clone(),
            });
        }
    }

    if let Some(domain) = domains.iter().find(|domain| {
        enums.iter().any(|observed_enum| {
            observed_enum.schema_name == domain.schema_name
                && observed_enum.enum_name == domain.domain_name
        })
    }) {
        return Err(ObservationError::DuplicateSchemaTypeName {
            schema_name: domain.schema_name.clone(),
            type_name: domain.domain_name.clone(),
        });
    }

    let resolves = |binding: &QualifiedTypeName| {
        if binding.schema_name == POSTGRES_CATALOG_SCHEMA_NAME {
            return true;
        }
        domains.iter().any(|domain| {
            domain.schema_name == binding.schema_name && domain.domain_name == binding.type_name
        }) || enums.iter().any(|observed_enum| {
            observed_enum.schema_name == binding.schema_name
                && observed_enum.enum_name == binding.type_name
        })
    };

    for domain in &domains {
        let binding = domain.base_type();
        if !resolves(binding) {
            return Err(ObservationError::UnknownTypeBinding {
                schema_name: binding.schema_name.clone(),
                type_name: binding.type_name.clone(),
            });
        }
    }

    for relation in &relations {
        for column in &relation.columns {
            let binding = &column.type_binding;
            if !resolves(binding) {
                return Err(ObservationError::UnknownTypeBinding {
                    schema_name: binding.schema_name.clone(),
                    type_name: binding.type_name.clone(),
                });
            }
        }
    }

    Ok(CanonicalSnapshotObjects {
        relations,
        domains,
        enums,
    })
}

/// Immutable receipt binding one exact successor coordinate to snapshot provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuccessorSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: SchemaObjectLocation,
}

impl SuccessorSourceReceipt {
    /// Returns the stable source reference used by candidate evidence binding.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the opaque immutable connection-policy revision used for this observation.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the immutable canonical successor snapshot digest.
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

    /// Returns the verified exact successor source coordinate inside the snapshot.
    #[must_use]
    pub const fn location(&self) -> &SchemaObjectLocation {
        &self.location
    }
}

/// Immutable evidence that one bounded PostgreSQL schema snapshot was observed.
///
/// The snapshot digest is computed by ConceptWeave from a versioned, domain-separated, deterministic
/// framing of the exact observed relation, column, domain, enum, and constraint metadata. Source
/// registry identity, connection-policy binding, extractor revision, and observation time remain
/// separate provenance coordinates and do not participate in source-content identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshotV3 {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
}

impl PostgresSchemaSnapshotV3 {
    /// Creates a deterministic successor snapshot from already-bounded, authorized source metadata.
    ///
    /// Collection order is canonicalized by exact qualified identifier before the digest is
    /// computed. Exact UTF-8 source text is preserved without Unicode, case, or quoting
    /// normalization. The complete registry-authorized request is required so every observed local
    /// schema can be checked against the exact request allowlist before immutable evidence or
    /// receipts are created, and so the authorized immutable connection-policy binding is retained
    /// as provenance. The observation time remains explicit provenance and must use the canonical
    /// UTC form enforced by the underlying observation contract.
    pub fn new(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
    ) -> Result<Self, ObservationError> {
        let allowed_schema_names = authorized_request.request().allowed_schema_names();
        let is_allowed = |schema_name: &str| {
            allowed_schema_names
                .iter()
                .any(|allowed| allowed == schema_name)
        };
        for relation in &relations {
            if !is_allowed(relation.schema_name()) {
                return Err(ObservationError::InvalidObservationField {
                    field: "unauthorized_schema_name",
                });
            }
        }
        for domain in &domains {
            if !is_allowed(domain.schema_name()) {
                return Err(ObservationError::InvalidObservationField {
                    field: "unauthorized_schema_name",
                });
            }
        }
        for observed_enum in &enums {
            if !is_allowed(observed_enum.schema_name()) {
                return Err(ObservationError::InvalidObservationField {
                    field: "unauthorized_schema_name",
                });
            }
        }

        let CanonicalSnapshotObjects {
            relations,
            domains,
            enums,
        } = canonicalize_snapshot_objects(relations, domains, enums)?;
        let snapshot_digest = compute_snapshot_digest_v3(&relations, &domains, &enums);
        let extractor_revision = extractor_revision.into();
        let observed_at_utc = observed_at_utc.into();
        validate_nonblank(&extractor_revision, "extractor_revision")?;
        validate_observed_at_utc(&observed_at_utc)?;
        Ok(Self {
            source_connection_key: authorized_request
                .source_connection()
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: authorized_request
                .source_connection()
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        })
    }

    /// Returns the stable source-connection reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the opaque immutable connection-policy revision authorized for this snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed canonical SHA-256 successor source-content digest.
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

    /// Returns qualified relations in deterministic exact-identifier order.
    #[must_use]
    pub fn relations(&self) -> &[RelationObservation] {
        &self.relations
    }

    /// Returns qualified domains in deterministic exact-identifier order.
    #[must_use]
    pub fn domains(&self) -> &[DomainObservation] {
        &self.domains
    }

    /// Returns qualified enums in deterministic exact-identifier order.
    #[must_use]
    pub fn enums(&self) -> &[EnumObservation] {
        &self.enums
    }

    /// Issues provenance for an exact successor coordinate only when it exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: SchemaObjectLocation,
    ) -> Result<SuccessorSourceReceipt, ObservationError> {
        if !self.contains_location(&location) {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(SuccessorSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location,
        })
    }

    fn contains_location(&self, location: &SchemaObjectLocation) -> bool {
        let relation_matches = |relation_name: &str, kind: RelationKind| {
            self.relations.iter().any(|relation| {
                relation.schema_name == location.schema_name
                    && relation.relation_name == relation_name
                    && relation.kind == kind
            })
        };
        let find_relation = |relation_name: &str, kind: RelationKind| {
            self.relations.iter().find(|relation| {
                relation.schema_name == location.schema_name
                    && relation.relation_name == relation_name
                    && relation.kind == kind
            })
        };

        match &location.element {
            SchemaObjectElement::Relation {
                relation_name,
                kind,
            } => relation_matches(relation_name, *kind),
            SchemaObjectElement::Column {
                relation_name,
                kind,
                column_name,
            } => find_relation(relation_name, *kind).is_some_and(|relation| {
                relation
                    .columns
                    .iter()
                    .any(|column| column.column_name == *column_name)
            }),
            SchemaObjectElement::Constraint {
                relation_name,
                kind,
                constraint_name,
            } => find_relation(relation_name, *kind).is_some_and(|relation| {
                relation
                    .constraints
                    .iter()
                    .any(|constraint| constraint.constraint_name() == constraint_name)
            }),
            SchemaObjectElement::Index {
                relation_name,
                kind,
                index_name,
            } => find_relation(relation_name, *kind).is_some_and(|relation| {
                relation
                    .indexes
                    .iter()
                    .any(|index| index.index_name() == index_name)
            }),
            SchemaObjectElement::Domain(domain_name) => self.domains.iter().any(|domain| {
                domain.schema_name == location.schema_name && domain.domain_name == *domain_name
            }),
            SchemaObjectElement::Enum(enum_name) => self.enums.iter().any(|observed_enum| {
                observed_enum.schema_name == location.schema_name
                    && observed_enum.enum_name == *enum_name
            }),
        }
    }
}

fn compute_snapshot_digest_v3(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3);

    encode_len(&mut hasher, relations.len());
    for relation in relations {
        encode_str(&mut hasher, relation.schema_name());
        encode_str(&mut hasher, relation.relation_name());
        hasher.update([relation.kind().tag()]);
        encode_optional_str(&mut hasher, relation.source_comment());

        encode_len(&mut hasher, relation.columns().len());
        for column in relation.columns() {
            encode_str(&mut hasher, column.column_name());
            hasher.update(column.ordinal_position().to_be_bytes());
            encode_str(&mut hasher, column.data_type());
            encode_str(&mut hasher, column.type_binding().schema_name());
            encode_str(&mut hasher, column.type_binding().type_name());
            encode_bool(&mut hasher, column.nullable());
            encode_optional_str(&mut hasher, column.source_comment());
        }

        encode_len(&mut hasher, relation.constraints().len());
        for constraint in relation.constraints() {
            encode_constraint(&mut hasher, constraint);
        }

        encode_len(&mut hasher, relation.indexes().len());
        for index in relation.indexes() {
            encode_index(&mut hasher, index);
        }
    }

    encode_len(&mut hasher, domains.len());
    for domain in domains {
        encode_str(&mut hasher, domain.schema_name());
        encode_str(&mut hasher, domain.domain_name());
        encode_str(&mut hasher, domain.base_type().schema_name());
        encode_str(&mut hasher, domain.base_type().type_name());
        encode_optional_i32(&mut hasher, domain.type_modifier());
        encode_optional_u32(&mut hasher, domain.array_dimensions());
        encode_optional_qualified_name(
            &mut hasher,
            domain
                .collation()
                .map(|collation| (collation.schema_name(), collation.collation_name())),
        );
        encode_optional_bool(&mut hasher, domain.not_null());
        encode_optional_str(&mut hasher, domain.default_expression());

        encode_len(&mut hasher, domain.check_constraints().len());
        for check_constraint in domain.check_constraints() {
            encode_str(&mut hasher, check_constraint.constraint_name());
            encode_str(&mut hasher, check_constraint.check_definition());
            encode_bool(&mut hasher, check_constraint.validated());
            encode_bool(&mut hasher, check_constraint.enforced());
        }
        encode_optional_str(&mut hasher, domain.source_comment());
    }

    encode_len(&mut hasher, enums.len());
    for observed_enum in enums {
        encode_str(&mut hasher, observed_enum.schema_name());
        encode_str(&mut hasher, observed_enum.enum_name());
        encode_str_slice(&mut hasher, observed_enum.labels());
        encode_optional_str(&mut hasher, observed_enum.source_comment());
    }

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

fn encode_constraint(hasher: &mut Sha256, constraint: &TableConstraintObservation) {
    match constraint {
        TableConstraintObservation::PrimaryKey(observation) => {
            hasher.update([0]);
            encode_str(hasher, observation.constraint_name());
            encode_str_slice(hasher, observation.column_names());
        }
        TableConstraintObservation::Unique(observation) => {
            hasher.update([1]);
            encode_str(hasher, observation.constraint_name());
            encode_str_slice(hasher, observation.column_names());
            encode_optional_bool(hasher, observation.nulls_not_distinct());
        }
        TableConstraintObservation::ForeignKey(observation) => {
            hasher.update([2]);
            encode_str(hasher, observation.constraint_name());
            encode_str_slice(hasher, observation.column_names());
            encode_str(hasher, observation.referenced_schema_name());
            encode_str(hasher, observation.referenced_table_name());
            encode_str_slice(hasher, observation.referenced_column_names());
            encode_reference_behavior(hasher, observation.reference_behavior());
            encode_optional_bool(hasher, observation.validated());
            encode_optional_bool(hasher, observation.enforced());
        }
        TableConstraintObservation::Check(observation) => {
            hasher.update([3]);
            encode_str(hasher, observation.constraint_name());
            encode_str(hasher, observation.definition());
            encode_bool(hasher, observation.validated());
            encode_bool(hasher, observation.enforced());
            encode_bool(hasher, observation.no_inherit());
        }
    }
}

fn encode_index(hasher: &mut Sha256, index: &IndexObservation) {
    encode_str(hasher, index.index_name());
    encode_bool(hasher, index.is_unique());
    encode_optional_bool(hasher, index.nulls_not_distinct());
    encode_optional_str(hasher, index.access_method());

    encode_len(hasher, index.key_attributes().len());
    for attribute in index.key_attributes() {
        encode_index_attribute(hasher, attribute);
    }

    encode_len(hasher, index.include_attributes().len());
    for attribute in index.include_attributes() {
        encode_index_attribute(hasher, attribute);
    }

    encode_optional_str(hasher, index.predicate());
    encode_optional_bool(hasher, index.ready());
    encode_optional_bool(hasher, index.valid());
    encode_optional_bool(hasher, index.live());
    encode_optional_str(hasher, index.index_definition());
    encode_optional_str(hasher, index.source_comment());
}

fn encode_index_attribute(hasher: &mut Sha256, attribute: &IndexAttributeObservation) {
    hasher.update(attribute.position().to_be_bytes());
    hasher.update([attribute.kind().tag()]);
    match &attribute.source {
        IndexAttributeSource::Column(attribute_name) => {
            hasher.update([0]);
            encode_str(hasher, attribute_name);
        }
        IndexAttributeSource::Expression(expression) => {
            hasher.update([1]);
            encode_str(hasher, expression);
        }
    }
}

fn encode_reference_behavior(hasher: &mut Sha256, behavior: Option<&ForeignKeyReferenceBehavior>) {
    match behavior {
        None => hasher.update([0]),
        Some(behavior) => {
            hasher.update([1]);
            encode_foreign_key_action(hasher, behavior.update_action());
            encode_foreign_key_action(hasher, behavior.delete_action());
            match behavior.delete_target_columns() {
                None => hasher.update([0]),
                Some(columns) => {
                    hasher.update([1]);
                    encode_str_slice(hasher, columns);
                }
            }
            encode_foreign_key_match_type(hasher, behavior.match_type());
            encode_foreign_key_deferrability(hasher, behavior.deferrability());
        }
    }
}

fn encode_foreign_key_action(hasher: &mut Sha256, action: ForeignKeyAction) {
    let tag = match action {
        ForeignKeyAction::NoAction => 0,
        ForeignKeyAction::Restrict => 1,
        ForeignKeyAction::Cascade => 2,
        ForeignKeyAction::SetNull => 3,
        ForeignKeyAction::SetDefault => 4,
    };
    hasher.update([tag]);
}

fn encode_foreign_key_match_type(hasher: &mut Sha256, match_type: ForeignKeyMatchType) {
    let tag = match match_type {
        ForeignKeyMatchType::Simple => 0,
        ForeignKeyMatchType::Full => 1,
        ForeignKeyMatchType::Partial => 2,
    };
    hasher.update([tag]);
}

fn encode_foreign_key_deferrability(hasher: &mut Sha256, deferrability: ForeignKeyDeferrability) {
    let tag = match deferrability {
        ForeignKeyDeferrability::NotDeferrable => 0,
        ForeignKeyDeferrability::InitiallyImmediate => 1,
        ForeignKeyDeferrability::InitiallyDeferred => 2,
    };
    hasher.update([tag]);
}

fn encode_optional_bool(hasher: &mut Sha256, value: Option<bool>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            encode_bool(hasher, value);
        }
    }
}

fn encode_optional_str(hasher: &mut Sha256, value: Option<&str>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            encode_str(hasher, value);
        }
    }
}

fn encode_optional_i32(hasher: &mut Sha256, value: Option<i32>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            hasher.update(value.to_be_bytes());
        }
    }
}

fn encode_optional_u32(hasher: &mut Sha256, value: Option<u32>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            hasher.update(value.to_be_bytes());
        }
    }
}

fn encode_optional_qualified_name(hasher: &mut Sha256, value: Option<(&str, &str)>) {
    match value {
        None => hasher.update([0]),
        Some((namespace, name)) => {
            hasher.update([1]);
            encode_str(hasher, namespace);
            encode_str(hasher, name);
        }
    }
}

fn encode_str_slice(hasher: &mut Sha256, values: &[String]) {
    encode_len(hasher, values.len());
    for value in values {
        encode_str(hasher, value);
    }
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

fn encode_bool(hasher: &mut Sha256, value: bool) {
    hasher.update([u8::from(value)]);
}
