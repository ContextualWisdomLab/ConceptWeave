use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
};

fn source_item(key: &str, version: u64) -> ZoteroItem {
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

fn resolution(key: &str, version: u64) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        library_version: 42,
        server_id: Some("local-server".into()),
        item_type: "attachment".into(),
        parent_item_key: String::new(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "Retain as standalone evidence.".into(),
    }
}

#[test]
fn source_resolution_rejects_duplicate_report_pending_keys() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![source_item("2A3B4C5D", 41)],
    );
    report.pending_source_item_keys = vec!["2A3B4C5D".into(), "2A3B4C5D".into()];

    assert!(
        prepare_source_resolution_review(&report, vec![resolution("2A3B4C5D", 41)]).is_err(),
        "a duplicated report pending-key sequence must not regain typed review status"
    );
}

#[test]
fn source_resolution_rejects_noncanonical_report_pending_key_order() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![source_item("2A3B4C5D", 40), source_item("3A4B5C6D", 41)],
    );
    report.pending_source_item_keys = vec!["3A4B5C6D".into(), "2A3B4C5D".into()];

    assert!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("2A3B4C5D", 40), resolution("3A4B5C6D", 41)],
        )
        .is_err(),
        "a noncanonical report pending-key sequence must fail before a trusted review is returned"
    );
}

#[test]
fn source_resolution_rejects_empty_report_pending_key_before_typed_review() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![source_item("", 41)],
    );
    report.pending_source_item_keys = vec![String::new()];

    assert!(matches!(
        prepare_source_resolution_review(&report, vec![resolution("", 41)]),
        Err(SourceResolutionError::InvalidPendingKeySet)
    ));
}
