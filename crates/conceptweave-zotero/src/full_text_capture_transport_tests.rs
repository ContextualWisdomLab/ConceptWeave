mod tests {
    use super::super::*;
    use crate::classify_snapshot;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread::{self, JoinHandle};

    fn report_fixture() -> ClassificationReport {
        let items = serde_json::from_value(serde_json::json!([
            {"key":"ABCD2345","version":2,"data":{"itemType":"book","title":"Synthetic ontology paper"}},
            {"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}}
        ]))
        .unwrap();
        let mut report =
            classify_snapshot("10.0.1".into(), Some("fixture-server".into()), 2, items);
        report.api_version = Some(3);
        report.schema_version = Some(44);
        report
    }

    fn wire_response(status: u16, version: Option<&str>, body: &[u8]) -> Vec<u8> {
        let version = version
            .map(|value| format!("Last-Modified-Version: {value}\r\n"))
            .unwrap_or_default();
        let mut response = format!(
            "HTTP/1.1 {status} Synthetic\r\nZotero-Server-ID: fixture-server\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 44\r\nX-Zotero-Version: 10.0.1\r\n{version}Content-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .into_bytes();
        response.extend_from_slice(body);
        response
    }

    fn serve_responses(responses: Vec<Vec<u8>>) -> (String, JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_root = format!("http://{}", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            responses
                .into_iter()
                .map(|response| {
                    let (mut stream, _) = listener.accept().unwrap();
                    let mut request = Vec::new();
                    while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                        let mut buffer = [0; 4096];
                        let length = stream.read(&mut buffer).unwrap();
                        assert_ne!(length, 0);
                        request.extend_from_slice(&buffer[..length]);
                    }
                    // Invalid headers or bounded bodies can make the client stop early.
                    let _ = stream.write_all(&response);
                    String::from_utf8(request).unwrap()
                })
                .collect()
        });
        (api_root, server)
    }

    fn fetch_wire(response: Vec<u8>, limit: u64) -> Result<CapturedResponse, FullTextError> {
        let (api_root, server) = serve_responses(vec![response]);
        let result = fetch_response(
            &local_agent(),
            &report_fixture(),
            &api_root,
            "items/BCDE3456/fulltext",
            limit,
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("GET /api/users/0/items/BCDE3456/fulltext HTTP/1.1\r\n"));
        assert!(requests[0].contains("zotero-api-version: 3\r\n"));
        assert!(requests[0].contains("zotero-server-id: fixture-server\r\n"));
        assert!(!requests[0].contains("zotero-api-key:"));
        result
    }

    #[test]
    fn response_keeps_exact_status_version_and_text() {
        let body = "synthetic text 한글";
        let response = fetch_wire(wire_response(200, Some("0"), body.as_bytes()), 128).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.version, Some(0));
        assert_eq!(response.body, body);
        for status in [404, 401, 403, 429, 500, 503] {
            let response =
                fetch_wire(wire_response(status, None, b"synthetic failure"), 128).unwrap();
            assert_eq!(response.status, status);
            assert_eq!(response.version, None);
            assert_eq!(response.body, "synthetic failure");
        }
    }

    #[test]
    fn every_response_requires_matching_identity_and_contract_headers() {
        let valid = String::from_utf8(wire_response(200, Some("2"), b"[]")).unwrap();
        for (header, value) in [
            ("Zotero-Server-ID", "fixture-server"),
            ("Zotero-API-Version", "3"),
            ("Zotero-Schema-Version", "44"),
            ("X-Zotero-Version", "10.0.1"),
        ] {
            let original = format!("{header}: {value}\r\n");
            for replacement in [String::new(), format!("{header}: changed\r\n")] {
                let response = valid.replace(&original, &replacement).into_bytes();
                assert_eq!(
                    fetch_wire(response, 128).err(),
                    Some(INVALID_EVIDENCE),
                    "{header}"
                );
            }
            let mut response = valid
                .replace(&original, &format!("{header}: invalid-byte\r\n"))
                .into_bytes();
            let position = response
                .windows(12)
                .position(|part| part == b"invalid-byte")
                .unwrap();
            response[position] = 0xff;
            assert!(fetch_wire(response, 128).is_err(), "{header}");
        }
    }

    #[test]
    fn malformed_present_versions_are_not_treated_as_missing() {
        for version in ["", "invalid", "-1", "18446744073709551616"] {
            assert_eq!(
                fetch_wire(wire_response(200, Some(version), b"[]"), 128).err(),
                Some(INVALID_EVIDENCE)
            );
        }
        let mut response = wire_response(200, Some("invalid-byte"), b"[]");
        let position = response
            .windows(12)
            .position(|part| part == b"invalid-byte")
            .unwrap();
        response[position] = 0xff;
        assert!(fetch_wire(response, 128).is_err());
    }

    #[test]
    fn body_limit_and_strict_utf8_are_enforced_on_wire_bytes() {
        assert_eq!(
            fetch_wire(wire_response(200, Some("2"), b"[]"), 2)
                .unwrap()
                .body,
            "[]"
        );
        assert_eq!(
            fetch_wire(wire_response(200, Some("2"), b"[]"), 1).err(),
            Some(INVALID_EVIDENCE)
        );
        assert_eq!(
            fetch_wire(wire_response(200, Some("2"), &[0xff]), 128).err(),
            Some(INVALID_EVIDENCE)
        );
        let response = String::from_utf8(wire_response(200, Some("2"), b"[]")).unwrap();
        let truncated = response.replace("Content-Length: 2", "Content-Length: 3");
        assert_eq!(
            fetch_wire(truncated.into_bytes(), 128).err(),
            Some(INVALID_EVIDENCE)
        );
        let chunked = response
            .replace("Content-Length: 2", "Transfer-Encoding: chunked")
            .replace("\r\n\r\n[]", "\r\n\r\n2\r\n[]\r\n0\r\n\r\n");
        assert_eq!(
            fetch_wire(chunked.into_bytes(), 1).err(),
            Some(INVALID_EVIDENCE)
        );
    }

    #[test]
    fn capture_rejects_non_success_status_without_following_redirects() {
        let target = TcpListener::bind("127.0.0.1:0").unwrap();
        target.set_nonblocking(true).unwrap();
        for status in [301, 302, 303, 307, 308, 401, 403, 404, 429, 500, 503] {
            let response = String::from_utf8(wire_response(status, Some("2"), b"[]"))
                .unwrap()
                .replace(
                    "Content-Length:",
                    &format!(
                        "Location: http://{}/redirect-target\r\nContent-Length:",
                        target.local_addr().unwrap()
                    ),
                );
            let (api_root, server) = serve_responses(vec![response.into_bytes()]);
            assert!(read_full_text_from_api(&report_fixture(), &api_root).is_err());
            assert_eq!(server.join().unwrap().len(), 1);
            assert_eq!(
                target.accept().err().unwrap().kind(),
                std::io::ErrorKind::WouldBlock
            );
        }
    }

    #[test]
    fn connection_failure_is_secret_free_and_missing_identity_prevents_network_access() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let api_root = format!("http://{}", listener.local_addr().unwrap());
        let mut report = report_fixture();
        report.server_id = None;
        assert_eq!(
            fetch_response(&local_agent(), &report, &api_root, "items?limit=1", 128).err(),
            Some(INVALID_EVIDENCE)
        );
        assert_eq!(
            listener.accept().err().unwrap().kind(),
            std::io::ErrorKind::WouldBlock
        );
        // The public wrapper rejects before its fixed production URL can be called.
        assert!(read_local_full_text(&report).is_err());
        drop(listener);
        let error = fetch_response(
            &local_agent(),
            &report_fixture(),
            &api_root,
            "items/BCDE3456/fulltext",
            128,
        )
        .err()
        .unwrap();
        assert_eq!(error.to_string(), "full-text local request failed");
        assert!(!format!("{error:?}").contains("BCDE3456"));
    }

    #[test]
    fn authorization_response_at_its_byte_limit_is_not_rejected() {
        let mut body = br#"{"key":"0123456789abcdef0123456789abcdef","remember":true}"#.to_vec();
        body.resize(crate::MAX_AUTH_RESPONSE_BYTES as usize, b' ');
        let (api_root, server) = serve_responses(vec![wire_response(200, None, &body)]);
        let result = crate::Zotero10LocalAuthorization::request_with_base(
            "Synthetic exact limit test",
            "fixture-server",
            api_root,
        );
        server.join().unwrap();
        assert!(
            result.is_ok(),
            "exact-limit authorization JSON must be accepted"
        );
    }

    #[test]
    fn snapshot_response_at_its_byte_limit_is_not_rejected() {
        const CHILD_CASE: &str = "CONCEPTWEAVE_EXACT_SNAPSHOT_LIMIT_CASE";
        if std::env::var_os(CHILD_CASE).is_none() {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "full_text_capture::transport_tests::tests::snapshot_response_at_its_byte_limit_is_not_rejected",
                ])
                .env_clear()
                .env(CHILD_CASE, "synthetic")
                .status()
                .unwrap();
            assert!(
                status.success(),
                "isolated snapshot byte-limit regression failed"
            );
            return;
        }
        let mut body = b"[]".to_vec();
        body.resize(MAX_PAGE_BYTES as usize, b' ');
        let mut response = wire_response(200, Some("2"), &body);
        let header_end = response
            .windows(4)
            .position(|part| part == b"\r\n\r\n")
            .unwrap();
        response.splice(
            header_end..header_end,
            b"\r\nTotal-Results: 0".iter().copied(),
        );
        let (api_root, server) = serve_responses(vec![response]);
        *crate::TEST_LOCAL_API.lock().unwrap() = Some(format!("{api_root}/api/users/0/items"));
        let result = crate::read_local_snapshot();
        *crate::TEST_LOCAL_API.lock().unwrap() = None;
        server.join().unwrap();
        assert!(result.is_ok(), "exact-limit snapshot JSON must be accepted");
    }

    #[test]
    fn synthetic_http_sweep_uses_the_same_agent_and_capture_path_as_the_public_wrapper() {
        let manifest = br#"{"BCDE3456":7}"#;
        let metadata = br#"{"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}}"#;
        let content = br#"{"content":"synthetic full text","providerExtra":{"retained":true}}"#;
        let (api_root, server) = serve_responses(vec![
            wire_response(200, Some("2"), b"[]"),
            wire_response(200, None, manifest),
            wire_response(200, Some("1"), metadata),
            wire_response(200, Some("7"), content),
            wire_response(200, None, manifest),
            wire_response(200, Some("2"), b"[]"),
        ]);
        let report = report_fixture();
        let capture = read_full_text_from_api(&report, &api_root).unwrap();
        verify_full_text_capture(&capture, &report).unwrap();
        assert_eq!(capture.capture_evidence.records.len(), 1);
        assert_eq!(
            capture.capture_evidence.records[0]
                .content_response
                .body
                .as_bytes(),
            content
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 6);
        assert!(requests[0].starts_with("GET /api/users/0/items?limit=1 "));
        assert!(requests[1].starts_with("GET /api/users/0/fulltext?since=0 "));
        assert!(requests[2].starts_with("GET /api/users/0/items/BCDE3456 "));
        assert!(requests[3].starts_with("GET /api/users/0/items/BCDE3456/fulltext "));
        assert_eq!(requests[0], requests[5]);
        assert_eq!(requests[1], requests[4]);
    }
}
