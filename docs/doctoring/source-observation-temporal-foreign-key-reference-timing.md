# PostgreSQL 18 PERIOD foreign-key referenced-key timing

Status: source repaired; exact-head native/Product acceptance pending.

## Problem

ConceptWeave already preserves `pg_constraint.conperiod` separately from PRIMARY KEY/UNIQUE timing. Before this repair, an in-snapshot `PERIOD` foreign key could resolve to an observed `WITHOUT OVERLAPS` key using column identity and period evidence while the referenced key's `condeferrable`/`condeferred` family was unobserved or explicitly deferrable.

That state is not a valid governed PostgreSQL 18 reference. `CREATE TABLE` requires an explicit foreign-key reference column list to resolve to a non-deferrable UNIQUE or PRIMARY KEY constraint (or a non-partial unique index), and a `PERIOD` foreign key additionally requires the referenced table to expose a PRIMARY KEY or UNIQUE constraint declared `WITHOUT OVERLAPS`. PostgreSQL exposes key timing directly as `pg_constraint.condeferrable` and `pg_constraint.condeferred`; it must not be inferred from GiST/exclusion shape.

## Decision

For an in-snapshot `PERIOD` foreign key, ConceptWeave now requires all of the following before immutable governed evidence is accepted:

- exact referenced relation and referenced column identity;
- explicit referenced PK/UNIQUE `conperiod=true` evidence;
- explicit referenced-key `ConstraintTimingObservation` on the same relation/constraint coordinate;
- `ConstraintDeferrability::NotDeferrable` for that referenced key.

Missing timing evidence and either deferrable timing state fail closed with `constraint_period_reference_timing`. Index exclusion/GiST evidence remains a consistency check and is not promoted into timing truth.

This rule is intentionally bounded to references whose target relation is present in the same observed snapshot. Cross-boundary reference evidence remains a separate adapter/contract concern rather than being guessed from incomplete local state.

## Alternatives rejected

Inferring non-deferrability from `pg_index.indimmediate` was rejected because the canonical owner already models `pg_constraint.condeferrable`/`condeferred` as an explicit observed family. Treating an unobserved timing family as PostgreSQL's default `NOT DEFERRABLE` was also rejected because absence of catalog evidence is not observed false/default evidence. Requiring timing for every temporal key, even when no foreign key references it, was rejected as broader than the PostgreSQL reference invariant being repaired here.

## Traceability

- Finding review: `5184299133` on PR #46 predecessor `bff3455e9ddd256a7aa9e1ba7eaa6466151b9e82`.
- Behavioral RED: `6c77cb6fb1664cd7ffeb517ad7bcd85382ebd825`, `constraint_period_reference_timing_contract.rs`.
- RED review: `5184303271`.
- Production repair: `fa21b47653192af83627ac77d14c9141f4419cbd`, `PostgresSchemaSnapshotV3::with_observed_constraint_periods` and `canonicalize_constraint_periods`.
- Retained action fixture repair: `b501b003fbcbfe612f92aa65d83a7fd82cedb68a`.
- Retained period fixture repair: `8b36c7a67f8a90b24ad2f08c02ead23374dc4c94`.
- Acceptance remains pending until repository-pinned Rust 1.98 fmt, strict workspace/all-target Clippy, workspace/doc tests, release build, owned coverage, and applicable Product/security/dependency/review gates are terminal on one unchanged exact head.

## Primary references

PostgreSQL Global Development Group. (n.d.). *CREATE TABLE*. PostgreSQL 18 documentation. Retrieved September 12, 2026, from https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (n.d.). *pg_constraint*. PostgreSQL 18 documentation. Retrieved September 12, 2026, from https://www.postgresql.org/docs/18/catalog-pg-constraint.html
