# PostgreSQL replica-identity index integrity

Status: executable RED on ConceptWeave PR #46. Production repair and exact-head execution evidence remain outstanding.

## Problem

`IndexCatalogFlags::replica_identity()` preserves exact `pg_index.indisreplident` state and participates in governed snapshot identity, but the base Source Observation aggregate currently does not validate the eligibility constraints PostgreSQL applies when an index is selected by `ALTER TABLE ... REPLICA IDENTITY USING INDEX`. That permits internally contradictory exact-catalog evidence to become an immutable source snapshot.

Review `5261851100` identified the gap on exact predecessor `e9b3823483905cfbdadf0596202d5bdf73d193d5`. Executable RED `23eb20a711c2929820cd5509468935ba51d2e316` introduced the focused contract; `7206f154fd67c4f512a74132d7aeb742d29c2e05` changed only the fixture observation time to the same-run UTC coordinate; `e77be30c905e71578b5ec0a8cea29e2d0afd736f` widened the RED to reject replica-identity state on an index-owning materialized view; `6dffae84483172a6b9e8bd2a9ecb586eb49864fb` adds positive relation controls proving partitioned-table identity remains admissible without optional lifecycle evidence and materialized-view indexes remain admissible when they do not claim replica identity.

## PostgreSQL 18 authority

PostgreSQL 18 `pg_index` defines `indisreplident` as true when an index has been chosen as replica identity with `ALTER TABLE ... REPLICA IDENTITY USING INDEX`:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL 18 `ALTER TABLE` requires a `USING INDEX` replica-identity index to be unique, non-partial, non-deferrable, and to include only columns marked `NOT NULL`:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html

Logical-replication documentation distinguishes an explicitly selected replica-identity index from `REPLICA IDENTITY FULL` and limits replication objects to tables, including partitioned tables. Materialized views are not replication relations even though ConceptWeave correctly allows them to own ordinary indexes:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Publication — Replica Identity*. https://www.postgresql.org/docs/18/logical-replication-publication.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Logical Replication Restrictions*. https://www.postgresql.org/docs/18/logical-replication-restrictions.html

PostgreSQL's upstream replica-identity regression suite also preserves a partitioned-table case in which replica identity is selected on an index that is not yet valid. That is implementation evidence that base replica-identity eligibility must not be widened into an unconditional `indisvalid`/`indisready`/`indislive` requirement:

- PostgreSQL Global Development Group. (2026). *replica_identity.sql* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/test/regress/sql/replica_identity.sql

Under `FULL`, subscriber-side search may use other candidate indexes; those candidates are not evidence that `pg_index.indisreplident` is true.

## Executable boundary

`crates/conceptweave-observation/tests/index_replica_identity_contract.rs` pins the following base-aggregate behavior:

- `replica_identity=true` is rejected for an index on a materialized view; explicit index replica identity belongs to table/partitioned-table relations rather than every relation kind that may own an index.
- `replica_identity=true` on a non-unique index fails closed.
- `replica_identity=true` with `indimmediate=false` fails closed; this is the same-generation catalog signal that the unique check is not immediate and therefore cannot satisfy the non-deferrable requirement.
- `replica_identity=true` on a partial index fails closed.
- `replica_identity=true` on an expression-key index fails closed because `USING INDEX` requires column identity rather than expression identity.
- every key column of a replica-identity index must resolve to an owning relation column observed as `NOT NULL`.
- a unique, non-partial, immediate, column-key-only table index over `NOT NULL` columns remains admissible.
- the same eligible shape on a partitioned table remains admissible without importing optional index lifecycle evidence.
- indexes with `replica_identity=false` do not inherit these restrictions merely because they could or could not be useful to logical replication.
- an ordinary materialized-view index remains admissible when it does not claim replica identity.

The focused RED deliberately does not model publication membership, subscriber configuration, or `REPLICA IDENTITY FULL`. Those are different catalog/runtime facts and are not prerequisites for validating an observed `indisreplident=true` index.

## Minimal causal repair

The repair belongs at `validate_schema_relation_invariants()` (or an equivalent final relation/index aggregate boundary), not in `IndexCatalogFlags::new()`: the latter cannot see the owning relation kind, owning relation columns, or a predicate added later to the index observation.

For each index whose observed catalog flags have `replica_identity() == true`, the aggregate should require all of the following before immutable snapshot construction:

1. the owning relation kind is `Table` or `PartitionedTable`;
2. `is_unique() == true`;
3. `catalog_flags.immediate() == true`;
4. `predicate().is_none()`;
5. every key attribute is a simple column reference rather than an expression;
6. every referenced key column resolves on the same relation and `nullable() == false`.

The repair should return `InvalidObservationField { field: "index_replica_identity" }` for contradictory states and leave non-replica indexes unchanged. It must not require optional `ready`, `valid`, or `live` lifecycle observations at this base seam; those facts have independent semantics and PostgreSQL itself supports replica-identity selection during partitioned-index restore before final validity.

## Rejected alternatives

Validating only `is_unique` in `with_catalog_flags()` is insufficient because relation kind, partiality, and owning-column nullability are available only after the complete index/relation aggregate exists. Reusing `REPLICA IDENTITY FULL` subscriber-search rules is also incorrect: PostgreSQL documents those as a separate fallback search path and `pg_index.indisreplident` specifically identifies the explicit `USING INDEX` selection. Publication membership is not imported into Source Observation because the catalog invariant exists independently of whether a table is currently published.

Requiring `ready/valid/live == Some(true)` would also be incorrect at this seam. Those fields are optional observations in ConceptWeave, and PostgreSQL's partitioned-table restore regression explicitly exercises a replica-identity index that is not yet valid. Unknown lifecycle evidence therefore must not be converted into a negative eligibility claim.

`IndexCatalogFlags::exclusion()` is intentionally not folded into this repair. Plain `EXCLUDE` constraint identity belongs to `IndexExclusionConstraintSnapshot`, while temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` can legitimately use exclusion semantics. Duplicating that successor truth in the base aggregate would violate the owner boundary.

## Acceptance

This finding is not GREEN until one unchanged exact head demonstrates the focused RED becoming GREEN together with retained observation tests, repository-pinned Rust 1.98 formatting and strict Clippy, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and the PostgreSQL 18 same-generation differential required by the parent PR. Documentation or source movement invalidates predecessor execution evidence.
