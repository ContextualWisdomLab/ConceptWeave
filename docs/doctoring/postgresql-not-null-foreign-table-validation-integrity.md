# PostgreSQL 18 foreign-table NOT NULL validation integrity

## Decision

ConceptWeave must reject a PostgreSQL 18 foreign-table `NOT NULL` observation when `pg_constraint.convalidated = false`.

This rule is deliberately narrower than the ordinary-table rule. PostgreSQL 18 permits ordinary-table `NOT NULL` constraints to carry `NOT VALID`, while `ALTER FOREIGN TABLE ... ADD table_constraint [ NOT VALID ]` states that `NOT VALID` is allowed only for the `CHECK` case. Foreign-table `NOT NULL` constraints may independently preserve `ENFORCED` or `NOT ENFORCED`; validation and enforcement are not collapsed into one flag.

## Problem

`NotNullConstraintObservation::new()` admitted `RelationKind::ForeignTable` with `validated = false`. Because `validated` participates in the NOT NULL digest, a caller could mint an immutable governed identity for a catalog tuple that PostgreSQL 18's supported foreign-table DDL cannot create.

The preceding foreign-table enforcement repair correctly preserved both `conenforced` states, but that does not imply the same state space for `convalidated`.

## Primary authority

PostgreSQL Global Development Group. (2026). *ALTER FOREIGN TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-alterforeigntable.html

The command supports `ADD table_constraint [ NOT VALID ]`, currently for `CHECK` and `NOT NULL`, and explicitly states that `NOT VALID` is allowed only for `CHECK`.

PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-createforeigntable.html

The foreign-table grammar permits `NOT NULL` with `ENFORCED | NOT ENFORCED` but has no `NOT VALID` form. The notes also distinguish PostgreSQL core enforcement from declared constraint state.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html

The release notes separately record ordinary-table `NOT NULL` support for `NOT VALID`; that broader capability must not be projected onto `ALTER FOREIGN TABLE` contrary to the foreign-table command contract.

## Rejected alternatives

Treating all relation kinds alike is rejected because PostgreSQL 18 exposes different DDL state spaces for ordinary and foreign tables. Rejecting every foreign-table `conenforced = false` is also rejected because `CREATE FOREIGN TABLE` explicitly permits `NOT ENFORCED`. Inferring `convalidated` from `conenforced` is rejected because the catalog fields represent distinct semantics.

## Source boundary

`NotNullConstraintObservation::new()` remains the fail-closed admission point. After relation-kind admission and enforcement-state validation, `RelationKind::ForeignTable && !validated` fails with `not_null_constraint_foreign_validation` before any immutable snapshot digest can be produced.

The adapter must capture `convalidated` and `conenforced` independently from the same bounded catalog snapshot. It must not normalize validation to false because PostgreSQL core does not enforce foreign-table constraints, and it must not copy ordinary-table `NOT VALID` semantics into the foreign-table path.

## Executable traceability

- Review finding: `5196715571` on predecessor `5befc54809b500e5a95aae04dd348d6cde88ff6d`.
- RED source contract: `06839f048c4c01a6164aaaf61b5cd861f8b06783`, `crates/conceptweave-observation/tests/not_null_constraint_enforcement_contract.rs::foreign_table_rejects_not_valid_not_null`.
- Minimal production repair: `f715dbff95df880f63b28a074a0cf711dfa14ddc`, `crates/conceptweave-observation/src/not_null_constraint.rs::NotNullConstraintObservation::new`.
- Production delta from RED: one file, eight added lines, zero deletions.

## Risk and follow-up

This is a source-representability guard, not evidence that the foreign server itself satisfies the declared constraint. Foreign-table semantic truth remains an observation of PostgreSQL's local catalog declaration. Concrete transport must still obtain all catalog fields from one bounded source snapshot and keep provider-side enforcement outside ConceptWeave's inferred authority.

Exact-head Rust 1.98 and hosted Product/security/review acceptance remain separate gates; source review and committed regression contracts do not substitute for their execution.
