# PostgreSQL 18 foreign-table NOT NULL validation integrity

## Decision

ConceptWeave rejects a PostgreSQL 18 foreign-table `NOT NULL` observation when `pg_constraint.convalidated = false`.

The enforcement and validation domains are distinct but both are narrow for NOT NULL: REL_18_STABLE stores NOT NULL constraints as enforced, and `ALTER FOREIGN TABLE ... ADD table_constraint [ NOT VALID ]` states that `NOT VALID` is allowed only for CHECK. Foreign-table NOT NULL therefore requires both `conenforced = true` and `convalidated = true` in source-authoritative evidence.

## Problem

`NotNullConstraintObservation::new()` admitted `RelationKind::ForeignTable` with `validated = false`. Because `validated` participates in the NOT NULL digest, a caller could mint an immutable governed identity for a catalog tuple that PostgreSQL 18's supported foreign-table DDL cannot create.

An earlier version of this note also repeated a broader interpretation of the `CREATE FOREIGN TABLE` synopsis and said foreign NOT NULL could preserve either enforcement value. That interpretation is superseded by REL_18_STABLE source: NOT NULL storage uses `is_enforced = true`, and `CreateConstraintEntry()` permits non-enforcement only for CHECK and FOREIGN KEY constraints.

## Primary authority

PostgreSQL Global Development Group. (2026). *ALTER FOREIGN TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-alterforeigntable.html

The command supports `ADD table_constraint [ NOT VALID ]`, currently for CHECK and NOT NULL, and explicitly states that `NOT VALID` is allowed only for CHECK.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: heap.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/heap.c

The NOT NULL storage path records the cooked constraint as enforced.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c

`CreateConstraintEntry()` permits a false enforcement bit only for CHECK or FOREIGN KEY constraints.

PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-createforeigntable.html

The synopsis and notes remain useful for DDL and remote-enforcement semantics, but they do not override the catalog-state invariants established by PostgreSQL's own storage code.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html

The release notes separately record ordinary-table `NOT NULL` support for `NOT VALID`; that broader capability must not be projected onto `ALTER FOREIGN TABLE` contrary to the foreign-table command contract.

## Rejected alternatives

Treating all relation kinds alike for validation is rejected because PostgreSQL 18 exposes different DDL state spaces for ordinary and foreign tables. Inferring validation from enforcement is also rejected: the model captures both catalog fields independently, then validates their relation-kind-specific admissibility. Allowing foreign-table `conenforced=false` based on synopsis placement is rejected by the stronger REL_18_STABLE implementation evidence.

## Source boundary

`NotNullConstraintObservation::new()` remains the fail-closed admission point. NOT NULL `enforced=false` is rejected first for every admitted relation kind. After that source invariant, `RelationKind::ForeignTable && !validated` fails with `not_null_constraint_foreign_validation` before any immutable snapshot digest can be produced.

The adapter must capture `convalidated` and `conenforced` independently from the same bounded catalog snapshot. It must not normalize validation to false because PostgreSQL core does not verify foreign-table declarations, and it must not copy ordinary-table `NOT VALID` semantics into the foreign-table path.

## Executable traceability

- Validation review finding: `5196715571` on predecessor `5befc54809b500e5a95aae04dd348d6cde88ff6d`.
- Validation RED source contract: `06839f048c4c01a6164aaaf61b5cd861f8b06783`, `crates/conceptweave-observation/tests/not_null_constraint_enforcement_contract.rs::foreign_table_rejects_not_valid_not_null`.
- Validation production repair: `f715dbff95df880f63b28a074a0cf711dfa14ddc`, `crates/conceptweave-observation/src/not_null_constraint.rs::NotNullConstraintObservation::new`.
- Enforcement correction review: `5196804677` on `6696dc8deb0c216b31eaa7ec9ec690401e631e58`.
- Enforcement correction RED: `4a611cc3dc15f4200f65bf08945c9cfa759daa7c`.
- Enforcement correction production repair: `3e9828311a55187d758b9bf729f69870c16f7be9`.

The two repairs compose: foreign-table NOT NULL must be both enforced and valid; ordinary-table NOT VALID support remains separately governed by its own source rules.

## Risk and follow-up

This is a source-representability guard, not evidence that the foreign server itself satisfies the declared constraint. Foreign-table semantic truth remains an observation of PostgreSQL's local catalog declaration. Concrete transport must still obtain all catalog fields from one bounded source snapshot and keep provider-side enforcement outside ConceptWeave's inferred authority.

Exact-head Rust 1.98 and hosted Product/security/review acceptance remain separate gates; source review and committed regression contracts do not substitute for their execution.
