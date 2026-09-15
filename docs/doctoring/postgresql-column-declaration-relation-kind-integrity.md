# PostgreSQL 18 column declaration relation-kind integrity

## Decision

ConceptWeave treats non-empty `pg_attribute.attidentity` and `pg_attribute.attgenerated` values as relation-kind-constrained source facts before they can participate in governed successor identity.

For PostgreSQL 18:

- non-empty identity state is admissible for `RelationKind::Table`, `RelationKind::PartitionedTable`, and `RelationKind::ForeignTable`;
- non-empty generated-column state is admissible for `RelationKind::Table`, `RelationKind::PartitionedTable`, and `RelationKind::ForeignTable`;
- an explicit identity declaration remains prohibited on a `PARTITION OF` child, but a declarative partition can carry inherited identity catalog state from its partitioned-table parent;
- the explicit catalog-empty states (`attidentity = ''`, `attgenerated = ''`) remain admissible for every modeled relation kind, because absence of a declaration is still observed source evidence rather than missing evidence.

This is a source-domain validation rule. It does not infer declarations, change an issued digest domain, or copy PostgreSQL catalog ownership into another bounded context.

## Problem

The complete column identity and generation families already validated exact column coordinates, family completeness, identity nullability, and the generated-column/identity mutual exclusion. They did not originally validate whether a non-empty declaration could exist on the observed `pg_class.relkind`.

Review `5214601753` found that broad hole on predecessor exact head `619a43b493ee4e3751eecb43e25e688ae5afda58`. The first repair correctly rejected impossible view/materialized-view/sequence/composite-type declaration states, but it over-constrained identity evidence by also rejecting `RelationKind::ForeignTable`.

Review `5214795520` records the corrective P1 finding on exact head `72dd82517c8d49b520100a45c12596373554e8d9`: PostgreSQL 18 source accepts direct identity declarations in `CREATE FOREIGN TABLE`, so a non-empty `attidentity` on `relkind='f'` is source-reachable and must remain representable.

## PostgreSQL 18 authority

Authority was rechecked against `postgres/postgres` `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` on 2026-09-16 KST.

The decisive authority is parser/DDL source rather than the abbreviated `CREATE FOREIGN TABLE` syntax synopsis:

- `gram.y` parses `CREATE FOREIGN TABLE ... (' OptTableElementList ') ...`; foreign tables therefore use the same column-definition grammar that carries identity constraints.
- `transformCreateStmt()` recognizes `CreateForeignTableStmt`, sets `cxt.isforeign = true`, and still routes every `ColumnDef` through `transformColumnDefinition()`.
- the `CONSTR_IDENTITY` branch explicitly rejects typed tables and `PARTITION OF` children. It does not reject `cxt->isforeign`.
- the same parser source contains explicit `cxt->isforeign` rejections for unsupported primary-key, unique, exclusion, and foreign-key constraints, so the absence of such a rejection in the identity branch is material rather than an inferred omission.
- `CREATE TABLE ... PARTITION OF` cannot declare its own identity property; PostgreSQL partition identity instead follows the partitioned-table hierarchy.

The reference-page synopsis for `CREATE FOREIGN TABLE` does not currently spell out identity syntax, so it is not used to narrow a source path that the PostgreSQL 18 grammar and transform code actually accept.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL grammar* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `src/backend/parser/gram.y`.
- PostgreSQL Global Development Group. (2026). *Utility statement transformation* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `src/backend/parser/parse_utilcmd.c`.
- PostgreSQL Global Development Group. (2026). *CREATE TABLE* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ref/create_table.sgml`.
- PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ref/create_foreign_table.sgml`.
- PostgreSQL Global Development Group. (2026). *Data Definition — Identity Columns* (`REL_18_STABLE`, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). `doc/src/sgml/ddl.sgml`.

## Alternatives considered

Rejecting the entire identity or generation family for unsupported relation kinds remains rejected because `pg_attribute` still exposes the empty declaration state and ConceptWeave must preserve that observed negative fact.

Treating the `CREATE FOREIGN TABLE` documentation synopsis as a complete parser contract was rejected after source inspection. The actual PostgreSQL 18 grammar uses `OptTableElementList`, and the shared identity transform path contains no foreign-table prohibition.

Allowing identity on every modeled relation kind was rejected because views, materialized views, sequences, and composite types do not enter the source path that creates non-empty `attidentity` state.

Inferring support from relation names, FDW conventions, or client DDL text was rejected because the governed boundary is the exact catalog relation kind plus exact `pg_attribute` state.

## RED and repair lineage

Source RED `e5a5f10140d8b66299e317612b36a20d28f52ac1` added `column_declaration_relation_kind_contract.rs`. Identity production repair converged at `2faafe07c2b53b9538a56d31a6febc5c07eae16e`, while generation production repair `6f4d8871fc41e0ad283d950a00b80d6d5c8c410d` correctly retained foreign-table generated columns.

Fresh PostgreSQL source review found that the identity half of that contract was too narrow. Corrective source RED `1c188a6a63470d89db6867da0cd689fa0ee667b6` moves `RelationKind::ForeignTable` into the identity positive-control set while retaining negative controls for view, materialized view, sequence, and composite type. Minimal production repair `c3804597f89f6c471f9d203b386325b308cad4ee` widens only `relation_kind_supports_identity()` to `Table | PartitionedTable | ForeignTable`; nullability, completeness, generated/identity conflict, coordinate, and digest rules remain unchanged.

The earlier intermediate ordinary-forward replacement `026f4e6a3ced6c4dbc6c81b2b1fb9b5b76b85401` introduced an `encode_len` typo while replacing the full source file; `2faafe07c2b53b9538a56d31a6febc5c07eae16e` restored that helper. The corrective foreign-table repair does not reopen or alter that settled helper.

## Validation boundary

The available execution host does not provide the repository-pinned Rust 1.98 toolchain, so the corrective commits are source-reviewed RED→causal-repair evidence, not executed exact-head GREEN. Acceptance still requires one unchanged exact head to pass formatting, strict workspace/all-target Clippy, the focused declaration relation-kind contract, all retained Source Observation tests, workspace/doc tests, release build, rustdoc/coverage obligations, and the hosted Product/security gates after the central workflow prerequisite lands.

A PostgreSQL 18 live differential should prove direct `CREATE FOREIGN TABLE` identity creation and resulting non-empty `pg_attribute.attidentity`, direct foreign-table generated state, table/partitioned-table identity state, inherited leaf-partition identity state, and empty `attidentity`/`attgenerated` on relation kinds that cannot own those declarations. It should also distinguish an identity-bearing foreign table from a `PARTITION OF` child, because the latter cannot declare identity independently.
