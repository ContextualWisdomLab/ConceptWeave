use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};

fn item(key: &str, title: &str, doi: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 11,
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
fn nested_supported_doi_wrappers_share_one_duplicate_identity() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        2,
        vec![
            item("A", "First record", "doi: https://doi.org/10.1/X"),
            item("B", "Second record", "10.1/x"),
        ],
    );

    let doi_candidates = report
        .duplicate_candidates
        .iter()
        .filter(|candidate| candidate.identity_kind == "doi")
        .collect::<Vec<_>>();
    assert_eq!(doi_candidates.len(), 1);
    assert_eq!(doi_candidates[0].normalized_identity, "10.1/x");
    assert_eq!(doi_candidates[0].item_keys, ["A", "B"]);
}
