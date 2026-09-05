use super::*;
use std::process::{Command, Stdio};
use std::time::Instant;

const CHILD_CASE: &str = "CONCEPTWEAVE_PROXY_ISOLATION_CASE";

fn assert_direct_routing(test_case: &str) {
    let mut failures = Vec::new();
    for proxy_variable in [
        "HTTP_PROXY",
        "http_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
    ] {
        let proxy_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        proxy_listener.set_nonblocking(true).unwrap();
        let proxy_url = format!("http://{}", proxy_listener.local_addr().unwrap());
        // Only the child receives synthetic settings. No process-global environment
        // mutation or actual local Zotero endpoint is used by this regression.
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "tests::proxy_isolation::proxy_isolation_child"])
            .env_clear()
            .env(CHILD_CASE, test_case)
            .env(proxy_variable, proxy_url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let started = Instant::now();
        let mut proxy_connections = 0;
        let child_status = loop {
            match proxy_listener.accept() {
                Ok((mut stream, _)) => {
                    proxy_connections += 1;
                    // Fail a misrouted request promptly without reading its body.
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
                panic!("isolated {test_case} routing check timed out");
            }
            thread::sleep(Duration::from_millis(5));
        };
        if proxy_connections != 0 || !child_status.success() {
            failures.push(format!(
                "{proxy_variable}: proxy_connections={proxy_connections}, direct_success={}",
                child_status.success()
            ));
        }
    }
    assert!(failures.is_empty(), "{test_case}: {failures:?}");
}

#[test]
fn snapshot_ignores_proxy_environment() {
    assert_direct_routing("snapshot");
}

#[test]
fn authorization_ignores_proxy_environment() {
    assert_direct_routing("authorization");
}

#[test]
fn item_reads_and_authenticated_writes_ignore_proxy_environment() {
    assert_direct_routing("item");
}

#[test]
fn proxy_isolation_child() {
    let Ok(test_case) = std::env::var(CHILD_CASE) else {
        return;
    };
    match test_case.as_str() {
        "snapshot" => {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nTotal-Results: 0\r\nLast-Modified-Version: 42\r\nX-Zotero-Version: 10.0.0\r\nZotero-API-Version: 3\r\nZotero-Schema-Version: 42\r\nZotero-Server-ID: server-10\r\nConnection: close\r\n\r\n[]";
            let (base, server) = serve(vec![response]);
            *TEST_LOCAL_API.lock().unwrap() = Some(base);
            let report = read_local_snapshot();
            *TEST_LOCAL_API.lock().unwrap() = None;
            let report = report.unwrap();
            assert_eq!(report.library_version, 42);
            assert!(report.classified_items.is_empty());
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].starts_with(
                "GET /api/users/0/items?format=json&include=data&limit=100&start=0 HTTP/1.1\r\n"
            ));
            assert!(requests[0].contains("zotero-api-version: 3\r\n"));
        }
        "authorization" => {
            let response = authorize_response(
                "200 OK",
                Some("server-10"),
                r#"{"key":"0123456789abcdef0123456789abcdef","remember":true}"#,
            );
            let (base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
            let authorization = Zotero10LocalAuthorization::request_with_base(
                "Synthetic Proxy Isolation Test",
                "server-10",
                base.replace("/api/users/0/items", ""),
            )
            .unwrap();
            assert!(authorization.remembered());
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].starts_with("POST /api/local/authorize HTTP/1.1\r\n"));
            assert!(requests[0].contains("zotero-server-id: server-10\r\n"));
        }
        "item" => {
            let responses = [
                library_response("server-10", 42),
                item_response("server-10", 7),
                library_response("server-10", 42),
                write_response("server-10", 43, 43),
            ]
            .into_iter()
            .map(|response| &*Box::leak(response.into_boxed_str()))
            .collect();
            let (base, server) = serve(responses);
            let adapter = transport(base);
            assert_eq!(adapter.get_item("ABCD2345").unwrap().library_version, 42);
            assert_eq!(
                adapter.write_item(&write_request()).unwrap().item_version,
                43
            );
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 4);
            assert!(requests[1].starts_with("GET /api/users/0/items/ABCD2345?"));
            assert!(requests[3].starts_with("POST /api/users/0/items HTTP/1.1\r\n"));
            assert!(requests[3].contains("zotero-api-key: 0123456789abcdef0123456789abcdef\r\n"));
            assert!(requests[3].contains("if-unmodified-since-version: 42\r\n"));
        }
        _ => panic!("unknown synthetic routing case"),
    }
}
