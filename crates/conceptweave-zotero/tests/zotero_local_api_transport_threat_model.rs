const THREAT_MODEL: &str = include_str!("../../../THREAT_MODEL.md");

#[test]
fn zotero_local_api_transport_boundary_is_explicit() {
    for required_statement in [
        "http://localhost:23119/api/",
        "Zotero-Server-ID is not cryptographic server authentication",
        "hostile same-host process",
        "enterprise-secure live write-back",
        "fail closed",
    ] {
        assert!(
            THREAT_MODEL.contains(required_statement),
            "THREAT_MODEL.md must preserve the Zotero transport boundary: {required_statement}"
        );
    }
}
