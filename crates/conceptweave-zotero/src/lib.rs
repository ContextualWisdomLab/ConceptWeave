#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! Deterministic, read-only classification of a Zotero library snapshot.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::Duration;

/// Classification rule revision recorded in every report.
pub const RULE_REVISION: &str = "ontology-research-v1";

const PAGE_LIMIT: usize = 100;
const MAX_PAGE_BYTES: u64 = 8 * 1024 * 1024;
const LOCAL_API: &str = "http://127.0.0.1:23119/api/users/0/items";

/// A Zotero item returned by the Local API.
#[derive(Debug, Clone, Deserialize)]
pub struct ZoteroItem {
    /// Stable item key.
    pub key: String,
    /// Item revision.
    pub version: u64,
    /// Item metadata.
    pub data: ItemData,
}

/// Metadata used by the classifier.
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
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

/// Evidence for a deterministic proposed disposition.
#[derive(Debug, Serialize)]
pub struct ClassificationEvidence {
    /// Metadata fields whose values matched.
    pub fields: Vec<&'static str>,
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
    /// A later page did not belong to the first page's snapshot.
    SnapshotChanged,
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
            Self::SnapshotChanged => write!(formatter, "Zotero library changed during the read"),
            Self::Json(error) => write!(formatter, "local API returned invalid JSON: {error}"),
            Self::Body(error) => write!(formatter, "local API response body failed: {error}"),
        }
    }
}

impl std::error::Error for ReadError {}

/// Reads every Zotero item from one stable Local API library version.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn read_local_snapshot() -> Result<ClassificationReport, ReadError> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .timeout_connect(Some(Duration::from_secs(2)))
        .timeout_recv_response(Some(Duration::from_secs(10)))
        .timeout_recv_body(Some(Duration::from_secs(10)))
        .max_redirects(0)
        .build();
    let agent = ureq::Agent::new_with_config(config);
    let mut items = Vec::new();
    let mut expected = None;
    let mut library_version = None;
    let mut zotero_version = None;
    let mut server_id = None;
    let mut metadata_initialized = false;

    loop {
        let url = format!(
            "{LOCAL_API}?format=json&include=data&limit={PAGE_LIMIT}&start={}",
            items.len()
        );
        let mut response = agent
            .get(&url)
            .call()
            .map_err(|error| ReadError::Http(error.to_string()))?;
        let headers = response.headers();
        let page_total = header_u64(headers, "Total-Results")? as usize;
        let page_version = header_u64(headers, "Last-Modified-Version")?;
        let page_zotero = header_string(headers, "X-Zotero-Version")?;
        let page_server = optional_header(headers, "Zotero-Server-ID");

        if metadata_initialized {
            if expected != Some(page_total)
                || library_version != Some(page_version)
                || zotero_version.as_ref() != Some(&page_zotero)
                || server_id != page_server
            {
                return Err(ReadError::SnapshotChanged);
            }
        } else {
            expected = Some(page_total);
            library_version = Some(page_version);
            zotero_version = Some(page_zotero);
            server_id = page_server;
            metadata_initialized = true;
        }

        let body = response
            .body_mut()
            .with_config()
            .limit(MAX_PAGE_BYTES)
            .read_to_string()
            .map_err(|error| ReadError::Body(error.to_string()))?;
        let page: Vec<ZoteroItem> = serde_json::from_str(&body).map_err(ReadError::Json)?;
        if page.is_empty() && items.len() < page_total {
            return Err(ReadError::SnapshotChanged);
        }
        items.extend(page);
        if items.len() > page_total {
            return Err(ReadError::SnapshotChanged);
        }
        if items.len() == page_total {
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

    Ok(classify_snapshot(
        zotero_version.ok_or(ReadError::Header("X-Zotero-Version"))?,
        server_id,
        library_version.ok_or(ReadError::Header("Last-Modified-Version"))?,
        items,
    ))
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
    let classified_items = bibliographic
        .into_iter()
        .map(|item| classify_item(item, children.get(&item.key).cloned().unwrap_or_default()))
        .collect();

    ClassificationReport {
        zotero_version,
        server_id,
        library_version,
        rule_revision: RULE_REVISION,
        observed_item_count: items.len(),
        classified_items,
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
    let title = item.data.title.to_lowercase();
    let abstract_note = item.data.abstract_note.to_lowercase();
    let tags = item
        .data
        .tags
        .iter()
        .map(|tag| tag.tag.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    let fields = [
        ("title", title.as_str()),
        ("abstract_note", abstract_note.as_str()),
        ("tags", tags.as_str()),
    ];
    let rules = [
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
        (
            Disposition::AdjacentEvidence,
            &[
                "ontology",
                "semantic web",
                "knowledge graph",
                "rdf",
                "owl",
                "skos",
            ][..],
        ),
    ];
    let mut disposition = Disposition::NeedsStewardReview;
    let mut matched_fields = BTreeSet::new();
    let mut matched_phrases = BTreeSet::new();
    for (candidate, phrases) in rules {
        for (field, value) in fields {
            for phrase in phrases {
                if contains_phrase(value, phrase) {
                    disposition = candidate;
                    matched_fields.insert(field);
                    matched_phrases.insert(*phrase);
                }
            }
        }
        if disposition != Disposition::NeedsStewardReview {
            break;
        }
    }
    ClassifiedItem {
        item_key: item.key.clone(),
        item_version: item.version,
        item_type: item.data.item_type.clone(),
        title: item.data.title.clone(),
        collection_keys: item.data.collections.clone(),
        tags: item.data.tags.iter().map(|tag| tag.tag.clone()).collect(),
        proposed_disposition: disposition,
        evidence: ClassificationEvidence {
            fields: matched_fields.into_iter().collect(),
            matched_phrases: matched_phrases.into_iter().collect(),
        },
        child_item_keys,
        model_receipt: None,
    }
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
                generation,
                item("A", "book", "Other", "", ""),
            ],
        );
        assert_eq!(report.observed_item_count, 3);
        assert_eq!(report.classified_items.len(), 2);
        assert_eq!(
            report.classified_items[0].proposed_disposition,
            Disposition::NeedsStewardReview
        );
        assert_eq!(
            report.classified_items[1].proposed_disposition,
            Disposition::Generation
        );
        assert_eq!(report.classified_items[1].child_item_keys, ["C"]);
        assert_eq!(report.classified_items[1].evidence.fields, ["title"]);
    }

    #[test]
    fn priority_and_all_rule_families_are_deterministic() {
        let cases = [
            (
                "ontology matching and ontology learning",
                Disposition::AlignmentVersioning,
            ),
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
        assert!(!contains_phrase("knowledge", "owl"));
        assert!(!contains_phrase("growl", "owl"));
        assert!(contains_phrase("owl-based", "owl"));
        assert!(contains_phrase("uses owl", "owl"));
        assert!(contains_phrase("owl", "owl"));
    }

    #[test]
    fn duplicate_candidates_are_reversible_and_normalized() {
        let report = classify_snapshot(
            "10".into(),
            None,
            1,
            vec![
                item("A", "book", "OWL: Overview", "doi:10.1/X", ""),
                item("B", "book", "owl overview", "https://doi.org/10.1/x", ""),
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
    fn empty_identities_do_not_form_duplicate_groups() {
        assert_eq!(normalize_doi("  "), None);
        assert_eq!(normalize_title("---"), None);
        assert_eq!(normalize_doi("http://doi.org/A"), Some("a".into()));
        assert_eq!(normalize_doi("https://doi.org/B"), Some("b".into()));
        assert_eq!(normalize_doi("C"), Some("c".into()));
        assert_eq!(normalize_title(" A---B "), Some("a b".into()));
        assert_eq!(normalize_title(" A--- "), Some("a".into()));
        let untitled = item("Z", "book", "", "", "");
        assert!(duplicate_candidates(&[&untitled]).is_empty());

        let report = classify_snapshot(
            "10".into(),
            None,
            1,
            vec![
                item("A", "book", "Only", "", ""),
                item("B", "note", "Ignored", "", ""),
                item("C", "annotation", "Ignored", "", ""),
                item("D", "book", "Child", "", "A"),
            ],
        );
        assert!(report.duplicate_candidates.is_empty());
        assert_eq!(report.classified_items.len(), 1);
    }

    #[test]
    fn read_errors_are_actionable() {
        assert!(ReadError::Header("x").to_string().contains('x'));
        assert!(ReadError::SnapshotChanged.to_string().contains("changed"));
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
