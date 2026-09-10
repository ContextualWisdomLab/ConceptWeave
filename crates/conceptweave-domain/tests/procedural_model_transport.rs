//! RED/GREEN contract tests for private procedural-model JSON transport admission.

#[path = "../src/procedural_model_transport.rs"]
mod procedural_model_transport;
use procedural_model_transport::*;

#[test]
fn duplicate_decoded_members_are_rejected_before_mapping() {
    for input in [
        r#"{"model_id":"first","model_id":"second"}"#,
        r#"{"model_id":"first","\u006dodel_id":"second"}"#,
        r#"{"outer":{"task_type":"one","\u0074ask_type":"two"}}"#,
    ] {
        assert_eq!(
            admit_procedural_json_transport(input),
            Err(ProceduralTransportError::DuplicateMember)
        );
    }
}

#[test]
fn malformed_strings_surrogates_and_trailing_data_fail_closed() {
    for input in [
        r#"{"x":"unterminated}"#,
        r#"{"x":"\uD800"}"#,
        r#"{"x":"\uDC00"}"#,
        r#"{"x":1} trailing"#,
        "{\"x\":\"line\nfeed\"}",
    ] {
        assert_eq!(
            admit_procedural_json_transport(input),
            Err(ProceduralTransportError::InvalidJson)
        );
    }
}

#[test]
fn depth_and_transport_size_are_bounded_without_changing_valid_boundary() {
    let depth_128 = format!("{}0{}", "[".repeat(128), "]".repeat(128));
    assert_eq!(admit_procedural_json_transport(&depth_128), Ok(()));

    let depth_129 = format!("{}0{}", "[".repeat(129), "]".repeat(129));
    assert_eq!(
        admit_procedural_json_transport(&depth_129),
        Err(ProceduralTransportError::DepthLimit)
    );

    let exact = format!("\"{}\"", "x".repeat(MAX_PROCEDURAL_TRANSPORT_BYTES - 2));
    assert_eq!(exact.len(), MAX_PROCEDURAL_TRANSPORT_BYTES);
    assert_eq!(admit_procedural_json_transport(&exact), Ok(()));

    let oversized = format!("\"{}\"", "x".repeat(MAX_PROCEDURAL_TRANSPORT_BYTES - 1));
    assert_eq!(oversized.len(), MAX_PROCEDURAL_TRANSPORT_BYTES + 1);
    assert_eq!(
        admit_procedural_json_transport(&oversized),
        Err(ProceduralTransportError::InputTooLarge)
    );
}

#[test]
fn valid_json_forms_and_unicode_escape_pairs_are_admitted() {
    for input in [
        r#"null"#,
        r#"true"#,
        r#"-12.5e+2"#,
        r#"[null,false,0,1.25,{"ko":"검증","emoji":"\uD83D\uDE80"}]"#,
        r#"{"schema_version":"0.1.0-draft.1","procedure_nodes":[],"procedure_edges":[]}"#,
    ] {
        assert_eq!(admit_procedural_json_transport(input), Ok(()));
    }
}

#[test]
fn duplicate_diagnostic_is_fixed_and_never_echoes_member_text() {
    let key = "attacker_controlled_".repeat(256);
    let input = format!("{{\"{key}\":1,\"{key}\":2}}");
    let error = admit_procedural_json_transport(&input).unwrap_err();
    assert_eq!(error, ProceduralTransportError::DuplicateMember);
    assert_eq!(error.to_string(), "duplicate_member");
    assert!(!error.to_string().contains(&key));
}
