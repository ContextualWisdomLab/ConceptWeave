use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    SnapshotItemRevision, ZoteroItem, classification_snapshot_digest, classify_snapshot,
    evaluate_reviewed_golden_set,
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

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
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

fn golden(labels: Vec<GoldenLabel>) -> ReviewedGoldenSet {
    ReviewedGoldenSet {
        approval: GoldenSetApproval {
            receipt_id: "synthetic-review-1".into(),
            reviewer_subject: "synthetic-steward".into(),
            library_version: 42,
            rule_revision: "ontology-research-v2".into(),
            snapshot_digest: classification_snapshot_digest(&report()),
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
    let report = report();
    assert_eq!(report.audit_summary.snapshot_item_count, 3);
    assert_eq!(report.audit_summary.bibliographic_item_count, 3);
    assert_eq!(report.audit_summary.proposed_disposition_count, 3);
    assert_eq!(report.audit_summary.provenance_complete_count, 3);
    assert_eq!(report.audit_summary.abstention_count, 1);
    assert_eq!(report.audit_summary.failure_count, 0);
    assert_eq!(
        report
            .audit_summary
            .disposition_counts
            .values()
            .sum::<usize>(),
        3
    );
    let evaluation = evaluate_reviewed_golden_set(
        &report,
        &golden(vec![
            GoldenLabel::new("A", Disposition::Generation),
            GoldenLabel::new("B", Disposition::AlignmentVersioning),
            GoldenLabel::new("C", Disposition::Generation),
        ]),
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
        (
            generation.true_positive,
            generation.predicted,
            generation.expected
        ),
        (1, 1, 2)
    );
    let serialized = serde_json::to_value(&evaluation).unwrap();
    assert!(serialized.get("labels").is_none());
    assert!(serialized.get("item_key").is_none());
    assert!(serialized.get("reviewer_subject").is_none());
}

#[test]
fn reviewed_golden_set_rejects_stale_unknown_and_duplicate_labels() {
    let report = report();

    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden(vec![]), verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );
    let mut blank = golden(vec![GoldenLabel::new(" ", Disposition::Generation)]);
    blank.approval.receipt_id.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &blank, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );
    blank.approval.receipt_id = "synthetic-review-1".into();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &blank, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );
    blank.approval.reviewer_subject.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &blank, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );
    let mut missing_revision = golden(vec![GoldenLabel::new("A", Disposition::Generation)]);
    missing_revision.approval.rule_revision.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &missing_revision, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );
    missing_revision.approval.rule_revision = "ontology-research-v2".into();
    missing_revision.approval.snapshot_digest.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &missing_revision, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );

    let mut stale = golden(vec![GoldenLabel::new("A", Disposition::Generation)]);
    stale.approval.library_version += 1;
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &stale, verify_synthetic_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    stale.approval.library_version = report.library_version;
    stale.approval.rule_revision = "older-rules".into();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &stale, verify_synthetic_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    stale.approval.rule_revision = report.rule_revision.into();
    stale.approval.snapshot_items[0].item_version += 1;
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &stale, verify_synthetic_approval),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut duplicate_snapshot = golden(vec![GoldenLabel::new("A", Disposition::Generation)]);
    duplicate_snapshot
        .approval
        .snapshot_items
        .push(duplicate_snapshot.approval.snapshot_items[0].clone());
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &duplicate_snapshot, verify_synthetic_approval),
        Err(EvaluationError::InvalidReview)
    );

    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![GoldenLabel::new("A", Disposition::Generation)]),
            |_| false
        ),
        Err(EvaluationError::UnverifiedApproval)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![GoldenLabel::new("A", Disposition::NeedsStewardReview)]),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::InvalidExpectedDisposition)
    );

    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![GoldenLabel::new("missing", Disposition::Generation)]),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::UnknownItem)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![
                GoldenLabel::new("A", Disposition::Generation),
                GoldenLabel::new("A", Disposition::Generation),
            ]),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::DuplicateItem)
    );

    let duplicate_report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", "ontology learning"),
            item("A", "ontology evaluation"),
        ],
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &duplicate_report,
            &golden(vec![GoldenLabel::new("A", Disposition::Generation)]),
            verify_synthetic_approval,
        ),
        Err(EvaluationError::InvalidReview)
    );
    for (error, fragment) in [
        (EvaluationError::InvalidReview, "invalid"),
        (EvaluationError::SnapshotMismatch, "snapshot"),
        (EvaluationError::UnverifiedApproval, "unverified"),
        (EvaluationError::InvalidExpectedDisposition, "abstention"),
        (EvaluationError::UnknownItem, "absent"),
        (EvaluationError::DuplicateItem, "duplicate"),
    ] {
        assert!(error.to_string().contains(fragment));
    }

    let blank_key_report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item(" ", "ontology learning")],
    );
    assert_eq!(blank_key_report.audit_summary.provenance_complete_count, 0);
}
