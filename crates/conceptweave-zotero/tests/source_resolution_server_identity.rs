use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
};

fn item(key: &str, version: u64, item_type: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: item_type.into(),
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
fn source_resolution_fails_closed_when_snapshot_has_no_server_identity() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        42,
        vec![
            item("PAPER", 40, "journalArticle"),
            item("SOURCE", 41, "attachment"),
        ],
    );
    let resolution = PendingSourceResolution {
        item_key: "SOURCE".into(),
        item_version: 41,
        library_version: 42,
        server_id: None,
        item_type: "attachment".into(),
        parent_item_key: String::new(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "Retain as standalone evidence.".into(),
    };

    assert!(matches!(
        prepare_source_resolution_review(&report, vec![resolution]),
        Err(SourceResolutionError::MissingServerIdentity)
    ));
}

#[test]
fn source_resolution_rejects_blank_server_identity_even_without_pending_sources() {
    let report = classify_snapshot("10.0.1".into(), Some(" \t".into()), 42, vec![]);

    assert!(matches!(
        prepare_source_resolution_review(&report, vec![]),
        Err(SourceResolutionError::MissingServerIdentity)
    ));
}
