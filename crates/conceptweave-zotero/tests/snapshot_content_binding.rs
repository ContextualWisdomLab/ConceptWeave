use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_proposal_digest, classify_snapshot,
    evaluate_reviewed_golden_set,
};

fn item(title: &str) -> ZoteroItem {
    ZoteroItem {
        key: "A".into(),
        version: 1,
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

#[test]
fn golden_approval_rejects_same_revision_coordinates_with_changed_snapshot_content() {
    let changed_report =
        classify_snapshot("9.0.6".into(), None, 42, vec![item("ontology evaluation")]);
    let golden = ReviewedGoldenSet {
        approval: GoldenSetApproval {
            receipt_id: "review-original-snapshot".into(),
            reviewer_subject: "synthetic-steward".into(),
            library_version: 42,
            rule_revision: "ontology-research-v2".into(),
            snapshot_digest: "sha256:approved-original-content".into(),
            proposal_digest: classification_proposal_digest(&changed_report),
            snapshot_items: vec![SnapshotItemRevision {
                item_key: "A".into(),
                item_version: 1,
                parent_item_key: None,
            }],
        },
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    };

    assert_eq!(
        evaluate_reviewed_golden_set(&changed_report, &golden, |_| true),
        Err(EvaluationError::SnapshotMismatch),
        "item key/version coordinates alone cannot bind a Zotero 9 local snapshot whose content changed without a synced-version change"
    );
}
