use super::*;
use std::process::{Command, Stdio};
use std::time::Instant;

const PROXY_CHILD_CASE: &str = "CONCEPTWEAVE_METADATA_PROXY_CASE";

fn read_fixture(
    body: Vec<u8>,
    declared_bytes: usize,
    total_results: usize,
) -> (
    Result<ClassificationReport, ReadError>,
    thread::JoinHandle<String>,
) {
    let _guard = LOCAL_API_TEST_LOCK.lock().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    *TEST_LOCAL_API.lock().unwrap() = Some(format!(
        "http://{}/api/users/0/items",
        listener.local_addr().unwrap()
    ));
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = Vec::new();
        while !request.windows(4).any(|part| part == b"\r\n\r\n") {
            let mut buffer = [0; 4096];
            let length = stream.read(&mut buffer).unwrap();
            assert_ne!(length, 0);
            request.extend_from_slice(&buffer[..length]);
        }
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {declared_bytes}\r\nTotal-Results: {total_results}\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nZotero-Server-ID: synthetic-server\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(headers.as_bytes()).unwrap();
        // An invalid or oversized body can make the client close before all bytes arrive.
        let _ = stream.write_all(&body);
        String::from_utf8(request).unwrap()
    });
    let result = read_local_snapshot();
    *TEST_LOCAL_API.lock().unwrap() = None;
    (result, server)
}

fn read_raw_response(response: Vec<u8>) -> Result<ClassificationReport, ReadError> {
    let _guard = LOCAL_API_TEST_LOCK.lock().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    *TEST_LOCAL_API.lock().unwrap() = Some(format!(
        "http://{}/api/users/0/items",
        listener.local_addr().unwrap()
    ));
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = Vec::new();
        while !request.windows(4).any(|part| part == b"\r\n\r\n") {
            let mut buffer = [0; 4096];
            let length = stream.read(&mut buffer).unwrap();
            assert_ne!(length, 0);
            request.extend_from_slice(&buffer[..length]);
        }
        stream.write_all(&response).unwrap();
    });
    let result = read_local_snapshot();
    *TEST_LOCAL_API.lock().unwrap() = None;
    server.join().unwrap();
    result
}

fn successful_response(extra_headers: &str, body: &[u8]) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nTotal-Results: 0\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\n{extra_headers}Connection: close\r\n\r\n",
        body.len()
    )
    .into_bytes()
    .into_iter()
    .chain(body.iter().copied())
    .collect()
}

fn source_item_body(key: &str) -> Vec<u8> {
    serde_json::to_vec(&vec![item(key, "attachment", "", "", "")]).unwrap()
}

#[test]
fn snapshot_rejects_noncanonical_zotero_object_keys() {
    for invalid_key in [
        "ABCDEFG",
        "ABCDEFGHI",
        "ABC1DEFG",
        "ABCOEFGH",
        "abcdefgh",
        "ABC-DEF2",
    ] {
        let body = source_item_body(invalid_key);
        let body_len = body.len();
        let (result, server) = read_fixture(body, body_len, 1);
        server.join().unwrap();
        assert!(
            matches!(result, Err(ReadError::SnapshotChanged)),
            "noncanonical Zotero object key was admitted: {invalid_key}"
        );
    }

    let body = source_item_body("2A3B4C5D");
    let body_len = body.len();
    let (result, server) = read_fixture(body, body_len, 1);
    server.join().unwrap();
    let valid = result.unwrap();
    assert_eq!(valid.observed_item_count, 1);
}

#[test]
fn snapshot_never_uses_environment_proxies() {
    let mut failures = Vec::new();
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
        // Isolate synthetic settings in the child; never mutate the test process
        // environment or route requests to the real Zotero port.
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::metadata_transport::metadata_routing_child",
            ])
            .env_clear()
            .env(PROXY_CHILD_CASE, "synthetic")
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
                panic!("isolated metadata routing check timed out");
            }
            thread::sleep(Duration::from_millis(5));
        };
        if proxy_connections != 0 || !status.success() {
            failures.push(format!(
                "{proxy_variable}: proxy_connections={proxy_connections}, direct_success={}",
                status.success()
            ));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn metadata_routing_child() {
    if std::env::var_os(PROXY_CHILD_CASE).is_none() {
        return;
    }
    let (result, server) = read_fixture(b"[]".to_vec(), 2, 0);
    let report = result.unwrap();
    assert_eq!(report.library_version(), 42);
    assert!(report.classified_items().is_empty());
    let request = server.join().unwrap();
    assert!(request.starts_with(
        "GET /api/users/0/items?format=json&include=data&limit=100&start=0 HTTP/1.1\r\n"
    ));
    assert!(request.contains("zotero-api-version: 3\r\n"));
    assert!(!request.contains("zotero-api-key:"));
}

#[test]
fn snapshot_accepts_a_response_exactly_at_the_byte_limit() {
    let mut body = b"[]".to_vec();
    body.resize(MAX_PAGE_BYTES as usize, b' ');
    let (result, server) = read_fixture(body, MAX_PAGE_BYTES as usize, 0);
    server.join().unwrap();
    let report = result.expect("exact-limit synthetic JSON must be accepted");
    assert_eq!(report.library_version(), 42);
    assert!(report.classified_items().is_empty());
}

#[test]
fn snapshot_rejects_oversized_invalid_utf8_and_truncated_bodies() {
    let mut oversized = b"[]".to_vec();
    oversized.resize(MAX_PAGE_BYTES as usize + 1, b' ');
    for (body, declared_bytes) in [
        (oversized, MAX_PAGE_BYTES as usize + 1),
        (vec![0xff], 1),
        (b"[]".to_vec(), 3),
    ] {
        let (result, server) = read_fixture(body, declared_bytes, 0);
        server.join().unwrap();
        assert!(matches!(result, Err(ReadError::Body(_))));
    }
}

#[test]
fn production_transport_rejects_missing_and_malformed_required_headers() {
    let missing_total = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nConnection: close\r\n\r\n[]".to_vec();
    assert!(matches!(
        read_raw_response(missing_total),
        Err(ReadError::Header("Total-Results"))
    ));

    let malformed_total = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nTotal-Results: many\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nConnection: close\r\n\r\n[]".to_vec();
    assert!(matches!(
        read_raw_response(malformed_total),
        Err(ReadError::Header("Total-Results"))
    ));

    for missing in [
        "Last-Modified-Version",
        "X-Zotero-Version",
        "Zotero-API-Version",
        "Zotero-Schema-Version",
    ] {
        let headers = [
            ("Total-Results", "0"),
            ("Last-Modified-Version", "42"),
            ("X-Zotero-Version", "9.0.6"),
            ("Zotero-API-Version", "3"),
            ("Zotero-Schema-Version", "42"),
        ]
        .into_iter()
        .filter(|(name, _)| *name != missing)
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .collect::<String>();
        let response =
            format!("HTTP/1.1 200 OK\r\nContent-Length: 2\r\n{headers}Connection: close\r\n\r\n[]")
                .into_bytes();
        match read_raw_response(response).unwrap_err() {
            ReadError::Header(actual) => assert_eq!(actual, missing),
            error => panic!("unexpected transport error: {error}"),
        }
    }
}

#[test]
fn production_transport_accepts_absent_optional_server_id_and_rejects_bad_json() {
    let report = read_raw_response(successful_response("", b"[]")).unwrap();
    assert_eq!(report.server_id(), None);

    assert!(matches!(
        read_raw_response(successful_response("Zotero-Server-ID: synthetic\r\n", b"{")),
        Err(ReadError::Json(_))
    ));
}

#[test]
fn production_transport_reports_connection_failures() {
    let _guard = LOCAL_API_TEST_LOCK.lock().unwrap();
    *TEST_LOCAL_API.lock().unwrap() = Some("http://127.0.0.1:0/api/users/0/items".to_owned());
    let result = read_local_snapshot();
    *TEST_LOCAL_API.lock().unwrap() = None;

    assert!(matches!(result, Err(ReadError::Http(_))));
}

#[test]
fn header_helpers_cover_optional_and_numeric_boundaries() {
    let mut headers = ureq::http::HeaderMap::new();
    headers.insert("X-Count", ureq::http::HeaderValue::from_static("42"));
    headers.insert(
        "X-Text",
        ureq::http::HeaderValue::from_static("synthetic-server"),
    );
    headers.insert(
        "X-Opaque",
        ureq::http::HeaderValue::from_bytes(&[0x80]).unwrap(),
    );

    assert_eq!(header_u64(&headers, "X-Count").unwrap(), 42);
    assert_eq!(
        header_string(&headers, "X-Text").unwrap(),
        "synthetic-server"
    );
    assert_eq!(optional_header(&headers, "X-Missing"), None);
    assert_eq!(optional_header(&headers, "X-Opaque"), None);

    headers.insert("X-Bad-Count", ureq::http::HeaderValue::from_static("many"));
    assert!(matches!(
        header_u64(&headers, "X-Bad-Count"),
        Err(ReadError::Header("X-Bad-Count"))
    ));
    assert!(matches!(
        header_string(&headers, "X-Missing"),
        Err(ReadError::Header("X-Missing"))
    ));
}

#[test]
fn production_transport_helpers_remain_in_owned_coverage() {
    let source = include_str!("../lib.rs");
    for function in [
        "fetch_local_page",
        "header_u64",
        "header_string",
        "optional_header",
    ] {
        let excluded = format!("#[cfg_attr(coverage_nightly, coverage(off))]\nfn {function}");
        assert!(
            !source.contains(&excluded),
            "production Local API helper {function} is still excluded from owned coverage"
        );
    }
}

#[test]
fn transport_rustdoc_tracks_the_owned_coverage_boundary() {
    let source = include_str!("../lib.rs");
    assert!(
        !source.contains("ureq transport shim is excluded from deterministic coverage"),
        "read_local_snapshot rustdoc still claims the now-covered transport seam is excluded"
    );
    assert!(
        source.contains("Local API request/header/body path are exercised"),
        "read_local_snapshot rustdoc must describe the loopback-covered production seam"
    );
}
