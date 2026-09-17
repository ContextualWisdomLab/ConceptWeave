# PostgreSQL EXCLUDE implementation-function transform-converter integrity

**Status:** Draft source-observation decision record  
**Owner:** ConceptWeave Source Observation  
**Scope:** ordinary PostgreSQL 18 EXCLUDE `conexclop -> pg_operator.oprcode -> pg_proc.protrftypes -> pg_transform` evidence  
**PR:** `ContextualWisdomLab/ConceptWeave#46`

## Problem

The predecessor `IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot` preserves the exact nullable `pg_proc.protrftypes` selection but deliberately stops before the mutable transform implementation. PostgreSQL 18.6 stores transform implementation in `pg_transform`, independently keyed by the transformed type and procedural language. Each row carries optional `trffromsql` and `trftosql` function OIDs. Consequently, equal target-function signature, language, body, ACL, planner support, planner cost, and `protrftypes` do not prove equal conversion behavior.

`CREATE OR REPLACE TRANSFORM` can replace the transform definition for the same type/language coordinate. The selected `protrftypes` set can therefore remain unchanged while the converter function identity or converter implementation changes. Treating `protrftypes` alone as runtime-conversion identity would publish a stronger semantic claim than the source evidence supports.

## Source constraints

PostgreSQL 18.6 `pg_transform` defines:

- `trftype` as the referenced `pg_type` OID;
- `trflang` as the referenced `pg_language` OID;
- `trffromsql` as the converter used from SQL into the procedural language, with zero meaning language-default behavior;
- `trftosql` as the converter used from the procedural language back to SQL, with zero meaning language-default behavior.

PostgreSQL 18.6 `CREATE TRANSFORM` further requires the FROM-SQL converter, when present, to take one `internal` argument and return `internal`; the TO-SQL converter, when present, takes one `internal` argument and returns the transform type. Either direction may be omitted. At least one direction is present in a transform definition.

Converter function bodies remain mutable through function-definition replacement. Stable function names alone are therefore insufficient implementation evidence. ConceptWeave already uses exact `prolang`, `prosrc`, optional `probin`, and optional `prosqlbody` material for target-function definition identity; the transform-converter successor applies the same content-binding principle under a distinct digest domain and discards plaintext implementation material after construction.

## Alternatives considered

### Keep `protrftypes` as the final transform identity

Rejected. `protrftypes` records only which types elect transforms. It does not identify the `(type, language)` transform row or either converter function.

### Reconstruct and hash `CREATE TRANSFORM` DDL

Rejected. Reconstructed DDL is not the catalog source of truth and can normalize away distinctions. The owner contract should bind direct catalog facts and same-generation resolved identities.

### Copy converter function source into receipts

Rejected. Plaintext function bodies and binary paths are unnecessary downstream provenance and broaden the data-exposure surface. A domain-separated definition digest is sufficient to distinguish implementation replacement while keeping the source material at the observation boundary.

### Rewrite the existing transform-type digest

Rejected. Issued predecessor domains are immutable. Stronger evidence is an ordinary-forward successor.

## Decision

Add `IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot` over the exact transform-type predecessor.

For every governed `(constraint coordinate, key_position)` the successor:

1. repeats the exact operator and target `oprcode` function identity;
2. cross-checks source connection, connection-policy binding, extractor revision, observation time, target coordinate, operator/function binding, and target language against the retained target-function definition evidence;
3. requires the transform rows to match the exact predecessor `protrftypes` set — no row when `protrftypes` is NULL, exactly one row per selected type otherwise;
4. preserves the exact same-generation target `pg_language.lanname` corresponding to `pg_transform.trflang`;
5. preserves optional FROM-SQL and TO-SQL converter directions separately and rejects a row with neither direction;
6. resolves every nonzero converter OID to exact schema/name, one `pg_catalog.internal` argument, exact return type, converter implementation language, and a digest over exact `prosrc`/`probin`/`prosqlbody` material;
7. validates FROM-SQL return type as `pg_catalog.internal` and TO-SQL return type as the exact transform type;
8. canonicalizes transform rows by qualified type while rejecting duplicate rows rather than silently collapsing them;
9. derives a new domain-separated successor digest without changing any predecessor digest.

This is observation, not policy. ConceptWeave does not require a specific procedural language, does not require both converter directions, and does not reinterpret a zero converter OID as an error when PostgreSQL defines it as language-default behavior.

## Evidence and traceability

Finding review: `5233258873` on predecessor exact `e8a60dc4a85c8850eacecb3e88ff6b60e6a48700`.

Ordinary-forward lineage:

- structural source/compile RED: `606916ee910ac611b208aaa587bdf933159459da`;
- production successor: `6ea573c5540236fba13e993cdfa295a42977ad54`;
- public composition: `433c416ada9151b9db839d130fce444384e5519c`;
- production API refactor removing the eight-argument constructor boundary in favor of explicit definition material: `fa4802d71a9ea282a7ceaaf97a10193ca2f77045`;
- focused contract adaptation to the definition-material boundary: `a82c50dd9af22d36096b2ee5984e35a784787934`.

Production source:
`crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter.rs`

Focused contract:
`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_contract.rs`

Direct predecessor:
`IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot`

Supporting target-language evidence:
`IndexExclusionConstraintOperatorProcedureDefinitionSnapshot`

Catalog/API facts:
`pg_transform.trftype`, `pg_transform.trflang`, `pg_transform.trffromsql`, `pg_transform.trftosql`, converter `pg_proc` call signature, `prolang`, `prosrc`, `probin`, and `prosqlbody`.

## Verification boundary

The RED commit is structural: it referenced public converter-evidence types before they existed. No executed Rust compiler failure is claimed. The current execution environment does not provide the repository-pinned Rust 1.98 toolchain, so native `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and coverage remain open until one unchanged exact head is executable in the native/hosted acceptance lane.

Synthetic converter fixtures are unit-test distinguishability controls only. They are not evidence that the synthetic `(type, language)` pair exists in PostgreSQL. The live differential must use a real selected transform, if one is available in the bounded source, or explicitly preserve the NULL/no-transform state without fabricating a positive control.

## Residual risk and next review surface

This successor content-binds converter function identity and executable definition material but does not yet mirror every auxiliary `pg_proc` property of each converter function. Security mode, owner, ACL, configuration, strictness/volatility/parallel flags, planner support/cost, and other independent converter-function catalog facts can change without changing the converter signature or source definition. They must not be inferred from target-function evidence. If the product intends to claim complete converter runtime/security identity, those converter-function auxiliary facts require a separately reviewed successor rather than expansion of this frozen digest domain.

## Effects

A same selected transform set can no longer collapse transform implementations that point to different converter functions or to the same converter signature with changed implementation material. NULL transform selection remains exactly representable. Missing same-generation transform rows, duplicate rows, language drift, wrong converter direction signatures, coordinate drift, and operator/target-function binding drift fail closed.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
