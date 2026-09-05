use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_proposal_digest,
    classification_snapshot_digest, classify_snapshot, evaluate_reviewed_golden_set,
};

fn bibliographic(key: &str, version: u64, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: "book".into(),
            title: title.into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
        source_record: None,
    }
}

fn child_note(key: &str, version: u64, parent_item: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: "note".into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
        source_record: None,
    }
}

fn approval(
    report: &conceptweave_zotero::ClassificationReport,
    snapshot_items: Vec<SnapshotItemRevision>,
) -> GoldenSetApproval {
    GoldenSetApproval {
        receipt_id: "approved-review".into(),
        reviewer_subject: "synthetic-steward".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: classification_snapshot_digest(report),
        proposal_digest: classification_proposal_digest(report),
        snapshot_items,
    }
}

#[test]
fn reviewed_snapshot_binding_includes_linked_child_revisions() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("PARENT", 7, "ontology learning"),
            child_note("NOTE1", 3, "PARENT"),
        ],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(
            &report,
            vec![
                SnapshotItemRevision {
                    item_key: "PARENT".into(),
                    item_version: 7,
                },
                SnapshotItemRevision {
                    item_key: "NOTE1".into(),
                    item_version: 3,
                },
            ],
        ),
        labels: vec![GoldenLabel::new("PARENT", Disposition::Generation)],
    };

    assert!(
        evaluate_reviewed_golden_set(&report, &golden, |_| true).is_ok(),
        "an approval for the complete observed Zotero snapshot must include linked child revisions even though only bibliographic parents receive dispositions"
    );
}

#[test]
fn verified_snapshot_receipt_cannot_authorize_mutated_steward_labels() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(
            &report,
            vec![SnapshotItemRevision {
                item_key: "A".into(),
                item_version: 1,
            }],
        ),
        labels: vec![GoldenLabel::new("A", Disposition::AlignmentVersioning)],
    };

    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |reviewed_set| {
            reviewed_set.approval.receipt_id == "approved-review"
                && reviewed_set.labels == vec![GoldenLabel::new("A", Disposition::Generation)]
        }),
        Err(EvaluationError::UnverifiedApproval),
        "approval verification must bind the reviewed labels as well as the snapshot receipt"
    );
}

#[test]
fn duplicate_zotero_keys_fail_closed_even_when_item_revisions_differ() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            bibliographic("A", 2, "ontology learning"),
        ],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(
            &report,
            vec![
                SnapshotItemRevision {
                    item_key: "A".into(),
                    item_version: 1,
                },
                SnapshotItemRevision {
                    item_key: "A".into(),
                    item_version: 2,
                },
            ],
        ),
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    };

    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |_| true),
        Err(EvaluationError::InvalidReview),
        "Zotero item keys are identities; duplicate keys cannot become distinct records merely because their revision counters differ"
    );
}

#[test]
fn approved_snapshot_cannot_authorize_a_prediction_changed_to_match_the_label() {
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(&report, report.snapshot_items.clone()),
        labels: vec![GoldenLabel::new("A", Disposition::AlignmentVersioning)],
    };
    let approved_golden = golden.clone();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |candidate| candidate == &approved_golden)
            .unwrap()
            .correct_count,
        0
    );

    report.classified_items[0].proposed_disposition = Disposition::AlignmentVersioning;
    let verifier_called = std::cell::Cell::new(false);
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |candidate| {
            verifier_called.set(true);
            candidate == &approved_golden
        }),
        Err(EvaluationError::SnapshotMismatch),
        "a source receipt cannot authorize changed predictions under unchanged source coordinates"
    );
    assert!(!verifier_called.get());
}

#[test]
fn rewriting_the_proposal_digest_cannot_reuse_an_independent_approval() {
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let mut golden = ReviewedGoldenSet {
        approval: approval(&report, report.snapshot_items.clone()),
        labels: vec![GoldenLabel::new("A", Disposition::AlignmentVersioning)],
    };
    let approved_golden = golden.clone();
    report.classified_items[0].proposed_disposition = Disposition::AlignmentVersioning;
    golden.approval.proposal_digest = classification_proposal_digest(&report);
    let imported_golden =
        serde_json::from_slice::<ReviewedGoldenSet>(&serde_json::to_vec(&golden).unwrap()).unwrap();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &imported_golden, |candidate| {
            candidate == &approved_golden
        }),
        Err(EvaluationError::UnverifiedApproval)
    );
}

#[test]
fn approval_binds_unreviewed_proposals_and_supporting_evidence_but_not_record_order() {
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            bibliographic("B", 1, "ontology evaluation"),
        ],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(&report, report.snapshot_items.clone()),
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    };
    report.classified_items.reverse();
    let evaluation =
        evaluate_reviewed_golden_set(&report, &golden, |candidate| candidate == &golden).unwrap();
    assert_eq!(evaluation.correct_count, 1);
    assert_eq!(evaluation.proposal_digest, golden.approval.proposal_digest);

    // B is outside the reviewed sample, but belongs to the approved proposal run.
    report.classified_items[0].evidence.field_values.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |_| {
            panic!("changed proposal evidence must fail before governance")
        }),
        Err(EvaluationError::SnapshotMismatch)
    );
}

#[test]
fn missing_proposal_binding_and_invalid_labels_fail_before_governance() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let mut golden = ReviewedGoldenSet {
        approval: approval(&report, report.snapshot_items.clone()),
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    };
    let mut legacy_json = serde_json::to_value(&golden).unwrap();
    legacy_json["approval"]
        .as_object_mut()
        .unwrap()
        .remove("proposal_digest");
    assert!(serde_json::from_value::<ReviewedGoldenSet>(legacy_json).is_err());

    golden.approval.proposal_digest.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |_| {
            panic!("unbound proposals must fail before governance")
        }),
        Err(EvaluationError::InvalidReview)
    );
    golden.approval.proposal_digest = classification_proposal_digest(&report);
    for (labels, expected_error) in [
        (
            vec![GoldenLabel::new(" ", Disposition::Generation)],
            EvaluationError::InvalidReview,
        ),
        (
            vec![GoldenLabel::new("A", Disposition::NeedsStewardReview)],
            EvaluationError::InvalidExpectedDisposition,
        ),
        (
            vec![GoldenLabel::new("absent", Disposition::Generation)],
            EvaluationError::UnknownItem,
        ),
        (
            vec![GoldenLabel::new("A", Disposition::Generation); 2],
            EvaluationError::DuplicateItem,
        ),
    ] {
        golden.labels = labels;
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &golden, |_| {
                panic!("invalid labels must fail before governance")
            }),
            Err(expected_error)
        );
    }
}

#[test]
fn malformed_proposal_identities_fail_before_governance() {
    for (replacement_key, replacement_version) in [("A", 1), ("B", 9), (" ", 1)] {
        let mut report = classify_snapshot(
            "9.0.6".into(),
            None,
            42,
            vec![
                bibliographic("A", 1, "ontology learning"),
                bibliographic("B", 1, "ontology evaluation"),
            ],
        );
        let golden = ReviewedGoldenSet {
            approval: approval(&report, report.snapshot_items.clone()),
            labels: vec![GoldenLabel::new("A", Disposition::Generation)],
        };
        report.classified_items[1].item_key = replacement_key.into();
        report.classified_items[1].item_version = replacement_version;
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &golden, |_| {
                panic!("malformed proposals must fail before governance")
            }),
            Err(EvaluationError::InvalidReview)
        );
    }
}
