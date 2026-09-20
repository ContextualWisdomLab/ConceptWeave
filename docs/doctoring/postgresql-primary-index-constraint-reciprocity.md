# PostgreSQL primary-index / primary-constraint reciprocity

## Decision

ConceptWeave must not publish a governed Source Observation in which `pg_index.indisprimary = true` is represented by an `IndexObservation` but the same bounded relation inventory has no matching `TableConstraintObservation::PrimaryKey`.

This is a bidirectional catalog-coherence rule, not a presentation rule. PostgreSQL 18 defines `pg_index.indisprimary` as meaning that the index represents the table primary key, and `pg_constraint` stores primary-key constraints as `contype = 'p'` with `conindid` identifying the supporting index. A primary key is therefore not an independent index flag that can exist without the corresponding constraint row.

## Finding and repair lineage

- Initial reviewed head: `d8005b1497e65a7d6212ddd39407f60ceead4f92`.
- Review: `5261275111`.
- Structural RED: `cd785e46e7cd06313381004f54441d040d849616` adds `crates/conceptweave-observation/tests/index_primary_constraint_reciprocity_contract.rs`.
- Retained catalog-fixture coherence: `571ba1faa873e80e2a16b44f1f9d88ce4880ec96` gives only the `primary=true` catalog-flag material-identity fixture its matching primary-key constraint; non-primary flag variants remain constraint-free.
- Contract refinement `38ad8fa64c730d292d7d382ba9c628fe391f3448` requires a differently named primary key to fail and preserves standalone non-primary unique indexes without a `UNIQUE` constraint.
- Contract refinement `baac84411c4c1d4e34d52cc4409955261309c585` requires a same-name primary key with the wrong ordered key shape to fail.
- Base-snapshot lifecycle control `abc26ac3b255ee43d9c1a82edb9a366ee6720168` proves exact name/key reciprocity must remain admissible when `ready`, `valid`, and `live` lifecycle evidence is unobserved.
- Diagnostic-only normalization `de1895e1c12d3a567adccb328a4f6f056f7248bf` is the exact predecessor reviewed in COMMENT review `5261567427`.
- Production repair `5789cfa1fd8707528edbfe9e7f4133b853a464ff` adds the inverse primary-index check to `validate_schema_relation_invariants()`. The exact predecessor comparison is one ordinary-forward commit changing only `crates/conceptweave-observation/src/lib.rs`, with 24 additions and no deletions.

## Why the existing checks were insufficient

`IndexObservation::with_catalog_flags()` checks only the local implication `primary => unique`. That is necessary but not sufficient. Before `5789cfa1...`, `validate_schema_relation_invariants()` validated primary-key cardinality and NOT NULL semantics when a `PrimaryKeyObservation` was present, but a relation could still carry an index whose exact catalog flag said it was the primary index while the relation constraint inventory was empty.

The optional constraint-timing family cannot repair this boundary. Timing is additional `pg_constraint.condeferrable` / `condeferred` evidence. The existence and identity of the primary-key constraint are already represented in the base relation constraint inventory, while `indisprimary` is already represented in the base index inventory. Their reciprocity must therefore be enforced before the base snapshot is hashed or receipted.

## Causal repair and ownership split

The base aggregate now inspects every relation index whose observed `catalog_flags().is_some_and(|flags| flags.primary())` is true. It requires a same-relation `TableConstraintObservation::PrimaryKey` with the exact same constraint/index name and exact ordered key-column shape. A mismatch fails closed through the existing `constraint_backing_index` error vocabulary.

The base seam intentionally does **not** call `key_constraint_backing_index_static_shape_matches()`. That helper is stronger because it also requires predicate/null-treatment/catalog-primary coherence plus `ready`, `valid`, and `live` lifecycle evidence and is used by timing/PERIOD validation. Importing those requirements into the base snapshot would reject a coherent catalog snapshot merely because optional lifecycle evidence was not observed yet.

The repair preserves these distinctions:

- ordinary unique indexes remain legal without a `UNIQUE` constraint because PostgreSQL permits standalone unique indexes;
- a primary index is different: `indisprimary` specifically states that it represents the table primary key;
- PRIMARY KEY `INCLUDE` payload remains legal because the base reciprocity compares only ordered key attributes, not included payload attributes;
- PostgreSQL 18 `WITHOUT OVERLAPS` primary keys may use GiST, so the inverse rule does not hard-code B-tree;
- predicate, null-treatment, lifecycle, timing, exclusion and PERIOD completeness remain owned by their existing stronger backing-index validation seams rather than being duplicated in the base aggregate.

## Primary sources

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.26. pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.13. pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 5.5. Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

## Acceptance

The source defect is repaired at `5789cfa1fd8707528edbfe9e7f4133b853a464ff`, but execution GREEN is not inferred from source inspection. Acceptance still requires the six focused reciprocity boundaries and retained constraint/index contracts to execute GREEN on the same exact head, followed by the normal ConceptWeave Rust 1.98, strict Clippy, workspace/doc tests, release/rustdoc/owned coverage, and PostgreSQL 18 same-generation differential evidence. Predecessor checks do not transfer.
