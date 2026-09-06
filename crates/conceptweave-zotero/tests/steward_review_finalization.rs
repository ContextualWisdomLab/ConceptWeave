use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenSetApproval, ItemData, ZoteroItem,
    build_steward_review_worksheet, classification_proposal_digest, classify_snapshot,
    reviewed_golden_set_from_worksheet,
};

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            ZoteroItem {
                source_record: None,
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
                source_record: None,
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

fn approval(
    report: &conceptweave_zotero::ClassificationReport,
    worksheet: &conceptweave_zotero::StewardReviewWorksheet,
) -> GoldenSetApproval {
    GoldenSetApproval {
        receipt_id: "review-receipt".into(),
        reviewer_subject: "steward-subject".into(),
        library_version: worksheet.library_version,
        rule_revision: worksheet.rule_revision.clone(),
        snapshot_digest: worksheet.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(report),
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
        reviewed_golden_set_from_worksheet(&report(), &worksheet, approval(&report(), &worksheet))
            .unwrap();

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
            approval(&report(), &worksheet)
        ),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid_approval = approval(&report(), &worksheet);
    invalid_approval.receipt_id.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid_approval = approval(&report(), &worksheet);
    invalid_approval.reviewer_subject.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.rule_revision.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_digest.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid_approval = approval(&report(), &worksheet);
    invalid_approval.library_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid_approval = approval(&report(), &worksheet);
    invalid_approval.rule_revision.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid_approval = approval(&report(), &worksheet);
    invalid_approval.snapshot_items.pop();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, invalid_approval),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.snapshot_items[0].item_key.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_items[1] = invalid.snapshot_items[0].clone();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.library_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.rule_revision.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut invalid = worksheet.clone();
    invalid.snapshot_digest.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::SnapshotMismatch)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].item_key.clear();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[0].item_version += 1;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[0].proposed_disposition = Disposition::AlignmentVersioning;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].abstention_reason =
        Some(conceptweave_zotero::AbstentionReason::NoDeterministicRuleMatch);
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );
    let mut invalid = worksheet.clone();
    invalid.decisions[1].abstention_reason = None;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions.pop();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::IncompleteReview)
    );

    let empty_report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![ZoteroItem {
            source_record: None,
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
        reviewed_golden_set_from_worksheet(&empty_report, &empty, approval(&empty_report, &empty)),
        Err(EvaluationError::IncompleteReview)
    );
}

#[test]
fn finalization_rejects_incomplete_invalid_or_mismatched_review() {
    let worksheet = build_steward_review_worksheet(&report()).unwrap();
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &worksheet, approval(&report(), &worksheet)),
        Err(EvaluationError::IncompleteReview)
    );

    let mut invalid = worksheet.clone();
    invalid.decisions[0].reviewed_disposition = Some(Disposition::NeedsStewardReview);
    invalid.decisions[1].reviewed_disposition = Some(Disposition::Generation);
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &invalid, approval(&report(), &invalid)),
        Err(EvaluationError::InvalidExpectedDisposition)
    );

    let mut complete = worksheet.clone();
    for decision in &mut complete.decisions {
        decision.reviewed_disposition = Some(Disposition::Generation);
    }
    let mut mismatched = approval(&report(), &complete);
    mismatched.snapshot_digest.push_str("-changed");
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &complete, mismatched),
        Err(EvaluationError::SnapshotMismatch)
    );

    complete.decisions[1].item_key = complete.decisions[0].item_key.clone();
    complete.decisions[1].item_version = complete.decisions[0].item_version;
    assert_eq!(
        reviewed_golden_set_from_worksheet(&report(), &complete, approval(&report(), &complete)),
        Err(EvaluationError::InvalidReview)
    );
}

#[test]
fn finalization_rejects_changed_review_evidence_under_the_original_approval() {
    for alter_title in [false, true] {
        let mut report = report();
        let worksheet = complete_worksheet();
        let original_approval = approval(&report, &worksheet);
        let golden =
            reviewed_golden_set_from_worksheet(&report, &worksheet, original_approval.clone())
                .unwrap();
        assert_eq!(golden.approval, original_approval);

        if alter_title {
            report.classified_items[0].title.push_str(" changed");
        } else {
            report.classified_items[0].evidence.field_values.clear();
        }
        let mut rebuilt_worksheet = build_steward_review_worksheet(&report).unwrap();
        for decision in &mut rebuilt_worksheet.decisions {
            decision.reviewed_disposition = Some(Disposition::Generation);
        }
        assert_eq!(
            reviewed_golden_set_from_worksheet(&report, &rebuilt_worksheet, original_approval),
            Err(EvaluationError::SnapshotMismatch)
        );
    }
}

#[test]
fn locally_rebound_finalization_does_not_renew_independent_approval() {
    let mut report = report();
    let mut worksheet = complete_worksheet();
    let issued_set =
        reviewed_golden_set_from_worksheet(&report, &worksheet, approval(&report, &worksheet))
            .unwrap();
    assert!(
        conceptweave_zotero::evaluate_reviewed_golden_set(&report, &issued_set, |value| value
            == &issued_set)
        .is_ok()
    );
    report.classified_items[0].title.push_str(" changed");
    worksheet.proposal_digest = classification_proposal_digest(&report);
    let rebound =
        reviewed_golden_set_from_worksheet(&report, &worksheet, approval(&report, &worksheet))
            .unwrap();
    assert_eq!(
        conceptweave_zotero::evaluate_reviewed_golden_set(&report, &rebound, |value| value
            == &issued_set),
        Err(EvaluationError::UnverifiedApproval)
    );
}

#[test]
fn pending_source_conversion_does_not_prove_complete_review() {
    let items = [("A", "book"), ("source", "attachment")]
        .into_iter()
        .map(|(key, item_type)| {
            serde_json::from_value::<ZoteroItem>(serde_json::json!({
                "key": key, "version": 7,
                "data": {"itemType": item_type, "title": "synthetic ontology learning"}
            }))
            .unwrap()
        })
        .collect();
    let report = classify_snapshot("9.0.6".into(), None, 42, items);
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    worksheet.decisions[0].reviewed_disposition = Some(Disposition::Generation);
    let local_set =
        reviewed_golden_set_from_worksheet(&report, &worksheet, approval(&report, &worksheet))
            .unwrap();
    assert_eq!(report.pending_source_item_keys, ["source"]);
    assert_eq!(
        conceptweave_zotero::evaluate_complete_reviewed_classification(&report, &local_set, |_| {
            panic!("pending scope must not contact governance")
        }),
        Err(EvaluationError::IncompleteReview)
    );
}

#[test]
fn finalization_rejects_stale_worksheet_even_with_current_approval_coordinates() {
    for changed_field in 0..3 {
        let mut report = report();
        let worksheet = complete_worksheet();
        match changed_field {
            0 => report.classified_items[0].title.push_str(" changed"),
            1 => report.classified_items[0].evidence.field_values.clear(),
            _ => report.classified_items[0].review_abstract_note = Some("changed context".into()),
        }
        let current_receipt = approval(&report, &worksheet);
        assert_eq!(
            reviewed_golden_set_from_worksheet(&report, &worksheet, current_receipt),
            Err(EvaluationError::SnapshotMismatch)
        );
    }
}

#[test]
fn finalization_rejects_blank_or_replaced_worksheet_binding() {
    let report = report();
    for (binding, expected) in [
        ("", EvaluationError::InvalidReview),
        ("  ", EvaluationError::InvalidReview),
        ("sha256:replaced", EvaluationError::SnapshotMismatch),
    ] {
        let mut worksheet = complete_worksheet();
        worksheet.proposal_digest = binding.into();
        assert_eq!(
            reviewed_golden_set_from_worksheet(&report, &worksheet, approval(&report, &worksheet)),
            Err(expected)
        );
    }
}

#[test]
fn finalization_rejects_missing_or_replaced_proposal_binding() {
    let report = report();
    let worksheet = complete_worksheet();
    for (digest, expected) in [
        ("", EvaluationError::InvalidReview),
        ("  ", EvaluationError::InvalidReview),
        ("sha256:replaced", EvaluationError::SnapshotMismatch),
    ] {
        let mut receipt = approval(&report, &worksheet);
        receipt.proposal_digest = digest.into();
        assert_eq!(
            reviewed_golden_set_from_worksheet(&report, &worksheet, receipt),
            Err(expected)
        );
    }
}
