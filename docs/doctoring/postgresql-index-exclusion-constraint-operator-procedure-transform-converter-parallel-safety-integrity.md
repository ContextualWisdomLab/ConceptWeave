# PostgreSQL transform-converter parallel-safety integrity

## Decision

ConceptWeave Source Observation preserves the raw same-row `pg_proc.proparallel` value for every nonzero FROM SQL / TO SQL converter selected by the governed ordinary-`EXCLUDE` transform chain. The successor is observational: `s`, `r`, and `u` are all representable. ConceptWeave does not reinterpret the catalog into a local rule that every transform converter must be `PARALLEL SAFE`.

The parallel-safety successor is keyed by the exact predecessor tuple `(constraint coordinate, key position, transform type, direction)` and repeats the converter schema/function identity. Missing or extra coordinates, duplicate observations, converter-binding drift, zero positions, blank converter identifiers, and catalog discriminators outside `s|r|u` fail closed. The successor digest is domain-separated from converter volatility and includes the raw `proparallel` byte.

## Why this is an independent fact

PostgreSQL stores routine parallel safety separately from volatility and other `pg_proc` attributes. `CREATE FUNCTION` exposes `PARALLEL UNSAFE`, `PARALLEL RESTRICTED`, and `PARALLEL SAFE` independently from `IMMUTABLE`/`STABLE`/`VOLATILE`, strictness, leakproof state, security context, support functions, and cost. PostgreSQL's parallel-safety documentation further states that the planner cannot derive the correct label for arbitrary user-defined functions; the declaration must be supplied by the function owner and incorrect labels can produce errors or wrong answers.

A transform binds conversion functions to a type/direction contract, but that binding does not make `proparallel` part of the transform coordinate. Consequently, converter definition, ownership, ACL, local configuration, security-definer state, leakproof state, strictness, or volatility cannot substitute for observing raw parallel-safety state from the same exact-generation `pg_proc` row.

## Source and contract traceability

- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety.rs`
- public composition: `crates/conceptweave-relation-partition/src/index_partition.rs`
- executable contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_contract.rs`
- finding review: `5250289810`
- RED: `0fc0f567386d39b848444879a41c4877e9f996b6`
- production implementation: `66846c34d668202a6f8fbbdfb3658fba908aec0d`
- public-composition successor: `6373c77652f63dae874e2b4bdb881b54f5680bfe`

The focused contract preserves raw `proparallel`, distinguishes `s/r/u` in the successor digest, requires exact FROM/TO completeness and converter binding, rejects invalid/duplicate/extra observations, rejects blank identifiers and zero positions, keeps exact receipt lookup, and rechecks quoted qualified-type provenance collision safety.

## Acceptance boundary

Source shape is not acceptance. Repository-pinned Rust 1.98 formatting, strict Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production coverage, and the bounded PostgreSQL 18 differential must execute on an exact descendant head before this successor can be called GREEN. The live differential must resolve each converter to the same exact-generation `pg_proc` row used by the rest of the converter chain and capture raw `proparallel` independently of `provolatile`.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: Parallel safety*. https://www.postgresql.org/docs/18/parallel-safety.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: Function optimization information*. https://www.postgresql.org/docs/18/xfunc-optimization.html
