use conceptweave_observation::{
    IndexAttributeKind, IndexAttributeObservation, IndexObservation, ObservationError,
};

fn column_attribute(
    position: u32,
    kind: IndexAttributeKind,
    attribute_name: &str,
) -> IndexAttributeObservation {
    IndexAttributeObservation::new(position, kind, attribute_name)
        .expect("index attribute fixture is valid")
}

#[test]
fn index_layout_rejects_role_collection_disagreement() {
    let error = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![column_attribute(
            1,
            IndexAttributeKind::Include,
            "parent_key",
        )],
        vec![column_attribute(2, IndexAttributeKind::Key, "event_key")],
    )
    .expect_err("key and INCLUDE collection roles must not contradict attribute roles");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_attribute_layout",
        }
    );
}

#[test]
fn index_layout_rejects_non_contiguous_key_include_boundary() {
    let error = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![column_attribute(2, IndexAttributeKind::Key, "parent_key")],
        vec![column_attribute(
            4,
            IndexAttributeKind::Include,
            "event_key",
        )],
    )
    .expect_err("pg_index ordinal evidence must be contiguous across the key/INCLUDE boundary");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_attribute_layout",
        }
    );
}

#[test]
fn index_layout_rejects_expression_include_attributes() {
    let expression =
        IndexAttributeObservation::expression(2, IndexAttributeKind::Include, "lower(event_key)")
            .expect("expression fixture is structurally valid before index-layout admission");

    let error = IndexObservation::new(
        "event_parent_ix",
        false,
        None,
        vec![column_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![expression],
    )
    .expect_err("PostgreSQL INCLUDE accepts columns, not expressions");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_attribute_layout",
        }
    );
}

#[test]
fn index_layout_accepts_contiguous_key_then_include_ordinals() {
    let index = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![column_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![column_attribute(
            2,
            IndexAttributeKind::Include,
            "event_key",
        )],
    )
    .expect("one key followed by one INCLUDE attribute is a valid pg_index layout");

    assert_eq!(index.key_attributes()[0].position(), 1);
    assert_eq!(index.include_attributes()[0].position(), 2);
}
