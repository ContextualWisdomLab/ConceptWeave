# PostgreSQL primary-index / primary-constraint reciprocity

## Decision

ConceptWeave must not publish a governed Source Observation in which `pg_index.indisprimary = true` is represented by an `IndexObservation` but the same bounded relation inventory has no matching `TableConstraintObservation::PrimaryKey`.

This is a bidirectional catalog-coherence rule, not a presentation rule. PostgreSQL 18 defines `pg_index.indisprimary` as meaning that the index represents the table primary key, and `pg_constraint` stores primary-key constraints as `contype = 'p'` with `conindid` identifying the supporting index. A primary key is therefore not an independent index flag that can exist without the corresponding constraint row.

## Finding lineage

- Exact reviewed head: `d8005b1497e65a7d6212ddd39407f60ceead4f92`.
- Review: `5261275111`.
- Structural RED: `cd785e46e7cd06313381004f54441d040d849616` adds `crates/conceptweave-observation/tests/index_primary_constraint_reciprocity_contract.rs`.
- The RED requires an orphan primary-flagged index to fail closed as `InvalidObservationField { field: "constraint_backing_index" }` and retains a positive control in which a same-name primary-key constraint and primary catalog index remain admissible.
- Production repair is intentionally not claimed by this document. The current aggregate still validates key-constraint → backing-index coherence more strongly than the inverse primary-index → key-constraint direction.

## Why the existing checks are insufficient

`IndexObservation::with_catalog_flags()` currently checks only the local implication `primary => unique`. That is necessary but not sufficient. `validate_schema_relation_invariants()` validates primary-key cardinality and NOT NULL semantics when a `PrimaryKeyObservation` is present, but a relation can still carry an index whose exact catalog flag says it is the primary index while the relation constraint inventory is empty.

The optional constraint-timing family cannot repair this boundary. Timing is additional `pg_constraint.condeferrable` / `condeferred` evidence. The existence and identity of the primary-key constraint are already represented in the base relation constraint inventory, while `indisprimary` is already represented in the base index inventory. Their reciprocity must therefore be enforced before the base snapshot is hashed or receipted.

## Minimal causal repair

At the public Source Observation aggregate completion boundary, inspect every relation index whose observed `catalog_flags().is_some_and(|flags| flags.primary())` is true. Require a same-relation `TableConstraintObservation::PrimaryKey` whose constraint identity resolves to that primary backing index. Reuse the existing `constraint_backing_index` error vocabulary and the existing detailed backing-index equivalence rules rather than creating a second competing definition of primary-key/index shape.

The repair must preserve these distinctions:

- ordinary unique indexes remain legal without a `UNIQUE` constraint because PostgreSQL permits standalone unique indexes;
- a primary index is different: `indisprimary` specifically states that it represents the table primary key;
- PRIMARY KEY `INCLUDE` payload remains legal;
- PostgreSQL 18 `WITHOUT OVERLAPS` primary keys may use GiST, so the inverse rule must not hard-code B-tree as a universal primary-index access method;
- detailed predicate/expression/key-order/lifecycle/timing coherence remains owned by the existing backing-index validation rather than being duplicated here.

## Primary sources

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.26. pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.13. pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 5.5. Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

## Acceptance

This finding remains RED until the production aggregate rejects the orphan primary-index state, the new focused contract and retained constraint/index contracts are GREEN on the same exact head, and the normal ConceptWeave acceptance set runs without transferring predecessor evidence. Documentation movement alone is not execution evidence.
