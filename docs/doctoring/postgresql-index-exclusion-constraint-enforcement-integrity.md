# PostgreSQL 18 EXCLUDE constraint enforcement integrity

## Problem

`IndexExclusionConstraintSnapshot` preserves the independent PostgreSQL `pg_constraint` identity for ordinary `EXCLUDE` constraints, including resolved `conindid`, partition constraint parentage, and inheritance state. `IndexExclusionConstraintTimingSnapshot` then preserves `condeferrable` and `condeferred` without rewriting that predecessor digest.

The stack still omitted `pg_constraint.conenforced`. PostgreSQL 18 exposes this bit for every constraint row, but supported DDL permits `NOT ENFORCED` only for `CHECK` and foreign-key constraints. An observed ordinary `contype = 'x'` row with `conenforced = false` is therefore contradictory PostgreSQL 18 source evidence. Without an explicit governed boundary, that impossible state could collapse into the same ConceptWeave identity as an enforced EXCLUDE constraint.

## Constraints

- Preserve every issued v3, relation-partition, index-partition, exclusion-constraint, and exclusion-timing digest domain.
- Do not infer an omitted catalog field from PostgreSQL defaults; extraction completeness must be explicit.
- Keep temporal `PRIMARY KEY`/`UNIQUE ... WITHOUT OVERLAPS` in the existing key-constraint owner family rather than double-counting its exclusion behavior as an ordinary `contype = 'x'` constraint.
- Do not copy source truth into semantic-data-portal or another consumer. ConceptWeave remains the canonical source-observation owner.

## Alternatives considered

1. **Ignore `conenforced` because EXCLUDE is always enforced in supported PostgreSQL 18 DDL.** Rejected: source observation must detect contradictory or corrupted catalog evidence instead of silently normalizing it.
2. **Add `conenforced` to `IndexExclusionConstraintSnapshot` or the timing digest.** Rejected: both domains have already issued governed identities; mutating either would break immutable predecessor semantics.
3. **Add a domain-separated enforcement successor over the exact timing snapshot.** Selected: it preserves predecessor identity, requires complete one-to-one coverage, retains the raw catalog bit, and fails closed on the unsupported false state.

## Decision

`IndexExclusionConstraintEnforcementObservation` records the exact exclusion-constraint coordinate plus raw `pg_constraint.conenforced`. Construction rejects `false` with `index_exclusion_constraint_enforcement_state` because PostgreSQL 18 does not support `NOT ENFORCED` for EXCLUDE constraints.

`IndexExclusionConstraintEnforcementSnapshot` composes over the exact `IndexExclusionConstraintTimingSnapshot` digest. Its inventory must match the timing predecessor exactly, duplicate coordinates fail closed, and its successor digest includes the predecessor digest, deterministic constraint coordinates, and the raw enforcement bit. Exact per-constraint provenance is issued at the `/enforcement` location.

## Exact traceability

- Finding review: `ContextualWisdomLab/ConceptWeave#46` review `5218290892` on `08847df6124f8834ed8b8ec33c9451a2a1474d3e`.
- Source/compile RED contract: commit `83bd71cabfba11239a6f9a78b761eaea94b8cce8`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_enforcement_contract.rs`.
- Production successor: commit `2538bb4ee2b8d97c33d706c9dbc4875da06d0607`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_enforcement.rs`.
- Public module composition: commit `fb32d84c39918974f0ad7500f07d907c8b4891ae`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- PostgreSQL documentation authority: PostgreSQL Global Development Group, *PostgreSQL 18 Documentation: CREATE TABLE*, https://www.postgresql.org/docs/18/sql-createtable.html . The `ENFORCED | NOT ENFORCED` section states that `NOT ENFORCED` is currently supported only for foreign-key and `CHECK` constraints.
- Catalog authority: PostgreSQL Global Development Group, *PostgreSQL 18 Documentation: pg_constraint*, https://www.postgresql.org/docs/18/catalog-pg-constraint.html . `conenforced` records whether a constraint is enforced and `contype = 'x'` denotes an exclusion constraint.
- Release authority: PostgreSQL Global Development Group. (2025). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html . PostgreSQL 18 added `conenforced` together with `NOT ENFORCED` support for `CHECK` and foreign-key constraints.

## Risks and acceptance boundary

This repair is source-contract complete only when the new contract and retained Source Observation suite execute on the same exact head under the repository-pinned Rust 1.98 toolchain. No native or hosted GREEN is inferred from source review alone.

The PostgreSQL live differential must read `contype`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `condeferrable`, `condeferred`, and `conenforced` from the same observed constraint family, prove ordinary EXCLUDE rows are enforced, and retain the temporal key-constraint control so `WITHOUT OVERLAPS` is not double-counted.
