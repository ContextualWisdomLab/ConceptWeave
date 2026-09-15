# PostgreSQL 18 partitioned-index foreign-child integrity

Status: Proposed source repair; execution acceptance pending.

## Problem

`IndexPartitionSnapshot` previously interpreted a valid partitioned index as requiring one attached child index for every direct relation partition. That rule is too strong for PostgreSQL 18 foreign-table partitions. `DefineIndex()` deliberately skips a `RELKIND_FOREIGN_TABLE` child when building a regular partitioned index because PostgreSQL does not create a local index relation on that foreign partition. The same source branch rejects `UNIQUE` or primary index construction when a foreign-table partition is present.

The old invariant therefore produced a false negative for a source-reachable state: a valid non-unique partitioned index, a direct foreign-table partition, and no child index relation for that foreign partition. Simply exempting every foreign partition would be incorrect because it would also admit a valid unique partitioned index over a foreign child, which PostgreSQL rejects.

## Authority and traceability

Primary authority is PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/backend/commands/indexcmds.c`. In the recursive partition loop, PostgreSQL checks `childrel->rd_rel->relkind == RELKIND_FOREIGN_TABLE`; `stmt->unique || stmt->primary` raises `ERRCODE_WRONG_OBJECT_TYPE`, while a regular index restores security context, closes the child relation, and `continue`s without creating or attaching a child index. Ordinary local partitions continue through `CompareIndexInfo()` and either attach an equivalent index or receive a recursively created child index.

ConceptWeave owner seams:

- PR #46 review finding: `5215600716`, anchored to `1623ee9f684873b8c2d94d44a6fc99d10ae340a3`.
- Source RED contract: `9e265eacdae023f2e33adcadecaffcfc9f30ff82`, `crates/conceptweave-relation-partition/tests/index_partition_parent_validity_contract.rs`.
- Minimal production repair: `692a50827af31cf17558e06a0ddf3ac95161efe5`, `crates/conceptweave-relation-partition/src/index_partition_base.rs`.
- Existing digest domain `conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.v1` is unchanged.

## Decision

For a valid partitioned parent index:

1. Direct local table or partitioned-table children still require an attached child index owned by that same parent index.
2. A direct foreign-table partition is exempt from child-index attachment only when the parent index is non-unique.
3. A valid unique parent index with a direct foreign-table partition fails closed with `index_partition_foreign_partition_unique`.
4. Invalid/staged parent indexes retain the existing `CREATE INDEX ON ONLY` behavior and are not forced into complete child attachment.

This is relation-kind-aware composition of already observed relation and index facts. It does not invent FDW index metadata, copy foreign ownership truth, or alter an issued digest.

## Alternatives rejected

Requiring a child index for foreign partitions was rejected because PostgreSQL explicitly skips creation for regular indexes. Exempting every foreign partition was rejected because it would admit PostgreSQL-impossible valid unique/primary partitioned-index topology. Adding synthetic placeholder child indexes was rejected because no corresponding PostgreSQL index relation exists and such placeholders would corrupt governed provenance.

## Verification boundary

The focused contract adds two source witnesses: a valid non-unique parent with a direct foreign-table partition and no child index must be admitted; a valid unique parent over the same topology must be rejected. Existing local-table coverage tests remain unchanged and continue to require attached children for valid partitioned indexes.

No Rust execution GREEN is claimed for these commits. The available execution host lacks `cargo`, `rustc`, and `rustup`, and protected ConceptWeave `main` still lacks the Product PR workflow carried by #35. Exact-head acceptance therefore still requires repository-pinned Rust 1.98 `fmt`, strict Clippy, focused/workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and hosted Product/security/review gates on one unchanged head.

The later PostgreSQL live differential should create or attach a foreign-table partition beneath a partitioned table, prove regular partitioned-index creation remains valid without a foreign child index relation, and prove unique index creation is rejected for the same hierarchy. Local-table partition coverage remains the positive control for mandatory child attachment.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code: `src/backend/commands/indexcmds.c`* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/indexcmds.c
