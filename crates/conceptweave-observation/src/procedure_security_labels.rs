//! Provider-owned source labels are evidence, never application authorization.

use crate::{
    ObservationError, ReferencedProcedureDefinitionObservation, ReferencedProcedureLocation,
    encode_bytes, encode_len, encode_sha256, encode_str,
};
use sha2::{Digest, Sha256};
use std::fmt;

/// One unchanged provider name and its opaque source security label.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcedureSecurityLabel {
    provider: String,
    label: String,
}
impl fmt::Debug for ProcedureSecurityLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcedureSecurityLabel")
            .finish_non_exhaustive()
    }
}
impl ProcedureSecurityLabel {
    /// Records exact source text. Empty labels remain observable; empty providers and NUL are invalid.
    pub fn new(
        provider: impl Into<String>,
        label: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let provider = provider.into();
        let label = label.into();
        if provider.is_empty() || provider.contains('\0') || label.contains('\0') {
            return Err(invalid());
        }
        Ok(Self { provider, label })
    }
    /// Returns the exact provider name, without interpreting its authority.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }
    /// Returns the unchanged label, whose semantics belong to its source provider.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Complete provider-to-label map for one overloaded procedure signature.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcedureSecurityLabelsObservation {
    location: ReferencedProcedureLocation,
    labels: Vec<ProcedureSecurityLabel>,
}
impl fmt::Debug for ProcedureSecurityLabelsObservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcedureSecurityLabelsObservation")
            .field("location", &self.location)
            .field("labels", &"<redacted>")
            .finish()
    }
}
impl ProcedureSecurityLabelsObservation {
    /// Records an observed label map, rejecting duplicate providers even if their labels match.
    pub fn new(
        location: ReferencedProcedureLocation,
        mut labels: Vec<ProcedureSecurityLabel>,
    ) -> Result<Self, ObservationError> {
        labels.sort_by(|a, b| a.provider.cmp(&b.provider));
        if labels
            .windows(2)
            .any(|pair| pair[0].provider == pair[1].provider)
        {
            return Err(invalid());
        }
        Ok(Self { location, labels })
    }
    /// Returns the exact overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
    /// Returns all observed labels in exact provider-name order; an empty slice means observed absence.
    #[must_use]
    pub fn labels(&self) -> &[ProcedureSecurityLabel] {
        &self.labels
    }
}

pub(crate) fn canonicalize(
    definitions: &[ReferencedProcedureDefinitionObservation],
    mut items: Vec<ProcedureSecurityLabelsObservation>,
) -> Result<Vec<ProcedureSecurityLabelsObservation>, ObservationError> {
    items.sort_by_cached_key(|item| item.location.canonical_location());
    if items.len() != definitions.len()
        || items
            .iter()
            .zip(definitions)
            .any(|(item, definition)| item.location() != definition.location())
    {
        return Err(invalid());
    }
    Ok(items)
}
pub(crate) fn digest(base: &str, items: &[ProcedureSecurityLabelsObservation]) -> String {
    let mut hash = Sha256::new();
    encode_bytes(
        &mut hash,
        b"conceptweave.postgres_schema_snapshot.v3.procedure_security_labels.v1",
    );
    encode_str(&mut hash, base);
    encode_len(&mut hash, items.len());
    for item in items {
        encode_str(&mut hash, &item.location.canonical_location());
        encode_len(&mut hash, item.labels.len());
        for label in &item.labels {
            encode_str(&mut hash, &label.provider);
            encode_str(&mut hash, &label.label);
        }
    }
    encode_sha256(hash)
}
fn invalid() -> ObservationError {
    ObservationError::InvalidObservationField {
        field: "procedure_security_labels",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labels_preserve_opaque_text_and_reject_duplicate_providers() {
        let location = ReferencedProcedureLocation::new("pg_catalog", "f", vec![]).unwrap();
        let empty = ProcedureSecurityLabel::new(" ", "").unwrap();
        assert_eq!(empty.provider(), " ");
        assert_eq!(empty.label(), "");
        let private = ProcedureSecurityLabel::new("provider", " private label ").unwrap();
        assert_eq!(private.label(), " private label ");
        assert!(!format!("{private:?}").contains("private label"));
        for (provider, label) in [("", "label"), ("p\0", "label"), ("p", "label\0")] {
            assert!(ProcedureSecurityLabel::new(provider, label).is_err());
        }
        assert!(
            ProcedureSecurityLabelsObservation::new(
                location.clone(),
                vec![private.clone(), private.clone()]
            )
            .is_err()
        );
        let observed = ProcedureSecurityLabelsObservation::new(
            location.clone(),
            vec![private.clone(), empty.clone()],
        )
        .unwrap();
        let definition = ReferencedProcedureDefinitionObservation::new(
            location.clone(),
            crate::QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            42,
            "owner",
            "definition",
        )
        .unwrap();
        let definitions = [definition];
        assert!(canonicalize(&definitions, vec![]).is_err());
        assert!(canonicalize(&definitions, vec![observed.clone(), observed.clone()]).is_err());
        let wrong = ProcedureSecurityLabelsObservation::new(
            ReferencedProcedureLocation::new("pg_catalog", "g", vec![]).unwrap(),
            vec![],
        )
        .unwrap();
        assert!(canonicalize(&definitions, vec![wrong]).is_err());
        assert_eq!(
            canonicalize(&definitions, vec![observed.clone()]).unwrap(),
            vec![observed.clone()]
        );
        assert!(canonicalize(&[], vec![]).unwrap().is_empty());
        assert_eq!(observed.location(), &location);
        assert_eq!(observed.labels(), &[empty, private]);
        assert!(!format!("{observed:?}").contains("private label"));
        assert_ne!(
            digest("base", &[observed]),
            digest(
                "base",
                &[ProcedureSecurityLabelsObservation::new(location, vec![]).unwrap()]
            )
        );
    }
}
