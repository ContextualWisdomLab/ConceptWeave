use super::*;
use crate::{StewardReviewWorksheet, ZoteroItem, classify_snapshot};

fn completed_full_text_view(
    report: &ClassificationReport,
    worksheet: &FullTextReviewWorksheet,
    capture: &FullTextCapture,
    limit: usize,
) -> Vec<u8> {
    let mut json: serde_json::Value = serde_json::from_slice(
        &build_bound_full_text_review_json(report, worksheet, capture, limit).unwrap(),
    )
    .unwrap();
    for decision in json["review_batch"]["decisions"].as_array_mut().unwrap() {
        decision["reviewed_disposition"] = serde_json::json!(crate::Disposition::OutOfScope);
    }
    serde_json::to_vec(&json).unwrap()
}

fn full_text_approval_fixture(
    report: &ClassificationReport,
    capture: &FullTextCapture,
) -> FullTextReviewApproval {
    serde_json::from_value(serde_json::json!({
        "capture_digest":capture.capture_digest,
        "full_text_approval_v1":{
            "receipt_id":"synthetic-review-receipt",
            "reviewer_subject":"synthetic-steward",
            "library_version":report.library_version,
            "rule_revision":report.rule_revision,
            "snapshot_digest":report.snapshot_digest,
            "proposal_digest":crate::classification_proposal_digest(report),
            "snapshot_items":report.snapshot_items,
        }
    }))
    .unwrap()
}

#[test]
fn full_text_review_preserves_context_through_decisions_and_full_approval() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let unchanged = serde_json::to_vec(&worksheet).unwrap();
    let view = completed_full_text_view(&report, &worksheet, &capture, 2);
    let decided = apply_full_text_review_view(&report, &worksheet, &capture, &view).unwrap();
    assert_eq!(serde_json::to_vec(&worksheet).unwrap(), unchanged);
    let golden = finalize_full_text_review(
        &report,
        &decided,
        &capture,
        full_text_approval_fixture(&report, &capture),
    )
    .unwrap();
    let expected = serde_json::to_value(&golden).unwrap();
    assert_eq!(expected["capture_digest"], capture.capture_digest);
    assert!(
        expected["full_text_golden_set_v1"]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .all(|label| label["expected_disposition"]
                == serde_json::json!(crate::Disposition::OutOfScope))
    );
    let evaluation = evaluate_full_text_review(&report, &capture, &golden, |received| {
        serde_json::to_value(received).unwrap() == expected
    })
    .unwrap();
    let result = serde_json::to_value(evaluation).unwrap();
    assert_eq!(result["capture_digest"], capture.capture_digest);
    assert_eq!(result["full_text_evaluation_v1"]["reviewed_count"], 2);
    assert!(evaluate_full_text_review(&report, &capture, &golden, |_| false).is_err());
    assert!(apply_full_text_review_view(&report, &decided, &capture, &view).is_err());
    assert!(serde_json::from_value::<crate::ReviewedGoldenSet>(expected.clone()).is_err());
    assert!(serde_json::from_value::<crate::ReviewedClassificationWriteSet>(expected).is_err());
}

#[test]
fn full_text_review_rejects_modified_display_and_never_overwrites_work() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let completed = completed_full_text_view(&report, &worksheet, &capture, 1);
    let original: serde_json::Value = serde_json::from_slice(&completed).unwrap();
    let before = serde_json::to_vec(&worksheet).unwrap();
    for (pointer, value) in [
        ("/capture_digest", serde_json::json!("changed")),
        ("/metadata_report_digest", serde_json::json!("changed")),
        ("/proposal_digest", serde_json::json!("changed")),
        ("/view_kind", serde_json::json!("changed")),
        ("/bibliographic_item_count", serde_json::json!(1)),
        ("/review_batch/remaining_count", serde_json::json!(1)),
        (
            "/review_batch/decisions/0/title",
            serde_json::json!("changed"),
        ),
        (
            "/review_batch/decisions/0/reviewed_disposition",
            serde_json::Value::Null,
        ),
        (
            "/review_batch/decisions/0/reviewed_disposition",
            serde_json::json!(crate::Disposition::NeedsStewardReview),
        ),
        (
            "/attachment_evidence/ABCD2345/0/content_response/body",
            serde_json::json!("changed"),
        ),
        (
            "/attachment_evidence/ABCD2345/0/content_response/status",
            serde_json::json!(404),
        ),
        (
            "/attachment_evidence/ABCD2345/0/content_response/version",
            serde_json::json!(42),
        ),
        (
            "/attachment_evidence/ABCD2345/0/metadata_version",
            serde_json::json!(42),
        ),
        ("/attachment_evidence/ABCD2345", serde_json::json!([])),
        ("/review_batch/decisions", serde_json::json!([])),
    ] {
        let mut invalid = original.clone();
        *invalid.pointer_mut(pointer).unwrap() = value;
        assert!(
            apply_full_text_review_view(
                &report,
                &worksheet,
                &capture,
                &serde_json::to_vec(&invalid).unwrap()
            )
            .is_err(),
            "{pointer}"
        );
    }
    for pointer in [
        "",
        "/review_batch",
        "/review_batch/decisions/0",
        "/review_batch/decisions/0/evidence",
        "/attachment_evidence/ABCD2345/0",
        "/attachment_evidence/ABCD2345/0/content_response",
    ] {
        let mut invalid = original.clone();
        invalid
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected_field".into(), serde_json::json!(true));
        assert!(
            apply_full_text_review_view(
                &report,
                &worksheet,
                &capture,
                &serde_json::to_vec(&invalid).unwrap()
            )
            .is_err()
        );
    }
    for bytes in [b"null".as_slice(), b"{}", b"{", b"{\"review_batch\":true}"] {
        assert!(apply_full_text_review_view(&report, &worksheet, &capture, bytes).is_err());
    }
    let advanced = apply_full_text_review_view(&report, &worksheet, &capture, &completed).unwrap();
    assert!(apply_full_text_review_view(&report, &advanced, &capture, &completed).is_err());
    assert!(
        finalize_full_text_review(
            &report,
            &advanced,
            &capture,
            full_text_approval_fixture(&report, &capture)
        )
        .is_err()
    );
    let next: serde_json::Value = serde_json::from_slice(
        &build_bound_full_text_review_json(&report, &advanced, &capture, 1).unwrap(),
    )
    .unwrap();
    assert_eq!(next["review_batch"]["remaining_count"], 1);
    assert_eq!(
        next["attachment_evidence"],
        serde_json::json!({"DEFG5678":[]})
    );
    let completed_next = completed_full_text_view(&report, &advanced, &capture, 1);
    let complete =
        apply_full_text_review_view(&report, &advanced, &capture, &completed_next).unwrap();
    assert!(
        finalize_full_text_review(
            &report,
            &complete,
            &capture,
            full_text_approval_fixture(&report, &capture)
        )
        .is_ok()
    );
    assert_eq!(serde_json::to_vec(&worksheet).unwrap(), before);
}

#[test]
fn full_text_review_rejects_duplicate_keys_before_comparing_presented_context() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let completed =
        String::from_utf8(completed_full_text_view(&report, &worksheet, &capture, 2)).unwrap();
    for (original, duplicated) in [
        (
            "\"capture_digest\":",
            "\"capture_digest\":\"changed\",\"capture_digest\":",
        ),
        ("\"item_key\":", "\"item_key\":\"changed\",\"item_key\":"),
        ("\"ABCD2345\":", "\"ABCD2345\":[],\"ABCD2345\":"),
        ("\"body\":", "\"body\":\"changed\",\"body\":"),
    ] {
        assert!(completed.contains(original));
        let invalid = completed.replacen(original, duplicated, 1);
        assert!(
            apply_full_text_review_view(&report, &worksheet, &capture, invalid.as_bytes()).is_err()
        );
    }
}

#[test]
fn full_text_review_revalidates_restored_bindings_before_governance() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let completed = completed_full_text_view(&report, &worksheet, &capture, 2);
    let decided = apply_full_text_review_view(&report, &worksheet, &capture, &completed).unwrap();
    let golden = finalize_full_text_review(
        &report,
        &decided,
        &capture,
        full_text_approval_fixture(&report, &capture),
    )
    .unwrap();
    let expected = serde_json::to_value(&golden).unwrap();
    let mut changed_report = report_fixture();
    changed_report.classified_items[0]
        .title
        .push_str(" changed");
    assert!(
        finalize_full_text_review(
            &changed_report,
            &decided,
            &capture,
            full_text_approval_fixture(&changed_report, &capture)
        )
        .is_err()
    );
    let mut calls = 0;
    assert!(
        evaluate_full_text_review(&changed_report, &capture, &golden, |_| {
            calls += 1;
            true
        })
        .is_err()
    );
    assert_eq!(calls, 0);
    for (pointer, replacement) in [
        ("/capture_digest", serde_json::json!("changed")),
        ("/full_text_golden_set_v1/labels", serde_json::json!([])),
        (
            "/full_text_golden_set_v1/approval/proposal_digest",
            serde_json::json!("changed"),
        ),
    ] {
        let mut invalid = expected.clone();
        *invalid.pointer_mut(pointer).unwrap() = replacement;
        let invalid: FullTextReviewedGoldenSet = serde_json::from_value(invalid).unwrap();
        assert!(
            evaluate_full_text_review(&report, &capture, &invalid, |_| {
                calls += 1;
                true
            })
            .is_err()
        );
    }
    assert_eq!(calls, 0);
    let mut relabeled = expected.clone();
    relabeled["full_text_golden_set_v1"]["labels"][0]["expected_disposition"] =
        serde_json::json!(crate::Disposition::Generation);
    let relabeled = serde_json::from_value(relabeled).unwrap();
    assert!(
        evaluate_full_text_review(
            &report,
            &capture,
            &relabeled,
            |received| serde_json::to_value(received).unwrap() == expected
        )
        .is_err()
    );
    let mut wrong_approval =
        serde_json::to_value(full_text_approval_fixture(&report, &capture)).unwrap();
    wrong_approval["capture_digest"] = serde_json::json!("changed");
    assert!(
        finalize_full_text_review(
            &report,
            &decided,
            &capture,
            serde_json::from_value(wrong_approval).unwrap()
        )
        .is_err()
    );
    let mut invalid_worksheet = serde_json::to_value(&worksheet).unwrap();
    invalid_worksheet["capture_digest"] = serde_json::json!("changed");
    let invalid_worksheet = serde_json::from_value(invalid_worksheet).unwrap();
    assert!(build_bound_full_text_review_json(&report, &invalid_worksheet, &capture, 1).is_err());
    assert!(
        apply_full_text_review_view(&report, &invalid_worksheet, &capture, &completed).is_err()
    );
    assert!(
        finalize_full_text_review(
            &report,
            &invalid_worksheet,
            &capture,
            full_text_approval_fixture(&report, &capture)
        )
        .is_err()
    );
    let mut invalid_report = report_fixture();
    invalid_report.audit_summary.failure_count = 1;
    assert!(build_full_text_review_worksheet(&invalid_report, &capture).is_err());
    assert!(build_full_text_review_worksheet(&changed_report, &capture).is_err());
}

#[test]
fn full_text_owned_artifacts_reject_unknown_fields_without_changing_legacy_validity() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let worksheet_json = serde_json::to_value(&worksheet).unwrap();
    for pointer in [
        "",
        "/full_text_worksheet_v1",
        "/full_text_worksheet_v1/decisions/0",
        "/full_text_worksheet_v1/snapshot_items/0",
    ] {
        let mut invalid = worksheet_json.clone();
        invalid
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected_field".into(), serde_json::json!(true));
        assert!(serde_json::from_value::<FullTextReviewWorksheet>(invalid).is_err());
    }
    let approval_json =
        serde_json::to_value(full_text_approval_fixture(&report, &capture)).unwrap();
    assert!(serde_json::from_value::<crate::GoldenSetApproval>(approval_json.clone()).is_err());
    assert!(
        serde_json::from_value::<FullTextReviewApproval>(
            approval_json["full_text_approval_v1"].clone()
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<crate::GoldenSetApproval>(
            approval_json["full_text_approval_v1"].clone()
        )
        .is_ok()
    );
    for pointer in [
        "",
        "/full_text_approval_v1",
        "/full_text_approval_v1/snapshot_items/0",
    ] {
        let mut invalid = approval_json.clone();
        invalid
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected_field".into(), serde_json::json!(true));
        assert!(serde_json::from_value::<FullTextReviewApproval>(invalid).is_err());
    }
    let completed = completed_full_text_view(&report, &worksheet, &capture, 2);
    let decided = apply_full_text_review_view(&report, &worksheet, &capture, &completed).unwrap();
    let golden = finalize_full_text_review(
        &report,
        &decided,
        &capture,
        full_text_approval_fixture(&report, &capture),
    )
    .unwrap();
    for pointer in [
        "",
        "/full_text_golden_set_v1",
        "/full_text_golden_set_v1/labels/0",
    ] {
        let mut invalid = serde_json::to_value(&golden).unwrap();
        invalid
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected_field".into(), serde_json::json!(true));
        assert!(serde_json::from_value::<FullTextReviewedGoldenSet>(invalid).is_err());
    }
}

#[test]
fn full_text_worksheet_starts_blank_without_a_metadata_downcast() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let json = serde_json::to_value(&worksheet).unwrap();
    assert_eq!(json["capture_digest"], capture.capture_digest);
    assert_eq!(
        json["full_text_worksheet_v1"]["decisions"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        json["full_text_worksheet_v1"]["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|decision| decision["reviewed_disposition"].is_null())
    );
    assert!(serde_json::from_value::<StewardReviewWorksheet>(json.clone()).is_err());
    assert!(serde_json::from_value::<FullTextReviewWorksheet>(json).is_ok());
}

#[test]
fn bound_review_view_reserves_space_for_every_valid_decision() {
    let report = report_fixture();
    let metadata = build_steward_review_worksheet(&report).unwrap();
    let make_capture = |text: String| {
        capture_with(&report, MAX_SNAPSHOT_BYTES, &mut |request_path, _| {
            let mut response = response_fixture(request_path);
            if request_path == "items/BCDE3456/fulltext" {
                response.body = serde_json::json!({"content":text}).to_string();
            }
            Ok(response)
        })
        .unwrap()
    };
    let empty = make_capture(String::new());
    let base_bytes = build_full_text_review_json(&report, &metadata, &empty, 1)
        .unwrap()
        .len();
    let max_bytes = 16 * 1024 * 1024;
    let full_size = max_bytes - base_bytes;
    let full_capture = make_capture(format!(
        "{}{}",
        "\"".repeat(full_size / 4),
        "a".repeat(full_size % 4)
    ));
    assert_eq!(
        build_full_text_review_json(&report, &metadata, &full_capture, 1)
            .unwrap()
            .len(),
        max_bytes
    );
    let worksheet = build_full_text_review_worksheet(&report, &full_capture).unwrap();
    assert!(build_bound_full_text_review_json(&report, &worksheet, &full_capture, 1).is_err());

    let growth = serde_json::to_vec(&crate::Disposition::SemanticConsumptionBridge)
        .unwrap()
        .len()
        - 4;
    let admitted_size = full_size - growth;
    let capture = make_capture(format!(
        "{}{}",
        "\"".repeat(admitted_size / 4),
        "a".repeat(admitted_size % 4)
    ));
    let worksheet = build_full_text_review_worksheet(&report, &capture).unwrap();
    let bytes = build_bound_full_text_review_json(&report, &worksheet, &capture, 1).unwrap();
    assert_eq!(bytes.len() + growth, max_bytes);
    for disposition in [
        crate::Disposition::Generation,
        crate::Disposition::AlignmentVersioning,
        crate::Disposition::SemanticConsumptionBridge,
        crate::Disposition::EvaluationGovernance,
        crate::Disposition::AdjacentEvidence,
        crate::Disposition::OutOfScope,
    ] {
        assert!(serde_json::to_vec(&disposition).unwrap().len() - 4 <= growth);
    }
    let mut view: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    view["review_batch"]["decisions"][0]["reviewed_disposition"] =
        serde_json::json!(crate::Disposition::SemanticConsumptionBridge);
    let completed = serde_json::to_vec(&view).unwrap();
    assert_eq!(completed.len(), max_bytes);
    assert!(apply_full_text_review_view(&report, &worksheet, &capture, &completed).is_ok());
}

#[test]
fn review_view_binds_exact_capture_and_keeps_missing_parents_without_decisions() {
    let report = report_fixture();
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let before_report = serde_json::to_vec(&report).unwrap();
    let before_worksheet = worksheet.clone();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let bytes = build_full_text_review_json(&report, &worksheet, &capture, 2).unwrap();
    let view: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(view["view_kind"], "full_text_review_view_v1");
    assert_eq!(view["capture_digest"], capture.capture_digest);
    assert_eq!(
        view["proposal_digest"],
        crate::classification_proposal_digest(&report)
    );
    assert_eq!(
        view["metadata_report_digest"],
        capture.capture_evidence.metadata_report_digest
    );
    assert_eq!(view["bibliographic_item_count"], 2);
    assert_eq!(view["review_batch"]["remaining_count"], 2);
    assert_eq!(
        view["review_batch"]["decisions"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        view["attachment_evidence"]["DEFG5678"],
        serde_json::json!([])
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][0]["item_key"],
        "BCDE3456"
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][0]["metadata_version"],
        1
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][0]["content_response"]["version"],
        12403
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][1]["metadata_version"],
        0
    );
    assert!(view["attachment_evidence"]["ABCD2345"][1]["content_response"]["version"].is_null());
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][0]["content_response"]["body"],
        r#"{"content":"fixture text 한글","indexedPages":2,"totalPages":2,"providerExtra":{"retained":true}}"#
    );
    assert_eq!(
        view["attachment_evidence"]["ABCD2345"][1]["content_response"]["status"],
        404
    );
    assert!(
        view["review_batch"]["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["reviewed_disposition"].is_null())
    );
    assert!(serde_json::from_slice::<crate::StewardReviewBatch>(&bytes).is_err());
    assert!(serde_json::from_slice::<crate::StewardDecisionPatch>(&bytes).is_err());
    assert_eq!(serde_json::to_vec(&report).unwrap(), before_report);
    assert_eq!(worksheet, before_worksheet);
    assert_eq!(
        build_full_text_review_json(&report, &worksheet, &capture, 2).unwrap(),
        bytes
    );
}

#[test]
fn review_view_selects_only_pending_parents_and_never_copies_unrelated_text() {
    let report = report_fixture();
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let first: serde_json::Value = serde_json::from_slice(
        &build_full_text_review_json(&report, &worksheet, &capture, 1).unwrap(),
    )
    .unwrap();
    assert_eq!(
        first["review_batch"]["decisions"].as_array().unwrap().len(),
        1
    );
    assert!(first["attachment_evidence"].get("DEFG5678").is_none());
    worksheet.decisions[0].reviewed_disposition = Some(crate::Disposition::OutOfScope);
    let next = build_full_text_review_json(&report, &worksheet, &capture, 2).unwrap();
    let view: serde_json::Value = serde_json::from_slice(&next).unwrap();
    assert_eq!(view["review_batch"]["remaining_count"], 1);
    assert_eq!(view["bibliographic_item_count"], 2);
    assert_eq!(
        view["attachment_evidence"],
        serde_json::json!({"DEFG5678":[]})
    );
    assert!(!String::from_utf8(next).unwrap().contains("fixture text"));
}

#[test]
fn review_view_separates_two_text_parents_and_excludes_standalone_attachments() {
    let source: Vec<ZoteroItem> = serde_json::from_value(serde_json::json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"book","title":"first paper"}},
        {"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}},
        {"key":"DEFG5678","version":2,"data":{"itemType":"book","title":"second paper"}},
        {"key":"EFGH6789","version":1,"data":{"itemType":"attachment","parentItem":"DEFG5678"}},
        {"key":"FGHI789A","version":1,"data":{"itemType":"attachment"}}
    ]))
    .unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("fixture-server".into()), 2, source);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(match request_path {
            "fulltext?since=0" => CapturedResponse {status:200,version:None,
                body:r#"{"BCDE3456":12403,"EFGH6789":9,"FGHI789A":0}"#.into()},
            "items/EFGH6789" => CapturedResponse {status:200,version:Some(1),
                body:r#"{"key":"EFGH6789","version":1,"data":{"itemType":"attachment","parentItem":"DEFG5678"}}"#.into()},
            "items/EFGH6789/fulltext" => CapturedResponse {status:200,version:Some(9),
                body:r#"{"content":"second parent evidence"}"#.into()},
            "items/FGHI789A" => CapturedResponse {status:200,version:Some(1),
                body:r#"{"key":"FGHI789A","version":1,"data":{"itemType":"attachment"}}"#.into()},
            "items/FGHI789A/fulltext" => CapturedResponse {status:200,version:Some(0),
                body:r#"{"content":"standalone evidence"}"#.into()},
            _ => response_fixture(request_path),
        })
    }).unwrap();
    let both: serde_json::Value = serde_json::from_slice(
        &build_full_text_review_json(&report, &worksheet, &capture, 2).unwrap(),
    )
    .unwrap();
    assert_eq!(both["bibliographic_item_count"], 2);
    assert_eq!(both["review_batch"]["pending_source_count"], 1);
    assert_eq!(
        both["review_batch"]["proposal_digest"],
        both["proposal_digest"]
    );
    assert_eq!(both["proposal_digest"], worksheet.proposal_digest);
    assert_eq!(report.pending_source_item_keys, ["FGHI789A"]);
    assert!(!both.to_string().contains("standalone evidence"));
    assert!(!both.to_string().contains("FGHI789A"));
    assert_eq!(
        both["attachment_evidence"]["ABCD2345"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        both["attachment_evidence"]["DEFG5678"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        both["attachment_evidence"]["DEFG5678"][0]["item_key"],
        "EFGH6789"
    );
    assert_eq!(
        both["attachment_evidence"]["DEFG5678"][0]["content_response"]["version"],
        9
    );
    let first =
        String::from_utf8(build_full_text_review_json(&report, &worksheet, &capture, 1).unwrap())
            .unwrap();
    assert!(first.contains("fixture text"));
    assert!(!first.contains("second parent evidence"));
    assert!(!first.contains("standalone evidence"));
    worksheet.decisions[0].reviewed_disposition = Some(crate::Disposition::OutOfScope);
    let second =
        String::from_utf8(build_full_text_review_json(&report, &worksheet, &capture, 1).unwrap())
            .unwrap();
    assert!(second.contains("second parent evidence"));
    assert!(!second.contains("fixture text"));
    assert!(!second.contains("standalone evidence"));
    assert_eq!(report.pending_source_item_keys, ["FGHI789A"]);
}

#[test]
fn review_view_rejects_changed_capture_report_and_invalid_pending_work() {
    let report = report_fixture();
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    for field in ["body", "status", "version"] {
        let mut saved = serde_json::to_value(&capture).unwrap();
        saved["capture_evidence"]["records"][0]["content_response"][field] = if field == "body" {
            serde_json::json!("private changed text")
        } else {
            serde_json::json!(999)
        };
        let changed: FullTextCapture = serde_json::from_value(saved).unwrap();
        assert!(build_full_text_review_json(&report, &worksheet, &changed, 2).is_err());
    }
    let mut changed_report = report_fixture();
    changed_report.classified_items[0].title = "different report evidence".into();
    assert!(build_full_text_review_json(&changed_report, &worksheet, &capture, 2).is_err());
    let mut changed_report = report_fixture();
    changed_report.unclassified_items[0].data.title = "changed retained source".into();
    let fresh_worksheet = build_steward_review_worksheet(&changed_report).unwrap();
    assert_eq!(fresh_worksheet.snapshot_digest, worksheet.snapshot_digest);
    assert_ne!(fresh_worksheet.proposal_digest, worksheet.proposal_digest);
    assert!(build_full_text_review_json(&changed_report, &fresh_worksheet, &capture, 2).is_err());
    let mut unbound = worksheet.clone();
    unbound.proposal_digest.clear();
    assert!(build_full_text_review_json(&report, &unbound, &capture, 2).is_err());
    for limit in [0, 101] {
        assert!(build_full_text_review_json(&report, &worksheet, &capture, limit).is_err());
    }
    let mut invalid = worksheet.clone();
    invalid.decisions[0].reviewed_disposition = Some(crate::Disposition::NeedsStewardReview);
    assert!(build_full_text_review_json(&report, &invalid, &capture, 2).is_err());
    for decision in &mut invalid.decisions {
        decision.reviewed_disposition = Some(crate::Disposition::OutOfScope);
    }
    assert!(build_full_text_review_json(&report, &invalid, &capture, 2).is_err());
}

#[test]
fn review_view_retains_empty_partial_and_unknown_provider_evidence() {
    for body in [
        r#"{"content":""}"#,
        r#"{"content":"partial text","indexedPages":1,"totalPages":2}"#,
        r#"{"content":"unknown completeness","providerExtra":{"original":true}}"#,
    ] {
        let report = report_fixture();
        let worksheet = build_steward_review_worksheet(&report).unwrap();
        let capture = capture_with(&report, 4096, &mut |request_path, _| {
            let mut response = response_fixture(request_path);
            if request_path == "items/BCDE3456/fulltext" {
                response.body = body.into();
            }
            Ok(response)
        })
        .unwrap();
        let view: serde_json::Value = serde_json::from_slice(
            &build_full_text_review_json(&report, &worksheet, &capture, 2).unwrap(),
        )
        .unwrap();
        assert_eq!(
            view["attachment_evidence"]["ABCD2345"][0]["content_response"]["body"],
            body
        );
        assert_eq!(view["bibliographic_item_count"], 2);
    }
}

#[test]
fn review_view_rejects_json_expansion_without_truncating_valid_captured_text() {
    let report = report_fixture();
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let capture = capture_with(&report, MAX_SNAPSHOT_BYTES, &mut |request_path, _| {
        let mut response = response_fixture(request_path);
        if request_path.ends_with("/fulltext") {
            response.status = 200;
            response.version = Some(if request_path.contains("BCDE3456") {
                12403
            } else {
                0
            });
            response.body = serde_json::json!({"content":"\"".repeat(3 * 1024 * 1024)}).to_string();
        }
        Ok(response)
    })
    .unwrap();
    verify_full_text_capture(&capture, &report).unwrap();
    let error = build_full_text_review_json(&report, &worksheet, &capture, 1).unwrap_err();
    assert!(error.to_string().contains("16 MiB output limit"));
    assert_eq!(worksheet.decisions.len(), 2);
    let bound = build_full_text_review_worksheet(&report, &capture).unwrap();
    assert!(build_bound_full_text_review_json(&report, &bound, &capture, 1).is_err());
    let mut batch = crate::build_steward_review_batch(&report, &worksheet, 1).unwrap();
    batch.decisions[0].reviewed_disposition = Some(crate::Disposition::OutOfScope);
    let input = serde_json::to_vec(&serde_json::json!({"review_batch":batch})).unwrap();
    let error = apply_full_text_review_view(&report, &bound, &capture, &input)
        .err()
        .unwrap();
    assert!(error.to_string().contains("16 MiB output limit"));
}

fn report_fixture() -> ClassificationReport {
    let items: Vec<ZoteroItem> = serde_json::from_value(serde_json::json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"journalArticle","title":"fixture paper"}},
        {"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}},
        {"key":"CDEF4567","version":0,"data":{"itemType":"attachment","parentItem":"ABCD2345"}},
        {"key":"DEFG5678","version":2,"data":{"itemType":"book","title":"no attachment fixture"}}
    ]))
    .unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("fixture-server".into()), 2, items);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    report
}

fn response_fixture(request_path: &str) -> CapturedResponse {
    let (status, version, body) = match request_path {
        "items?limit=1" => (200, Some(2), "[]"),
        "fulltext?since=0" => (200, None, r#"{"BCDE3456":12403,"CDEF4567":0}"#),
        "items/BCDE3456" => (
            200,
            Some(1),
            r#"{"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}}"#,
        ),
        "items/CDEF4567" => (
            200,
            Some(0),
            r#"{"key":"CDEF4567","version":0,"data":{"itemType":"attachment","parentItem":"ABCD2345"}}"#,
        ),
        "items/BCDE3456/fulltext" => (
            200,
            Some(12403),
            r#"{"content":"fixture text 한글","indexedPages":2,"totalPages":2,"providerExtra":{"retained":true}}"#,
        ),
        "items/CDEF4567/fulltext" => (404, None, "missing"),
        _ => panic!("unexpected fixture request"),
    };
    CapturedResponse {
        status,
        version,
        body: body.into(),
    }
}

#[test]
fn capture_retains_exact_text_missing_results_and_full_parent_denominator() {
    let report = report_fixture();
    let old_digest = report.snapshot_digest.clone();
    let mut requests = Vec::new();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        requests.push(request_path.to_owned());
        Ok(response_fixture(request_path))
    })
    .unwrap();
    assert_eq!(requests.len(), 8);
    assert_eq!(requests.first().unwrap(), "items?limit=1");
    assert_eq!(requests.last().unwrap(), "items?limit=1");
    let saved = serde_json::to_value(&capture).unwrap();
    assert_eq!(
        saved["capture_evidence"]["capture_kind"],
        "non_atomic_fulltext_sweep_v1"
    );
    assert_eq!(saved["capture_evidence"]["bibliographic_item_count"], 2);
    assert_eq!(
        saved["capture_evidence"]["records"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        saved["capture_evidence"]["records"][0]["content_response"]["body"],
        response_fixture("items/BCDE3456/fulltext").body
    );
    assert_eq!(
        saved["capture_evidence"]["records"][0]["content_response"]["version"],
        12403
    );
    assert_eq!(
        saved["capture_evidence"]["records"][1]["content_response"]["status"],
        404
    );
    assert_eq!(report.snapshot_digest, old_digest);
    let restored: FullTextCapture = serde_json::from_value(saved.clone()).unwrap();
    verify_full_text_capture(&restored, &report).unwrap();
    let mut changed = saved;
    changed["capture_evidence"]["records"][0]["content_response"]["body"] =
        r#"{"content":"changed text"}"#.into();
    let changed: FullTextCapture = serde_json::from_value(changed).unwrap();
    assert!(verify_full_text_capture(&changed, &report).is_err());
}

#[test]
fn capture_rejects_unbound_reports_before_any_request() {
    let mut report = report_fixture();
    report.server_id = None;
    let mut calls = 0;
    assert!(
        capture_with(&report, 4096, &mut |_, _| {
            calls += 1;
            unreachable!()
        })
        .is_err()
    );
    assert_eq!(calls, 0);
}

#[test]
fn capture_rejects_incomplete_or_inconsistent_retained_sources_before_fetch() {
    for mismatch_parent in [false, true] {
        let mut report = report_fixture();
        if mismatch_parent {
            report.unclassified_items[0].data.parent_item = "DEFG5678".into();
        } else {
            report.unclassified_items.pop();
        }
        let mut calls = 0;
        assert!(
            capture_with(&report, 4096, &mut |_, _| {
                calls += 1;
                unreachable!()
            })
            .is_err()
        );
        assert_eq!(calls, 0);
    }
}

#[test]
fn capture_preserves_pending_sources_and_rejects_rebound_report_context() {
    let items: Vec<ZoteroItem> = serde_json::from_value(serde_json::json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"journalArticle","title":"fixture paper"}},
        {"key":"BCDE3456","version":1,"data":{"itemType":"attachment"}}
    ]))
    .unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("fixture-server".into()), 2, items);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    let original_report = serde_json::to_vec(&report).unwrap();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        let mut response = response_fixture(request_path);
        if request_path == "fulltext?since=0" {
            response.body = r#"{"BCDE3456":12403}"#.into();
        } else if request_path == "items/BCDE3456" {
            response.body =
                r#"{"key":"BCDE3456","version":1,"data":{"itemType":"attachment"}}"#.into();
        }
        Ok(response)
    })
    .unwrap();
    let restored: FullTextCapture =
        serde_json::from_slice(&serde_json::to_vec(&capture).unwrap()).unwrap();
    verify_full_text_capture(&restored, &report).unwrap();
    assert_eq!(serde_json::to_vec(&report).unwrap(), original_report);
    assert_eq!(report.pending_source_item_keys, vec!["BCDE3456"]);
    assert_eq!(capture.capture_evidence.bibliographic_item_count, 1);
    assert_eq!(capture.capture_evidence.records.len(), 1);
    let snapshot_digest = report.snapshot_digest.clone();
    report.unclassified_items[0].data.title = "changed retained source context".into();
    assert!(build_steward_review_worksheet(&report).is_ok());
    assert_eq!(report.snapshot_digest, snapshot_digest);
    assert!(verify_full_text_capture(&restored, &report).is_err());
}

#[test]
fn capture_rejects_foreign_manifest_items_and_duplicate_manifest_keys() {
    for body in [r#"{"EFGH6789":0}"#, r#"{"BCDE3456":1,"BCDE3456":2}"#] {
        let mut calls = 0;
        assert!(
            capture_with(&report_fixture(), 4096, &mut |request_path, _| {
                calls += 1;
                let mut response = response_fixture(request_path);
                if request_path == "fulltext?since=0" {
                    response.body = body.into();
                }
                Ok(response)
            })
            .is_err()
        );
        assert_eq!(calls, 2);
    }
}

#[test]
fn capture_rejects_parent_version_status_and_bookend_drift() {
    for scenario in 0..5 {
        let mut manifest_reads = 0;
        let result = capture_with(&report_fixture(), 4096, &mut |request_path, _| {
            let mut response = response_fixture(request_path);
            if request_path == "items/BCDE3456" {
                if scenario == 0 {
                    response.body = response.body.replace("ABCD2345", "DEFG5678");
                }
                if scenario == 1 {
                    response.version = Some(3);
                }
            }
            if request_path == "items/BCDE3456/fulltext" {
                if scenario == 2 {
                    response.status = 500;
                }
                if scenario == 3 {
                    response.version = Some(12404);
                }
            }
            if request_path == "fulltext?since=0" {
                manifest_reads += 1;
                if scenario == 4 && manifest_reads == 2 {
                    response.body = "{}".into();
                }
            }
            Ok(response)
        });
        assert!(result.is_err(), "drift scenario {scenario}");
    }
}

#[test]
fn capture_checks_total_budget_before_another_request() {
    let mut calls = 0;
    assert!(
        capture_with(&report_fixture(), 2, &mut |request_path, limit| {
            calls += 1;
            assert_eq!(limit, 2);
            Ok(response_fixture(request_path))
        })
        .is_err()
    );
    assert_eq!(calls, 1);
}

#[test]
fn capture_validates_each_report_admission_condition() {
    for scenario in 0..10 {
        let mut report = report_fixture();
        match scenario {
            0 => report.api_version = None,
            1 => report.schema_version = None,
            2 => report.server_id = Some("  ".into()),
            3 => report.zotero_version = "9.0.6".into(),
            4 => report.zotero_version = "invalid".into(),
            5 => report.classified_items.clear(),
            6 => report.observed_item_count = MAX_SNAPSHOT_ITEMS + 1,
            7 => report.snapshot_digest.clear(),
            8 => report.snapshot_items[0].item_version += 1,
            _ => {
                let items = serde_json::from_value(serde_json::json!([
                    {"key":"INVALID0","version":2,"data":{"itemType":"book","title":"fixture"}}
                ]))
                .unwrap();
                report =
                    classify_snapshot("10.0.1".into(), Some("fixture-server".into()), 2, items);
                report.api_version = Some(3);
                report.schema_version = Some(44);
            }
        }
        let result = capture_with(&report, 4096, &mut |_, _| {
            panic!("admission must precede requests")
        });
        assert!(result.is_err(), "admission scenario {scenario}");
    }
}

#[test]
fn replay_checks_bindings_and_structure_even_when_digest_is_recomputed() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let saved = serde_json::to_value(capture).unwrap();
    for scenario in 0..16 {
        let mut restored: FullTextCapture = serde_json::from_value(saved.clone()).unwrap();
        let evidence = &mut restored.capture_evidence;
        match scenario {
            0 => evidence.capture_kind = "snapshot".into(),
            1 => evidence.metadata_report_digest.clear(),
            2 => evidence.metadata_snapshot_digest.clear(),
            3 => evidence.bibliographic_item_count = 1,
            4 => evidence.finished_unix_ms = 0,
            5 => evidence.library_before.status = 500,
            6 => evidence.library_after.version = Some(3),
            7 => evidence.library_after.body = "null".into(),
            8 => evidence.manifest_after.status = 500,
            9 => evidence.manifest_after.body = "{}".into(),
            10 => {
                evidence.records.pop();
            }
            11 => evidence.records.swap(0, 1),
            12 => evidence.records[0].metadata_response.body = "{}".into(),
            13 => evidence.records[0].content_response.body = r#"{"content":null}"#.into(),
            14 => evidence.records[0].content_response.version = None,
            _ => {
                let serialized = serde_json::to_string(&evidence.records[0]).unwrap();
                for _ in 0..3 {
                    evidence
                        .records
                        .push(serde_json::from_str(&serialized).unwrap());
                }
            }
        }
        restored.capture_digest = json_digest(evidence);
        assert!(
            verify_full_text_capture(&restored, &report).is_err(),
            "replay scenario {scenario}"
        );
    }
    let mut another_report = report_fixture();
    another_report.schema_version = Some(45);
    let restored: FullTextCapture = serde_json::from_value(saved).unwrap();
    assert!(verify_full_text_capture(&restored, &another_report).is_err());
}

#[test]
fn malformed_and_oversized_manifests_never_become_partial_captures() {
    let report = report_fixture();
    let snapshot = validate_report(&report).unwrap();
    for body in [
        "[]",
        "not json",
        r#"{"BCDE3456":-1}"#,
        r#"{"BCDE3456":1.5}"#,
        r#"{"BCDE3456":0} {}"#,
    ] {
        let response = CapturedResponse {
            status: 200,
            version: None,
            body: body.into(),
        };
        assert!(parse_manifest(&response, &snapshot).is_err());
    }
    let response = CapturedResponse {
        status: 503,
        version: None,
        body: "{}".into(),
    };
    assert!(parse_manifest(&response, &snapshot).is_err());
    let entries: BTreeMap<_, _> = (0..=MAX_SNAPSHOT_ITEMS)
        .map(|index| (format!("{index:08}"), 0))
        .collect();
    let response = CapturedResponse {
        status: 200,
        version: None,
        body: serde_json::to_string(&entries).unwrap(),
    };
    assert!(parse_manifest(&response, &snapshot).is_err());
    assert!(
        capture_with(&report, 4096, &mut |_, _| Err(FullTextError(
            "fixture request failed"
        )))
        .is_err()
    );
}

#[test]
fn metadata_requires_attachment_identity_revision_and_parent() {
    let report = report_fixture();
    let snapshot = validate_report(&report).unwrap();
    let expected = snapshot["BCDE3456"];
    for scenario in 0..5 {
        let mut response = response_fixture("items/BCDE3456");
        let mut body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
        match scenario {
            0 => response.status = 404,
            1 => body["key"] = "CDEF4567".into(),
            2 => body["version"] = 2.into(),
            3 => body["data"]["itemType"] = "note".into(),
            _ => body["data"]["parentItem"] = "DEFG5678".into(),
        }
        response.body = serde_json::to_string(&body).unwrap();
        assert!(validate_metadata(&response, expected).is_err());
    }
    let response = CapturedResponse {
        status: 200,
        version: Some(1),
        body: r#"{"key":"BCDE3456","version":1,"data":{"itemType":"attachment"}}"#.into(),
    };
    let expected = SnapshotItemRevision {
        item_key: "BCDE3456".into(),
        item_version: 1,
        parent_item_key: None,
    };
    validate_metadata(&response, &expected).unwrap();
}

#[test]
fn empty_partial_and_unknown_content_counters_are_retained_without_approval() {
    for body in [
        r#"{"content":""}"#,
        r#"{"content":"text","indexedPages":1,"totalPages":2}"#,
        r#"{"content":"text","indexedChars":null,"totalChars":null}"#,
    ] {
        let capture = capture_with(&report_fixture(), 4096, &mut |request_path, _| {
            let mut response = response_fixture(request_path);
            if request_path == "items/BCDE3456/fulltext" {
                response.body = body.into();
            }
            Ok(response)
        })
        .unwrap();
        assert_eq!(
            capture.capture_evidence.records[0].content_response.body,
            body
        );
    }
}

#[test]
fn byte_and_clock_limits_include_exact_boundary_and_overflow_failures() {
    let small = response_fixture("items?limit=1");
    assert!(account_body(&mut 1, &small).is_err());
    let mut exact_remaining = 2;
    account_body(&mut exact_remaining, &small).unwrap();
    assert_eq!(exact_remaining, 0);
    let oversized = CapturedResponse {
        status: 404,
        version: None,
        body: "x".repeat(MAX_PAGE_BYTES as usize + 1),
    };
    let mut remaining = MAX_SNAPSHOT_BYTES;
    assert!(account_body(&mut remaining, &oversized).is_err());
    assert!(check_admission(0, Duration::ZERO).is_err());
    assert!(check_admission(1, CAPTURE_DEADLINE).is_err());
    check_admission(1, CAPTURE_DEADLINE - Duration::from_nanos(1)).unwrap();
    assert!(unix_millis(UNIX_EPOCH - Duration::from_secs(1)).is_err());
    assert_eq!(unix_millis(UNIX_EPOCH).unwrap(), 0);
    let max_time = UNIX_EPOCH
        .checked_add(Duration::from_secs(u64::MAX / 1000 + 1))
        .unwrap();
    assert!(unix_millis(max_time).is_err());
    let message = FullTextError("fixture static error");
    assert_eq!(message.to_string(), "fixture static error");
    assert_eq!(
        format!("{message:?}"),
        "FullTextError(\"fixture static error\")"
    );
}

#[test]
fn replay_applies_byte_admission_before_digest_or_json_work() {
    let report = report_fixture();
    let mut capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    capture.capture_evidence.records[1].content_response.body =
        "x".repeat(MAX_PAGE_BYTES as usize + 1);
    assert_eq!(
        verify_full_text_capture(&capture, &report),
        Err(BUDGET_EXCEEDED)
    );
}

#[test]
fn streaming_digest_preserves_the_existing_exact_json_representation() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let bytes = serde_json::to_vec(&capture.capture_evidence).unwrap();
    assert_eq!(
        capture.capture_digest,
        format!("sha256:{:x}", Sha256::digest(bytes))
    );
}

#[test]
fn capture_rejects_clock_failure_and_late_results_without_real_waiting() {
    for failed_poll in [0, 1, 2, 17, 18] {
        let mut clock_polls = 0;
        let result = capture_with_clock(
            &report_fixture(),
            4096,
            &mut |request_path, _| Ok(response_fixture(request_path)),
            &mut || {
                let poll = clock_polls;
                clock_polls += 1;
                if poll == failed_poll && [0, 17].contains(&poll) {
                    (UNIX_EPOCH - Duration::from_secs(1), Duration::ZERO)
                } else if poll == failed_poll {
                    (UNIX_EPOCH + Duration::from_secs(100), CAPTURE_DEADLINE)
                } else {
                    (UNIX_EPOCH + Duration::from_secs(100), Duration::ZERO)
                }
            },
        );
        assert!(result.is_err(), "clock poll {failed_poll}");
    }
}

#[test]
fn every_failed_request_and_invalid_replay_input_fails_closed() {
    let report = report_fixture();
    for failed_request in 0..8 {
        let mut requests = 0;
        let result = capture_with(&report, 4096, &mut |request_path, _| {
            requests += 1;
            if requests == failed_request + 1 {
                Err(INVALID_EVIDENCE)
            } else {
                Ok(response_fixture(request_path))
            }
        });
        assert!(result.is_err());
        assert_eq!(requests, failed_request + 1);
    }
    assert!(
        capture_with(&report, 1, &mut |request_path, _| Ok(response_fixture(
            request_path
        )))
        .is_err()
    );
    let mut capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let mut invalid_report = report_fixture();
    invalid_report.api_version = None;
    assert!(verify_full_text_capture(&capture, &invalid_report).is_err());
    capture.capture_evidence.manifest_before.body = "null".into();
    assert!(verify_full_text_capture(&capture, &report).is_err());
}
