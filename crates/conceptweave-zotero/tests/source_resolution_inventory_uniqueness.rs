use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
};

fn item(key: &str, version: u64) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
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
fn source_resolution_rejects_ambiguous_retained_inventory_identity() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![item("SOURCE", 3)],
    );
    report.unclassified_items.push(item("SOURCE", 4));

    let resolution = PendingSourceResolution {
        item_key: "SOURCE".into(),
        item_version: 3,
        library_version: 7,
        server_id: Some("local-server".into()),
        item_type: "attachment".into(),
        parent_item_key: String::new(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "Retain the exact source as independent evidence.".into(),
    };

    let error = prepare_source_resolution_review(&report, vec![resolution])
        .expect_err("duplicate retained records must not select an arbitrary source identity");
    assert_eq!(
        error,
        SourceResolutionError::AmbiguousInventory("SOURCE".into())
    );
    assert_eq!(
        error.to_string(),
        "pending source has ambiguous report inventory identity: SOURCE"
    );
}
