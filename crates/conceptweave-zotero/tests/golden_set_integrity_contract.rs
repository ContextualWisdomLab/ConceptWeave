use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_snapshot_digest, classify_snapshot,
    evaluate_reviewed_golden_set,
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
        evaluate_reviewed_golden_set(&report, &golden, |receipt| {
            receipt.receipt_id == "approved-review"
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
