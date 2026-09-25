# PostgreSQL replica-identity mode/index consistency

## Decision

ConceptWeave Source Observation preserves `pg_class.relreplident` and `pg_index.indisreplident` as separate source facts, but when both families are observed it also validates the PostgreSQL cross-catalog invariant between them.

- `relreplident = 'i'` (`ReplicaIdentityMode::Index`) may coexist with zero or one surviving `indisreplident = true` index. Zero is legitimate because PostgreSQL documents that INDEX mode can remain after the chosen index has disappeared.
- An observed `relreplident` of DEFAULT, NOTHING, or FULL must not coexist with a surviving `indisreplident = true` index.
- An unobserved relation mode (`None`) must not be reinterpreted as DEFAULT/NOTHING/FULL and therefore does not activate this cross-family rejection.
- Existing per-index eligibility/cardinality rules remain independent: a surviving chosen index is still bounded to at most one and must satisfy PostgreSQL replica-identity index requirements.

This is validation of simultaneously observed owner evidence, not derivation of one catalog field from another.

## PostgreSQL 18 authority

Authority is pinned to PostgreSQL `REL_18_STABLE` commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc` (2026-09-19).

`src/include/catalog/pg_class.h` defines four relation-level modes: DEFAULT (`d`), NOTHING (`n`), FULL (`f`), and INDEX (`i`). Its INDEX comment explicitly states that the mode may remain set after the chosen index has been dropped, in which case it behaves like NOTHING. That exception is why ConceptWeave must not require an `indisreplident` row whenever mode INDEX is observed.

`src/test/regress/sql/replica_identity.sql` exercises the reciprocal state transition. After selecting a unique non-null index as replica identity, PostgreSQL verifies one `pg_index.indisreplident` row. It then executes `ALTER TABLE ... REPLICA IDENTITY DEFAULT` and checks the count of `indisreplident` rows again, establishing that a non-INDEX relation mode does not retain a chosen-index flag. The same regression file separately exercises FULL and NOTHING as non-index modes and documents the post-drop/invalid-index lifecycle behavior that prevents treating INDEX as equivalent to “one surviving flagged index”.

## ConceptWeave traceability

Current owner seam before the repair:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- predecessor exact head: `b106a63da3fbb66daf280b80c911a43013f88268`
- aggregate validator: `crates/conceptweave-observation/src/lib.rs::validate_schema_relation_invariants`
- relation evidence: `RelationObservation::replica_identity_mode()`
- per-index evidence: `IndexCatalogFlags::replica_identity()`
- existing cardinality/eligibility contracts: `index_replica_identity_cardinality_contract.rs`, `index_replica_identity_contract.rs`
- new focused RED: `relation_replica_identity_mode_index_consistency_contract.rs`
- finding review: `5274536526`

The predecessor validator counts and validates `indisreplident` indexes but does not condition that flag on an observed relation mode. Consequently, contradictory snapshots such as `relreplident = FULL` plus `indisreplident = true` can currently enter governed evidence.

## Repair boundary

The minimal production change belongs in `validate_schema_relation_invariants()` after replica-identity index cardinality is known and before the existing per-index eligibility loop. When `relation.replica_identity_mode()` is observed and is not `ReplicaIdentityMode::Index`, `replica_identity_index_count` must be zero; otherwise return the existing `ObservationError::InvalidObservationField { field: "index_replica_identity" }`.

Do not add the inverse requirement `Index => count == 1`: PostgreSQL explicitly permits INDEX mode after the chosen index is gone. Do not reject `None + indisreplident`: `None` means the relation-mode family was not observed, not that PostgreSQL reported a non-INDEX mode. Do not infer relation mode from indexes, and do not move this rule to semantic-data-portal or context-graph-contracts; it is Source Observation catalog integrity owned by ConceptWeave.

## Acceptance

The focused contract must demonstrate all of the following on one exact head:

1. DEFAULT + surviving `indisreplident` is rejected.
2. NOTHING + surviving `indisreplident` is rejected.
3. FULL + surviving `indisreplident` is rejected.
4. INDEX + one surviving chosen index remains accepted.
5. INDEX + zero surviving chosen indexes remains accepted.
6. unobserved relation mode + surviving chosen index remains accepted.
7. DEFAULT/NOTHING/FULL without a chosen index remain accepted.

Retained replica-identity eligibility/cardinality, NOT NULL-validation, clustered-index, temporal-constraint, digest, and projection contracts must remain unchanged-head GREEN before the finding is resolved.

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source tree: `pg_class` replica identity constants and replica-identity regression tests* (REL_18_STABLE, commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc`). PostgreSQL Global Development Group.
