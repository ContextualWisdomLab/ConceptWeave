# PostgreSQL index-constraint parentage integrity

## Decision

ConceptWeave now treats PostgreSQL key-constraint parentage as a separate immutable successor above the existing relation/index-partition topology. The new `IndexConstraintParentageSnapshot` preserves resolved `pg_constraint.conparentid` for every observed `PRIMARY KEY` and `UNIQUE` constraint without changing the frozen v3, relation-partition, or index-partition digest domains.

Finding review `5217351716` was recorded against exact #46 predecessor `8f61b6352f87a18efb113cd6fae1f54fe55adbc9`. The source regression contract was introduced first at `1534eb3d01872a0f2cdc9c14ab5dff3cc5c86a99`; the production successor followed at `6b3b4ad06b79e51cee48c261244f1b216f86fa4e` and was exported at `5d94610f70a75292aa58c2b086fedfa17bf07122`.

## Problem

The predecessor `IndexPartitionSnapshot` already required an attached child index to have its own observed key constraint when the attached parent index backs an observed key constraint. That closed constraint *presence*, but not constraint *parentage*.

Two catalog states therefore collapsed into the same governed evidence:

- the source-reachable state in which the child constraint's nonzero `pg_constraint.conparentid` resolves to the exact parent key constraint; and
- a fabricated or incompletely extracted state in which the child merely has a local key constraint with the same structural/index evidence but no constraint-parent edge.

This is material because PostgreSQL exposes `conparentid` as the parent-partition constraint coordinate, and the PostgreSQL 18 attach path installs constraint parentage independently from index parentage.

## PostgreSQL 18 authority

Authority was re-read from PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` on 2026-09-16.

`pg_constraint.conindid` identifies the index supporting a unique, primary-key, foreign-key, or exclusion constraint. `pg_constraint.conparentid` identifies the corresponding constraint on the parent partitioned table when the row is a partition constraint.

In `src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()`, PostgreSQL resolves the parent constraint with `get_relation_idx_constraint_oid(parent_table_oid, parent_index_oid)`. If that exists, it requires a child constraint resolved by the child table/index OIDs. After definition and NOT NULL checks succeed, the command executes `IndexSetParentIndex()` and then `ConstraintSetParentConstraint(child_constraint_oid, parent_constraint_oid, child_table_oid)`. Conversely, the detach path explicitly checks whether both child and parent indexes are constraint-backed before clearing constraint parentage, because PostgreSQL permits a constraint-backed child index to be a child of a non-constraint parent index.

That last source case is a required positive control: a child constraint below a non-constraint parent index must retain `conparentid = 0`; ConceptWeave must not infer parentage from attachment alone.

## Model and invariants

`IndexConstraintParentageCoordinate` resolves catalog OIDs to exact `(schema, relation, relation_kind, constraint_name)` identity. `IndexConstraintParentageObservation::root` records `conparentid = 0`; `partition` records the exact resolved parent coordinate.

`IndexConstraintParentageSnapshot` is complete over the bounded snapshot's observed primary-key and unique constraints. It rebound-validates the exact `IndexPartitionSnapshot` predecessor before issuing a new domain-separated digest. For each key constraint, the same-name backing index is located using the v3 key-constraint/backing-index contract. If that index is attached and its parent index backs an observed key constraint, the observed parent constraint must equal that exact parent coordinate. If the parent index is standalone with respect to constraints, the child constraint must remain root/local even when its index is attached.

The successor does not infer constraint subtype equality beyond the PostgreSQL attach path already established, does not copy foreign domain truth, does not use catalog OIDs as governed identity, and does not mutate issued predecessor digests.

## Alternatives

Mutating `PostgresSchemaSnapshotV3` to add `conparentid` was rejected because v3 is an issued/frozen identity domain and the source coordinate is specifically needed only after partition/index composition. Extending `IndexPartitionSnapshot` in place was rejected for the same digest-compatibility reason. Treating child-constraint presence as sufficient was rejected because it cannot represent or verify the catalog parent edge that PostgreSQL installs.

A domain-separated successor was selected because it preserves immutable predecessor identity, keeps one canonical owner for source semantics, and allows adapters to resolve transient OIDs before evidence enters governed identity.

## Risks and acceptance

The implementation has not been called GREEN in this environment. The current execution host has no repository-pinned Rust 1.98 toolchain, and #46 still has no repository-owned pull-request workflow run on its moving head. The source contract is therefore a source/compile RED followed by a causal repair, not executed RED/GREEN evidence.

Before authoritative acceptance, one unchanged exact #46 head must pass Rust 1.98 formatting, strict all-target Clippy, the new parentage contract plus every retained Source Observation test, workspace/doc tests, release build, rustdoc/coverage requirements, and applicable hosted Product/security/review gates. A live PostgreSQL 18 differential must additionally inspect `pg_constraint.conparentid` after successful `ALTER INDEX ... ATTACH PARTITION`, prove missing/incorrect parentage is rejected by ConceptWeave, and preserve the valid constraint-child/non-constraint-parent `conparentid = 0` case.

## Traceability

- Finding review: #46 review `5217351716` on `8f61b6352f87a18efb113cd6fae1f54fe55adbc9`.
- Source/compile RED contract: `1534eb3d01872a0f2cdc9c14ab5dff3cc5c86a99`, `crates/conceptweave-relation-partition/tests/index_constraint_parentage_contract.rs`.
- Production successor: `6b3b4ad06b79e51cee48c261244f1b216f86fa4e`, `crates/conceptweave-relation-partition/src/index_constraint_parentage.rs`.
- Public export: `5d94610f70a75292aa58c2b086fedfa17bf07122`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- PostgreSQL source authority: `postgres/postgres@REL_18_STABLE`, exact `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/commands/tablecmds.c::ATExecAttachPartitionIdx()`.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: tablecmds.c* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/tablecmds.c
