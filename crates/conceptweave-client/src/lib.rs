#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Offline ConceptWeave semantic-release client contracts.
//!
//! This crate is intentionally provider- and transport-independent. A consumer
//! can inspect release identity, provenance and governance state before making
//! authoritative use of a semantic release. Generator-private classes, source
//! database access and LLM orchestration stay outside this boundary.

use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};
use core::fmt;
use std::collections::BTreeSet;

/// A validated content-digest identity carried by a semantic release.
///
/// The current contract accepts only the explicit `sha256:<64 hex>` shape. This
/// value object validates digest identity syntax; byte-for-byte cryptographic
/// re-hashing belongs to the serialized-artifact verification adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDigest(String);

impl ReleaseDigest {
    /// Parses a release digest and rejects unsupported algorithms or malformed hex.
    pub fn new(value: impl Into<String>) -> Result<Self, ReleaseContractError> {
        let value = value.into();
        let Some(hex) = value.strip_prefix("sha256:") else {
            return Err(ReleaseContractError::InvalidDigest);
        };
        if hex.len() != 64 {
            return Err(ReleaseContractError::InvalidDigest);
        }
        if !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ReleaseContractError::InvalidDigest);
        }
        Ok(Self(value))
    }

    /// Returns the canonical digest identity string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable identity and version metadata for one semantic release.
///
/// Keeping these related identity fields in one value object prevents call sites
/// from relying on a long positional constructor and makes later compatibility
/// policy explicit without exposing mutable release internals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseMetadata {
    release_id: String,
    contract_version: String,
    ontology_version: String,
}

impl ReleaseMetadata {
    /// Creates validated stable release, contract, and ontology identities.
    pub fn new(
        release_id: impl Into<String>,
        contract_version: impl Into<String>,
        ontology_version: impl Into<String>,
    ) -> Result<Self, ReleaseContractError> {
        let release_id = release_id.into();
        let contract_version = contract_version.into();
        let ontology_version = ontology_version.into();
        require_non_blank(&release_id, "release_id")?;
        require_non_blank(&contract_version, "contract_version")?;
        require_non_blank(&ontology_version, "ontology_version")?;
        Ok(Self {
            release_id,
            contract_version,
            ontology_version,
        })
    }

    /// Returns the stable semantic-release identity.
    pub fn release_id(&self) -> &str {
        &self.release_id
    }

    /// Returns the client contract version encoded by the release.
    pub fn contract_version(&self) -> &str {
        &self.contract_version
    }

    /// Returns the ontology/model version encoded by the release.
    pub fn ontology_version(&self) -> &str {
        &self.ontology_version
    }
}

/// Immutable client-visible metadata required to admit a semantic release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRelease {
    metadata: ReleaseMetadata,
    truth_status: TruthStatus,
    publication_state: PublicationState,
    artifact_digest: ReleaseDigest,
    provenance: Vec<EvidenceReference>,
    concept_ids: Vec<String>,
}

impl SemanticRelease {
    /// Constructs a structurally valid semantic release contract.
    ///
    /// Construction validates provenance and concept identity uniqueness while
    /// [`ReleaseMetadata`] validates stable release/version identities. Whether
    /// the release is safe for authoritative use is a separate client-policy
    /// decision performed by [`SemanticReleaseClient::validate_for_authoritative_use`].
    pub fn new(
        metadata: ReleaseMetadata,
        truth_status: TruthStatus,
        publication_state: PublicationState,
        artifact_digest: ReleaseDigest,
        provenance: Vec<EvidenceReference>,
        concept_ids: Vec<String>,
    ) -> Result<Self, ReleaseContractError> {
        if provenance.is_empty() {
            return Err(ReleaseContractError::MissingProvenance);
        }

        let mut unique_concepts = BTreeSet::new();
        for concept_id in &concept_ids {
            require_non_blank(concept_id, "concept_id")?;
            if !unique_concepts.insert(concept_id.as_str()) {
                return Err(ReleaseContractError::DuplicateConceptId(concept_id.clone()));
            }
        }

        Ok(Self {
            metadata,
            truth_status,
            publication_state,
            artifact_digest,
            provenance,
            concept_ids,
        })
    }

    /// Returns the stable semantic-release identity.
    pub fn release_id(&self) -> &str {
        self.metadata.release_id()
    }

    /// Returns the client contract version encoded by this release.
    pub fn contract_version(&self) -> &str {
        self.metadata.contract_version()
    }

    /// Returns the ontology/model version carried by this release.
    pub fn ontology_version(&self) -> &str {
        self.metadata.ontology_version()
    }

    /// Returns the release truth status.
    pub fn truth_status(&self) -> TruthStatus {
        self.truth_status
    }

    /// Returns the governance/publication state of this release.
    pub fn publication_state(&self) -> PublicationState {
        self.publication_state
    }

    /// Returns the declared immutable artifact digest identity.
    pub fn artifact_digest(&self) -> &ReleaseDigest {
        &self.artifact_digest
    }

    /// Returns immutable evidence/provenance references for this release.
    pub fn provenance(&self) -> &[EvidenceReference] {
        &self.provenance
    }

    /// Returns stable concept identifiers carried by this release.
    pub fn concept_ids(&self) -> &[String] {
        &self.concept_ids
    }
}

/// Offline admission policy for one supported semantic-release contract version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReleaseClient {
    supported_contract_version: String,
}

impl SemanticReleaseClient {
    /// Creates a client pinned to one explicit semantic-release contract version.
    pub fn new(
        supported_contract_version: impl Into<String>,
    ) -> Result<Self, ReleaseContractError> {
        let supported_contract_version = supported_contract_version.into();
        require_non_blank(&supported_contract_version, "supported_contract_version")?;
        Ok(Self {
            supported_contract_version,
        })
    }

    /// Returns the exact semantic-release contract version this client accepts.
    pub fn supported_contract_version(&self) -> &str {
        &self.supported_contract_version
    }

    /// Fails closed unless a release is compatible, Published and Authoritative.
    ///
    /// This check is deterministic and performs no network or model calls. It is
    /// suitable as an admission gate before a consuming product performs its own
    /// tenant/purpose authorization and physical query planning.
    pub fn validate_for_authoritative_use(
        &self,
        release: &SemanticRelease,
    ) -> Result<(), ReleaseContractError> {
        if release.contract_version() != self.supported_contract_version {
            return Err(ReleaseContractError::UnsupportedContractVersion {
                expected: self.supported_contract_version.clone(),
                actual: release.contract_version().to_string(),
            });
        }
        if release.publication_state != PublicationState::Published {
            return Err(ReleaseContractError::ReleaseNotPublished {
                actual: release.publication_state,
            });
        }
        if release.truth_status != TruthStatus::Authoritative {
            return Err(ReleaseContractError::ReleaseNotAuthoritative {
                actual: release.truth_status,
            });
        }
        Ok(())
    }
}

fn require_non_blank(value: &str, field: &'static str) -> Result<(), ReleaseContractError> {
    if value.trim().is_empty() {
        return Err(ReleaseContractError::EmptyField(field));
    }
    Ok(())
}

/// A deterministic semantic-release contract or admission failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseContractError {
    /// A required stable identity or version field was blank.
    EmptyField(&'static str),
    /// The declared release digest is not `sha256:<64 hex>`.
    InvalidDigest,
    /// The release carries no provenance evidence.
    MissingProvenance,
    /// The release repeats one semantic concept identity.
    DuplicateConceptId(String),
    /// The release uses a contract version this client does not support.
    UnsupportedContractVersion {
        /// Contract version required by the client.
        expected: String,
        /// Contract version supplied by the release.
        actual: String,
    },
    /// The release has not crossed the governed Published boundary.
    ReleaseNotPublished {
        /// Actual release publication state.
        actual: PublicationState,
    },
    /// The release is Published but its truth status is not Authoritative.
    ReleaseNotAuthoritative {
        /// Actual release truth status.
        actual: TruthStatus,
    },
}

impl fmt::Display for ReleaseContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "required field `{field}` is blank"),
            Self::InvalidDigest => write!(formatter, "release digest must use sha256:<64 hex>"),
            Self::MissingProvenance => {
                write!(formatter, "semantic releases require provenance evidence")
            }
            Self::DuplicateConceptId(concept_id) => write!(
                formatter,
                "semantic release contains duplicate concept id `{concept_id}`"
            ),
            Self::UnsupportedContractVersion { expected, actual } => write!(
                formatter,
                "semantic release contract version `{actual}` is unsupported; expected `{expected}`"
            ),
            Self::ReleaseNotPublished { actual } => {
                write!(formatter, "semantic release is {actual:?}, not Published")
            }
            Self::ReleaseNotAuthoritative { actual } => write!(
                formatter,
                "semantic release truth status is {actual:?}, not Authoritative"
            ),
        }
    }
}

impl std::error::Error for ReleaseContractError {}
