# PostgreSQL identifier fidelity: systemic Source Observation boundary

## Problem

ConceptWeave preserves exact PostgreSQL catalog identifiers. Generic presentation validation is not sufficient for this boundary because `value.trim().is_empty()` rejects a legal quoted identifier whose exact catalog value consists only of whitespace. PostgreSQL 18 §4.1.1 permits whitespace inside delimited identifiers and forbids the NUL character.

The invariant is therefore field-specific: identifier values preserve source bytes, reject zero-length/NUL states, and are not trimmed, case-folded, normalized, or reinterpreted. Rendered type text, CHECK definitions, expressions, extractor revisions, and other non-identifier text retain their existing nonblank policy.

## Exact lineage

- Finding review `5257069180` on predecessor `427587e8be39d38240e57a2ebcc9cdaf291f6ebe` identified the shared legacy/v3 trim-based boundary.
- Systemic structural RED `a1638573b9d99f78c7fcd211fc93559d86d4f93a` covers qualified type, collation, operator-class, and v3 column identifiers.
- Bounded column-identity RED `9395d141c70f074a1b60ff1c5d26097973db768a` and repair `c2b247b24c45f9e45c90aad7b4fb68e296d0225e` proved the field-specific rule without globally weakening text validation.
- Scope review `5257371727` found that separately exported prefixed families were missing from that contract. RED `3291d594da042d8497b1cc17493fc180f213f397` added `ColumnCollationObservation` and `ColumnExpressionObservation`, including the negative control that whitespace-only rendered expression text stays invalid.
- Fresh exact-head review `5257607599` on `fe70b0c5077fb436c69d75e72502f12e4e209f80` reconfirmed the causal split between identifier coordinates and rendered expression text.
- Shared identifier admission became crate-reusable at `7de4d9b9072869b562ad34719241a774a170e903` without changing its semantics.
- `ColumnCollationObservation` coordinates were repaired at `7961c0229938048442c7b1839230f136bb59fd04`.
- `ColumnExpressionObservation` coordinates were repaired at `a8ad7cf1430a13aaae33bdeff31b29885eaed06f`; its expression payload still uses generic nonblank validation.
- Generated-column coordinates were repaired at `d295661d0360b88a0da24ec1129b6a9aaf8b72e9`.
- Constraint-timing coordinates were repaired at `943518644b28d6255453ce574c90ddcef22a21c6`.
- Temporal-constraint and exclusion-operator identifiers were repaired at `4bbe2ab1438a0949a7e994e3150c0b413737c74d`.
- Dedicated NOT NULL structural RED `7efd69164df643b2bf325207df70c93ffb6318dc` covers local and parent schema/relation/constraint/column coordinates. Production repair `6b032de91b034f9ab76541eeb1a02150dad1d1d1` applies the same identifier admission to local, parent-constraint, and partition-parent coordinates.
- Retained catalog-family regression contract `263091310809b223881c2a54e1ee4f5fb761dbe5` covers generated-column, timing, and temporal-constraint coordinates after their repair. It is retained regression evidence, not a claim that an executed pre-fix failure was observed.

## Current status

The repair is materially broader but still not systemic GREEN. `model.rs` and `representation_v3.rs` retain identifier-bearing call sites that use generic trim-based `validate_nonblank()`. The remaining repair must separate those identifier call sites from non-identifier text call sites rather than weakening the generic text validator.

The exact accepted head must independently pass repository-pinned Rust 1.98 formatting, strict Clippy, retained/workspace/doc tests, release/rustdoc, owned statement/branch/edge coverage, and a PostgreSQL 18 same-generation differential exercising quoted-whitespace identifiers and NUL rejection. No predecessor execution evidence transfers after source or documentation movement.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 4.1.1 Identifiers and key words*. https://www.postgresql.org/docs/18/sql-syntax-lexical.html
