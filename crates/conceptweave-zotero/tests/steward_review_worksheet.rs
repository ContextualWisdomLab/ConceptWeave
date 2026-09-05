use conceptweave_zotero::{
    Disposition, ItemData, WorksheetError, ZoteroItem, build_steward_review_worksheet,
    classify_snapshot,
};

fn item(key: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
        source_record: None,
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "book".into(),
            title: title.into(),
            abstract_note: "sensitive abstract".into(),
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
        vec![item("B", "unmatched title"), item("A", "ontology learning")],
    )
}

#[test]
fn worksheet_is_snapshot_bound_complete_and_contains_no_bibliographic_text() {
    let report = report();

    let worksheet = build_steward_review_worksheet(&report).unwrap();

    assert_eq!(worksheet.library_version, 42);
    assert_eq!(worksheet.rule_revision, report.rule_revision);
    assert_eq!(worksheet.snapshot_digest, report.snapshot_digest);
    assert_eq!(worksheet.snapshot_items, report.snapshot_items);
    assert_eq!(
        worksheet
            .decisions
            .iter()
            .map(|decision| decision.item_key.as_str())
            .collect::<Vec<_>>(),
        ["A", "B"]
    );
    assert!(
        worksheet
            .decisions
            .iter()
            .all(|decision| decision.reviewed_disposition.is_none())
    );
    let serialized = serde_json::to_value(&worksheet).unwrap();
    let serialized = serialized.to_string();
    assert!(!serialized.contains("unmatched title"));
    assert!(!serialized.contains("sensitive abstract"));
    assert!(!serialized.contains("field_values"));
}

#[test]
fn worksheet_rejects_each_inconsistent_report_coordinate() {
    let mut invalid = report();
    invalid.rule_revision.clear();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.snapshot_digest.clear();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.observed_item_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.bibliographic_item_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.proposed_disposition_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.snapshot_item_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.provenance_complete_count -= 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.abstention_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.duplicate_candidate_count += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.failure_count = 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.audit_summary.disposition_counts.clear();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.classified_items[0].child_item_keys = vec![String::new()];
    invalid.audit_summary.provenance_complete_count -= 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.snapshot_items[0].item_key.clear();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.snapshot_items[1].item_key = invalid.snapshot_items[0].item_key.clone();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.classified_items[0].item_key.clear();
    invalid.audit_summary.provenance_complete_count -= 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.classified_items[1].item_key = invalid.classified_items[0].item_key.clone();
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid.classified_items[0].item_version += 1;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid
        .classified_items
        .iter_mut()
        .find(|item| item.proposed_disposition == Disposition::NeedsStewardReview)
        .unwrap()
        .abstention_reason = None;
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = report();
    invalid
        .classified_items
        .iter_mut()
        .find(|item| item.proposed_disposition != Disposition::NeedsStewardReview)
        .unwrap()
        .abstention_reason = Some(conceptweave_zotero::AbstentionReason::NoDeterministicRuleMatch);
    assert_eq!(
        build_steward_review_worksheet(&invalid),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn worksheet_rejects_a_report_with_duplicate_source_identity() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", "ontology learning"),
            item("A", "ontology learning"),
        ],
    );

    assert_eq!(
        build_steward_review_worksheet(&report),
        Err(WorksheetError::InvalidReport)
    );
    assert!(
        WorksheetError::InvalidReport
            .to_string()
            .contains("invalid")
    );
}
