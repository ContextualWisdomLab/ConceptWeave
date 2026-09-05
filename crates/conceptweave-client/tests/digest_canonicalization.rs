use conceptweave_client::{ReleaseContractError, ReleaseDigest};

#[test]
fn uppercase_sha256_digest_identity_is_rejected() {
    let uppercase_digest = format!("sha256:{}", "A".repeat(64));

    assert_eq!(
        ReleaseDigest::new(&uppercase_digest),
        Err(ReleaseContractError::InvalidDigest)
    );
}
