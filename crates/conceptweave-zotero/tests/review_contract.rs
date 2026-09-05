use conceptweave_zotero::{AbstentionReason, Disposition, ItemData, ZoteroItem, classify_snapshot};

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
        source_record: None,
    }
}

#[test]
fn steward_abstention_reason_is_explicit_and_deterministic() {
    let blank = item("A", "", "");
    let multilingual = item("B", "온톨로지 정렬", "");
    let unmatched = item("C", "Other evidence", "");
    let matched = item("D", "Ontology alignment", "");

    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![blank, multilingual, unmatched, matched],
    );

    assert_eq!(
        report.classified_items[0].abstention_reason,
        Some(AbstentionReason::MissingClassificationMetadata)
    );
    assert_eq!(
        report.classified_items[1].abstention_reason,
        Some(AbstentionReason::UnsupportedRuleVocabulary)
    );
    assert_eq!(
        report.classified_items[2].abstention_reason,
        Some(AbstentionReason::NoDeterministicRuleMatch)
    );
    assert_eq!(
        report.classified_items[3].proposed_disposition,
        Disposition::AlignmentVersioning
    );
    assert_eq!(report.classified_items[3].abstention_reason, None);
}

#[test]
fn legacy_dx_doi_uri_collapses_into_the_same_duplicate_group() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", "First", "10.1/X"),
            item("B", "Second", "http://dx.doi.org/10.1/x"),
            item("C", "Third", "https://dx.doi.org/10.1/X"),
        ],
    );

    let doi_group = report
        .duplicate_candidates
        .iter()
        .find(|candidate| candidate.identity_kind == "doi")
        .expect("all DOI resolver forms must normalize to one candidate group");
    assert_eq!(doi_group.normalized_identity, "10.1/x");
    assert_eq!(doi_group.item_keys, ["A", "B", "C"]);
}
