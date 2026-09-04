#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! Deterministic, read-only classification of a Zotero library snapshot.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Duration;

/// Classification rule revision recorded in every report.
pub const RULE_REVISION: &str = "ontology-research-v2";

const SUPPORTED_API_VERSION: u64 = 3;
const SUPPORTED_API_VERSION_HEADER: &str = "3";
const PAGE_LIMIT: usize = 100;
const MAX_PAGE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_SNAPSHOT_ITEMS: usize = 50_000;
const MAX_SNAPSHOT_BYTES: u64 = 256 * 1024 * 1024;
const LOCAL_API: &str = "http://127.0.0.1:23119/api/users/0/items";

#[cfg(test)]
static TEST_LOCAL_API: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// A Zotero item returned by the Local API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ZoteroItem {
    /// Stable item key.
    pub key: String,
    /// Item revision.
    pub version: u64,
    /// Item metadata.
    pub data: ItemData,
}

/// Metadata used by the classifier.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    /// Zotero item type.
    pub item_type: String,
    /// Display title when present.
    #[serde(default)]
    pub title: String,
    /// Abstract when present.
    #[serde(default)]
    pub abstract_note: String,
    /// DOI when present.
    #[serde(default, rename = "DOI")]
    pub doi: String,
    /// Parent item key for notes and attachments.
    #[serde(default)]
    pub parent_item: String,
    /// Collection keys.
    #[serde(default)]
    pub collections: Vec<String>,
    /// Tags applied to the item.
    #[serde(default)]
    pub tags: Vec<ItemTag>,
}

/// A Zotero item tag.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ItemTag {
    /// Tag text.
    pub tag: String,
    /// Zotero automatic-tag marker, preserved for exact write rollback.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub tag_type: Option<u64>,
}

/// One mutually exclusive proposed disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    /// Evidence about ontology or taxonomy generation.
    Generation,
    /// Evidence about alignment, matching, evolution, or versioning.
    AlignmentVersioning,
    /// Evidence about semantic consumption or query bridges.
    SemanticConsumptionBridge,
    /// Evidence about evaluation, validation, or governance.
    EvaluationGovernance,
    /// Ontology-adjacent evidence without a narrower match.
    AdjacentEvidence,
    /// Explicitly reviewed as outside the program scope.
    OutOfScope,
    /// No deterministic rule supplies enough evidence for a narrower proposal.
    NeedsStewardReview,
}

/// Deterministic reason that a bibliographic item requires steward review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AbstentionReason {
    /// Title, abstract, and tags contain no classification metadata.
    MissingClassificationMetadata,
    /// Metadata uses vocabulary outside the current deterministic rule set.
    UnsupportedRuleVocabulary,
    /// Metadata is present but no deterministic rule phrase matches it.
    NoDeterministicRuleMatch,
    /// More than one specific disposition family is supported by the item.
    ConflictingDispositionEvidence,
}

/// Evidence for a deterministic proposed disposition.
#[derive(Debug, Serialize)]
pub struct ClassificationEvidence {
    /// Metadata fields whose values matched.
    pub fields: Vec<&'static str>,
    /// Exact snapshot values for matched fields, retained only in the local report.
    pub field_values: BTreeMap<&'static str, String>,
    /// Rule phrases found in those fields.
    pub matched_phrases: Vec<&'static str>,
}

/// A single top-level bibliographic classification proposal.
#[derive(Debug, Serialize)]
pub struct ClassifiedItem {
    /// Stable Zotero item key.
    pub item_key: String,
    /// Item revision observed in this snapshot.
    pub item_version: u64,
    /// Zotero item type.
    pub item_type: String,
    /// Human-readable title retained in the local report only.
    pub title: String,
    /// Collection keys observed with the item.
    pub collection_keys: Vec<String>,
    /// Tag text observed with the item.
    pub tags: Vec<ItemTag>,
    /// Proposed disposition; never an authoritative governance decision.
    pub proposed_disposition: Disposition,
    /// Deterministic reason for abstention, absent when a rule proposes a disposition.
    pub abstention_reason: Option<AbstentionReason>,
    /// Deterministic supporting evidence.
    pub evidence: ClassificationEvidence,
    /// Child note and attachment keys linked to the top-level item.
    pub child_item_keys: Vec<String>,
    /// Model receipt is absent because this slice performs no model call.
    pub model_receipt: Option<String>,
}

/// A duplicate candidate group; no item is merged or deleted.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DuplicateCandidate {
    /// Identity kind used for the candidate group.
    pub identity_kind: String,
    /// Normalized identity value.
    pub normalized_identity: String,
    /// Zotero item keys sharing the identity.
    pub item_keys: Vec<String>,
}

/// One steward decision selecting the canonical identity for a duplicate cluster.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DuplicateMergeDecision {
    /// Candidate identity kind from the classification report.
    pub identity_kind: String,
    /// Candidate normalized identity from the classification report.
    pub normalized_identity: String,
    /// Existing Zotero item retained as the canonical reference.
    pub retained_item_key: String,
}

/// Snapshot-bound duplicate decisions verified by the caller's governance boundary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewedDuplicateMergeSet {
    /// Opaque steward review receipt.
    pub review_id: String,
    /// Opaque governance authority receipt; contains no person identity.
    pub authority_receipt: String,
    /// Exact Zotero library revision reviewed by the steward.
    pub library_version: u64,
    /// Exact classifier rule revision reviewed by the steward.
    pub rule_revision: String,
    /// Exact raw-snapshot digest reviewed by the steward.
    pub snapshot_digest: String,
    /// Exact item-key/item-version coordinates reviewed by the steward.
    pub snapshot_items: Vec<SnapshotItemRevision>,
    /// Exact duplicate membership reviewed by the steward.
    pub duplicate_candidates: Vec<DuplicateCandidate>,
    /// Exactly one decision for every duplicate candidate.
    pub decisions: Vec<DuplicateMergeDecision>,
}

/// One reversible canonical-key mapping; Zotero source records remain unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DuplicateMergeOperation {
    /// Candidate identity kind.
    pub identity_kind: String,
    /// Candidate normalized identity retained only in the local manifest.
    pub normalized_identity: String,
    /// Steward-selected canonical Zotero item key.
    pub retained_item_key: String,
    /// Exact source revisions participating in the decision.
    pub source_items: Vec<SnapshotItemRevision>,
    /// Mapping before the reviewed canonicalization; every item maps to itself.
    pub before_canonical_keys: BTreeMap<String, String>,
    /// Reviewed mapping after canonicalization; every item maps to the retained key.
    pub after_canonical_keys: BTreeMap<String, String>,
    /// Exact inverse plan restoring the pre-review mapping.
    pub rollback_canonical_keys: BTreeMap<String, String>,
}

/// Aggregate of reviewed, reversible duplicate identity operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DuplicateMergeReviewManifest {
    /// Opaque steward review receipt.
    pub review_id: String,
    /// Opaque governance authority receipt.
    pub authority_receipt: String,
    /// Exact Zotero library revision reviewed by the steward.
    pub library_version: u64,
    /// Exact classifier rule revision reviewed by the steward.
    pub rule_revision: String,
    /// Exact raw-snapshot digest shared with the reviewed decisions.
    pub snapshot_digest: String,
    /// Deterministically ordered canonical-key operations.
    pub operations: Vec<DuplicateMergeOperation>,
    /// Classification never deletes or mutates Zotero source records.
    pub source_records_preserved: bool,
}

/// A fail-closed duplicate review contract violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateReviewError {
    /// Required review metadata or a decision is missing.
    InvalidReview,
    /// The review belongs to another raw snapshot.
    SnapshotMismatch,
    /// A decision does not identify a report candidate.
    UnknownCandidate,
    /// More than one decision targets the same candidate.
    DuplicateDecision,
    /// The retained key is not a member of the candidate cluster.
    InvalidRetainedItem,
    /// The caller's governance boundary rejected the complete review set.
    UnverifiedApproval,
}

impl fmt::Display for DuplicateReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidReview => "duplicate review metadata or decisions are invalid",
            Self::SnapshotMismatch => "duplicate review does not match the report snapshot",
            Self::UnknownCandidate => "duplicate review contains an unknown candidate",
            Self::DuplicateDecision => "duplicate review repeats a candidate decision",
            Self::InvalidRetainedItem => "retained item is absent from its duplicate candidate",
            Self::UnverifiedApproval => "duplicate review approval is unverified",
        })
    }
}

impl std::error::Error for DuplicateReviewError {}

/// Requested behavior for a reviewed classification change set.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteMode {
    /// Validate and emit a plan without contacting Zotero.
    #[default]
    DryRun,
    /// Permit a future authenticated Zotero 10+ adapter to apply the plan.
    Execute,
}

/// One steward-reviewed complete collection and tag replacement.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewedClassificationChange {
    /// Stable top-level bibliographic item key.
    pub item_key: String,
    /// Exact item revision reviewed by the steward.
    pub item_version: u64,
    /// Approved classification; abstention cannot be written.
    pub reviewed_disposition: Disposition,
    /// Complete collection state observed before the change.
    pub before_collection_keys: Vec<String>,
    /// Complete collection state requested after the change.
    pub after_collection_keys: Vec<String>,
    /// Complete tag state observed before the change.
    pub before_tags: Vec<ItemTag>,
    /// Complete tag state requested after the change.
    pub after_tags: Vec<ItemTag>,
}

/// Governance-bound reviewed changes retained outside the repository.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewedClassificationWriteSet {
    /// Opaque review receipt identifier.
    pub review_id: String,
    /// Opaque governance authority receipt.
    pub authority_receipt: String,
    /// Exact Local API server identity, when supplied by Zotero.
    pub server_id: Option<String>,
    /// Exact reviewed library revision.
    pub library_version: u64,
    /// Exact reviewed classifier revision.
    pub rule_revision: String,
    /// Exact reviewed raw-snapshot digest.
    pub snapshot_digest: String,
    /// Reviewed item-level changes.
    pub changes: Vec<ReviewedClassificationChange>,
}

/// One deterministic item operation with an exact rollback state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationWriteOperation {
    /// Stable top-level bibliographic item key.
    pub item_key: String,
    /// Optimistic item-version precondition.
    pub item_version: u64,
    /// Approved classification.
    pub reviewed_disposition: Disposition,
    /// Complete collection state before the change.
    pub before_collection_keys: Vec<String>,
    /// Complete collection state after the change.
    pub after_collection_keys: Vec<String>,
    /// Complete collection rollback state.
    pub rollback_collection_keys: Vec<String>,
    /// Complete tag state before the change.
    pub before_tags: Vec<ItemTag>,
    /// Complete tag state after the change.
    pub after_tags: Vec<ItemTag>,
    /// Complete tag rollback state, including Zotero tag type.
    pub rollback_tags: Vec<ItemTag>,
}

/// Local-only, snapshot-bound plan for reviewed Zotero classification writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationWritePlan {
    /// Requested write behavior; dry-run is the default.
    pub mode: WriteMode,
    /// Opaque review receipt identifier.
    pub review_id: String,
    /// Opaque governance authority receipt.
    pub authority_receipt: String,
    /// Exact Local API server identity.
    pub server_id: Option<String>,
    /// Exact library-version precondition.
    pub library_version: u64,
    /// Exact classifier revision.
    pub rule_revision: String,
    /// Exact raw-snapshot digest.
    pub snapshot_digest: String,
    /// Deterministically ordered item operations.
    pub operations: Vec<ClassificationWriteOperation>,
    /// Classification writes never delete source records or attachments.
    pub source_records_preserved: bool,
}

/// A fail-closed reviewed write-plan contract violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritePlanError {
    /// Required review metadata or item changes are absent.
    InvalidReview,
    /// The reviewed set belongs to another snapshot.
    SnapshotMismatch,
    /// The caller's governance boundary rejected the complete reviewed set.
    UnverifiedApproval,
    /// A change does not identify a classified top-level item.
    UnknownItem,
    /// More than one change targets the same item.
    DuplicateItem,
    /// An item revision or complete before-state is stale.
    StaleItem,
    /// A collection or tag value is blank or duplicated.
    InvalidMetadata,
    /// A reviewed change makes no collection or tag change.
    NoChange,
    /// An abstention cannot become a write instruction.
    UnreviewedDisposition,
    /// Execute mode is unsupported by this Zotero major version.
    UnsupportedExecute,
    /// Execute mode lacks a nonblank Local API server identity.
    MissingServerIdentity,
}

impl fmt::Display for WritePlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidReview => "classification write review is invalid",
            Self::SnapshotMismatch => "classification write review does not match the snapshot",
            Self::UnverifiedApproval => "classification write approval is unverified",
            Self::UnknownItem => "classification write targets an unknown item",
            Self::DuplicateItem => "classification write repeats an item",
            Self::StaleItem => "classification write item preconditions are stale",
            Self::InvalidMetadata => "classification write metadata is blank or duplicated",
            Self::NoChange => "classification write contains no metadata change",
            Self::UnreviewedDisposition => "steward-review abstention cannot be written",
            Self::UnsupportedExecute => "this Zotero version does not support local execution",
            Self::MissingServerIdentity => "local execution requires a Zotero server identity",
        })
    }
}

impl std::error::Error for WritePlanError {}

/// Complete local classification report for one immutable library version.
#[derive(Debug, Serialize)]
pub struct ClassificationReport {
    /// Zotero desktop version that served the snapshot.
    pub zotero_version: String,
    /// Requested and observed Local API version for a live read.
    pub api_version: Option<u64>,
    /// Zotero schema revision observed consistently across a live read.
    pub schema_version: Option<u64>,
    /// Local API server identifier observed on every page when supplied.
    pub server_id: Option<String>,
    /// Library version shared by every fetched page.
    pub library_version: u64,
    /// Rule revision used for all proposals.
    pub rule_revision: &'static str,
    /// Number of items read, including child notes and attachments.
    pub observed_item_count: usize,
    /// Complete item-revision identity of every observed record.
    pub snapshot_items: Vec<SnapshotItemRevision>,
    /// Canonical SHA-256 digest of every observed raw Zotero item.
    pub snapshot_digest: String,
    /// One proposal for every top-level bibliographic item.
    pub classified_items: Vec<ClassifiedItem>,
    /// Reversible DOI/title duplicate candidates.
    pub duplicate_candidates: Vec<DuplicateCandidate>,
    /// Aggregate completeness evidence for this successful snapshot.
    pub audit_summary: ClassificationAudit,
}

/// Aggregate-only evidence that a successful report covers its input and proposals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationAudit {
    /// Records captured from the immutable snapshot.
    pub snapshot_item_count: usize,
    /// Top-level bibliographic records eligible for classification.
    pub bibliographic_item_count: usize,
    /// Eligible records with exactly one proposed disposition.
    pub proposed_disposition_count: usize,
    /// Proposals retaining required item and classifier provenance.
    pub provenance_complete_count: usize,
    /// Proposals routed to steward review.
    pub abstention_count: usize,
    /// Reversible duplicate identity groups.
    pub duplicate_candidate_count: usize,
    /// Reader or classifier failures; successful reports always record zero.
    pub failure_count: usize,
    /// Proposal totals by disposition.
    pub disposition_counts: BTreeMap<Disposition, usize>,
}

/// One steward-reviewed expected disposition in a local golden set.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GoldenLabel {
    /// Zotero item key used only to join the local report and local review set.
    pub item_key: String,
    /// Steward-approved disposition used as evaluation truth.
    pub expected_disposition: Disposition,
}

impl GoldenLabel {
    /// Creates a local golden label.
    pub fn new(item_key: impl Into<String>, expected_disposition: Disposition) -> Self {
        Self {
            item_key: item_key.into(),
            expected_disposition,
        }
    }
}

/// Version-bound steward labels that remain outside the repository.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewedGoldenSet {
    /// Approval receipt verified by the caller's governance boundary.
    pub approval: GoldenSetApproval,
    /// Item-level expected dispositions.
    pub labels: Vec<GoldenLabel>,
}

/// One item revision in the exact reviewed classification snapshot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SnapshotItemRevision {
    /// Stable Zotero item key.
    pub item_key: String,
    /// Item revision observed during review.
    pub item_version: u64,
}

/// Governance receipt binding a steward approval to one exact classifier input.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GoldenSetApproval {
    /// Opaque receipt identifier.
    pub receipt_id: String,
    /// Stable reviewer subject understood by the governance verifier.
    pub reviewer_subject: String,
    /// Zotero library version reviewed by the steward.
    pub library_version: u64,
    /// Classifier rule revision whose proposals were reviewed.
    pub rule_revision: String,
    /// Immutable digest over the approved snapshot, verified by the caller.
    pub snapshot_digest: String,
    /// Complete sorted item-revision identity of the reviewed report.
    pub snapshot_items: Vec<SnapshotItemRevision>,
}

/// Computes the canonical content identity verified by a golden-set approval.
pub fn classification_snapshot_digest(report: &ClassificationReport) -> String {
    report.snapshot_digest.clone()
}

/// Integer evidence from which precision and recall can be calculated exactly.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct DispositionEvaluation {
    /// Correct predictions for this disposition.
    pub true_positive: usize,
    /// All classifier predictions for reviewed items in this disposition.
    pub predicted: usize,
    /// All steward labels expecting this disposition.
    pub expected: usize,
}

/// Aggregate-only evaluation result; item keys and bibliographic text are omitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GoldenSetEvaluation {
    /// Opaque review receipt identifier.
    pub review_id: String,
    /// Zotero library revision bound to the verified receipt.
    pub library_version: u64,
    /// Classifier revision bound to the verified receipt.
    pub rule_revision: String,
    /// Opaque immutable snapshot digest from the verified receipt.
    pub snapshot_digest: String,
    /// Number of steward-reviewed items.
    pub reviewed_count: usize,
    /// Number of exact disposition matches.
    pub correct_count: usize,
    /// Number of reviewed items on which the classifier abstained.
    pub abstention_count: usize,
    /// Precision/recall numerators and denominators per observed disposition.
    pub by_disposition: BTreeMap<Disposition, DispositionEvaluation>,
}

/// A fail-closed golden-set contract violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationError {
    /// Review receipt, labels, or revisions are missing or incompatible.
    InvalidReview,
    /// The golden set was reviewed against another library or rule revision.
    SnapshotMismatch,
    /// The caller's governance boundary did not verify the approval receipt.
    UnverifiedApproval,
    /// Abstention cannot be used as steward-approved semantic truth.
    InvalidExpectedDisposition,
    /// A reviewed key is absent from the classification report.
    UnknownItem,
    /// A reviewed key occurs more than once.
    DuplicateItem,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidReview => "golden-set review metadata or labels are invalid",
            Self::SnapshotMismatch => "golden set does not match the report snapshot",
            Self::UnverifiedApproval => "golden-set approval receipt is unverified",
            Self::InvalidExpectedDisposition => {
                "steward truth cannot use the classifier abstention disposition"
            }
            Self::UnknownItem => "golden set contains an item absent from the report",
            Self::DuplicateItem => "golden set contains a duplicate item",
        })
    }
}

impl std::error::Error for EvaluationError {}

/// Evaluates reviewed labels without copying item identities into the result.
pub fn evaluate_reviewed_golden_set<F>(
    report: &ClassificationReport,
    golden: &ReviewedGoldenSet,
    verify_approval: F,
) -> Result<GoldenSetEvaluation, EvaluationError>
where
    F: FnOnce(&ReviewedGoldenSet) -> bool,
{
    if golden.approval.receipt_id.trim().is_empty()
        || golden.approval.reviewer_subject.trim().is_empty()
        || golden.labels.is_empty()
        || golden.approval.rule_revision.trim().is_empty()
        || golden.approval.snapshot_digest.trim().is_empty()
    {
        return Err(EvaluationError::InvalidReview);
    }
    if !verify_approval(golden) {
        return Err(EvaluationError::UnverifiedApproval);
    }
    let report_snapshot = report
        .snapshot_items
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let report_keys = report
        .snapshot_items
        .iter()
        .map(|item| item.item_key.as_str())
        .collect::<BTreeSet<_>>();
    let approved_snapshot = golden
        .approval
        .snapshot_items
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if report_snapshot.len() != report.snapshot_items.len()
        || report_keys.len() != report.snapshot_items.len()
        || approved_snapshot.len() != golden.approval.snapshot_items.len()
    {
        return Err(EvaluationError::InvalidReview);
    }
    if golden.approval.library_version != report.library_version
        || golden.approval.rule_revision != report.rule_revision
        || golden.approval.snapshot_digest != classification_snapshot_digest(report)
        || approved_snapshot != report_snapshot
    {
        return Err(EvaluationError::SnapshotMismatch);
    }

    let classified = report
        .classified_items
        .iter()
        .map(|item| (item.item_key.as_str(), item.proposed_disposition))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut correct_count = 0;
    let mut abstention_count = 0;
    let mut by_disposition = BTreeMap::<Disposition, DispositionEvaluation>::new();

    for label in &golden.labels {
        if label.item_key.trim().is_empty() {
            return Err(EvaluationError::InvalidReview);
        }
        if label.expected_disposition == Disposition::NeedsStewardReview {
            return Err(EvaluationError::InvalidExpectedDisposition);
        }
        if !seen.insert(label.item_key.as_str()) {
            return Err(EvaluationError::DuplicateItem);
        }
        let predicted = classified
            .get(label.item_key.as_str())
            .copied()
            .ok_or(EvaluationError::UnknownItem)?;
        by_disposition.entry(predicted).or_default().predicted += 1;
        by_disposition
            .entry(label.expected_disposition)
            .or_default()
            .expected += 1;
        if predicted == label.expected_disposition {
            correct_count += 1;
            by_disposition.entry(predicted).or_default().true_positive += 1;
        }
        if predicted == Disposition::NeedsStewardReview {
            abstention_count += 1;
        }
    }

    Ok(GoldenSetEvaluation {
        review_id: golden.approval.receipt_id.clone(),
        library_version: golden.approval.library_version,
        rule_revision: golden.approval.rule_revision.clone(),
        snapshot_digest: golden.approval.snapshot_digest.clone(),
        reviewed_count: golden.labels.len(),
        correct_count,
        abstention_count,
        by_disposition,
    })
}

/// Builds a local-only reversible canonical-key manifest from steward-reviewed decisions.
pub fn build_duplicate_merge_review_manifest<F>(
    report: &ClassificationReport,
    reviewed: &ReviewedDuplicateMergeSet,
    verify_review: F,
) -> Result<DuplicateMergeReviewManifest, DuplicateReviewError>
where
    F: FnOnce(&ReviewedDuplicateMergeSet) -> bool,
{
    if reviewed.review_id.trim().is_empty()
        || reviewed.authority_receipt.trim().is_empty()
        || reviewed.snapshot_digest.trim().is_empty()
        || reviewed.rule_revision.trim().is_empty()
        || reviewed.decisions.len() != report.duplicate_candidates.len()
    {
        return Err(DuplicateReviewError::InvalidReview);
    }
    if reviewed.snapshot_digest != report.snapshot_digest
        || reviewed.library_version != report.library_version
        || reviewed.rule_revision != report.rule_revision
        || reviewed.snapshot_items != report.snapshot_items
        || reviewed.duplicate_candidates != report.duplicate_candidates
    {
        return Err(DuplicateReviewError::SnapshotMismatch);
    }
    if !verify_review(reviewed) {
        return Err(DuplicateReviewError::UnverifiedApproval);
    }

    if report
        .snapshot_items
        .iter()
        .any(|item| item.item_key.trim().is_empty())
        || report
            .snapshot_items
            .iter()
            .map(|item| item.item_key.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != report.snapshot_items.len()
    {
        return Err(DuplicateReviewError::InvalidReview);
    }
    let item_revisions = report
        .snapshot_items
        .iter()
        .map(|item| (item.item_key.as_str(), item.item_version))
        .collect::<BTreeMap<_, _>>();
    let candidates = report
        .duplicate_candidates
        .iter()
        .map(|candidate| {
            (
                (
                    candidate.identity_kind.as_str(),
                    candidate.normalized_identity.as_str(),
                ),
                candidate,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen_candidates = BTreeSet::new();
    let mut operations = Vec::with_capacity(reviewed.decisions.len());

    for decision in &reviewed.decisions {
        let candidate_key = (
            decision.identity_kind.as_str(),
            decision.normalized_identity.as_str(),
        );
        if !seen_candidates.insert(candidate_key) {
            return Err(DuplicateReviewError::DuplicateDecision);
        }
        let candidate = candidates
            .get(&candidate_key)
            .ok_or(DuplicateReviewError::UnknownCandidate)?;
        // ponytail: quadratic component expansion is enough for a steward-sized review;
        // replace with union-find only if measured duplicate sets become large.
        let mut component_keys = candidate.item_keys.iter().collect::<BTreeSet<_>>();
        loop {
            let previous_len = component_keys.len();
            for related in &report.duplicate_candidates {
                if related
                    .item_keys
                    .iter()
                    .any(|item_key| component_keys.contains(item_key))
                {
                    component_keys.extend(&related.item_keys);
                }
            }
            if component_keys.len() == previous_len {
                break;
            }
        }
        if !component_keys.contains(&decision.retained_item_key) {
            return Err(DuplicateReviewError::InvalidRetainedItem);
        }
        if reviewed.decisions.iter().any(|related_decision| {
            candidates
                .get(&(
                    related_decision.identity_kind.as_str(),
                    related_decision.normalized_identity.as_str(),
                ))
                .is_some_and(|related_candidate| {
                    related_candidate
                        .item_keys
                        .iter()
                        .any(|item_key| component_keys.contains(item_key))
                        && related_decision.retained_item_key != decision.retained_item_key
                })
        }) {
            return Err(DuplicateReviewError::InvalidReview);
        }

        let source_items = component_keys
            .iter()
            .map(|item_key| {
                item_revisions
                    .get(item_key.as_str())
                    .map(|item_version| SnapshotItemRevision {
                        item_key: (*item_key).clone(),
                        item_version: *item_version,
                    })
                    .ok_or(DuplicateReviewError::InvalidReview)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let before_canonical_keys = component_keys
            .iter()
            .map(|item_key| ((*item_key).clone(), (*item_key).clone()))
            .collect::<BTreeMap<_, _>>();
        let after_canonical_keys = component_keys
            .iter()
            .map(|item_key| ((*item_key).clone(), decision.retained_item_key.clone()))
            .collect::<BTreeMap<_, _>>();
        operations.push(DuplicateMergeOperation {
            identity_kind: decision.identity_kind.clone(),
            normalized_identity: decision.normalized_identity.clone(),
            retained_item_key: decision.retained_item_key.clone(),
            source_items,
            rollback_canonical_keys: before_canonical_keys.clone(),
            before_canonical_keys,
            after_canonical_keys,
        });
    }

    operations.sort_by(|left, right| {
        (&left.identity_kind, &left.normalized_identity)
            .cmp(&(&right.identity_kind, &right.normalized_identity))
    });
    Ok(DuplicateMergeReviewManifest {
        review_id: reviewed.review_id.clone(),
        authority_receipt: reviewed.authority_receipt.clone(),
        library_version: reviewed.library_version,
        rule_revision: reviewed.rule_revision.clone(),
        snapshot_digest: reviewed.snapshot_digest.clone(),
        operations,
        source_records_preserved: true,
    })
}

fn normalized_metadata(
    collection_keys: &[String],
    tags: &[ItemTag],
) -> Result<(Vec<String>, Vec<ItemTag>), WritePlanError> {
    if collection_keys.iter().any(|key| key.trim().is_empty())
        || tags.iter().any(|tag| tag.tag.trim().is_empty())
        || collection_keys.iter().collect::<BTreeSet<_>>().len() != collection_keys.len()
        || tags
            .iter()
            .map(|tag| tag.tag.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != tags.len()
    {
        return Err(WritePlanError::InvalidMetadata);
    }
    let mut collection_keys = collection_keys.to_vec();
    let mut tags = tags.to_vec();
    collection_keys.sort();
    tags.sort();
    Ok((collection_keys, tags))
}

/// Builds a deterministic reviewed write plan without contacting or mutating Zotero.
pub fn build_classification_write_plan<F>(
    report: &ClassificationReport,
    reviewed: &ReviewedClassificationWriteSet,
    mode: WriteMode,
    verify_review: F,
) -> Result<ClassificationWritePlan, WritePlanError>
where
    F: FnOnce(&ReviewedClassificationWriteSet) -> bool,
{
    if reviewed.review_id.trim().is_empty()
        || reviewed.authority_receipt.trim().is_empty()
        || reviewed.rule_revision.trim().is_empty()
        || reviewed.snapshot_digest.trim().is_empty()
        || reviewed.changes.is_empty()
    {
        return Err(WritePlanError::InvalidReview);
    }
    if reviewed.server_id != report.server_id
        || reviewed.library_version != report.library_version
        || reviewed.rule_revision != report.rule_revision
        || reviewed.snapshot_digest != report.snapshot_digest
    {
        return Err(WritePlanError::SnapshotMismatch);
    }
    if !verify_review(reviewed) {
        return Err(WritePlanError::UnverifiedApproval);
    }
    if mode == WriteMode::Execute
        && report
            .zotero_version
            .split('.')
            .next()
            .and_then(|major| major.parse::<u64>().ok())
            .is_none_or(|major| major < 10)
    {
        return Err(WritePlanError::UnsupportedExecute);
    }
    if mode == WriteMode::Execute
        && reviewed
            .server_id
            .as_deref()
            .is_none_or(|server_id| server_id.trim().is_empty())
    {
        return Err(WritePlanError::MissingServerIdentity);
    }

    if report
        .classified_items
        .iter()
        .map(|item| item.item_key.as_str())
        .collect::<BTreeSet<_>>()
        .len()
        != report.classified_items.len()
    {
        return Err(WritePlanError::InvalidReview);
    }
    let classified = report
        .classified_items
        .iter()
        .map(|item| (item.item_key.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let mut seen_items = BTreeSet::new();
    let mut operations = Vec::with_capacity(reviewed.changes.len());
    for change in &reviewed.changes {
        if change.item_key.trim().is_empty() {
            return Err(WritePlanError::InvalidReview);
        }
        if !seen_items.insert(change.item_key.as_str()) {
            return Err(WritePlanError::DuplicateItem);
        }
        if change.reviewed_disposition == Disposition::NeedsStewardReview {
            return Err(WritePlanError::UnreviewedDisposition);
        }
        let item = classified
            .get(change.item_key.as_str())
            .ok_or(WritePlanError::UnknownItem)?;
        let (actual_collections, actual_tags) =
            normalized_metadata(&item.collection_keys, &item.tags)?;
        let (before_collections, before_tags) =
            normalized_metadata(&change.before_collection_keys, &change.before_tags)?;
        let (after_collections, after_tags) =
            normalized_metadata(&change.after_collection_keys, &change.after_tags)?;
        if change.item_version != item.item_version
            || before_collections != actual_collections
            || before_tags != actual_tags
        {
            return Err(WritePlanError::StaleItem);
        }
        if before_collections == after_collections && before_tags == after_tags {
            return Err(WritePlanError::NoChange);
        }
        operations.push(ClassificationWriteOperation {
            item_key: change.item_key.clone(),
            item_version: change.item_version,
            reviewed_disposition: change.reviewed_disposition,
            rollback_collection_keys: before_collections.clone(),
            before_collection_keys: before_collections,
            after_collection_keys: after_collections,
            rollback_tags: before_tags.clone(),
            before_tags,
            after_tags,
        });
    }
    operations.sort_by(|left, right| left.item_key.cmp(&right.item_key));
    Ok(ClassificationWritePlan {
        mode,
        review_id: reviewed.review_id.clone(),
        authority_receipt: reviewed.authority_receipt.clone(),
        server_id: reviewed.server_id.clone(),
        library_version: reviewed.library_version,
        rule_revision: reviewed.rule_revision.clone(),
        snapshot_digest: reviewed.snapshot_digest.clone(),
        operations,
        source_records_preserved: true,
    })
}

/// Failure raised when a bounded, immutable Local API read cannot be proven.
#[derive(Debug)]
pub enum ReadError {
    /// Network or HTTP protocol failure.
    Http(String),
    /// Required response header is absent or invalid.
    Header(&'static str),
    /// Provider contract is present but unsupported.
    Contract(&'static str),
    /// A later page did not belong to the first page's snapshot.
    SnapshotChanged,
    /// Configured whole-snapshot resource budget was exceeded.
    Budget(&'static str),
    /// Zotero returned malformed JSON.
    Json(serde_json::Error),
    /// Response body exceeded the configured bound or could not be read.
    Body(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(formatter, "local API request failed: {error}"),
            Self::Header(name) => write!(formatter, "local API response lacks valid {name}"),
            Self::Contract(name) => write!(formatter, "local API returned unsupported {name}"),
            Self::SnapshotChanged => write!(formatter, "Zotero library changed during the read"),
            Self::Budget(kind) => write!(formatter, "Zotero snapshot exceeds {kind} budget"),
            Self::Json(error) => write!(formatter, "local API returned invalid JSON: {error}"),
            Self::Body(error) => write!(formatter, "local API response body failed: {error}"),
        }
    }
}

impl std::error::Error for ReadError {}

#[derive(Debug, Clone)]
struct FetchedPage {
    total: usize,
    library_version: u64,
    zotero_version: String,
    api_version: u64,
    schema_version: u64,
    server_id: Option<String>,
    body_bytes: u64,
    items: Vec<ZoteroItem>,
}

/// Reads every Zotero item from one stable Local API library version.
///
/// Snapshot consistency, resource budgets, API-version validation, pagination,
/// and duplicate-key checks live in an injectable reader core. Only the narrow
/// ureq transport shim is excluded from deterministic coverage.
pub fn read_local_snapshot() -> Result<ClassificationReport, ReadError> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .timeout_connect(Some(Duration::from_secs(2)))
        .timeout_recv_response(Some(Duration::from_secs(10)))
        .timeout_recv_body(Some(Duration::from_secs(10)))
        .max_redirects(0)
        .build();
    let agent = ureq::Agent::new_with_config(config);
    read_snapshot_with(&mut |start| fetch_local_page(&agent, start))
}

fn local_api_base() -> String {
    #[cfg(test)]
    {
        if let Some(value) = TEST_LOCAL_API
            .lock()
            .expect("test Local API lock must not be poisoned")
            .clone()
        {
            return value;
        }
    }
    LOCAL_API.to_owned()
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn fetch_local_page(agent: &ureq::Agent, start: usize) -> Result<FetchedPage, ReadError> {
    let url = format!(
        "{}?format=json&include=data&limit={PAGE_LIMIT}&start={start}",
        local_api_base()
    );
    let mut response = agent
        .get(&url)
        .header("Zotero-API-Version", SUPPORTED_API_VERSION_HEADER)
        .call()
        .map_err(|error| ReadError::Http(error.to_string()))?;
    let headers = response.headers();
    let total = header_u64(headers, "Total-Results")?;
    let total = usize::try_from(total).map_err(|_| ReadError::Budget("item-count"))?;
    let library_version = header_u64(headers, "Last-Modified-Version")?;
    let zotero_version = header_string(headers, "X-Zotero-Version")?;
    let api_version = header_u64(headers, "Zotero-API-Version")?;
    let schema_version = header_u64(headers, "Zotero-Schema-Version")?;
    let server_id = optional_header(headers, "Zotero-Server-ID");

    let body = response
        .body_mut()
        .with_config()
        .limit(MAX_PAGE_BYTES)
        .read_to_string()
        .map_err(|error| ReadError::Body(error.to_string()))?;
    let body_bytes = u64::try_from(body.len()).map_err(|_| ReadError::Budget("byte-count"))?;
    let items = serde_json::from_str(&body).map_err(ReadError::Json)?;

    Ok(FetchedPage {
        total,
        library_version,
        zotero_version,
        api_version,
        schema_version,
        server_id,
        body_bytes,
        items,
    })
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn header_u64(headers: &ureq::http::HeaderMap, name: &'static str) -> Result<u64, ReadError> {
    header_string(headers, name)?
        .parse()
        .map_err(|_| ReadError::Header(name))
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn header_string(headers: &ureq::http::HeaderMap, name: &'static str) -> Result<String, ReadError> {
    optional_header(headers, name).ok_or(ReadError::Header(name))
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn optional_header(headers: &ureq::http::HeaderMap, name: &'static str) -> Option<String> {
    headers.get(name)?.to_str().ok().map(str::to_owned)
}

fn read_snapshot_with(
    fetch_page: &mut dyn FnMut(usize) -> Result<FetchedPage, ReadError>,
) -> Result<ClassificationReport, ReadError> {
    let mut items = Vec::new();
    let mut snapshot_bytes = 0_u64;
    let mut expected_total = None;
    let mut library_version = None;
    let mut zotero_version = None;
    let mut schema_version = None;
    let mut server_id = None;

    loop {
        if expected_total.is_some_and(|total| items.len() < total)
            && snapshot_bytes >= MAX_SNAPSHOT_BYTES
        {
            return Err(ReadError::Budget("whole-snapshot"));
        }

        let page = fetch_page(items.len())?;
        if page.api_version != SUPPORTED_API_VERSION {
            return Err(ReadError::Contract("Zotero-API-Version"));
        }
        if page.total > MAX_SNAPSHOT_ITEMS {
            return Err(ReadError::Budget("item-count"));
        }

        if let Some(expected) = expected_total {
            if expected != page.total
                || library_version != Some(page.library_version)
                || zotero_version.as_ref() != Some(&page.zotero_version)
                || schema_version != Some(page.schema_version)
                || server_id != page.server_id
            {
                return Err(ReadError::SnapshotChanged);
            }
        } else {
            expected_total = Some(page.total);
            library_version = Some(page.library_version);
            zotero_version = Some(page.zotero_version.clone());
            schema_version = Some(page.schema_version);
            server_id = page.server_id.clone();
        }

        if page.items.is_empty() && items.len() < page.total {
            return Err(ReadError::SnapshotChanged);
        }
        let (next_item_count, next_snapshot_bytes) = checked_snapshot_usage(
            items.len(),
            page.items.len(),
            snapshot_bytes,
            page.body_bytes,
            page.total,
        )?;
        items.extend(page.items);
        snapshot_bytes = next_snapshot_bytes;
        debug_assert_eq!(items.len(), next_item_count);

        if items.len() == page.total {
            break;
        }
    }

    if items
        .iter()
        .map(|item| &item.key)
        .collect::<BTreeSet<_>>()
        .len()
        != items.len()
    {
        return Err(ReadError::SnapshotChanged);
    }

    let mut report = classify_snapshot(
        zotero_version.expect("a completed snapshot has Zotero version metadata"),
        server_id,
        library_version.expect("a completed snapshot has library version metadata"),
        items,
    );
    report.api_version = Some(SUPPORTED_API_VERSION);
    report.schema_version = schema_version;
    Ok(report)
}

fn checked_snapshot_usage(
    current_items: usize,
    page_items: usize,
    current_bytes: u64,
    page_bytes: u64,
    advertised_total: usize,
) -> Result<(usize, u64), ReadError> {
    if advertised_total > MAX_SNAPSHOT_ITEMS {
        return Err(ReadError::Budget("item-count"));
    }
    let next_items = current_items.saturating_add(page_items);
    if next_items > MAX_SNAPSHOT_ITEMS {
        return Err(ReadError::Budget("item-count"));
    }
    if next_items > advertised_total {
        return Err(ReadError::SnapshotChanged);
    }
    let next_bytes = current_bytes.saturating_add(page_bytes);
    if next_bytes > MAX_SNAPSHOT_BYTES {
        return Err(ReadError::Budget("byte-count"));
    }
    Ok((next_items, next_bytes))
}

/// Classifies an already captured snapshot without network access.
pub fn classify_snapshot(
    zotero_version: String,
    server_id: Option<String>,
    library_version: u64,
    mut items: Vec<ZoteroItem>,
) -> ClassificationReport {
    items.sort_by(|left, right| left.key.cmp(&right.key));
    let snapshot_items = items
        .iter()
        .map(|item| SnapshotItemRevision {
            item_key: item.key.clone(),
            item_version: item.version,
        })
        .collect();
    let snapshot_bytes = serde_json::to_vec(&items)
        .expect("Zotero snapshot items contain only JSON-compatible values");
    let snapshot_digest = format!("sha256:{:x}", Sha256::digest(snapshot_bytes));
    let children = child_index(&items);
    let bibliographic: Vec<&ZoteroItem> =
        items.iter().filter(|item| is_bibliographic(item)).collect();
    let bibliographic_item_count = bibliographic.len();
    let duplicate_candidates = duplicate_candidates(&bibliographic);
    let classified_items: Vec<ClassifiedItem> = bibliographic
        .into_iter()
        .map(|item| classify_item(item, children.get(&item.key).cloned().unwrap_or_default()))
        .collect();

    let mut disposition_counts = BTreeMap::new();
    for item in &classified_items {
        *disposition_counts
            .entry(item.proposed_disposition)
            .or_insert(0) += 1;
    }
    let audit_summary = ClassificationAudit {
        snapshot_item_count: items.len(),
        bibliographic_item_count,
        proposed_disposition_count: classified_items.len(),
        provenance_complete_count: classified_items
            .iter()
            .filter(|item| {
                !item.item_key.trim().is_empty()
                    && item
                        .child_item_keys
                        .iter()
                        .all(|child_key| !child_key.trim().is_empty())
            })
            .count(),
        abstention_count: classified_items
            .iter()
            .filter(|item| item.proposed_disposition == Disposition::NeedsStewardReview)
            .count(),
        duplicate_candidate_count: duplicate_candidates.len(),
        failure_count: 0,
        disposition_counts,
    };

    ClassificationReport {
        zotero_version,
        api_version: None,
        schema_version: None,
        server_id,
        library_version,
        rule_revision: RULE_REVISION,
        observed_item_count: items.len(),
        snapshot_items,
        snapshot_digest,
        classified_items,
        duplicate_candidates,
        audit_summary,
    }
}

fn is_bibliographic(item: &ZoteroItem) -> bool {
    item.data.parent_item.is_empty()
        && !matches!(
            item.data.item_type.as_str(),
            "attachment" | "note" | "annotation"
        )
}

fn child_index(items: &[ZoteroItem]) -> BTreeMap<String, Vec<String>> {
    let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for item in items
        .iter()
        .filter(|item| !item.data.parent_item.is_empty())
    {
        index
            .entry(item.data.parent_item.clone())
            .or_default()
            .push(item.key.clone());
    }
    index
}

fn classify_item(item: &ZoteroItem, child_item_keys: Vec<String>) -> ClassifiedItem {
    let title_normalized = item.data.title.to_lowercase();
    let abstract_normalized = item.data.abstract_note.to_lowercase();
    let tags_original = item
        .data
        .tags
        .iter()
        .map(|tag| tag.tag.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let tags_normalized = tags_original.to_lowercase();
    let fields = [
        ("title", title_normalized.as_str(), item.data.title.as_str()),
        (
            "abstract_note",
            abstract_normalized.as_str(),
            item.data.abstract_note.as_str(),
        ),
        ("tags", tags_normalized.as_str(), tags_original.as_str()),
    ];

    let specific_rules = [
        (
            Disposition::AlignmentVersioning,
            &[
                "ontology alignment",
                "ontology matching",
                "ontology mapping",
                "ontology evolution",
                "ontology versioning",
            ][..],
        ),
        (
            Disposition::Generation,
            &[
                "ontology learning",
                "ontology extraction",
                "ontology generation",
                "taxonomy induction",
                "knowledge graph construction",
            ][..],
        ),
        (
            Disposition::SemanticConsumptionBridge,
            &[
                "semantic layer",
                "semantic model",
                "ontology-based data access",
                "knowledge graph query",
                "linked data",
            ][..],
        ),
        (
            Disposition::EvaluationGovernance,
            &[
                "ontology evaluation",
                "ontology quality",
                "ontology validation",
                "ontology governance",
                "competency question",
                "shacl",
            ][..],
        ),
    ];
    let adjacent_phrases = [
        "ontology",
        "semantic web",
        "knowledge graph",
        "rdf",
        "owl",
        "skos",
    ];

    let mut matched_dispositions = Vec::new();
    let mut matched_fields = BTreeSet::new();
    let mut field_values = BTreeMap::new();
    let mut matched_phrases = BTreeSet::new();

    for (candidate, phrases) in specific_rules {
        let mut family_matched = false;
        for (field, normalized, original) in fields {
            for phrase in phrases {
                if contains_phrase(normalized, phrase) {
                    family_matched = true;
                    matched_fields.insert(field);
                    field_values
                        .entry(field)
                        .or_insert_with(|| original.to_owned());
                    matched_phrases.insert(*phrase);
                }
            }
        }
        if family_matched {
            matched_dispositions.push(candidate);
        }
    }

    let (proposed_disposition, abstention_reason) = match matched_dispositions.as_slice() {
        [] => {
            for (field, normalized, original) in fields {
                for phrase in adjacent_phrases {
                    if contains_phrase(normalized, phrase) {
                        matched_fields.insert(field);
                        field_values
                            .entry(field)
                            .or_insert_with(|| original.to_owned());
                        matched_phrases.insert(phrase);
                    }
                }
            }
            if matched_phrases.is_empty() {
                (
                    Disposition::NeedsStewardReview,
                    Some(classify_abstention_reason(&fields)),
                )
            } else {
                (Disposition::AdjacentEvidence, None)
            }
        }
        [single] => (*single, None),
        _ => (
            Disposition::NeedsStewardReview,
            Some(AbstentionReason::ConflictingDispositionEvidence),
        ),
    };

    ClassifiedItem {
        item_key: item.key.clone(),
        item_version: item.version,
        item_type: item.data.item_type.clone(),
        title: item.data.title.clone(),
        collection_keys: item.data.collections.clone(),
        tags: item.data.tags.clone(),
        proposed_disposition,
        abstention_reason,
        evidence: ClassificationEvidence {
            fields: matched_fields.into_iter().collect(),
            field_values,
            matched_phrases: matched_phrases.into_iter().collect(),
        },
        child_item_keys,
        model_receipt: None,
    }
}

fn classify_abstention_reason(fields: &[(&'static str, &str, &str)]) -> AbstentionReason {
    if fields
        .iter()
        .all(|(_, normalized, _)| normalized.trim().is_empty())
    {
        return AbstentionReason::MissingClassificationMetadata;
    }
    if fields.iter().any(|(_, _, original)| {
        original
            .chars()
            .any(|character| character.is_alphabetic() && !character.is_ascii())
    }) {
        return AbstentionReason::UnsupportedRuleVocabulary;
    }
    AbstentionReason::NoDeterministicRuleMatch
}

fn contains_phrase(value: &str, phrase: &str) -> bool {
    value.match_indices(phrase).any(|(start, matched)| {
        let before = value[..start].chars().next_back();
        let after = value[start + matched.len()..].chars().next();
        before.is_none_or(|character| !character.is_alphanumeric())
            && after.is_none_or(|character| !character.is_alphanumeric())
    })
}

fn duplicate_candidates(items: &[&ZoteroItem]) -> Vec<DuplicateCandidate> {
    let mut identities: BTreeMap<(&'static str, String), Vec<String>> = BTreeMap::new();
    for item in items {
        if let Some(doi) = normalize_doi(&item.data.doi) {
            identities
                .entry(("doi", doi))
                .or_default()
                .push(item.key.clone());
        }
        if let Some(title) = normalize_title(&item.data.title) {
            identities
                .entry(("title", title))
                .or_default()
                .push(item.key.clone());
        }
    }
    identities
        .into_iter()
        .filter_map(|((identity_kind, normalized_identity), item_keys)| {
            (item_keys.len() > 1).then_some(DuplicateCandidate {
                identity_kind: identity_kind.into(),
                normalized_identity,
                item_keys,
            })
        })
        .collect()
}

fn normalize_doi(value: &str) -> Option<String> {
    let normalized = value.trim().to_lowercase();
    let normalized = normalized
        .strip_prefix("https://doi.org/")
        .or_else(|| normalized.strip_prefix("http://doi.org/"))
        .or_else(|| normalized.strip_prefix("https://dx.doi.org/"))
        .or_else(|| normalized.strip_prefix("http://dx.doi.org/"))
        .or_else(|| normalized.strip_prefix("doi:"))
        .unwrap_or(&normalized)
        .trim();
    (!normalized.is_empty()).then(|| normalized.to_owned())
}

fn normalize_title(value: &str) -> Option<String> {
    let mut normalized = String::new();
    let mut separated = true;
    for character in value.trim().to_lowercase().chars() {
        if character.is_alphanumeric() {
            normalized.push(character);
            separated = false;
        } else if !separated {
            normalized.push(' ');
            separated = true;
        }
    }
    if normalized.ends_with(' ') {
        normalized.pop();
    }
    (!normalized.is_empty()).then_some(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    static LOCAL_API_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn item(key: &str, item_type: &str, title: &str, doi: &str, parent: &str) -> ZoteroItem {
        ZoteroItem {
            key: key.into(),
            version: 7,
            data: ItemData {
                item_type: item_type.into(),
                title: title.into(),
                abstract_note: String::new(),
                doi: doi.into(),
                parent_item: parent.into(),
                collections: vec![],
                tags: vec![],
            },
        }
    }

    fn fetched_page(total: usize, items: Vec<ZoteroItem>) -> FetchedPage {
        FetchedPage {
            total,
            library_version: 42,
            zotero_version: "9.0.6".into(),
            api_version: 3,
            schema_version: 42,
            server_id: Some("server".into()),
            body_bytes: 100,
            items,
        }
    }

    #[test]
    fn production_wrapper_requests_api_v3_and_records_contract_versions() {
        let _guard = LOCAL_API_TEST_LOCK.lock().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let base = format!("http://{address}/api/users/0/items");
        *TEST_LOCAL_API.lock().unwrap() = Some(base);

        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = vec![0_u8; 4096];
            let read = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..read]).to_lowercase();
            assert!(request.contains("zotero-api-version: 3"));
            let body = r#"[{"key":"A","version":1,"data":{"itemType":"book","title":"ontology evaluation"}}]"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nTotal-Results: 1\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nZotero-Server-ID: server\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
            stream.flush().unwrap();
        });

        let report = read_local_snapshot().unwrap();
        *TEST_LOCAL_API.lock().unwrap() = None;
        server.join().unwrap();

        assert_eq!(report.api_version, Some(3));
        assert_eq!(report.schema_version, Some(42));
        assert_eq!(report.library_version, 42);
        assert_eq!(report.classified_items.len(), 1);
        assert_eq!(local_api_base(), LOCAL_API);
    }

    #[test]
    fn reader_core_paginates_and_preserves_one_snapshot_contract() {
        let mut pages = vec![
            fetched_page(2, vec![item("A", "book", "ontology quality", "", "")]),
            fetched_page(2, vec![item("B", "book", "semantic web", "", "")]),
        ]
        .into_iter();
        let report = read_snapshot_with(&mut |_| Ok(pages.next().unwrap())).unwrap();
        assert_eq!(report.observed_item_count, 2);
        assert_eq!(report.api_version, Some(3));
        assert_eq!(report.schema_version, Some(42));

        let empty = read_snapshot_with(&mut |_| Ok(fetched_page(0, vec![]))).unwrap();
        assert_eq!(empty.observed_item_count, 0);
    }

    #[test]
    fn reader_core_rejects_contract_drift_empty_pages_and_duplicate_keys() {
        let mut unsupported = fetched_page(1, vec![item("A", "book", "x", "", "")]);
        unsupported.api_version = 4;
        assert!(matches!(
            read_snapshot_with(&mut |_| Ok(unsupported.clone())),
            Err(ReadError::Contract("Zotero-API-Version"))
        ));

        assert!(matches!(
            read_snapshot_with(&mut |_| Err(ReadError::Http("offline".into()))),
            Err(ReadError::Http(_))
        ));

        for changed in ["total", "library", "zotero", "schema", "server"] {
            let mut second = fetched_page(2, vec![item("B", "book", "y", "", "")]);
            match changed {
                "total" => second.total = 3,
                "library" => second.library_version += 1,
                "zotero" => second.zotero_version = "10.0".into(),
                "schema" => second.schema_version += 1,
                "server" => second.server_id = Some("other".into()),
                _ => unreachable!(),
            }
            let mut pages = vec![
                fetched_page(2, vec![item("A", "book", "x", "", "")]),
                second,
            ]
            .into_iter();
            assert!(matches!(
                read_snapshot_with(&mut |_| Ok(pages.next().unwrap())),
                Err(ReadError::SnapshotChanged)
            ));
        }

        assert!(matches!(
            read_snapshot_with(&mut |_| Ok(fetched_page(1, vec![]))),
            Err(ReadError::SnapshotChanged)
        ));

        assert!(matches!(
            read_snapshot_with(&mut |_| {
                Ok(fetched_page(
                    2,
                    vec![
                        item("A", "book", "x", "", ""),
                        item("A", "book", "y", "", ""),
                    ],
                ))
            }),
            Err(ReadError::SnapshotChanged)
        ));
    }

    #[test]
    fn reader_core_rejects_total_and_between_request_resource_exhaustion() {
        let too_many = fetched_page(MAX_SNAPSHOT_ITEMS + 1, vec![]);
        assert!(matches!(
            read_snapshot_with(&mut |_| Ok(too_many.clone())),
            Err(ReadError::Budget("item-count"))
        ));

        let mut calls = 0;
        assert!(matches!(
            read_snapshot_with(&mut |_| {
                calls += 1;
                let mut page = fetched_page(2, vec![item("A", "book", "x", "", "")]);
                page.body_bytes = MAX_SNAPSHOT_BYTES;
                Ok(page)
            }),
            Err(ReadError::Budget("whole-snapshot"))
        ));
        assert_eq!(calls, 1);

        let mut oversized = fetched_page(1, vec![item("A", "book", "x", "", "")]);
        oversized.body_bytes = MAX_SNAPSHOT_BYTES + 1;
        assert!(matches!(
            read_snapshot_with(&mut |_| Ok(oversized.clone())),
            Err(ReadError::Budget("byte-count"))
        ));
    }

    #[test]
    fn snapshot_usage_is_bounded_before_accumulation() {
        assert_eq!(checked_snapshot_usage(1, 1, 10, 20, 2).unwrap(), (2, 30));
        assert!(matches!(
            checked_snapshot_usage(0, 1, 0, 1, MAX_SNAPSHOT_ITEMS + 1),
            Err(ReadError::Budget("item-count"))
        ));
        assert!(matches!(
            checked_snapshot_usage(MAX_SNAPSHOT_ITEMS, 1, 0, 1, MAX_SNAPSHOT_ITEMS),
            Err(ReadError::Budget("item-count"))
        ));
        assert!(matches!(
            checked_snapshot_usage(usize::MAX, 1, 0, 1, usize::MAX),
            Err(ReadError::Budget("item-count"))
        ));
        assert!(matches!(
            checked_snapshot_usage(0, 1, MAX_SNAPSHOT_BYTES, 1, 1),
            Err(ReadError::Budget("byte-count"))
        ));
        assert!(matches!(
            checked_snapshot_usage(0, 1, u64::MAX, 1, 1),
            Err(ReadError::Budget("byte-count"))
        ));
        assert!(matches!(
            checked_snapshot_usage(1, 1, 0, 1, 1),
            Err(ReadError::SnapshotChanged)
        ));
    }

    #[test]
    fn classifies_every_bibliographic_item_and_links_children() {
        let mut generation = item("B", "journalArticle", "Ontology Learning", "10.1/X", "");
        generation.data.tags.push(ItemTag {
            tag: "SHACL".into(),
            tag_type: Some(1),
        });
        let report = classify_snapshot(
            "9.0.6".into(),
            None,
            42,
            vec![
                item("C", "attachment", "", "", "B"),
                item("D", "note", "Ignored", "", ""),
                generation,
                item("A", "book", "Other", "", ""),
            ],
        );
        assert_eq!(report.observed_item_count, 4);
        assert_eq!(report.classified_items.len(), 2);
        assert_eq!(
            report.classified_items[0].abstention_reason,
            Some(AbstentionReason::NoDeterministicRuleMatch)
        );
        assert_eq!(
            report.classified_items[1].proposed_disposition,
            Disposition::NeedsStewardReview
        );
        assert_eq!(
            report.classified_items[1].abstention_reason,
            Some(AbstentionReason::ConflictingDispositionEvidence)
        );
        assert_eq!(report.classified_items[1].child_item_keys, ["C"]);
        assert_eq!(
            report.classified_items[1].evidence.fields,
            ["tags", "title"]
        );
    }

    #[test]
    fn specific_rule_families_and_conflicts_are_deterministic() {
        let cases = [
            ("taxonomy induction", Disposition::Generation),
            (
                "ontology-based data access",
                Disposition::SemanticConsumptionBridge,
            ),
            ("ontology quality", Disposition::EvaluationGovernance),
            ("semantic web", Disposition::AdjacentEvidence),
        ];
        for (title, expected) in cases {
            let report = classify_snapshot(
                "10".into(),
                Some("s".into()),
                1,
                vec![item("A", "book", title, "", "")],
            );
            assert_eq!(report.classified_items[0].proposed_disposition, expected);
        }

        let conflict = classify_snapshot(
            "10".into(),
            None,
            1,
            vec![item(
                "A",
                "book",
                "ontology matching and ontology learning",
                "",
                "",
            )],
        );
        assert_eq!(
            conflict.classified_items[0].proposed_disposition,
            Disposition::NeedsStewardReview
        );
        assert_eq!(
            conflict.classified_items[0].abstention_reason,
            Some(AbstentionReason::ConflictingDispositionEvidence)
        );
    }

    #[test]
    fn matched_values_and_abstention_reasons_are_replayable() {
        let mut abstract_match = item("A", "book", "Uninformative", "", "");
        abstract_match.data.abstract_note = "Evidence for ontology alignment".into();
        let report = classify_snapshot("10".into(), None, 1, vec![abstract_match]);
        assert_eq!(
            report.classified_items[0]
                .evidence
                .field_values
                .get("abstract_note")
                .map(String::as_str),
            Some("Evidence for ontology alignment")
        );

        let missing = classify_snapshot("10".into(), None, 1, vec![item("A", "book", "", "", "")]);
        assert_eq!(
            missing.classified_items[0].abstention_reason,
            Some(AbstentionReason::MissingClassificationMetadata)
        );

        let unsupported = classify_snapshot(
            "10".into(),
            None,
            1,
            vec![item("A", "book", "온톨로지 정렬", "", "")],
        );
        assert_eq!(
            unsupported.classified_items[0].abstention_reason,
            Some(AbstentionReason::UnsupportedRuleVocabulary)
        );
    }

    #[test]
    fn phrase_boundaries_and_duplicate_normalization_are_exact() {
        assert!(!contains_phrase("knowledge", "owl"));
        assert!(!contains_phrase("growl", "owl"));
        assert!(contains_phrase("owl-based", "owl"));
        assert!(contains_phrase("uses owl", "owl"));
        assert!(contains_phrase("owl", "owl"));

        assert_eq!(normalize_doi("  "), None);
        assert_eq!(normalize_title("---"), None);
        assert_eq!(normalize_doi("http://doi.org/A"), Some("a".into()));
        assert_eq!(normalize_doi("https://doi.org/B"), Some("b".into()));
        assert_eq!(normalize_doi("http://dx.doi.org/C"), Some("c".into()));
        assert_eq!(normalize_doi("https://dx.doi.org/D"), Some("d".into()));
        assert_eq!(normalize_doi("E"), Some("e".into()));
        assert_eq!(normalize_title(" A---B "), Some("a b".into()));
        assert_eq!(normalize_title(" A--- "), Some("a".into()));

        let report = classify_snapshot(
            "10".into(),
            None,
            1,
            vec![
                item("A", "book", "OWL: Overview", "doi:10.1/X", ""),
                item("B", "book", "owl overview", "https://dx.doi.org/10.1/x", ""),
            ],
        );
        assert_eq!(report.duplicate_candidates.len(), 2);
        assert!(
            report
                .duplicate_candidates
                .iter()
                .all(|candidate| candidate.item_keys == ["A", "B"])
        );
    }

    #[test]
    fn read_errors_are_actionable() {
        assert!(ReadError::Header("x").to_string().contains('x'));
        assert!(ReadError::Contract("v").to_string().contains("unsupported"));
        assert!(ReadError::SnapshotChanged.to_string().contains("changed"));
        assert!(ReadError::Budget("items").to_string().contains("budget"));
        assert!(ReadError::Http("down".into()).to_string().contains("down"));
        assert!(
            ReadError::Body("large".into())
                .to_string()
                .contains("large")
        );
        let json_error = serde_json::from_str::<ZoteroItem>("{}").unwrap_err();
        assert!(ReadError::Json(json_error).to_string().contains("JSON"));
    }
}
