use conceptweave_zotero::{Disposition, ItemData, ItemTag, ZoteroItem, classify_snapshot};
use serde_json::json;

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
                .map(|tag| ItemTag { tag: (*tag).into() })
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
    assert_eq!(
        classified.proposed_disposition,
        Disposition::AdjacentEvidence
    );
    assert!(
        !classified
            .evidence
            .matched_phrases
            .contains(&"ontology alignment")
    );
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
        classified
            .evidence
            .field_values
            .get("tags")
            .map(String::as_str),
        Some("ontology alignment")
    );
    assert!(
        classified
            .evidence
            .matched_phrases
            .contains(&"ontology alignment")
    );
}

#[test]
fn every_matching_tag_is_retained_as_explicit_evidence() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        2,
        vec![item_with_tags(&[
            "ontology alignment",
            "unrelated note",
            "ontology learning",
        ])],
    );

    let classified = &report.classified_items[0];
    assert_eq!(
        classified.proposed_disposition,
        Disposition::NeedsStewardReview
    );
    let serialized = serde_json::to_value(classified).expect("proposal serializes");
    assert_eq!(
        serialized["abstention_reason"],
        json!("conflicting_disposition_evidence")
    );
    assert_eq!(
        serialized["evidence"]["matched_tag_values"],
        json!(["ontology alignment", "ontology learning"]),
        "all and only source tags that matched rule phrases must remain explicit replay evidence"
    );
}

#[test]
fn matching_tag_evidence_preserves_source_order_and_deduplicates_equal_values() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        2,
        vec![item_with_tags(&[
            "ontology learning",
            "ontology alignment",
            "ontology learning",
            "unrelated note",
        ])],
    );

    let classified = &report.classified_items[0];
    assert_eq!(
        classified.proposed_disposition,
        Disposition::NeedsStewardReview
    );
    assert_eq!(
        classified.evidence.matched_tag_values,
        ["ontology learning", "ontology alignment"]
    );
}
