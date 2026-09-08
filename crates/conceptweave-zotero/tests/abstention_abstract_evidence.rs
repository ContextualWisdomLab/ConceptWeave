use conceptweave_zotero::{ItemData, ZoteroItem, classify_snapshot};
use serde_json::Value;

fn item(title: &str, abstract_note: &str) -> ZoteroItem {
    ZoteroItem {
        key: "A".into(),
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

fn serialized_item(title: &str, abstract_note: &str) -> Value {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        42,
        vec![item(title, abstract_note)],
    );
    serde_json::to_value(
        report
            .classified_items
            .first()
            .expect("one bibliographic proposal"),
    )
    .expect("classification proposal serializes")
}

#[test]
fn abstention_retains_exact_nonempty_abstract_for_steward_replay() {
    let abstract_note = "Calibration evidence with latent variables and α thresholds.";
    let proposal = serialized_item("Unmatched calibration study", abstract_note);

    assert_eq!(
        proposal["proposed_disposition"],
        Value::String("needs_steward_review".into())
    );
    assert_eq!(
        proposal["abstention_reason"],
        Value::String("no_deterministic_rule_match".into())
    );
    assert_eq!(
        proposal["review_abstract_note"],
        Value::String(abstract_note.into()),
        "a steward must be able to replay the exact abstract that produced abstention"
    );
}

#[test]
fn review_abstract_is_omitted_when_no_steward_replay_context_is_needed() {
    let matched = serialized_item("ontology learning", "A generation study");
    assert!(matched.get("review_abstract_note").is_none());

    let empty_abstention = serialized_item("Unmatched calibration study", "");
    assert!(empty_abstention.get("review_abstract_note").is_none());
}

#[test]
fn conflicting_evidence_reuses_the_matched_abstract_instead_of_copying_it_twice() {
    let abstract_note = "Ontology learning evidence from a generation study.";
    let proposal = serialized_item("Ontology alignment evidence", abstract_note);

    assert_eq!(
        proposal["proposed_disposition"],
        Value::String("needs_steward_review".into())
    );
    assert_eq!(
        proposal["abstention_reason"],
        Value::String("conflicting_disposition_evidence".into())
    );
    assert_eq!(proposal["evidence"]["field_values"]["abstract_note"], abstract_note);
    assert!(
        proposal.get("review_abstract_note").is_none(),
        "matched abstract evidence already supplies steward replay context and must not be duplicated"
    );
}
