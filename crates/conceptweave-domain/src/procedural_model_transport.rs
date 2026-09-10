#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Private Rust transport-admission seam for procedural-model drafts.
//!
//! This module is intentionally not exported while the strict JSON contract is under
//! RED/GREEN development. Successful transport admission does not validate the
//! procedural-model schema, authenticate scope, or grant publication/execution authority.

use std::fmt;

/// Maximum accepted UTF-8 transport size before any structural parsing.
pub const MAX_PROCEDURAL_TRANSPORT_BYTES: usize = 2 * 1024 * 1024;

/// Fixed transport error codes; diagnostics never contain attacker-controlled JSON text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralTransportError {
    /// Input exceeded the bounded transport size.
    InputTooLarge,
    /// Input was not strict JSON for this admission boundary.
    InvalidJson,
    /// An object repeated the same decoded member name.
    DuplicateMember,
    /// Structural nesting exceeded the parser ceiling.
    DepthLimit,
}

impl fmt::Display for ProceduralTransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::InputTooLarge => "input_too_large",
            Self::InvalidJson => "invalid_json",
            Self::DuplicateMember => "duplicate_member",
            Self::DepthLimit => "depth_limit",
        };
        formatter.write_str(code)
    }
}

impl std::error::Error for ProceduralTransportError {}

/// Admits one bounded UTF-8 JSON transport before schema/domain projection.
///
/// RED scaffold: only the outer byte ceiling exists at this commit. The following
/// regression must demonstrate that duplicate decoded members are still admitted
/// before the strict parser repair is implemented.
pub fn admit_procedural_json_transport(input: &str) -> Result<(), ProceduralTransportError> {
    if input.len() > MAX_PROCEDURAL_TRANSPORT_BYTES {
        return Err(ProceduralTransportError::InputTooLarge);
    }
    Ok(())
}
