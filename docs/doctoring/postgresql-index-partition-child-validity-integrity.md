# PostgreSQL 18 partitioned-index child-validity integrity

Status: Proposed source repair; execution acceptance pending.

## Problem

`IndexPartitionSnapshot` already required a valid partitioned parent index to cover every direct local relation partition with an attached child index. The retained rule checked attachment existence but did not compose the attached child's observed `pg_index.indisvalid` value. A governed tuple could therefore claim `parent.indisvalid = true` while an attached local child index remained `indisvalid = false`.

That state contradicts PostgreSQL's partitioned-index validity contract. PostgreSQL's `validatePartitionedIndex()` defines a valid partitioned index as one whose required partition indexes are themselves valid. PostgreSQL has had bugs in the opposite direction, where a parent remained invalid after children were repaired, but upstream explicitly treats parent-valid with an invalid child as an invalid state. The 2023 PostgreSQL fix for partitioned-index creation states that `indisvalid` is true only when all partitions are valid and must be false when at least one partition index is invalid. The 2026 revalidation discussion preserves the same invariant while repairing stale-invalid parents.

## Authority and traceability

Primary current authority is PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`. The branch tip was re-read on 2026-09-16 KST. The upstream partitioned-index validity semantics are also recorded by PostgreSQL commit `cfc43aeb3810ebaa8dbda4807046a4c953d9e992` and the April 2026 `validatePartitionedIndex()` repair discussion.

ConceptWeave owner seams:

- PR #46 review finding: `5216108863`, anchored to `4956a1b8c35de068de41a1abdcc425b8835719c5`.
- Source regression contract: `e613eccc32197f777753d4a79226b99af75b7540`, `crates/conceptweave-relation-partition/tests/index_partition_parent_validity_contract.rs`.
- Minimal production repair: `098e20fef89dc9604d9341362f7f950862250bd4`, `crates/conceptweave-relation-partition/src/index_partition_base.rs`.
- Existing digest domain `conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.v1` remains unchanged.

## Decision

When a partitioned parent index reports `valid() == Some(true)`:

1. Every direct local table or partitioned-table partition still requires an attached child index owned by that parent index.
2. The attached local child index must itself report `valid() == Some(true)`; false or absent validity fails closed with `index_partition_child_validity`.
3. The existing foreign-table exception remains unchanged: regular/non-unique parent indexes do not require a local foreign child index, while unique parents over a foreign partition remain rejected.
4. Invalid parent indexes preserve `CREATE INDEX ON ONLY` staging semantics; this repair does not require staged unattached children to be valid.

The rule composes already observed `pg_index.indisvalid` facts. It does not synthesize child state, mutate the predecessor digest, or infer a repaired parent from child state.

## Alternatives rejected

Checking only attachment existence was rejected because it can certify a PostgreSQL-inconsistent parent-valid/child-invalid topology. Recomputing or overwriting the parent's validity from children was rejected because ConceptWeave observes source truth rather than repairing PostgreSQL catalog state. Requiring every child to be valid even while the parent is invalid was rejected because PostgreSQL intentionally supports incomplete/staged partitioned-index construction and has source-reachable stale-invalid parent states.

## Verification boundary

The focused contract now has three local controls: a valid parent with an unattached local child is rejected, a valid parent with an attached but invalid local child is rejected, and a valid parent with an attached valid local child is admitted. The invalid-parent staging control and the foreign-table exception/rejection controls remain.

No executed Rust RED/GREEN is claimed for these commits. The automation execution host exposes no `cargo`, `rustc`, or `rustup`, and protected ConceptWeave `main` still lacks the Product PR workflow carried by #35. Exact-head acceptance still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the focused contract and every retained Source Observation contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and applicable hosted Product/security/review gates on one unchanged head.

The PostgreSQL live differential should construct or reproduce an invalid child partition index and verify that a partitioned parent cannot be source-observed as valid while that child remains invalid. The positive control is a fully attached all-valid local hierarchy. A stale-invalid parent with repaired children remains a separate accepted source observation until PostgreSQL itself revalidates it.

## References

PostgreSQL Global Development Group. (2023). *Fix marking of indisvalid for partitioned indexes at creation* (commit `cfc43aeb3810ebaa8dbda4807046a4c953d9e992`). https://git.postgresql.org/pg/commitdiff/cfc43aeb3810ebaa8dbda4807046a4c953d9e992

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source code* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/tree/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1

PostgreSQL hackers. (2026, April). *Fix: Partitioned parent index remains invalid after child indexes are repaired*. https://www.postgresql.org/message-id/CAGnOmWqi1D9ycBgUeOGf6mOCd2Dcf%3D6sKhbf4sHLs5xAcKVCMQ%40mail.gmail.com
