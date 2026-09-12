# Source Observation — key backing-index lifecycle completeness

## Problem

ConceptWeave may preserve an `IndexObservation` with `ready`, `valid`, and `live` left unobserved. That is acceptable for generic partial index evidence. It is not sufficient when the same index is promoted as the authoritative PostgreSQL backing evidence for an explicitly observed PRIMARY KEY/UNIQUE timing or positive `WITHOUT OVERLAPS` constraint.

PostgreSQL 18 exposes `pg_constraint.conindid` as the supporting-index join coordinate for primary-key, unique, foreign-key, and exclusion constraints. The joined `pg_index` row exposes `indisready`, `indisvalid`, and `indislive` as material boolean catalog facts. `indisvalid = false` means the index can be incomplete and, for a unique index, uniqueness is not guaranteed; `indisready = false` means `INSERT`/`UPDATE` must ignore it; `indislive = false` means it is being dropped and must be ignored for all purposes. Omitting those facts while asserting that the index is usable support therefore leaves the governed claim underdetermined.

The predecessor repair at `ff842b358106c0adeda97fd2405952ff7be029b1` correctly rejects explicitly false lifecycle observations but still accepts `None` through `is_none_or`. Review `5187402940` records the remaining completeness finding. Behavioral RED `9d9e8d9e35f008edf0e3359188723dd94db512c8` covers both owned admission paths: PK/UNIQUE timing and positive temporal-key backing evidence must reject lifecycle-unobserved supporting indexes, while explicit ready + valid + live controls remain admissible.

## Decision

Lifecycle completeness is contextual rather than global.

- Generic standalone index observations may continue to represent lifecycle as unobserved.
- An index used by `key_constraint_backing_index_static_shape_matches()` as authoritative support for PK/UNIQUE timing or positive `conperiod` admission must carry `ready == Some(true)`, `valid == Some(true)`, and `live == Some(true)`.
- A lifecycle flag never creates PRIMARY KEY, UNIQUE, `WITHOUT OVERLAPS`, PERIOD, timing, or operator semantics. Those authorities remain in their owning catalog evidence.
- Catalog OIDs remain bounded-snapshot join coordinates only. Durable semantic identity continues to use exact owned names and versioned observation framing.
- The adapter must capture these three booleans from the same bounded catalog snapshot whenever it claims constraint-backing evidence; it may not synthesize `true` when a field was not collected.

Rejected alternatives are: treating `None` as implicitly healthy; globally requiring lifecycle for every standalone index; or using lifecycle state to infer constraint truth. The first admits unsupported authority, while the latter two collapse optional generic index evidence into a constraint-specific contract.

## Acceptance

The minimum production repair is confined to the shared backing-index predicate: the three lifecycle fields must equal `Some(true)` for constraint-support admission. The timing and positive-period callers must not duplicate that logic. Existing explicit-false REDs, the new unobserved-lifecycle REDs, and explicit all-true controls must pass together on one exact head before this slice is GREEN.

The PostgreSQL adapter remains blocked until the representation stack is accepted. When implemented, its catalog capture must bind `pg_constraint.conindid` to the exact `pg_index` row inside one authorized snapshot and retain the three lifecycle booleans as observed evidence rather than defaults.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: REINDEX*. https://www.postgresql.org/docs/18/sql-reindex.html
