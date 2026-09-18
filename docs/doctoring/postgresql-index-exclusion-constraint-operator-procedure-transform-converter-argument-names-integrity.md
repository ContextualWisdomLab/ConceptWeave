# PostgreSQL transform-converter argument-name integrity

## Decision

ConceptWeave Source Observation preserves raw `pg_proc.proargnames` for every selected transform-converter direction. `NULL` remains distinct from a populated array, and empty strings inside a populated array remain positional unnamed-argument sentinels rather than being normalized away.

This is observation evidence, not a fresh `CREATE TRANSFORM` admission rule.

## Primary authority

PostgreSQL 18 `pg_proc` documents `proargnames` as a `text[]` whose positions correspond to `proallargtypes`. Unnamed arguments are represented by empty strings; the whole field is `NULL` when none of the arguments are named. The adjacent `proargmodes` vector uses the same all-argument positional space.

PostgreSQL 18 `CREATE FUNCTION` also makes this state lifecycle-relevant: `CREATE OR REPLACE FUNCTION` cannot rename an already named input parameter, but it can add names to input parameters that previously had none. The function remains the same object and dependent objects need not be recreated. A transform converter can therefore retain its binding while its live `proargnames` changes from `NULL` to a populated array.

Primary references:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

## Invariant

`IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot` is an ordinary-forward successor of the argument-mode snapshot and must:

- cover exactly the predecessor `(constraint, key_position, transform_type, direction)` inventory;
- preserve the exact converter schema/function binding;
- preserve `None` separately from a populated name vector;
- preserve empty strings at unnamed positions when at least one position is named;
- reject an empty populated vector or an all-empty populated vector because PostgreSQL represents the latter as `NULL`;
- require a populated vector to have the same positional length as the predecessor all-argument shape: one position when `proargmodes` is `NULL`, otherwise the exact `proargmodes` vector length;
- domain-separate the successor digest and encode each name exactly, without trimming, case folding, locale normalization, or signature-derived substitution;
- retain collision-safe component-wise percent encoding for qualified transform-type provenance.

Quoted SQL identifiers can contain whitespace, so nonempty argument names are not trimmed. The empty string alone is the catalog sentinel for an unnamed position.

## Traceability

- finding review: `5252092881` on exact head `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`;
- RED contract: `2ccf80dd19f7710d14512f2b184d8551142ad4d5`;
- production observation/snapshot/receipt: `653aa476578cb3e75bf19103c37ffec4e17e9a5e`;
- public composition: `bbcf388dfd01323da86b25df300ba112be4dc2cb`.

The immediately preceding concurrent `proargmodes` work is adopted, not overwritten: RED `6ecc6418fce88b3aaf1519e7c422b2a1583b5407`, production `39602ea78c4bbff856c6274052918c4c0541e9df`, public composition `a3c61e33fbd84c86e9167d17377072df3e0e6ec0`, doctoring head `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`.

## Acceptance

Source repair is not acceptance. The final exact head still needs repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 live differential.

The live differential must read `proargmodes` and `proargnames` from the same exact-generation `pg_proc` row. It must not infer argument names from SQL text, function signatures, application metadata, or another catalog generation.