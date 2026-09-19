# PostgreSQL EXCLUDE operator-kind integrity

## Problem

The ordinary-EXCLUDE lineage already retained stable operator identity, self-commutator, exact implementation procedure, and the independently resolved Boolean result contract. That still left one catalog discriminator normalized away: `pg_operator.oprkind`.

`QualifiedOperatorSignature` intentionally resolves an operator to stable schema/name/left-type/right-type identity. Those normalized operand identities do not prove that the source `pg_operator` row itself was observed as a binary operator. A faulty or synthetic adapter could manufacture two stable operand identities around a row whose raw `oprkind` was not binary and still satisfy the predecessor identity checks.

Review `5227579925` therefore treats raw operator kind as a separate source-integrity fact, not as another spelling of the stable signature.

## PostgreSQL 18 authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.34. `pg_operator`*. https://www.postgresql.org/docs/18/catalog-pg-operator.html

- `oprkind` is a catalog `char`: `b` means infix (“both”) and `l` means prefix (“left”).
- `oprleft` is zero for a prefix operator, so operator kind is not merely a presentation property.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE OPERATOR*. https://www.postgresql.org/docs/18/sql-createoperator.html

- A binary operator has both `LEFTARG` and `RIGHTARG`; a prefix operator has only `RIGHTARG`.
- The implementation object is a function. This remains distinct from the raw operator-row discriminator.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

- An exclusion constraint compares pairs of rows through its listed operators and those operators must be commutative.
- The exclusion constraint is enforced through its associated index. The operator used for one EXCLUDE element is therefore a binary comparison operator, not a prefix operator normalized into a two-operand signature by the adapter.

These are primary PostgreSQL authorities. No empirical claim about performance or prevalence is made.

## Decision

Add a successor after `IndexExclusionConstraintOperatorResultSnapshot` rather than rewriting any issued operator, procedure, commutator, or result digest domain.

`IndexExclusionConstraintOperatorKindSnapshot` owns exactly one independently observed raw `pg_operator.oprkind` per governed `conexclop` position. It:

1. derives the expected exact constraint/key inventory from the result-contract predecessor;
2. rejects missing or duplicate coordinates;
3. requires the repeated stable operator to equal the exact predecessor operator at that position;
4. admits only raw `oprkind='b'`;
5. hashes the predecessor digest, exact constraint coordinate, key position, stable operator signature, and raw discriminator under a new domain separator; and
6. issues provenance only for positions that survived those checks.

The adapter must resolve the exact `conexclop` OID to its `pg_operator` row and read `oprkind` from that same row. It must not infer `'b'` from the fact that two normalized operand types are available.

## Rejected alternatives

**Infer binary state from `QualifiedOperatorSignature`.** Rejected because that would turn a source observation into an adapter assumption. The gap exists precisely because normalized stable identity can be fabricated independently of the raw row discriminator.

**Extend `QualifiedOperatorSignature` and rewrite predecessor digests.** Rejected because stable operator identity is already issued throughout the retained Source Observation stack. A successor preserves backward semantic meaning and makes the new fact explicit.

**Accept every known `oprkind` and merely record it.** Rejected because ordinary EXCLUDE semantics require a binary comparison operator. Recording `l` without failing would preserve contradictory source state as governed ordinary-EXCLUDE evidence.

## RED and repair lineage

- Review finding: `5227579925` on exact predecessor `17a697f8b658d90f5c361a1abc489e4174df1429`.
- Structural source/compile RED: `3ddf0b09c50b8f1a60a677a4fe4be268ae679120`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_kind_contract.rs`. The test referenced the new successor before production types existed; no executed compiler failure is claimed.
- Production successor: `69c180a37ce2ec0d34070733806ab5419c081375`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_kind.rs`.
- Public composition: `da7567871d61f7bb2f4ba284a62660c4d5cd2928`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

Focused coverage includes normal binary provenance, prefix `l`, unknown discriminator, operator-binding drift, missing evidence, duplicate coordinate, zero position, and unknown receipt position.

## Acceptance and live differential

Source repair is not execution evidence. One unchanged exact head still has to pass the repository-pinned Rust 1.98 formatter, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc/coverage, and applicable hosted gates.

The bounded PostgreSQL 18 differential must resolve each exact `conexclop` row and collect `oprkind` independently alongside retained `oprcom`, `oprcode`, `oprresult`, result `pg_type`, implementation `pg_proc.prorettype`, operator-family/strategy, and backing-index evidence. Positive control is raw `b` on the governed operator row. Negative control is raw non-binary state in synthetic/corrupt source evidence; the test must not pretend PostgreSQL normal DDL would create an invalid ordinary EXCLUDE row.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review: `5227579925`
- Production API: `IndexExclusionConstraintOperatorKindObservation`, `IndexExclusionConstraintOperatorKindSnapshot`, `IndexExclusionConstraintOperatorKindSourceReceipt`
- Test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_kind_contract.rs`
- Predecessor API: `IndexExclusionConstraintOperatorResultSnapshot`
- Catalog fact: `pg_operator.oprkind`
