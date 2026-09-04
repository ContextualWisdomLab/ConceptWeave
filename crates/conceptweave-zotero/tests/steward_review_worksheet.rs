use conceptweave_zotero::{
    ItemData, WorksheetError, ZoteroItem, build_steward_review_worksheet, classify_snapshot,
};

fn item(key: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
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

#[test]
fn worksheet_is_snapshot_bound_complete_and_contains_no_bibliographic_text() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("B", "unmatched title"), item("A", "ontology learning")],
    );

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
