# PostgreSQL 18 NOT NULL relation-kind integrity

## Decision

ConceptWeave admits first-class `pg_constraint.contype = 'n'` evidence only for relation kinds that PostgreSQL 18 can expose as table-like constraint owners: ordinary tables, partitioned tables, and foreign tables.

Views, materialized views, sequences, and standalone composite types fail closed at `NotNullConstraintObservation` construction. This is a source-integrity boundary, not a semantic normalization: ConceptWeave must not mint governed NOT NULL identity for a relation kind that cannot own such a source constraint.

## PostgreSQL 18 authority

The PostgreSQL 18 `pg_constraint` catalog defines `contype = 'n'` as a not-null constraint and describes the catalog as storing constraints on tables. `CREATE TABLE` admits named and unnamed NOT NULL constraints, and a table created with `PARTITION BY` is a partitioned table under the same command. `CREATE FOREIGN TABLE` independently admits NOT NULL constraints and may create a foreign table as a declarative partition.

The boundary is intentionally not `RelationKind::Table` alone. PostgreSQL explicitly supports foreign-table NOT NULL constraints and foreign-table partitions, while partitioned tables inherit NOT NULL constraints to every partition. Conversely, PostgreSQL composite-type syntax does not admit constraints such as NOT NULL, and view/materialized-view/sequence creation does not define table NOT NULL constraint ownership.

Primary sources (APA 7th-style traceability):

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FOREIGN TABLE*. https://www.postgresql.org/docs/18/sql-createforeigntable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Composite types*. https://www.postgresql.org/docs/18/rowtypes.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

## Alternatives considered

Accepting every modeled `RelationKind` was rejected because the generic v3 relation model intentionally represents views, materialized views, sequences, and composite types as well as table-like relations. Presence of a same-kind relation and a non-nullable column in the snapshot does not prove that `pg_constraint.contype = 'n'` can exist for that relation.

Restricting NOT NULL evidence to ordinary tables was also rejected because PostgreSQL 18 permits NOT NULL constraints on partitioned tables and foreign tables, including foreign tables used as partitions.

The selected rule therefore admits exactly `Table`, `PartitionedTable`, and `ForeignTable` and rejects the other currently modeled relation kinds before immutable evidence construction.

## Executable traceability

- Finding: PR #46 review `5195739325`, exact predecessor `79caeb06afbfbf59ec4d68f68d43585b083f3b48`.
- RED contract: `crates/conceptweave-observation/tests/not_null_constraint_relation_kind_contract.rs`, commit `7c39e5ef803429f38d7897ad75d19f0a5e2db90b`.
- Production repair: `crates/conceptweave-observation/src/not_null_constraint.rs`, commit `2e3be2591e0c68acb0a180421d3d6a20fc6e9ff4`.

The regression preserves all three source-representable owner kinds and separately rejects View, MaterializedView, Sequence, and CompositeType. The production change is confined to constructor validation; digest framing, inheritance semantics, partition-parent validation, and source identifiers are unchanged.

## Adapter obligation

A PostgreSQL transport must resolve `pg_constraint.conrelid` to the exact `pg_class.relkind` in the same bounded catalog snapshot and pass that relation kind without coercion. It must not map views or other non-table relations to `Table` merely to satisfy the domain contract. Foreign-table constraints remain source assertions whose remote enforcement semantics are outside this observation invariant.

## Acceptance status

The RED contract and causal source repair are source-level evidence. They are not an executed Rust GREEN until one unchanged exact PR head passes the repository-pinned Rust 1.98 native suite and applicable hosted acceptance gates.
