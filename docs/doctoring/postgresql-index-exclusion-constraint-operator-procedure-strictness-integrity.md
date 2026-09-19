# PostgreSQL ordinary EXCLUDE operator procedure strictness integrity

## Decision

ConceptWeave must preserve raw `pg_proc.proisstrict` for every exact ordinary `EXCLUDE` operator implementation procedure resolved through `pg_operator.oprcode`.

This is an observation requirement, not a new PostgreSQL DDL validity rule. Both `proisstrict=true` and `proisstrict=false` remain representable. Missing, duplicate, or operator/procedure-misaligned strictness evidence fails closed; the two Boolean states must produce different governed successor digests.

## Problem

The predecessor chain already proves the exact `conexclop` operator, its exact `oprcode -> pg_proc` implementation procedure, Boolean result identity, binary operator kind, and scalar cardinality (`proretset=false`). It did not preserve `pg_proc.proisstrict`.

That omission is material for lossless semantic observation. PostgreSQL stores `proisstrict` independently from `proretset` and `prorettype`. A strict function returns null whenever an input is null and PostgreSQL need not call the function in that case; a non-strict function is allowed to inspect null arguments. Two otherwise identical procedure signatures therefore cannot be normalized to one semantic identity merely because both are scalar Boolean functions.

The exclusion executor makes the distinction especially important. PostgreSQL 18 `index_recheck_constraint()` explicitly comments that it assumes exclusion operators are strict before it short-circuits an existing NULL index value instead of invoking the exclusion procedure. The ordinary `EXCLUDE` creation path, however, explicitly validates commutativity and operator-family compatibility without establishing raw `proisstrict=true` as a separate catalog admission gate. ConceptWeave must therefore retain the source fact without pretending PostgreSQL rejects every non-strict operator definition.

## Contract

`IndexExclusionConstraintOperatorProcedureStrictnessSnapshot` is layered directly over `IndexExclusionConstraintOperatorProcedureScalarSnapshot`.

For every exact constraint/key position in the scalar predecessor it requires exactly one observation containing:

- the same `IndexExclusionConstraintCoordinate`;
- the same one-based key position;
- the same stable `QualifiedOperatorSignature`;
- the same exact `QualifiedProcedureSignature` resolved from `pg_operator.oprcode`;
- independently observed raw `pg_proc.proisstrict`.

The successor digest domain includes the scalar predecessor digest, exact coordinate and key position, stable operator/procedure signatures, and the raw strictness Boolean. `true` and `false` therefore remain distinguishable. No predecessor digest domain is rewritten.

The layer deliberately does **not** reject `proisstrict=false`. PostgreSQL source establishes an executor assumption, not a separate creation-time strictness gate in the ordinary EXCLUDE path. Enforcing `true` here would make ConceptWeave stricter than the observed source system and would erase a catalog state that PostgreSQL itself can represent.

## Failure modes

The snapshot fails closed when raw strictness evidence is missing, duplicated for one exact coordinate/key position, or bound to a different operator/procedure. A zero key position is invalid. Provenance receipts are issued only for observed positions.

A non-strict value by itself is not a validation failure. It is preserved as governed evidence so downstream analysis can distinguish a PostgreSQL executor assumption from catalog-declared function behavior.

## Live differential

The PostgreSQL 18 differential must follow each exact `pg_constraint.conexclop` OID to its `pg_operator` row, resolve `oprcode` to the exact `pg_proc` row, and read `proisstrict` directly from that row in the same bounded source-content generation as the retained operator/result/cardinality evidence. The adapter must not infer strictness from operator name, operator family, access method, Boolean return type, `proretset`, or the executor's NULL short-circuit.

Positive controls must include a strict implementation. A second control must demonstrate that an independently observed non-strict implementation remains distinguishable rather than being normalized to strict state. Catalog mutation or fabricated PostgreSQL validity claims are not required for this contract.

## Traceability

- ConceptWeave review: `5228673154` on PR #46 exact predecessor `a1c23696f6db6faf6780446ecfac965ae309948d`.
- Structural source/compile RED: `413971a472e4393a4b0e6089569ba2b991c3ae65`.
- Production successor: `b3348b9f62942432b5f3ecb376511677c47092fb`.
- Public composition: `4e9b5dd2257ac314e68998a9c5be9291d04376e1`.
- Focused contract: `2ccd5b10889c2f081a182c24341e75161fe0c61e`.
- PostgreSQL source authority: `REL_18_STABLE@1ac292cb1436c788fb6ea29551b0fe459e2cb340`, `src/backend/executor/execIndexing.c` (`index_recheck_constraint`) and `src/backend/commands/indexcmds.c` (`ComputeIndexAttrs`).
- ConceptWeave modules: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_strictness.rs` and `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_strictness_contract.rs`.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source code, REL_18_STABLE* (Commit `1ac292cb1436c788fb6ea29551b0fe459e2cb340`). https://github.com/postgres/postgres/tree/1ac292cb1436c788fb6ea29551b0fe459e2cb340
