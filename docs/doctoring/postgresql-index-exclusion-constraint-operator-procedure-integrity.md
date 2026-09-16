# PostgreSQL ordinary EXCLUDE operator implementation-function integrity

## Problem

ConceptWeave already preserves the ordered `pg_constraint.conexclop` vector and the backing index's resolved exclusion operator, procedure, and strategy tuple. That predecessor verifies operator identity and procedure argument-type shape, but argument types are not operator implementation identity. A faulty source adapter can substitute a different `pg_proc` function with the same two argument types and still produce an apparently coherent exclusion tuple.

For governed semantic evidence this is a cross-catalog integrity gap. The implementation function must come from the exact operator row, not from a same-typed function guess or from the already-retained backing procedure itself.

## PostgreSQL 18 authority

PostgreSQL 18 `pg_operator.oprcode` references the `pg_proc` function that implements the operator. In PostgreSQL executor/index metadata, `IndexInfo::ii_ExclusionProcs` is explicitly the array of underlying function OIDs for the corresponding exclusion operators. Therefore an ordinary EXCLUDE capture has two independently sourced facts that must agree after resolution:

1. the implementation procedure reached from each exact `conexclop` operator through `pg_operator.oprcode`; and
2. the procedure already retained in the backing index exclusion-semantics tuple for the same key position.

The source adapter must resolve both OID paths independently to stable schema/name/argument-type signatures before comparison. OIDs remain capture-local join coordinates and are not governed identity.

Primary references:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_operator*. `oprcode` is the function that implements the operator.
- PostgreSQL source, `src/include/nodes/execnodes.h`, `IndexInfo`: `ii_ExclusionProcs` is documented as the underlying function OID array for `ii_ExclusionOps`.

## Decision

Add `IndexExclusionConstraintOperatorProcedureSnapshot` as a domain-separated successor over the existing ordinary-EXCLUDE operator predecessor. Do not change any issued predecessor digest.

`IndexExclusionConstraintOperatorSnapshot` retains the backing exclusion procedure for each exact constraint/key position as immutable metadata derived from the already-rebound `IndexExclusionSemanticsSnapshot`. This metadata is not added to the existing operator digest calculation. The new successor then requires one explicit independently resolved `pg_operator.oprcode -> pg_proc` observation for every governed `conexclop` position and fails closed unless:

- the constraint coordinate and one-based key position are complete and unique;
- the repeated operator signature equals the exact predecessor operator at that position; and
- the independently resolved operator procedure equals the retained backing exclusion procedure signature.

A same-typed but different procedure is rejected. Function-name heuristics, operator-name allowlists, copying the predecessor procedure into the new observation, and rewriting the existing operator digest were rejected because they do not prove cross-catalog agreement.

## Traceability

- Finding review: `5226950243` on exact predecessor `84e287842d094755caa75272fae73111125fd3e7`.
- Structural source/compile RED: `2f7e37da0362b59d2875e10dc51ed91935d434ed`. The first contract referenced the new successor before production composition existed; no executed compiler failure is claimed.
- Backing procedure metadata repair: `c05f681081412b12daca0e9b88479c0058878f27` in `index_exclusion_constraint_operator.rs`.
- Production successor: `9ff1b49b4188ea98f223d0d45075dbdb1490b813` in `index_exclusion_constraint_operator_procedure.rs`.
- Public composition: `b6776db96a677f39aea5fe8f86d5186fb9ee03a8` in `index_partition.rs`.
- Initial focused positive/same-typed-negative contract: `2f7e37da0362b59d2875e10dc51ed91935d434ed` temporarily proved the source-RED in the neighboring commutator test file.
- Edge/provenance expansion: `ae24b046301199fabfb3528300b258d961dc89ba` added operator-binding drift, missing and duplicate position evidence, zero-position rejection, canonical receipt location, and unknown-receipt coverage.
- Test ownership repair: `7d9c9bf2bda0b4db8dc35e2fc111f8e6748b07cb` restores the commutator contract to its original single-purpose scope; `6fb013b53ac95f02a96dacd05661016df0ed03b2` moves the complete operator-procedure contract into `index_exclusion_constraint_operator_procedure_contract.rs` so the new bounded context owns its own tests.

## Acceptance

Source repair is not GREEN evidence. One unchanged exact head must pass the repository-pinned Rust 1.98 formatting, strict Clippy, focused and retained workspace/doc tests, release build, rustdoc/coverage, and applicable hosted gates.

The PostgreSQL 18 live differential must resolve each `conexclop` OID to its `pg_operator` row, read `oprcode`, resolve that OID independently through `pg_proc`, and compare the resulting stable procedure signature with the backing exclusion-procedure path for the same exact `conindid` index/key position. A negative control must use a distinct procedure with the same argument types so type-shape equality cannot masquerade as implementation identity.
