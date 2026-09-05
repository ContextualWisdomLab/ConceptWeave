use super::*;
use crate::StewardReviewWorksheet;

/// Private single-capture review work, distinct from a metadata-only worksheet.
///
/// Serialize this owner-only artifact for storage. Restoring it grants no
/// authority; each operation must revalidate the original report and capture.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FullTextReviewWorksheet {
    capture_digest: String,
    #[serde(rename = "full_text_worksheet_v1")]
    review_worksheet: StewardReviewWorksheet,
}

/// Starts an entirely blank review bound to the verified retained full text.
/// Existing metadata decisions are deliberately not imported as text-reviewed.
pub fn build_full_text_review_worksheet(
    report: &ClassificationReport,
    capture: &FullTextCapture,
) -> Result<FullTextReviewWorksheet, FullTextError> {
    verify_full_text_capture(capture, report)?;
    Ok(FullTextReviewWorksheet {
        capture_digest: capture.capture_digest.clone(),
        review_worksheet: build_steward_review_worksheet(report).map_err(|_| INVALID_EVIDENCE)?,
    })
}
