# PostgreSQL ordinary EXCLUDE constraint namespace integrity

Status: source repaired; exact-head execution and PostgreSQL 18 live differential pending.

## Problem

The ordinary `contype='x'` EXCLUDE successor retained the owning relation schema as part of its stable constraint coordinate but did not independently retain resolved `pg_constraint.connamespace`. That allowed an extractor or inconsistent catalog tuple to provide a constraint namespace different from the owning relation namespace while ConceptWeave normalized both cases into the same governed identity.

This is a source-integrity defect, not a naming-policy change. PostgreSQL permits the same textual constraint name on different relations. The invariant is only that the independently read namespace of this index-backed relation constraint agrees with its owning relation namespace.

## Primary evidence

Pinned source authority is PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`. In `src/backend/catalog/index.c`, `index_constraint_create()` computes `namespaceId = RelationGetNamespace(heapRelation)` and passes that value as the namespace argument to `CreateConstraintEntry()`. The same call separately supplies the relation OID, backing index OID, key attributes, exclusion operators, inheritance state, and period state. Constraint namespace is therefore a material catalog fact even when normal DDL makes it equal to the relation namespace.

PostgreSQL 18 `pg_constraint` documents `connamespace` as the OID of the namespace containing the constraint. Constraint names are not globally or schema-wide unique, so this repair deliberately does not add a schema-wide `conname` uniqueness rule.

## Decision

Add a new domain-separated `IndexExclusionConstraintNamespaceSnapshot` rather than extending the already-issued ordinary EXCLUDE identity digest. Each predecessor ordinary EXCLUDE coordinate must have exactly one explicit resolved `connamespace -> pg_namespace.nspname` observation. The raw resolved schema name is retained in digest and provenance. A nonblank namespace that differs from the owning relation schema fails closed as `index_exclusion_constraint_namespace_state`.

Rejected alternatives:

- Derive `connamespace` from `conrelid`/relation schema: loses independent source evidence and cannot detect extractor drift.
- Add the namespace field to the existing `IndexExclusionConstraintObservation`: rewrites an issued digest contract.
- Enforce schema-wide constraint-name uniqueness: stronger than PostgreSQL and would reject legal same-name constraints on different relations.

## Traceability

Finding review: `5222226880` on predecessor `250ed695fc36f83e97a26b004d1d7728712ef2cc`.

Source/compile contract: `a4f56f9432bb0d8d368d1b36f89d59661623fd2b`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_namespace_contract.rs`.

Production successor: `b71fb962177302377a3bd913e185e2ad587025bb`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_namespace.rs`.

Public composition: `c7dcee510671be162908b75a4c7b4723e914adc0`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

No Rust RED/GREEN is claimed for these heads because the current execution host exposes no Rust toolchain. Hosted acceptance must also be reacquired on one unchanged exact head.

## Required live differential

From one bounded PostgreSQL 18 snapshot, independently read `pg_constraint.connamespace`, resolve it through `pg_namespace`, read the owning `conrelid` relation namespace separately, and verify equality for ordinary EXCLUDE constraints. Retain same-name constraints on distinct relations as a positive control. Do not synthesize the constraint namespace from relation metadata before comparison.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/catalog/index.c` (`REL_18_STABLE` pinned revision `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`).* https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`.* https://www.postgresql.org/docs/18/catalog-pg-constraint.html
