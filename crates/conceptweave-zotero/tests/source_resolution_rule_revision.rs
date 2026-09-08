use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionReview,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
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

fn exact_resolution() -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: "SOURCE".into(),
        item_version: 3,
        library_version: 42,
        server_id: Some("local-server".into()),
        item_type: "attachment".into(),
        parent_item_key: String::new(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "The source remains useful independent evidence.".into(),
    }
}

#[test]
fn stored_source_resolution_rejects_blank_rule_revision() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![source_item()],
    );
    let review = prepare_source_resolution_review(&report, vec![exact_resolution()])
        .expect("the exact pending source is resolvable");
    let mut stored = serde_json::to_value(review).expect("review must serialize");
    stored["rule_revision"] = serde_json::json!(" \t\n");

    assert!(serde_json::from_value::<SourceResolutionReview>(stored).is_err());
}
