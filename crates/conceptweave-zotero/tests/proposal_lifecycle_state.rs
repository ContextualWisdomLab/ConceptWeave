use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};

#[test]
fn serialized_research_proposal_exposes_truth_and_publication_state_separately() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        42,
        vec![ZoteroItem {
            key: "A".into(),
            version: 7,
            data: ItemData {
                item_type: "journalArticle".into(),
                title: "Ontology alignment".into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );

    let serialized = serde_json::to_value(&report).expect("classification report must serialize");
    let proposal = &serialized["classified_items"][0];

    assert_eq!(proposal["truth_status"], "proposed");
    assert_eq!(proposal["publication_state"], "proposed");
    assert_ne!(proposal["truth_status"], "authoritative");
    assert_ne!(proposal["publication_state"], "published");
}
