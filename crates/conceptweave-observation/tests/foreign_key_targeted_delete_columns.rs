use conceptweave_observation::{
    ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType, ForeignKeyObservation,
    ForeignKeyReferenceBehavior,
};

#[test]
fn targeted_set_null_preserves_the_exact_local_column_subset() {
    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::SetNull,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    )
    .with_delete_target_columns(vec!["author_id".to_owned()])
    .expect("PostgreSQL ON DELETE SET NULL may target a subset of local FK columns");

    let foreign_key = ForeignKeyObservation::with_reference_behavior(
        "posts_author_fk",
        vec!["tenant_id".to_owned(), "author_id".to_owned()],
        "identity",
        "users",
        vec!["tenant_id".to_owned(), "user_id".to_owned()],
        behavior,
    )
    .expect("targeted delete columns belong to the local foreign key");

    let observed = foreign_key.reference_behavior().unwrap();
    assert_eq!(observed.delete_action(), ForeignKeyAction::SetNull);
    assert_eq!(
        observed.delete_target_columns(),
        Some(&["author_id".to_owned()][..])
    );
}

#[test]
fn targeted_set_default_rejects_unknown_or_non_targetable_columns() {
    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::SetDefault,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    )
    .with_delete_target_columns(vec!["missing_column".to_owned()])
    .expect("action-local syntax is structurally valid before FK-local validation");

    assert!(
        ForeignKeyObservation::with_reference_behavior(
            "posts_author_fk",
            vec!["tenant_id".to_owned(), "author_id".to_owned()],
            "identity",
            "users",
            vec!["tenant_id".to_owned(), "user_id".to_owned()],
            behavior,
        )
        .is_err(),
        "targeted delete columns must be a subset of the local FK columns"
    );

    assert!(
        ForeignKeyReferenceBehavior::new(
            ForeignKeyAction::NoAction,
            ForeignKeyAction::Cascade,
            ForeignKeyMatchType::Simple,
            ForeignKeyDeferrability::NotDeferrable,
        )
        .with_delete_target_columns(vec!["author_id".to_owned()])
        .is_err(),
        "PostgreSQL column lists are valid only for ON DELETE SET NULL/SET DEFAULT"
    );
}

#[test]
fn targeted_delete_columns_reject_empty_blank_and_duplicate_coordinates() {
    for target_columns in [
        Vec::new(),
        vec![" ".to_owned()],
        vec!["author_id".to_owned(), "author_id".to_owned()],
    ] {
        assert!(
            ForeignKeyReferenceBehavior::new(
                ForeignKeyAction::NoAction,
                ForeignKeyAction::SetNull,
                ForeignKeyMatchType::Simple,
                ForeignKeyDeferrability::NotDeferrable,
            )
            .with_delete_target_columns(target_columns)
            .is_err()
        );
    }

    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::SetNull,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    );
    assert_eq!(behavior.delete_target_columns(), None);
}
