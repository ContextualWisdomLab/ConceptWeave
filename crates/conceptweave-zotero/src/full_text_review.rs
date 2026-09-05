use super::*;
use crate::StewardReviewWorksheet;

/// Independently issued approval input for one full-text review context.
/// The issuer must bind the complete reviewed labels as well as this capture.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FullTextReviewApproval {
    capture_digest: String,
    #[serde(rename = "full_text_approval_v1")]
    review_approval: crate::GoldenSetApproval,
}

/// Completed labels awaiting verification of the entire owner-only envelope.
/// Serialization is available to governance; no metadata-only downcast is offered.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FullTextReviewedGoldenSet {
    capture_digest: String,
    #[serde(rename = "full_text_golden_set_v1")]
    reviewed_golden_set: crate::ReviewedGoldenSet,
}

/// Aggregate evaluation that retains the verified capture identity without text.
#[derive(Serialize)]
pub struct FullTextReviewEvaluation {
    capture_digest: String,
    #[serde(rename = "full_text_evaluation_v1")]
    review_evaluation: crate::GoldenSetEvaluation,
}

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

/// Shows the next pending rows only after revalidating the worksheet's capture.
pub fn build_bound_full_text_review_json(
    _report: &ClassificationReport,
    _worksheet: &FullTextReviewWorksheet,
    _capture: &FullTextCapture,
    _limit: usize,
) -> Result<Vec<u8>, FullTextError> {
    Err(INVALID_EVIDENCE)
}

/// Applies only completed decision slots in an otherwise unchanged evidence view.
/// Stale views fail without changing the input; preserve it and request a new view.
pub fn apply_full_text_review_view(
    _report: &ClassificationReport,
    _worksheet: &FullTextReviewWorksheet,
    _capture: &FullTextCapture,
    _completed_view: &[u8],
) -> Result<FullTextReviewWorksheet, FullTextError> {
    Err(INVALID_EVIDENCE)
}

/// Prepares a fully decided review for external verification, never issuing approval.
/// The approval must already name this capture; legacy receipts are not upgraded.
pub fn finalize_full_text_review(
    _report: &ClassificationReport,
    _worksheet: &FullTextReviewWorksheet,
    _capture: &FullTextCapture,
    _approval: FullTextReviewApproval,
) -> Result<FullTextReviewedGoldenSet, FullTextError> {
    Err(INVALID_EVIDENCE)
}

/// Evaluates all papers after local validation and verification of the whole envelope.
/// The verifier must authenticate capture identity and every label against an
/// independently issued receipt. JSON restoration or matching digests alone do
/// not prove human review. No result from this function authorizes Zotero writes.
pub fn evaluate_full_text_review<F>(
    _report: &ClassificationReport,
    _capture: &FullTextCapture,
    _reviewed: &FullTextReviewedGoldenSet,
    _verify_approval: F,
) -> Result<FullTextReviewEvaluation, FullTextError>
where
    F: FnOnce(&FullTextReviewedGoldenSet) -> bool,
{
    Err(INVALID_EVIDENCE)
}
