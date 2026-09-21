# PostgreSQL PRIMARY KEY backing-index predicate integrity

## Decision

ConceptWeave must reject a relation observation when an index claims `pg_index.indisprimary=true` while also carrying a non-empty partial-index predicate (`pg_index.indpred`). This is a base catalog-coherence invariant and does not depend on optional `indisready`, `indisvalid`, `indislive`, PRIMARY KEY/UNIQUE timing, or PERIOD evidence.

## PostgreSQL 18 authority

Primary source: PostgreSQL `REL_18_STABLE` at commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc`, `src/backend/parser/parse_utilcmd.c`.

When PostgreSQL converts an existing index into a PRIMARY KEY or UNIQUE constraint, it checks `RelationGetIndexPredicate(index_rel) != NIL` and rejects the operation with `Cannot create a primary key or unique constraint using such an index.` The provider therefore does not admit a same-generation state in which a PRIMARY KEY backing index is partial.

This constraint is narrower than generic partial-index validity. A standalone `CREATE UNIQUE INDEX ... WHERE ...` remains valid PostgreSQL state; only an index that claims PRIMARY KEY ownership is constrained here.

## ConceptWeave trace

Canonical aggregate owner: `crates/conceptweave-observation/src/lib.rs::validate_schema_relation_invariants`.

Focused contract: `crates/conceptweave-observation/tests/index_primary_partial_predicate_contract.rs`.

The contract keeps three cases separate:

- `indisprimary=true`, matching same-name/same-key PRIMARY KEY, predicate present: reject as `constraint_backing_index`.
- `indisprimary=true`, matching same-name/same-key PRIMARY KEY, no predicate, lifecycle evidence unobserved: admit.
- ordinary unique index with a predicate and no PRIMARY KEY claim: admit.

The existing optional timing/PERIOD path already calls `key_constraint_backing_index_static_shape_matches()`, which rejects predicates, but the base constructor does not consume that optional family. Base PRIMARY KEY reciprocity therefore needs its own static predicate check and must not import lifecycle or timing requirements.

## Rejected alternatives

Rejecting all partial unique indexes would contradict valid PostgreSQL `CREATE UNIQUE INDEX ... WHERE ...` state. Requiring `ready/valid/live` in the base constructor would collapse optional lifecycle evidence into an unrelated catalog-coherence invariant. Moving this rule into `IndexCatalogFlags::new()` would also be order-sensitive because `IndexObservation::with_predicate()` and `with_catalog_flags()` are independent builders.

## Acceptance

The minimal production repair is at the existing primary-index reciprocity seam: a matching `indisprimary=true` index is admissible only when the PRIMARY KEY name and ordered key columns match and `predicate().is_none()`. Focused RED must turn GREEN without changing the standalone partial-unique positive control or the lifecycle-unobserved PRIMARY KEY positive control. Retained primary reciprocity, replica-identity, clustered-index, workspace, doc, and release tests then run on the same exact head.
