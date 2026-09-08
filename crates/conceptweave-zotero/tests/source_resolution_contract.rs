use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, ZoteroItem, classify_snapshot,
    prepare_source_resolution_review,
};

fn item(key: &str, version: u64, item_type: &str, parent_item: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: item_type.into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn resolution(
    key: &str,
    version: u64,
    item_type: &str,
    parent_item: &str,
) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        item_type: item_type.into(),
        parent_item_key: parent_item.into(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "The source remains useful independent evidence.".into(),
    }
}

#[test]
fn source_resolution_review_requires_the_exact_pending_snapshot_set() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("PAPER", 40, "journalArticle", ""),
            item("SOURCE", 41, "attachment", ""),
            item("NOTE", 42, "note", "SOURCE"),
        ],
    );

    assert!(prepare_source_resolution_review(&report, vec![]).is_err());

    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("SOURCE", 41, "attachment", ""),
            resolution("NOTE", 42, "note", "SOURCE"),
        ],
    )
    .expect("every pending source has one exact-snapshot resolution");

    assert_eq!(review.library_version, 42);
    assert_eq!(review.resolved_sources.len(), 2);
}
