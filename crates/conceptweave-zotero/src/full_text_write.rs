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
/// use conceptweave_zotero::FullTextWritePlan;
/// let forged = serde_json::from_str::<FullTextWritePlan>("{}");
/// ```
///
/// ```compile_fail
/// use conceptweave_zotero::{FullTextWritePlan, ClassificationWritePlan};
/// fn detach(plan: &FullTextWritePlan) -> &ClassificationWritePlan {
///     &plan.write_plan
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
    #[serde(skip_serializing_if = "Option::is_none")]
    indeterminate_request: Option<ClassificationWriteRequest>,
}

/// A later, unverified observation attached to the complete original write receipt.
///
/// Matching metadata does not establish who changed it or whether an earlier
/// request has finished. This evidence neither clears uncertainty nor authorizes
/// retry or rollback. Save it only as an owner-only artifact; keep earlier views.
///
/// ```compile_fail
/// use conceptweave_zotero::FullTextWriteObservation;
/// let forged = serde_json::from_str::<FullTextWriteObservation<'_>>("{}");
/// ```
#[derive(Serialize)]
pub struct FullTextWriteObservation<'receipt> {
    write_receipt: &'receipt FullTextWriteReceipt,
    observed_state: Option<ClassificationItemState>,
}

/// One bound rollback attempt, including operations still awaiting resolution.
/// Keep earlier receipts: this outcome reports this attempt, not a new approval.
///
/// ```compile_fail
/// use conceptweave_zotero::FullTextRollbackReceipt;
/// fn mix(left: &mut FullTextRollbackReceipt, right: &FullTextRollbackReceipt) {
///     left.rollback_result.remaining_operations = right.rollback_result.remaining_operations.clone();
/// }
/// ```
#[derive(Serialize)]
pub struct FullTextRollbackReceipt {
    full_text_write_v1: FullTextWriteBinding,
    rollback_result: crate::ClassificationRollbackReceipt,
}

/// Read-only reconciliation retaining the same scope and untouched recovery work.
/// The complete preceding receipt stays attached; observation never resolves causality.
#[derive(Serialize)]
pub struct FullTextRollbackReconciliationReceipt<'receipt> {
    rollback_receipt: &'receipt FullTextRollbackReceipt,
    full_text_write_v1: FullTextWriteBinding,
    reconciliation_result: crate::ClassificationRollbackReconciliationReceipt,
    remaining_operations: Vec<crate::ClassificationRollbackOperation>,
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
    let write_result =
        crate::execute_classification_write_plan(&plan.write_plan, preflight, write_item);
    let indeterminate_request = write_result.indeterminate_request.clone();
    FullTextWriteReceipt {
        full_text_write_v1: plan.full_text_write_v1.clone(),
        write_result,
        indeterminate_request,
    }
}

/// Reads an indeterminate original request's item once, without sending a write.
///
/// Other receipts fail before I/O. A failed read records no observation and omits
/// the adapter error. Even foreign or malformed returned state remains unverified
/// evidence: the original outcome, inverse work and untouched items never change.
pub fn observe_full_text_write<ReadError>(
    receipt: &FullTextWriteReceipt,
    read_item: impl FnOnce(&str) -> Result<ClassificationItemState, ReadError>,
) -> Result<FullTextWriteObservation<'_>, FullTextError> {
    let request = receipt
        .indeterminate_request
        .as_ref()
        .ok_or(INVALID_WRITE_SCOPE)?;
    Ok(FullTextWriteObservation {
        write_receipt: receipt,
        observed_state: read_item(&request.item_key).ok(),
    })
}

/// Restores verified writes without accepting detached or mixed inverse operations.
/// Unknown write state, dry-run and an empty inverse set fail before any I/O.
/// An unknown write requires separate write reconciliation, not an empty rollback.
pub fn execute_full_text_rollback<ReadError, WriteError>(
    receipt: &FullTextWriteReceipt,
    preflight: impl FnMut(&str) -> Result<ClassificationItemState, ReadError>,
    write_item: impl FnMut(&ClassificationWriteRequest) -> Result<ClassificationItemState, WriteError>,
) -> Result<FullTextRollbackReceipt, FullTextError> {
    if receipt.full_text_write_v1.mode != WriteMode::Execute
        || receipt.write_result.indeterminate_item_key.is_some()
        || receipt.write_result.rollback_operations.is_empty()
    {
        return Err(INVALID_WRITE_SCOPE);
    }
    Ok(FullTextRollbackReceipt {
        full_text_write_v1: receipt.full_text_write_v1.clone(),
        rollback_result: crate::execute_classification_rollback(
            &receipt.write_result.rollback_operations,
            preflight,
            write_item,
        ),
    })
}

/// Retries only the unchanged or unattempted work retained by one bound receipt.
/// An unresolved operation must be reconciled first; it cannot be silently dropped.
pub fn retry_full_text_rollback<ReadError, WriteError>(
    receipt: &FullTextRollbackReceipt,
    preflight: impl FnMut(&str) -> Result<ClassificationItemState, ReadError>,
    write_item: impl FnMut(&ClassificationWriteRequest) -> Result<ClassificationItemState, WriteError>,
) -> Result<FullTextRollbackReceipt, FullTextError> {
    if receipt.rollback_result.indeterminate_operation.is_some()
        || receipt.rollback_result.remaining_operations.is_empty()
    {
        return Err(INVALID_WRITE_SCOPE);
    }
    Ok(FullTextRollbackReceipt {
        full_text_write_v1: receipt.full_text_write_v1.clone(),
        rollback_result: crate::execute_classification_rollback(
            &receipt.rollback_result.remaining_operations,
            preflight,
            write_item,
        ),
    })
}

/// Observes the unresolved operation in a bound rollback receipt, without writing.
/// The untouched tail stays attached so resolving one item cannot discard it.
pub fn reconcile_full_text_rollback<ReadError>(
    receipt: &FullTextRollbackReceipt,
    read_item: impl FnOnce(&str) -> Result<ClassificationItemState, ReadError>,
) -> Result<FullTextRollbackReconciliationReceipt<'_>, FullTextError> {
    let operation = receipt
        .rollback_result
        .indeterminate_operation
        .as_ref()
        .ok_or(INVALID_WRITE_SCOPE)?;
    Ok(FullTextRollbackReconciliationReceipt {
        rollback_receipt: receipt,
        full_text_write_v1: receipt.full_text_write_v1.clone(),
        reconciliation_result: crate::reconcile_classification_rollback(operation, read_item),
        remaining_operations: receipt.rollback_result.remaining_operations.clone(),
    })
}

/// Retries a resolved operation and its untouched tail through complete preflight.
/// Already restored work is not written again; unresolved work makes zero I/O.
pub fn retry_full_text_reconciled_rollback<ReadError, WriteError>(
    receipt: &FullTextRollbackReconciliationReceipt<'_>,
    preflight: impl FnMut(&str) -> Result<ClassificationItemState, ReadError>,
    write_item: impl FnMut(&ClassificationWriteRequest) -> Result<ClassificationItemState, WriteError>,
) -> Result<FullTextRollbackReceipt, FullTextError> {
    if receipt.reconciliation_result.state == crate::ClassificationRollbackState::Indeterminate {
        return Err(INVALID_WRITE_SCOPE);
    }
    let operations = receipt
        .reconciliation_result
        .retry_operation
        .iter()
        .cloned()
        .chain(receipt.remaining_operations.iter().cloned())
        .collect::<Vec<_>>();
    Ok(FullTextRollbackReceipt {
        full_text_write_v1: receipt.full_text_write_v1.clone(),
        rollback_result: crate::execute_classification_rollback(&operations, preflight, write_item),
    })
}
