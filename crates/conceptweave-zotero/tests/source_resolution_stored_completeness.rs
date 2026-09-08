use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionReview, ZoteroItem,
    classify_snapshot, prepare_source_resolution_review,
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

fn resolution(key: &str, version: u64, item_type: &str) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        library_version: 42,
        server_id: Some("local-server".into()),
        item_type: item_type.into(),
        parent_item_key: String::new(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "The source remains independent evidence.".into(),
    }
}

#[test]
fn stored_source_resolution_review_rejects_a_removed_pending_decision() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("NOTE", 42, "note"),
            item("SOURCE", 41, "attachment"),
        ],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("NOTE", 42, "note"),
            resolution("SOURCE", 41, "attachment"),
        ],
    )
    .expect("constructor requires the complete pending-source decision set");

    let mut stored = serde_json::to_value(review).expect("review must serialize");
    stored["resolved_sources"]
        .as_array_mut()
        .expect("resolved sources are an array")
        .remove(0);

    assert!(
        serde_json::from_value::<SourceResolutionReview>(stored).is_err(),
        "stored review must not regain typed status after one pending decision is removed"
    );
}
