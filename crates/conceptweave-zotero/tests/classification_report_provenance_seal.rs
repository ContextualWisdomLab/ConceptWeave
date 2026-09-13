use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};

fn pending_source() -> ZoteroItem {
    ZoteroItem {
        key: "SOURCE".into(),
        version: 3,
        data: ItemData {
            item_type: "attachment".into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

#[test]
fn constructor_bound_report_preserves_read_only_identity_and_json_shape() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![pending_source()],
    );

    assert_eq!(report.zotero_version(), "10.0.1");
    assert_eq!(report.api_version(), None);
    assert_eq!(report.schema_version(), None);
    assert_eq!(report.server_id(), Some("local-server"));
    assert_eq!(report.library_version(), 7);
    assert_eq!(report.rule_revision(), "ontology-research-v2");
    assert_eq!(report.observed_item_count(), 1);
    assert!(report.classified_items().is_empty());
    assert_eq!(report.unclassified_items()[0].version, 3);
    assert_eq!(report.pending_source_item_keys(), ["SOURCE"]);
    assert!(report.duplicate_candidates().is_empty());

    let json = serde_json::to_value(&report).unwrap();
    assert_eq!(json["library_version"], 7);
    assert_eq!(json["unclassified_items"][0]["version"], 3);
}
