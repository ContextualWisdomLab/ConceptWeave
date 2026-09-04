use conceptweave_zotero::{
    ClassificationItemState, ClassificationRollbackOperation, ClassificationRollbackState,
    ItemTag, reconcile_classification_rollback,
};

fn tag(value: &str) -> ItemTag {
    ItemTag {
        tag: value.into(),
        tag_type: None,
    }
}

fn operation() -> ClassificationRollbackOperation {
    ClassificationRollbackOperation {
        server_id: "server-1".into(),
        item_key: "ABCDEFGH".into(),
        item_version: 10,
        expected_collection_keys: vec!["classified".into()],
        expected_tags: vec![tag("Classified")],
        collection_keys: vec!["source".into()],
        tags: vec![tag("Imported")],
    }
}

fn state(
    item_version: u64,
    collection_keys: Vec<String>,
    tags: Vec<ItemTag>,
) -> ClassificationItemState {
    ClassificationItemState {
        server_id: "server-1".into(),
        library_version: 99,
        item_key: "ABCDEFGH".into(),
        item_version,
        collection_keys,
        tags,
    }
}

#[test]
fn later_reconciliation_distinguishes_restored_unchanged_and_indeterminate_state() {
    let operation = operation();
    let cases = [
        (
            state(
                11,
                operation.collection_keys.clone(),
                operation.tags.clone(),
            ),
            ClassificationRollbackState::Restored,
        ),
        (
            state(
                10,
                operation.expected_collection_keys.clone(),
                operation.expected_tags.clone(),
            ),
            ClassificationRollbackState::Unchanged,
        ),
        (
            state(11, vec!["other".into()], operation.tags.clone()),
            ClassificationRollbackState::Indeterminate,
        ),
    ];

    for (observed, expected) in cases {
        assert_eq!(
            reconcile_classification_rollback(&operation, |_| Ok::<_, ()>(observed.clone())),
            Ok(expected)
        );
    }
}

#[test]
fn later_reconciliation_preserves_read_failures_and_rejects_wrong_identity() {
    let operation = operation();
    assert_eq!(
        reconcile_classification_rollback(&operation, |_| Err::<ClassificationItemState, _>(
            "read_failed"
        )),
        Err("read_failed")
    );

    let mut wrong_server = state(
        10,
        operation.expected_collection_keys.clone(),
        operation.expected_tags.clone(),
    );
    wrong_server.server_id = "server-2".into();
    assert_eq!(
        reconcile_classification_rollback(&operation, |_| Ok::<_, ()>(wrong_server.clone())),
        Ok(ClassificationRollbackState::Indeterminate)
    );
}
