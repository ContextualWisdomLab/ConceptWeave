# PostgreSQL rendered index text: Source Observation integrity

## Problem

ConceptWeave retains two different kinds of PostgreSQL index evidence that must not share one generic string policy:

- identifiers such as index, schema, tablespace, access-method, collation, and operator-class names; and
- server-rendered text such as a partial-index predicate and a reconstructed index definition.

PostgreSQL identifiers are admitted by identifier rules and can legally contain whitespace when quoted. Server-rendered predicate/definition text is not an identifier and is not original DDL. When the field is present, an empty or whitespace-only payload is not useful source evidence; accepted material text must nevertheless remain byte-for-byte unchanged so Source Observation does not silently normalize PostgreSQL output.

The defect found on #46 was at the relation completion boundary. `IndexObservation::with_predicate()` and `IndexObservation::with_index_definition()` deliberately retained exact strings, but `RelationObservation::with_indexes()` previously admitted present whitespace-only values into governed snapshot material.

## PostgreSQL boundary

PostgreSQL 18 separates catalog structure from human-readable reconstruction/decompilation:

- `pg_index.indpred` stores the partial-index predicate as an expression tree and is null for a non-partial index.
- `pg_get_expr` decompiles an internal expression representation to text.
- `pg_get_indexdef` reconstructs a `CREATE INDEX` command from catalog state.
- `pg_indexes.indexdef` exposes an index definition as a reconstructed `CREATE INDEX` command.

Consequently, rendered predicate/index-definition text is exact source evidence for one observed PostgreSQL generation, not the original DDL and not a replacement for structured index semantics. Formatting changes across PostgreSQL versions or reconstruction paths must not be hidden by trimming or normalization inside Source Observation. Cross-version semantic alignment belongs in a later semantic layer, with the original evidence retained.

## Constraints

1. Identifier admission remains separate. A quoted whitespace-only PostgreSQL identifier can be valid and must not be rejected by `validate_nonblank` merely because presentation trimming would erase it.
2. A present rendered predicate or reconstructed definition must contain material text. Whitespace-only presence fails closed.
3. Accepted rendered text is stored exactly. Leading/trailing whitespace, line breaks, quoting, and PostgreSQL formatting are not normalized.
4. `None` stays distinct from present text. ConceptWeave must not manufacture a predicate or reconstructed definition when it was not represented by the observation.
5. The structured index model remains authoritative for semantic fields such as key/include layout, access method, operator classes, collations, raw access-method options, catalog flags, storage options, tablespace, readiness/validity/liveness, uniqueness, and null comparison. Rendered text is corroborating/recovery evidence, not a shortcut around those fields.
6. This repair does not claim current-head execution acceptance. Hosted Rust/coverage/PostgreSQL 18 differential evidence must be produced on the final unchanged head.

## Alternatives considered

### Treat rendered text as an identifier

Rejected. `pg_get_expr`/`pg_get_indexdef` output is rendered SQL/expression text, not an SQL identifier. Identifier validation would encode the wrong grammar and would blur the already explicit identifier-versus-text boundary.

### Trim before storage or digesting

Rejected. Trimming makes observed source evidence mutable and can collapse distinct PostgreSQL output. It also prevents later audit/reconstruction from seeing what the server actually returned.

### Accept any present string and rely on downstream consumers

Rejected. A governed snapshot should not admit a field that is syntactically present but carries no material evidence. The relation completion boundary is the correct place to reject incomplete index evidence before snapshot/digest construction.

### Treat reconstructed text as original DDL

Rejected. PostgreSQL documents these functions/views as reconstruction/decompilation surfaces. Original author formatting, comments, and equivalent syntactic choices are not recoverable from `pg_get_indexdef`/`pg_get_expr` alone.

## Decision

Keep exact-text setters lossless and enforce completeness only when an index is admitted into a relation:

- `IndexObservation::with_predicate()` stores the exact rendered predicate.
- `IndexObservation::with_index_definition()` stores the exact reconstructed definition.
- `RelationObservation::with_indexes()` conditionally applies `validate_nonblank(predicate, "index_predicate")` and `validate_nonblank(index_definition, "index_definition")` when those values are present.
- No trimming or normalization is performed after successful validation.

This preserves the Source Observation invariant: source evidence is exact; governed completion is fail closed; identifier and rendered-text grammars remain distinct.

## Exact lineage and traceability

- Pull request: ConceptWeave #46, `feat(observation): add v3 relation-scoped index evidence`.
- Finding review: `5260015442`.
- Behavioral RED: `e47975b8e409cb9e8937190d051ae6d69d4ab724`.
- RED contract: `crates/conceptweave-observation/tests/index_rendered_text_contract.rs`.
- Production repair: `d254d220b8e4dee5699ddc9b1e1ac6b05513cb54`.
- Production module: `crates/conceptweave-observation/src/representation_v3.rs`.
- Completion API: `RelationObservation::with_indexes()`.
- Lossless setters: `IndexObservation::with_predicate()` and `IndexObservation::with_index_definition()`.
- Source repair review: `5260119920`.
- Traceability-gap review: `5260290141` on predecessor exact head `1c57acfe82aaf31e91b6b0ccffba7effb4e266dd`.

The RED requires:

- whitespace-only present partial predicate -> `index_predicate` failure;
- whitespace-only present reconstructed index definition -> `index_definition` failure; and
- nonblank rendered values with leading/trailing whitespace -> byte-for-byte preservation.

## Risks and effects

Exact PostgreSQL-rendered text can legitimately change when PostgreSQL changes deparser/reconstruction behavior even if higher-level index semantics are equivalent. That is acceptable for immutable observation evidence: a new source rendering is new evidence. Semantic equivalence across PostgreSQL versions must be derived from the structured model or a version-aware later semantic transformation, not by rewriting historical observation text.

Rejecting whitespace-only present text is intentionally stricter than preserving arbitrary source bytes. The evidence contract treats a present predicate/definition as material rendered output. If a supported PostgreSQL version or extension can validly return whitespace-only text for these functions, that must be demonstrated with a primary-source/live differential witness before relaxing the invariant.

The repair does not change identifier semantics, index layout, storage options, digest framing outside the admitted value content, or publication authority. It prevents incomplete rendered text from entering governed snapshot material while keeping accepted server output auditable.

## Acceptance and follow-up

The source repair is not release evidence. The final exact head must independently pass repository-pinned Rust 1.98 formatting, strict Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and the PostgreSQL 18 same-generation differential.

The differential should include at least one partial index and one non-partial index, compare structured catalog evidence with `pg_get_expr`/`pg_get_indexdef`, and prove that ConceptWeave preserves the returned nonblank text exactly without treating reconstruction as original DDL. Any future change to this boundary requires a new PostgreSQL witness or an independently demonstrated buyer/semantic/security/provenance distinction.

## Primary authority

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 9.27. System information functions and operators*. https://www.postgresql.org/docs/18/functions-info.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.26. pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 53.12. pg_indexes*. https://www.postgresql.org/docs/18/view-pg-indexes.html
