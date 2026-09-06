use conceptweave_source_port::ObservationRequestBudget;

#[test]
fn schema_count_budget_cannot_be_effectively_unbounded_by_caller_choice() {
    assert!(
        ObservationRequestBudget::new(usize::MAX, 512).is_err(),
        "a caller-selected structural budget must not permit an effectively unbounded schema count before trusted policy runs"
    );
}

#[test]
fn schema_byte_budget_cannot_be_effectively_unbounded_by_caller_choice() {
    assert!(
        ObservationRequestBudget::new(8, usize::MAX).is_err(),
        "a caller-selected structural budget must not permit effectively unbounded retained schema bytes before trusted policy runs"
    );
}

#[test]
fn ordinary_bounded_structural_budget_remains_constructible() {
    let budget = ObservationRequestBudget::new(8, 512)
        .expect("ordinary provider-independent structural ceilings remain valid");

    assert_eq!(budget.max_schema_count(), 8);
    assert_eq!(budget.max_schema_bytes(), 512);
}
