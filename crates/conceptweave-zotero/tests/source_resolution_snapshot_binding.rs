use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionError, ZoteroItem, classify_snapshot,
    prepare_source_resolution_review,
};

fn source_item() -> ZoteroItem {
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

fn resolution_for_library(library_version: u64) -> PendingSourceResolution {
    serde_json::from_value(serde_json::json!({
        "item_key": "SOURCE",
        "item_version": 3,
        "library_version": library_version,
        "server_id": "local-server",
        "item_type": "attachment",
        "parent_item_key": "",
        "disposition": "retain_standalone_evidence",
        "reason": "The source remains useful independent evidence."
    }))
    .expect("source-resolution input must deserialize")
}

#[test]
fn source_resolution_cannot_be_rebound_to_a_different_library_snapshot() {
    let report_v7 = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![source_item()],
    );
    let report_v8 = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        8,
        vec![source_item()],
    );

    assert!(prepare_source_resolution_review(&report_v7, vec![resolution_for_library(7)]).is_ok());
    assert!(matches!(
        prepare_source_resolution_review(&report_v8, vec![resolution_for_library(7)]),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    let mut foreign_server = resolution_for_library(7);
    foreign_server.server_id = Some("other-server".into());
    assert!(matches!(
        prepare_source_resolution_review(&report_v7, vec![foreign_server]),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));
}

#[test]
fn source_resolution_rejects_blank_snapshot_identity_coordinates() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![source_item()],
    );

    let mut blank_zotero_version = report.clone();
    blank_zotero_version.zotero_version = " \t\n".into();
    assert!(matches!(
        prepare_source_resolution_review(&blank_zotero_version, vec![resolution_for_library(7)]),
        Err(SourceResolutionError::InvalidSnapshotIdentity)
    ));

    let mut blank_rule_revision = report;
    blank_rule_revision.rule_revision = " \t\n";
    assert!(matches!(
        prepare_source_resolution_review(&blank_rule_revision, vec![resolution_for_library(7)]),
        Err(SourceResolutionError::InvalidSnapshotIdentity)
    ));
}
