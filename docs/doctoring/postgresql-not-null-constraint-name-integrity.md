# PostgreSQL 18 NOT NULL constraint-name integrity

## Problem

ConceptWeave models PostgreSQL 18 `NOT NULL` rows as a first-class evidence family because `pg_constraint.contype = 'n'` carries identity and lifecycle state that the column-level `attnotnull` summary cannot preserve. The v3 relation model already canonicalizes CHECK, PRIMARY KEY, UNIQUE, FOREIGN KEY, and other `TableConstraintObservation` names within each owning relation. Before this repair, the separately attached NOT NULL family canonicalized only against other NOT NULL rows.

That separation admitted a source-impossible snapshot: one relation could contain (for example) a CHECK named `metric_required` and a NOT NULL row with the same `conname`, even though both are table constraints owned by the same `conrelid`.

## PostgreSQL 18 authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. PostgreSQL. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

`pg_constraint` stores CHECK, NOT NULL, PRIMARY KEY, UNIQUE, FOREIGN KEY, and exclusion constraints on tables; NOT NULL is therefore not a separate naming namespace merely because ConceptWeave represents it in a separate optional evidence family.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE — Constraint Naming*. PostgreSQL. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL requires constraint names to be unique across the constraints attached to a particular table or domain. The scope is the owning table/domain, not the constraint kind. Index-backed constraints remain additionally constrained by schema-wide relation-name rules for their backing indexes.

## Decision

At NOT NULL family integration, after resolving the exact owning `RelationObservation` and before digest extension, reject any `NotNullConstraintObservation.constraint_name` already present in that relation's `TableConstraintObservation` collection. Reuse the existing `ObservationError::DuplicateConstraintName` so the same relation-wide invariant has one error vocabulary.

The repair intentionally does not require schema-wide uniqueness. PostgreSQL allows a non-index-backed constraint name to repeat on different tables. A companion contract preserves that valid case.

## Alternatives rejected

- **Keep separate naming namespaces by ConceptWeave family.** Rejected because representation boundaries would override source catalog semantics and permit fabricated governed evidence.
- **Require constraint-name uniqueness across the schema.** Rejected because PostgreSQL 18 is intentionally laxer than the SQL standard for table/domain constraints; this would reject valid source state.
- **Normalize or rename one family during capture.** Rejected because exact source identifiers are evidence and must not be rewritten to manufacture consistency.

## Traceability

- Review finding: PR #46 review `5195981967`, exact predecessor `e2f7ca69eb4102865342384dc875596ab147aa74`.
- RED contract: `crates/conceptweave-observation/tests/not_null_constraint_name_contract.rs`, commit `79d91622a403a29028e463ba3d7e8adc88f988ad`.
  - same-relation CHECK/NOT NULL name collision must fail with `DuplicateConstraintName`;
  - the same constraint name on different relations remains valid.
- Production repair: `crates/conceptweave-observation/src/not_null_constraint.rs`, commit `aaf4b3144452a67d938fd12a78b4e6fee6c44b6b`.
- Fixture isolation: commit `0cff8f0a23fc65a66ee5ce9499ea99d9bb101bce` makes the different-relation preservation case keep its CHECK-only relation nullable, so NOT NULL completeness tests only the intended non-null relation instead of introducing an unrelated missing-row failure.
- Integration boundary: `PostgresSchemaSnapshotV3::with_observed_not_null_constraints` → `canonicalize_not_null_constraints`.

## Adapter obligation

The PostgreSQL adapter must preserve exact `conname`, `conrelid`, and constraint kind for every bounded table constraint. It must not pre-deduplicate or rename rows across kinds. ConceptWeave performs the relation-wide collision check after all standard relation constraints and the explicit NOT NULL family are available, so contradictory source evidence fails closed before immutable digest publication.

## Risk and follow-up

This repair closes the cross-family name collision between the existing relation constraint collection and NOT NULL. Any future first-class `pg_constraint` family split out of `TableConstraintObservation` must join the same relation-wide naming invariant rather than establishing a new representation-local namespace.
