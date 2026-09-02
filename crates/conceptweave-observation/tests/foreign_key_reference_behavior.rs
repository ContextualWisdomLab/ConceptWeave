use conceptweave_observation::{
    ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType, ForeignKeyObservation,
    ForeignKeyReferenceBehavior,
};

fn local_columns() -> Vec<String> {
    vec!["tenant_key".to_owned(), "account_key".to_owned()]
}

fn referenced_columns() -> Vec<String> {
    vec!["tenant_key".to_owned(), "account_key".to_owned()]
}

#[test]
fn foreign_key_preserves_exact_reference_actions_match_and_deferrability() {
    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::Cascade,
        ForeignKeyAction::SetNull,
        ForeignKeyMatchType::Full,
        ForeignKeyDeferrability::InitiallyDeferred,
    );
    let foreign_key = ForeignKeyObservation::with_reference_behavior(
        "event_account_fk",
        local_columns(),
        "identity",
        "account_record",
        referenced_columns(),
        behavior,
    )
    .expect("foreign-key metadata is valid");

    let observed = foreign_key
        .reference_behavior()
        .expect("explicitly observed reference behavior must be retained");
    assert_eq!(observed.update_action(), ForeignKeyAction::Cascade);
    assert_eq!(observed.delete_action(), ForeignKeyAction::SetNull);
    assert_eq!(observed.match_type(), ForeignKeyMatchType::Full);
    assert_eq!(
        observed.deferrability(),
        ForeignKeyDeferrability::InitiallyDeferred
    );
}

#[test]
fn foreign_key_without_observed_reference_behavior_remains_explicitly_unknown() {
    let foreign_key = ForeignKeyObservation::new(
        "event_account_fk",
        local_columns(),
        "identity",
        "account_record",
        referenced_columns(),
    )
    .expect("legacy source metadata remains structurally valid");

    assert_eq!(foreign_key.reference_behavior(), None);
}

#[test]
fn reference_behavior_represents_all_postgresql_action_and_timing_states_without_strings() {
    let actions = [
        ForeignKeyAction::NoAction,
        ForeignKeyAction::Restrict,
        ForeignKeyAction::Cascade,
        ForeignKeyAction::SetNull,
        ForeignKeyAction::SetDefault,
    ];
    let match_types = [
        ForeignKeyMatchType::Simple,
        ForeignKeyMatchType::Full,
        ForeignKeyMatchType::Partial,
    ];
    let timings = [
        ForeignKeyDeferrability::NotDeferrable,
        ForeignKeyDeferrability::InitiallyImmediate,
        ForeignKeyDeferrability::InitiallyDeferred,
    ];

    assert_eq!(actions.len(), 5);
    assert_eq!(match_types.len(), 3);
    assert_eq!(timings.len(), 3);
}
