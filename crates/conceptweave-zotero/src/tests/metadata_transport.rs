use super::*;
use std::process::{Command, Stdio};
use std::time::Instant;

const PROXY_CHILD_CASE: &str = "CONCEPTWEAVE_METADATA_PROXY_CASE";

fn read_fixture(
    body: Vec<u8>,
    declared_bytes: usize,
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
            "HTTP/1.1 200 OK\r\nContent-Length: {declared_bytes}\r\nTotal-Results: 0\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 9.0.6\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nZotero-Server-ID: synthetic-server\r\nConnection: close\r\n\r\n"
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
    let (result, server) = read_fixture(b"[]".to_vec(), 2);
    let report = result.unwrap();
    assert_eq!(report.library_version, 42);
    assert!(report.classified_items.is_empty());
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
    let (result, server) = read_fixture(body, MAX_PAGE_BYTES as usize);
    server.join().unwrap();
    let report = result.expect("exact-limit synthetic JSON must be accepted");
    assert_eq!(report.library_version, 42);
    assert!(report.classified_items.is_empty());
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
        let (result, server) = read_fixture(body, declared_bytes);
        server.join().unwrap();
        assert!(matches!(result, Err(ReadError::Body(_))));
    }
}
