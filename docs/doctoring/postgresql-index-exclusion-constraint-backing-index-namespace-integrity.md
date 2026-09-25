# PostgreSQL ordinary EXCLUDE backing-index namespace integrity

## Problem

ConceptWeave already governs the ordinary `EXCLUDE` constraint namespace (`pg_constraint.connamespace`) and the exact `conindid` backing-index coordinate. The backing index itself is represented under its owning relation, so `IndexPartitionCoordinate.schema_name` is inherited from that relation coordinate. That is useful semantic structure, but it is not the same thing as observing the backing index row's own `pg_class.relnamespace`.

Without an independent backing-index namespace observation, an extractor can read the table as `public.bookings`, mis-resolve the index `pg_class.relnamespace` as another namespace, then normalize the index back under `public.bookings`. The current normalized coordinate would hide that source contradiction and issue the same governed identity.

## PostgreSQL authority

PostgreSQL 18 `pg_class` describes indexes as relations and stores `relnamespace` as a distinct catalog column referencing `pg_namespace.oid`. PostgreSQL 18 `CREATE INDEX` also specifies that an index is always created in the same schema as its parent table. The schema namespace is shared by tables, sequences, indexes, views, materialized views, and foreign tables.

Primary references:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE INDEX*. https://www.postgresql.org/docs/18/sql-createindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Schemas*. https://www.postgresql.org/docs/18/ddl-schemas.html

## Decision

`IndexExclusionConstraintIndexNamespaceSnapshot` is a domain-separated successor of `IndexExclusionConstraintIndexNameSnapshot`.

For every exact ordinary-EXCLUDE constraint/index binding already proven by the predecessor, the adapter must supply exactly one `IndexExclusionConstraintIndexNamespaceObservation` containing the namespace name independently resolved from the backing index row's `pg_class.relnamespace`.

The successor rejects:

- missing namespace evidence for any predecessor binding;
- duplicate constraint coordinates;
- namespace evidence attached to a different backing-index coordinate;
- blank resolved namespace names;
- a resolved index namespace that differs from either the exact backing-index coordinate schema or the owning constraint/relation schema.

The raw resolved namespace name, exact constraint coordinate, and exact backing-index coordinate enter a new digest domain and provenance receipt. No predecessor digest domain changes.

## Rejected alternatives

Deriving the index namespace from `RelationObservation.schema_name`, `IndexPartitionCoordinate.schema_name`, constraint namespace evidence, or the fact that PostgreSQL normally creates indexes beside their parent table was rejected. Those approaches restate an invariant instead of preserving the independent source fact needed to detect extractor or catalog drift.

Adding a schema-wide constraint-name rule was also rejected. The relevant namespace rule belongs to the index relation in `pg_class`; non-index constraints do not generally occupy that relation namespace.

## Traceability

Finding review: `5224968197` on PR #46 exact `9ee431847d4814d8500d879d8e21134be1d7ec3b`.

RED contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_namespace_contract.rs` first added at `81a813cedf6c239cccd64e123e85b6a352cee032` before the successor type existed.

Production contract: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_namespace.rs`, introduced at `b7783930e17741ff04efcac2ca0e39135f45192e` and publicly composed through `crates/conceptweave-relation-partition/src/index_partition.rs` at `7914d15d44b50bf092121d5af2951d94e3926e4d`.

The RED commit is structural source/compile evidence only until the repository-pinned Rust toolchain executes the focused contract on the same exact head. Hosted/native GREEN must not be inferred from source presence, mechanical mergeability, or unrelated commit statuses.

## Live differential requirement

A PostgreSQL 18 differential must read the ordinary EXCLUDE row, follow `conindid` to the backing index `pg_class` row, and independently resolve that row's `relnamespace` through `pg_namespace`. Only after that independent read may it compare the resolved namespace to the parent relation and constraint coordinates. Copying the parent table namespace into the index observation is not valid differential evidence.
