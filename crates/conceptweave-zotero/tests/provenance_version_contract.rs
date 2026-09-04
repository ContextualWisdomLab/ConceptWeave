use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};

#[test]
fn zotero_nine_zero_item_version_is_still_a_valid_provenance_coordinate() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![ZoteroItem {
            key: "UNSYNCED1".into(),
            version: 0,
            data: ItemData {
                item_type: "book".into(),
                title: "ontology learning".into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );

    assert_eq!(
        report.audit_summary.provenance_complete_count, 1,
        "Zotero 9 may report version 0 for never-synced items; zero is a valid observed version, not missing provenance"
    );
}
