use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};
use serde_json::{Value, json};

fn item(key: &str, title: &str, doi: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: String::new(),
            doi: doi.into(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

#[test]
fn doi_duplicate_candidate_retains_exact_source_values_by_item_key() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        42,
        vec![
            item("A", "First paper", "doi: https://doi.org/10.1/X"),
            item("B", "Second paper", "10.1/x"),
        ],
    );
    let serialized = serde_json::to_value(report).expect("classification report serializes");
    let doi_candidate = serialized["duplicate_candidates"]
        .as_array()
        .expect("duplicate candidates are an array")
        .iter()
        .find(|candidate| candidate["identity_kind"] == Value::String("doi".into()))
        .expect("one DOI duplicate candidate");

    assert_eq!(doi_candidate["normalized_identity"], "10.1/x");
    assert_eq!(doi_candidate["item_keys"], json!(["A", "B"]));
    assert_eq!(
        doi_candidate["source_identity_values"],
        json!({
            "A": "doi: https://doi.org/10.1/X",
            "B": "10.1/x"
        }),
        "the immutable duplicate candidate must retain the exact DOI value observed for each source item"
    );
}
