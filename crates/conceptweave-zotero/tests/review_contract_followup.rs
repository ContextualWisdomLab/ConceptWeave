use conceptweave_zotero::{AbstentionReason, Disposition, ItemData, ZoteroItem, classify_snapshot};

fn item(key: &str, title: &str, abstract_note: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 11,
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
fn conflicting_specific_rule_families_abstain_for_steward_review() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "Ontology matching and ontology learning", "")],
    );

    let classified = &report.classified_items[0];
    assert_eq!(
        classified.proposed_disposition,
        Disposition::NeedsStewardReview
    );
    assert_eq!(
        classified.abstention_reason,
        Some(AbstentionReason::ConflictingDispositionEvidence)
    );
    assert_eq!(
        classified.evidence.matched_phrases,
        ["ontology learning", "ontology matching"]
    );
}

#[test]
fn matched_abstract_value_is_preserved_for_replayable_review() {
    let abstract_note = "We evaluate ontology alignment under schema drift.";
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "Uninformative title", abstract_note)],
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
            .get("abstract_note")
            .map(String::as_str),
        Some(abstract_note)
    );
}
