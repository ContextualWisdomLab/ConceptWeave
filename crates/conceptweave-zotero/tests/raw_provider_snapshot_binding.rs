use conceptweave_zotero::{
    CapturedZoteroItem, ItemData, ZoteroItem, classify_captured_golden_snapshot,
    classify_typed_golden_snapshot, classification_snapshot_digest,
};
use serde_json::{Value, json};

fn provider_snapshot_digest(raw_item: Value) -> String {
    let item = CapturedZoteroItem::try_from(raw_item).unwrap();
    let snapshot = classify_captured_golden_snapshot("9.0.6".into(), None, 42, vec![item]);
    classification_snapshot_digest(&snapshot)
}

#[test]
fn snapshot_digest_binds_unmodeled_provider_metadata_at_every_item_level() {
    let original = json!({
        "key": "SYNTH001",
        "version": 7,
        "meta": {"parsedDate": "2025-01-01"},
        "data": {
            "itemType": "journalArticle",
            "title": "Ontology learning",
            "date": "2025-01-01",
            "creators": [{"creatorType": "author", "name": "Synthetic Author"}],
            "tags": [{"tag": "ontology", "type": 0}]
        }
    });
    let original_digest = provider_snapshot_digest(original.clone());
    for (pointer, replacement) in [
        ("/meta/parsedDate", json!("2026-01-01")),
        ("/data/date", json!("2026-01-01")),
        ("/data/creators/0/name", json!("Other Synthetic Author")),
        ("/data/tags/0/type", json!(1)),
    ] {
        let mut changed = original.clone();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert_ne!(provider_snapshot_digest(changed), original_digest, "{pointer}");
    }
}

#[test]
fn provider_object_order_is_canonical_but_field_presence_and_array_order_are_evidence() {
    let ordered: Value = serde_json::from_str(
        r#"{"key":"SYNTH001","version":7,"meta":{"a":1,"b":{"c":2,"d":3}},"data":{"itemType":"book","date":"2025","creators":[{"name":"Synthetic Author","creatorType":"author"}]}}"#,
    )
    .unwrap();
    let reordered: Value = serde_json::from_str(
        r#"{"data":{"creators":[{"creatorType":"author","name":"Synthetic Author"}],"date":"2025","itemType":"book"},"meta":{"b":{"d":3,"c":2},"a":1},"version":7,"key":"SYNTH001"}"#,
    )
    .unwrap();
    assert_eq!(provider_snapshot_digest(ordered), provider_snapshot_digest(reordered));

    let omitted = json!({"key": "SYNTH001", "version": 7, "data": {"itemType": "book"}});
    let omitted_digest = provider_snapshot_digest(omitted.clone());
    let mut explicit = omitted;
    explicit["data"]["title"] = json!("");
    assert_ne!(provider_snapshot_digest(explicit), omitted_digest);
}

#[test]
fn typed_fixture_digest_binds_actual_classifier_inputs_without_claiming_provider_authenticity() {
    fn item(title: &str) -> ZoteroItem {
        ZoteroItem {
            key: "SYNTH001".into(),
            version: 7,
            data: ItemData {
                item_type: "book".into(),
                title: title.into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }
    }
    let original = classify_typed_golden_snapshot("9.0.6".into(), None, 42, vec![item("Ontology learning")]);
    let changed = classify_typed_golden_snapshot("9.0.6".into(), None, 42, vec![item("Ontology alignment")]);
    assert_ne!(classification_snapshot_digest(&original), classification_snapshot_digest(&changed));

    let captured = CapturedZoteroItem::try_from(json!({
        "key": "SYNTH001", "version": 7, "data": {"itemType": "book", "title": "Ontology learning"}
    }))
    .unwrap();
    let provider = classify_captured_golden_snapshot("9.0.6".into(), None, 42, vec![captured]);
    assert_ne!(
        classification_snapshot_digest(&original),
        classification_snapshot_digest(&provider),
        "typed fixtures and provider-captured receipts use distinct domains"
    );
}

#[test]
fn captured_boundary_preserves_provider_shape_validation() {
    assert!(CapturedZoteroItem::try_from(json!(null)).is_err());
    for invalid in [
        json!({"key": 7, "version": 7, "data": {"itemType": "book"}}),
        json!({"key": "SYNTH001", "version": 7, "data": {}}),
        json!({"key": "SYNTH001", "version": 7, "data": {"itemType": false}}),
    ] {
        assert!(CapturedZoteroItem::try_from(invalid).is_err());
    }
}
