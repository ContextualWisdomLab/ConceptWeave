use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_proposal_digest,
    classification_snapshot_digest, classify_snapshot, evaluate_reviewed_golden_set,
    validate_classification_report,
};
use sha2::{Digest, Sha256};

#[test]
fn legacy_proposal_receipt_is_rejected_without_calling_governance() {
    let report = scope_report();
    let mut golden = scope_golden(&report);
    let mut proposals = report.classified_items.iter().collect::<Vec<_>>();
    proposals.sort_by_key(|item| (&item.item_key, item.item_version));
    let old_bytes =
        serde_json::to_vec(&("conceptweave-classification-proposals-v1", proposals)).unwrap();
    golden.approval.proposal_digest = format!("sha256:{:x}", Sha256::digest(old_bytes));
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |_| panic!(
            "legacy receipt reached governance"
        )),
        Err(EvaluationError::SnapshotMismatch)
    );
}

#[test]
fn empty_and_unresolved_scope_remain_valid_without_becoming_reviewed_papers() {
    for (items, pending) in [
        (vec![], vec![]),
        (vec![child_note("S", 0, "")], vec!["S"]),
        (vec![child_note("C", 1, "MISSING")], vec!["C"]),
        (
            vec![child_note("C", 1, "D"), child_note("D", 1, "C")],
            vec!["C", "D"],
        ),
        (vec![child_note("C", 1, "C")], vec!["C"]),
    ] {
        let report = classify_snapshot("10.0.1".into(), None, 42, items);
        assert_eq!(validate_classification_report(&report), Ok(()));
        assert_eq!(report.pending_source_item_keys, pending);
        assert!(report.classified_items.is_empty());
    }
}

#[test]
fn blank_snapshot_identity_is_rejected_before_governance() {
    let mut report = scope_report();
    report.snapshot_items[0].item_key = " \t\n".into();
    let golden = scope_golden(&report);
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |_| panic!(
            "invalid identity reached governance"
        )),
        Err(EvaluationError::InvalidReview)
    );
}

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

fn scope_report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "10.0.1".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            child_note("C", 1, "A"),
            child_note("S", 0, ""),
            child_note("T", 1, ""),
        ],
    )
}

fn scope_golden(report: &conceptweave_zotero::ClassificationReport) -> ReviewedGoldenSet {
    ReviewedGoldenSet {
        approval: approval(report, report.snapshot_items.clone()),
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    }
}

#[test]
fn malformed_source_scope_fails_before_approval_even_with_recomputed_receipt() {
    for mutation in [
        "count",
        "snapshot_count",
        "missing",
        "duplicate",
        "overlap",
        "unknown",
        "revision",
        "blank",
        "top_level_book",
        "blank_type",
        "proposal_type",
        "proposal_blank_type",
        "future_snapshot",
        "pending_missing",
        "pending_extra",
        "pending_duplicate",
        "child_missing",
        "child_extra",
        "child_duplicate",
        "parent_changed",
    ] {
        let mut report = scope_report();
        match mutation {
            "count" => report.observed_item_count += 1,
            "snapshot_count" => {
                report.snapshot_items.pop();
            }
            "missing" => {
                report.unclassified_items.pop();
            }
            "duplicate" => report
                .unclassified_items
                .push(report.unclassified_items[0].clone()),
            "overlap" => report.unclassified_items[0].key = "A".into(),
            "unknown" => report.unclassified_items[0].key = "unknown".into(),
            "revision" => report.unclassified_items[0].version += 1,
            "blank" => report.unclassified_items[0].key = " ".into(),
            "top_level_book" => report.unclassified_items[1].data.item_type = "book".into(),
            "blank_type" => report.unclassified_items[0].data.item_type.clear(),
            "proposal_type" => report.classified_items[0].item_type = "attachment".into(),
            "proposal_blank_type" => report.classified_items[0].item_type.clear(),
            "future_snapshot" => {
                report.snapshot_items[0].item_version = 43;
                report.classified_items[0].item_version = 43;
            }
            "pending_missing" => report.pending_source_item_keys.clear(),
            "pending_extra" => report.pending_source_item_keys.push("A".into()),
            "pending_duplicate" => report.pending_source_item_keys.push("S".into()),
            "child_missing" => report.classified_items[0].child_item_keys.clear(),
            "child_extra" => report.classified_items[0].child_item_keys.push("S".into()),
            "child_duplicate" => report.classified_items[0].child_item_keys.push("C".into()),
            "parent_changed" => report.unclassified_items[0].data.parent_item = "missing".into(),
            _ => unreachable!(),
        }
        let golden = scope_golden(&report);
        let calls = std::cell::Cell::new(0);
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &golden, |_| {
                calls.set(calls.get() + 1);
                true
            }),
            Err(EvaluationError::InvalidReview),
            "mutation {mutation}"
        );
        assert_eq!(calls.get(), 0, "mutation {mutation}");
    }
}

#[test]
fn source_metadata_mutations_invalidate_the_original_approval_before_verification() {
    for mutation in [
        "title",
        "abstract",
        "doi",
        "tags",
        "collections",
        "type",
        "parent",
    ] {
        let mut report = scope_report();
        let golden = scope_golden(&report);
        let source = &mut report.unclassified_items[1];
        match mutation {
            "title" => source.data.title = "changed evidence".into(),
            "abstract" => source.data.abstract_note = "changed evidence".into(),
            "doi" => source.data.doi = "10.1/changed".into(),
            "tags" => source.data.tags.push(conceptweave_zotero::ItemTag {
                tag: "changed".into(),
            }),
            "collections" => source.data.collections.push("changed".into()),
            "type" => source.data.item_type = "attachment".into(),
            "parent" => {
                source.data.parent_item = "C".into();
                report.pending_source_item_keys = vec!["T".into()];
            }
            _ => unreachable!(),
        }
        let calls = std::cell::Cell::new(0);
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &golden, |_| {
                calls.set(calls.get() + 1);
                true
            }),
            Err(EvaluationError::SnapshotMismatch),
            "mutation {mutation}"
        );
        assert_eq!(calls.get(), 0);
    }
}

#[test]
fn rewritten_source_scope_receipt_still_requires_independent_approval() {
    let mut report = scope_report();
    let mut golden = scope_golden(&report);
    let approved = golden.clone();
    report.unclassified_items[1].data.title = "changed evidence".into();
    golden.approval.proposal_digest = classification_proposal_digest(&report);
    let calls = std::cell::Cell::new(0);
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden, |candidate| {
            calls.set(calls.get() + 1);
            candidate == &approved
        }),
        Err(EvaluationError::UnverifiedApproval)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn valid_pending_source_scope_is_order_independent_and_not_additional_paper_labels() {
    let mut report = scope_report();
    let golden = scope_golden(&report);
    report.unclassified_items.reverse();
    report.pending_source_item_keys.reverse();
    report.snapshot_items.reverse();
    let result =
        evaluate_reviewed_golden_set(&report, &golden, |candidate| candidate == &golden).unwrap();
    assert_eq!(result.reviewed_count, 1);
    assert_eq!(result.correct_count, 1);
    assert_eq!(
        classification_proposal_digest(&report),
        golden.approval.proposal_digest
    );
    assert_eq!(report.pending_source_item_keys.len(), 2);
}
