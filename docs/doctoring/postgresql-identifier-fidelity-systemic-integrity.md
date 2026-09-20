# PostgreSQL identifier fidelity: systemic Source Observation boundary

## Problem

ConceptWeave preserves exact PostgreSQL catalog identifiers. Generic presentation validation is not sufficient for this boundary because `value.trim().is_empty()` rejects a legal quoted identifier whose exact catalog value consists only of whitespace. PostgreSQL 18 §4.1.1 permits whitespace inside delimited identifiers and forbids the NUL character.

The invariant is therefore field-specific: identifier values preserve source bytes, reject zero-length/NUL states, and are not trimmed, case-folded, normalized, or reinterpreted. Rendered type text, CHECK definitions, expressions, extractor revisions, option text, and other non-identifier text retain their existing nonblank policy.

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
- Legacy-model structural RED `3136e841ed925deffa904f578acc4e46c80bb763` adds coverage for v2-era column/table/constraint/foreign-key/location identifier coordinates while keeping rendered data-type and CHECK text as negative controls.
- Production repair `b4068f5e78de4f5e76ca1eef70f17cce68b83c1c` routes those `model.rs` identifier-bearing fields through the existing exact PostgreSQL identifier predicate. `validate_nonblank()` itself is unchanged, so non-identifier text keeps the prior trim-based policy.
- Review `5257904564` records the inspected legacy-model finding and explicitly does not backdate review evidence ahead of the RED/fix commits.
- V3 catalog structural RED `5ed597aab1e0770e9702e766beeb78c3aa03c84d` extends the same rule through qualified type/collation/operator-class, v3 column/relation/domain/enum/index/location, domain-constraint, and simple index-column coordinates. `IndexTablespace` contract correction `fe2d3f01efae5fdad03b39a785ffd096f5d91fdd` likewise treats tablespace names as PostgreSQL identifiers rather than presentation text.
- V3 production repair `1f2c4470849dacbc06632c6fe87e59a9f6d9df87` routes those identifier-bearing `representation_v3.rs` fields through the PostgreSQL identifier boundary while leaving rendered expressions/definitions and other non-identifier text on their existing policies. Ordinary-forward recovery `9aa5034c21328fd1dbe0b6d4d9e614ef6079d098` preserves that focused tree after an earlier broad file write; no history rewrite was used.
- Exact successor RED `31639335a1c65583997917133bfe1f9cf6e6d4aa` isolates the residual index access-method admission defect: quoted-whitespace access-method identifiers must survive byte-for-byte while empty and NUL-bearing identifiers fail closed.
- Production repair `ae86c5a885d5f5a02a6f19957485d397f44f1e3c` replaces trim-based access-method validation in `RelationObservation::with_indexes()` with the shared PostgreSQL identifier predicate while still rejecting missing access-method evidence. Current-head review `5259712533` verifies that the diff is confined to that admission block and rustdoc and does not weaken generic nonblank text validation.

## Current status

The known systemic PostgreSQL identifier admission defect is now **source-repaired across the shared legacy model and the v3 representation, including index access-method identity**, but this is not yet exact-head GREEN. Identifier coordinates use the PostgreSQL-specific empty/NUL boundary; rendered type text, CHECK definitions, expressions, extractor revisions, option text, and other non-identifier payloads retain their existing field-specific policies.

The exact accepted head must independently pass repository-pinned Rust 1.98 formatting, strict Clippy, retained/workspace/doc tests, release/rustdoc, owned statement/branch/edge coverage, and a PostgreSQL 18 same-generation differential exercising quoted-whitespace identifiers and NUL rejection, including the access-method witness from `31639335...`. The current source-level RED/fix lineage is not a claim that a failing or passing workflow was observed, and no predecessor execution evidence transfers after source or documentation movement.

A future identifier change is justified only by a newly demonstrated uncovered identifier-bearing field or contradictory PostgreSQL semantics. Otherwise the next action is exact-head acceptance and ordinary downstream adoption rather than manufacturing another semantic successor.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 4.1.1 Identifiers and key words*. https://www.postgresql.org/docs/18/sql-syntax-lexical.html
