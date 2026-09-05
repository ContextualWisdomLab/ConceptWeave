use super::*;

#[test]
fn private_json_reader_does_not_echo_unknown_keys_or_invalid_values() {
    let unknown_field = br#"{"synthetic-private-field-sentinel":true}"#;
    let invalid_disposition = br#""synthetic-private-enum-sentinel""#;
    let unknown_error = read_bounded_json::<FullTextReviewWorksheet>(
        &mut unknown_field.as_slice(),
        unknown_field.len() as u64,
    )
    .err()
    .unwrap();
    let value_error = read_bounded_json::<conceptweave_zotero::Disposition>(
        &mut invalid_disposition.as_slice(),
        invalid_disposition.len() as u64,
    )
    .unwrap_err();
    for error in [unknown_error, value_error] {
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "review input is invalid");
        assert_eq!(
            label_input("worksheet", error).to_string(),
            "worksheet: review input is invalid"
        );
    }
}

#[test]
fn capture_reader_accepts_exact_budget_without_truncating_trailing_bytes() {
    let bytes = b"{\"synthetic\":true}  ";
    let length = bytes.len() as u64;
    let parsed: serde_json::Value =
        read_bounded_capture(&mut bytes.as_slice(), length, length).unwrap();
    assert_eq!(parsed, serde_json::json!({"synthetic":true}));
    let parsed: serde_json::Value =
        read_bounded_capture(&mut bytes.as_slice(), length, length + 1).unwrap();
    assert_eq!(parsed, serde_json::json!({"synthetic":true}));
}

#[test]
fn capture_reader_rejects_advertised_oversize_growth_and_shrink() {
    let bytes = b"{}   ";
    for (advertised, limit, expected) in [
        (6, 5, "full-text capture input exceeds the file size limit"),
        (
            4,
            4,
            "full-text capture input grew beyond the file size limit",
        ),
        (4, 5, "full-text capture input changed size while reading"),
        (6, 6, "full-text capture input changed size while reading"),
    ] {
        let error =
            read_bounded_capture::<serde_json::Value>(&mut bytes.as_slice(), advertised, limit)
                .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), expected);
    }
    let mut stream = io::Cursor::new(b"{}                ");
    assert!(read_bounded_capture::<serde_json::Value>(&mut stream, 4, 4).is_err());
    assert_eq!(stream.position(), 5);
}

#[test]
fn capture_reader_sanitizes_json_and_io_failures() {
    for mut bytes in [
        b"{\"private-sentinel\":".as_slice(),
        b"{} private-sentinel",
        b"\xff",
        b"",
    ] {
        let length = bytes.len() as u64;
        let error = read_bounded_capture::<serde_json::Value>(&mut bytes, length, 64).unwrap_err();
        assert_eq!(error.to_string(), "full-text capture input is invalid");
    }
    struct FailedReader;
    impl Read for FailedReader {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("private-sentinel"))
        }
    }
    let error = read_bounded_capture::<serde_json::Value>(&mut FailedReader, 0, 64).unwrap_err();
    assert_eq!(error.to_string(), "full-text capture input is invalid");
}

#[test]
fn private_capture_reader_reuses_regular_owner_only_file_boundary() {
    let file_path = tests::unique_temp_path("streaming-capture-reader");
    let file = create_report_file(&file_path).unwrap();
    drop(file);
    let error = read_private_capture(file_path.to_str().unwrap())
        .err()
        .unwrap();
    assert_eq!(error.to_string(), "full-text capture input is invalid");
    fs::remove_file(file_path).unwrap();
}

#[test]
fn metadata_writer_rejects_oversized_output_before_creating_file() {
    let output = tests::unique_temp_path("oversized-private-output");
    let _ = fs::remove_file(&output);
    let content = vec![b'x'; MAX_ARTIFACT_BYTES as usize + 1];

    let result = write_private_output(&output, &content);
    let output_was_created = output.exists();
    let _ = fs::remove_file(&output);

    let error = result.expect_err("oversized metadata output must fail closed");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "metadata output exceeds the artifact size limit"
    );
    assert!(!output_was_created);
}

#[test]
fn metadata_writer_accepts_exact_limit_and_capture_writer_keeps_its_separate_budget() {
    let output = tests::unique_temp_path("exact-limit-private-output");
    let mut content = vec![b'x'; MAX_ARTIFACT_BYTES as usize];
    write_private_output(&output, &content).unwrap();
    assert_eq!(fs::metadata(&output).unwrap().len(), MAX_ARTIFACT_BYTES);
    fs::remove_file(&output).unwrap();
    content.push(b'x');
    write_private_output_with(&output, &content, &mut write_all_and_flush).unwrap();
    assert_eq!(fs::metadata(&output).unwrap().len(), MAX_ARTIFACT_BYTES + 1);
    fs::remove_file(output).unwrap();
}

#[test]
fn completed_view_reader_preserves_duplicate_keys_byte_for_byte() {
    let input = tests::unique_temp_path("exact-completed-view");
    let bytes = br#"{"decision":false,"decision":true}"#;
    let mut file = create_report_file(&input).unwrap();
    file.write_all(bytes).unwrap();
    drop(file);
    let (restored, _) = read_private_bytes(input.to_str().unwrap()).unwrap();
    assert_eq!(restored, bytes);
    fs::remove_file(input).unwrap();
}
