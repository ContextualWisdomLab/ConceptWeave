use super::*;
use crate::{
    ClassificationItemState, ClassificationWritePlan, ClassificationWriteReceipt,
    ClassificationWriteRequest, ReviewedClassificationWriteSet, WriteMode,
};

const INVALID_WRITE_SCOPE: FullTextError =
    FullTextError("full-text change request is invalid or unverified");

/// Complete typed input for independent meaning and destination authorization.
///
/// The write verifier must authenticate every field, including mode, against
/// independently issued authority. Receipt strings are opaque references, never
/// API keys or bearer credentials. Persist this only as an owner-only artifact.
/// There is no executable JSON deserializer: legacy nested DTOs are permissive.
#[derive(Serialize)]
pub struct FullTextWriteScope {
    /// Every reviewed label and its capture-bound approval input.
    pub full_text_review: FullTextReviewedGoldenSet,
    /// Explicit complete before/after metadata, separately authorized for writing.
    pub reviewed_writes: ReviewedClassificationWriteSet,
    /// Behavior authenticated by the write verifier; meaning does not select it.
    pub mode: WriteMode,
}

#[derive(Clone, Serialize)]
struct FullTextWriteBinding {
    scope_digest: String,
    capture_digest: String,
    proposal_digest: String,
    snapshot_digest: String,
    mode: WriteMode,
}

/// Verified plan retaining the complete approved input and its versioned identity.
///
/// The legacy plan stays private; serialized output is audit evidence, not an
/// executable restoration. Retain the owner-only scope to verify receipt hashes.
///
/// ```compile_fail
/// use conceptweave_zotero::{FullTextWritePlan, execute_classification_write_plan};
/// fn detach(plan: &FullTextWritePlan) {
///     execute_classification_write_plan(&plan.write_plan,
///         |_| Ok::<_, ()>(unreachable!()), |_| Ok::<_, ()>(unreachable!()));
/// }
/// ```
#[derive(Serialize)]
pub struct FullTextWritePlan {
    full_text_write_v1: FullTextWriteBinding,
    approved_scope: FullTextWriteScope,
    #[serde(skip)]
    write_plan: ClassificationWritePlan,
}

/// Bound write outcome; no public inverse-operation projection can detach it.
/// Source text, reviewer identity, authority inputs and adapter errors are omitted.
#[derive(Serialize)]
pub struct FullTextWriteReceipt {
    full_text_write_v1: FullTextWriteBinding,
    #[serde(serialize_with = "serialize_write_result")]
    write_result: ClassificationWriteReceipt,
}

fn serialize_write_result<S: serde::Serializer>(
    receipt: &ClassificationWriteReceipt,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut audit_value =
        serde_json::to_value(receipt).expect("write receipts contain JSON values");
    let audit_fields = audit_value
        .as_object_mut()
        .expect("write receipts are objects");
    audit_fields.remove("authority_receipt");
    audit_fields.remove("review_id");
    audit_value.serialize(serializer)
}

/// Validates the complete evidence and change scope before calling either verifier.
///
/// A denied meaning review never invokes the write verifier. A valid request
/// reaches each exactly once; neither callback is an approval issuer. All changed
/// labels must match the complete review, and destinations are never inferred.
pub fn build_full_text_write_plan(
    report: &ClassificationReport,
    capture: &FullTextCapture,
    scope: FullTextWriteScope,
    verify_meaning: impl FnOnce(&FullTextReviewedGoldenSet) -> bool,
    verify_writes: impl FnOnce(&FullTextWriteScope) -> bool,
) -> Result<FullTextWritePlan, FullTextError> {
    let evaluation = prepare_full_text_review(report, capture, &scope.full_text_review)?;
    let write_plan =
        crate::prepare_classification_write_plan(report, &scope.reviewed_writes, scope.mode)
            .map_err(|_| INVALID_WRITE_SCOPE)?;
    let labels = scope
        .full_text_review
        .reviewed_golden_set
        .labels
        .iter()
        .map(|label| (label.item_key.as_str(), label.expected_disposition))
        .collect::<BTreeMap<_, _>>();
    if scope
        .reviewed_writes
        .changes
        .iter()
        .any(|change| labels.get(change.item_key.as_str()) != Some(&change.reviewed_disposition))
    {
        return Err(INVALID_WRITE_SCOPE);
    }
    let scope_bytes = serde_json::to_vec(&("conceptweave-full-text-write-v1", &scope))
        .expect("typed full-text write scopes contain JSON values");
    let binding = FullTextWriteBinding {
        scope_digest: format!("sha256:{:x}", Sha256::digest(scope_bytes)),
        capture_digest: scope.full_text_review.capture_digest.clone(),
        proposal_digest: evaluation.proposal_digest,
        snapshot_digest: evaluation.snapshot_digest,
        mode: scope.mode,
    };
    if !verify_meaning(&scope.full_text_review) || !verify_writes(&scope) {
        return Err(INVALID_WRITE_SCOPE);
    }
    Ok(FullTextWritePlan {
        full_text_write_v1: binding,
        approved_scope: scope,
        write_plan,
    })
}

/// Runs the existing complete-preflight executor and attaches the admitted scope.
/// Dry-run performs no reads or writes. Authentication remains caller-owned.
pub fn execute_full_text_write_plan<ReadError, WriteError>(
    plan: &FullTextWritePlan,
    preflight: impl FnMut(&str) -> Result<ClassificationItemState, ReadError>,
    write_item: impl FnMut(&ClassificationWriteRequest) -> Result<ClassificationItemState, WriteError>,
) -> FullTextWriteReceipt {
    FullTextWriteReceipt {
        full_text_write_v1: plan.full_text_write_v1.clone(),
        write_result: crate::execute_classification_write_plan(
            &plan.write_plan,
            preflight,
            write_item,
        ),
    }
}
