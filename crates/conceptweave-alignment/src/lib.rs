#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Explicit, reviewable alignment of source-backed drafts to proposed semantic identities.
//!
//! Alignment choices are supplied by a caller; source names and comments never become business
//! meaning by default. Deterministic validation checks those choices without granting steward
//! authority or producing a client-visible release.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use conceptweave_discovery::RelationalProposal;
use conceptweave_domain::{ContractError, PublicationState, SemanticCandidate};

/// A caller's explicit choice for one exact source-backed candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlignmentDisposition {
    /// Offer a semantic identity and human-readable meaning for validation.
    Map {
        /// Proposed stable semantic identity.
        semantic_id: String,
        /// Human-readable semantic name, not copied automatically from the source.
        name: String,
        /// Explanation for the proposed mapping.
        rationale: String,
    },
    /// Deliberately leave this source candidate out of the proposed model.
    Exclude {
        /// Explanation for the exclusion.
        rationale: String,
    },
}

/// One decision bound to a candidate identity, never to a mutable source label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlignmentDecision {
    candidate_id: String,
    disposition: AlignmentDisposition,
}

impl AlignmentDecision {
    /// Proposes a semantic mapping with an explicit explanation.
    pub fn map(
        candidate_id: impl Into<String>,
        semantic_id: impl Into<String>,
        name: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, AlignmentError> {
        let candidate_id = candidate_id.into();
        let semantic_id = semantic_id.into();
        let name = name.into();
        let rationale = rationale.into();
        require_text(&candidate_id, "candidate_id")?;
        require_text(&semantic_id, "semantic_id")?;
        require_text(&name, "name")?;
        require_text(&rationale, "rationale")?;
        Ok(Self {
            candidate_id,
            disposition: AlignmentDisposition::Map {
                semantic_id,
                name,
                rationale,
            },
        })
    }

    /// Records a deliberate exclusion with an explicit explanation.
    pub fn exclude(
        candidate_id: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, AlignmentError> {
        let candidate_id = candidate_id.into();
        let rationale = rationale.into();
        require_text(&candidate_id, "candidate_id")?;
        require_text(&rationale, "rationale")?;
        Ok(Self {
            candidate_id,
            disposition: AlignmentDisposition::Exclude { rationale },
        })
    }

    /// Returns the exact discovery candidate being decided.
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Returns the explicit mapping or exclusion.
    pub const fn disposition(&self) -> &AlignmentDisposition {
        &self.disposition
    }
}

/// Source-backed candidate together with its explicit alignment decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlignedCandidate {
    candidate: SemanticCandidate,
    disposition: AlignmentDisposition,
}

impl AlignedCandidate {
    /// Returns the source-backed domain candidate and its current lifecycle state.
    pub const fn candidate(&self) -> &SemanticCandidate {
        &self.candidate
    }

    /// Returns the explanation and proposed semantic identity, if mapped.
    pub const fn disposition(&self) -> &AlignmentDisposition {
        &self.disposition
    }
}

/// Complete decisions for one exact proposal, awaiting deterministic validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlignedProposal {
    proposal_id: String,
    source_id: String,
    source_digest: String,
    candidates: Vec<AlignedCandidate>,
    relation_endpoints: BTreeMap<String, (String, String)>,
}

impl AlignedProposal {
    /// Returns the exact discovery proposal identity.
    pub fn proposal_id(&self) -> &str {
        &self.proposal_id
    }

    /// Returns decisions in stable candidate-ID order.
    pub fn candidates(&self) -> &[AlignedCandidate] {
        &self.candidates
    }
}

/// A complete alignment whose source binding and semantic topology passed deterministic checks.
///
/// Every mapped candidate is `Validated` but still `Inferred`; this value does not authorize
/// review, publication, or client consumption.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedAlignment {
    proposal_id: String,
    source_digest: String,
    candidates: Vec<AlignedCandidate>,
    validation: ValidationSummary,
}

impl ValidatedAlignment {
    /// Returns the exact discovery proposal identity used for validation.
    pub fn proposal_id(&self) -> &str {
        &self.proposal_id
    }

    /// Returns the owner-computed source snapshot digest.
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns mapped validated candidates and explicitly excluded candidates.
    pub fn candidates(&self) -> &[AlignedCandidate] {
        &self.candidates
    }

    /// Returns counts of the deterministic checks that passed for this exact alignment.
    pub const fn validation(&self) -> &ValidationSummary {
        &self.validation
    }
}

/// Reviewable record of the checks completed before candidates entered `Validated`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationSummary {
    source_bound_candidates: usize,
    unique_semantic_ids: usize,
    mapped_relations_with_endpoints: usize,
}

impl ValidationSummary {
    /// Number of candidates checked against one exact source identity and digest.
    pub const fn source_bound_candidates(&self) -> usize {
        self.source_bound_candidates
    }

    /// Number of distinct proposed semantic identities checked.
    pub const fn unique_semantic_ids(&self) -> usize {
        self.unique_semantic_ids
    }

    /// Number of mapped relations checked against both included endpoint concepts.
    pub const fn mapped_relations_with_endpoints(&self) -> usize {
        self.mapped_relations_with_endpoints
    }
}

/// A decision or validation failure that prevents semantic promotion.
#[derive(Debug, PartialEq, Eq)]
pub enum AlignmentError {
    /// A required decision field was blank or contained NUL.
    InvalidText(&'static str),
    /// Decisions refer to a different source revision.
    StaleProposal,
    /// The same candidate was decided more than once.
    DuplicateDecision,
    /// A decision names a candidate absent from this proposal.
    UnknownCandidate,
    /// A candidate has no explicit decision.
    MissingDecision,
    /// Two mapped candidates claim one semantic identity.
    DuplicateSemanticIdentity,
    /// A mapped relation refers to an excluded concept.
    ExcludedRelationEndpoint,
    /// Candidate evidence disagrees with the exact proposal source.
    InvalidSourceBinding,
    /// A domain lifecycle transition failed.
    Contract(ContractError),
}

impl fmt::Display for AlignmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidText(_) => "a required alignment value is missing",
            Self::StaleProposal => "the alignment belongs to another proposal revision",
            Self::DuplicateDecision => "a candidate has more than one alignment decision",
            Self::UnknownCandidate => "an alignment decision names an unknown candidate",
            Self::MissingDecision => "a candidate still needs an alignment decision",
            Self::DuplicateSemanticIdentity => "two candidates share a proposed semantic identity",
            Self::ExcludedRelationEndpoint => "a mapped relation needs both concepts included",
            Self::InvalidSourceBinding => "candidate source evidence does not match this proposal",
            Self::Contract(_) => "a candidate could not enter its validation state",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for AlignmentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Contract(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ContractError> for AlignmentError {
    fn from(error: ContractError) -> Self {
        Self::Contract(error)
    }
}

fn require_text(value: &str, field: &'static str) -> Result<(), AlignmentError> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(AlignmentError::InvalidText(field));
    }
    Ok(())
}

/// Binds one explicit decision to every candidate of the exact source proposal revision.
pub fn align_relational_proposal(
    proposal: &RelationalProposal,
    expected_proposal_id: &str,
    decisions: Vec<AlignmentDecision>,
) -> Result<AlignedProposal, AlignmentError> {
    if proposal.proposal_id() != expected_proposal_id {
        return Err(AlignmentError::StaleProposal);
    }
    let mut pending = BTreeMap::new();
    for decision in decisions {
        if pending
            .insert(decision.candidate_id.clone(), decision.disposition)
            .is_some()
        {
            return Err(AlignmentError::DuplicateDecision);
        }
    }

    let mut candidates = Vec::new();
    let mut relation_endpoints = BTreeMap::new();
    let source_candidates = proposal
        .concepts()
        .iter()
        .map(|concept| concept.candidate())
        .chain(
            proposal
                .relations()
                .iter()
                .map(|relation| relation.candidate()),
        )
        .chain(
            proposal
                .source_types()
                .iter()
                .map(|source_type| source_type.candidate()),
        );
    for source_candidate in source_candidates {
        let mut candidate = source_candidate.clone();
        let disposition = pending
            .remove(candidate.candidate_id())
            .ok_or(AlignmentError::MissingDecision)?;
        candidate.transition(match disposition {
            AlignmentDisposition::Map { .. } => PublicationState::Proposed,
            AlignmentDisposition::Exclude { .. } => PublicationState::Rejected,
        })?;
        candidates.push(AlignedCandidate {
            candidate,
            disposition,
        });
    }
    if !pending.is_empty() {
        return Err(AlignmentError::UnknownCandidate);
    }
    for relation in proposal.relations() {
        relation_endpoints.insert(
            relation.candidate().candidate_id().to_owned(),
            (
                relation.from_concept_id().to_owned(),
                relation.to_concept_id().to_owned(),
            ),
        );
    }
    candidates.sort_by(|left, right| {
        left.candidate
            .candidate_id()
            .cmp(right.candidate.candidate_id())
    });
    Ok(AlignedProposal {
        proposal_id: proposal.proposal_id().to_owned(),
        source_id: proposal.source_id().to_owned(),
        source_digest: proposal.source_digest().to_owned(),
        candidates,
        relation_endpoints,
    })
}

/// Checks source binding, semantic identity uniqueness, and mapped relation endpoints.
///
/// Validation is deterministic over the immutable alignment. It checks structure and declared
/// provenance, not whether the caller's business meaning is true or steward-approved.
pub fn validate_alignment(aligned: &AlignedProposal) -> Result<ValidatedAlignment, AlignmentError> {
    let mut semantic_ids = BTreeSet::new();
    let mut mapped_candidates = BTreeSet::new();
    for aligned_candidate in &aligned.candidates {
        if aligned_candidate.candidate.evidence().is_empty()
            || aligned_candidate
                .candidate
                .evidence()
                .iter()
                .any(|evidence| {
                    evidence.source_id() != aligned.source_id
                        || evidence.source_digest() != aligned.source_digest
                })
        {
            return Err(AlignmentError::InvalidSourceBinding);
        }
        if let AlignmentDisposition::Map { semantic_id, .. } = &aligned_candidate.disposition {
            if !semantic_ids.insert(semantic_id) {
                return Err(AlignmentError::DuplicateSemanticIdentity);
            }
            mapped_candidates.insert(aligned_candidate.candidate.candidate_id());
        }
    }
    let mut mapped_relations_with_endpoints = 0;
    for (relation_id, (from, to)) in &aligned.relation_endpoints {
        if mapped_candidates.contains(relation_id.as_str())
            && (!mapped_candidates.contains(from.as_str())
                || !mapped_candidates.contains(to.as_str()))
        {
            return Err(AlignmentError::ExcludedRelationEndpoint);
        }
        if mapped_candidates.contains(relation_id.as_str()) {
            mapped_relations_with_endpoints += 1;
        }
    }
    let mut candidates = aligned.candidates.clone();
    for aligned_candidate in &mut candidates {
        if matches!(
            aligned_candidate.disposition,
            AlignmentDisposition::Map { .. }
        ) {
            aligned_candidate
                .candidate
                .transition(PublicationState::Validated)?;
        }
    }
    Ok(ValidatedAlignment {
        proposal_id: aligned.proposal_id.clone(),
        source_digest: aligned.source_digest.clone(),
        candidates,
        validation: ValidationSummary {
            source_bound_candidates: aligned.candidates.len(),
            unique_semantic_ids: semantic_ids.len(),
            mapped_relations_with_endpoints,
        },
    })
}
