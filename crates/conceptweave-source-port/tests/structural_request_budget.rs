use conceptweave_source_port::{
    ObservationRequestBudget, ObservationRequestBudgetError, MAX_STRUCTURAL_SCHEMA_BYTES,
    MAX_STRUCTURAL_SCHEMA_COUNT,
};

#[test]
fn schema_count_budget_cannot_exceed_canonical_structural_cap() {
    assert_eq!(
        ObservationRequestBudget::new(MAX_STRUCTURAL_SCHEMA_COUNT + 1, 512),
        Err(ObservationRequestBudgetError::SchemaCountLimitTooLarge {
            maximum: MAX_STRUCTURAL_SCHEMA_COUNT,
        })
    );
}

#[test]
fn schema_byte_budget_cannot_exceed_canonical_structural_cap() {
    assert_eq!(
        ObservationRequestBudget::new(8, MAX_STRUCTURAL_SCHEMA_BYTES + 1),
        Err(ObservationRequestBudgetError::SchemaByteLimitTooLarge {
            maximum: MAX_STRUCTURAL_SCHEMA_BYTES,
        })
    );
}

#[test]
fn canonical_structural_caps_remain_constructible() {
    let budget = ObservationRequestBudget::new(
        MAX_STRUCTURAL_SCHEMA_COUNT,
        MAX_STRUCTURAL_SCHEMA_BYTES,
    )
    .expect("canonical provider-independent structural ceilings remain valid");

    assert_eq!(budget.max_schema_count(), MAX_STRUCTURAL_SCHEMA_COUNT);
    assert_eq!(budget.max_schema_bytes(), MAX_STRUCTURAL_SCHEMA_BYTES);
}

#[test]
fn ordinary_bounded_structural_budget_remains_constructible() {
    let budget = ObservationRequestBudget::new(8, 512)
        .expect("ordinary provider-independent structural ceilings remain valid");

    assert_eq!(budget.max_schema_count(), 8);
    assert_eq!(budget.max_schema_bytes(), 512);
}
