#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Core domain contracts for ConceptWeave.
//!
//! ConceptWeave separates observed source evidence from inferred semantic
//! candidates and from reviewed, published semantic-model truth. This crate
//! contains only that domain contract; adapters, persistence, LLM orchestration,
//! and publication formats belong to other bounded contexts.

use core::fmt;

/// The kind of source material observed by ConceptWeave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    /// A relational schema or database-introspection snapshot.
    RelationalSchema,
    /// An OpenAPI contract.
    OpenApi,
    /// An AsyncAPI or event contract.
    AsyncApi,
    /// Human-authored documentation or a business glossary.
    Document,
    /// Source-code structure observed through a bounded adapter.
    SourceCode,
    /// An existing ontology or controlled vocabulary.
    ExistingOntology,
    /// Provenance or lineage evidence produced by another system.
    Lineage,
}

/// The semantic artifact that a candidate proposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateKind {
    /// A domain concept or class.
    Concept,
    /// A broader/narrower or parent/child taxonomy relation.
    TaxonomyRelation,
    /// A non-taxonomic semantic relation or object property.
    SemanticRelation,
    /// A data-quality, cardinality, or semantic constraint.
    Constraint,
    /// An analytical dimension.
    Dimension,
    /// A governed analytical measure or metric definition.
    Measure,
    /// A mapping between a physical source element and a semantic concept.
    PhysicalMapping,
}

/// The epistemic status of a fact or relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthStatus {
    /// Directly observed from a source without semantic inference.
    Observed,
    /// Derived by deterministic or model-assisted inference.
    Inferred,
    /// Explicitly proposed for governance review.
    Proposed,
    /// Approved and published by the owning governance process.
    Authoritative,
    /// Previously authoritative but replaced by a newer fact or release.
    Superseded,
    /// Explicitly rejected by validation or governance review.
    Rejected,
}

/// The governance state of a semantic candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationState {
    /// Newly discovered and not yet submitted for validation.
    Draft,
    /// Submitted as a candidate for validation.
    Proposed,
    /// Passed deterministic validation and consistency checks.
    Validated,
    /// Reviewed by an authorized semantic steward or equivalent workflow.
    Reviewed,
    /// Published as governed semantic truth.
    Published,
    /// Replaced by a later published release.
    Superseded,
    /// Rejected and no longer eligible for publication.
    Rejected,
}

/// A stable reference to the evidence supporting a semantic candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceReference {
    /// Stable identifier of the observed source snapshot or artifact.
    pub source_id: String,
    /// Content digest of the exact source revision used as evidence.
    pub source_digest: String,
    /// Human- and machine-readable location within the source artifact.
    pub location: String,
}

impl EvidenceReference {
    /// Creates an evidence reference, rejecting blank identity fields.
    pub fn new(
        source_id: impl Into<String>,
        source_digest: impl Into<String>,
        location: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let source_id = source_id.into();
        let source_digest = source_digest.into();
        let location = location.into();
        if source_id.trim().is_empty() {
            return Err(ContractError::EmptyField("source_id"));
        }
        if source_digest.trim().is_empty() {
            return Err(ContractError::EmptyField("source_digest"));
        }
        if location.trim().is_empty() {
            return Err(ContractError::EmptyField("location"));
        }
        Ok(Self {
            source_id,
            source_digest,
            location,
        })
    }
}

/// A governed candidate for an ontology or semantic-layer artifact.
///
/// External consumers cannot mutate evidence or governance state without a
/// validated domain operation.
///
/// ```compile_fail
/// use conceptweave_domain::{CandidateKind, EvidenceReference, SemanticCandidate};
///
/// let evidence = EvidenceReference::new("source-1", "sha256:abc", "public.orders").unwrap();
/// let mut candidate = SemanticCandidate::new("candidate-1", CandidateKind::Concept, vec![evidence]).unwrap();
/// candidate.evidence.clear();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCandidate {
    candidate_id: String,
    kind: CandidateKind,
    truth_status: TruthStatus,
    publication_state: PublicationState,
    evidence: Vec<EvidenceReference>,
}

impl SemanticCandidate {
    /// Creates an inferred draft candidate with at least one evidence reference.
    pub fn new(
        candidate_id: impl Into<String>,
        kind: CandidateKind,
        evidence: Vec<EvidenceReference>,
    ) -> Result<Self, ContractError> {
        let candidate_id = candidate_id.into();
        if candidate_id.trim().is_empty() {
            return Err(ContractError::EmptyField("candidate_id"));
        }
        if evidence.is_empty() {
            return Err(ContractError::MissingEvidence);
        }
        let publication_state = PublicationState::Draft;
        Ok(Self {
            candidate_id,
            kind,
            truth_status: truth_for_state(publication_state),
            publication_state,
            evidence,
        })
    }

    /// Returns the stable candidate identifier.
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Returns the semantic artifact kind proposed by this candidate.
    pub fn kind(&self) -> CandidateKind {
        self.kind
    }

    /// Returns the candidate's current epistemic status.
    pub fn truth_status(&self) -> TruthStatus {
        self.truth_status
    }

    /// Returns the candidate's current governance/publication state.
    pub fn publication_state(&self) -> PublicationState {
        self.publication_state
    }

    /// Returns the immutable evidence references supporting this candidate.
    pub fn evidence(&self) -> &[EvidenceReference] {
        &self.evidence
    }

    /// Moves the candidate through the fail-closed governance lifecycle.
    pub fn transition(&mut self, target: PublicationState) -> Result<(), ContractError> {
        let from = self.publication_state;
        if !ALLOWED_TRANSITIONS.contains(&(from, target)) {
            return Err(ContractError::InvalidTransition { from, to: target });
        }
        if target == PublicationState::Published && self.evidence.is_empty() {
            return Err(ContractError::MissingEvidence);
        }
        self.publication_state = target;
        self.truth_status = truth_for_state(target);
        Ok(())
    }

    /// Returns whether the candidate is immediately eligible for publication.
    pub fn is_publishable(&self) -> bool {
        self.publication_state == PublicationState::Reviewed && !self.evidence.is_empty()
    }
}

const ALLOWED_TRANSITIONS: &[(PublicationState, PublicationState)] = &[
    (PublicationState::Draft, PublicationState::Proposed),
    (PublicationState::Draft, PublicationState::Rejected),
    (PublicationState::Proposed, PublicationState::Validated),
    (PublicationState::Proposed, PublicationState::Rejected),
    (PublicationState::Validated, PublicationState::Reviewed),
    (PublicationState::Validated, PublicationState::Rejected),
    (PublicationState::Reviewed, PublicationState::Published),
    (PublicationState::Reviewed, PublicationState::Rejected),
    (PublicationState::Published, PublicationState::Superseded),
];

fn truth_for_state(state: PublicationState) -> TruthStatus {
    match state {
        PublicationState::Draft | PublicationState::Validated | PublicationState::Reviewed => {
            TruthStatus::Inferred
        }
        PublicationState::Proposed => TruthStatus::Proposed,
        PublicationState::Published => TruthStatus::Authoritative,
        PublicationState::Superseded => TruthStatus::Superseded,
        PublicationState::Rejected => TruthStatus::Rejected,
    }
}

/// A domain-contract validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    /// A required identity or evidence field was blank.
    EmptyField(&'static str),
    /// A candidate was created without any supporting evidence.
    MissingEvidence,
    /// A governance state transition attempted to skip or reverse required review.
    InvalidTransition {
        /// State before the rejected transition.
        from: PublicationState,
        /// Requested target state.
        to: PublicationState,
    },
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "required field `{field}` is blank"),
            Self::MissingEvidence => write!(formatter, "semantic candidates require source evidence"),
            Self::InvalidTransition { from, to } => write!(
                formatter,
                "publication transition from {from:?} to {to:?} is not permitted"
            ),
        }
    }
}

impl std::error::Error for ContractError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> EvidenceReference {
        EvidenceReference::new("source-1", "sha256:abc", "schema.orders.total").unwrap()
    }

    fn candidate() -> SemanticCandidate {
        SemanticCandidate::new("candidate-1", CandidateKind::Concept, vec![evidence()]).unwrap()
    }

    #[test]
    fn evidence_reference_accepts_valid_values() {
        let reference = evidence();
        assert_eq!(reference.source_id, "source-1");
        assert_eq!(reference.source_digest, "sha256:abc");
        assert_eq!(reference.location, "schema.orders.total");
    }

    #[test]
    fn evidence_reference_rejects_each_blank_field() {
        assert_eq!(
            EvidenceReference::new(" ", "digest", "location"),
            Err(ContractError::EmptyField("source_id"))
        );
        assert_eq!(
            EvidenceReference::new("source", " ", "location"),
            Err(ContractError::EmptyField("source_digest"))
        );
        assert_eq!(
            EvidenceReference::new("source", "digest", " "),
            Err(ContractError::EmptyField("location"))
        );
    }

    #[test]
    fn candidate_requires_identity_and_evidence() {
        assert_eq!(
            SemanticCandidate::new(" ", CandidateKind::Concept, vec![evidence()]),
            Err(ContractError::EmptyField("candidate_id"))
        );
        assert_eq!(
            SemanticCandidate::new("candidate", CandidateKind::Concept, vec![]),
            Err(ContractError::MissingEvidence)
        );
    }

    #[test]
    fn candidate_accessors_expose_read_only_domain_state() {
        let candidate = candidate();
        assert_eq!(candidate.candidate_id(), "candidate-1");
        assert_eq!(candidate.kind(), CandidateKind::Concept);
        assert_eq!(candidate.truth_status(), TruthStatus::Inferred);
        assert_eq!(candidate.publication_state(), PublicationState::Draft);
        assert_eq!(candidate.evidence().len(), 1);
    }

    #[test]
    fn reviewed_candidate_can_be_published() {
        let mut candidate = candidate();
        assert_eq!(candidate.truth_status(), TruthStatus::Inferred);
        assert!(!candidate.is_publishable());

        candidate.transition(PublicationState::Proposed).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Proposed);
        candidate.transition(PublicationState::Validated).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Inferred);
        candidate.transition(PublicationState::Reviewed).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Inferred);
        assert!(candidate.is_publishable());
        candidate.transition(PublicationState::Published).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Authoritative);
        assert!(!candidate.is_publishable());
    }

    #[test]
    fn lifecycle_rejects_skipped_and_post_rejection_transitions() {
        let mut candidate = candidate();
        assert_eq!(
            candidate.transition(PublicationState::Published),
            Err(ContractError::InvalidTransition {
                from: PublicationState::Draft,
                to: PublicationState::Published,
            })
        );
        candidate.transition(PublicationState::Rejected).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Rejected);
        assert_eq!(
            candidate.transition(PublicationState::Proposed),
            Err(ContractError::InvalidTransition {
                from: PublicationState::Rejected,
                to: PublicationState::Proposed,
            })
        );
    }

    #[test]
    fn published_candidate_can_only_be_superseded() {
        let mut candidate = candidate();
        for state in [
            PublicationState::Proposed,
            PublicationState::Validated,
            PublicationState::Reviewed,
            PublicationState::Published,
        ] {
            candidate.transition(state).unwrap();
        }
        candidate.transition(PublicationState::Superseded).unwrap();
        assert_eq!(candidate.truth_status(), TruthStatus::Superseded);
        assert_eq!(
            candidate.transition(PublicationState::Rejected),
            Err(ContractError::InvalidTransition {
                from: PublicationState::Superseded,
                to: PublicationState::Rejected,
            })
        );
    }

    #[test]
    fn publication_rechecks_evidence_at_authority_boundary() {
        let mut candidate = candidate();
        for state in [
            PublicationState::Proposed,
            PublicationState::Validated,
            PublicationState::Reviewed,
        ] {
            candidate.transition(state).unwrap();
        }

        candidate.evidence.clear();

        assert_eq!(
            candidate.transition(PublicationState::Published),
            Err(ContractError::MissingEvidence)
        );
        assert_eq!(candidate.publication_state(), PublicationState::Reviewed);
        assert_eq!(candidate.truth_status(), TruthStatus::Inferred);
    }

    #[test]
    fn contract_errors_explain_the_failure() {
        assert_eq!(
            ContractError::EmptyField("field").to_string(),
            "required field `field` is blank"
        );
        assert_eq!(
            ContractError::MissingEvidence.to_string(),
            "semantic candidates require source evidence"
        );
        assert_eq!(
            ContractError::InvalidTransition {
                from: PublicationState::Draft,
                to: PublicationState::Published,
            }
            .to_string(),
            "publication transition from Draft to Published is not permitted"
        );
    }
}
