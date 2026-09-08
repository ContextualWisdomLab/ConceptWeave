use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionRestoreError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review, restore_source_resolution_review,
};

fn pending_item() -> ZoteroItem {
    ZoteroItem {
        key: "SOURCE23".into(),
        version: 41,
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

fn valid_review_json() -> (
    conceptweave_zotero::ClassificationReport,
    serde_json::Value,
) {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![pending_item()],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![PendingSourceResolution {
            item_key: "SOURCE23".into(),
            item_version: 41,
            library_version: 42,
            server_id: Some("local-server".into()),
            item_type: "attachment".into(),
            parent_item_key: String::new(),
            disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
            reason: "Retain as standalone evidence.".into(),
        }],
    )
    .expect("the exact pending source is resolvable");
    (
        report,
        serde_json::to_value(review).expect("review must serialize"),
    )
}

fn assert_invalid(
    report: &conceptweave_zotero::ClassificationReport,
    value: serde_json::Value,
) {
    let stored = serde_json::to_vec(&value).expect("stored value must serialize");
    assert!(matches!(
        restore_source_resolution_review(report, &stored),
        Err(SourceResolutionRestoreError::InvalidStoredArtifact)
    ));
}

#[test]
fn stored_source_resolution_review_rejects_unknown_wire_fields() {
    let (report, serialized) = valid_review_json();

    let mut top_level = serialized.clone();
    top_level["approval"] = serde_json::json!("published");
    assert_invalid(&report, top_level);

    let mut expected_identity = serialized.clone();
    expected_identity["expected_source_identities"][0]["source_digest"] =
        serde_json::json!("legacy-untrusted-digest");
    assert_invalid(&report, expected_identity);

    let mut decision = serialized;
    decision["resolved_sources"][0]["semantic_authority"] = serde_json::json!(true);
    assert_invalid(&report, decision);
}
