# Source Observation constraint relation-kind invariant

## Decision

`ConstraintTimingObservation` and `ConstraintPeriodObservation` are successor PostgreSQL catalog evidence for table constraints. Their relation coordinate is therefore valid only for `RelationKind::Table` and `RelationKind::PartitionedTable`.

The constructors fail closed for views, materialized views, foreign tables, sequences, and standalone composite-type relations. Frozen v2 contracts are unchanged.

## Problem and evidence

The value objects previously accepted every `RelationKind`. Aggregate admission rejected many impossible combinations later because no matching PRIMARY KEY, UNIQUE, or FOREIGN KEY constraint could be found, but the domain object itself could still represent an impossible catalog coordinate before aggregate construction.

PostgreSQL 18 exposes `pg_constraint.conperiod` only as `WITHOUT OVERLAPS` state for PRIMARY KEY/UNIQUE constraints or `PERIOD` state for FOREIGN KEY constraints. `CREATE TABLE` is the owner syntax for those constraint families. `CREATE FOREIGN TABLE` exposes only `NOT NULL` and `CHECK` as table constraints; it does not expose PRIMARY KEY, UNIQUE, or FOREIGN KEY table constraints. This matches ConceptWeave's existing relation-level admission boundary.

Allowing an invalid relation kind and relying on a later aggregate error was rejected because it makes an impossible PostgreSQL fact representable inside the Source Observation domain and duplicates responsibility across validation layers.

## Traceability

- Finding review: `5184492366` on #46 exact `db879504dce68dc532a8b09ed2d4fec71f68d244`.
- Behavioral RED: `ccf1a0558389dcf6b7d5af456c83f58f018a8704`, `constraint_catalog_relation_kind_contract.rs`.
- `ConstraintPeriodObservation` repair: `635a9ea05af9daffd78a469bc72e69df4633317e`.
- `ConstraintTimingObservation` repair: `67da1f9d7ac34e9aa75eb92696c7ec695553e481`.
- Production modules: `crates/conceptweave-observation/src/constraint_period.rs`, `crates/conceptweave-observation/src/constraint_timing.rs`.
- Acceptance remains exact-head scoped; predecessor execution evidence does not transfer after these commits.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-createforeigntable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
