# PostgreSQL ordinary EXCLUDE implementation-procedure volatility integrity

Status: Proposed / Source Observation evidence contract

## Problem

PR #46 already binds every governed ordinary `EXCLUDE` key position to the exact `pg_operator` signature, self-commutator evidence, exact `pg_operator.oprcode -> pg_proc` implementation procedure, Boolean result identity, binary `oprkind`, scalar `proretset=false`, and raw `proisstrict`. Review `5229099545` found that this chain still omitted `pg_proc.provolatile`.

That omission is material. PostgreSQL 18.6 stores `provolatile` independently from `proisstrict`, `proretset`, and `prorettype`. Its catalog values are `i` (immutable), `s` (stable), and `v` (volatile). These states differ in repeatability, side-effect allowance, optimizer treatment, and MVCC snapshot visibility. Consequently two source catalogs could retain the same currently governed operator/procedure signature and all predecessor procedure fields while differing in volatility, yet collapse to one governed semantic identity.

## Constraints and alternatives

The repair must not infer volatility from procedure names, operator family/strategy, Boolean result type, scalar cardinality, strictness, or built-in conventions. It also must not invent a new ConceptWeave rule that all ordinary exclusion-operator procedures are immutable unless PostgreSQL itself establishes that as the relevant source validity condition.

Three alternatives were considered:

1. Ignore volatility because the exact procedure identity is already retained. Rejected: identity does not preserve mutable catalog state attached to that procedure row.
2. Admit only `IMMUTABLE`. Rejected for this bounded repair: that would turn a source-observation integrity gap into a stronger local DDL validity rule without establishing such a rule as the PostgreSQL ordinary-EXCLUDE admission contract.
3. Preserve the raw catalog discriminator and domain-separate all legal PostgreSQL states. Selected: it is lossless, fail-closed on malformed catalog values, and remains observational.

## Decision

`IndexExclusionConstraintOperatorProcedureVolatilitySnapshot` is layered over the exact `IndexExclusionConstraintOperatorProcedureStrictnessSnapshot` generation. Every predecessor constraint/key position requires exactly one `IndexExclusionConstraintOperatorProcedureVolatilityObservation` carrying:

- the exact `IndexExclusionConstraintCoordinate`;
- the exact one-based key position;
- the same stable `QualifiedOperatorSignature`;
- the same exact `QualifiedProcedureSignature` resolved from `pg_operator.oprcode`;
- raw `pg_proc.provolatile` as one of `i`, `s`, or `v`.

Missing or duplicate positions, operator/procedure binding drift, zero positions, unknown receipt coordinates, and any raw discriminator outside `i|s|v` fail closed. All three valid states are admitted, but they produce different successor digests. Existing predecessor digest algorithms are unchanged.

## Evidence lineage

- reviewed predecessor: `fe69a28d229ee35b721e6b852631d07da9e19072`;
- finding review: `5229099545`;
- structural source/compile RED contract: `3aa7d5a4aeed555cc76fbb0b97ec2c415d104011`;
- minimum production successor: `9b10d980ec45c9d2e6886e4714ab71bb7f94d32a`;
- public composition: `1ca6200860dec62310b87fa4f194b19fe4983af3`.

The RED commit imports the volatility types before they exist. This is source-shaped compile RED evidence, not a claim that `rustc` executed in the current automation host. Native and hosted acceptance remain exact-head obligations.

## Test contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_volatility_contract.rs` pins:

- immutable-state provenance;
- pairwise digest distinction for immutable, stable, and volatile states;
- rejection of unknown raw discriminators;
- exact operator binding;
- exact `oprcode` procedure binding;
- complete position inventory;
- duplicate-coordinate rejection;
- one-based key positions;
- unknown receipt rejection.

## Live PostgreSQL differential

A concrete PostgreSQL 18 adapter must resolve every exact `conexclop` OID to the source `pg_operator` row, read retained operator fields independently, follow that row's `oprcode` to the exact `pg_proc` row, and read `prorettype`, `proretset`, `proisstrict`, and `provolatile` independently from that same bounded source generation. Copying or deriving volatility from normalized signatures or another procedure field is not admissible evidence.

Positive controls should include a known immutable operator implementation. Distinguishability controls must demonstrate that otherwise identical fixture observations with `i`, `s`, and `v` never collapse to one governed digest. Such synthetic states test the evidence contract; they are not claims that PostgreSQL accepts every state for a particular DDL definition.

## Effects and remaining risk

This closes one semantic-loss channel in the ordinary-EXCLUDE procedure chain without changing existing digest meanings. It does not yet preserve every `pg_proc` behavioral/security attribute; future bounded review should examine remaining independent fields such as parallel-safety and leakproof/security semantics before creating another successor. No such future field is implicitly covered by this decision.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 36.7. Function volatility categories*. https://www.postgresql.org/docs/18/xfunc-volatility.html
