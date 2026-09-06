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
fn worksheet_identity_binds_review_context_and_retained_source_metadata() {
    let mut source_item = item("source", "synthetic source");
    source_item.data.item_type = "attachment".into();
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "unmatched"), source_item],
    );
    let original = build_steward_review_worksheet(&report).unwrap();
    report.unclassified_items[0].data.title.push_str(" changed");
    let source_changed = build_steward_review_worksheet(&report).unwrap();
    assert_ne!(original, source_changed);
    report.classified_items[0].review_abstract_note = Some("changed review context".into());
    let context_changed = build_steward_review_worksheet(&report).unwrap();
    assert_ne!(source_changed, context_changed);
    let serialized = serde_json::to_value(&context_changed).unwrap();
    assert_eq!(
        serialized["proposal_digest"],
        conceptweave_zotero::classification_proposal_digest(&report)
    );
}

#[test]
fn worksheet_cannot_deserialize_without_proposal_binding() {
    let worksheet = build_steward_review_worksheet(&report()).unwrap();
    let mut serialized = serde_json::to_value(&worksheet).unwrap();
    serialized
        .as_object_mut()
        .unwrap()
        .remove("proposal_digest");
    assert!(
        serde_json::from_value::<conceptweave_zotero::StewardReviewWorksheet>(serialized).is_err()
    );
}

#[test]
fn worksheet_preserves_complete_inventory_without_requiring_source_resolution() {
    for parent_key in ["", "missing", "source", "A"] {
        let mut source_item = item("source", "synthetic source");
        source_item.data.item_type = "attachment".into();
        source_item.data.parent_item = parent_key.into();
        let report = classify_snapshot(
            "9.0.6".into(),
            None,
            42,
            vec![item("A", "ontology learning"), source_item],
        );
        let worksheet = build_steward_review_worksheet(&report).unwrap();
        assert_eq!(worksheet.snapshot_items, report.snapshot_items);
        assert_eq!(worksheet.decisions.len(), 1);
        assert_eq!(worksheet.decisions[0].reviewed_disposition, None);
    }
}

#[test]
fn worksheet_rejects_omitted_retained_source_inventory() {
    let mut source_item = item("source", "synthetic source");
    source_item.data.item_type = "attachment".into();
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "ontology learning"), source_item],
    );
    report.unclassified_items.clear();
    assert_eq!(
        build_steward_review_worksheet(&report),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn worksheet_rejects_hidden_pending_sources() {
    let mut source_item = item("source", "synthetic source");
    source_item.data.item_type = "attachment".into();
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "ontology learning"), source_item],
    );
    report.pending_source_item_keys.clear();
    assert_eq!(
        build_steward_review_worksheet(&report),
        Err(WorksheetError::InvalidReport)
    );
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
