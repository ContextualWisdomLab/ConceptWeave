#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Deterministic draft proposals from immutable relational source receipts.
//!
//! A table is only a possible concept and a foreign key is only a possible semantic relation.
//! This boundary makes no business-truth or publication decision.

use std::collections::BTreeMap;
use std::fmt;

use conceptweave_domain::{CandidateKind, ContractError, EvidenceReference, SemanticCandidate};
use conceptweave_observation::{
    DomainObservation, EnumObservation, ForeignKeyReferenceBehavior, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
    SchemaObjectLocation, TableConstraintObservation,
};

const REVISION: &str = "conceptweave.relational_proposal.v1";

/// One source column offered as a physical-to-semantic field mapping candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedField {
    candidate: SemanticCandidate,
    source_name: String,
    ordinal_position: u32,
    display_type: String,
    type_binding: QualifiedTypeName,
    nullable: bool,
    source_comment: Option<String>,
}

impl ProposedField {
    /// Returns the draft, inferred physical-mapping candidate.
    pub const fn candidate(&self) -> &SemanticCandidate {
        &self.candidate
    }

    /// Returns the exact observed column name.
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Returns source column order.
    pub const fn ordinal_position(&self) -> u32 {
        self.ordinal_position
    }

    /// Returns PostgreSQL's display type, separate from exact type identity.
    pub fn display_type(&self) -> &str {
        &self.display_type
    }

    /// Returns the exact qualified source type.
    pub const fn type_binding(&self) -> &QualifiedTypeName {
        &self.type_binding
    }

    /// Returns observed nullability.
    pub const fn nullable(&self) -> bool {
        self.nullable
    }

    /// Returns the exact optional source comment as observed, without treating it as business truth.
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }

    /// Returns the verified column evidence.
    pub fn evidence(&self) -> &EvidenceReference {
        &self.candidate.evidence()[0]
    }
}

/// A table-shaped concept proposal awaiting semantic review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedConcept {
    candidate: SemanticCandidate,
    source_schema: String,
    source_relation: String,
    source_comment: Option<String>,
    fields: Vec<ProposedField>,
    primary_key_columns: Option<Vec<String>>,
}

impl ProposedConcept {
    /// Returns the draft, inferred domain candidate.
    pub const fn candidate(&self) -> &SemanticCandidate {
        &self.candidate
    }

    /// Returns the exact source schema name.
    pub fn source_schema(&self) -> &str {
        &self.source_schema
    }

    /// Returns the exact source table name, offered as a review label only.
    pub fn source_relation(&self) -> &str {
        &self.source_relation
    }

    /// Returns the exact optional source comment as observed, without treating it as business truth.
    pub fn source_comment(&self) -> Option<&str> {
        self.source_comment.as_deref()
    }

    /// Returns observed columns in source order.
    pub fn fields(&self) -> &[ProposedField] {
        &self.fields
    }

    /// Returns observed primary-key columns, or `None` when no primary key was observed.
    pub fn primary_key_columns(&self) -> Option<&[String]> {
        self.primary_key_columns.as_deref()
    }
}

/// A foreign-key-shaped relationship proposal awaiting semantic review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedRelation {
    candidate: SemanticCandidate,
    from_concept_id: String,
    to_concept_id: String,
    source_constraint: String,
    column_pairs: Vec<(String, String)>,
    reference_behavior: ForeignKeyReferenceBehavior,
    validated: bool,
    enforced: bool,
}

impl ProposedRelation {
    /// Returns the draft, inferred domain candidate.
    pub const fn candidate(&self) -> &SemanticCandidate {
        &self.candidate
    }

    /// Returns the proposed source concept identifier.
    pub fn from_concept_id(&self) -> &str {
        &self.from_concept_id
    }

    /// Returns the proposed target concept identifier.
    pub fn to_concept_id(&self) -> &str {
        &self.to_concept_id
    }

    /// Returns the exact observed foreign-key name.
    pub fn source_constraint(&self) -> &str {
        &self.source_constraint
    }

    /// Returns local and referenced source columns in foreign-key order.
    pub fn column_pairs(&self) -> &[(String, String)] {
        &self.column_pairs
    }

    /// Returns observed match, update, and delete behavior without semantic inference.
    pub const fn reference_behavior(&self) -> &ForeignKeyReferenceBehavior {
        &self.reference_behavior
    }

    /// Returns the observed PostgreSQL validation state.
    pub const fn validated(&self) -> bool {
        self.validated
    }

    /// Returns the observed PostgreSQL enforcement state.
    pub const fn enforced(&self) -> bool {
        self.enforced
    }
}

/// A schema-scoped type offered for semantic review with its complete source observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposedSourceType {
    /// A domain may suggest a governed constraint; its source definition remains evidence.
    Domain {
        /// Draft, inferred candidate bound to the domain receipt.
        candidate: SemanticCandidate,
        /// Exact source domain observation.
        observation: DomainObservation,
    },
    /// An enum may suggest a governed dimension; its labels remain source evidence.
    Enum {
        /// Draft, inferred candidate bound to the enum receipt.
        candidate: SemanticCandidate,
        /// Exact source enum observation.
        observation: EnumObservation,
    },
    /// A standalone composite type may suggest a structured concept; its attributes remain source evidence.
    Composite {
        /// Draft candidate bound to the exact composite relation receipt.
        candidate: SemanticCandidate,
        /// Exact source composite type observation.
        observation: RelationObservation,
    },
}

impl ProposedSourceType {
    /// Returns the draft, inferred domain candidate.
    pub const fn candidate(&self) -> &SemanticCandidate {
        match self {
            Self::Domain { candidate, .. }
            | Self::Enum { candidate, .. }
            | Self::Composite { candidate, .. } => candidate,
        }
    }
}

/// Immutable review draft derived from one exact source snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalProposal {
    proposal_id: String,
    source_id: String,
    source_digest: String,
    concepts: Vec<ProposedConcept>,
    relations: Vec<ProposedRelation>,
    source_types: Vec<ProposedSourceType>,
}

impl RelationalProposal {
    /// Returns the deterministic proposal identity for this source revision.
    pub fn proposal_id(&self) -> &str {
        &self.proposal_id
    }

    /// Returns the source registry key, never a credential.
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the exact owner-computed source-content digest.
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns proposed table concepts in exact source order.
    pub fn concepts(&self) -> &[ProposedConcept] {
        &self.concepts
    }

    /// Returns proposed foreign-key relations in exact source order.
    pub fn relations(&self) -> &[ProposedRelation] {
        &self.relations
    }

    /// Returns schema-scoped source type proposals in deterministic coordinate order.
    pub fn source_types(&self) -> &[ProposedSourceType] {
        &self.source_types
    }
}

/// A source fact cannot be safely turned into this bounded proposal.
#[derive(Debug)]
pub enum ProposalError {
    /// The snapshot cannot issue a receipt for required evidence.
    Observation(ObservationError),
    /// Domain candidate or evidence construction failed.
    Contract(ContractError),
    /// This proposal path cannot represent this relation kind.
    UnsupportedRelationKind,
    /// The referenced table or column is outside the complete proposal.
    MissingForeignKeyTarget,
    /// Foreign-key validation or enforcement was not observed.
    UnobservedForeignKeyState,
    /// Catalog families needed for a complete relational proposal were not observed.
    IncompleteSourceObservation,
}

impl fmt::Display for ProposalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Observation(_) => formatter.write_str("source evidence could not be verified"),
            Self::Contract(_) => formatter.write_str("proposal evidence could not be validated"),
            Self::UnsupportedRelationKind => {
                formatter.write_str("this source contains a relation the proposal cannot represent")
            }
            Self::MissingForeignKeyTarget => {
                formatter.write_str("a related source object is missing from the observation")
            }
            Self::UnobservedForeignKeyState => {
                formatter.write_str("relationship validation evidence is incomplete")
            }
            Self::IncompleteSourceObservation => {
                formatter.write_str("source observation is incomplete for this proposal")
            }
        }
    }
}

impl std::error::Error for ProposalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Observation(error) => Some(error),
            Self::Contract(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ObservationError> for ProposalError {
    fn from(error: ObservationError) -> Self {
        Self::Observation(error)
    }
}

impl From<ContractError> for ProposalError {
    fn from(error: ContractError) -> Self {
        Self::Contract(error)
    }
}

fn evidence(
    snapshot: &PostgresSchemaSnapshotV3,
    location: SchemaObjectLocation,
) -> Result<EvidenceReference, ProposalError> {
    let receipt = snapshot.source_receipt(location)?;
    Ok(EvidenceReference::new(
        receipt.source_id(),
        receipt.source_digest(),
        receipt.location().canonical_location(),
    )?)
}

fn candidate_id(source_id: &str, kind: &str, location: &SchemaObjectLocation) -> String {
    format!(
        "cw:{kind}:{}:{source_id}{}",
        source_id.len(),
        location.canonical_location()
    )
}

/// Builds a deterministic draft from exact relation, column, and foreign-key receipts.
///
/// Unknown relation kinds, out-of-scope targets, and unobserved foreign-key state fail closed.
/// No source comment or table name is promoted to authoritative business truth.
pub fn propose_relational_model(
    snapshot: &PostgresSchemaSnapshotV3,
) -> Result<RelationalProposal, ProposalError> {
    let source_id = snapshot.source_connection_key();
    let mut concept_ids = BTreeMap::new();
    let mut concepts = Vec::with_capacity(snapshot.relations().len());
    for relation in snapshot.relations() {
        if relation.kind() == RelationKind::CompositeType {
            continue;
        }
        if relation.kind() != RelationKind::Table {
            return Err(ProposalError::UnsupportedRelationKind);
        }
        let location = SchemaObjectLocation::relation(
            relation.schema_name(),
            relation.relation_name(),
            relation.kind(),
        )?;
        let id = candidate_id(source_id, "concept", &location);
        let candidate = SemanticCandidate::new(
            &id,
            CandidateKind::Concept,
            vec![evidence(snapshot, location)?],
        )?;
        let fields = relation
            .columns()
            .iter()
            .map(|column| {
                let location = SchemaObjectLocation::column(
                    relation.schema_name(),
                    relation.relation_name(),
                    relation.kind(),
                    column.column_name(),
                )?;
                Ok(ProposedField {
                    candidate: SemanticCandidate::new(
                        candidate_id(source_id, "field", &location),
                        CandidateKind::PhysicalMapping,
                        vec![evidence(snapshot, location)?],
                    )?,
                    source_name: column.column_name().to_owned(),
                    ordinal_position: column.ordinal_position(),
                    display_type: column.data_type().to_owned(),
                    type_binding: column.type_binding().clone(),
                    nullable: column.nullable(),
                    source_comment: column.source_comment().map(str::to_owned),
                })
            })
            .collect::<Result<Vec<_>, ProposalError>>()?;
        concept_ids.insert((relation.schema_name(), relation.relation_name()), id);
        concepts.push(ProposedConcept {
            candidate,
            source_schema: relation.schema_name().to_owned(),
            source_relation: relation.relation_name().to_owned(),
            source_comment: relation.source_comment().map(str::to_owned),
            fields,
            primary_key_columns: relation.constraints().iter().find_map(|constraint| {
                if let TableConstraintObservation::PrimaryKey(primary_key) = constraint {
                    Some(primary_key.column_names().to_vec())
                } else {
                    None
                }
            }),
        });
    }

    let mut relations = Vec::new();
    for relation in snapshot.relations() {
        for constraint in relation.constraints() {
            let TableConstraintObservation::ForeignKey(foreign_key) = constraint else {
                continue;
            };
            let from = concept_ids
                .get(&(relation.schema_name(), relation.relation_name()))
                .ok_or(ProposalError::MissingForeignKeyTarget)?;
            let to = concept_ids
                .get(&(
                    foreign_key.referenced_schema_name(),
                    foreign_key.referenced_table_name(),
                ))
                .ok_or(ProposalError::MissingForeignKeyTarget)?;
            let target = snapshot
                .relations()
                .iter()
                .find(|target| {
                    target.schema_name() == foreign_key.referenced_schema_name()
                        && target.relation_name() == foreign_key.referenced_table_name()
                })
                .ok_or(ProposalError::MissingForeignKeyTarget)?;
            if foreign_key.referenced_column_names().iter().any(|name| {
                !target
                    .columns()
                    .iter()
                    .any(|column| column.column_name() == name)
            }) {
                return Err(ProposalError::MissingForeignKeyTarget);
            }
            let (Some(validated), Some(enforced), Some(reference_behavior)) = (
                foreign_key.validated(),
                foreign_key.enforced(),
                foreign_key.reference_behavior(),
            ) else {
                return Err(ProposalError::UnobservedForeignKeyState);
            };
            let location = SchemaObjectLocation::constraint(
                relation.schema_name(),
                relation.relation_name(),
                relation.kind(),
                foreign_key.constraint_name(),
            )?;
            let candidate = SemanticCandidate::new(
                candidate_id(source_id, "relation", &location),
                CandidateKind::SemanticRelation,
                vec![evidence(snapshot, location)?],
            )?;
            relations.push(ProposedRelation {
                candidate,
                from_concept_id: from.clone(),
                to_concept_id: to.clone(),
                source_constraint: foreign_key.constraint_name().to_owned(),
                column_pairs: foreign_key
                    .column_names()
                    .iter()
                    .cloned()
                    .zip(foreign_key.referenced_column_names().iter().cloned())
                    .collect(),
                reference_behavior: reference_behavior.clone(),
                validated,
                enforced,
            });
        }
    }
    let mut source_types = Vec::with_capacity(
        snapshot.domains().len() + snapshot.enums().len() + snapshot.relations().len(),
    );
    for relation in snapshot.relations() {
        if relation.kind() != RelationKind::CompositeType {
            continue;
        }
        let location = SchemaObjectLocation::relation(
            relation.schema_name(),
            relation.relation_name(),
            relation.kind(),
        )?;
        source_types.push(ProposedSourceType::Composite {
            candidate: SemanticCandidate::new(
                candidate_id(source_id, "composite", &location),
                CandidateKind::Concept,
                vec![evidence(snapshot, location)?],
            )?,
            observation: relation.clone(),
        });
    }
    for domain in snapshot.domains() {
        let location = SchemaObjectLocation::domain(domain.schema_name(), domain.domain_name())?;
        source_types.push(ProposedSourceType::Domain {
            candidate: SemanticCandidate::new(
                candidate_id(source_id, "domain", &location),
                CandidateKind::Constraint,
                vec![evidence(snapshot, location)?],
            )?,
            observation: domain.clone(),
        });
    }
    for observed_enum in snapshot.enums() {
        let location =
            SchemaObjectLocation::enum_(observed_enum.schema_name(), observed_enum.enum_name())?;
        source_types.push(ProposedSourceType::Enum {
            candidate: SemanticCandidate::new(
                candidate_id(source_id, "enum", &location),
                CandidateKind::Dimension,
                vec![evidence(snapshot, location)?],
            )?,
            observation: observed_enum.clone(),
        });
    }
    source_types.sort_by(|left, right| {
        left.candidate()
            .candidate_id()
            .cmp(right.candidate().candidate_id())
    });
    if !snapshot.relations().is_empty()
        && (snapshot.column_collations().is_none()
            || snapshot.column_generations().is_none()
            || snapshot.column_expressions().is_none()
            || snapshot.not_null_constraints().is_none()
            || snapshot.constraint_timings().is_none()
            || snapshot.foreign_key_catalog().is_none()
            || snapshot.collation_definitions().is_none())
    {
        return Err(ProposalError::IncompleteSourceObservation);
    }
    Ok(RelationalProposal {
        proposal_id: format!(
            "{REVISION}:{}:{source_id}:{}",
            source_id.len(),
            snapshot.snapshot_digest()
        ),
        source_id: source_id.to_owned(),
        source_digest: snapshot.snapshot_digest().to_owned(),
        concepts,
        relations,
        source_types,
    })
}
