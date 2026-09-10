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
        r#"{"🚀":1,"\uD83D\uDE80":2}"#,
    ] {
        assert_eq!(
            admit_procedural_json_transport(input),
            Err(ProceduralTransportError::DuplicateMember)
        );
    }
}

#[test]
fn byte_transport_rejects_malformed_utf8_without_lossy_conversion() {
    let malformed = [b'{', b'"', b'x', b'"', b':', b'"', 0xff, b'"', b'}'];
    assert_eq!(
        admit_procedural_json_transport_bytes(&malformed),
        Err(ProceduralTransportError::InvalidJson)
    );
}

#[test]
fn byte_transport_applies_wire_size_limit_before_utf8_validation() {
    let payload = vec![b' '; MAX_PROCEDURAL_TRANSPORT_BYTES - 2];
    let mut exact_json = Vec::with_capacity(MAX_PROCEDURAL_TRANSPORT_BYTES);
    exact_json.extend_from_slice(b"\"");
    exact_json.extend_from_slice(&payload);
    exact_json.extend_from_slice(b"\"");
    assert_eq!(exact_json.len(), MAX_PROCEDURAL_TRANSPORT_BYTES);
    assert_eq!(admit_procedural_json_transport_bytes(&exact_json), Ok(()));

    let oversized_invalid = vec![0xff; MAX_PROCEDURAL_TRANSPORT_BYTES + 1];
    assert_eq!(
        admit_procedural_json_transport_bytes(&oversized_invalid),
        Err(ProceduralTransportError::InputTooLarge)
    );
}

#[test]
fn malformed_strings_surrogates_and_trailing_data_fail_closed() {
    for input in [
        r#"{"x":"unterminated}"#,
        r#"{"x":"\uD800"}"#,
        r#"{"x":"\uDC00"}"#,
        r#"{"x":"\uD800\u0041"}"#,
        r#"{"x":"\u12xz"}"#,
        r#"{"x":"\q"}"#,
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
fn malformed_container_and_number_grammar_fails_closed() {
    for input in [
        "",
        " ",
        "{",
        "[",
        r#"{"x" 1}"#,
        r#"{"x":1,}"#,
        r#"[1,]"#,
        r#"{"x":01}"#,
        r#"{"x":-}"#,
        r#"{"x":1.}"#,
        r#"{"x":1e}"#,
        r#"{"x":1e+}"#,
        "truth",
    ] {
        assert_eq!(
            admit_procedural_json_transport(input),
            Err(ProceduralTransportError::InvalidJson)
        );
    }
}

#[test]
fn depth_and_transport_size_are_bounded_without_changing_valid_boundary() {
    let at_limit = format!(
        "{}0{}",
        "[".repeat(MAX_PROCEDURAL_TRANSPORT_DEPTH),
        "]".repeat(MAX_PROCEDURAL_TRANSPORT_DEPTH)
    );
    assert_eq!(admit_procedural_json_transport(&at_limit), Ok(()));

    let beyond_limit = format!(
        "{}0{}",
        "[".repeat(MAX_PROCEDURAL_TRANSPORT_DEPTH + 1),
        "]".repeat(MAX_PROCEDURAL_TRANSPORT_DEPTH + 1)
    );
    assert_eq!(
        admit_procedural_json_transport(&beyond_limit),
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
fn valid_json_forms_unicode_and_all_string_escapes_are_admitted() {
    for input in [
        r#"null"#,
        r#"true"#,
        r#"false"#,
        r#"0"#,
        r#"-12.5e+2"#,
        r#"{}"#,
        r#"[]"#,
        " \n\t{\"escaped\":\"\\\"\\\\\\/\\b\\f\\n\\r\\t\\u0041\",\"ko\":\"검증\",\"emoji\":\"\\uD83D\\uDE80\"}\r ",
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
