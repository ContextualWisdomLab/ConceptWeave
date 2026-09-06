use super::*;
use crate::{
    FullTextWriteScope, WriteMode, build_full_text_write_plan, execute_full_text_write_plan,
};
use std::cell::Cell;

fn write_scope_fixture(report: &ClassificationReport, capture: &FullTextCapture) -> FullTextWriteScope {
    let worksheet = build_full_text_review_worksheet(report, capture).unwrap();
    let completed = completed_full_text_view(report, &worksheet, capture, 2);
    let decided = apply_full_text_review_view(report, &worksheet, capture, &completed).unwrap();
    let full_text_review = finalize_full_text_review(
        report, &decided, capture, full_text_approval_fixture(report, capture),
    ).unwrap();
    let reviewed_writes = crate::ReviewedClassificationWriteSet {
        review_id: "synthetic-write-review".into(),
        authority_receipt: "synthetic-write-authority".into(),
        server_id: report.server_id.clone(),
        zotero_version: report.zotero_version.clone(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.clone(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        changes: report.classified_items.iter().map(|item| crate::ReviewedClassificationChange {
            item_key: item.item_key.clone(),
            item_version: item.item_version,
            reviewed_disposition: crate::Disposition::OutOfScope,
            before_collection_keys: item.collection_keys.clone(),
            after_collection_keys: vec!["EXPLICIT_COLLECTION".into()],
            before_tags: item.tags.clone(),
            after_tags: item.tags.clone(),
        }).collect(),
    };
    FullTextWriteScope { full_text_review, reviewed_writes, mode: WriteMode::DryRun }
}

#[test]
fn full_text_write_validates_both_inputs_before_either_authority() {
    for scenario in 0..7 {
        let report = report_fixture();
        let capture = capture_with(&report, 4096, &mut |request_path, _| Ok(response_fixture(request_path))).unwrap();
        let mut scope = write_scope_fixture(&report, &capture);
        match scenario {
            0 => scope.reviewed_writes.changes[1].after_collection_keys = vec![" ".into()],
            1 => scope.reviewed_writes.changes[1].reviewed_disposition = crate::Disposition::SemanticConsumptionBridge,
            2 => scope.reviewed_writes.snapshot_digest = "changed".into(),
            3 => scope.reviewed_writes.changes[1].item_version += 1,
            _ => {
                let mut value = serde_json::to_value(&scope.full_text_review).unwrap();
                match scenario {
                    4 => { value["full_text_golden_set_v1"]["labels"].as_array_mut().unwrap().pop(); }
                    5 => value["capture_digest"] = "changed".into(),
                    _ => value["full_text_golden_set_v1"]["approval"]["proposal_digest"] = "changed".into(),
                }
                scope.full_text_review = serde_json::from_value(value).unwrap();
            }
        }
        let calls = Cell::new(0);
        assert!(build_full_text_write_plan(&report, &capture, scope,
            |_| { calls.set(calls.get()+1); true },
            |_| { calls.set(calls.get()+1); true }).is_err(), "scenario {scenario}");
        assert_eq!(calls.get(), 0, "scenario {scenario}");
    }
}

#[test]
fn full_text_write_verifiers_receive_exact_scope_and_mode() {
    for mode in [WriteMode::DryRun, WriteMode::Execute] {
        for semantic_allowed in [false, true] {
            for write_allowed in [false, true] {
                let report = report_fixture();
                let capture = capture_with(&report, 4096, &mut |request_path, _| Ok(response_fixture(request_path))).unwrap();
                let mut scope = write_scope_fixture(&report, &capture);
                scope.mode = mode;
                let expected = serde_json::to_value(&scope).unwrap();
                let semantic_calls = Cell::new(0);
                let write_calls = Cell::new(0);
                let result = build_full_text_write_plan(&report, &capture, scope, |actual| {
                    semantic_calls.set(semantic_calls.get()+1);
                    assert_eq!(serde_json::to_value(actual).unwrap(), expected["full_text_review"]);
                    semantic_allowed
                }, |actual| {
                    write_calls.set(write_calls.get()+1);
                    assert_eq!(serde_json::to_value(actual).unwrap(), expected);
                    write_allowed
                });
                assert_eq!(result.is_ok(), semantic_allowed && write_allowed);
                assert_eq!(semantic_calls.get(), 1);
                assert_eq!(write_calls.get(), usize::from(semantic_allowed));
            }
        }
    }
}

#[test]
fn full_text_write_dry_run_preserves_binding_without_reads_or_writes() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| Ok(response_fixture(request_path))).unwrap();
    let scope = write_scope_fixture(&report, &capture);
    let plan = build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
    let receipt = execute_full_text_write_plan(&plan,
        |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run read") },
        |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run write") });
    let plan_json = serde_json::to_value(&plan).unwrap();
    let receipt_json = serde_json::to_value(receipt).unwrap();
    assert_eq!(receipt_json["full_text_write_v1"], plan_json["full_text_write_v1"]);
    assert_eq!(receipt_json["write_result"]["outcome"], "dry_run");
    assert!(!receipt_json.to_string().contains("fixture text"));
    assert!(!receipt_json.to_string().contains("synthetic-write-authority"));
}
