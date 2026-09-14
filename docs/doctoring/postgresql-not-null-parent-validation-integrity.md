# PostgreSQL 18 NOT NULL parent-validation integrity

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-14

## Problem

The first-class PostgreSQL 18 NOT NULL family retains `pg_constraint.convalidated` and already resolves every nonzero `conparentid` to the corresponding observed parent constraint, verifies the direct `pg_inherits` parent relation, rejects detach-pending edges, checks the constrained column, inheritance flags, and parent-graph acyclicity. One compatibility rule was still missing: the resolved parent and child validation states were independently representable, but not every pair is a PostgreSQL-representable inherited relationship.

A family could therefore contain a valid parent NOT NULL constraint (`convalidated=true`) and an existing partition-child copy with `convalidated=false`. Every coordinate, parent witness, column, locality/count flag and graph invariant could otherwise be valid, allowing the impossible edge to acquire governed identity.

## Primary-source basis

PostgreSQL 18 exposes `convalidated` on `pg_constraint`, including first-class `contype='n'` NOT NULL rows. PostgreSQL 18 also supports `NOT VALID` NOT NULL constraints, so `convalidated=false` must remain representable in the general NOT NULL model rather than being rejected at construction.

The compatibility rule is asymmetric. In PostgreSQL `REL_18_STABLE`, `AdjustNotNullInheritance()` compares an existing child NOT NULL constraint with the desired inherited constraint. If the desired inherited constraint is valid while the existing child is not valid, PostgreSQL raises an error; the source comment explicitly states that the opposite direction is acceptable. Consequently:

- parent `convalidated=true`, child `convalidated=false` is not a valid inherited parent/child tuple;
- parent `convalidated=false`, child `convalidated=true` is valid and must remain accepted.

This is a source-integrity rule on the resolved parent edge, not a global prohibition on `NOT VALID` NOT NULL constraints.

## Decision

During canonicalization, after the parent constraint has resolved to an exact same-column observation, ConceptWeave checks validation compatibility. If `parent_observation.validated()` is true while the child observation is not validated, the family fails closed as `not_null_constraint_parent_validation` before hashing.

The inverse direction remains accepted. Constructor behavior for standalone or classic-inheritance NOT NULL rows is unchanged, and `convalidated` remains part of governed identity.

## Repair lineage

Review `5194758807` identified the missing compatibility rule on exact predecessor `d94a14328ebbade1f4a086ee404a15d05ef464d6`.

Executable regression commit `6cdadfa9e261f1a99f4e2afd96c9a52e7b455d17` adds two asymmetric cases to `not_null_constraint_parent_contract.rs`: a valid parent plus NOT VALID child must fail as `not_null_constraint_parent_validation`, while a NOT VALID parent plus already-valid child remains accepted.

Minimal production repair `503e88bb6a5813913da7eb502791a68c9c56c04e` adds the compatibility check only after exact parent-row and corresponding-column resolution in `canonicalize_not_null_constraints()`. Parent resolution, direct `pg_inherits` witness handling, detach-state rejection, graph acyclicity, digest framing, generic `NOT VALID` support, and all prior inheritance rules remain unchanged.

The regression is executable source evidence, not an execution claim. This runtime does not provide the repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the Product pull-request workflow. Exact-head Rust 1.98, coverage, hosted security/Product, independent review, and unchanged-head acceptance remain separate gates.

## Adapter obligation

The future PostgreSQL transport must capture `convalidated` for both the child `contype='n'` row and its resolved `conparentid` target in the same bounded catalog snapshot. It must not normalize a missing value, infer validation from `attnotnull`, or treat an impossible valid-parent/invalid-child pair as recoverable metadata. Such a pair fails closed before immutable snapshot construction.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
