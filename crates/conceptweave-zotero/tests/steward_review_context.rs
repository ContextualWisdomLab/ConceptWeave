use conceptweave_zotero::{Disposition, ItemData, ZoteroItem, classify_snapshot};

fn item(key: &str, title: &str, abstract_note: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: abstract_note.into(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

#[test]
fn abstentions_retain_only_the_abstract_needed_for_local_steward_review() {
    let review_abstract = "A domain-specific vocabulary outside the deterministic rules.";
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("REVIEW01", "Unmatched domain study", review_abstract),
            item(
                "DECIDED1",
                "Ontology learning",
                "This abstract must not be copied into review-only context.",
            ),
            item("EMPTY001", "Unmatched title", ""),
        ],
    );

    let review = &report.classified_items[0];
    assert_eq!(review.proposed_disposition, Disposition::NeedsStewardReview);
    assert_eq!(review.review_abstract_note.as_deref(), Some(review_abstract));

    let decided = &report.classified_items[1];
    assert_eq!(decided.proposed_disposition, Disposition::Generation);
    assert!(decided.review_abstract_note.is_none());

    let empty = &report.classified_items[2];
    assert_eq!(empty.proposed_disposition, Disposition::NeedsStewardReview);
    assert!(empty.review_abstract_note.is_none());
}
