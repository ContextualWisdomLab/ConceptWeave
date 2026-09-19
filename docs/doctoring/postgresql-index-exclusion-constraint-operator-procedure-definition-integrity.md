# PostgreSQL ordinary-EXCLUDE implementation-definition integrity

## Decision

ConceptWeave must not treat an exact `pg_operator.oprcode -> pg_proc` signature as sufficient evidence of the executable implementation behind an ordinary PostgreSQL `EXCLUDE` operator. The governed successor binds each exact constraint/key position to the already-governed operator and function, independently resolves the implementation language, consumes the same-row `pg_proc.prosrc`, optional `probin`, and optional `prosqlbody`, and derives a domain-separated definition digest.

The plaintext implementation material is not retained in the observation or receipt. The content digest is evidence identity, not a claim that PostgreSQL internal `pg_node_tree` text is a stable cross-major semantic representation.

## Problem

The predecessor chain already binds the exact `conexclop` operator to its `oprcode` function and preserves result/cardinality, strictness, volatility, parallel safety, routine kind, security-definer mode, and leakproofness. `QualifiedProcedureSignature`, however, is intentionally a stable schema/name/input-type identity. It does not include the executable definition.

PostgreSQL 18 permits `CREATE OR REPLACE FUNCTION` to replace an existing function definition without changing the function name or input argument types, and without breaking objects that refer to that function. The return type also remains fixed. An ordinary-EXCLUDE operator can therefore keep the same governed operator/function identity while the function body or dynamically loaded implementation changes.

`pg_proc` stores the implementation language in `prolang`. The implementation material is language-specific: `prosrc` contains source text for SQL string-body and procedural-language functions, or a C/internal link symbol for compiled functions; `probin` supplies additional invocation information and is used for dynamically loaded C functions; `prosqlbody` contains the pre-parsed body for SQL-standard function notation. Omitting these facts allows materially different executable definitions to collapse to one semantic evidence identity.

## Constraints

- The source capture must resolve `prolang` to the exact observed `pg_language.lanname` for the same `pg_proc` row.
- `prosrc`, `probin`, and `prosqlbody` must come from that same row and same bounded source generation as the retained operator/procedure evidence.
- `NULL` and an empty string are distinct for optional `probin` and `prosqlbody`; the digest encoding preserves option presence explicitly.
- Empty `prosrc` is valid because SQL-standard `sql_body` notation normally leaves `prosrc` unused while `prosqlbody` carries the parsed body.
- `prosqlbody` is PostgreSQL internal `pg_node_tree` source evidence. Its exact text is bound to the observed PostgreSQL/extractor generation; ConceptWeave does not declare cross-major semantic equivalence from equal or unequal node-tree text.
- Plaintext function source, object-file paths, and pre-parsed body text are discarded after the domain-separated material digest is derived. Receipts expose only the resolved language name and digest.
- No predecessor digest domain is rewritten.

## Alternatives

### Continue using function signature plus auxiliary flags

Rejected. `CREATE OR REPLACE FUNCTION` explicitly allows definition replacement while retaining function identity and dependent references, so this would preserve a known semantic collision.

### Persist raw `prosrc` / `probin` / `prosqlbody` in receipts

Rejected. Raw function bodies and binary paths are not required for downstream provenance and can contain proprietary or operationally sensitive implementation detail. Content identity is sufficient for immutable evidence comparison.

### Hash `pg_get_functiondef()` output

Rejected as the canonical source contract for this layer. It is a reconstructed SQL representation whose formatting and decompilation behavior are not the raw catalog tuple being governed. It remains useful for human diagnosis, but the owner contract should bind the independently observed source fields directly.

## Implementation and invariants

`IndexExclusionConstraintOperatorProcedureDefinitionMaterial` consumes resolved language plus exact `prosrc`/`probin`/`prosqlbody` and stores only a SHA-256 digest in a dedicated material domain. `IndexExclusionConstraintOperatorProcedureDefinitionObservation` binds that material identity to one exact ordinary-EXCLUDE coordinate/key position and the repeated operator/function signatures. `IndexExclusionConstraintOperatorProcedureDefinitionSnapshot` derives completeness from `IndexExclusionConstraintOperatorProcedureLeakproofSnapshot`, rejects duplicate/missing coordinates and operator/function drift, and frames all observations under a separate successor digest domain.

The focused contract requires definition-digest separation for changes in `prosrc`, `probin`, `prosqlbody`, or language identity; rejects blank language identity, binding drift, missing/duplicate evidence, zero positions, and unknown receipt coordinates; and checks public composition. These are source-level contracts until one unchanged exact head receives the repository-pinned Rust 1.98 native and hosted GREEN evidence.

## Traceability

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5230742538`
- Structural source/compile RED commit: `f4f652e3f9385b0495ff2ae40a69a7733c971d11`
- Initial production successor: `cc7e48e1d823634bcdb74c906f4deea27819a355`
- Public composition: `1191dc74d5878825f5db6949bbf5f1f222514cae`
- No-lint-suppression production refinement: `499e5336609a52145ef85a1d9d63effdaab6bab2`
- Focused contract refinement: `3dd47a11ed652ff3eb2bb14daf00bd4c23bac959`
- Production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_definition.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_definition_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureLeakproofSnapshot`
- Catalog facts: `pg_proc.prolang`, `pg_proc.prosrc`, `pg_proc.probin`, `pg_proc.prosqlbody`, resolved `pg_language.lanname`

## References

PostgreSQL Global Development Group. (2026). *CREATE FUNCTION (PostgreSQL 18.6 documentation).* https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *pg_proc (PostgreSQL 18.6 documentation).* https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *System catalogs (PostgreSQL 18.6 documentation).* https://www.postgresql.org/docs/18/catalogs.html
