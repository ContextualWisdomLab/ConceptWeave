use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, QualifiedOperatorClassName, QualifiedTypeName,
    RelationKind, RelationObservation,
};

fn relation() -> RelationObservation {
    let column = ColumnObservationV3::new(
        "event_key",
        1,
        "uuid",
        QualifiedTypeName::new("pg_catalog", "uuid").expect("qualified type fixture is valid"),
        false,
        None,
    )
    .expect("column fixture is valid");

    RelationObservation::new(
        "public",
        "event_record",
        RelationKind::Table,
        vec![column],
    )
    .expect("relation fixture is valid")
}

fn complete_index() -> IndexObservation {
    let attribute = IndexAttributeObservation::new(1, IndexAttributeKind::Key, "event_key")
        .expect("index attribute fixture is valid");
    let semantics = IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "uuid_ops")
            .expect("operator class fixture is valid"),
        0,
    )
    .expect("key semantics fixture is valid");

    IndexObservation::new("event_key_ix", false, None, vec![attribute], Vec::new())
        .expect("index fixture is valid")
        .with_access_method("btree")
        .with_key_semantics(vec![semantics])
        .expect("complete key semantics are valid")
}

#[test]
fn relation_rejects_whitespace_only_rendered_index_predicate() {
    let result = relation().with_indexes(vec![complete_index().with_predicate("   ")]);

    assert!(matches!(
        result,
        Err(ObservationError::InvalidObservationField {
            field: "index_predicate"
        })
    ));
}

#[test]
fn relation_rejects_whitespace_only_reconstructed_index_definition() {
    let result = relation().with_indexes(vec![complete_index().with_index_definition("\t ")]);

    assert!(matches!(
        result,
        Err(ObservationError::InvalidObservationField {
            field: "index_definition"
        })
    ));
}

#[test]
fn relation_preserves_exact_nonblank_rendered_index_text() {
    let predicate = " (event_key IS NOT NULL) ";
    let definition =
        "\tCREATE INDEX event_key_ix ON public.event_record USING btree (event_key) WHERE (event_key IS NOT NULL)\n";
    let observed = relation()
        .with_indexes(vec![
            complete_index()
                .with_predicate(predicate)
                .with_index_definition(definition),
        ])
        .expect("nonblank server-rendered index evidence is valid");
    let index = &observed.indexes()[0];

    assert_eq!(index.predicate(), Some(predicate));
    assert_eq!(index.index_definition(), Some(definition));
}
