//! Private, content-bound full-text read sweeps, separate from classification approval.

use crate::ClassificationReport;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    _report: &ClassificationReport,
) -> Result<FullTextCapture, FullTextError> {
    Err(FullTextError(
        "full-text capture requires a bound Zotero 10+ report",
    ))
}

/// Verifies a restored capture's complete content and original report binding.
///
/// This detects altered artifacts under an unchanged digest. It does not grant
/// approval, establish atomicity, or authenticate a locally replaced digest.
pub fn verify_full_text_capture(
    _capture: &FullTextCapture,
    _report: &ClassificationReport,
) -> Result<(), FullTextError> {
    Err(FullTextError("full-text capture evidence is invalid"))
}

fn capture_with(
    _report: &ClassificationReport,
    _max_bytes: u64,
    _fetch: &mut dyn FnMut(&str, u64) -> Result<CapturedResponse, FullTextError>,
) -> Result<FullTextCapture, FullTextError> {
    Err(FullTextError("full-text capture evidence is invalid"))
}

#[cfg(test)]
#[path = "full_text_capture_tests.rs"]
mod tests;
