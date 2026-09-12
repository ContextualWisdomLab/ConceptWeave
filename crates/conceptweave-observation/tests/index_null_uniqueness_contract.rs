use conceptweave_observation::{
    IndexAttributeKind, IndexAttributeObservation, IndexObservation, ObservationError,
};

fn key_attribute() -> IndexAttributeObservation {
    IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
        .expect("key attribute fixture is valid")
}

#[test]
fn nulls_not_distinct_true_requires_a_unique_index() {
    let error = IndexObservation::new(
        "document_id_ix",
        false,
        Some(true),
        vec![key_attribute()],
        Vec::new(),
    )
    .expect_err("a non-unique PostgreSQL index cannot claim NULLS NOT DISTINCT semantics");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "nulls_not_distinct"
        }
    );
}

#[test]
fn observed_false_remains_admissible_for_a_non_unique_index() {
    let index = IndexObservation::new(
        "document_id_ix",
        false,
        Some(false),
        vec![key_attribute()],
        Vec::new(),
    )
    .expect("false is the directly observable catalog value when uniqueness is absent");

    assert_eq!(index.nulls_not_distinct(), Some(false));
}
