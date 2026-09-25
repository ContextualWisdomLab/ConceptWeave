#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Steward-gated review and deterministic immutable publication of validated proposals.
//!
//! The injected authority adapter must authenticate a steward and authorize the exact review
//! request. This crate neither stores credentials nor supplies a default authorizer. A published
//! manifest pin must still reach clients through protected distribution independent of the
//! release being consumed.

use std::fmt;

use conceptweave_alignment::{AlignedCandidate, AlignmentDisposition, ValidatedAlignment};
use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, TrustedReleaseManifest,
};
use conceptweave_domain::{CandidateKind, PublicationState, TruthStatus};
use sha2::{Digest, Sha256};

const MAX_ARTIFACT_BYTES: usize = 16 * 1024 * 1024;

/// Exact review request presented to a trusted steward-authorization adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewRequest {
    proposal_id: String,
    source_digest: String,
    alignment_digest: String,
    steward_id: String,
    rationale: String,
}

impl ReviewRequest {
    /// Returns the exact source-backed proposal revision.
    pub fn proposal_id(&self) -> &str {
        &self.proposal_id
    }

    /// Returns the owner-computed source-content digest.
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the digest of every candidate decision and validation count.
    pub fn alignment_digest(&self) -> &str {
        &self.alignment_digest
    }

    /// Returns the steward principal whose authority must be verified externally.
    pub fn steward_id(&self) -> &str {
        &self.steward_id
    }

    /// Returns the steward's stated reason for approving this exact alignment.
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// External policy boundary that verifies a steward's authority over one exact review request.
///
/// `Some(audit_receipt_id)` means the adapter authenticated and authorized the review and wrote
/// an immutable audit decision. `None` denies it. Implementations must not infer authority from
/// the steward ID or proposal shape alone.
pub trait StewardReviewAuthority {
    /// Authorizes an exact proposal revision, alignment digest, and steward decision.
    fn authorize_review(&self, request: &ReviewRequest) -> Result<Option<String>, GovernanceError>;
}

/// A validated alignment with one externally authorized steward decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewedAlignment {
    alignment: ValidatedAlignment,
    request: ReviewRequest,
    audit_receipt_id: String,
    alignment_bytes: Vec<u8>,
}

impl ReviewedAlignment {
    /// Returns the exact review request that was authorized.
    pub const fn request(&self) -> &ReviewRequest {
        &self.request
    }

    /// Returns the immutable audit receipt identity supplied by the authority adapter.
    pub fn audit_receipt_id(&self) -> &str {
        &self.audit_receipt_id
    }

    /// Returns the validated candidates and their explicit alignment decisions.
    pub const fn alignment(&self) -> &ValidatedAlignment {
        &self.alignment
    }
}

/// Immutable artifact bytes, client release contract, and pin to distribute independently.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedModel {
    release: SemanticRelease,
    artifact_bytes: Vec<u8>,
    manifest_pin: TrustedReleaseManifest,
    review: ReviewedAlignment,
}

impl PublishedModel {
    /// Returns the client-visible semantic release metadata.
    pub const fn release(&self) -> &SemanticRelease {
        &self.release
    }

    /// Returns exact immutable bytes bound to the release artifact digest.
    pub fn artifact_bytes(&self) -> &[u8] {
        &self.artifact_bytes
    }

    /// Returns the exact manifest pin for protected, independent distribution.
    pub const fn manifest_pin(&self) -> &TrustedReleaseManifest {
        &self.manifest_pin
    }

    /// Returns the externally authorized review bound into the artifact.
    pub const fn review(&self) -> &ReviewedAlignment {
        &self.review
    }
}

/// A review or publication request could not cross its trust boundary.
#[derive(Debug, Eq, PartialEq)]
pub enum GovernanceError {
    /// A required human decision or audit field is blank or contains NUL.
    InvalidText,
    /// Review text exceeds the bounded publication envelope.
    ArtifactTooLarge,
    /// The proposal has no included semantic concept.
    NoSemanticConcept,
    /// A candidate does not have the validated or rejected state its decision requires.
    InvalidCandidateState,
    /// The steward authority adapter denied this exact review.
    ReviewDenied,
    /// The steward authority adapter was unavailable or failed.
    AuthorityUnavailable,
    /// The authority adapter returned an invalid audit receipt identity.
    InvalidAuditReceipt,
    /// Client release metadata construction failed.
    Release(ReleaseContractError),
}

impl fmt::Display for GovernanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidText => "review information is incomplete",
            Self::ArtifactTooLarge => "the reviewed model exceeds the release size limit",
            Self::NoSemanticConcept => "the reviewed model needs an included concept",
            Self::InvalidCandidateState => "a candidate has not passed validation",
            Self::ReviewDenied => "this review was not authorized",
            Self::AuthorityUnavailable => "review authorization is unavailable",
            Self::InvalidAuditReceipt => "review authorization returned no valid receipt",
            Self::Release(_) => "the semantic release could not be created",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for GovernanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Release(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ReleaseContractError> for GovernanceError {
    fn from(error: ReleaseContractError) -> Self {
        Self::Release(error)
    }
}

fn valid_text(value: &str) -> Result<(), GovernanceError> {
    if value.trim().is_empty() || value.contains('\0') {
        return Err(GovernanceError::InvalidText);
    }
    if value.len() > 4_096 {
        return Err(GovernanceError::ArtifactTooLarge);
    }
    Ok(())
}

/// Requests steward approval over an exact validated alignment before publication.
pub fn review(
    alignment: &ValidatedAlignment,
    steward_id: &str,
    rationale: &str,
    authority: &impl StewardReviewAuthority,
) -> Result<ReviewedAlignment, GovernanceError> {
    valid_text(steward_id)?;
    valid_text(rationale)?;
    let mut has_concept = false;
    for aligned in alignment.candidates() {
        match aligned.disposition() {
            AlignmentDisposition::Map { .. } => {
                if aligned.candidate().publication_state() != PublicationState::Validated {
                    return Err(GovernanceError::InvalidCandidateState);
                }
                has_concept |= aligned.candidate().kind() == CandidateKind::Concept;
            }
            AlignmentDisposition::Exclude { .. } => {
                if aligned.candidate().publication_state() != PublicationState::Rejected {
                    return Err(GovernanceError::InvalidCandidateState);
                }
            }
        }
    }
    if !has_concept {
        return Err(GovernanceError::NoSemanticConcept);
    }
    let alignment_bytes = encode_alignment(alignment)?;
    let request = ReviewRequest {
        proposal_id: alignment.proposal_id().to_owned(),
        source_digest: alignment.source_digest().to_owned(),
        alignment_digest: sha256(&alignment_bytes),
        steward_id: steward_id.to_owned(),
        rationale: rationale.to_owned(),
    };
    let audit_receipt_id = authority
        .authorize_review(&request)?
        .ok_or(GovernanceError::ReviewDenied)?;
    valid_text(&audit_receipt_id).map_err(|_| GovernanceError::InvalidAuditReceipt)?;
    Ok(ReviewedAlignment {
        alignment: alignment.clone(),
        request,
        audit_receipt_id,
        alignment_bytes,
    })
}

/// Publishes canonical bytes and release metadata from an authorized review.
///
/// This operation produces a pin for protected distribution. It does not install that pin in a
/// client or implement the external steward-authorization adapter.
pub fn publish(
    reviewed: &ReviewedAlignment,
    metadata: ReleaseMetadata,
) -> Result<PublishedModel, GovernanceError> {
    for value in [
        metadata.release_id(),
        metadata.contract_version(),
        metadata.ontology_version(),
    ] {
        valid_text(value)?;
    }
    let mut artifact = Vec::new();
    put_text(&mut artifact, "conceptweave.governed_semantic_artifact.v1")?;
    put_text(&mut artifact, metadata.release_id())?;
    put_text(&mut artifact, metadata.contract_version())?;
    put_text(&mut artifact, metadata.ontology_version())?;
    put_blob(&mut artifact, &reviewed.alignment_bytes)?;
    for value in [
        reviewed.request.steward_id(),
        reviewed.request.rationale(),
        reviewed.audit_receipt_id(),
        reviewed.request.alignment_digest(),
    ] {
        put_text(&mut artifact, value)?;
    }
    let mut concept_ids = Vec::new();
    let mut provenance = Vec::new();
    for aligned in reviewed.alignment.candidates() {
        if let AlignmentDisposition::Map { semantic_id, .. } = aligned.disposition() {
            if aligned.candidate().kind() == CandidateKind::Concept {
                concept_ids.push(semantic_id.clone());
            }
            provenance.extend(aligned.candidate().evidence().iter().cloned());
        }
    }
    concept_ids.sort();
    provenance.sort_by(|left, right| {
        (left.source_id(), left.source_digest(), left.location()).cmp(&(
            right.source_id(),
            right.source_digest(),
            right.location(),
        ))
    });
    provenance.dedup();
    let release = SemanticRelease::new(
        metadata,
        TruthStatus::Authoritative,
        PublicationState::Published,
        ReleaseDigest::new(&sha256(&artifact))?,
        provenance,
        concept_ids,
    )?;
    let manifest_pin =
        TrustedReleaseManifest::new(release.release_id(), release.manifest_digest())?;
    Ok(PublishedModel {
        release,
        artifact_bytes: artifact,
        manifest_pin,
        review: reviewed.clone(),
    })
}

fn encode_alignment(alignment: &ValidatedAlignment) -> Result<Vec<u8>, GovernanceError> {
    let mut bytes = Vec::new();
    put_text(&mut bytes, "conceptweave.validated_alignment.v1")?;
    put_text(&mut bytes, alignment.proposal_id())?;
    put_text(&mut bytes, alignment.source_digest())?;
    let mut candidates = alignment.candidates().iter().collect::<Vec<_>>();
    candidates.sort_by_key(|aligned| aligned.candidate().candidate_id());
    put_len(&mut bytes, candidates.len())?;
    for aligned in candidates {
        encode_candidate(&mut bytes, aligned)?;
    }
    let validation = alignment.validation();
    for count in [
        validation.source_bound_candidates(),
        validation.unique_semantic_ids(),
        validation.mapped_fields_with_concepts(),
        validation.mapped_relations_with_endpoints(),
    ] {
        put_len(&mut bytes, count)?;
    }
    put_len(&mut bytes, alignment.field_parents().len())?;
    for (field_id, concept_id) in alignment.field_parents() {
        put_text(&mut bytes, field_id)?;
        put_text(&mut bytes, concept_id)?;
    }
    put_len(&mut bytes, alignment.relation_endpoints().len())?;
    for (relation_id, (from_id, to_id)) in alignment.relation_endpoints() {
        put_text(&mut bytes, relation_id)?;
        put_text(&mut bytes, from_id)?;
        put_text(&mut bytes, to_id)?;
    }
    Ok(bytes)
}

fn encode_candidate(
    bytes: &mut Vec<u8>,
    aligned: &AlignedCandidate,
) -> Result<(), GovernanceError> {
    let candidate = aligned.candidate();
    put_text(bytes, candidate.candidate_id())?;
    put_raw(bytes, &[kind_tag(candidate.kind())])?;
    match aligned.disposition() {
        AlignmentDisposition::Map {
            semantic_id,
            name,
            rationale,
        } => {
            put_raw(bytes, &[1])?;
            for value in [semantic_id, name, rationale] {
                put_text(bytes, value)?;
            }
        }
        AlignmentDisposition::Exclude { rationale } => {
            put_raw(bytes, &[0])?;
            put_text(bytes, rationale)?;
        }
    }
    let mut evidence = candidate.evidence().iter().collect::<Vec<_>>();
    evidence.sort_by_key(|item| (item.source_id(), item.source_digest(), item.location()));
    put_len(bytes, evidence.len())?;
    for item in evidence {
        for value in [item.source_id(), item.source_digest(), item.location()] {
            put_text(bytes, value)?;
        }
    }
    Ok(())
}

const fn kind_tag(kind: CandidateKind) -> u8 {
    match kind {
        CandidateKind::Concept => 0,
        CandidateKind::TaxonomyRelation => 1,
        CandidateKind::SemanticRelation => 2,
        CandidateKind::Constraint => 3,
        CandidateKind::Dimension => 4,
        CandidateKind::Measure => 5,
        CandidateKind::PhysicalMapping => 6,
    }
}

fn put_len(bytes: &mut Vec<u8>, len: usize) -> Result<(), GovernanceError> {
    let len = u64::try_from(len).map_err(|_| GovernanceError::ArtifactTooLarge)?;
    put_raw(bytes, &len.to_be_bytes())
}

fn put_text(bytes: &mut Vec<u8>, value: &str) -> Result<(), GovernanceError> {
    put_blob(bytes, value.as_bytes())
}

fn put_blob(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), GovernanceError> {
    put_len(bytes, value.len())?;
    put_raw(bytes, value)
}

fn put_raw(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), GovernanceError> {
    // ponytail: a fixed envelope bounds this in-memory artifact; grow it after measured demand.
    if bytes
        .len()
        .checked_add(value.len())
        .is_none_or(|len| len > MAX_ARTIFACT_BYTES)
    {
        return Err(GovernanceError::ArtifactTooLarge);
    }
    bytes.extend_from_slice(value);
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
