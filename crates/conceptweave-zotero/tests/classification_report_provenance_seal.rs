use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, ZoteroItem, classify_snapshot,
    prepare_source_resolution_review,
};

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
fn coherently_mutated_report_cannot_be_admitted_as_the_original_snapshot() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![pending_source()],
    );

    report.zotero_version = "fabricated-version".into();
    report.server_id = Some("fabricated-server".into());
    report.library_version = 8;
    report.rule_revision = "fabricated-rule";
    report.unclassified_items[0].version = 4;

    let result = prepare_source_resolution_review(
        &report,
        vec![PendingSourceResolution {
            item_key: "SOURCE".into(),
            item_version: 4,
            library_version: 8,
            item_type: "attachment".into(),
            parent_item_key: String::new(),
            disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
            reason: "retain exact source evidence".into(),
        }],
    );

    assert!(
        result.is_err(),
        "a caller must not manufacture a new trusted snapshot by coherently mutating report coordinates and retained inventory"
    );
}
