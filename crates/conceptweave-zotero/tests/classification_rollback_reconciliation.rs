use conceptweave_zotero::{
    ClassificationItemState, ClassificationRollbackOperation, ClassificationRollbackState, ItemTag,
    reconcile_classification_rollback,
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
        (
            state(
                11,
                operation.expected_collection_keys.clone(),
                operation.expected_tags.clone(),
            ),
            ClassificationRollbackState::Indeterminate,
        ),
        (
            state(
                10,
                operation.collection_keys.clone(),
                operation.tags.clone(),
            ),
            ClassificationRollbackState::Indeterminate,
        ),
        (
            state(10, vec![" ".into()], operation.expected_tags.clone()),
            ClassificationRollbackState::Indeterminate,
        ),
        (
            state(
                10,
                operation.expected_collection_keys.clone(),
                vec![tag("Other")],
            ),
            ClassificationRollbackState::Indeterminate,
        ),
        (
            state(11, operation.collection_keys.clone(), vec![tag("Other")]),
            ClassificationRollbackState::Indeterminate,
        ),
    ];

    for (observed, expected) in cases {
        let receipt =
            reconcile_classification_rollback(&operation, |_| Ok::<_, ()>(observed.clone()));
        assert_eq!(receipt.state, expected);
        assert_eq!(receipt.operation, operation);
        assert_eq!(receipt.observed_state, Some(observed));
        assert_eq!(
            receipt.retry_operation,
            (expected == ClassificationRollbackState::Unchanged).then(|| operation.clone())
        );
    }
}

#[test]
fn later_reconciliation_preserves_read_failures_and_rejects_wrong_identity() {
    let operation = operation();
    let unreadable = reconcile_classification_rollback(&operation, |_| {
        Err::<ClassificationItemState, _>("read_failed")
    });
    assert_eq!(unreadable.state, ClassificationRollbackState::Indeterminate);
    assert_eq!(unreadable.operation, operation);
    assert!(unreadable.observed_state.is_none());
    assert!(unreadable.retry_operation.is_none());

    let mut wrong_server = state(
        10,
        operation.expected_collection_keys.clone(),
        operation.expected_tags.clone(),
    );
    wrong_server.server_id = "server-2".into();
    let mismatched =
        reconcile_classification_rollback(&operation, |_| Ok::<_, ()>(wrong_server.clone()));
    assert_eq!(mismatched.state, ClassificationRollbackState::Indeterminate);
    assert_eq!(mismatched.observed_state, Some(wrong_server));
    assert!(mismatched.retry_operation.is_none());

    let mut wrong_item = state(
        10,
        operation.expected_collection_keys.clone(),
        operation.expected_tags.clone(),
    );
    wrong_item.item_key = "BCDEFGHJ".into();
    assert_eq!(
        reconcile_classification_rollback(&operation, |_| Ok::<_, ()>(wrong_item)).state,
        ClassificationRollbackState::Indeterminate
    );

    let mut invalid_operations = Vec::new();
    let mut blank_server = operation.clone();
    blank_server.server_id = " ".into();
    invalid_operations.push(blank_server);
    let mut blank_item = operation.clone();
    blank_item.item_key = " ".into();
    invalid_operations.push(blank_item);
    let mut invalid_expected = operation.clone();
    invalid_expected.expected_collection_keys.push(" ".into());
    invalid_operations.push(invalid_expected);
    let mut invalid_restoration = operation.clone();
    invalid_restoration.collection_keys.push(" ".into());
    invalid_operations.push(invalid_restoration);

    for invalid_operation in invalid_operations {
        let invalid = reconcile_classification_rollback(
            &invalid_operation,
            |_| -> Result<ClassificationItemState, ()> {
                panic!("invalid reconciliation evidence must fail before reading")
            },
        );
        assert_eq!(invalid.state, ClassificationRollbackState::Indeterminate);
        assert!(invalid.observed_state.is_none());
        assert!(invalid.retry_operation.is_none());

        let serialized = serde_json::to_string(&invalid).unwrap();
        assert!(!serialized.to_ascii_lowercase().contains("api_key"));
        assert!(!serialized.contains("read_failed"));
    }
}
