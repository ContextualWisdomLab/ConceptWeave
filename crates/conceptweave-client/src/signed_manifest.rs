//! Authenticated transfer of release pins using independently installed publisher keys.

use ring::signature::{ED25519, UnparsedPublicKey};

use crate::{ReleaseContractError, ReleaseDigest, TrustedReleaseManifest};

const DOMAIN: &str = "conceptweave.signed_release_manifest.v1";
const MAX_ID_BYTES: usize = 4096;

fn valid_id(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= MAX_ID_BYTES && !value.contains('\0')
}

fn put_text(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

/// A publisher public key installed independently of any release or signed pin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedPublisherKey {
    key_id: String,
    public_key: [u8; 32],
}

impl TrustedPublisherKey {
    /// Validates a stable key ID and Ed25519 public-key bytes.
    pub fn new(key_id: impl Into<String>, public_key: &[u8]) -> Result<Self, ReleaseContractError> {
        let key_id = key_id.into();
        if !valid_id(&key_id) || public_key.len() != 32 {
            return Err(ReleaseContractError::InvalidSignedManifest);
        }
        let mut bytes = [0; 32];
        bytes.copy_from_slice(public_key);
        Ok(Self {
            key_id,
            public_key: bytes,
        })
    }

    /// Returns the configured key identity.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Returns the Ed25519 verification key bytes.
    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }
}

/// A release manifest pin signed by one publisher; it contains no trust anchor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedReleaseManifest {
    key_id: String,
    release_id: String,
    manifest_digest: ReleaseDigest,
    signature: [u8; 64],
}

impl SignedReleaseManifest {
    /// Constructs a signed pin without assuming the signature is trusted.
    pub fn new(
        key_id: impl Into<String>,
        release_id: impl Into<String>,
        manifest_digest: ReleaseDigest,
        signature: &[u8],
    ) -> Result<Self, ReleaseContractError> {
        let key_id = key_id.into();
        let release_id = release_id.into();
        if !valid_id(&key_id) || !valid_id(&release_id) || signature.len() != 64 {
            return Err(ReleaseContractError::InvalidSignedManifest);
        }
        let mut bytes = [0; 64];
        bytes.copy_from_slice(signature);
        Ok(Self {
            key_id,
            release_id,
            manifest_digest,
            signature: bytes,
        })
    }

    /// Returns the publisher key identity.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Returns the exact signed release identity.
    pub fn release_id(&self) -> &str {
        &self.release_id
    }

    /// Returns the exact signed manifest digest.
    pub fn manifest_digest(&self) -> &ReleaseDigest {
        &self.manifest_digest
    }

    /// Returns the detached Ed25519 signature bytes.
    pub fn signature(&self) -> &[u8; 64] {
        &self.signature
    }

    /// Encodes the exact domain-separated bytes that a publisher must sign.
    pub fn signing_message(
        key_id: &str,
        release_id: &str,
        manifest_digest: &ReleaseDigest,
    ) -> Result<Vec<u8>, ReleaseContractError> {
        if !valid_id(key_id) || !valid_id(release_id) {
            return Err(ReleaseContractError::InvalidSignedManifest);
        }
        let mut bytes =
            Vec::with_capacity(8 * 4 + DOMAIN.len() + key_id.len() + release_id.len() + 71);
        for value in [DOMAIN, key_id, release_id, manifest_digest.as_str()] {
            put_text(&mut bytes, value);
        }
        Ok(bytes)
    }

    /// Authenticates this pin with an independently configured publisher key.
    pub fn verify(
        &self,
        publisher_keys: &[TrustedPublisherKey],
    ) -> Result<TrustedReleaseManifest, ReleaseContractError> {
        if publisher_keys
            .iter()
            .filter(|key| key.key_id == self.key_id)
            .count()
            > 1
        {
            return Err(ReleaseContractError::DuplicatePublisherKeyId(
                self.key_id.clone(),
            ));
        }
        let key = publisher_keys
            .iter()
            .find(|key| key.key_id == self.key_id)
            .ok_or(ReleaseContractError::UnknownPublisherKey)?;
        let message = Self::signing_message(&self.key_id, &self.release_id, &self.manifest_digest)?;
        UnparsedPublicKey::new(&ED25519, key.public_key)
            .verify(&message, &self.signature)
            .map_err(|_| ReleaseContractError::InvalidPublisherSignature)?;
        TrustedReleaseManifest::new(&self.release_id, self.manifest_digest.clone())
    }
}
