//! Structural RED ensuring compatibility projections preserve observed `pg_class.relreplident`.
//!
//! The public aggregate rebuilds `RelationObservation` values when adapting direct type-kind and
//! true-array bindings to the frozen private v3 validator. Once relation-level replica-identity mode
//! becomes first-class source evidence, both rebuild paths must copy an observed mode and must leave
//! an unobserved mode unobserved.

const PUBLIC_AGGREGATE: &str = include_str!("../src/lib.rs");

fn function_body(name: &str) -> &str {
    let start = PUBLIC_AGGREGATE
        .find(name)
        .unwrap_or_else(|| panic!("expected production function {name}"));
    let tail = &PUBLIC_AGGREGATE[start..];
    let end = tail
        .find("\n}\n\nfn ")
        .map_or(tail.len(), |offset| offset + 2);
    &tail[..end]
}

#[test]
fn compatibility_projections_copy_observed_relation_replica_identity_mode() {
    for function in [
        "fn project_relation_type_kind_bindings",
        "fn project_relation_array_bindings",
    ] {
        let body = function_body(function);
        assert!(
            body.contains(
                "if let Some(replica_identity_mode) = relation.replica_identity_mode()"
            ),
            "{function} must preserve observed relreplident without inventing an unobserved mode"
        );
        assert!(
            body.contains("projected = projected.with_replica_identity_mode(replica_identity_mode)"),
            "{function} must carry the exact observed mode into its rebuilt relation"
        );
    }
}
