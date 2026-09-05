use super::*;

/// Private single-capture review work, distinct from a metadata-only worksheet.
///
/// Serialize this owner-only artifact for storage. Restoring it grants no
/// authority; each operation must revalidate the original report and capture.
#[derive(Deserialize, Serialize)]
#[serde(
    tag = "artifact_kind",
    rename = "full_text_review_worksheet_v1",
    deny_unknown_fields
)]
pub struct FullTextReviewWorksheet {
    capture_digest: String,
    review_worksheet: StewardReviewWorksheet,
}

/// Starts an entirely blank review bound to the verified retained full text.
/// Existing metadata decisions are deliberately not imported as text-reviewed.
pub fn build_full_text_review_worksheet(
    _report: &ClassificationReport,
    _capture: &FullTextCapture,
) -> Result<FullTextReviewWorksheet, FullTextError> {
    Err(FullTextError(INVALID_EVIDENCE))
}
