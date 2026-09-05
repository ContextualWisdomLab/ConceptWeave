//! Private, content-bound full-text read sweeps, separate from classification approval.

use crate::{
    ClassificationReport, MAX_PAGE_BYTES, MAX_SNAPSHOT_BYTES, MAX_SNAPSHOT_ITEMS,
    SnapshotItemRevision, ZoteroItem, bounded_body_with_limit, build_steward_review_worksheet,
    local_agent, validate_item_key, verify_server_id,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const CAPTURE_KIND: &str = "non_atomic_fulltext_sweep_v1";
const INVALID_EVIDENCE: FullTextError = FullTextError("full-text capture evidence is invalid");
const BUDGET_EXCEEDED: FullTextError = FullTextError("full-text capture budget exceeded");
const CAPTURE_DEADLINE: Duration = Duration::from_secs(300);

/// A bounded full-text observation artifact, not an atomic snapshot or approval.
///
/// This contains sensitive source text. Store it only through the owner-only CLI
/// boundary. Deserialization establishes shape; call [`verify_full_text_capture`]
/// before using a restored artifact. Neither operation authenticates a provider.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FullTextCapture {
    capture_digest: String,
    capture_evidence: CaptureEvidence,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CaptureEvidence {
    capture_kind: String,
    metadata_report_digest: String,
    metadata_snapshot_digest: String,
    bibliographic_item_count: usize,
    started_unix_ms: u64,
    finished_unix_ms: u64,
    library_before: CapturedResponse,
    manifest_before: CapturedResponse,
    records: Vec<CapturedItem>,
    manifest_after: CapturedResponse,
    library_after: CapturedResponse,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CapturedItem {
    item_key: String,
    metadata_response: CapturedResponse,
    content_response: CapturedResponse,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CapturedResponse {
    status: u16,
    version: Option<u64>,
    body: String,
}

/// A secret-free capture failure that never embeds an item URL or source content.
#[derive(Debug, PartialEq, Eq)]
pub struct FullTextError(&'static str);

impl fmt::Display for FullTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for FullTextError {}

/// Reads every full-text manifest entry through the fixed loopback API.
///
/// The report remains unchanged. Missing content is retained explicitly, while
/// invalid identity, request failure, drift or exhausted budgets fail the sweep.
pub fn read_local_full_text(
    report: &ClassificationReport,
) -> Result<FullTextCapture, FullTextError> {
    read_full_text_from_api(report, crate::LOCAL_API_ROOT)
}

fn read_full_text_from_api(
    report: &ClassificationReport,
    api_root: &str,
) -> Result<FullTextCapture, FullTextError> {
    let agent = local_agent();
    capture_with(report, MAX_SNAPSHOT_BYTES, &mut |request_path, limit| {
        fetch_response(&agent, report, api_root, request_path, limit)
    })
}

/// Verifies a restored capture's complete content and original report binding.
///
/// This detects altered artifacts under an unchanged digest. It does not grant
/// approval, establish atomicity, or authenticate a locally replaced digest.
pub fn verify_full_text_capture(
    capture: &FullTextCapture,
    report: &ClassificationReport,
) -> Result<(), FullTextError> {
    let snapshot = validate_report(report)?;
    let evidence = &capture.capture_evidence;
    if capture.capture_digest != json_digest(evidence)
        || evidence.capture_kind != CAPTURE_KIND
        || evidence.metadata_report_digest != json_digest(report)
        || evidence.metadata_snapshot_digest != report.snapshot_digest
        || evidence.bibliographic_item_count != report.classified_items.len()
        || evidence.finished_unix_ms < evidence.started_unix_ms
    {
        return Err(INVALID_EVIDENCE);
    }
    validate_library(&evidence.library_before, report)?;
    validate_library(&evidence.library_after, report)?;
    let manifest = parse_manifest(&evidence.manifest_before, &snapshot)?;
    if evidence.manifest_after.status != 200
        || evidence.manifest_after.body != evidence.manifest_before.body
        || evidence.records.len() != manifest.len()
    {
        return Err(INVALID_EVIDENCE);
    }
    let mut remaining = MAX_SNAPSHOT_BYTES;
    for response in [
        &evidence.library_before,
        &evidence.manifest_before,
        &evidence.manifest_after,
        &evidence.library_after,
    ] {
        account_body(&mut remaining, response)?;
    }
    for (record, (item_key, version)) in evidence.records.iter().zip(&manifest) {
        if &record.item_key != item_key {
            return Err(INVALID_EVIDENCE);
        }
        validate_metadata(&record.metadata_response, snapshot[item_key.as_str()])?;
        validate_content(&record.content_response, *version)?;
        account_body(&mut remaining, &record.metadata_response)?;
        account_body(&mut remaining, &record.content_response)?;
    }
    Ok(())
}

fn capture_with(
    report: &ClassificationReport,
    max_bytes: u64,
    fetch: &mut dyn FnMut(&str, u64) -> Result<CapturedResponse, FullTextError>,
) -> Result<FullTextCapture, FullTextError> {
    let snapshot = validate_report(report)?;
    let started_unix_ms = unix_millis(SystemTime::now())?;
    let started = Instant::now();
    let mut remaining = max_bytes.min(MAX_SNAPSHOT_BYTES);
    let mut read = |request_path: &str| {
        check_admission(remaining, started.elapsed())?;
        let response = fetch(request_path, remaining.min(MAX_PAGE_BYTES))?;
        account_body(&mut remaining, &response)?;
        check_deadline(started.elapsed())?;
        Ok::<_, FullTextError>(response)
    };
    let library_before = read("items?limit=1")?;
    validate_library(&library_before, report)?;
    let manifest_before = read("fulltext?since=0")?;
    let manifest = parse_manifest(&manifest_before, &snapshot)?;
    let mut records = Vec::with_capacity(manifest.len());
    for (item_key, version) in manifest {
        let metadata_response = read(&format!("items/{item_key}"))?;
        validate_metadata(&metadata_response, snapshot[item_key.as_str()])?;
        let content_response = read(&format!("items/{item_key}/fulltext"))?;
        validate_content(&content_response, version)?;
        records.push(CapturedItem {
            item_key,
            metadata_response,
            content_response,
        });
    }
    let manifest_after = read("fulltext?since=0")?;
    let library_after = read("items?limit=1")?;
    let capture_evidence = CaptureEvidence {
        capture_kind: CAPTURE_KIND.into(),
        metadata_report_digest: json_digest(report),
        metadata_snapshot_digest: report.snapshot_digest.clone(),
        bibliographic_item_count: report.classified_items.len(),
        started_unix_ms,
        finished_unix_ms: unix_millis(SystemTime::now())?,
        library_before,
        manifest_before,
        records,
        manifest_after,
        library_after,
    };
    let capture = FullTextCapture {
        capture_digest: json_digest(&capture_evidence),
        capture_evidence,
    };
    verify_full_text_capture(&capture, report)?;
    check_deadline(started.elapsed())?;
    Ok(capture)
}

fn validate_report(
    report: &ClassificationReport,
) -> Result<BTreeMap<&str, &SnapshotItemRevision>, FullTextError> {
    let valid = report.api_version == Some(3)
        && report.schema_version.is_some()
        && report
            .server_id
            .as_deref()
            .is_some_and(|server| !server.trim().is_empty())
        && report
            .zotero_version
            .split('.')
            .next()
            .and_then(|major| major.parse::<u64>().ok())
            .is_some_and(|major| major >= 10)
        && !report.classified_items.is_empty()
        && report.observed_item_count <= MAX_SNAPSHOT_ITEMS
        && build_steward_review_worksheet(report).is_ok();
    if !valid {
        return Err(FullTextError(
            "full-text capture requires a bound Zotero 10+ report",
        ));
    }
    let mut snapshot = BTreeMap::new();
    for item in &report.snapshot_items {
        validate_item_key(&item.item_key).map_err(|_| INVALID_EVIDENCE)?;
        snapshot.insert(item.item_key.as_str(), item);
    }
    Ok(snapshot)
}

fn validate_library(
    response: &CapturedResponse,
    report: &ClassificationReport,
) -> Result<(), FullTextError> {
    if response.status != 200 || response.version != Some(report.library_version) {
        return Err(INVALID_EVIDENCE);
    }
    let _: Vec<serde_json::Value> =
        serde_json::from_str(&response.body).map_err(|_| INVALID_EVIDENCE)?;
    Ok(())
}

fn parse_manifest(
    response: &CapturedResponse,
    snapshot: &BTreeMap<&str, &SnapshotItemRevision>,
) -> Result<BTreeMap<String, u64>, FullTextError> {
    struct UniqueManifest;
    impl<'de> serde::de::Visitor<'de> for UniqueManifest {
        type Value = BTreeMap<String, u64>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a unique full-text manifest")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> Result<Self::Value, M::Error> {
            let mut entries = BTreeMap::new();
            while let Some((key, value)) = map.next_entry::<String, u64>()? {
                if entries.len() == MAX_SNAPSHOT_ITEMS || entries.insert(key, value).is_some() {
                    return Err(serde::de::Error::custom("invalid manifest entries"));
                }
            }
            Ok(entries)
        }
    }
    if response.status != 200 {
        return Err(INVALID_EVIDENCE);
    }
    let mut deserializer = serde_json::Deserializer::from_str(&response.body);
    let manifest = serde::Deserializer::deserialize_map(&mut deserializer, UniqueManifest)
        .map_err(|_| INVALID_EVIDENCE)?;
    deserializer.end().map_err(|_| INVALID_EVIDENCE)?;
    if manifest
        .keys()
        .any(|key| !snapshot.contains_key(key.as_str()))
    {
        return Err(INVALID_EVIDENCE);
    }
    Ok(manifest)
}

fn validate_metadata(
    response: &CapturedResponse,
    expected: &SnapshotItemRevision,
) -> Result<(), FullTextError> {
    if response.status != 200 || response.version != Some(expected.item_version) {
        return Err(INVALID_EVIDENCE);
    }
    let item: ZoteroItem = serde_json::from_str(&response.body).map_err(|_| INVALID_EVIDENCE)?;
    if item.key != expected.item_key
        || item.version != expected.item_version
        || item.data.item_type != "attachment"
        || item.data.parent_item != expected.parent_item_key.as_deref().unwrap_or("")
    {
        return Err(INVALID_EVIDENCE);
    }
    Ok(())
}

fn validate_content(response: &CapturedResponse, version: u64) -> Result<(), FullTextError> {
    match response.status {
        404 => Ok(()),
        200 if response.version == Some(version) => {
            #[derive(Deserialize)]
            struct ContentProjection {
                content: String,
            }
            let content: ContentProjection =
                serde_json::from_str(&response.body).map_err(|_| INVALID_EVIDENCE)?;
            let _ = content.content;
            Ok(())
        }
        _ => Err(INVALID_EVIDENCE),
    }
}

fn account_body(remaining: &mut u64, response: &CapturedResponse) -> Result<(), FullTextError> {
    let body_bytes = response.body.len() as u64;
    if body_bytes > MAX_PAGE_BYTES {
        return Err(BUDGET_EXCEEDED);
    }
    *remaining = remaining.checked_sub(body_bytes).ok_or(BUDGET_EXCEEDED)?;
    Ok(())
}

fn check_admission(remaining: u64, elapsed: Duration) -> Result<(), FullTextError> {
    if remaining == 0 {
        return Err(BUDGET_EXCEEDED);
    }
    check_deadline(elapsed)
}

fn check_deadline(elapsed: Duration) -> Result<(), FullTextError> {
    if elapsed >= CAPTURE_DEADLINE {
        Err(BUDGET_EXCEEDED)
    } else {
        Ok(())
    }
}

fn unix_millis(time: SystemTime) -> Result<u64, FullTextError> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|value| value.as_millis().try_into().ok())
        .ok_or(INVALID_EVIDENCE)
}

fn json_digest(value: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(value).expect("capture values are JSON-compatible");
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn fetch_response(
    agent: &ureq::Agent,
    report: &ClassificationReport,
    api_root: &str,
    request_path: &str,
    limit: u64,
) -> Result<CapturedResponse, FullTextError> {
    let mut response = agent
        .get(&format!("{api_root}/api/users/0/{request_path}"))
        .header("Zotero-API-Version", "3")
        .header(
            "Zotero-Server-ID",
            report.server_id.as_deref().ok_or(INVALID_EVIDENCE)?,
        )
        .call()
        .map_err(|_| FullTextError("full-text local request failed"))?;
    let headers = response.headers();
    verify_server_id(
        headers,
        report.server_id.as_deref().ok_or(INVALID_EVIDENCE)?,
    )
    .map_err(|_| INVALID_EVIDENCE)?;
    for (header, expected) in [
        ("Zotero-API-Version", "3".to_owned()),
        (
            "Zotero-Schema-Version",
            report.schema_version.ok_or(INVALID_EVIDENCE)?.to_string(),
        ),
        ("X-Zotero-Version", report.zotero_version.clone()),
    ] {
        if headers.get(header).and_then(|value| value.to_str().ok()) != Some(expected.as_str()) {
            return Err(INVALID_EVIDENCE);
        }
    }
    let version = headers
        .get("Last-Modified-Version")
        .map(|value| {
            value
                .to_str()
                .ok()
                .and_then(|text| text.parse::<u64>().ok())
                .ok_or(INVALID_EVIDENCE)
        })
        .transpose()?;
    Ok(CapturedResponse {
        status: response.status().as_u16(),
        version,
        body: bounded_body_with_limit(&mut response, limit).map_err(|_| INVALID_EVIDENCE)?,
    })
}

#[cfg(test)]
#[path = "full_text_capture_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "full_text_capture_transport_tests.rs"]
mod transport_tests;
