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
use std::io::{self, Write};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const CAPTURE_KIND: &str = "non_atomic_fulltext_sweep_v1";
const INVALID_EVIDENCE: FullTextError = FullTextError("full-text capture evidence is invalid");
const BUDGET_EXCEEDED: FullTextError = FullTextError("full-text capture budget exceeded");
const CAPTURE_DEADLINE: Duration = Duration::from_secs(300);
const MAX_PERSISTED_CAPTURE_BYTES: u64 = 512 * 1024 * 1024;

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

/// Builds a bounded private evidence view for the next pending review rows.
///
/// This verifies the complete capture against the unchanged report, then selects
/// the same 1–100 pending rows as the metadata review command. Each selected paper
/// keeps an attachment list, even when no text was captured. Exact response bodies
/// preserve missing, empty and partially indexed content without interpreting it.
/// The serialized output is limited to 16 MiB, including JSON escaping; an
/// oversized view fails without truncation. Callers must separately bound private
/// capture-file deserialization before passing the restored capture here.
///
/// The returned JSON is not a decision patch or an approval. It must not replace
/// the original report or worksheet, and it is rejected by legacy apply commands.
pub fn build_full_text_review_json(
    report: &ClassificationReport,
    worksheet: &crate::StewardReviewWorksheet,
    capture: &FullTextCapture,
    limit: usize,
) -> Result<Vec<u8>, FullTextError> {
    #[derive(Serialize)]
    struct AttachmentEvidence<'a> {
        item_key: &'a str,
        metadata_version: Option<u64>,
        content_response: &'a CapturedResponse,
    }
    #[derive(Serialize)]
    struct ReviewView<'a> {
        view_kind: &'static str,
        capture_digest: &'a str,
        metadata_report_digest: &'a str,
        proposal_digest: String,
        bibliographic_item_count: usize,
        review_batch: crate::StewardReviewBatch,
        attachment_evidence: BTreeMap<String, Vec<AttachmentEvidence<'a>>>,
    }

    let review_batch = crate::build_steward_review_batch(report, worksheet, limit)
        .map_err(|_| FullTextError("full-text review requires a valid pending batch"))?;
    verify_full_text_capture(capture, report)?;
    let parent_by_key: BTreeMap<_, _> = report
        .snapshot_items
        .iter()
        .map(|item| (item.item_key.as_str(), item.parent_item_key.as_deref()))
        .collect();
    let mut attachment_evidence: BTreeMap<_, Vec<_>> = review_batch
        .decisions
        .iter()
        .map(|decision| (decision.item_key.clone(), Vec::new()))
        .collect();
    for record in &capture.capture_evidence.records {
        if let Some(rows) = parent_by_key[record.item_key.as_str()]
            .and_then(|parent| attachment_evidence.get_mut(parent))
        {
            rows.push(AttachmentEvidence {
                item_key: &record.item_key,
                metadata_version: record.metadata_response.version,
                content_response: &record.content_response,
            });
        }
    }
    let view = ReviewView {
        view_kind: "full_text_review_view_v1",
        capture_digest: &capture.capture_digest,
        metadata_report_digest: &capture.capture_evidence.metadata_report_digest,
        proposal_digest: crate::classification_proposal_digest(report),
        bibliographic_item_count: report.classified_items.len(),
        review_batch,
        attachment_evidence,
    };
    // A fixed slice bounds serialization itself, including JSON escaping.
    let mut bytes = vec![0; 16 * 1024 * 1024];
    let mut writer = std::io::Cursor::new(bytes.as_mut_slice());
    serde_json::to_writer(&mut writer, &view)
        .map_err(|_| FullTextError("full-text review exceeds the 16 MiB output limit"))?;
    let length = writer.position() as usize;
    bytes.truncate(length);
    Ok(bytes)
}

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
    verify_capture_with_persisted_limit(capture, report, MAX_PERSISTED_CAPTURE_BYTES)
}

fn verify_capture_with_persisted_limit(
    capture: &FullTextCapture,
    report: &ClassificationReport,
    max_persisted_bytes: u64,
) -> Result<(), FullTextError> {
    let snapshot = validate_report(report)?;
    validate_persisted_capture_size(capture, max_persisted_bytes)?;
    let evidence = &capture.capture_evidence;
    if evidence.records.len() > snapshot.len() {
        return Err(INVALID_EVIDENCE);
    }
    let mut remaining = MAX_SNAPSHOT_BYTES;
    for response in [
        &evidence.library_before,
        &evidence.manifest_before,
        &evidence.manifest_after,
        &evidence.library_after,
    ]
    .into_iter()
    .chain(
        evidence
            .records
            .iter()
            .flat_map(|record| [&record.metadata_response, &record.content_response]),
    ) {
        account_body(&mut remaining, response)?;
    }
    if evidence.capture_kind != CAPTURE_KIND
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
    for (record, (item_key, version)) in evidence.records.iter().zip(&manifest) {
        if &record.item_key != item_key {
            return Err(INVALID_EVIDENCE);
        }
        validate_metadata(&record.metadata_response, snapshot[item_key.as_str()])?;
        validate_content(&record.content_response, *version)?;
    }
    if capture.capture_digest != json_digest(evidence) {
        return Err(INVALID_EVIDENCE);
    }
    Ok(())
}

fn capture_with(
    report: &ClassificationReport,
    max_bytes: u64,
    fetch: &mut dyn FnMut(&str, u64) -> Result<CapturedResponse, FullTextError>,
) -> Result<FullTextCapture, FullTextError> {
    let started = Instant::now();
    capture_with_clock(report, max_bytes, fetch, &mut || {
        (SystemTime::now(), started.elapsed())
    })
}

fn capture_with_clock(
    report: &ClassificationReport,
    max_bytes: u64,
    fetch: &mut dyn FnMut(&str, u64) -> Result<CapturedResponse, FullTextError>,
    observe_time: &mut dyn FnMut() -> (SystemTime, Duration),
) -> Result<FullTextCapture, FullTextError> {
    let snapshot = validate_report(report)?;
    let started_unix_ms = unix_millis(observe_time().0)?;
    let mut remaining = max_bytes.min(MAX_SNAPSHOT_BYTES);
    let mut read = |request_path: &str| {
        check_admission(remaining, observe_time().1)?;
        let response = fetch(request_path, remaining.min(MAX_PAGE_BYTES))?;
        account_body(&mut remaining, &response)?;
        check_deadline(observe_time().1)?;
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
        finished_unix_ms: unix_millis(observe_time().0)?,
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
    check_deadline(observe_time().1)?;
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

struct SizeLimitedWriter {
    written: u64,
    max_bytes: u64,
}

impl Write for SizeLimitedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let length = bytes.len() as u64;
        if length > self.max_bytes - self.written {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "full-text capture exceeds the persisted size limit",
            ));
        }
        self.written += length;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn validate_persisted_capture_size(
    capture: &FullTextCapture,
    max_bytes: u64,
) -> Result<(), FullTextError> {
    let mut writer = SizeLimitedWriter {
        written: 0,
        max_bytes,
    };
    serde_json::to_writer(&mut writer, capture).map_err(|_| BUDGET_EXCEEDED)
}

fn json_digest(value: &impl Serialize) -> String {
    let mut digest = Sha256::new();
    serde_json::to_writer(&mut digest, value).expect("capture values are JSON-compatible");
    format!("sha256:{:x}", digest.finalize())
}

fn fetch_response(
    agent: &ureq::Agent,
    report: &ClassificationReport,
    api_root: &str,
    request_path: &str,
    limit: u64,
) -> Result<CapturedResponse, FullTextError> {
    let expected_server = report.server_id.as_deref().ok_or(INVALID_EVIDENCE)?;
    let mut response = agent
        .get(&format!("{api_root}/api/users/0/{request_path}"))
        .header("Zotero-API-Version", "3")
        .header("Zotero-Server-ID", expected_server)
        .call()
        .map_err(|_| FullTextError("full-text local request failed"))?;
    let headers = response.headers();
    verify_server_id(headers, expected_server).map_err(|_| INVALID_EVIDENCE)?;
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
#[test]
fn persisted_capture_limit_counts_outer_json_escaping_at_the_exact_boundary() {
    let content_body = serde_json::to_string(&serde_json::json!({
        "content": "\\".repeat(1850)
    }))
    .unwrap();
    let manifest_body = r#"{"ABCD2345":1}"#.to_owned();
    let metadata_body =
        r#"{"key":"ABCD2345","version":1,"data":{"itemType":"attachment"}}"#.to_owned();
    let capture_evidence = CaptureEvidence {
        capture_kind: CAPTURE_KIND.into(),
        metadata_report_digest: format!("sha256:{}", "a".repeat(64)),
        metadata_snapshot_digest: format!("sha256:{}", "b".repeat(64)),
        bibliographic_item_count: 1,
        started_unix_ms: 0,
        finished_unix_ms: 0,
        library_before: CapturedResponse {
            status: 200,
            version: Some(1),
            body: "[]".into(),
        },
        manifest_before: CapturedResponse {
            status: 200,
            version: None,
            body: manifest_body.clone(),
        },
        records: vec![CapturedItem {
            item_key: "ABCD2345".into(),
            metadata_response: CapturedResponse {
                status: 200,
                version: Some(1),
                body: metadata_body,
            },
            content_response: CapturedResponse {
                status: 200,
                version: Some(1),
                body: content_body,
            },
        }],
        manifest_after: CapturedResponse {
            status: 200,
            version: None,
            body: manifest_body,
        },
        library_after: CapturedResponse {
            status: 200,
            version: Some(1),
            body: "[]".into(),
        },
    };
    let capture = FullTextCapture {
        capture_digest: json_digest(&capture_evidence),
        capture_evidence,
    };
    let raw_body_bytes: usize = [
        &capture.capture_evidence.library_before,
        &capture.capture_evidence.manifest_before,
        &capture.capture_evidence.manifest_after,
        &capture.capture_evidence.library_after,
    ]
    .into_iter()
    .chain(
        capture
            .capture_evidence
            .records
            .iter()
            .flat_map(|record| [&record.metadata_response, &record.content_response]),
    )
    .map(|response| response.body.len())
    .sum();
    let serialized_bytes = serde_json::to_vec(&capture).unwrap().len() as u64;
    assert!(raw_body_bytes <= 4096);
    assert!(serialized_bytes > 8192);
    assert_eq!(
        validate_persisted_capture_size(&capture, 8192),
        Err(BUDGET_EXCEEDED)
    );
    validate_persisted_capture_size(&capture, serialized_bytes).unwrap();
    SizeLimitedWriter {
        written: 0,
        max_bytes: 0,
    }
    .flush()
    .unwrap();
}

#[cfg(test)]
#[path = "full_text_capture_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "full_text_capture_transport_tests.rs"]
mod transport_tests;
