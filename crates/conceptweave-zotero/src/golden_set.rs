use super::{ClassificationReport, Disposition, ZoteroItem, classify_snapshot};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const PROVIDER_SNAPSHOT_DIGEST_DOMAIN: &str = "conceptweave-zotero-provider-snapshot-v3";
const TYPED_SNAPSHOT_DIGEST_DOMAIN: &str = "conceptweave-zotero-typed-snapshot-v3";
const PROPOSAL_DIGEST_DOMAIN: &str = "conceptweave-classification-proposals-v3";

fn disposition_rank(value: Disposition) -> u8 {
    match value {
        Disposition::Generation => 0,
        Disposition::AlignmentVersioning => 1,
        Disposition::SemanticConsumptionBridge => 2,
        Disposition::EvaluationGovernance => 3,
        Disposition::AdjacentEvidence => 4,
        Disposition::OutOfScope => 5,
        Disposition::NeedsStewardReview => 6,
    }
}

impl Ord for Disposition {
    fn cmp(&self, other: &Self) -> Ordering {
        disposition_rank(*self).cmp(&disposition_rank(*other))
    }
}

impl PartialOrd for Disposition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'de> Deserialize<'de> for Disposition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "generation" => Ok(Self::Generation),
            "alignment_versioning" => Ok(Self::AlignmentVersioning),
            "semantic_consumption_bridge" => Ok(Self::SemanticConsumptionBridge),
            "evaluation_governance" => Ok(Self::EvaluationGovernance),
            "adjacent_evidence" => Ok(Self::AdjacentEvidence),
            "out_of_scope" => Ok(Self::OutOfScope),
            "needs_steward_review" => Ok(Self::NeedsStewardReview),
            _ => Err(serde::de::Error::unknown_variant(
                &value,
                &[
                    "generation",
                    "alignment_versioning",
                    "semantic_consumption_bridge",
                    "evaluation_governance",
                    "adjacent_evidence",
                    "out_of_scope",
                    "needs_steward_review",
                ],
            )),
        }
    }
}

/// A provider record captured before projection into the stable public `ZoteroItem` shape.
///
/// The raw JSON stays private so callers cannot mutate it independently of the typed
/// classifier input. Construct this boundary from provider JSON with `TryFrom<Value>`.
#[derive(Debug, Clone)]
pub struct CapturedZoteroItem {
    item: ZoteroItem,
    source_record: Value,
}

impl CapturedZoteroItem {
    /// Returns the immutable typed classifier input decoded from the captured provider record.
    pub fn item(&self) -> &ZoteroItem {
        &self.item
    }
}

impl TryFrom<Value> for CapturedZoteroItem {
    type Error = serde_json::Error;

    fn try_from(source_record: Value) -> Result<Self, Self::Error> {
        let item = serde_json::from_value(source_record.clone())?;
        Ok(Self {
            item,
            source_record,
        })
    }
}

/// One item revision in the exact reviewed classification snapshot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SnapshotItemRevision {
    /// Stable Zotero item key.
    pub item_key: String,
    /// Item revision observed during review.
    pub item_version: u64,
}

/// A trusted Research Intake report paired with immutable evaluation snapshot evidence.
///
/// The underlying `ClassificationReport` remains constructor-bound and private. This wrapper
/// adds only evaluation evidence produced from the same admitted inputs.
#[derive(Debug)]
pub struct GoldenSnapshot {
    report: ClassificationReport,
    snapshot_items: Vec<SnapshotItemRevision>,
    snapshot_digest: String,
}

impl GoldenSnapshot {
    /// Returns the trusted Research Intake aggregate without exposing mutable internals.
    pub fn report(&self) -> &ClassificationReport {
        &self.report
    }

    /// Returns the canonical key/revision inventory bound by review approval.
    pub fn snapshot_items(&self) -> &[SnapshotItemRevision] {
        &self.snapshot_items
    }

    /// Returns the versioned SHA-256 snapshot digest.
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }
}

/// Classifies an explicitly typed offline fixture and binds exactly those typed inputs.
///
/// This path is for deterministic fixtures or already-controlled typed evidence. It is not a
/// substitute for provider authenticity because unknown/omitted provider JSON is unavailable.
pub fn classify_typed_golden_snapshot(
    zotero_version: String,
    server_id: Option<String>,
    library_version: u64,
    mut items: Vec<ZoteroItem>,
) -> GoldenSnapshot {
    items.sort_by(|left, right| {
        (&left.key, left.version).cmp(&(&right.key, right.version))
    });
    let snapshot_items = snapshot_revisions(&items);
    let snapshot_bytes = serde_json::to_vec(&(TYPED_SNAPSHOT_DIGEST_DOMAIN, &items))
        .expect("typed Zotero inputs are JSON-serializable");
    let snapshot_digest = sha256(snapshot_bytes);
    let report = classify_snapshot(zotero_version, server_id, library_version, items);
    GoldenSnapshot {
        report,
        snapshot_items,
        snapshot_digest,
    }
}

/// Classifies provider-captured records while binding the complete raw JSON and typed inputs.
///
/// Provider objects are sorted by stable key/revision. JSON object key order is canonicalized,
/// while array order and omitted-versus-present fields remain meaningful evidence.
pub fn classify_captured_golden_snapshot(
    zotero_version: String,
    server_id: Option<String>,
    library_version: u64,
    mut items: Vec<CapturedZoteroItem>,
) -> GoldenSnapshot {
    items.sort_by(|left, right| {
        (&left.item.key, left.item.version).cmp(&(&right.item.key, right.item.version))
    });
    let snapshot_items = items
        .iter()
        .map(|captured| SnapshotItemRevision {
            item_key: captured.item.key.clone(),
            item_version: captured.item.version,
        })
        .collect();
    let bound_records = items
        .iter()
        .map(|captured| (canonical_json(&captured.source_record), &captured.item))
        .collect::<Vec<_>>();
    let snapshot_bytes = serde_json::to_vec(&(PROVIDER_SNAPSHOT_DIGEST_DOMAIN, bound_records))
        .expect("captured Zotero inputs are JSON-serializable");
    let snapshot_digest = sha256(snapshot_bytes);
    let typed_items = items.into_iter().map(|captured| captured.item).collect();
    let report = classify_snapshot(zotero_version, server_id, library_version, typed_items);
    GoldenSnapshot {
        report,
        snapshot_items,
        snapshot_digest,
    }
}

fn snapshot_revisions(items: &[ZoteroItem]) -> Vec<SnapshotItemRevision> {
    items
        .iter()
        .map(|item| SnapshotItemRevision {
            item_key: item.key.clone(),
            item_version: item.version,
        })
        .collect()
}

fn canonical_json(value: &Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.iter().map(canonical_json).collect()),
        Value::Object(values) => {
            let sorted = values
                .iter()
                .map(|(key, value)| (key.clone(), canonical_json(value)))
                .collect::<BTreeMap<_, _>>();
            Value::Object(sorted.into_iter().collect())
        }
        _ => value.clone(),
    }
}

fn sha256(bytes: Vec<u8>) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
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

/// Governance receipt binding a steward approval to exact inputs and proposals.
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
    /// Immutable digest over the approved source/classifier-input snapshot.
    pub snapshot_digest: String,
    /// Digest of the complete proposal/report evidence reviewed by the steward.
    pub proposal_digest: String,
    /// Complete sorted item-revision identity of the reviewed report.
    pub snapshot_items: Vec<SnapshotItemRevision>,
}

/// Version-bound steward labels that remain outside the trusted report aggregate.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewedGoldenSet {
    /// Approval receipt verified by the caller's governance boundary.
    pub approval: GoldenSetApproval,
    /// Item-level expected dispositions.
    pub labels: Vec<GoldenLabel>,
}

/// Returns the canonical content identity verified by a golden-set approval.
pub fn classification_snapshot_digest(snapshot: &GoldenSnapshot) -> String {
    snapshot.snapshot_digest.clone()
}

/// Computes the versioned SHA-256 identity of all current Research Intake report evidence.
///
/// The digest covers lifecycle fields carried by proposals, source inventory, unresolved scope,
/// duplicate evidence, reader metadata, and every proposal whether or not it is in the reviewed
/// sample. Record ordering is canonicalized; changing material evidence changes the digest.
pub fn classification_proposal_digest(snapshot: &GoldenSnapshot) -> String {
    let report = snapshot.report();
    let mut proposals = report.classified_items().iter().collect::<Vec<_>>();
    proposals.sort_by_key(|item| (&item.item_key, item.item_version));
    let mut source_items = report.unclassified_items().iter().collect::<Vec<_>>();
    source_items.sort_by_key(|item| (&item.key, item.version));
    let mut pending_keys = report.pending_source_item_keys().iter().collect::<Vec<_>>();
    pending_keys.sort();
    let mut duplicate_candidates = report.duplicate_candidates().iter().collect::<Vec<_>>();
    duplicate_candidates.sort_by(|left, right| {
        (left.identity_kind, &left.normalized_identity)
            .cmp(&(right.identity_kind, &right.normalized_identity))
    });
    let bytes = serde_json::to_vec(&(
        PROPOSAL_DIGEST_DOMAIN,
        report.zotero_version(),
        report.api_version(),
        report.schema_version(),
        report.server_id(),
        report.library_version(),
        report.rule_revision(),
        report.observed_item_count(),
        proposals,
        source_items,
        pending_keys,
        duplicate_candidates,
    ))
    .expect("Research Intake report evidence is JSON-serializable");
    sha256(bytes)
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
    /// Opaque digest binding the exact proposal records used for these counts.
    pub proposal_digest: String,
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
    /// Review receipt, labels, or report structure are missing or incompatible.
    InvalidReview,
    /// The golden set was reviewed against another snapshot or proposal state.
    SnapshotMismatch,
    /// The caller's governance boundary did not verify the approval receipt and labels.
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
            Self::InvalidReview => "golden-set review metadata or report structure is invalid",
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

/// Validates that the immutable Research Intake aggregate matches its bound snapshot inventory.
///
/// Because report fields are private, this checks constructor output rather than accepting a
/// caller-mutated trusted report. Orphans and disconnected cycles remain pending evidence.
pub fn validate_classification_report(snapshot: &GoldenSnapshot) -> Result<(), EvaluationError> {
    let report = snapshot.report();
    let invalid = EvaluationError::InvalidReview;
    if report.observed_item_count() != snapshot.snapshot_items.len()
        || report
            .classified_items()
            .len()
            .checked_add(report.unclassified_items().len())
            != Some(report.observed_item_count())
    {
        return Err(invalid);
    }

    let mut remaining_items = BTreeMap::new();
    for item in &snapshot.snapshot_items {
        if item.item_key.trim().is_empty()
            || item.item_version > report.library_version()
            || remaining_items
                .insert(item.item_key.as_str(), item.item_version)
                .is_some()
        {
            return Err(invalid);
        }
    }

    let children = child_index(report.unclassified_items());
    for item in report.classified_items() {
        let mut reported_children = item.child_item_keys.clone();
        reported_children.sort();
        let mut actual_children = children.get(&item.item_key).cloned().unwrap_or_default();
        actual_children.sort();
        if item.item_key.trim().is_empty()
            || item.item_type.trim().is_empty()
            || matches!(item.item_type.as_str(), "attachment" | "note" | "annotation")
            || item.truth_status != "proposed"
            || item.publication_state != "proposed"
            || remaining_items.remove(item.item_key.as_str()) != Some(item.item_version)
            || reported_children != actual_children
        {
            return Err(invalid);
        }
    }

    for item in report.unclassified_items() {
        if item.key.trim().is_empty()
            || item.data.item_type.trim().is_empty()
            || is_bibliographic(item)
            || remaining_items.remove(item.key.as_str()) != Some(item.version)
        {
            return Err(invalid);
        }
    }
    if !remaining_items.is_empty() {
        return Err(invalid);
    }

    let expected_pending = pending_source_keys(
        report.classified_items().iter().map(|item| item.item_key.as_str()),
        report.unclassified_items(),
        children,
    );
    let mut reported_pending = report.pending_source_item_keys().to_vec();
    reported_pending.sort();
    if reported_pending != expected_pending {
        return Err(invalid);
    }
    Ok(())
}

fn is_bibliographic(item: &ZoteroItem) -> bool {
    item.data.parent_item.is_empty()
        && !matches!(item.data.item_type.as_str(), "attachment" | "note" | "annotation")
}

fn child_index(items: &[ZoteroItem]) -> BTreeMap<String, Vec<String>> {
    let mut children = BTreeMap::<String, Vec<String>>::new();
    for item in items.iter().filter(|item| !item.data.parent_item.is_empty()) {
        children
            .entry(item.data.parent_item.clone())
            .or_default()
            .push(item.key.clone());
    }
    children
}

fn pending_source_keys<'a>(
    classified_keys: impl Iterator<Item = &'a str>,
    unclassified_items: &[ZoteroItem],
    mut children: BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut pending = unclassified_items
        .iter()
        .map(|item| item.key.clone())
        .collect::<BTreeSet<_>>();
    let mut parents = classified_keys.map(str::to_owned).collect::<Vec<_>>();
    while let Some(parent) = parents.pop() {
        for child in children.remove(&parent).unwrap_or_default() {
            pending.remove(&child);
            parents.push(child);
        }
    }
    pending.into_iter().collect()
}

/// Evaluates reviewed labels without copying item identities into the result.
///
/// All structural, snapshot, proposal, and label checks run before the external governance
/// verifier. The verifier must authenticate the complete reviewed set, not merely a receipt ID.
pub fn evaluate_reviewed_golden_set<F>(
    snapshot: &GoldenSnapshot,
    golden: &ReviewedGoldenSet,
    verify_approval: F,
) -> Result<GoldenSetEvaluation, EvaluationError>
where
    F: FnOnce(&ReviewedGoldenSet) -> bool,
{
    validate_classification_report(snapshot)?;
    let report = snapshot.report();
    if golden.approval.receipt_id.trim().is_empty()
        || golden.approval.reviewer_subject.trim().is_empty()
        || golden.labels.is_empty()
        || golden.approval.rule_revision.trim().is_empty()
        || golden.approval.snapshot_digest.trim().is_empty()
        || golden.approval.proposal_digest.trim().is_empty()
    {
        return Err(EvaluationError::InvalidReview);
    }

    let report_snapshot = snapshot
        .snapshot_items
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let approved_snapshot = golden
        .approval
        .snapshot_items
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if approved_snapshot.len() != golden.approval.snapshot_items.len() {
        return Err(EvaluationError::InvalidReview);
    }
    if golden.approval.library_version != report.library_version()
        || golden.approval.rule_revision != report.rule_revision()
        || golden.approval.snapshot_digest != classification_snapshot_digest(snapshot)
        || approved_snapshot != report_snapshot
        || golden.approval.proposal_digest != classification_proposal_digest(snapshot)
    {
        return Err(EvaluationError::SnapshotMismatch);
    }

    let classified = report
        .classified_items()
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

    if !verify_approval(golden) {
        return Err(EvaluationError::UnverifiedApproval);
    }

    Ok(GoldenSetEvaluation {
        review_id: golden.approval.receipt_id.clone(),
        library_version: golden.approval.library_version,
        rule_revision: golden.approval.rule_revision.clone(),
        snapshot_digest: golden.approval.snapshot_digest.clone(),
        proposal_digest: golden.approval.proposal_digest.clone(),
        reviewed_count: golden.labels.len(),
        correct_count,
        abstention_count,
        by_disposition,
    })
}
