use conceptweave_zotero::{
    ClassificationReport, ItemData, WorksheetError, ZoteroItem, build_steward_review_worksheet,
    classify_snapshot,
};

#[test]
fn owner_only_report_roundtrip_preserves_the_review_workload() {
    let mut item = ZoteroItem {
        key: "ITEM".into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "ontology alignment".into(),
            abstract_note: "review context".into(),
            doi: "10.1000/example".into(),
            parent_item: String::new(),
            collections: vec!["COLLECTION".into()],
            tags: vec![],
        },
    };
    item.data.tags.push(conceptweave_zotero::ItemTag {
        tag: "ontology".into(),
        tag_type: Some(1),
    });
    let original = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
    let serialized = serde_json::to_vec(&original).unwrap();

    let restored: ClassificationReport = serde_json::from_slice(&serialized).unwrap();

    assert_eq!(restored.library_version, original.library_version);
    assert_eq!(restored.rule_revision, original.rule_revision);
    assert_eq!(restored.snapshot_digest, original.snapshot_digest);
    assert_eq!(
        build_steward_review_worksheet(&restored).unwrap(),
        build_steward_review_worksheet(&original).unwrap()
    );
}

#[test]
fn restored_report_rejects_child_provenance_outside_the_bound_snapshot() {
    let item = ZoteroItem {
        key: "ITEM".into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "ontology alignment".into(),
            abstract_note: "review context".into(),
            doi: "10.1000/example".into(),
            parent_item: String::new(),
            collections: vec!["COLLECTION".into()],
            tags: vec![],
        },
    };
    let original = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
    let serialized = serde_json::to_vec(&original).unwrap();
    let mut restored: ClassificationReport = serde_json::from_slice(&serialized).unwrap();

    restored.classified_items[0].child_item_keys = vec!["UNKNOWN_CHILD".into()];

    assert_eq!(
        build_steward_review_worksheet(&restored),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn restored_report_rejects_classified_or_reused_child_provenance() {
    let bibliographic = |key: &str| ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "ontology alignment".into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    };
    let child = ZoteroItem {
        key: "CHILD".into(),
        version: 3,
        data: ItemData {
            item_type: "note".into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: "PARENT_A".into(),
            collections: vec![],
            tags: vec![],
        },
    };
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("PARENT_A"), bibliographic("PARENT_B"), child],
    );
    let serialized = serde_json::to_vec(&report).unwrap();

    let mut classified_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    classified_child.classified_items[0].child_item_keys = vec!["PARENT_A".into()];
    assert_eq!(
        build_steward_review_worksheet(&classified_child),
        Err(WorksheetError::InvalidReport)
    );

    let mut reused_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    reused_child.classified_items[1].child_item_keys = vec!["CHILD".into()];
    assert_eq!(
        build_steward_review_worksheet(&reused_child),
        Err(WorksheetError::InvalidReport)
    );
}
