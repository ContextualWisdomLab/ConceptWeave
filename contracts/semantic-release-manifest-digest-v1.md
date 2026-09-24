# Semantic release manifest digest v1

The client computes `sha256:<64 lowercase hex>` over the following byte stream. This digest is distinct from the release's declared artifact digest. A consuming product must obtain the expected manifest digest and release ID from a protected Governance & Publication channel independently of the release it checks.

All text is UTF-8. `text(x)` is an unsigned 64-bit big-endian byte length followed by the exact UTF-8 bytes of `x`. `count(n)` is an unsigned 64-bit big-endian count. No trimming, case folding, JSON serialization, or timestamp is part of this encoding.

1. `text("conceptweave.semantic_release_manifest.v1")`.
2. `text(release_id)`, `text(contract_version)`, `text(ontology_version)`, `text(artifact_digest)` in that order.
3. One truth-status byte, then one publication-state byte. Truth tags: Observed `0`, Inferred `1`, Proposed `2`, Authoritative `3`, Superseded `4`, Rejected `5`. Publication tags: Draft `0`, Proposed `1`, Validated `2`, Reviewed `3`, Published `4`, Superseded `5`, Rejected `6`.
4. Sort provenance references by the exact UTF-8 byte tuples `(source_id, source_digest, location)`. Encode `count(provenance.len)` and then `text(source_id)`, `text(source_digest)`, `text(location)` for each reference. Duplicate references remain separate entries.
5. Sort concept IDs by exact UTF-8 bytes. Encode `count(concept_ids.len)` and then `text(concept_id)` for each ID.

SHA-256 hashes the complete stream. A trusted manifest pin covers the release metadata and its declared artifact digest; the client separately hashes exact detached artifact bytes before using those bytes. A pin supplied by the same untrusted release is not authentication. Protected pin issuance, steward decisions, and any signing or key rotation contract remain Governance & Publication responsibilities.

Reference vector:

- Release ID `semantic-release-grc-2026-09-01`, contract `1.0.0`, ontology `grc-ontology-2026-09`, Authoritative/Published.
- Artifact digest `sha256:` followed by 64 `b` characters.
- One provenance reference: source ID `snapshot:grc-schema-2026-09-01`, source digest `sha256:` followed by 64 `a` characters, location `public.control_evidence.control_identifier`.
- Concept IDs `control.evidence` and `control.owner`.

Expected manifest digest: `sha256:4abb03b6f9cf0f4d0d70b5deaf84f141cce4cba71c7178e577e1f883c4f6b974`.
