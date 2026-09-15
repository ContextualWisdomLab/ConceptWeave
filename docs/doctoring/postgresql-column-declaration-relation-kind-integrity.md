# PostgreSQL 18 column declaration relation-kind integrity

## Decision

ConceptWeave treats non-empty `pg_attribute.attidentity` and `pg_attribute.attgenerated` values as relation-kind-constrained source facts before they can participate in governed successor identity.

For PostgreSQL 18:

- non-empty identity state is admissible only for `RelationKind::Table` and `RelationKind::PartitionedTable`;
- non-empty generated-column state is admissible only for `RelationKind::Table`, `RelationKind::PartitionedTable`, and `RelationKind::ForeignTable`;
- the explicit catalog-empty states (`attidentity = ''`, `attgenerated = ''`) remain admissible for every modeled relation kind, because absence of a declaration is still observed source evidence rather than missing evidence.

This is a source-domain validation rule. It does not infer declarations, change an issued digest domain, or copy PostgreSQL catalog ownership into another bounded context.

## Problem

The complete column identity and generation families already validated exact column coordinates, family completeness, identity nullability, and the generated-column/identity mutual exclusion. They did not validate whether a non-empty declaration could exist on the observed `pg_class.relkind`.

That allowed fabricated observations such as a view carrying `attidentity = 'a'`, or a materialized view carrying `attgenerated = 's'`, to enter a governed successor digest even though PostgreSQL 18 DDL cannot create those catalog states.

Review `5214601753` records the finding on predecessor exact head `619a43b493ee4e3751eecb43e25e688ae5afda58`.

## PostgreSQL 18 authority

Authority was checked against `postgres/postgres` `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

`CREATE TABLE` admits both generated-column declarations and identity declarations, and the same statement family can produce a partitioned table with `PARTITION BY`. PostgreSQL also documents that leaf partitions inherit identity columns from the partitioned table rather than owning a separate identity definition. `CREATE FOREIGN TABLE` admits generated-column declarations (`GENERATED ALWAYS AS (...) [ STORED | VIRTUAL ]`) but exposes no identity-column syntax.

Primary sources:

- PostgreSQL Global Development Group. (2026). *CREATE TABLE* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ref/create_table.sgml`.
- PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ref/create_foreign_table.sgml`.
- PostgreSQL Global Development Group. (2026). *Data Definition — Identity Columns* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ddl.sgml`.

## Alternatives considered

Rejecting the entire identity or generation family for unsupported relation kinds was rejected because `pg_attribute` still exposes the empty declaration state and ConceptWeave must preserve that observed negative fact.

Treating foreign tables like ordinary tables for identity was rejected because PostgreSQL 18 `CREATE FOREIGN TABLE` supports generated expressions but not identity declarations.

Inferring support from relation names, provider conventions, or client DDL text was rejected because the governed boundary is the exact catalog relation kind plus exact `pg_attribute` state.

## RED and repair lineage

Source RED `e5a5f10140d8b66299e317612b36a20d28f52ac1` adds `column_declaration_relation_kind_contract.rs`. It retains positive controls for table/partitioned-table identity and table/partitioned-table/foreign-table generated columns, rejects non-empty modes on unsupported relation kinds, and proves that explicit empty modes remain representable on a view.

Identity production repair converges at `2faafe07c2b53b9538a56d31a6febc5c07eae16e`: `canonicalize_column_identities` rejects non-empty identity state unless the relation is a table or partitioned table. An intermediate ordinary-forward replacement (`026f4e6a3ced6c4dbc6c81b2b1fb9b5b76b85401`) introduced an `encode_len` typo while replacing the full source file; the immediately following commit restores the original helper exactly and retains only the intended relation-kind guard. No history rewrite or force-push was used.

Generation production repair `6f4d8871fc41e0ad283d950a00b80d6d5c8c410d` rejects non-empty generated-column state unless the relation is a table, partitioned table, or foreign table.

## Validation boundary

The repository host used for this lane does not provide the repository-pinned Rust 1.98 toolchain, so these commits are source-reviewed RED→causal-repair evidence, not executed exact-head GREEN. Acceptance still requires one unchanged exact head to pass formatting, strict workspace/all-target Clippy, the focused declaration relation-kind contract, all retained Source Observation tests, workspace/doc tests, release build, rustdoc/coverage obligations, and the hosted Product/security gates after the central workflow prerequisite lands.

A PostgreSQL 18 live differential should additionally prove the catalog boundary: table and partitioned-table identity state, inherited leaf-partition identity state, foreign-table generated state, and empty `attidentity`/`attgenerated` on relation kinds that cannot own those declarations.
