use conceptweave_observation::{ObservationError, RelationKind};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCatalogShapeObservation, IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintForeignActionCodes, IndexExclusionConstraintForeignPayloadPresence,
};

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn empty_actions() -> IndexExclusionConstraintForeignActionCodes {
    IndexExclusionConstraintForeignActionCodes::new(' ', ' ', ' ')
}

fn empty_payload() -> IndexExclusionConstraintForeignPayloadPresence {
    IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], false)
}

fn assert_shape_rejected(
    actions: IndexExclusionConstraintForeignActionCodes,
    payload: IndexExclusionConstraintForeignPayloadPresence,
) {
    let error = IndexExclusionConstraintCatalogShapeObservation::new(
        coordinate(),
        'x',
        true,
        false,
        actions,
        payload,
        false,
    )
    .expect_err("non-EXCLUDE-family residue must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_catalog_shape",
        }
    );
}

#[test]
fn each_foreign_action_code_must_be_the_non_fk_space_sentinel() {
    for actions in [
        IndexExclusionConstraintForeignActionCodes::new('a', ' ', ' '),
        IndexExclusionConstraintForeignActionCodes::new(' ', 'a', ' '),
        IndexExclusionConstraintForeignActionCodes::new(' ', ' ', 's'),
    ] {
        assert_shape_rejected(actions, empty_payload());
    }
}

#[test]
fn every_fk_only_payload_slot_fails_closed_independently() {
    let payloads = [
        IndexExclusionConstraintForeignPayloadPresence::new(true, false, [false; 3], false),
        IndexExclusionConstraintForeignPayloadPresence::new(false, true, [false; 3], false),
        IndexExclusionConstraintForeignPayloadPresence::new(false, false, [true, false, false], false),
        IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false, true, false], false),
        IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false, false, true], false),
        IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], true),
    ];

    for payload in payloads {
        assert_shape_rejected(empty_actions(), payload);
    }
}

#[test]
fn raw_foreign_payload_accessors_preserve_observed_presence() {
    let payload = IndexExclusionConstraintForeignPayloadPresence::new(
        true,
        true,
        [true, false, true],
        true,
    );
    assert!(payload.foreign_relation_present());
    assert!(payload.foreign_key_columns_present());
    assert_eq!(payload.equality_operator_vectors_present(), [true, false, true]);
    assert!(payload.delete_set_columns_present());

    let actions = IndexExclusionConstraintForeignActionCodes::new('a', 'c', 'f');
    assert_eq!(actions.update_action(), 'a');
    assert_eq!(actions.delete_action(), 'c');
    assert_eq!(actions.match_type(), 'f');
}
