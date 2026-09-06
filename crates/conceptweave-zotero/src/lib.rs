#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! Deterministic, read-only classification of a Zotero library snapshot.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::Read;
use std::time::{Duration, Instant};

/// Classification rule revision recorded in every report.
pub const RULE_REVISION: &str = "ontology-research-v2";

const SUPPORTED_API_VERSION: u64 = 3;
const SUPPORTED_API_VERSION_HEADER: &str = "3";
const PAGE_LIMIT: usize = 100;
const MAX_PAGE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_SNAPSHOT_ITEMS: usize = 50_000;
const MAX_SNAPSHOT_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SNAPSHOT_ELAPSED: Duration = Duration::from_secs(300);
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemTag {
    /// Tag text.
    pub tag: String,
}

/// One mutually exclusive proposed disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
    pub tags: Vec<String>,
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
#[derive(Debug, Serialize)]
pub struct DuplicateCandidate {
    /// Identity kind used for the candidate group.
    pub identity_kind: &'static str,
    /// Normalized identity value.
    pub normalized_identity: String,
    /// Zotero item keys sharing the identity.
    pub item_keys: Vec<String>,
}

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
    /// One proposal for every top-level bibliographic item.
    pub classified_items: Vec<ClassifiedItem>,
    /// Metadata for every remaining record, sorted by its original key.
    ///
    /// Together with `classified_items`, this accounts for all observed items.
    /// Notes, attachments and annotations remain evidence, not paper proposals.
    /// Only the fields represented by `ItemData` are retained; this is not a
    /// full-text capture or a lossless copy of the provider's original JSON.
    pub unclassified_items: Vec<ZoteroItem>,
    /// Sorted keys whose parent chain does not reach a bibliographic proposal.
    ///
    /// Standalone sources, their descendants, orphan trees and cycles remain
    /// pending. An empty list proves only parent-link accounting for this input,
    /// never research completion, semantic approval or permission to write.
    pub pending_source_item_keys: Vec<String>,
    /// Reversible DOI/title duplicate candidates.
    pub duplicate_candidates: Vec<DuplicateCandidate>,
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
/// No page starts or completed report is accepted at or beyond five minutes.
/// An in-flight request may finish later under its existing per-request limits;
/// its late result is rejected, not returned as a partial snapshot.
pub fn read_local_snapshot() -> Result<ClassificationReport, ReadError> {
    let config = ureq::Agent::config_builder()
        .proxy(None)
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

    let body = read_bounded_response_text(&mut response, MAX_PAGE_BYTES)
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

/// Reads strict UTF-8 with an inclusive byte limit and one byte of overrun evidence.
fn read_bounded_response_text(
    response: &mut ureq::http::Response<ureq::Body>,
    limit: u64,
) -> std::io::Result<String> {
    let mut body = String::new();
    response
        .body_mut()
        .as_reader()
        .take(limit.saturating_add(1))
        .read_to_string(&mut body)?;
    if body.len() as u64 > limit {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "response body exceeds byte limit",
        ));
    }
    Ok(body)
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
    let started = Instant::now();
    read_snapshot_with_clock(fetch_page, &mut || started.elapsed())
}

fn read_snapshot_with_clock(
    fetch_page: &mut dyn FnMut(usize) -> Result<FetchedPage, ReadError>,
    elapsed: &mut dyn FnMut() -> Duration,
) -> Result<ClassificationReport, ReadError> {
    let mut items = Vec::new();
    let mut snapshot_bytes = 0_u64;
    let mut expected_total = None;
    let mut library_version = None;
    let mut zotero_version = None;
    let mut schema_version = None;
    let mut server_id = None;

    loop {
        if elapsed() >= MAX_SNAPSHOT_ELAPSED {
            return Err(ReadError::Budget("elapsed-time"));
        }
        if expected_total.is_some_and(|total| items.len() < total)
            && snapshot_bytes >= MAX_SNAPSHOT_BYTES
        {
            return Err(ReadError::Budget("whole-snapshot"));
        }

        let page = fetch_page(items.len())?;
        if elapsed() >= MAX_SNAPSHOT_ELAPSED {
            return Err(ReadError::Budget("elapsed-time"));
        }
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
        if page
            .items
            .iter()
            .any(|item| item.version > page.library_version)
        {
            return Err(ReadError::SnapshotChanged);
        }
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
    if elapsed() >= MAX_SNAPSHOT_ELAPSED {
        return Err(ReadError::Budget("elapsed-time"));
    }
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
    let children = child_index(&items);
    let bibliographic: Vec<&ZoteroItem> =
        items.iter().filter(|item| is_bibliographic(item)).collect();
    let duplicate_candidates = duplicate_candidates(&bibliographic);
    let classified_items: Vec<_> = bibliographic
        .into_iter()
        .map(|item| classify_item(item, children.get(&item.key).cloned().unwrap_or_default()))
        .collect();
    let observed_item_count = items.len();
    let unclassified_items: Vec<_> = items
        .into_iter()
        .filter(|item| !is_bibliographic(item))
        .collect();
    let mut pending_source_item_keys: BTreeSet<_> = unclassified_items
        .iter()
        .map(|item| item.key.clone())
        .collect();
    let mut parent_item_keys: Vec<_> = classified_items
        .iter()
        .map(|item| item.item_key.clone())
        .collect();
    while let Some(parent_item_key) = parent_item_keys.pop() {
        for child_item_key in children.get(&parent_item_key).into_iter().flatten() {
            if pending_source_item_keys.remove(child_item_key) {
                parent_item_keys.push(child_item_key.clone());
            }
        }
    }

    ClassificationReport {
        zotero_version,
        api_version: None,
        schema_version: None,
        server_id,
        library_version,
        rule_revision: RULE_REVISION,
        observed_item_count,
        classified_items,
        unclassified_items,
        pending_source_item_keys: pending_source_item_keys.into_iter().collect(),
        duplicate_candidates,
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
        tags: item.data.tags.iter().map(|tag| tag.tag.clone()).collect(),
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
                identity_kind,
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
    mod metadata_transport;

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
    fn reader_deadline_rejects_slow_drip_without_another_request() {
        let elapsed = std::cell::Cell::new(Duration::ZERO);
        let mut starts = Vec::new();
        let result = read_snapshot_with_clock(
            &mut |start| {
                starts.push(start);
                elapsed.set(elapsed.get() + Duration::from_secs(50));
                Ok(fetched_page(
                    7,
                    vec![item(&format!("P{start}"), "book", "semantic web", "", "")],
                ))
            },
            &mut || elapsed.get(),
        );
        assert!(matches!(result, Err(ReadError::Budget("elapsed-time"))));
        assert_eq!(starts, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn reader_deadline_rejects_expired_admission_page_and_report() {
        // Expired before first I/O, after a page, before next I/O, and after
        // classifying the final page; no partial or late report may escape.
        for (ticks, total, expected_calls) in [
            (vec![300], 1, 0),
            (vec![0, 301], 1, 1),
            (vec![0, 0, 300], 2, 1),
            (vec![0, 0, 300], 1, 1),
            (vec![0, 300], 0, 1),
        ] {
            let mut ticks = ticks.into_iter();
            let mut calls = 0;
            let result = read_snapshot_with_clock(
                &mut |start| {
                    calls += 1;
                    let items = if total == 0 {
                        vec![]
                    } else {
                        vec![item(&format!("P{start}"), "book", "x", "", "")]
                    };
                    Ok(fetched_page(total, items))
                },
                &mut || Duration::from_secs(ticks.next().unwrap()),
            );
            assert!(matches!(result, Err(ReadError::Budget("elapsed-time"))));
            assert_eq!(calls, expected_calls);
        }
    }

    #[test]
    fn reader_deadline_accepts_complete_short_pages_before_limit() {
        for total in [0, 2] {
            let report = read_snapshot_with_clock(
                &mut |start| {
                    let items = if total == 0 {
                        vec![]
                    } else {
                        vec![item(&format!("P{start}"), "book", "semantic web", "", "")]
                    };
                    Ok(fetched_page(total, items))
                },
                &mut || Duration::from_secs(300) - Duration::from_nanos(1),
            )
            .unwrap();
            assert_eq!(report.observed_item_count, total);
            assert_eq!(report.classified_items.len(), total);
            assert_eq!(report.library_version, 42);
        }
    }

    #[test]
    fn reader_rejects_future_item_revisions_before_another_page() {
        for item_type in ["book", "attachment", "note", "annotation"] {
            for invalid_start in [0, 1] {
                let mut starts = Vec::new();
                let result = read_snapshot_with(&mut |start| {
                    starts.push(start);
                    let mut observed = item(&format!("I{start}"), "book", "x", "", "");
                    observed.version = 42;
                    let mut page_items = vec![observed];
                    if start == invalid_start {
                        let mut future = item(&format!("I{}", start + 1), item_type, "x", "", "");
                        future.version = 43;
                        page_items.push(future);
                    }
                    Ok(fetched_page(4, page_items))
                });
                assert!(matches!(result, Err(ReadError::SnapshotChanged)));
                assert_eq!(
                    starts,
                    if invalid_start == 0 {
                        vec![0]
                    } else {
                        vec![0, 1]
                    }
                );
            }
        }
    }

    #[test]
    fn reader_preserves_item_revisions_within_the_library_version() {
        for zotero_version in ["9.0.6", "10.0.1"] {
            for (library_version, item_version) in
                [(0, 0), (42, 0), (42, 41), (42, 42), (u64::MAX, u64::MAX)]
            {
                let mut observed = item("A", "book", "semantic web", "", "");
                observed.version = item_version;
                let mut page = fetched_page(1, vec![observed]);
                page.library_version = library_version;
                page.zotero_version = zotero_version.into();
                let report = read_snapshot_with(&mut |_| Ok(page.clone())).unwrap();
                assert_eq!(report.library_version, library_version);
                assert_eq!(report.observed_item_count, 1);
                assert_eq!(report.classified_items.len(), 1);
                assert_eq!(report.classified_items[0].item_version, item_version);
            }
        }
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
    fn source_inventory_retains_standalone_and_nested_metadata_without_proposals() {
        let mut standalone = item("A", "attachment", "Ontology Learning", "10.1/X", "");
        standalone.version = 0;
        standalone.data.collections = vec!["COLLECTION".into()];
        standalone.data.tags = vec![ItemTag {
            tag: "Evidence".into(),
        }];
        let report = read_snapshot_with(&mut |_| {
            Ok(fetched_page(
                6,
                vec![
                    item("F", "annotation", "", "", "C"),
                    item("E", "note", "", "", ""),
                    item("D", "annotation", "", "", "A"),
                    item("C", "attachment", "", "", "B"),
                    item("B", "book", "Ontology Learning", "10.1/X", ""),
                    standalone.clone(),
                ],
            ))
        })
        .unwrap();
        let serialized = serde_json::to_value(&report).unwrap();
        assert_eq!(
            serialized["pending_source_item_keys"],
            serde_json::json!(["A", "D", "E"])
        );
        let inventory = serialized["unclassified_items"]
            .as_array()
            .expect("every excluded source must survive in the report");
        assert_eq!(
            inventory
                .iter()
                .map(|entry| entry["key"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["A", "C", "D", "E", "F"]
        );
        assert_eq!(inventory[0]["version"], 0);
        assert_eq!(inventory[0]["data"]["title"], "Ontology Learning");
        assert_eq!(
            inventory[0]["data"]["collections"],
            serde_json::json!(["COLLECTION"])
        );
        assert_eq!(
            inventory[0]["data"]["tags"],
            serde_json::json!([{"tag":"Evidence"}])
        );
        assert_eq!(inventory[4]["data"]["parentItem"], "C");
        assert_eq!(report.observed_item_count, 6);
        assert_eq!(report.classified_items.len(), 1);
        assert_eq!(report.classified_items[0].item_key, "B");
        assert_eq!(report.classified_items[0].child_item_keys, ["C"]);
        assert!(report.duplicate_candidates.is_empty());
    }

    #[test]
    fn source_inventory_keeps_orphans_cycles_and_unattached_annotations_pending() {
        let sources = vec![
            item("A", "book", "", "", ""),
            item("B", "note", "", "", "missing"),
            item("C", "attachment", "", "", "B"),
            item("D", "note", "", "", "E"),
            item("E", "attachment", "", "", "D"),
            item("F", "annotation", "", "", "F"),
            item("G", "annotation", "", "", ""),
        ];
        let forward =
            serde_json::to_value(classify_snapshot("10".into(), None, 42, sources.clone()))
                .unwrap();
        let reverse = serde_json::to_value(classify_snapshot(
            "10".into(),
            None,
            42,
            sources.into_iter().rev().collect(),
        ))
        .unwrap();
        assert_eq!(
            forward["pending_source_item_keys"],
            serde_json::json!(["B", "C", "D", "E", "F", "G"])
        );
        assert_eq!(forward, reverse);
        assert_eq!(forward["unclassified_items"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn source_inventory_distinguishes_empty_and_fully_linked_evidence() {
        for sources in [
            vec![],
            vec![item("A", "book", "", "", "")],
            vec![
                item("A", "book", "", "", ""),
                item("B", "attachment", "", "", "A"),
                item("C", "annotation", "", "", "B"),
            ],
        ] {
            let serialized =
                serde_json::to_value(classify_snapshot("10".into(), None, 42, sources)).unwrap();
            assert_eq!(
                serialized["pending_source_item_keys"],
                serde_json::json!([])
            );
            assert_eq!(
                serialized["observed_item_count"].as_u64().unwrap() as usize,
                serialized["classified_items"].as_array().unwrap().len()
                    + serialized["unclassified_items"].as_array().unwrap().len()
            );
        }
    }

    #[test]
    fn classifies_every_bibliographic_item_and_links_children() {
        let mut generation = item("B", "journalArticle", "Ontology Learning", "10.1/X", "");
        generation.data.tags.push(ItemTag {
            tag: "SHACL".into(),
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
