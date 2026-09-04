use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, ItemData, ReviewedGoldenSet, ZoteroItem,
    classify_snapshot, evaluate_reviewed_golden_set,
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
        review_id: "synthetic-review-1".into(),
        library_version: 42,
        rule_revision: "ontology-research-v2".into(),
        labels,
    }
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
    )
    .unwrap();

    assert_eq!(evaluation.review_id, "synthetic-review-1");
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
}

#[test]
fn reviewed_golden_set_rejects_stale_unknown_and_duplicate_labels() {
    let report = report();

    assert_eq!(
        evaluate_reviewed_golden_set(&report, &golden(vec![])),
        Err(EvaluationError::InvalidReview)
    );
    let mut blank = golden(vec![GoldenLabel::new(" ", Disposition::Generation)]);
    blank.review_id.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &blank),
        Err(EvaluationError::InvalidReview)
    );
    blank.review_id = "synthetic-review-1".into();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &blank),
        Err(EvaluationError::InvalidReview)
    );
    let mut missing_revision = golden(vec![GoldenLabel::new("A", Disposition::Generation)]);
    missing_revision.rule_revision.clear();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &missing_revision),
        Err(EvaluationError::InvalidReview)
    );

    let mut stale = golden(vec![GoldenLabel::new("A", Disposition::Generation)]);
    stale.library_version += 1;
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &stale),
        Err(EvaluationError::SnapshotMismatch)
    );
    stale.library_version = report.library_version;
    stale.rule_revision = "older-rules".into();
    assert_eq!(
        evaluate_reviewed_golden_set(&report, &stale),
        Err(EvaluationError::SnapshotMismatch)
    );

    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![GoldenLabel::new("missing", Disposition::Generation)])
        ),
        Err(EvaluationError::UnknownItem)
    );
    assert_eq!(
        evaluate_reviewed_golden_set(
            &report,
            &golden(vec![
                GoldenLabel::new("A", Disposition::Generation),
                GoldenLabel::new("A", Disposition::Generation),
            ])
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
            &golden(vec![GoldenLabel::new("A", Disposition::Generation)])
        ),
        Err(EvaluationError::InvalidReview)
    );
    for (error, fragment) in [
        (EvaluationError::InvalidReview, "invalid"),
        (EvaluationError::SnapshotMismatch, "snapshot"),
        (EvaluationError::UnknownItem, "absent"),
        (EvaluationError::DuplicateItem, "duplicate"),
    ] {
        assert!(error.to_string().contains(fragment));
    }

    let mut incomplete = item("A", "ontology learning");
    incomplete.version = 0;
    let incomplete_report = classify_snapshot("9.0.6".into(), None, 42, vec![incomplete]);
    assert_eq!(incomplete_report.audit_summary.provenance_complete_count, 0);
    let blank_key_report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item(" ", "ontology learning")],
    );
    assert_eq!(blank_key_report.audit_summary.provenance_complete_count, 0);
}
