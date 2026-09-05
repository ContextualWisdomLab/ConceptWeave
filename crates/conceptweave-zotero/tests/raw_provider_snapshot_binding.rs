use conceptweave_zotero::{ZoteroItem, classify_snapshot};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

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

#[test]
fn snapshot_digest_preserves_omitted_versus_explicit_default_metadata() {
    let omitted = json!({"key": "SYNTH001", "version": 7, "data": {"itemType": "book"}});
    let omitted_digest = snapshot_digest(omitted.clone());
    for (field_name, explicit_default) in [
        ("title", json!("")),
        ("abstractNote", json!("")),
        ("DOI", json!("")),
        ("parentItem", json!("")),
        ("collections", json!([])),
        ("tags", json!([])),
    ] {
        let mut explicit = omitted.clone();
        explicit["data"][field_name] = explicit_default;
        assert_ne!(snapshot_digest(explicit), omitted_digest, "{field_name}");
    }
}

#[test]
fn snapshot_digest_binds_actual_classifier_inputs_after_provider_decode() {
    let original: ZoteroItem = serde_json::from_value(json!({
        "key": "SYNTH001", "version": 7,
        "data": {"itemType": "book", "title": "Ontology learning"}
    }))
    .unwrap();
    let original_digest =
        classify_snapshot("9.0.6".into(), None, 42, vec![original.clone()]).snapshot_digest;
    let mut changed = original;
    changed.data.title = "Ontology alignment".into();
    assert_ne!(
        classify_snapshot("9.0.6".into(), None, 42, vec![changed]).snapshot_digest,
        original_digest,
        "changed classifier input cannot retain the original source receipt"
    );
}

#[test]
fn source_capture_preserves_provider_shape_validation() {
    for invalid in [
        json!(null),
        json!({"key": 7, "version": 7, "data": {"itemType": "book"}}),
        json!({"key": "SYNTH001", "version": 7, "data": {}}),
    ] {
        assert!(serde_json::from_value::<ZoteroItem>(invalid).is_err());
    }
    assert!(
        serde_json::from_str::<ZoteroItem>(
            r#"{"key":"SYNTH001","version":7,"data":{"itemType":false}}"#
        )
        .is_err()
    );
}

#[test]
fn snapshot_digest_has_versioned_domain_separation() {
    let item: ZoteroItem = serde_json::from_value(json!({
        "key": "SYNTH001", "version": 7, "data": {"itemType": "book"}
    }))
    .unwrap();
    let unmarked_content = serde_json::to_vec(&[(&item.source_record, &item)]).unwrap();
    let unmarked_digest = format!("sha256:{:x}", Sha256::digest(unmarked_content));
    assert_ne!(
        classify_snapshot("9.0.6".into(), None, 42, vec![item]).snapshot_digest,
        unmarked_digest,
        "snapshot receipts must be separated from unversioned content hashes"
    );
}
