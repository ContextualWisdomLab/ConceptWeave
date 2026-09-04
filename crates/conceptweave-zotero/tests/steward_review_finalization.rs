use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenSetApproval, ItemData, ZoteroItem,
    build_steward_review_worksheet, classify_snapshot, reviewed_golden_set_from_worksheet,
};

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            ZoteroItem {
                key: "B".into(),
                version: 8,
                data: ItemData {
                    item_type: "book".into(),
                    title: "unknown vocabulary".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            },
            ZoteroItem {
                key: "A".into(),
                version: 7,
                data: ItemData {
                    item_type: "book".into(),
                    title: "ontology learning".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            },
        ],
    )
}

fn approval(worksheet: &conceptweave_zotero::StewardReviewWorksheet) -> GoldenSetApproval {
    GoldenSetApproval {
        receipt_id: "review-receipt".into(),
        reviewer_subject: "steward-subject".into(),
        library_version: worksheet.library_version,
        rule_revision: worksheet.rule_revision.clone(),
        snapshot_digest: worksheet.snapshot_digest.clone(),
        snapshot_items: worksheet.snapshot_items.clone(),
    }
}

fn complete_worksheet() -> conceptweave_zotero::StewardReviewWorksheet {
    let mut worksheet = build_steward_review_worksheet(&report()).unwrap();
    for decision in &mut worksheet.decisions {
        decision.reviewed_disposition = Some(Disposition::Generation);
    }
    worksheet
}

#[test]
fn complete_worksheet_becomes_a_snapshot_bound_golden_set() {
    let worksheet = complete_worksheet();

    let golden =
        reviewed_golden_set_from_worksheet(&report(), &worksheet, approval(&worksheet)).unwrap();

    assert_eq!(golden.labels.len(), 2);
    assert_eq!(golden.labels[0].item_key, "A");
    assert_eq!(golden.labels[1].item_key, "B");
    assert_eq!(golden.approval.snapshot_items, worksheet.snapshot_items);
}

#[test]
fn finalization_rejects_each_invalid_identity_coordinate() {
    let worksheet = complete_worksheet();

    let mut invalid_report = report();
    invalid_report.rule_revision = "";
    assert_eq!(
        reviewed_golden_set_from_worksheet(
            &invalid_report,
            &worksheet,
            approval(&worksheet)
        ),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid_approval = approval(&worksheet);
    invalid_approval.receipt_id.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid_approval = approval(&worksheet);
    invalid_approval.reviewer_subject.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.rule_revision.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_digest.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid_approval = approval(&worksheet);
    invalid_approval.library_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid_approval = approval(&worksheet);
    invalid_approval.rule_revision.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid_approval = approval(&worksheet);
    invalid_approval.snapshot_items.pop();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.snapshot_items[0].item_key.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_items[1] = invalid.snapshot_items[0].clone();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.library_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.rule_revision.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_digest.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].item_key.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[0].item_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[0].proposed_disposition = Disposition::AlignmentVersioning;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].abstention_reason =
        Some(conceptweave_zotero::AbstentionReason::NoDeterministicRuleMatch);
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[1].abstention_reason = None;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions.pop();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::IncompleteReview)
    );

    let empty_report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![ZoteroItem {
            key: "NOTE".into(),
            version: 1,
            data: ItemData {
                item_type: "note".into(),
                title: String::new(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );
    let empty = build_steward_review_worksheet(&empty_report).unwrap();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&empty_report, &empty, approval(&empty)),
        Err(EvaluationError::IncompleteReview)
    );
}

#[test]
fn finalization_rejects_incomplete_invalid_or_mismatched_review() {
    let worksheet = build_steward_review_worksheet(&report()).unwrap();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, approval(&worksheet)),
        Err(EvaluationError::IncompleteReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].reviewed_disposition = Some(Disposition::NeedsStewardReview);
    invalid.decisions[1].reviewed_disposition = Some(Disposition::Generation);
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&invalid)),
        Err(EvaluationError::InvalidExpectedDisposition)
    );

    let mut complete = worksheet.clone();
    for decision in &mut complete.decisions {
        decision.reviewed_disposition = Some(Disposition::Generation);
    }
    let mut mismatched = approval(&complete);
    mismatched.snapshot_digest.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &complete, mismatched),
        Err(EvaluationError::SnapshotMismatch)
    );

    complete.decisions[1].item_key = complete.decisions[0].item_key.clone();
    complete.decisions[1].item_version = complete.decisions[0].item_version;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &complete, approval(&complete)),
        Err(EvaluationError::InvalidReview)
    );
}
