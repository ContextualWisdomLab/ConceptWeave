use super::*;

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
    if output_was_created {
        fs::remove_file(&output).unwrap();
    }

    let error = result.expect_err("oversized metadata output must fail closed");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(error.to_string(), "metadata output exceeds the artifact size limit");
    assert!(!output_was_created);
}
