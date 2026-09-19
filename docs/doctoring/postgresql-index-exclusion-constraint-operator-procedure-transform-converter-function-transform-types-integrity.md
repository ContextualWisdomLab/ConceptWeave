# PostgreSQL transform-converter function transform-selection integrity

## Decision

ConceptWeave Source Observation preserves each selected transform converter function's own raw `pg_proc.protrftypes` state independently from the `pg_transform` row that binds the converter. NULL remains distinct from a nonempty resolved qualified-type set.

This is function-call conversion evidence. It does not recursively copy the selected types' `pg_transform` rows and it does not replace the existing `(trftype, trflang)` converter binding authority.

## Primary authority

PostgreSQL 18 `pg_proc` defines `protrftypes` as the OID array of argument/result data types for which a function call applies transforms, NULL when none are selected.

PostgreSQL 18 `CREATE FUNCTION` defines `TRANSFORM FOR TYPE` as the list of transforms a call to the function applies. The documentation explicitly notes that language implementations may otherwise fall back to language-specific default conversion behavior. `CREATE OR REPLACE FUNCTION` preserves the function object and dependents while reassigning function properties that are specified or implied, so transform selection is mutable source state rather than part of the function's overload identity.

Primary references:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

## Invariant

`IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot` is an ordinary-forward successor of the converter argument-name snapshot and must:

- cover exactly the predecessor `(constraint, key_position, transform_type, direction)` inventory;
- preserve the exact converter schema/function binding;
- preserve NULL separately from a nonempty selected-type set;
- resolve each stored OID to an exact qualified type in the same observation generation;
- canonicalize set ordering without collapsing duplicates or inventing an empty-array representation that PostgreSQL does not store;
- domain-separate the successor digest and encode each qualified type component exactly;
- retain collision-safe component-wise percent encoding for the transform type that identifies the converter direction;
- leave the selected types' actual `pg_transform` rows with the existing transform binding surface rather than duplicating foreign source truth here.

## Traceability

- finding review: `5252233297` on exact head `3dc7c81c8432f370b54aeac8375482d88b99a0cc`;
- RED contract: `20be74cacfce08778d00cb767bf05379eeba6626`;
- production observation/snapshot/receipt: `3e97370dc221738b0d408ed2a19be32f01ebff5c`;
- public composition: `303677ee2e1807aae883f0b752b897dbf2c23f01`.

The immediate predecessors are preserved ordinary-forward: converter argument-name public head `bbcf388dfd01323da86b25df300ba112be4dc2cb` and its documentation/currentization through `3dc7c81c8432f370b54aeac8375482d88b99a0cc`.

## Acceptance

Source movement is not acceptance. The final exact head still needs repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 live differential.

The live differential must read converter `protrftypes` from the same exact-generation `pg_proc` row used for the rest of that converter's function evidence, resolve nonzero OIDs to qualified types in that same source snapshot, and preserve NULL rather than replacing it with an empty list. Reconstructing the set from implementation language, return type, source text, or the target function's own `protrftypes` is a capture failure.