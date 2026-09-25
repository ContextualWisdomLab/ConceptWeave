# PostgreSQL 18 attached-index definition equivalence integrity

## Problem

`IndexPartitionSnapshot` now rejects a direct index-partition edge when the child and parent disagree on uniqueness. That is necessary but not sufficient for PostgreSQL 18 attachment semantics. `ALTER INDEX ... ATTACH PARTITION` records the direct index parent only after PostgreSQL has established an equivalent index definition. In `REL_18_STABLE`, `CompareIndexInfo()` checks `ii_Unique`, then `ii_NullsNotDistinct`, then the index access method before continuing into attribute mapping, collations, operator families, expressions, predicates, and exclusion properties.

ConceptWeave already models `NULLS NOT DISTINCT` and requires every governed relation-scoped index to carry a nonblank access method. Before this repair, an explicit source edge could therefore claim that a `NULLS DISTINCT` child belonged to a `NULLS NOT DISTINCT` parent, or that a `hash` child belonged to a `btree` parent, while the successor still issued immutable governed identity. Those states contradict PostgreSQL's own attachment predicate.

## Constraint and alternatives

The direct `pg_inherits` edge remains the authoritative source fact that PostgreSQL attached the index. ConceptWeave does not infer attachment from index names, rendered `pg_get_indexdef`, or visual similarity. Once an edge is observed, however, material definition facts already owned by the bounded snapshot cannot contradict the server invariant.

The repair deliberately does not claim full `CompareIndexInfo()` parity. Operator-family identity is not yet represented directly enough to reproduce PostgreSQL's comparison without conflating operator-class names with access-method-specific operator families. Expression and predicate comparison also requires mapped relation coordinates rather than raw text equality. Exclusion semantics need their own modeled operator information. Those remain explicit follow-up work rather than being approximated with strings.

## Decision

For every admitted direct child-index → parent-index edge, `IndexPartitionSnapshot` now requires:

1. identical `is_unique()`;
2. identical observed `nulls_not_distinct()` state;
3. identical observed access-method name.

These checks run after exact parent resolution and kind validation and before the topology digest is computed. The successor does not compare names, comments, tablespaces, readiness/liveness, storage options, or rendered DDL because they are not substitutes for PostgreSQL's definition-equivalence predicate.

## Review, RED, repair

- Finding review: PR #46 review `5199613530` on exact predecessor `d9b6de9709925d7a33502eaff7c5d338232b3543`.
- Behavioral RED source: `b84e49773bc31c140ec6ed0638d75e4f54d75f80` adds focused contracts for a unique parent/child pair with different `NULLS NOT DISTINCT` semantics and a non-unique parent/child pair with different access methods. A matching positive control remains admissible.
- Minimal production repair: `f44973e113dcbacd13ff5554bb464a252025361e` compares the two already-modeled properties on each resolved direct edge and fails closed with field-specific errors before hashing.

No native Rust RED/GREEN is claimed for these commits. The available execution host still lacks the repository-pinned Rust toolchain and protected ConceptWeave `main` still lacks the repository-owned Product pull-request workflow.

## Remaining definition-equivalence gap

Full PostgreSQL equivalence remains open for the portions that are not yet safe to compare with current representation semantics:

- mapped key and INCLUDE attribute identity across parent/child table coordinates;
- collation and operator-family identity;
- expression trees;
- partial-index predicates;
- exclusion operator properties;
- any attachment-relevant definition field added by PostgreSQL that is not yet represented canonically.

The next repair must follow PostgreSQL `CompareIndexInfo()` semantics and attribute mapping. Raw rendered DDL, index names, or source-copy heuristics are not acceptable substitutes.

## Primary references

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER INDEX*. https://www.postgresql.org/docs/18/sql-alterindex.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: `src/backend/catalog/index.c`, `CompareIndexInfo()` (`REL_18_STABLE`)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c
