use super::*;
use std::process::{Command, Stdio};
use std::time::Instant;

const PROXY_CHILD_CASE: &str = "CONCEPTWEAVE_AUTHENTICATED_PROXY_CASE";
const SYNTHETIC_API_KEY: &str = "0123456789abcdef0123456789abcdef";

#[test]
fn failed_http_write_with_matching_observation_remains_indeterminate() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("server-10".into()),
        42,
        vec![item("ABCD2345", "book", "ontology learning", "", "")],
    );
    let expected_request = write_request();
    let review = ReviewedClassificationWriteSet {
        review_id: "synthetic-review".into(),
        authority_receipt: "synthetic-authority".into(),
        server_id: report.server_id.clone(),
        zotero_version: report.zotero_version.clone(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(&report),
        snapshot_items: report.snapshot_items.clone(),
        changes: vec![ReviewedClassificationChange {
            item_key: expected_request.item_key.clone(),
            item_version: expected_request.item_version,
            reviewed_disposition: Disposition::Generation,
            before_collection_keys: vec![],
            before_tags: vec![],
            after_collection_keys: expected_request.collection_keys.clone(),
            after_tags: expected_request.tags.clone(),
        }],
    };
    let plan =
        build_classification_write_plan(&report, &review, WriteMode::Execute, |set| set == &review)
            .unwrap();
    let responses = vec![
        library_response("server-10", 42),
        raw_response(
            Some("server-10"),
            Some(7),
            r#"{"key":"ABCD2345","version":7,"data":{"itemType":"book"}}"#,
        ),
        library_response("server-10", 42),
        "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            .into(),
        library_response("server-10", 43),
        item_response("server-10", 43),
        library_response("server-10", 43),
    ];
    let (base, server) = serve(
        responses
            .into_iter()
            .map(|response| &*Box::leak(response.into_boxed_str()))
            .collect(),
    );
    let adapter =
        Zotero10LocalAdapter::new_with_base(SYNTHETIC_API_KEY, "server-10", base).unwrap();
    let receipt = execute_classification_write_plan(
        &plan,
        |key| adapter.get_item(key),
        |request| adapter.write_item(request),
    );
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 7);
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.starts_with("POST "))
            .count(),
        1
    );
    let body: serde_json::Value =
        serde_json::from_str(requests[3].split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(
        body[0]["collections"],
        serde_json::json!(expected_request.collection_keys)
    );
    assert_eq!(
        body[0]["tags"],
        serde_json::to_value(&expected_request.tags).unwrap()
    );
    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("ABCD2345"));
    assert_eq!(
        receipt.indeterminate_request,
        Some(expected_request.clone())
    );
    assert_eq!(
        receipt.reconciliation_observation,
        Some(ClassificationItemState {
            server_id: expected_request.server_id,
            library_version: 43,
            item_key: expected_request.item_key,
            item_version: 43,
            collection_keys: expected_request.collection_keys,
            tags: expected_request.tags,
        })
    );
    assert_eq!(receipt.proposal_digest, review.proposal_digest);
    assert!(receipt.applied_item_keys.is_empty());
    assert!(receipt.rollback_operations.is_empty());
    assert!(
        !serde_json::to_string(&plan)
            .unwrap()
            .contains(SYNTHETIC_API_KEY)
    );
}

#[test]
fn synthetic_server_retains_headers_and_body_larger_than_its_read_buffer() {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    let (base, server) = serve(vec![response]);
    let address = base
        .strip_prefix("http://")
        .unwrap()
        .split_once('/')
        .unwrap()
        .0;
    let mut connection = std::net::TcpStream::connect(address).unwrap();
    // Both sections exceed the 4 KiB read buffer regardless of TCP packet timing.
    let request = format!(
        "POST /api/users/0/items HTTP/1.1\r\nHost: localhost\r\nX-Synthetic-Padding: {}\r\nContent-Length: 8192\r\n\r\n{}",
        "h".repeat(8192),
        "b".repeat(8192)
    );
    connection.write_all(request.as_bytes()).unwrap();
    let mut received_response = String::new();
    connection.read_to_string(&mut received_response).unwrap();
    assert_eq!(received_response, response);
    assert_eq!(server.join().unwrap(), [request]);
}

#[test]
fn authenticated_calls_never_use_environment_proxies() {
    let mut failures = Vec::new();
    for request_kind in ["read", "write"] {
        for proxy_variable in [
            "HTTP_PROXY",
            "http_proxy",
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let proxy_url = format!("http://{}", listener.local_addr().unwrap());
            let mut child = Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "tests::authenticated_transport::authenticated_routing_child",
                ])
                .env_clear()
                .env(PROXY_CHILD_CASE, request_kind)
                .env(proxy_variable, proxy_url)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            let started = Instant::now();
            let mut proxy_connections = 0;
            let status = loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        proxy_connections += 1;
                        // Count connections without reading or retaining request credentials.
                        let _ = stream.write_all(
                            b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        );
                    }
                    Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::WouldBlock),
                }
                if let Some(status) = child.try_wait().unwrap() {
                    break status;
                }
                if started.elapsed() > Duration::from_secs(10) {
                    child.kill().unwrap();
                    child.wait().unwrap();
                    panic!("isolated authenticated routing check timed out");
                }
                thread::sleep(Duration::from_millis(5));
            };
            if proxy_connections != 0 || !status.success() {
                failures.push(format!(
                    "{request_kind}/{proxy_variable}: proxy_connections={proxy_connections}, direct_success={}",
                    status.success()
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn authenticated_routing_child() {
    let Ok(request_kind) = std::env::var(PROXY_CHILD_CASE) else {
        return;
    };
    let responses = match request_kind.as_str() {
        "read" => vec![
            library_response("server-10", 42),
            item_response("server-10", 7),
            library_response("server-10", 42),
        ],
        "write" => vec![write_response("server-10", 43, 43)],
        _ => panic!("unknown synthetic routing case"),
    };
    let (base, server) = serve(
        responses
            .into_iter()
            .map(|response| &*Box::leak(response.into_boxed_str()))
            .collect(),
    );
    let adapter =
        Zotero10LocalAdapter::new_with_base(SYNTHETIC_API_KEY, "server-10", base).unwrap();
    let state = match request_kind.as_str() {
        "read" => adapter.get_item("ABCD2345").unwrap(),
        "write" => adapter.write_item(&write_request()).unwrap(),
        _ => unreachable!(),
    };
    assert_eq!(state.item_key, "ABCD2345");
    let requests = server.join().unwrap();
    for request in &requests {
        assert!(request.contains("zotero-api-version: 3\r\n"));
        assert!(request.contains("zotero-server-id: server-10\r\n"));
    }
    if request_kind == "read" {
        assert_eq!(state.library_version, 42);
        assert_eq!(requests.len(), 3);
        assert!(requests[0].starts_with("GET /api/users/0/items?format=versions&limit=1 "));
        assert!(
            requests[1].starts_with("GET /api/users/0/items/ABCD2345?format=json&include=data ")
        );
        assert!(requests[2].starts_with("GET /api/users/0/items?format=versions&limit=1 "));
        assert!(
            requests
                .iter()
                .all(|request| !request.contains("zotero-api-key:"))
        );
    } else {
        assert_eq!(state.library_version, 43);
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("POST /api/users/0/items HTTP/1.1\r\n"));
        assert!(requests[0].contains(&format!("zotero-api-key: {SYNTHETIC_API_KEY}\r\n")));
        assert!(requests[0].contains("if-unmodified-since-version: 42\r\n"));
    }
}

fn padded_response(response: String, version: u64, byte_count: usize) -> &'static str {
    let (_, original_body) = response.split_once("\r\n\r\n").unwrap();
    let body = format!(
        "{original_body}{}",
        " ".repeat(byte_count - original_body.len())
    );
    Box::leak(raw_response(Some("server-10"), Some(version), &body).into_boxed_str())
}

#[test]
fn item_and_library_reads_accept_exactly_the_byte_limit() {
    let byte_count = MAX_ITEM_RESPONSE_BYTES as usize;
    let (base, server) = serve(vec![
        padded_response(library_response("server-10", 42), 42, byte_count),
        padded_response(item_response("server-10", 7), 7, byte_count),
        padded_response(library_response("server-10", 42), 42, byte_count),
    ]);
    let state = transport(base)
        .get_item("ABCD2345")
        .expect("inclusive byte limit must accept each read-stage response");
    assert_eq!(state.library_version, 42);
    assert_eq!(state.item_version, 7);
    assert_eq!(server.join().unwrap().len(), 3);
}

#[test]
fn authenticated_write_accepts_exactly_the_byte_limit() {
    let (base, server) = serve(vec![padded_response(
        write_response("server-10", 43, 43),
        43,
        MAX_ITEM_RESPONSE_BYTES as usize,
    )]);
    let result = transport(base).write_item(&write_request());
    let requests = server.join().unwrap();
    let (headers, request_body) = requests[0].split_once("\r\n\r\n").unwrap();
    let declared_bytes: usize = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        request_body.len(),
        declared_bytes,
        "the synthetic server must consume the complete POST before closing"
    );
    let state = result.expect("inclusive byte limit must accept the write response");
    assert_eq!(state.library_version, 43);
    assert_eq!(state.item_version, 43);
}

#[test]
fn authenticated_write_rejects_one_byte_over_the_limit() {
    let (base, server) = serve(vec![padded_response(
        write_response("server-10", 43, 43),
        43,
        MAX_ITEM_RESPONSE_BYTES as usize + 1,
    )]);
    let result = transport(base).write_item(&write_request());
    server.join().unwrap();
    assert_eq!(result.unwrap_err(), ZoteroTransportError::InvalidResponse);
}
