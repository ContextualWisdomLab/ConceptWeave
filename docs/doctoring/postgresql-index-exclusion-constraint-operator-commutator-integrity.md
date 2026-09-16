# PostgreSQL 18 ordinary EXCLUDE operator-commutator integrity

## Decision

ConceptWeave must preserve an independently resolved PostgreSQL operator commutator for every ordinary `pg_constraint.contype = 'x'` exclusion-key position and must reject governed evidence unless that commutator resolves to the exact same stable operator signature already governed from the corresponding `pg_constraint.conexclop` position.

Capture-time OIDs remain adapter-local join coordinates. Governed identity uses the resolved operator schema, operator name, left operand type, and right operand type. Commutativity is not inferred from an operator name, operator family, strategy number, or underlying procedure.

## Problem

Before review `5224623318`, the Source Observation stack retained both the ordered constraint-side `conexclop` vector and independently reconstructed backing-index exclusion operator/procedure/strategy semantics. It did not retain the `pg_operator.oprcom` edge for the selected operator.

That omission mattered because PostgreSQL does not admit an arbitrary operator merely because it belongs to a suitable operator family. For an exclusion constraint, PostgreSQL checks that the selected operator's commutator is the operator itself. Without preserving the independently observed commutator edge, an otherwise coherent source tuple could carry a non-self-commutative operator through ConceptWeave's governed identity.

## PostgreSQL authority

Pinned PostgreSQL authority for this stack is `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` (`REL_18_STABLE`). In `src/backend/commands/indexcmds.c`, exclusion-index attribute construction resolves each selected operator and rejects it when `get_commutator(opid) != opid`; PostgreSQL reports that only commutative operators can be used in exclusion constraints.

The PostgreSQL 18 `CREATE TABLE` documentation states the same contract directly: operators specified by an `EXCLUDE` constraint are required to be commutative. The source check is stronger than a name-based assumption because it evaluates the catalog operator identity selected for the exact exclusion key.

## Chosen contract

`IndexExclusionConstraintOperatorCommutatorSnapshot` is a domain-separated successor over the exact `IndexExclusionConstraintOperatorSnapshot` predecessor.

For every key position of every ordinary EXCLUDE operator observation, it requires exactly one `IndexExclusionConstraintOperatorCommutatorObservation` containing the exact constraint coordinate, one-based key position, the stable operator signature repeated from the independently governed `conexclop` position, and the independently resolved stable signature reached through `pg_operator.oprcom`.

Snapshot construction derives the complete expected coordinate/position inventory from the operator predecessor, rejects missing or duplicate positions, proves the supplied operator signature is the exact predecessor operator for that position, and then requires the resolved commutator signature to equal that operator signature. The successor digest includes the predecessor digest, exact coordinate, key position, operator signature, and commutator signature. Existing predecessor digest domains remain unchanged. Provenance is issued below the exact operator position at `/exclusion-operators/{position}/commutator`.

## Rejected alternatives

A built-in operator-name allowlist was rejected because extension operators are valid PostgreSQL operators and names do not prove catalog commutator identity. Inferring commutativity from operator-family membership, strategy number, or the underlying procedure was rejected because those are separate catalog facts. Deriving the commutator by copying the governed `conexclop` value was rejected because it would turn validation into circular reconstruction. Persisting raw OIDs in semantic identity was rejected because OIDs are database-local capture coordinates rather than portable semantic identifiers.

## Contract evidence

Review finding: `5224623318` on predecessor `0578d70e684e8bf306b0642131aceca5ba16617b`.

Ordinary-forward implementation lineage:

- structural source/compile contract: `8e6fb2df3b97a18605c32dc37ae91cc3b6f74f18`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_commutator_contract.rs`;
- production successor: `d3b2b1433314afa5ea5e320d42323532d52d64b7`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_commutator.rs`;
- public composition: `14a4776dd4b66a526abdfc3ab28b1987103ea42a`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

The focused contract covers a self-commutative positive case, a non-self-commutative negative case, operator/predecessor binding drift, missing evidence, duplicate coordinate/position evidence, invalid zero key position, provenance issuance, and unknown provenance positions.

No executed Rust RED/GREEN is claimed by this document. At the structural RED commit the new public types did not yet exist, so the contract was structurally compile-red. Native and hosted acceptance still require one unchanged exact head to pass repository-pinned Rust 1.98 gates and applicable central workflow/review/security gates.

## Live differential requirement

The PostgreSQL 18 live differential must read the ordinary EXCLUDE `conexclop` OID vector and independently follow each selected operator row's `pg_operator.oprcom` edge within the same bounded source observation. Both OIDs must be resolved to stable qualified operator signatures before governance, and equality is checked only after those independent reads.

A self-commutative built-in operator supplies the positive control. Negative coverage should use independent operator-catalog evidence for an operator whose commutator differs from itself rather than fabricating a PostgreSQL EXCLUDE row that PostgreSQL itself would refuse to create. Extension operators should be accepted when their independently observed catalog commutator resolves to themselves; no provider or operator-name allowlist is part of the domain rule.

Missing `oprcom`, unresolved commutator identity, position drift, or inability to bind the commutator to the exact governed `conexclop` operator must fail closed at the adapter/observation boundary.

## Traceability

- Owner bounded context: ConceptWeave Source Observation / relation-index semantic evidence.
- Exact PR: `ContextualWisdomLab/ConceptWeave#46`.
- Production module: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_commutator.rs`.
- Focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_commutator_contract.rs`.
- Direct predecessor: `IndexExclusionConstraintOperatorSnapshot` in `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator.rs`.
- PostgreSQL source authority: `src/backend/commands/indexcmds.c` at pinned commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2025). *PostgreSQL source: indexcmds.c* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/commands/indexcmds.c
