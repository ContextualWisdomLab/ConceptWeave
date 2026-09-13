use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_proposal_digest,
    classification_snapshot_digest, classify_typed_golden_snapshot, evaluate_reviewed_golden_set,
};

fn item(key: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
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
    }
}

fn report() -> conceptweave_zotero::GoldenSnapshot {
    classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", "ontology learning"),
            item("B", "ontology evaluation"),
            item("C", "unmatched"),
        ],
    )
}

fn golden(snapshot: &conceptweave_zotero::GoldenSnapshot, labels: Vec<GoldenLabel>) -> ReviewedGoldenSet {
    ReviewedGoldenSet {
        approval: GoldenSetApproval {
            receipt_id: "synthetic-review-1".into(),
            reviewer_subject: "synthetic-steward".into(),
            library_version: 42,
            rule_revision: "ontology-research-v2".into(),
            snapshot_digest: classification_snapshot_digest(snapshot),
            proposal_digest: classification_proposal_digest(snapshot),
            snapshot_items: ["A", "B", "C"]
                .into_iter()
                .map(|item_key| SnapshotItemRevision {
                    item_key: item_key.into(),
                    item_version: 1,
                })
                .collect(),
        },
        labels,
    }
}

fn verify_synthetic_approval(golden: &ReviewedGoldenSet) -> bool {
    golden.approval.receipt_id == "synthetic-review-1"
        && golden.approval.reviewer_subject == "synthetic-steward"
}

#[test]
fn reviewed_golden_set_reports_count_based_precision_and_recall_evidence() {
    let snapshot = report();
    let evaluation = evaluate_reviewed_golden_set(
        &snapshot,
        &golden(
            &snapshot,
            vec![
                GoldenLabel::new("A", Disposition::Generation),
                GoldenLabel::new("B", Disposition::AlignmentVersioning),
                GoldenLabel::new("C", Disposition::Generation),
            ],
        ),
        verify_synthetic_approval,
    )
    .unwrap();

    assert_eq!(evaluation.review_id, "synthetic-review-1");
    assert_eq!(evaluation.library_version, 42);
    assert_eq!(evaluation.rule_revision, "ontology-research-v2");
    assert!(evaluation.snapshot_digest.starts_with("sha256:"));
    assert_eq!(evaluation.reviewed_count, 3);
    assert_eq!(evaluation.correct_count, 1);
    assert_eq!(evaluation.abstention_count, 1);
    let generation = &evaluation.by_disposition[&Disposition::Generation];
    assert_eq!(
        (generation.true_positive, generation.predicted, generation.expected),
        (1, 1, 2)
    );
    let serialized = serde_json::to_value(&evaluation).unwrap();
    assert!(serialized.get("labels").is_none());
    assert!(serialized.get("item_key").is_none());
    assert!(serialized.get("reviewer_subject").is_none());
}

#[test]
fn reviewed_golden_set_rejects_stale_unknown_duplicate_and_unverified_labels() {
    let snapshot = report();
    assert_eq!(
        evaluate_reviewed_golden_set(&snapshot, &golden(&snapshot, vec![]), verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );

    let mut stale = golden(&snapshot, vec![GoldenLabel::new("A", Disposition::Generation)]);
    stale.approval.library_version += 1;
    assert_eq!(
        evaluate_reviewed_golden_set(&snapshot, &stale, verify_synthetic_approval),
        Err(EvaluationError::SnapshotMismatch)
    );

    assert_eq!(
        evaluate_reviewed_golden_set(
            &snapshot,
            &golden(&snapshot, vec![GoldenLabel::new("A", Disposition::Generation)]),
            |_| false,
        ),
        Err(EvaluationError::UnverifiedApproval)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &snapshot,
            &golden(
                &snapshot,
                vec![GoldenLabel::new("A", Disposition::NeedsStewardReview)],
            ),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::InvalidExpectedDisposition)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &snapshot,
            &golden(&snapshot, vec![GoldenLabel::new("missing", Disposition::Generation)]),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::UnknownItem)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &snapshot,
            &golden(
                &snapshot,
                vec![
                    GoldenLabel::new("A", Disposition::Generation),
                    GoldenLabel::new("A", Disposition::Generation),
                ],
            ),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::DuplicateItem)
    );
}
