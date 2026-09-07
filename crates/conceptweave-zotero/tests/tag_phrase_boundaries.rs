use conceptweave_zotero::{Disposition, ItemData, ItemTag, ZoteroItem, classify_snapshot};

fn item_with_tags(tags: &[&str]) -> ZoteroItem {
    ZoteroItem {
        key: "A".into(),
        version: 11,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "Uninformative title".into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: tags
                .iter()
                .map(|tag| ItemTag {
                    tag: (*tag).into(),
                })
                .collect(),
        },
    }
}

#[test]
fn separate_tags_do_not_synthesize_a_multiword_rule_phrase() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        2,
        vec![item_with_tags(&["ontology", "alignment"])],
    );

    let classified = &report.classified_items[0];
    assert_eq!(classified.proposed_disposition, Disposition::AdjacentEvidence);
    assert!(!classified.evidence.matched_phrases.contains(&"ontology alignment"));
}

#[test]
fn one_tag_containing_the_complete_phrase_still_matches_exactly() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        2,
        vec![item_with_tags(&["ontology alignment"])],
    );

    let classified = &report.classified_items[0];
    assert_eq!(
        classified.proposed_disposition,
        Disposition::AlignmentVersioning
    );
    assert_eq!(
        classified.evidence.field_values.get("tags").map(String::as_str),
        Some("ontology alignment")
    );
    assert!(classified.evidence.matched_phrases.contains(&"ontology alignment"));
}
