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
    let review_worksheet = build_steward_review_worksheet(report).map_err(|_| INVALID_EVIDENCE)?;
    verify_full_text_capture(capture, report)?;
    Ok(FullTextReviewWorksheet {
        capture_digest: capture.capture_digest.clone(),
        review_worksheet,
    })
}

/// Shows the next pending rows only after revalidating the worksheet's capture.
pub fn build_bound_full_text_review_json(
    report: &ClassificationReport,
    worksheet: &FullTextReviewWorksheet,
    capture: &FullTextCapture,
    limit: usize,
) -> Result<Vec<u8>, FullTextError> {
    validate_review_capture(report, capture, &worksheet.capture_digest)?;
    build_full_text_review_json(report, &worksheet.review_worksheet, capture, limit)
}

/// Applies only completed decision slots in an otherwise unchanged evidence view.
/// Stale views fail without changing the input; preserve it and request a new view.
pub fn apply_full_text_review_view(
    report: &ClassificationReport,
    worksheet: &FullTextReviewWorksheet,
    capture: &FullTextCapture,
    completed_view: &[u8],
) -> Result<FullTextReviewWorksheet, FullTextError> {
    validate_review_capture(report, capture, &worksheet.capture_digest)?;
    let mut completed =
        unique_review_json::parse_review_json(completed_view).map_err(|_| INVALID_EVIDENCE)?;
    let batch: crate::StewardReviewBatch = serde_json::from_value(
        completed
            .get("review_batch")
            .ok_or(INVALID_EVIDENCE)?
            .clone(),
    )
    .map_err(|_| INVALID_EVIDENCE)?;
    let patch =
        crate::decision_patch_from_review_batch(report, &worksheet.review_worksheet, &batch)
            .map_err(|_| INVALID_EVIDENCE)?;
    let expected: serde_json::Value = serde_json::from_slice(&build_full_text_review_json(
        report,
        &worksheet.review_worksheet,
        capture,
        batch.decisions.len(),
    )?)
    .expect("generated full-text view contains valid JSON");
    // The metadata boundary above already checked every displayed batch field.
    // Replace only that verified batch before comparing the retained text envelope.
    completed["review_batch"] = expected["review_batch"].clone();
    if completed != expected {
        return Err(INVALID_EVIDENCE);
    }
    let review_worksheet =
        crate::apply_steward_decision_patch(report, &worksheet.review_worksheet, &patch).expect(
            "a validated pending review patch cannot conflict with its unchanged worksheet",
        );
    Ok(FullTextReviewWorksheet {
        capture_digest: worksheet.capture_digest.clone(),
        review_worksheet,
    })
}

/// Prepares a fully decided review for external verification, never issuing approval.
/// The approval must already name this capture; legacy receipts are not upgraded.
pub fn finalize_full_text_review(
    report: &ClassificationReport,
    worksheet: &FullTextReviewWorksheet,
    capture: &FullTextCapture,
    approval: FullTextReviewApproval,
) -> Result<FullTextReviewedGoldenSet, FullTextError> {
    validate_review_capture(report, capture, &worksheet.capture_digest)?;
    if approval.capture_digest != worksheet.capture_digest {
        return Err(INVALID_EVIDENCE);
    }
    let reviewed_golden_set = crate::reviewed_golden_set_from_worksheet(
        report,
        &worksheet.review_worksheet,
        approval.review_approval,
    )
    .map_err(|_| INVALID_EVIDENCE)?;
    Ok(FullTextReviewedGoldenSet {
        capture_digest: worksheet.capture_digest.clone(),
        reviewed_golden_set,
    })
}

/// Evaluates all papers after local validation and verification of the whole envelope.
/// The verifier must authenticate capture identity and every label against an
/// independently issued receipt. JSON restoration or matching digests alone do
/// not prove human review. No result from this function authorizes Zotero writes.
pub fn evaluate_full_text_review<F>(
    report: &ClassificationReport,
    capture: &FullTextCapture,
    reviewed: &FullTextReviewedGoldenSet,
    verify_approval: F,
) -> Result<FullTextReviewEvaluation, FullTextError>
where
    F: FnOnce(&FullTextReviewedGoldenSet) -> bool,
{
    validate_review_capture(report, capture, &reviewed.capture_digest)?;
    let review_evaluation = crate::evaluate_complete_reviewed_classification(
        report,
        &reviewed.reviewed_golden_set,
        |_| verify_approval(reviewed),
    )
    .map_err(|_| FullTextError("full-text review is invalid or unverified"))?;
    Ok(FullTextReviewEvaluation {
        capture_digest: reviewed.capture_digest.clone(),
        review_evaluation,
    })
}

fn validate_review_capture(
    report: &ClassificationReport,
    capture: &FullTextCapture,
    capture_digest: &str,
) -> Result<(), FullTextError> {
    if capture_digest != capture.capture_digest {
        return Err(INVALID_EVIDENCE);
    }
    verify_full_text_capture(capture, report)
}

#[path = "unique_review_json.rs"]
mod unique_review_json;
