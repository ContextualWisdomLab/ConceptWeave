use conceptweave_alignment::{AlignmentDecision, AlignmentDisposition, AlignmentError};

#[test]
fn decision_text_rejects_blank_and_nul_without_normalizing_valid_descriptions() {
    for invalid in ["", " \n\t", "bad\0text"] {
        for (position, field) in ["candidate_id", "semantic_id", "name", "rationale"]
            .into_iter()
            .enumerate()
        {
            let mut values = ["candidate", "semantic", "Name", "Rationale"];
            values[position] = invalid;
            assert_eq!(
                AlignmentDecision::map(values[0], values[1], values[2], values[3]),
                Err(AlignmentError::InvalidText(field))
            );
        }
        assert_eq!(
            AlignmentDecision::exclude(invalid, "Rationale"),
            Err(AlignmentError::InvalidText("candidate_id"))
        );
        assert_eq!(
            AlignmentDecision::exclude("candidate", invalid),
            Err(AlignmentError::InvalidText("rationale"))
        );
    }
    let mapped = AlignmentDecision::map("candidate", "semantic", " 이름 ", " 이유\n").unwrap();
    assert_eq!(mapped.candidate_id(), "candidate");
    assert_eq!(
        mapped.disposition(),
        &AlignmentDisposition::Map {
            semantic_id: "semantic".to_owned(),
            name: " 이름 ".to_owned(),
            rationale: " 이유\n".to_owned(),
        }
    );
    let excluded = AlignmentDecision::exclude("candidate", " 제외 이유\n").unwrap();
    assert_eq!(
        excluded.disposition(),
        &AlignmentDisposition::Exclude {
            rationale: " 제외 이유\n".to_owned(),
        }
    );
}
