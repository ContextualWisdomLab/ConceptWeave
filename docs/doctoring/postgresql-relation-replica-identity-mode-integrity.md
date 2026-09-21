# PostgreSQL relation replica-identity mode integrity

## Decision status

Proposed. This document records a Source Observation P1 found on ConceptWeave PR #46 predecessor `c3c294e40fd1dc46083379906a2b822a6e1a3f17`. Review `5272182113` precedes structural RED `ff73ccea344e8f6ecd567f44a40195f8aa0c5e30`. Production remains RED until relation-level `pg_class.relreplident` evidence is represented and framed into the governed v3 snapshot digest.

## Problem

ConceptWeave v3 currently retains per-index `pg_index.indisreplident` through index catalog flags but does not retain the owning relation's `pg_class.relreplident`. `RelationObservation` therefore cannot distinguish PostgreSQL's four relation-level replica-identity modes when surviving index evidence is otherwise identical.

This is not safely derivable from `indisreplident`. PostgreSQL 18 defines:

- `d` — default replica identity;
- `n` — no replica identity;
- `f` — all columns (`FULL`);
- `i` — an explicitly chosen candidate key.

The PostgreSQL source additionally states that `REPLICA_IDENTITY_INDEX ('i')` remains set if the chosen index has been dropped; in that state it has the same runtime meaning as `n`. Therefore `relreplident='n'` and post-drop `relreplident='i'` may both have no surviving `indisreplident` index while remaining distinct source catalog states. Reconstructing relation mode from the index set would falsify source evidence.

## Source authority

PostgreSQL `REL_18_STABLE`, `src/include/catalog/pg_class.h`, stores `relreplident` as a non-null `char` and defines `REPLICA_IDENTITY_DEFAULT`, `REPLICA_IDENTITY_NOTHING`, `REPLICA_IDENTITY_FULL`, and `REPLICA_IDENTITY_INDEX`. The source comment on `REPLICA_IDENTITY_INDEX` explicitly preserves the post-index-drop state.

`src/include/catalog/pg_index.h` separately defines `indisreplident` as whether a particular index is the identity for replication. The two facts therefore have different owners and cardinalities: one relation-level mode and zero-or-more observed index rows, with normal PostgreSQL operation ordinarily selecting at most one index but index-drop history preventing inverse reconstruction of the relation mode.

The PostgreSQL 18 regression surface `src/test/regress/expected/replica_identity.out` independently demonstrates relation `relreplident` transitions across DEFAULT, USING INDEX, FULL, and NOTHING and verifies the selected index through `pg_index.indisreplident` as a separate query.

## ConceptWeave seam

At structural RED `ff73ccea344e8f6ecd567f44a40195f8aa0c5e30`:

- `crates/conceptweave-observation/src/representation_v3.rs::RelationObservation` stores schema name, relation name, relation kind, columns, constraints, indexes, and comment, but no relation replica-identity mode;
- `compute_snapshot_digest_v3()` hashes relation kind, comment, columns, constraints, and indexes, including per-index replica-identity flags, but cannot hash `pg_class.relreplident` because the relation observation does not carry it;
- consequently two exact PostgreSQL catalog snapshots that differ only in relation-level replica-identity mode can collapse to the same governed ConceptWeave v3 snapshot identity.

## Required contract

The bounded repair should introduce an explicit relation-level value object with the four PostgreSQL 18 modes and retain unobserved state separately during migration. `RelationObservation` must expose a bounded attachment/access path and `compute_snapshot_digest_v3()` must frame observed mode into immutable source identity.

The first repair must **not** infer the relation mode from `indisreplident`, and it must **not** reject `mode = Index` merely because no observed index carries `indisreplident=true`. PostgreSQL explicitly permits that post-drop state. Cross-family validation may later reject catalog states proven impossible by PostgreSQL 18, but only with source authority for each direction and without erasing valid historical/transitional states.

## Alternatives considered

### Derive relation mode from index flags

Rejected. It cannot distinguish DEFAULT, NOTHING, or FULL when no identity index is selected, and it misclassifies the explicitly documented post-drop INDEX state.

### Treat relation mode as presentation-only metadata

Rejected. `relreplident` changes logical replication semantics and is source catalog truth. Omitting it from the governed digest permits semantically distinct source snapshots to share identity.

### Require one identity index whenever mode is INDEX

Rejected for the initial invariant. PostgreSQL's own `pg_class.h` comment says INDEX mode may remain after the index is dropped and then behaves like NOTHING.

### Preserve mode as first-class relation evidence

Selected. It keeps owner boundaries faithful: `pg_class.relreplident` remains relation truth; `pg_index.indisreplident` remains index truth. Neither is synthesized from the other.

## Risk and effect

Until repaired, ConceptWeave can publish a source snapshot whose immutable digest does not commit to an observed logical-replication identity mode. A buyer or downstream validation process could therefore receive identical governed identity for relations that PostgreSQL would treat differently under logical replication. The error is source-fidelity and governance-significant even when no replication operation is executed inside ConceptWeave.

The repair changes digest material and must therefore follow the repository's versioning/reproducibility rules. Existing v3 receipts cannot be silently reinterpreted after mode framing changes; migration or digest-domain consequences must be handled explicitly by the owner.

## Verification order

1. keep review `5272182113` as predecessor finding evidence;
2. make structural RED `ff73ccea344e8f6ecd567f44a40195f8aa0c5e30` GREEN with a bounded relation-level representation;
3. add behavioral coverage proving DEFAULT/NOTHING/FULL/INDEX remain distinguishable and unobserved is not conflated with any source value;
4. add digest coverage proving otherwise-identical relations with different observed modes produce different governed snapshot digests;
5. add a post-drop negative control proving INDEX mode without a surviving identity index remains representable;
6. run retained index/replica/clustered/NOT NULL contracts on one unchanged exact head;
7. update `docs/product-technical-gap-baseline.md` and `CHANGELOG.md`, then obtain fresh exact-source review before Ready/merge/release.

## Traceability

| Evidence | Exact location |
| --- | --- |
| Finding review | ConceptWeave PR #46 review `5272182113`, predecessor `c3c294e40fd1dc46083379906a2b822a6e1a3f17` |
| Structural RED | `ff73ccea344e8f6ecd567f44a40195f8aa0c5e30`, `crates/conceptweave-observation/tests/relation_replica_identity_mode_contract.rs` |
| Current owner seam | `crates/conceptweave-observation/src/representation_v3.rs::RelationObservation` and `compute_snapshot_digest_v3()` |
| PostgreSQL relation authority | `postgres/postgres`, `REL_18_STABLE`, `src/include/catalog/pg_class.h` |
| PostgreSQL index authority | `postgres/postgres`, `REL_18_STABLE`, `src/include/catalog/pg_index.h` |
| PostgreSQL behavioral regression | `postgres/postgres`, `REL_18_STABLE`, `src/test/regress/expected/replica_identity.out` |

## References

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 source: pg_class catalog and replica-identity constants* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_class.h

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: pg_index catalog* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_index.h

PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 regression expectations: replica identity* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/test/regress/expected/replica_identity.out
