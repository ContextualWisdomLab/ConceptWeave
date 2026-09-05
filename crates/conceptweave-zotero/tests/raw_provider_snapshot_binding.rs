use conceptweave_zotero::{ZoteroItem, classify_snapshot};
use serde_json::{Value, json};

fn snapshot_digest(raw_item: Value) -> String {
    let item: ZoteroItem = serde_json::from_value(raw_item).unwrap();
    classify_snapshot("9.0.6".into(), None, 42, vec![item]).snapshot_digest
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
    let original_digest = snapshot_digest(original.clone());

    for (pointer, replacement) in [
        ("/meta/parsedDate", json!("2026-01-01")),
        ("/data/date", json!("2026-01-01")),
        ("/data/creators/0/name", json!("Other Synthetic Author")),
        ("/data/tags/0/type", json!(1)),
    ] {
        let mut changed = original.clone();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert_ne!(
            snapshot_digest(changed),
            original_digest,
            "same-revision content change at {pointer} must invalidate the snapshot receipt"
        );
    }
}

#[test]
fn snapshot_digest_is_independent_of_provider_object_field_order() {
    let ordered: Value = serde_json::from_str(
        r#"{"key":"SYNTH001","version":7,"meta":{"a":1,"b":{"c":2,"d":3}},"data":{"itemType":"book","date":"2025","creators":[{"name":"Synthetic Author","creatorType":"author"}]}}"#,
    )
    .unwrap();
    let reordered: Value = serde_json::from_str(
        r#"{"data":{"creators":[{"creatorType":"author","name":"Synthetic Author"}],"date":"2025","itemType":"book"},"meta":{"b":{"d":3,"c":2},"a":1},"version":7,"key":"SYNTH001"}"#,
    )
    .unwrap();
    assert_eq!(snapshot_digest(ordered), snapshot_digest(reordered));
}
