# PostgreSQL 18 index-partition child-constraint integrity

Status: Proposed source repair; exact-head execution acceptance pending.

## Problem

`IndexPartitionSnapshot` already composes relation-partition ownership, index-definition equivalence, local child attachment, child validity, and the foreign-table exception. It did not compose the already-observed key-constraint/backing-index relationship across an attached index edge.

That omission allowed a governed topology in which a partitioned parent index backs an observed `PRIMARY KEY` or `UNIQUE` constraint while the attached compatible child index is standalone and backs no child constraint. PostgreSQL 18 refuses that attachment. In `ATExecAttachPartitionIdx()`, after `CompareIndexInfo()` succeeds, PostgreSQL resolves the parent index's constraint with `get_relation_idx_constraint_oid()`. When the parent has one, it requires the child index to resolve its own constraint before `IndexSetParentIndex()` and `ConstraintSetParentConstraint()` are executed; absence raises `ERRCODE_INVALID_OBJECT_DEFINITION`.

## Authority and traceability

Primary authority is PostgreSQL `REL_18_STABLE`, re-read on 2026-09-16 KST. At the current source path `src/backend/commands/tablecmds.c`, `ATExecAttachPartitionIdx()` checks index-definition equivalence at lines 20365-20382 and the parent/child constraint requirement at lines 20383-20415.

ConceptWeave owner seams:

- PR #46 finding review: `5216568917`, anchored to exact `74ef156e0eefe250ec4d74bcba68ab60e256c90a`.
- Source regression contract: `ead7376a2c497c354a544d76ce80325bbe037524`, `crates/conceptweave-relation-partition/tests/index_partition_constraint_backing_contract.rs`.
- Minimal production repair: `72d7000e9130fa4b51d4b9b03a801a37d3b72351`, `crates/conceptweave-relation-partition/src/index_partition_base.rs`.
- Existing digest domain `conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.v1` remains unchanged.

## Decision

For an observed attached index edge:

1. Existing PostgreSQL-equivalence checks remain unchanged.
2. If the parent index is already bound by the v3 observation contract to an observed primary-key or unique constraint, the child index must also be bound to an observed primary-key or unique constraint before the edge is admitted.
3. Standalone parent indexes do not cause ConceptWeave to invent a child constraint.
4. This repair does not require constraint names to match between parent and child. Each relation continues to bind its own constraint to its own backing index under the v3 owner invariant.
5. Primary/unique constraint subtype equality is not introduced here because the upstream attach path first requires child constraint presence and then delegates the parent relationship to `ConstraintSetParentConstraint()`; this repair does not claim a stronger rule than the source evidence established in this lane.

The implementation uses relation-local observed constraint/backing-index evidence and only composes it across the already-observed attached index edge. It does not copy constraint truth into the index-partition owner and does not change issued digest identity.

## Alternatives rejected

Checking only `CompareIndexInfo()`-equivalent index properties was rejected because PostgreSQL performs a separate constraint lookup after definition equivalence. Inferring a child constraint from a unique or primary-shaped index was rejected because an index can exist without being owned by a constraint and ConceptWeave must preserve that distinction. Requiring constraints for every attached index was rejected because PostgreSQL allows standalone partitioned indexes.

## Verification boundary

The focused contract has two controls. A valid parent primary-key backing index plus an otherwise compatible valid child index with no child constraint must fail with `index_partition_child_constraint`. The same topology with a child primary-key constraint bound to the child index must be admitted.

No executed Rust RED/GREEN is claimed for these commits. The available execution host exposes no `cargo`, `rustc`, or `rustup`; exact-head acceptance therefore still requires the repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the focused contract plus all retained Source Observation contracts, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/review gates on one unchanged head.

The PostgreSQL live differential should create a partitioned primary-key or unique-constraint parent, create a structurally compatible standalone unique index on a direct child partition, and verify that `ALTER INDEX parent_index ATTACH PARTITION child_index` fails because the child index has no constraint. The positive control should attach a child index that is already owned by its child constraint and then verify both index inheritance and constraint parentage from the catalogs.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `ATExecAttachPartitionIdx()` in `src/backend/commands/tablecmds.c`* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/tablecmds.c#L20277-L20435
