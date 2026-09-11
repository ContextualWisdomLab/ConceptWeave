use conceptweave_observation::{ArrayTypeObservation, QualifiedTypeName};

fn type_name(schema: &str, name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema, name).expect("qualified type coordinate is valid")
}

#[test]
fn associated_true_array_and_element_cannot_diverge_by_schema() {
    assert!(
        ArrayTypeObservation::new(
            type_name("public", "_status"),
            type_name("archive", "status"),
        )
        .is_err(),
        "PostgreSQL keeps an associated true array in the same schema as its element type"
    );
}
