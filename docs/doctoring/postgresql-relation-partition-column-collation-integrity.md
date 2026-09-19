# PostgreSQL 18 relation-partition column-collation integrity

## Decision

ConceptWeave relation-partition evidence must compose an already-observed complete `pg_attribute.attcollation` family with each direct declarative-partition edge. When the column-collation family is observed, every child partition column must retain the exact resolved collation identity of the matching partitioned-table parent column. When the family was not observed, relation-partition construction must not infer collation state.

This is a Source Observation invariant, not a presentation or catalog-consumption rule. The existing `conceptweave-observation` owner remains responsible for resolving capture-time collation OIDs to exact qualified `pg_collation` coordinates and for validating family completeness and determinism consistency. `conceptweave-relation-partition` only composes those already-governed facts across the direct `pg_inherits` edge.

## Problem and source authority

The predecessor relation-partition validator checked matching column-name sets, exact type binding, retained typmod evidence, identity mode, generated-column mode, acyclicity, and PostgreSQL 18 NOT NULL inheritance. It did not compare the optional complete column-collation family. A snapshot could therefore admit a parent `text COLLATE "C"` column and a child partition `text COLLATE "POSIX"` column even though PostgreSQL rejects that parent/child shape.

PostgreSQL 18 `REL_18_STABLE` is pinned here to `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`. In `src/backend/commands/tablecmds.c`, the inheritance/partition attachment path calls `MergeAttributesIntoExisting()`. That routine checks the existing child attribute against the parent attribute and rejects a different `attcollation` after type and typmod compatibility checks. The public PostgreSQL 18 partitioning documentation likewise requires a table attached as a partition to have columns that exactly match the parent table.

The governed model cannot rely on planner-time defensive checks or on a later consumer to discover this contradiction. A source-unreachable partition tuple must fail before immutable relation-partition evidence is admitted.

## Alternatives

1. **Ignore collation at the partition owner boundary.** Rejected. It permits a PostgreSQL-rejected source state to become governed immutable evidence whenever the complete column-collation family is already present.
2. **Copy collation state into the relation-partition aggregate.** Rejected. `conceptweave-observation` already owns column-collation truth. Copying it would create a second mutable semantic identity and violate the canonical-owner boundary.
3. **Require column-collation evidence for every relation-partition snapshot.** Rejected. Optional source families deliberately distinguish unobserved from observed; relation-partition evidence must not invent facts that the adapter did not capture.
4. **Compose the existing observed family only when present.** Selected. This preserves owner boundaries, keeps the predecessor and relation-partition digest domains unchanged, and fails closed only when source-authoritative evidence proves a contradiction.

## RED and repair traceability

- Review finding: ConceptWeave PR #46 review `5215040123`, anchored to predecessor exact head `17104b906369b8e6ade7fde4030bf2dba260e745`.
- Source RED: commit `7cf61bae74c72595fba47b92061d7366ef39c329`, `crates/conceptweave-relation-partition/tests/relation_partition_column_collation_contract.rs`.
  - Parent `public.labels.label`: `text`, resolved `pg_catalog.C`.
  - Child `public.labels_2026.label`: same type, resolved `pg_catalog.POSIX`.
  - Required failure field: `relation_partition_column_collation`.
  - Matching `C`/`C` remains the positive control.
- Minimal production repair: commit `107ac145b4bb9c18c45ae6d495cd97ceb3497ea9`, `crates/conceptweave-relation-partition/src/lib.rs`.
  - `canonicalize_relation_partitions()` composes column collations after rowtype compatibility and before declaration/graph/NOT NULL validation.
  - `validate_partition_column_collations()` is a no-op when the family is unobserved.
  - When observed, it resolves each parent column to the corresponding child coordinate and rejects different qualified collation identity.
  - No issued digest domain, receipt coordinate, source connection identity, or observation-family ownership changes.

`ColumnCollationObservation` also carries `collisdeterministic` evidence. The column-collation owner already guarantees that repeated use of the same qualified collation coordinate cannot disagree on determinism within a snapshot, so partition integrity is fundamentally the exact `attcollation` identity rule; determinism remains retained source evidence rather than a new partition-specific truth.

## Risk and effect

The repair narrows accepted evidence to PostgreSQL-reachable direct partition shapes when collation evidence is available. It does not require collation capture for legacy/unobserved snapshots, does not normalize `C` and `POSIX`, does not infer database-default collation, and does not compare rendered labels in place of exact qualified collation coordinates.

The remaining risk is execution evidence. The current automation host exposes no `cargo`, `rustc`, or `rustup`, and the ConceptWeave protected branch does not yet provide the repository-owned Product PR workflow needed to produce hosted exact-head acceptance. Therefore the RED and repair are source-contract evidence only until one unchanged head passes the repository-pinned Rust 1.98 toolchain and hosted gates.

## Required live differential

A PostgreSQL 18 transport/live differential should prove both sides of the boundary:

- create a partitioned parent with a collatable column and attach/create a child carrying the same collation; catalog capture must produce identical resolved parent/child `attcollation` identity and ConceptWeave must accept it;
- attempt an `ATTACH PARTITION` using an otherwise-compatible child whose matching column uses a different collation; PostgreSQL must reject the attach and the equivalent synthetic source tuple must be rejected by the focused owner contract;
- retain an unobserved-column-collation control to prove that relation-partition evidence does not invent optional source-family state;
- preserve existing identity/generated/NOT NULL and type/typmod partition differentials on the same exact successor lineage.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source code, REL_18_STABLE: `src/backend/commands/tablecmds.c`* (Commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/commands/tablecmds.c
