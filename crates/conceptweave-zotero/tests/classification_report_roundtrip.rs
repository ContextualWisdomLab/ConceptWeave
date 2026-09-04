use conceptweave_zotero::{
    ClassificationReport, ItemData, ZoteroItem, build_steward_review_worksheet, classify_snapshot,
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
