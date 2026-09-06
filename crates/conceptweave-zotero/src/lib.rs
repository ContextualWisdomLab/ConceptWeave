#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! Deterministic, read-only classification of a Zotero library snapshot.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::Read;
use std::time::{Duration, Instant};

/// Classification rule revision recorded in every report.
pub const RULE_REVISION: &str = "ontology-research-v2";

const SUPPORTED_API_VERSION: u64 = 3;
const SNAPSHOT_DIGEST_DOMAIN: &str = "conceptweave-zotero-snapshot-v2";
const SUPPORTED_API_VERSION_HEADER: &str = "3";
const PAGE_LIMIT: usize = 100;
const MAX_PAGE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_SNAPSHOT_ITEMS: usize = 50_000;
const MAX_SNAPSHOT_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SNAPSHOT_ELAPSED: Duration = Duration::from_secs(300);
const MAX_ITEM_RESPONSE_BYTES: u64 = 1024 * 1024;
const MAX_AUTH_RESPONSE_BYTES: u64 = 512;
const MAX_AUTH_APP_NAME_BYTES: usize = 128;
const MAX_RETRY_AFTER_SECONDS: u64 = 86_400;
const LOCAL_API: &str = "http://127.0.0.1:23119/api/users/0/items";
const LOCAL_API_ROOT: &str = "http://127.0.0.1:23119";

#[cfg(test)]
static TEST_LOCAL_API: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// A Zotero item returned by the Local API.
#[derive(Debug, Clone, Serialize)]
pub struct ZoteroItem {
    /// Stable item key.
    pub key: String,
    /// Item revision.
    pub version: u64,
    /// Item metadata.
    pub data: ItemData,
    /// Complete original JSON object, captured automatically during deserialization.
    ///
    /// Offline callers constructing synthetic typed items use `None`. The digest
    /// binds this source value together with the actual typed classifier input,
    /// so later projection changes also invalidate the receipt. It retains omitted fields,
    /// unknown metadata, nested objects, and array order exactly as observed.
    #[serde(skip)]
    pub source_record: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for ZoteroItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ItemProjection {
            key: String,
            version: u64,
            data: ItemData,
        }

        let source_record = serde_json::Value::deserialize(deserializer)?;
        let projection =
            ItemProjection::deserialize(&source_record).map_err(serde::de::Error::custom)?;
        Ok(Self {
            key: projection.key,
            version: projection.version,
            data: projection.data,
            source_record: Some(source_record),
        })
    }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
    /// Nonempty abstract retained only when a steward must classify the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_abstract_note: Option<String>,
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
    /// Required v2 identity of proposals, retained metadata and pending sources.
    pub proposal_digest: String,
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
    /// Verified identity of the proposals and complete retained source scope.
    pub proposal_digest: String,
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
    /// Exact Zotero version reviewed for write capability.
    pub zotero_version: String,
    /// Exact reviewed library revision.
    pub library_version: u64,
    /// Exact reviewed classifier revision.
    pub rule_revision: String,
    /// Exact reviewed raw-snapshot digest.
    pub snapshot_digest: String,
    /// Required identity of reviewed proposals, unclassified metadata, and pending keys.
    pub proposal_digest: String,
    /// Exact item-key/item-version coordinates reviewed by the steward.
    pub snapshot_items: Vec<SnapshotItemRevision>,
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
///
/// Execution-critical fields are read-only outside this crate, so callers cannot
/// mutate a verified plan before passing it to the execution boundary.
///
/// ```compile_fail
/// use conceptweave_zotero::{ClassificationWritePlan, WriteMode};
/// fn forge(plan: &mut ClassificationWritePlan) {
///     plan.mode = WriteMode::Execute;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationWritePlan {
    /// Requested write behavior; dry-run is the default.
    mode: WriteMode,
    /// Opaque review receipt identifier.
    review_id: String,
    /// Opaque governance authority receipt.
    authority_receipt: String,
    /// Exact Local API server identity.
    server_id: Option<String>,
    /// Exact Zotero version used to establish execute eligibility.
    zotero_version: String,
    /// Exact library-version precondition.
    library_version: u64,
    /// Exact classifier revision.
    rule_revision: String,
    /// Exact raw-snapshot digest.
    snapshot_digest: String,
    /// Content identity retained from the independently verified review.
    proposal_digest: String,
    /// Deterministically ordered item operations.
    operations: Vec<ClassificationWriteOperation>,
    /// Classification writes never delete source records or attachments.
    source_records_preserved: bool,
}

impl ClassificationWritePlan {
    /// Returns the requested dry-run or execute behavior.
    pub const fn mode(&self) -> WriteMode {
        self.mode
    }

    /// Returns the exact library revision used by every initial preflight.
    pub const fn library_version(&self) -> u64 {
        self.library_version
    }

    /// Returns the deterministic reviewed operations.
    pub fn operations(&self) -> &[ClassificationWriteOperation] {
        &self.operations
    }

    /// Confirms that the plan contains metadata changes only.
    pub const fn source_records_preserved(&self) -> bool {
        self.source_records_preserved
    }
}

/// Complete item state observed at the Local API write boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationItemState {
    /// Local API server identity that produced this state.
    pub server_id: String,
    /// Library revision that produced this state.
    pub library_version: u64,
    /// Stable Zotero item key.
    pub item_key: String,
    /// Current optimistic item revision.
    pub item_version: u64,
    /// Complete collection state.
    pub collection_keys: Vec<String>,
    /// Complete typed-tag state.
    pub tags: Vec<ItemTag>,
}

/// Secret-free failure returned by the authenticated Zotero 10 Local API adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoteroTransportError {
    /// The caller did not provide a usable API key and server identity.
    InvalidCredentials,
    /// Zotero did not recognize the caller's authorization request.
    Unauthorized,
    /// The user denied the one-time authorization request.
    Denied,
    /// Zotero rate-limited authorization; a safe delta may be exposed.
    RateLimited {
        /// Retry delay in seconds when it was a bounded integer delta.
        retry_after_seconds: Option<u64>,
    },
    /// A write key is no longer authorized and must be replaced by the caller.
    ReauthorizationRequired,
    /// A write precondition no longer matches the Zotero library.
    StalePrecondition,
    /// The item key is not an official eight-character Zotero object key.
    InvalidItemKey,
    /// The Local API rejected the request or could not be reached.
    RequestFailed,
    /// The response came from a different Zotero database.
    ServerMismatch,
    /// The response headers or bounded JSON body were invalid.
    InvalidResponse,
}

/// One user-approved Zotero 10 Local API authorization.
///
/// The key remains private and this type deliberately implements neither
/// [`Debug`] nor [`Serialize`]. Authorization performs exactly one request;
/// callers decide whether and when another user prompt is appropriate.
pub struct Zotero10LocalAuthorization {
    api_key: String,
    server_id: String,
    remembered: bool,
}

impl Zotero10LocalAuthorization {
    /// Requests authorization from Zotero's fixed loopback endpoint once.
    pub fn request(
        app_name: &str,
        server_id: impl Into<String>,
    ) -> Result<Self, ZoteroTransportError> {
        Self::request_from(app_name, server_id.into(), LOCAL_API_ROOT)
    }

    #[cfg(test)]
    fn request_with_base(
        app_name: &str,
        server_id: impl Into<String>,
        base: String,
    ) -> Result<Self, ZoteroTransportError> {
        Self::request_from(app_name, server_id.into(), &base)
    }

    fn request_from(
        app_name: &str,
        server_id: String,
        base: &str,
    ) -> Result<Self, ZoteroTransportError> {
        if app_name.trim().is_empty()
            || app_name.len() > MAX_AUTH_APP_NAME_BYTES
            || server_id.trim().is_empty()
        {
            return Err(ZoteroTransportError::InvalidCredentials);
        }
        let url = format!("{base}/api/local/authorize");
        let mut response = local_agent()
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Zotero-Server-ID", &server_id)
            .send(serde_json::json!({ "appName": app_name }).to_string())
            .map_err(|_| ZoteroTransportError::RequestFailed)?;
        verify_server_id(response.headers(), &server_id)?;
        match response.status().as_u16() {
            401 => return Err(ZoteroTransportError::Unauthorized),
            403 => {
                #[derive(Deserialize)]
                struct DenialResponse {
                    denied: bool,
                }
                let body = bounded_body_with_limit(&mut response, MAX_AUTH_RESPONSE_BYTES)?;
                let denial: DenialResponse = serde_json::from_str(&body)
                    .map_err(|_| ZoteroTransportError::InvalidResponse)?;
                return if denial.denied {
                    Err(ZoteroTransportError::Denied)
                } else {
                    Err(ZoteroTransportError::InvalidResponse)
                };
            }
            429 => {
                return Err(ZoteroTransportError::RateLimited {
                    retry_after_seconds: retry_after_seconds(response.headers()),
                });
            }
            200 => {}
            _ => return Err(ZoteroTransportError::RequestFailed),
        }
        #[derive(Deserialize)]
        struct AuthorizationResponse {
            key: String,
            remember: bool,
        }
        let body = bounded_body_with_limit(&mut response, MAX_AUTH_RESPONSE_BYTES)?;
        let authorization: AuthorizationResponse =
            serde_json::from_str(&body).map_err(|_| ZoteroTransportError::InvalidResponse)?;
        if !is_valid_local_api_key(&authorization.key) {
            return Err(ZoteroTransportError::InvalidResponse);
        }
        Ok(Self {
            api_key: authorization.key,
            server_id,
            remembered: authorization.remember,
        })
    }

    /// Reports whether Zotero agreed to remember this authorization.
    pub const fn remembered(&self) -> bool {
        self.remembered
    }

    /// Consumes the authorization and creates the existing write adapter.
    pub fn into_adapter(self) -> Zotero10LocalAdapter {
        Zotero10LocalAdapter::build(self.api_key, self.server_id, LOCAL_API.to_owned())
            .expect("validated authorization always builds an adapter")
    }
}

/// Minimal authenticated adapter for Zotero 10+ Local API item metadata writes.
///
/// Credentials remain private and this type deliberately implements neither
/// [`Debug`] nor [`Serialize`].
pub struct Zotero10LocalAdapter {
    api_key: String,
    server_id: String,
    base: String,
    agent: ureq::Agent,
}

impl Zotero10LocalAdapter {
    /// Creates an adapter pinned to Zotero's loopback production endpoint.
    pub fn new(
        api_key: impl Into<String>,
        server_id: impl Into<String>,
    ) -> Result<Self, ZoteroTransportError> {
        Self::build(api_key.into(), server_id.into(), LOCAL_API.to_owned())
    }

    #[cfg(test)]
    fn new_with_base(
        api_key: impl Into<String>,
        server_id: impl Into<String>,
        base: String,
    ) -> Result<Self, ZoteroTransportError> {
        Self::build(api_key.into(), server_id.into(), base)
    }

    fn build(
        api_key: String,
        server_id: String,
        base: String,
    ) -> Result<Self, ZoteroTransportError> {
        if !is_valid_local_api_key(&api_key) || server_id.trim().is_empty() {
            return Err(ZoteroTransportError::InvalidCredentials);
        }
        Ok(Self {
            api_key,
            server_id,
            base,
            agent: local_agent(),
        })
    }

    /// Reads one item's current collection, tag, and version coordinates.
    pub fn get_item(
        &self,
        item_key: &str,
    ) -> Result<ClassificationItemState, ZoteroTransportError> {
        validate_item_key(item_key)?;
        let before = self.library_version()?;
        let url = format!("{}/{item_key}?format=json&include=data", self.base);
        let response = self
            .agent
            .get(&url)
            .header("Zotero-API-Version", SUPPORTED_API_VERSION_HEADER)
            .header("Zotero-Server-ID", &self.server_id)
            .call()
            .map_err(|_| ZoteroTransportError::RequestFailed)?;
        self.verify_server(response.headers())?;
        if response.status() != ureq::http::StatusCode::OK {
            return Err(ZoteroTransportError::RequestFailed);
        }
        let item = self.read_item(response, item_key)?;
        let after = self.library_version()?;
        if before != after {
            return Err(ZoteroTransportError::InvalidResponse);
        }
        Ok(ClassificationItemState {
            server_id: self.server_id.clone(),
            library_version: before,
            item_key: item.key,
            item_version: item.version,
            collection_keys: item.data.collections,
            tags: item.data.tags,
        })
    }

    /// Atomically replaces one item's complete collection and tag arrays.
    pub fn write_item(
        &self,
        request: &ClassificationWriteRequest,
    ) -> Result<ClassificationItemState, ZoteroTransportError> {
        validate_item_key(&request.item_key)?;
        if request.server_id != self.server_id {
            return Err(ZoteroTransportError::ServerMismatch);
        }
        #[derive(Serialize)]
        struct Write<'a> {
            key: &'a str,
            version: u64,
            collections: &'a [String],
            tags: &'a [ItemTag],
        }
        let body = serde_json::json!([Write {
            key: &request.item_key,
            version: request.item_version,
            collections: &request.collection_keys,
            tags: &request.tags,
        }])
        .to_string();
        let mut response = self
            .agent
            .post(&self.base)
            .header("Zotero-API-Version", SUPPORTED_API_VERSION_HEADER)
            .header("Zotero-API-Key", &self.api_key)
            .header("Zotero-Server-ID", &self.server_id)
            .header(
                "If-Unmodified-Since-Version",
                &request.library_version.to_string(),
            )
            .header("Content-Type", "application/json")
            .send(body)
            .map_err(|_| ZoteroTransportError::RequestFailed)?;
        self.verify_server(response.headers())?;
        match response.status().as_u16() {
            401 => return Err(ZoteroTransportError::ReauthorizationRequired),
            412 => return Err(ZoteroTransportError::StalePrecondition),
            _ => {}
        }
        if response.status() != ureq::http::StatusCode::OK {
            return Err(ZoteroTransportError::RequestFailed);
        }
        let library_version = version_header(response.headers())?;
        #[derive(Deserialize)]
        struct WriteResponse {
            successful: BTreeMap<String, ZoteroItem>,
        }
        let body = bounded_body(&mut response)?;
        let mut written: WriteResponse =
            serde_json::from_str(&body).map_err(|_| ZoteroTransportError::InvalidResponse)?;
        let item = written
            .successful
            .remove("0")
            .filter(|_| written.successful.is_empty())
            .ok_or(ZoteroTransportError::InvalidResponse)?;
        if item.key != request.item_key
            || item.version <= request.item_version
            || item.version != library_version
            || item.data.collections != request.collection_keys
            || item.data.tags != request.tags
        {
            return Err(ZoteroTransportError::InvalidResponse);
        }
        Ok(ClassificationItemState {
            server_id: self.server_id.clone(),
            library_version,
            item_key: item.key,
            item_version: item.version,
            collection_keys: item.data.collections,
            tags: item.data.tags,
        })
    }

    fn library_version(&self) -> Result<u64, ZoteroTransportError> {
        let url = format!("{}?format=versions&limit=1", self.base);
        let mut response = self
            .agent
            .get(&url)
            .header("Zotero-API-Version", SUPPORTED_API_VERSION_HEADER)
            .header("Zotero-Server-ID", &self.server_id)
            .call()
            .map_err(|_| ZoteroTransportError::RequestFailed)?;
        self.verify_server(response.headers())?;
        if response.status() != ureq::http::StatusCode::OK {
            return Err(ZoteroTransportError::RequestFailed);
        }
        let version = version_header(response.headers())?;
        bounded_body(&mut response)?;
        Ok(version)
    }

    fn read_item(
        &self,
        mut response: ureq::http::Response<ureq::Body>,
        requested_key: &str,
    ) -> Result<ZoteroItem, ZoteroTransportError> {
        let object_version = version_header(response.headers())?;
        let body = bounded_body(&mut response)?;
        let item: ZoteroItem =
            serde_json::from_str(&body).map_err(|_| ZoteroTransportError::InvalidResponse)?;
        if item.key != requested_key || item.version != object_version {
            return Err(ZoteroTransportError::InvalidResponse);
        }
        Ok(item)
    }

    fn verify_server(&self, headers: &ureq::http::HeaderMap) -> Result<(), ZoteroTransportError> {
        verify_server_id(headers, &self.server_id)
    }
}

fn local_agent() -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .proxy(None)
        .timeout_global(Some(Duration::from_secs(30)))
        .timeout_connect(Some(Duration::from_secs(2)))
        .timeout_recv_response(Some(Duration::from_secs(10)))
        .timeout_recv_body(Some(Duration::from_secs(10)))
        .http_status_as_error(false)
        .max_redirects(0)
        .build();
    ureq::Agent::new_with_config(config)
}

fn verify_server_id(
    headers: &ureq::http::HeaderMap,
    expected: &str,
) -> Result<(), ZoteroTransportError> {
    let server = headers
        .get("Zotero-Server-ID")
        .and_then(|value| value.to_str().ok())
        .ok_or(ZoteroTransportError::InvalidResponse)?;
    if server == expected {
        Ok(())
    } else {
        Err(ZoteroTransportError::ServerMismatch)
    }
}

fn retry_after_seconds(headers: &ureq::http::HeaderMap) -> Option<u64> {
    headers
        .get("Retry-After")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
        .filter(|seconds| *seconds <= MAX_RETRY_AFTER_SECONDS)
}

fn is_valid_local_api_key(api_key: &str) -> bool {
    api_key.len() == 32 && api_key.bytes().all(|byte| byte.is_ascii_graphic())
}

fn version_header(headers: &ureq::http::HeaderMap) -> Result<u64, ZoteroTransportError> {
    headers
        .get("Last-Modified-Version")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
        .ok_or(ZoteroTransportError::InvalidResponse)
}

fn bounded_body(
    response: &mut ureq::http::Response<ureq::Body>,
) -> Result<String, ZoteroTransportError> {
    bounded_body_with_limit(response, MAX_ITEM_RESPONSE_BYTES)
}

fn bounded_body_with_limit(
    response: &mut ureq::http::Response<ureq::Body>,
    limit: u64,
) -> Result<String, ZoteroTransportError> {
    read_bounded_response_text(response, limit).map_err(|_| ZoteroTransportError::InvalidResponse)
}

fn validate_item_key(item_key: &str) -> Result<(), ZoteroTransportError> {
    const ALPHABET: &[u8] = b"23456789ABCDEFGHIJKLMNPQRSTUVWXYZ";
    if item_key.len() == 8 && item_key.bytes().all(|byte| ALPHABET.contains(&byte)) {
        Ok(())
    } else {
        Err(ZoteroTransportError::InvalidItemKey)
    }
}

/// One conditional complete-state replacement passed to an authenticated adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationWriteRequest {
    /// Expected Local API server identity.
    pub server_id: String,
    /// Current library revision precondition.
    pub library_version: u64,
    /// Stable Zotero item key.
    pub item_key: String,
    /// Current item revision precondition.
    pub item_version: u64,
    /// Complete collection replacement.
    pub collection_keys: Vec<String>,
    /// Complete typed-tag replacement.
    pub tags: Vec<ItemTag>,
}

/// Mutable inverse-write audit data; this value alone does not prove authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationRollbackOperation {
    /// Local API server identity that produced the state to undo.
    pub server_id: String,
    /// Stable Zotero item key.
    pub item_key: String,
    /// Post-write item revision required by a rollback adapter.
    pub item_version: u64,
    /// Complete post-write collection state that must still be current.
    pub expected_collection_keys: Vec<String>,
    /// Complete post-write typed-tag state that must still be current.
    pub expected_tags: Vec<ItemTag>,
    /// Complete collection state to restore.
    pub collection_keys: Vec<String>,
    /// Complete typed-tag state to restore.
    pub tags: Vec<ItemTag>,
}

/// Observable result of one rollback execution attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationRollbackOutcome {
    /// Every inverse write was verified.
    Restored,
    /// No inverse write began because complete preflight could not be proven.
    PreflightFailure,
    /// An inverse write or its response failed after preflight.
    PartialFailure,
}

/// Legacy state vocabulary; metadata-only reconciliation emits only `Indeterminate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationRollbackState {
    /// The restoration metadata is present at a newer item revision.
    Restored,
    /// The expected post-write metadata and item revision remain unchanged.
    Unchanged,
    /// The observed state proves neither safe outcome.
    Indeterminate,
}

/// Secret-free evidence from one delayed, read-only rollback reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationRollbackReconciliationReceipt {
    /// Causal state remains indeterminate; metadata alone cannot settle the write.
    pub state: ClassificationRollbackState,
    /// Complete operation under reconciliation.
    pub operation: ClassificationRollbackOperation,
    /// Successfully observed state, absent when validation or reading failed.
    pub observed_state: Option<ClassificationItemState>,
    /// Legacy audit slot; this read-only observer never grants a retry operation.
    pub retry_operation: Option<ClassificationRollbackOperation>,
}

/// Secret-free evidence for restored, failed, indeterminate, and pending inverse writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationRollbackReceipt {
    /// Overall rollback outcome.
    pub outcome: ClassificationRollbackOutcome,
    /// Items whose restored state was verified, in receipt order.
    pub restored_item_keys: Vec<String>,
    /// First item whose preflight, write, or response failed.
    pub failed_item_key: Option<String>,
    /// Item whose state could not be proven after an unverifiable response.
    pub indeterminate_item_key: Option<String>,
    /// Complete operation retained for manual reconciliation of indeterminate state.
    pub indeterminate_operation: Option<ClassificationRollbackOperation>,
    /// Exact submitted inverse request; observations do not authorize its retry.
    pub indeterminate_request: Option<ClassificationWriteRequest>,
    /// Complete readback after failure, retained without causal completion claims.
    pub reconciliation_observation: Option<ClassificationItemState>,
    /// Items whose inverse write was not attempted.
    pub not_attempted_item_keys: Vec<String>,
    /// Untouched operations only; this audit list does not grant retry authority.
    pub remaining_operations: Vec<ClassificationRollbackOperation>,
}

/// Observable result of a write-plan execution attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationWriteOutcome {
    /// The plan intentionally made no Local API calls.
    DryRun,
    /// Every requested item replacement was verified.
    Applied,
    /// No write began because a complete preflight could not be proven.
    PreflightFailure,
    /// A write or its response failed after preflight.
    PartialFailure,
}

/// Secret-free evidence for applied, failed, pending, and reversible writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationWriteReceipt {
    /// Opaque review receipt identifier bound to this attempt.
    pub review_id: String,
    /// Opaque governance authority receipt bound to this attempt.
    pub authority_receipt: String,
    /// Exact Local API server identity bound to this attempt.
    pub server_id: Option<String>,
    /// Exact Zotero version bound to this attempt.
    pub zotero_version: String,
    /// Exact library-version precondition bound to this attempt.
    pub library_version: u64,
    /// Exact classifier revision bound to this attempt.
    pub rule_revision: String,
    /// Exact raw-snapshot digest bound to this attempt.
    pub snapshot_digest: String,
    /// Verified proposal and retained-source identity bound to this attempt.
    pub proposal_digest: String,
    /// Overall execution outcome.
    pub outcome: ClassificationWriteOutcome,
    /// Items whose post-write state was verified, in application order.
    pub applied_item_keys: Vec<String>,
    /// First item whose preflight, write, or response failed.
    pub failed_item_key: Option<String>,
    /// Item whose state could not be proven after an unverifiable write response.
    pub indeterminate_item_key: Option<String>,
    /// Exact submitted request whose completion is unknown; audit evidence, not retry authority.
    pub indeterminate_request: Option<ClassificationWriteRequest>,
    /// Subsequent observation, if available; it cannot prove causal completion.
    pub reconciliation_observation: Option<ClassificationItemState>,
    /// Items whose write was not attempted.
    pub not_attempted_item_keys: Vec<String>,
    /// Verified inverse operations in safe reverse application order.
    pub rollback_operations: Vec<ClassificationRollbackOperation>,
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

/// One editable local steward decision without duplicated bibliographic text.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StewardReviewDecision {
    /// Stable Zotero item key used to join the sensitive classification report.
    pub item_key: String,
    /// Exact item revision observed in the classified snapshot.
    pub item_version: u64,
    /// Deterministic proposal supplied for comparison, never as approval.
    pub proposed_disposition: Disposition,
    /// Deterministic abstention reason when the proposal requires review.
    pub abstention_reason: Option<AbstentionReason>,
    /// Steward decision to fill; abstention is rejected by completion evaluation.
    pub reviewed_disposition: Option<Disposition>,
}

/// Snapshot-bound local worksheet for one decision per bibliographic item.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StewardReviewWorksheet {
    /// Zotero library revision observed with the source snapshot.
    pub library_version: u64,
    /// Classifier revision that produced the proposals.
    pub rule_revision: String,
    /// Canonical content digest of the complete raw snapshot.
    pub snapshot_digest: String,
    /// Versioned identity of proposals, review context and retained source scope.
    /// Older worksheets require regeneration; this binding is not approval.
    pub proposal_digest: String,
    /// Complete parent and child item-revision coordinates.
    pub snapshot_items: Vec<SnapshotItemRevision>,
    /// Deterministically ordered editable decisions for bibliographic items.
    pub decisions: Vec<StewardReviewDecision>,
}

/// A classification report cannot safely produce a review worksheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorksheetError {
    /// Report identity or coverage is incomplete, duplicated, or inconsistent.
    InvalidReport,
}

impl fmt::Display for WorksheetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("classification report is invalid for steward review")
    }
}

impl std::error::Error for WorksheetError {}

/// Builds a complete local worksheet without copying titles, abstracts, or evidence.
///
/// Complete inventory validation precedes projection. Valid unresolved sources
/// may be reviewed, but worksheet creation never proves completed review.
pub fn build_steward_review_worksheet(
    report: &ClassificationReport,
) -> Result<StewardReviewWorksheet, WorksheetError> {
    validate_classification_report(report).map_err(|_| WorksheetError::InvalidReport)?;
    if report.rule_revision.trim().is_empty() || report.snapshot_digest.trim().is_empty() {
        return Err(WorksheetError::InvalidReport);
    }
    let mut decisions = Vec::with_capacity(report.classified_items.len());
    for item in &report.classified_items {
        if (item.proposed_disposition == Disposition::NeedsStewardReview)
            != item.abstention_reason.is_some()
        {
            return Err(WorksheetError::InvalidReport);
        }
        decisions.push(StewardReviewDecision {
            item_key: item.item_key.clone(),
            item_version: item.item_version,
            proposed_disposition: item.proposed_disposition,
            abstention_reason: item.abstention_reason,
            reviewed_disposition: None,
        });
    }
    decisions.sort_by(|left, right| left.item_key.cmp(&right.item_key));

    Ok(StewardReviewWorksheet {
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(report),
        snapshot_items: report.snapshot_items.clone(),
        decisions,
    })
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

/// Governance receipt binding a steward approval to exact input and proposals.
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
    /// Digest of the actual proposal records reviewed and verified by the caller.
    pub proposal_digest: String,
    /// Complete sorted item-revision identity of the reviewed report.
    pub snapshot_items: Vec<SnapshotItemRevision>,
}

/// Converts a fully decided local worksheet into the input for approval verification.
///
/// The supplied approval must already bind the current complete proposal records.
/// This function validates that binding without creating or renewing authority.
pub fn reviewed_golden_set_from_worksheet(
    report: &ClassificationReport,
    worksheet: &StewardReviewWorksheet,
    approval: GoldenSetApproval,
) -> Result<ReviewedGoldenSet, EvaluationError> {
    let expected =
        build_steward_review_worksheet(report).map_err(|_| EvaluationError::InvalidReview)?;
    if worksheet.decisions.len() != expected.decisions.len() {
        return Err(EvaluationError::IncompleteReview);
    }
    if approval.receipt_id.trim().is_empty()
        || approval.reviewer_subject.trim().is_empty()
        || approval.proposal_digest.trim().is_empty()
        || worksheet.rule_revision.trim().is_empty()
        || worksheet.snapshot_digest.trim().is_empty()
        || worksheet.proposal_digest.trim().is_empty()
    {
        return Err(EvaluationError::InvalidReview);
    }
    if approval.library_version != worksheet.library_version
        || approval.rule_revision != worksheet.rule_revision
        || approval.snapshot_digest != worksheet.snapshot_digest
        || approval.proposal_digest != classification_proposal_digest(report)
        || approval.snapshot_items != worksheet.snapshot_items
    {
        return Err(EvaluationError::SnapshotMismatch);
    }
    if worksheet.library_version != expected.library_version
        || worksheet.rule_revision != expected.rule_revision
        || worksheet.snapshot_digest != expected.snapshot_digest
        || worksheet.proposal_digest != expected.proposal_digest
        || worksheet.snapshot_items != expected.snapshot_items
    {
        return Err(EvaluationError::SnapshotMismatch);
    }

    let mut labels = Vec::with_capacity(worksheet.decisions.len());
    for (decision, expected_decision) in worksheet.decisions.iter().zip(expected.decisions) {
        if decision.item_key != expected_decision.item_key
            || decision.item_version != expected_decision.item_version
            || decision.proposed_disposition != expected_decision.proposed_disposition
            || decision.abstention_reason != expected_decision.abstention_reason
        {
            return Err(EvaluationError::InvalidReview);
        }
        let expected_disposition = decision
            .reviewed_disposition
            .ok_or(EvaluationError::IncompleteReview)?;
        if expected_disposition == Disposition::NeedsStewardReview {
            return Err(EvaluationError::InvalidExpectedDisposition);
        }
        labels.push(GoldenLabel::new(
            decision.item_key.clone(),
            expected_disposition,
        ));
    }
    if labels.is_empty() {
        return Err(EvaluationError::IncompleteReview);
    }

    Ok(ReviewedGoldenSet { approval, labels })
}

/// Computes the canonical content identity verified by a golden-set approval.
pub fn classification_snapshot_digest(report: &ClassificationReport) -> String {
    report.snapshot_digest.clone()
}

/// Computes the versioned SHA-256 identity of the report's current proposals.
///
/// Every proposal field is covered, including its prediction, evidence, and item
/// revision, plus unclassified metadata and pending source identities. This is
/// not a full-text backup. Records are sorted by item key and revision so ordering does
/// not change their identity. No second source snapshot is stored. Governance
/// must bind this value when issuing an approval; recomputing it alone grants no
/// authority. Evaluation recomputes it rather than trusting report metadata.
pub fn classification_proposal_digest(report: &ClassificationReport) -> String {
    let mut proposals = report.classified_items.iter().collect::<Vec<_>>();
    proposals.sort_by_key(|item| (&item.item_key, item.item_version));
    let mut source_items = report.unclassified_items.iter().collect::<Vec<_>>();
    source_items.sort_by_key(|item| (&item.key, item.version));
    let mut pending_keys = report.pending_source_item_keys.iter().collect::<Vec<_>>();
    pending_keys.sort();
    let proposal_bytes = serde_json::to_vec(&(
        "conceptweave-classification-proposals-v2",
        proposals,
        source_items,
        pending_keys,
    ))
    .expect("classification proposal records contain only JSON-serializable values");
    format!("sha256:{:x}", Sha256::digest(proposal_bytes))
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
    /// Bibliographic labels are incomplete or source records remain unresolved.
    IncompleteReview,
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
            Self::IncompleteReview => {
                "complete review must label every bibliographic item exactly once and resolve all pending sources"
            }
        })
    }
}

impl std::error::Error for EvaluationError {}

/// Evaluates a review covering every bibliographic item with no unresolved sources.
///
/// Standalone sources, orphan trees and disconnected cycles must be resolved
/// before completion. Sampled quality evaluation remains available separately.
/// Success proves reviewed metadata coverage, not a Zotero write or full-text approval.
pub fn evaluate_complete_reviewed_classification<F>(
    report: &ClassificationReport,
    golden: &ReviewedGoldenSet,
    verify_approval: F,
) -> Result<GoldenSetEvaluation, EvaluationError>
where
    F: FnOnce(&ReviewedGoldenSet) -> bool,
{
    if golden.labels.len() != report.classified_items.len()
        || !report.pending_source_item_keys.is_empty()
    {
        return Err(EvaluationError::IncompleteReview);
    }
    let evaluation = evaluate_reviewed_golden_set(report, golden, verify_approval)?;
    Ok(evaluation)
}

/// Checks that every observed item belongs to exactly one report partition.
///
/// Child links and unresolved source keys are recomputed from preserved metadata.
/// Orphans and disconnected cycles remain valid pending evidence. This checks
/// internal consistency, not source authenticity or independent approval.
pub fn validate_classification_report(
    report: &ClassificationReport,
) -> Result<(), EvaluationError> {
    let invalid = EvaluationError::InvalidReview;
    if report.observed_item_count != report.snapshot_items.len()
        || report
            .classified_items
            .len()
            .checked_add(report.unclassified_items.len())
            != Some(report.observed_item_count)
    {
        return Err(invalid);
    }
    let mut remaining_items = BTreeMap::new();
    for item in &report.snapshot_items {
        if item.item_key.trim().is_empty()
            || item.item_version > report.library_version
            || remaining_items
                .insert(item.item_key.as_str(), item.item_version)
                .is_some()
        {
            return Err(invalid);
        }
    }
    let children = child_index(&report.unclassified_items);
    for item in &report.classified_items {
        let mut reported_children = item.child_item_keys.clone();
        reported_children.sort();
        let mut actual_children = children.get(&item.item_key).cloned().unwrap_or_default();
        actual_children.sort();
        if item.item_type.trim().is_empty()
            || matches!(
                item.item_type.as_str(),
                "attachment" | "note" | "annotation"
            )
            || remaining_items.remove(item.item_key.as_str()) != Some(item.item_version)
            || reported_children != actual_children
        {
            return Err(invalid);
        }
    }
    for item in &report.unclassified_items {
        if item.data.item_type.trim().is_empty()
            || is_bibliographic(item)
            || remaining_items.remove(item.key.as_str()) != Some(item.version)
        {
            return Err(invalid);
        }
    }
    if report.audit_summary
        != classification_audit(
            &report.snapshot_items,
            &report.classified_items,
            report.duplicate_candidates.len(),
        )
    {
        return Err(invalid);
    }
    let mut reported_pending = report.pending_source_item_keys.clone();
    reported_pending.sort();
    // Equal partition size and one successful removal per record prove completeness.
    if reported_pending
        != pending_source_keys(
            &report.classified_items,
            &report.unclassified_items,
            children,
        )
    {
        return Err(invalid);
    }
    Ok(())
}

/// Evaluates reviewed labels without copying item identities into the result.
///
/// Structural, source, proposal, and label validation run before governance is
/// contacted. The verifier must authenticate the complete reviewed set against
/// an independently issued receipt, including both digests and every label;
/// accepting a self-declared receipt identifier or digest is not verification.
pub fn evaluate_reviewed_golden_set<F>(
    report: &ClassificationReport,
    golden: &ReviewedGoldenSet,
    verify_approval: F,
) -> Result<GoldenSetEvaluation, EvaluationError>
where
    F: FnOnce(&ReviewedGoldenSet) -> bool,
{
    validate_classification_report(report)?;
    if golden.approval.receipt_id.trim().is_empty()
        || golden.approval.reviewer_subject.trim().is_empty()
        || golden.labels.is_empty()
        || golden.approval.rule_revision.trim().is_empty()
        || golden.approval.snapshot_digest.trim().is_empty()
        || golden.approval.proposal_digest.trim().is_empty()
    {
        return Err(EvaluationError::InvalidReview);
    }
    let report_snapshot = report
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
    if golden.approval.proposal_digest != classification_proposal_digest(report) {
        return Err(EvaluationError::SnapshotMismatch);
    }
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

/// Builds a local-only reversible canonical-key manifest from steward-reviewed decisions.
///
/// Validates source scope, receipt bindings and all operations before contacting
/// governance. The verifier must authenticate the complete independently issued
/// review, including candidate membership and the v2 proposal digest. Legacy
/// receipts require reapproval; digest recomputation alone grants no authority.
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
        || reviewed.proposal_digest.trim().is_empty()
        || reviewed.rule_revision.trim().is_empty()
        || reviewed.decisions.len() != report.duplicate_candidates.len()
    {
        return Err(DuplicateReviewError::InvalidReview);
    }
    if reviewed.snapshot_digest != report.snapshot_digest
        || reviewed.proposal_digest != classification_proposal_digest(report)
        || reviewed.library_version != report.library_version
        || reviewed.rule_revision != report.rule_revision
        || reviewed.snapshot_items != report.snapshot_items
        || reviewed.duplicate_candidates != report.duplicate_candidates
    {
        return Err(DuplicateReviewError::SnapshotMismatch);
    }
    validate_classification_report(report).map_err(|_| DuplicateReviewError::InvalidReview)?;
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

    if !verify_review(reviewed) {
        return Err(DuplicateReviewError::UnverifiedApproval);
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
        proposal_digest: reviewed.proposal_digest.clone(),
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
        || tags
            .iter()
            .any(|tag| tag.tag_type.is_some_and(|tag_type| tag_type > 1))
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
    for tag in &mut tags {
        if tag.tag_type == Some(0) {
            tag.tag_type = None;
        }
    }
    collection_keys.sort();
    tags.sort();
    Ok((collection_keys, tags))
}

/// Builds a deterministic reviewed write plan without contacting or mutating Zotero.
///
/// Local identity, execution-mode, and metadata checks finish before the caller's
/// governance verifier runs. Invalid input never consumes a one-use approval;
/// a locally valid complete review is verified exactly once before returning a plan.
/// The verifier must authenticate every field of the reviewed set, including its
/// proposal digest and requested changes. Recomputing a digest grants no authority.
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
        || reviewed.proposal_digest.trim().is_empty()
        || reviewed.changes.is_empty()
    {
        return Err(WritePlanError::InvalidReview);
    }
    if reviewed.server_id != report.server_id
        || reviewed.zotero_version != report.zotero_version
        || reviewed.library_version != report.library_version
        || reviewed.rule_revision != report.rule_revision
        || reviewed.snapshot_digest != report.snapshot_digest
        || reviewed.snapshot_items != report.snapshot_items
    {
        return Err(WritePlanError::SnapshotMismatch);
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
        || report
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
    let snapshot_revisions = report
        .snapshot_items
        .iter()
        .map(|item| (item.item_key.as_str(), item.item_version))
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
        if snapshot_revisions.get(change.item_key.as_str()) != Some(&item.item_version) {
            return Err(WritePlanError::StaleItem);
        }
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
    validate_classification_report(report).map_err(|_| WritePlanError::InvalidReview)?;
    if reviewed.proposal_digest != classification_proposal_digest(report) {
        return Err(WritePlanError::SnapshotMismatch);
    }
    if !verify_review(reviewed) {
        return Err(WritePlanError::UnverifiedApproval);
    }
    Ok(ClassificationWritePlan {
        mode,
        review_id: reviewed.review_id.clone(),
        authority_receipt: reviewed.authority_receipt.clone(),
        server_id: reviewed.server_id.clone(),
        zotero_version: reviewed.zotero_version.clone(),
        library_version: reviewed.library_version,
        rule_revision: reviewed.rule_revision.clone(),
        snapshot_digest: reviewed.snapshot_digest.clone(),
        proposal_digest: reviewed.proposal_digest.clone(),
        operations,
        source_records_preserved: true,
    })
}

/// Executes a reviewed plan through caller-owned authenticated Local API functions.
///
/// Every item is preflighted before the first write. Adapter errors are deliberately
/// reduced to secret-free receipt states instead of being serialized.
pub fn execute_classification_write_plan<PreflightError, WriteError>(
    plan: &ClassificationWritePlan,
    mut preflight: impl FnMut(&str) -> Result<ClassificationItemState, PreflightError>,
    mut write_item: impl FnMut(
        &ClassificationWriteRequest,
    ) -> Result<ClassificationItemState, WriteError>,
) -> ClassificationWriteReceipt {
    if plan.mode == WriteMode::DryRun {
        return ClassificationWriteReceipt {
            review_id: plan.review_id.clone(),
            authority_receipt: plan.authority_receipt.clone(),
            server_id: plan.server_id.clone(),
            zotero_version: plan.zotero_version.clone(),
            library_version: plan.library_version,
            rule_revision: plan.rule_revision.clone(),
            snapshot_digest: plan.snapshot_digest.clone(),
            proposal_digest: plan.proposal_digest.clone(),
            outcome: ClassificationWriteOutcome::DryRun,
            applied_item_keys: Vec::new(),
            failed_item_key: None,
            indeterminate_item_key: None,
            indeterminate_request: None,
            reconciliation_observation: None,
            not_attempted_item_keys: plan
                .operations
                .iter()
                .map(|operation| operation.item_key.clone())
                .collect(),
            rollback_operations: Vec::new(),
        };
    }
    let server_id = plan
        .server_id
        .as_deref()
        .filter(|server_id| !server_id.trim().is_empty())
        .expect("execute plans are built with a nonblank server identity");

    for operation in &plan.operations {
        let Ok(state) = preflight(&operation.item_key) else {
            return preflight_failure_receipt(plan, Some(&operation.item_key));
        };
        if !matches_before_state(&state, server_id, plan.library_version, operation) {
            return preflight_failure_receipt(plan, Some(&operation.item_key));
        }
    }

    let mut current_library_version = plan.library_version;
    let mut applied_item_keys = Vec::new();
    let mut rollback_operations = Vec::new();
    for (operation_index, operation) in plan.operations.iter().enumerate() {
        let request = ClassificationWriteRequest {
            server_id: server_id.to_owned(),
            library_version: current_library_version,
            item_key: operation.item_key.clone(),
            item_version: operation.item_version,
            collection_keys: operation.after_collection_keys.clone(),
            tags: operation.after_tags.clone(),
        };
        let response = write_item(&request);
        let verified_state = response.ok().filter(|state| {
            matches_after_state(state, server_id, current_library_version, operation)
        });
        let Some(state) = verified_state else {
            let reconciled_state = preflight(&operation.item_key).ok();
            // A delayed request or concurrent writer can explain any observed state.
            // Retain the observation without granting completion, retry, or inverse authority.
            rollback_operations.reverse();
            return partial_failure_receipt(
                plan,
                operation_index,
                applied_item_keys,
                rollback_operations,
                request,
                reconciled_state,
            );
        };
        current_library_version = state.library_version;
        applied_item_keys.push(operation.item_key.clone());
        rollback_operations.push(ClassificationRollbackOperation {
            server_id: server_id.to_owned(),
            item_key: operation.item_key.clone(),
            item_version: state.item_version,
            expected_collection_keys: operation.after_collection_keys.clone(),
            expected_tags: operation.after_tags.clone(),
            collection_keys: operation.rollback_collection_keys.clone(),
            tags: operation.rollback_tags.clone(),
        });
    }
    rollback_operations.reverse();
    ClassificationWriteReceipt {
        review_id: plan.review_id.clone(),
        authority_receipt: plan.authority_receipt.clone(),
        server_id: plan.server_id.clone(),
        zotero_version: plan.zotero_version.clone(),
        library_version: plan.library_version,
        rule_revision: plan.rule_revision.clone(),
        snapshot_digest: plan.snapshot_digest.clone(),
        proposal_digest: plan.proposal_digest.clone(),
        outcome: ClassificationWriteOutcome::Applied,
        applied_item_keys,
        failed_item_key: None,
        indeterminate_item_key: None,
        indeterminate_request: None,
        reconciliation_observation: None,
        not_attempted_item_keys: Vec::new(),
        rollback_operations,
    }
}

/// Executes a reviewed plan through one server-bound Zotero 10 adapter.
///
/// The adapter may be created from a successful local authorization or from an
/// exact caller-owned local key. The existing execution core retains dry-run,
/// complete-preflight and receipt behavior. A failed write remains indeterminate;
/// matching observations do not grant completion, retry, or rollback authority.
pub fn execute_classification_write_plan_with_zotero10(
    plan: &ClassificationWritePlan,
    adapter: &Zotero10LocalAdapter,
) -> ClassificationWriteReceipt {
    execute_classification_write_plan(
        plan,
        |item_key| adapter.get_item(item_key),
        |request| adapter.write_item(request),
    )
}

/// Executes caller-validated inverse operations; this primitive grants no authority.
pub fn execute_classification_rollback<PreflightError, WriteError>(
    operations: &[ClassificationRollbackOperation],
    mut preflight: impl FnMut(&str) -> Result<ClassificationItemState, PreflightError>,
    mut write_item: impl FnMut(
        &ClassificationWriteRequest,
    ) -> Result<ClassificationItemState, WriteError>,
) -> ClassificationRollbackReceipt {
    if let Some(operation) = operations
        .windows(2)
        .find(|pair| pair[0].server_id != pair[1].server_id)
        .map(|pair| &pair[1])
    {
        return rollback_preflight_failure(operations, &operation.item_key);
    }

    let mut preflight_states = Vec::with_capacity(operations.len());
    let mut library_version = None;
    for operation in operations {
        let Ok(state) = preflight(&operation.item_key) else {
            return rollback_preflight_failure(operations, &operation.item_key);
        };
        let same_library = library_version.is_none_or(|version| version == state.library_version);
        if !same_library || !matches_rollback_current(&state, operation) {
            return rollback_preflight_failure(operations, &operation.item_key);
        }
        library_version = Some(state.library_version);
        preflight_states.push(state);
    }

    let mut current_library_version = library_version.unwrap_or_default();
    let mut restored_item_keys = Vec::new();
    for (index, (operation, state)) in operations.iter().zip(preflight_states).enumerate() {
        let request = ClassificationWriteRequest {
            server_id: operation.server_id.clone(),
            library_version: current_library_version,
            item_key: operation.item_key.clone(),
            item_version: state.item_version,
            collection_keys: operation.collection_keys.clone(),
            tags: operation.tags.clone(),
        };
        let verified = write_item(&request)
            .ok()
            .filter(|state| matches_rollback_restored(state, current_library_version, operation));
        if let Some(state) = verified {
            current_library_version = state.library_version;
            restored_item_keys.push(operation.item_key.clone());
            continue;
        }

        let reconciled = preflight(&operation.item_key).ok();
        return ClassificationRollbackReceipt {
            outcome: ClassificationRollbackOutcome::PartialFailure,
            restored_item_keys,
            failed_item_key: Some(operation.item_key.clone()),
            indeterminate_item_key: Some(operation.item_key.clone()),
            indeterminate_operation: Some(operation.clone()),
            indeterminate_request: Some(request),
            reconciliation_observation: reconciled,
            not_attempted_item_keys: operations[index + 1..]
                .iter()
                .map(|operation| operation.item_key.clone())
                .collect(),
            remaining_operations: operations[index + 1..].to_vec(),
        };
    }
    ClassificationRollbackReceipt {
        outcome: ClassificationRollbackOutcome::Restored,
        restored_item_keys,
        failed_item_key: None,
        indeterminate_item_key: None,
        indeterminate_operation: None,
        indeterminate_request: None,
        reconciliation_observation: None,
        not_attempted_item_keys: Vec::new(),
        remaining_operations: Vec::new(),
    }
}

/// Executes caller-validated inverses through an adapter, without issuing approval.
pub fn execute_classification_rollback_with_zotero10(
    operations: &[ClassificationRollbackOperation],
    adapter: &Zotero10LocalAdapter,
) -> ClassificationRollbackReceipt {
    execute_classification_rollback(
        operations,
        |item_key| adapter.get_item(item_key),
        |request| adapter.write_item(request),
    )
}

/// Re-reads one indeterminate rollback operation without performing a write.
pub fn reconcile_classification_rollback<ReadError>(
    operation: &ClassificationRollbackOperation,
    read_item: impl FnOnce(&str) -> Result<ClassificationItemState, ReadError>,
) -> ClassificationRollbackReconciliationReceipt {
    let valid_operation = !operation.server_id.trim().is_empty()
        && !operation.item_key.trim().is_empty()
        && normalized_metadata(
            &operation.expected_collection_keys,
            &operation.expected_tags,
        )
        .is_ok()
        && normalized_metadata(&operation.collection_keys, &operation.tags).is_ok();
    let observed_state = valid_operation
        .then(|| read_item(&operation.item_key).ok())
        .flatten();
    ClassificationRollbackReconciliationReceipt {
        state: ClassificationRollbackState::Indeterminate,
        operation: operation.clone(),
        observed_state,
        retry_operation: None,
    }
}

/// Reconciles one rollback operation through the server-bound Zotero 10 adapter.
pub fn reconcile_classification_rollback_with_zotero10(
    operation: &ClassificationRollbackOperation,
    adapter: &Zotero10LocalAdapter,
) -> ClassificationRollbackReconciliationReceipt {
    reconcile_classification_rollback(operation, |item_key| adapter.get_item(item_key))
}

fn rollback_preflight_failure(
    operations: &[ClassificationRollbackOperation],
    failed_item_key: &str,
) -> ClassificationRollbackReceipt {
    ClassificationRollbackReceipt {
        outcome: ClassificationRollbackOutcome::PreflightFailure,
        restored_item_keys: Vec::new(),
        failed_item_key: Some(failed_item_key.to_owned()),
        indeterminate_item_key: None,
        indeterminate_operation: None,
        indeterminate_request: None,
        reconciliation_observation: None,
        not_attempted_item_keys: operations
            .iter()
            .map(|item| item.item_key.clone())
            .collect(),
        remaining_operations: operations.to_vec(),
    }
}

fn matches_rollback_current(
    state: &ClassificationItemState,
    operation: &ClassificationRollbackOperation,
) -> bool {
    normalized_metadata(&state.collection_keys, &state.tags).is_ok_and(|(collections, tags)| {
        state.server_id == operation.server_id
            && state.item_key == operation.item_key
            && state.item_version == operation.item_version
            && collections == operation.expected_collection_keys
            && tags == operation.expected_tags
    })
}

fn matches_rollback_restored(
    state: &ClassificationItemState,
    library_version: u64,
    operation: &ClassificationRollbackOperation,
) -> bool {
    normalized_metadata(&state.collection_keys, &state.tags).is_ok_and(|(collections, tags)| {
        state.server_id == operation.server_id
            && state.library_version > library_version
            && state.item_key == operation.item_key
            && state.item_version > operation.item_version
            && collections == operation.collection_keys
            && tags == operation.tags
    })
}

fn matches_before_state(
    state: &ClassificationItemState,
    server_id: &str,
    library_version: u64,
    operation: &ClassificationWriteOperation,
) -> bool {
    normalized_metadata(&state.collection_keys, &state.tags).is_ok_and(|(collections, tags)| {
        state.server_id == server_id
            && state.library_version == library_version
            && state.item_key == operation.item_key
            && state.item_version == operation.item_version
            && collections == operation.before_collection_keys
            && tags == operation.before_tags
    })
}

fn matches_after_state(
    state: &ClassificationItemState,
    server_id: &str,
    library_version: u64,
    operation: &ClassificationWriteOperation,
) -> bool {
    normalized_metadata(&state.collection_keys, &state.tags).is_ok_and(|(collections, tags)| {
        state.server_id == server_id
            && state.library_version > library_version
            && state.item_key == operation.item_key
            && state.item_version > operation.item_version
            && collections == operation.after_collection_keys
            && tags == operation.after_tags
    })
}

fn preflight_failure_receipt(
    plan: &ClassificationWritePlan,
    failed_item_key: Option<&str>,
) -> ClassificationWriteReceipt {
    ClassificationWriteReceipt {
        review_id: plan.review_id.clone(),
        authority_receipt: plan.authority_receipt.clone(),
        server_id: plan.server_id.clone(),
        zotero_version: plan.zotero_version.clone(),
        library_version: plan.library_version,
        rule_revision: plan.rule_revision.clone(),
        snapshot_digest: plan.snapshot_digest.clone(),
        proposal_digest: plan.proposal_digest.clone(),
        outcome: ClassificationWriteOutcome::PreflightFailure,
        applied_item_keys: Vec::new(),
        failed_item_key: failed_item_key.map(str::to_owned),
        indeterminate_item_key: None,
        indeterminate_request: None,
        reconciliation_observation: None,
        not_attempted_item_keys: plan
            .operations
            .iter()
            .map(|operation| operation.item_key.clone())
            .collect(),
        rollback_operations: Vec::new(),
    }
}

fn partial_failure_receipt(
    plan: &ClassificationWritePlan,
    failed_index: usize,
    applied_item_keys: Vec<String>,
    rollback_operations: Vec<ClassificationRollbackOperation>,
    indeterminate_request: ClassificationWriteRequest,
    reconciliation_observation: Option<ClassificationItemState>,
) -> ClassificationWriteReceipt {
    ClassificationWriteReceipt {
        review_id: plan.review_id.clone(),
        authority_receipt: plan.authority_receipt.clone(),
        server_id: plan.server_id.clone(),
        zotero_version: plan.zotero_version.clone(),
        library_version: plan.library_version,
        rule_revision: plan.rule_revision.clone(),
        snapshot_digest: plan.snapshot_digest.clone(),
        proposal_digest: plan.proposal_digest.clone(),
        outcome: ClassificationWriteOutcome::PartialFailure,
        applied_item_keys,
        failed_item_key: Some(plan.operations[failed_index].item_key.clone()),
        indeterminate_item_key: Some(indeterminate_request.item_key.clone()),
        indeterminate_request: Some(indeterminate_request),
        reconciliation_observation,
        not_attempted_item_keys: plan
            .operations
            .iter()
            .skip(failed_index + 1)
            .map(|operation| operation.item_key.clone())
            .collect(),
        rollback_operations,
    }
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
            .any(|item| item.key.trim().is_empty() || item.version > page.library_version)
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
    let snapshot_items: Vec<_> = items
        .iter()
        .map(|item| SnapshotItemRevision {
            item_key: item.key.clone(),
            item_version: item.version,
        })
        .collect();
    let snapshot_records: Vec<_> = items
        .iter()
        .map(|item| (&item.source_record, item))
        .collect();
    let snapshot_bytes = serde_json::to_vec(&(SNAPSHOT_DIGEST_DOMAIN, snapshot_records))
        .expect("Zotero snapshot items contain only JSON-compatible values");
    let snapshot_digest = format!("sha256:{:x}", Sha256::digest(snapshot_bytes));
    let children = child_index(&items);
    let bibliographic: Vec<&ZoteroItem> =
        items.iter().filter(|item| is_bibliographic(item)).collect();
    let duplicate_candidates = duplicate_candidates(&bibliographic);
    let classified_items: Vec<ClassifiedItem> = bibliographic
        .into_iter()
        .map(|item| classify_item(item, children.get(&item.key).cloned().unwrap_or_default()))
        .collect();
    let observed_item_count = items.len();
    let unclassified_items: Vec<_> = items
        .into_iter()
        .filter(|item| !is_bibliographic(item))
        .collect();
    let pending_source_item_keys =
        pending_source_keys(&classified_items, &unclassified_items, children);

    let audit_summary = classification_audit(
        &snapshot_items,
        &classified_items,
        duplicate_candidates.len(),
    );

    ClassificationReport {
        zotero_version,
        api_version: None,
        schema_version: None,
        server_id,
        library_version,
        rule_revision: RULE_REVISION,
        observed_item_count,
        snapshot_items,
        snapshot_digest,
        classified_items,
        unclassified_items,
        pending_source_item_keys,
        duplicate_candidates,
        audit_summary,
    }
}

fn classification_audit(
    snapshot_items: &[SnapshotItemRevision],
    classified_items: &[ClassifiedItem],
    duplicate_candidate_count: usize,
) -> ClassificationAudit {
    let mut identity_counts = BTreeMap::new();
    for item in snapshot_items {
        *identity_counts
            .entry(item.item_key.as_str())
            .or_insert(0usize) += 1;
    }
    let mut disposition_counts = BTreeMap::new();
    for item in classified_items {
        *disposition_counts
            .entry(item.proposed_disposition)
            .or_insert(0) += 1;
    }
    ClassificationAudit {
        snapshot_item_count: snapshot_items.len(),
        bibliographic_item_count: classified_items.len(),
        proposed_disposition_count: classified_items.len(),
        provenance_complete_count: classified_items
            .iter()
            .filter(|item| {
                !item.item_key.trim().is_empty()
                    && identity_counts.get(item.item_key.as_str()) == Some(&1)
                    && item.child_item_keys.iter().all(|child_key| {
                        !child_key.trim().is_empty()
                            && identity_counts.get(child_key.as_str()) == Some(&1)
                    })
            })
            .count(),
        abstention_count: classified_items
            .iter()
            .filter(|item| item.proposed_disposition == Disposition::NeedsStewardReview)
            .count(),
        duplicate_candidate_count,
        failure_count: 0,
        disposition_counts,
    }
}

fn pending_source_keys(
    classified_items: &[ClassifiedItem],
    unclassified_items: &[ZoteroItem],
    mut children: BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut pending_source_item_keys: BTreeSet<_> = unclassified_items
        .iter()
        .map(|item| item.key.clone())
        .collect();
    let mut parent_item_keys: Vec<_> = classified_items
        .iter()
        .map(|item| item.item_key.clone())
        .collect();
    while let Some(parent_item_key) = parent_item_keys.pop() {
        for child_item_key in children.remove(&parent_item_key).unwrap_or_default() {
            pending_source_item_keys.remove(&child_item_key);
            parent_item_keys.push(child_item_key);
        }
    }

    pending_source_item_keys.into_iter().collect()
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

    let review_abstract_note = (proposed_disposition == Disposition::NeedsStewardReview
        && !item.data.abstract_note.trim().is_empty()
        && !field_values.contains_key("abstract_note"))
    .then(|| item.data.abstract_note.clone());

    ClassifiedItem {
        item_key: item.key.clone(),
        item_version: item.version,
        item_type: item.data.item_type.clone(),
        title: item.data.title.clone(),
        review_abstract_note,
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
    mod authenticated_transport;
    mod metadata_transport;

    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    static LOCAL_API_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn serve(responses: Vec<&'static str>) -> (String, std::thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            responses
                .into_iter()
                .map(|response| {
                    let (mut stream, _) = listener.accept().unwrap();
                    let mut bytes = Vec::new();
                    loop {
                        let mut chunk = [0; 4096];
                        let length = stream.read(&mut chunk).unwrap();
                        assert_ne!(length, 0);
                        bytes.extend_from_slice(&chunk[..length]);
                        let Some(header_end) =
                            bytes.windows(4).position(|part| part == b"\r\n\r\n")
                        else {
                            continue;
                        };
                        let headers = String::from_utf8_lossy(&bytes[..header_end]);
                        let content_length = headers
                            .lines()
                            .find_map(|line| {
                                let (name, value) = line.split_once(':')?;
                                name.eq_ignore_ascii_case("content-length")
                                    .then(|| value.trim().parse::<usize>().ok())
                                    .flatten()
                            })
                            .unwrap_or_default();
                        if bytes.len() >= header_end + 4 + content_length {
                            break;
                        }
                    }
                    stream.write_all(response.as_bytes()).unwrap();
                    String::from_utf8(bytes).unwrap()
                })
                .collect()
        });
        (format!("http://{address}/api/users/0/items"), handle)
    }

    fn library_response(server_id: &str, library_version: u64) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: {server_id}\r\nLast-Modified-Version: {library_version}\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}"
        )
    }

    fn item_response(server_id: &str, item_version: u64) -> String {
        let body = format!(
            r#"{{"key":"ABCD2345","version":{item_version},"data":{{"itemType":"book","collections":["BCDE3456"],"tags":[{{"tag":"kept","type":1}}]}}}}"#
        );
        format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: {server_id}\r\nLast-Modified-Version: {item_version}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn write_response(server_id: &str, library_version: u64, item_version: u64) -> String {
        let item = format!(
            r#"{{"key":"ABCD2345","version":{item_version},"data":{{"itemType":"book","collections":["BCDE3456"],"tags":[{{"tag":"kept","type":1}}]}}}}"#
        );
        let body = format!(r#"{{"successful":{{"0":{item}}}}}"#);
        format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: {server_id}\r\nLast-Modified-Version: {library_version}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn raw_response(server_id: Option<&str>, version: Option<u64>, body: &str) -> String {
        let server = server_id
            .map(|value| format!("Zotero-Server-ID: {value}\r\n"))
            .unwrap_or_default();
        let version = version
            .map(|value| format!("Last-Modified-Version: {value}\r\n"))
            .unwrap_or_default();
        format!(
            "HTTP/1.1 200 OK\r\n{server}{version}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn authorize_response(status: &str, server_id: Option<&str>, body: &str) -> String {
        let server = server_id
            .map(|value| format!("Zotero-Server-ID: {value}\r\n"))
            .unwrap_or_default();
        format!(
            "HTTP/1.1 {status}\r\n{server}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    fn write_request() -> ClassificationWriteRequest {
        ClassificationWriteRequest {
            server_id: "server-10".into(),
            library_version: 42,
            item_key: "ABCD2345".into(),
            item_version: 7,
            collection_keys: vec!["BCDE3456".into()],
            tags: vec![ItemTag {
                tag: "kept".into(),
                tag_type: Some(1),
            }],
        }
    }

    fn transport(base: String) -> Zotero10LocalAdapter {
        Zotero10LocalAdapter::new_with_base("0123456789abcdef0123456789abcdef", "server-10", base)
            .unwrap()
    }

    fn authorization_error(
        result: Result<Zotero10LocalAuthorization, ZoteroTransportError>,
    ) -> ZoteroTransportError {
        match result {
            Ok(_) => panic!("authorization unexpectedly succeeded"),
            Err(error) => error,
        }
    }

    #[test]
    fn approved_zotero10_adapter_executes_the_reviewed_plan_boundary() {
        let mut source_item = item("ABCD2345", "book", "ontology learning", "", "");
        source_item.data.collections = vec!["BCDE3456".into()];
        source_item.data.tags = vec![ItemTag {
            tag: "kept".into(),
            tag_type: Some(1),
        }];
        let report = classify_snapshot(
            "10.0.0".into(),
            Some("server-10".into()),
            42,
            vec![source_item],
        );
        let review = ReviewedClassificationWriteSet {
            review_id: "review-1".into(),
            authority_receipt: "authority-1".into(),
            server_id: report.server_id.clone(),
            zotero_version: report.zotero_version.clone(),
            library_version: report.library_version,
            rule_revision: report.rule_revision.into(),
            snapshot_digest: report.snapshot_digest.clone(),
            proposal_digest: classification_proposal_digest(&report),
            snapshot_items: report.snapshot_items.clone(),
            changes: vec![ReviewedClassificationChange {
                item_key: "ABCD2345".into(),
                item_version: 7,
                reviewed_disposition: Disposition::Generation,
                before_collection_keys: vec!["BCDE3456".into()],
                after_collection_keys: vec!["CDEF4567".into()],
                before_tags: vec![ItemTag {
                    tag: "kept".into(),
                    tag_type: Some(1),
                }],
                after_tags: vec![ItemTag {
                    tag: "classified".into(),
                    tag_type: None,
                }],
            }],
        };
        let plan = build_classification_write_plan(&report, &review, WriteMode::Execute, |set| {
            set == &review
        })
        .unwrap();
        let before = library_response("server-10", 42);
        let item = item_response("server-10", 7);
        let after = library_response("server-10", 42);
        let written_body = r#"{"successful":{"0":{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":["CDEF4567"],"tags":[{"tag":"classified"}]}}}}"#;
        let written = format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: server-10\r\nLast-Modified-Version: 43\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{written_body}",
            written_body.len()
        );
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
            Box::leak(after.into_boxed_str()),
            Box::leak(written.into_boxed_str()),
        ]);

        let receipt = execute_classification_write_plan_with_zotero10(&plan, &transport(base));

        assert_eq!(receipt.outcome, ClassificationWriteOutcome::Applied);
        assert_eq!(receipt.proposal_digest, review.proposal_digest);
        assert_eq!(receipt.applied_item_keys, ["ABCD2345"]);
        assert_eq!(receipt.rollback_operations[0].item_version, 43);
        assert_eq!(server.join().unwrap().len(), 4);
    }

    #[test]
    fn approved_zotero10_adapter_executes_the_rollback_boundary() {
        let operation = ClassificationRollbackOperation {
            server_id: "server-10".into(),
            item_key: "ABCD2345".into(),
            item_version: 43,
            expected_collection_keys: vec!["CDEF4567".into()],
            expected_tags: vec![ItemTag {
                tag: "classified".into(),
                tag_type: None,
            }],
            collection_keys: vec!["BCDE3456".into()],
            tags: vec![ItemTag {
                tag: "kept".into(),
                tag_type: Some(1),
            }],
        };
        let before = library_response("server-10", 44);
        let item_body = r#"{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":["CDEF4567"],"tags":[{"tag":"classified"}]}}"#;
        let item = raw_response(Some("server-10"), Some(43), item_body);
        let after = library_response("server-10", 44);
        let written = write_response("server-10", 45, 45);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
            Box::leak(after.into_boxed_str()),
            Box::leak(written.into_boxed_str()),
        ]);

        let receipt = execute_classification_rollback_with_zotero10(&[operation], &transport(base));

        assert_eq!(receipt.outcome, ClassificationRollbackOutcome::Restored);
        assert_eq!(receipt.restored_item_keys, ["ABCD2345"]);
        assert_eq!(server.join().unwrap().len(), 4);
    }

    #[test]
    fn zotero10_adapter_reconciles_an_indeterminate_rollback_without_writing() {
        let operation = ClassificationRollbackOperation {
            server_id: "server-10".into(),
            item_key: "ABCD2345".into(),
            item_version: 43,
            expected_collection_keys: vec!["CDEF4567".into()],
            expected_tags: vec![ItemTag {
                tag: "classified".into(),
                tag_type: None,
            }],
            collection_keys: vec!["BCDE3456".into()],
            tags: vec![ItemTag {
                tag: "kept".into(),
                tag_type: Some(1),
            }],
        };
        let item_body = r#"{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":["CDEF4567"],"tags":[{"tag":"classified"}]}}"#;
        let responses: Vec<&'static str> = vec![
            Box::leak(library_response("server-10", 99).into_boxed_str()),
            Box::leak(raw_response(Some("server-10"), Some(43), item_body).into_boxed_str()),
            Box::leak(library_response("server-10", 99).into_boxed_str()),
        ];
        let (base, server) = serve(responses);

        let receipt = reconcile_classification_rollback_with_zotero10(&operation, &transport(base));

        assert_eq!(receipt.state, ClassificationRollbackState::Indeterminate);
        assert!(receipt.retry_operation.is_none());
        assert_eq!(receipt.operation, operation);
        assert_eq!(
            receipt.observed_state,
            Some(ClassificationItemState {
                server_id: operation.server_id,
                library_version: 99,
                item_key: operation.item_key,
                item_version: 43,
                collection_keys: operation.expected_collection_keys,
                tags: operation.expected_tags,
            })
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 3);
        assert!(requests.iter().all(|request| request.starts_with("GET ")));
    }

    #[test]
    fn zotero10_authorization_uses_exact_wire_contract_and_builds_adapter() {
        let body = r#"{"key":"0123456789abcdef0123456789abcdef","remember":true}"#;
        let response = authorize_response("200 OK", Some("server-10"), body);
        let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        let authorize_base = items_base.replace("/api/users/0/items", "");

        let authorization = Zotero10LocalAuthorization::request_with_base(
            "ConceptWeave",
            "server-10",
            authorize_base,
        )
        .unwrap();
        assert!(authorization.remembered());
        let _adapter = authorization.into_adapter();

        let request = &server.join().unwrap()[0];
        assert!(request.starts_with("POST /api/local/authorize HTTP/1.1\r\n"));
        assert!(request.contains("content-type: application/json\r\n"));
        assert!(request.contains("zotero-server-id: server-10\r\n"));
        assert!(request.ends_with(r#"{"appName":"ConceptWeave"}"#));
        assert!(!request.contains("0123456789abcdef0123456789abcdef"));

        let body = r#"{"key":"fedcba9876543210fedcba9876543210","remember":false}"#;
        let response = authorize_response("200 OK", Some("server-10"), body);
        let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        let authorization = Zotero10LocalAuthorization::request_with_base(
            "ConceptWeave",
            "server-10",
            items_base.replace("/api/users/0/items", ""),
        )
        .unwrap();
        assert!(!authorization.remembered());
        server.join().unwrap();
    }

    #[test]
    fn zotero10_authorization_rejects_invalid_input_and_unproven_success() {
        let oversized_name = "x".repeat(MAX_AUTH_APP_NAME_BYTES + 1);
        for app_name in ["", " ", oversized_name.as_str()] {
            assert_eq!(
                authorization_error(Zotero10LocalAuthorization::request(app_name, "server-10")),
                ZoteroTransportError::InvalidCredentials
            );
        }
        assert_eq!(
            authorization_error(Zotero10LocalAuthorization::request("ConceptWeave", " ")),
            ZoteroTransportError::InvalidCredentials
        );

        for response in [
            authorize_response(
                "200 OK",
                None,
                r#"{"key":"0123456789abcdef0123456789abcdef","remember":false}"#,
            ),
            authorize_response(
                "200 OK",
                Some("other-server"),
                r#"{"key":"0123456789abcdef0123456789abcdef","remember":false}"#,
            ),
            authorize_response(
                "200 OK",
                Some("server-10"),
                r#"{"key":"too-short","remember":false}"#,
            ),
            authorize_response(
                "200 OK",
                Some("server-10"),
                r#"{"key":"0123456789abcdef0123456789abcde ","remember":false}"#,
            ),
            authorize_response("200 OK", Some("server-10"), "{"),
            authorize_response(
                "200 OK",
                Some("server-10"),
                &format!(r#"{{"key":"{}","remember":false}}"#, "x".repeat(1024)),
            ),
        ] {
            let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            let error = authorization_error(Zotero10LocalAuthorization::request_with_base(
                "ConceptWeave",
                "server-10",
                items_base.replace("/api/users/0/items", ""),
            ));
            assert!(matches!(
                error,
                ZoteroTransportError::InvalidResponse | ZoteroTransportError::ServerMismatch
            ));
            server.join().unwrap();
        }

        for (response, expected) in [
            (
                "HTTP/1.1 500 Internal Server Error\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                ZoteroTransportError::RequestFailed,
            ),
            (
                "HTTP/1.1 429 Too Many Requests\r\nZotero-Server-ID: server-10\r\nRetry-After: tomorrow\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                ZoteroTransportError::RateLimited {
                    retry_after_seconds: None,
                },
            ),
        ] {
            let (items_base, server) = serve(vec![response]);
            assert_eq!(
                authorization_error(Zotero10LocalAuthorization::request_with_base(
                    "ConceptWeave",
                    "server-10",
                    items_base.replace("/api/users/0/items", ""),
                )),
                expected
            );
            server.join().unwrap();
        }

        assert_eq!(
            authorization_error(Zotero10LocalAuthorization::request_with_base(
                "ConceptWeave",
                "server-10",
                "http://127.0.0.1:0".into(),
            )),
            ZoteroTransportError::RequestFailed
        );
    }

    #[test]
    fn zotero10_authorization_denial_and_rate_limit_are_single_attempt_errors() {
        for (response, expected) in [
            (
                authorize_response("403 Forbidden", Some("server-10"), r#"{"denied":true}"#),
                ZoteroTransportError::Denied,
            ),
            (
                authorize_response("401 Unauthorized", Some("server-10"), ""),
                ZoteroTransportError::Unauthorized,
            ),
            (
                "HTTP/1.1 429 Too Many Requests\r\nZotero-Server-ID: server-10\r\nRetry-After: 17\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_owned(),
                ZoteroTransportError::RateLimited {
                    retry_after_seconds: Some(17),
                },
            ),
            (
                "HTTP/1.1 429 Too Many Requests\r\nZotero-Server-ID: server-10\r\nRetry-After: 999999999999\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_owned(),
                ZoteroTransportError::RateLimited {
                    retry_after_seconds: None,
                },
            ),
        ] {
            let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            assert_eq!(
                authorization_error(Zotero10LocalAuthorization::request_with_base(
                    "ConceptWeave",
                    "server-10",
                    items_base.replace("/api/users/0/items", ""),
                )),
                expected
            );
            assert_eq!(server.join().unwrap().len(), 1);
        }

        for response in [
            authorize_response("403 Forbidden", Some("server-10"), ""),
            authorize_response("403 Forbidden", Some("server-10"), r#"{"denied":false}"#),
            authorize_response("403 Forbidden", None, r#"{"denied":true}"#),
            authorize_response(
                "403 Forbidden",
                Some("server-10"),
                &"x".repeat((MAX_AUTH_RESPONSE_BYTES + 1) as usize),
            ),
        ] {
            let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            assert!(matches!(
                authorization_error(Zotero10LocalAuthorization::request_with_base(
                    "ConceptWeave",
                    "server-10",
                    items_base.replace("/api/users/0/items", ""),
                )),
                ZoteroTransportError::InvalidResponse
            ));
            assert_eq!(server.join().unwrap().len(), 1);
        }
    }

    #[test]
    fn zotero10_write_names_reauthorization_and_stale_precondition() {
        for (status, expected) in [
            (
                "401 Unauthorized",
                ZoteroTransportError::ReauthorizationRequired,
            ),
            (
                "412 Precondition Failed",
                ZoteroTransportError::StalePrecondition,
            ),
        ] {
            let response = format!(
                "HTTP/1.1 {status}\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            assert_eq!(
                transport(base).write_item(&write_request()).unwrap_err(),
                expected
            );
            assert_eq!(server.join().unwrap().len(), 1);
        }

        assert_eq!(
            transport("http://127.0.0.1:0/api/users/0/items".into())
                .write_item(&write_request())
                .unwrap_err(),
            ZoteroTransportError::RequestFailed
        );

        let response = library_response("server-10", 42);
        let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::RequestFailed
        );
        server.join().unwrap();

        assert_eq!(
            transport("http://127.0.0.1:0/api/users/0/items".into())
                .get_item("ABCD2345")
                .unwrap_err(),
            ZoteroTransportError::RequestFailed
        );
    }

    fn assert_write_invalid(response: String) {
        let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        assert_eq!(
            transport(base).write_item(&write_request()).unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();
    }

    #[test]
    fn zotero10_get_uses_exact_item_route_and_server_partition() {
        let before = library_response("server-10", 42);
        let item = item_response("server-10", 7);
        let after = library_response("server-10", 42);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
            Box::leak(after.into_boxed_str()),
        ]);
        let state = transport(base).get_item("ABCD2345").unwrap();
        assert_eq!(state.server_id, "server-10");
        assert_eq!(state.library_version, 42);
        assert_eq!(state.item_version, 7);
        assert_eq!(state.collection_keys, ["BCDE3456"]);
        assert_eq!(state.tags[0].tag_type, Some(1));
        let requests = server.join().unwrap();
        assert!(
            requests[1].starts_with(
                "GET /api/users/0/items/ABCD2345?format=json&include=data HTTP/1.1\r\n"
            )
        );
        assert!(requests[0].starts_with("GET /api/users/0/items?format=versions&limit=1 "));
        assert!(requests[2].starts_with("GET /api/users/0/items?format=versions&limit=1 "));
        assert!(
            requests
                .iter()
                .all(|request| request.contains("zotero-api-version: 3\r\n"))
        );
        assert!(
            requests
                .iter()
                .all(|request| request.contains("zotero-server-id: server-10\r\n"))
        );
        assert!(
            requests
                .iter()
                .all(|request| !request.contains("0123456789abcdef0123456789abcdef"))
        );
    }

    #[test]
    fn zotero10_post_atomically_replaces_complete_arrays() {
        let response = write_response("server-10", 43, 43);
        let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        let request = write_request();
        let state = transport(base).write_item(&request).unwrap();
        assert_eq!(state.library_version, 43);
        assert_eq!(state.item_version, 43);
        let requests = server.join().unwrap();
        assert!(requests[0].starts_with("POST /api/users/0/items HTTP/1.1\r\n"));
        assert!(requests[0].contains("zotero-api-key: 0123456789abcdef0123456789abcdef\r\n"));
        assert!(requests[0].contains("zotero-server-id: server-10\r\n"));
        assert!(requests[0].contains("if-unmodified-since-version: 42\r\n"));
        assert!(requests[0].contains("content-type: application/json\r\n"));
        assert!(
            requests[0].ends_with(
                r#"[{"collections":["BCDE3456"],"key":"ABCD2345","tags":[{"tag":"kept","type":1}],"version":7}]"#
            )
        );
    }

    #[test]
    fn zotero10_transport_rejects_stale_non_success_and_server_mismatch() {
        let stale = "HTTP/1.1 412 Precondition Failed\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (base, server) = serve(vec![stale]);
        let request = ClassificationWriteRequest {
            server_id: "server-10".into(),
            library_version: 42,
            item_key: "ABCD2345".into(),
            item_version: 7,
            collection_keys: vec![],
            tags: vec![],
        };
        assert_eq!(
            transport(base).write_item(&request).unwrap_err(),
            ZoteroTransportError::StalePrecondition
        );
        assert!(server.join().unwrap()[0].starts_with("POST "));

        let switched = "HTTP/1.1 412 Precondition Failed\r\nZotero-Server-ID: other-server\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (base, server) = serve(vec![switched]);
        assert_eq!(
            transport(base).write_item(&request).unwrap_err(),
            ZoteroTransportError::ServerMismatch
        );
        server.join().unwrap();

        let mut mismatched_request = request.clone();
        mismatched_request.server_id = "other-server".into();
        assert_eq!(
            Zotero10LocalAdapter::new("0123456789abcdef0123456789abcdef", "server-10")
                .unwrap()
                .write_item(&mismatched_request)
                .unwrap_err(),
            ZoteroTransportError::ServerMismatch
        );

        let unexpected_success = "HTTP/1.1 204 No Content\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (base, server) = serve(vec![unexpected_success]);
        assert_eq!(
            transport(base).write_item(&request).unwrap_err(),
            ZoteroTransportError::RequestFailed
        );
        server.join().unwrap();

        let wrong_server = write_response("other-server", 43, 43);
        let (base, server) = serve(vec![Box::leak(wrong_server.into_boxed_str())]);
        assert_eq!(
            transport(base).write_item(&request).unwrap_err(),
            ZoteroTransportError::ServerMismatch
        );
        server.join().unwrap();

        for response in [
            "HTTP/1.1 500 Internal Server Error\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_owned(),
            library_response("other-server", 42),
        ] {
            let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            let error = transport(base).get_item("ABCD2345").unwrap_err();
            assert!(matches!(
                error,
                ZoteroTransportError::RequestFailed | ZoteroTransportError::ServerMismatch
            ));
            server.join().unwrap();
        }
    }

    #[test]
    fn zotero10_transport_rejects_invalid_keys_credentials_and_bounded_bodies() {
        assert_eq!(
            Zotero10LocalAdapter::new(" ", "server-10").err().unwrap(),
            ZoteroTransportError::InvalidCredentials
        );
        assert_eq!(
            Zotero10LocalAdapter::new("0123456789abcdef0123456789abcdef", " ")
                .err()
                .unwrap(),
            ZoteroTransportError::InvalidCredentials
        );
        for api_key in [
            "too-short",
            "0123456789abcdef0123456789abcde ",
            "0123456789abcdef0123456789abcdefx",
        ] {
            assert_eq!(
                Zotero10LocalAdapter::new(api_key, "server-10")
                    .err()
                    .unwrap(),
                ZoteroTransportError::InvalidCredentials
            );
        }
        let adapter =
            Zotero10LocalAdapter::new("0123456789abcdef0123456789abcdef", "server-10").unwrap();
        for key in ["ABCD234", "ABCD2340", "abcd2345", "ABCD2345/../X"] {
            assert_eq!(
                adapter.get_item(key).unwrap_err(),
                ZoteroTransportError::InvalidItemKey
            );
        }
        let mut invalid_write = write_request();
        invalid_write.item_key = "invalid".into();
        assert_eq!(
            adapter.write_item(&invalid_write).unwrap_err(),
            ZoteroTransportError::InvalidItemKey
        );

        let malformed = "HTTP/1.1 200 OK\r\nZotero-Server-ID: server-10\r\nLast-Modified-Version: 42\r\nContent-Length: 1\r\nConnection: close\r\n\r\n{";
        let before = library_response("server-10", 42);
        let (base, server) = serve(vec![Box::leak(before.into_boxed_str()), malformed]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();

        let wrong_key_body = r#"{"key":"BCDE3456","version":7,"data":{"itemType":"book"}}"#;
        let wrong_key_response = format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: server-10\r\nLast-Modified-Version: 7\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{wrong_key_body}",
            wrong_key_body.len()
        );
        let before = library_response("server-10", 42);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(wrong_key_response.into_boxed_str()),
        ]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();

        let before = library_response("server-10", 42);
        let item = item_response("server-10", 7);
        let after = library_response("server-10", 43);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
            Box::leak(after.into_boxed_str()),
        ]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();

        let oversized = "x".repeat((MAX_ITEM_RESPONSE_BYTES + 1) as usize);
        let response = format!(
            "HTTP/1.1 200 OK\r\nZotero-Server-ID: server-10\r\nLast-Modified-Version: 42\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{oversized}",
            oversized.len()
        );
        let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();
    }

    #[test]
    fn zotero10_transport_covers_read_stage_failures() {
        let library = library_response("server-10", 42);
        let (base, server) = serve(vec![
            Box::leak(library.into_boxed_str()),
            "HTTP/1.1 500 Internal Server Error\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        ]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::RequestFailed
        );
        server.join().unwrap();

        let before = library_response("server-10", 42);
        let item = item_response("server-10", 7);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
            "HTTP/1.1 500 Internal Server Error\r\nZotero-Server-ID: server-10\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        ]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::RequestFailed
        );
        server.join().unwrap();

        let switched = "HTTP/1.1 412 Precondition Failed\r\nZotero-Server-ID: other-server\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (base, server) = serve(vec![switched]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::ServerMismatch
        );
        server.join().unwrap();

        let before = library_response("server-10", 42);
        let (base, server) = serve(vec![Box::leak(before.into_boxed_str()), switched]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::ServerMismatch
        );
        server.join().unwrap();

        for response in [
            raw_response(None, Some(42), "{}"),
            raw_response(Some("server-10"), None, "{}"),
        ] {
            let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            assert_eq!(
                transport(base).get_item("ABCD2345").unwrap_err(),
                ZoteroTransportError::InvalidResponse
            );
            server.join().unwrap();
        }

        let item_body = r#"{"key":"ABCD2345","version":8,"data":{"itemType":"book"}}"#;
        for item in [
            raw_response(Some("other-server"), Some(8), item_body),
            raw_response(Some("server-10"), None, item_body),
            raw_response(Some("server-10"), Some(7), item_body),
        ] {
            let before = library_response("server-10", 42);
            let (base, server) = serve(vec![
                Box::leak(before.into_boxed_str()),
                Box::leak(item.into_boxed_str()),
            ]);
            let error = transport(base).get_item("ABCD2345").unwrap_err();
            assert!(matches!(
                error,
                ZoteroTransportError::InvalidResponse | ZoteroTransportError::ServerMismatch
            ));
            server.join().unwrap();
        }

        let oversized = "x".repeat((MAX_ITEM_RESPONSE_BYTES + 1) as usize);
        let before = library_response("server-10", 42);
        let item = raw_response(Some("server-10"), Some(7), &oversized);
        let (base, server) = serve(vec![
            Box::leak(before.into_boxed_str()),
            Box::leak(item.into_boxed_str()),
        ]);
        assert_eq!(
            transport(base).get_item("ABCD2345").unwrap_err(),
            ZoteroTransportError::InvalidResponse
        );
        server.join().unwrap();
    }

    #[test]
    fn zotero10_transport_rejects_every_unproven_write_result() {
        assert_write_invalid(raw_response(Some("server-10"), None, "{}"));
        assert_write_invalid(raw_response(Some("server-10"), Some(43), "{"));
        assert_write_invalid(raw_response(Some("server-10"), Some(43), "{}"));
        assert_write_invalid(raw_response(
            Some("server-10"),
            Some(43),
            r#"{"successful":{"0":{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":["BCDE3456"],"tags":[{"tag":"kept","type":1}]}},"1":{"key":"BCDE3456","version":43,"data":{"itemType":"book"}}}}"#,
        ));
        for (version, body) in [
            (
                43,
                r#"{"successful":{"0":{"key":"BCDE3456","version":43,"data":{"itemType":"book","collections":["BCDE3456"],"tags":[{"tag":"kept","type":1}]}}}}"#,
            ),
            (
                7,
                r#"{"successful":{"0":{"key":"ABCD2345","version":7,"data":{"itemType":"book","collections":["BCDE3456"],"tags":[{"tag":"kept","type":1}]}}}}"#,
            ),
            (
                43,
                r#"{"successful":{"0":{"key":"ABCD2345","version":44,"data":{"itemType":"book","collections":["BCDE3456"],"tags":[{"tag":"kept","type":1}]}}}}"#,
            ),
            (
                43,
                r#"{"successful":{"0":{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":[],"tags":[{"tag":"kept","type":1}]}}}}"#,
            ),
            (
                43,
                r#"{"successful":{"0":{"key":"ABCD2345","version":43,"data":{"itemType":"book","collections":["BCDE3456"],"tags":[]}}}}"#,
            ),
        ] {
            assert_write_invalid(raw_response(Some("server-10"), Some(version), body));
        }
        let oversized = "x".repeat((MAX_ITEM_RESPONSE_BYTES + 1) as usize);
        assert_write_invalid(raw_response(Some("server-10"), Some(43), &oversized));
    }

    #[test]
    fn zotero10_transport_never_formats_or_serializes_the_key() {
        let adapter =
            Zotero10LocalAdapter::new("0123456789abcdef0123456789abcdef", "server-10").unwrap();
        assert!(!std::any::type_name_of_val(&adapter).contains("0123456789abcdef0123456789abcdef"));
        assert_eq!(
            format!("{:?}", ZoteroTransportError::RequestFailed),
            "RequestFailed"
        );
    }

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
            source_record: None,
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
    fn reader_rejects_blank_source_identity_before_another_page() {
        for blank_key in ["", " ", "\t\n"] {
            for invalid_start in [0, 1] {
                let mut starts = Vec::new();
                let result = read_snapshot_with(&mut |start| {
                    starts.push(start);
                    let key = if start == invalid_start {
                        blank_key.to_owned()
                    } else {
                        format!("A{start}")
                    };
                    Ok(fetched_page(3, vec![item(&key, "attachment", "", "", "")]))
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
    fn source_inventory_retains_standalone_and_nested_metadata_without_proposals() {
        let mut standalone = item("A", "attachment", "Ontology Learning", "10.1/X", "");
        standalone.version = 0;
        standalone.data.collections = vec!["COLLECTION".into()];
        standalone.data.tags = vec![ItemTag {
            tag: "Evidence".into(),
            tag_type: None,
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
