# PostgreSQL index relation-kind ownership

## Decision

`RelationObservation::with_indexes()` admits non-empty local index evidence only when the observed PostgreSQL relation kind is `Table`, `PartitionedTable`, or `MaterializedView`. `View`, `ForeignTable`, `Sequence`, and `CompositeType` fail closed with `ObservationError::InvalidObservationField { field: "indexes" }`. An explicitly observed empty index collection remains valid for every modeled relation kind.

This is an observation-integrity rule, not a catalog-normalization rule. ConceptWeave records a relation's exact `pg_class.relkind`; it must not construct a governed relation/index snapshot that PostgreSQL cannot own locally.

## Problem and demonstrated RED

Review `5260402128` found that the predecessor completion boundary validated index key semantics, access method, rendered predicate/index-definition evidence, duplicate names, and local-column references but never checked the owning `RelationKind`. A complete `IndexObservation` could therefore be attached to `View`, `ForeignTable`, `Sequence`, or `CompositeType` and enter immutable snapshot/digest material.

Executable contract commit `974972e2885baa956af9b728fee158b1782704d1` added `crates/conceptweave-observation/tests/index_relation_kind_ownership_contract.rs` before production movement. The contract requires:

- non-empty indexes on `View`, `ForeignTable`, `Sequence`, and `CompositeType` to fail as `indexes`;
- non-empty indexes on `Table`, `PartitionedTable`, and `MaterializedView` to remain admissible when the index evidence itself is complete;
- empty index collections to remain admissible for every relation kind.

Synthetic Rust fixtures establish the local domain contract only. They do not substitute for the required PostgreSQL 18 same-generation differential.

## Primary authority

PostgreSQL 18 distinguishes relation kinds in `pg_class`; tables, indexes, sequences, views, materialized views, composite types, foreign tables, and partitioned tables are separate `relkind` states, and not every `pg_class` attribute is meaningful for every kind.

PostgreSQL's index documentation defines indexes over table-like stored relations, while the materialized-view documentation explicitly demonstrates local indexes on materialized views and contrasts them with `file_fdw` foreign access that does not support indexes in that example. `REFRESH MATERIALIZED VIEW CONCURRENTLY` further requires a qualifying unique index on the materialized view. Partitioned-table index ownership remains represented as the parent partitioned-index contract with matching indexes on partitions; ConceptWeave therefore keeps `PartitionedTable` in the admissible owner set.

Primary references (APA 7th style):

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.11. pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 39.3. Materialized views*. https://www.postgresql.org/docs/18/rules-materializedviews.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: REFRESH MATERIALIZED VIEW*. https://www.postgresql.org/docs/18/sql-refreshmaterializedview.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: CREATE INDEX*. https://www.postgresql.org/docs/18/sql-createindex.html

## Alternatives considered

Allowing every `pg_class` relation kind and relying on the adapter to omit impossible indexes was rejected because the governed domain boundary would remain forgeable by another adapter or caller.

Rejecting all non-ordinary-table kinds was rejected because materialized views are explicitly indexable and partitioned tables own partitioned-index semantics.

Moving the rule into `IndexObservation::new()` was rejected because an index object does not own or know the enclosing relation kind. The invariant belongs at `RelationObservation::with_indexes()`, where both the relation kind and the completed index collection are available.

## Production repair

Commit `25cacf190a18659edc0e6aea82173fbf878b94fd` adds the minimal completion-boundary guard before any index contents are consumed. It changes only `crates/conceptweave-observation/src/representation_v3.rs` (+13/-1 versus the RED head), including rustdoc. Existing key/layout/access-method/identifier/rendered-text/duplicate/local-column validation is unchanged.

Review `5260989326` records the exact repair and explicitly does not claim hosted GREEN.

## Acceptance and rollback

The source defect is repaired, but acceptance remains open. The final unchanged exact head must execute repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 same-generation differential. The differential must include at least one table, partitioned-table, and materialized-view index owner and must demonstrate that non-index-owning modeled relation kinds do not acquire local index evidence through the adapter.

No predecessor run transfers after this source or documentation movement. If PostgreSQL owner semantics are widened in a future supported release or extension boundary, the change requires a new demonstrated contract and versioned semantic decision rather than silently relaxing this invariant.
