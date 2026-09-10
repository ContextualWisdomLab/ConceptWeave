#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Private transport-to-domain admission seam for canonical procedural-model drafts.
//!
//! This module remains unpublished while Draft 2020-12 mapping is developed. It must
//! not authenticate callers or referenced artifacts, publish a model, or authorize
//! execution merely because deterministic transport/schema/domain checks succeed.

use crate::procedural_model_transport::{
    admit_procedural_json_transport_bytes, ProceduralTransportError,
};
use crate::procedural_model_validation::{
    ProceduralScope, ProceduralValidationError, ReachabilityRule, TopologySummary,
};
use std::fmt;

/// Fixed ingress failures; diagnostics never echo untrusted draft content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralIngressError {
    /// Raw bytes failed the bounded strict-JSON transport gate.
    Transport(ProceduralTransportError),
    /// JSON was syntactically valid but did not satisfy the canonical draft shape.
    SchemaInvalid,
    /// A schema-valid projection failed deterministic semantic/topology validation.
    Validation(ProceduralValidationError),
}

impl fmt::Display for ProceduralIngressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => write!(formatter, "transport:{error}"),
            Self::SchemaInvalid => formatter.write_str("schema_invalid"),
            Self::Validation(error) => write!(formatter, "validation:{error}"),
        }
    }
}

impl std::error::Error for ProceduralIngressError {}

impl From<ProceduralTransportError> for ProceduralIngressError {
    fn from(error: ProceduralTransportError) -> Self {
        Self::Transport(error)
    }
}

/// Admits one canonical procedural-model draft from raw bytes through deterministic
/// transport, Draft 2020-12 shape mapping, and borrowed-domain validation.
///
/// `expected_scope` is caller-supplied context only; this function does not establish
/// that the caller or scope is authentic. Released artifact authenticity, ACL,
/// stewardship, publication and runtime authority remain separate gates.
pub fn validate_procedural_model_json_transport(
    input: &[u8],
    _expected_scope: ProceduralScope<'_>,
    _reachability: ReachabilityRule,
) -> Result<TopologySummary, ProceduralIngressError> {
    admit_procedural_json_transport_bytes(input)?;
    // RED scaffold: canonical Draft 2020-12 materialization/mapping is intentionally
    // absent so a shape-valid document cannot yet be promoted to a domain projection.
    Err(ProceduralIngressError::SchemaInvalid)
}
