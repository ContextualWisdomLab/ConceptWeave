# PostgreSQL ordinary EXCLUDE operator procedure-kind integrity

Status: Proposed source repair; exact-head native/hosted validation and PostgreSQL 18 live differential remain open.

## Problem

The ordinary-EXCLUDE evidence chain already binds each exact `pg_constraint.conexclop` position to its `pg_operator` row and exact `oprcode -> pg_proc` implementation routine. It separately preserves binary operator kind, Boolean result identity, scalar cardinality, strictness, volatility, and parallel-safety state. The stable `QualifiedProcedureSignature`, however, contains namespace, routine name, and input argument types; it does not preserve `pg_proc.prokind`.

PostgreSQL 18.6 stores functions, procedures, aggregates, and window functions in `pg_proc`, with `prokind='f'`, `'p'`, `'a'`, and `'w'` respectively. `CREATE OPERATOR` requires the implementation routine to have been created with `CREATE FUNCTION`; although its syntax accepts either the keyword `FUNCTION` or the historical keyword `PROCEDURE`, the referenced routine must still be a function. A malformed, synthetic, or incorrectly adapted capture could therefore present a non-function `pg_proc` row with a normalized procedure signature that otherwise resembles the governed operator implementation and collapse that invalid source state into the same semantic identity.

## Constraints

This repair stays inside ConceptWeave Source Observation. It does not copy PostgreSQL domain truth into another owner, change an issued predecessor digest domain, or infer routine kind from name, argument types, return type, `proretset`, `proisstrict`, `provolatile`, `proparallel`, language, or operator-family membership. Raw OIDs remain capture-local join coordinates; governed identity uses stable qualified signatures plus the independently observed catalog discriminator.

The rule is not a new product policy. It mirrors PostgreSQL's own operator-definition invariant: the implementation object must be a normal function. `p`, `a`, and `w` are retained as negative distinguishability fixtures, not claims that PostgreSQL admits those routine kinds as operator implementations.

## Alternatives considered

Keeping the existing `QualifiedProcedureSignature` alone was rejected because it describes call identity without proving `pg_proc.prokind`. Inferring function kind from `prorettype`, `proretset=false`, strictness, volatility, parallel safety, or the fact that `pg_operator.oprcode` references the row was rejected because those are separate catalog facts and would turn an adapter assumption into semantic authority. Expanding the existing procedure-signature digest was rejected because it would rewrite an issued predecessor identity. A new successor layer preserves immutability and makes the new fact independently auditable.

## Decision

`IndexExclusionConstraintOperatorProcedureKindSnapshot` is layered over `IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot`. For every exact constraint/key position it requires exactly one `IndexExclusionConstraintOperatorProcedureKindObservation` bound to the predecessor's same stable operator and exact implementation routine. The raw discriminator must be `f`. `p`, `a`, `w`, unknown values, missing/duplicate coordinates, zero positions, or operator/procedure binding drift fail closed.

The new domain-separated digest includes the predecessor digest, exact constraint coordinate, one-based key position, stable operator signature, stable routine signature, and raw `prokind`. Existing operator, commutator, implementation-procedure, result, operator-kind, scalar, strictness, volatility, parallel-safety, backing-index, and Source Observation digest domains remain unchanged. Provenance receipts inherit source key, immutable connection-policy binding, extractor revision, and observation timestamp from the exact predecessor generation.

## Evidence and edge cases

The focused contract covers a valid `f` observation with provenance, rejection of `p/a/w` and unknown discriminators, operator and implementation-routine binding drift, missing evidence, duplicate coordinates, zero position, unknown receipt coordinates, and public composition. The structural RED commit referenced the new public types before production existed; no executed compiler failure is claimed because this execution environment does not provide the repository-pinned Rust 1.98 toolchain.

Exact-head GREEN still requires Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc/test/edge-case coverage, and applicable hosted quality/security/dependency/review gates on one unchanged head. Any head movement resets that acceptance evidence.

## PostgreSQL 18 live differential

For each exact ordinary-EXCLUDE `conexclop` position, the live adapter must resolve the source `pg_operator` row, independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`, follow `oprcode` to the exact `pg_proc` row, and independently read `prokind`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`. `prokind` must come from that exact row and equal `f`; it may not be reconstructed from the normalized signature or any other routine/operator property. Existing operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in the same v3 source-content generation.

## Risk, effect, and follow-up

The repair increases evidence width by one byte-scale discriminator per governed operator position plus digest/provenance framing; it does not create a new query hot path. The principal remaining risk is adapter conflation: a transport that copies an expected `f` instead of reading `pg_proc.prokind` would satisfy the value shape without proving the source fact. The bounded PostgreSQL 18 differential must therefore demonstrate that the discriminator is independently selected from the exact joined `pg_proc` row, including a negative synthetic/corrupt-capture control at the evidence boundary.

After the canonical `.github` workflow prerequisite and #35 acceptance are repaired, one unchanged #46 head must obtain native and hosted terminal GREEN, then pass the bounded live differential before this delta can be adopted completely and non-force into #45 and later #6.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5229806144`
- structural RED: `7f9d964d27abbfea05b8cbd4761246e8955db1c9`
- production successor: `c9f4d8e719c9c5a4e1d82c150c8b6e942c7fb0a1`
- public composition: `616ce311672f2ab9bd57e2cdbdd6ba1eb9b5e758`
- focused edge/provenance contract: `e7d10cdb3b0877146207e6881099c2d5b008493d`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_kind.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_kind_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot`
- source catalog fact: `pg_proc.prokind`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE OPERATOR*. https://www.postgresql.org/docs/18/sql-createoperator.html
