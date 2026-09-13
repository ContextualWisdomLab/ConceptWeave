use conceptweave_zotero::{AbstentionReason, ItemData, ZoteroItem, classify_snapshot};

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

fn abstention_reason(title: &str, abstract_note: &str) -> Option<AbstentionReason> {
    classify_snapshot("10.0.1".into(), None, 42, vec![item(title, abstract_note)])
        .classified_items()
        .iter()
        .next()
        .expect("one bibliographic proposal")
        .abstention_reason
}

#[test]
fn ordinary_non_ascii_characters_do_not_imply_unsupported_vocabulary() {
    assert_eq!(
        abstention_reason("Analysis of naïve Bayes", ""),
        Some(AbstentionReason::NoDeterministicRuleMatch)
    );
    assert_eq!(
        abstention_reason("Unmatched study", "Estimated α threshold"),
        Some(AbstentionReason::NoDeterministicRuleMatch)
    );
}

#[test]
fn wholly_non_ascii_alphabetic_metadata_remains_explicitly_unsupported() {
    assert_eq!(
        abstention_reason("온톨로지 정렬", ""),
        Some(AbstentionReason::UnsupportedRuleVocabulary)
    );
}

#[test]
fn punctuation_only_metadata_is_not_unsupported_vocabulary() {
    assert_eq!(
        abstention_reason("---", "!!!"),
        Some(AbstentionReason::NoDeterministicRuleMatch)
    );
}
