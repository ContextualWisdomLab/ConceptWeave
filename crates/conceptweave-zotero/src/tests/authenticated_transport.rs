use super::*;
use std::process::{Command, Stdio};
use std::time::Instant;

const PROXY_CHILD_CASE: &str = "CONCEPTWEAVE_AUTHENTICATED_PROXY_CASE";
const SYNTHETIC_API_KEY: &str = "0123456789abcdef0123456789abcdef";

#[test]
fn authenticated_calls_never_use_environment_proxies() {
    let mut failures = Vec::new();
    for request_kind in ["read", "write", "authorize"] {
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
    if request_kind == "authorize" {
        let body = format!(r#"{{"key":"{SYNTHETIC_API_KEY}","remember":true}}"#);
        let (result, server) = authorization_fixture("200 OK", &body);
        assert!(result.unwrap().remembered());
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("POST /api/local/authorize HTTP/1.1\r\n"));
        assert!(requests[0].contains("zotero-server-id: server-10\r\n"));
        assert!(requests[0].ends_with(r#"{"appName":"ConceptWeave"}"#));
        assert!(!requests[0].contains(SYNTHETIC_API_KEY));
        assert!(!requests[0].contains("zotero-api-key:"));
        return;
    }
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

fn authorization_fixture(
    status: &str,
    body: &str,
) -> (
    Result<Zotero10LocalAuthorization, ZoteroTransportError>,
    thread::JoinHandle<Vec<String>>,
) {
    let response = authorize_response(status, Some("server-10"), body);
    let (items_base, server) = serve(vec![Box::leak(response.into_boxed_str())]);
    let result = Zotero10LocalAuthorization::request_with_base(
        "ConceptWeave",
        "server-10",
        items_base.trim_end_matches("/api/users/0/items").into(),
    );
    (result, server)
}

#[test]
fn authorization_accepts_exactly_the_byte_limit() {
    let mut body = format!(r#"{{"key":"{SYNTHETIC_API_KEY}","remember":true}}"#);
    body.push_str(&" ".repeat(MAX_AUTH_RESPONSE_BYTES as usize - body.len()));
    let (result, server) = authorization_fixture("200 OK", &body);
    server.join().unwrap();
    assert!(
        result
            .expect("exact-limit authorization must succeed")
            .remembered()
    );
}

#[test]
fn authorization_denial_accepts_exactly_the_byte_limit() {
    let mut body = r#"{"denied":true}"#.to_owned();
    body.push_str(&" ".repeat(MAX_AUTH_RESPONSE_BYTES as usize - body.len()));
    let (result, server) = authorization_fixture("403 Forbidden", &body);
    server.join().unwrap();
    assert!(matches!(result, Err(ZoteroTransportError::Denied)));
}

#[test]
fn authorization_rejects_one_byte_over_the_limit_for_success_and_denial() {
    for (status, mut body) in [
        (
            "200 OK",
            format!(r#"{{"key":"{SYNTHETIC_API_KEY}","remember":true}}"#),
        ),
        ("403 Forbidden", r#"{"denied":true}"#.to_owned()),
    ] {
        body.push_str(&" ".repeat(MAX_AUTH_RESPONSE_BYTES as usize + 1 - body.len()));
        let (result, server) = authorization_fixture(status, &body);
        server.join().unwrap();
        assert!(matches!(result, Err(ZoteroTransportError::InvalidResponse)));
    }
}
