# PostgreSQL index-partition collation integrity

Status: source RED and minimal production repair committed; exact-head native/hosted acceptance pending.

## Problem

PostgreSQL 18 `ALTER INDEX ... ATTACH PARTITION` delegates definition equivalence to `CompareIndexInfo()`. After access method and mapped attribute checks, each key slot must carry the same collation. A direct `pg_inherits` index edge whose parent and child key collations differ is therefore source-impossible.

ConceptWeave already preserves per-key `QualifiedCollationName` in `IndexKeySemantics`, so this contradiction can be rejected without widening the representation or comparing reconstructed DDL text.

## Traceability

- Canonical owner PR: ConceptWeave #46.
- Reviewed predecessor: `d41f30312120555bf466ed8980cf23793faf83b2`.
- Review finding: `5199923125`.
- Behavioral source RED: `7380cbc663e5f643fa5409f5d49eb314113c2531`.
- RED contract: `crates/conceptweave-relation-partition/tests/index_partition_definition_collation_contract.rs`.
- Minimal production repair: `af2e6ad76d08e069127236e47b342f480275539a`.
- Fixture realism correction: `4ab3657e28cb312a6812b6142a783f1849868045` replaces the initial non-collatable `bigint/int8_ops` fixture with a collatable `text/text_ops` key, so the RED isolates collation mismatch rather than encoding a PostgreSQL-impossible collation on a non-collatable type.
- Production seam: `crates/conceptweave-relation-partition/src/index_partition.rs::validate_modeled_index_key_collations`.

## Repair

For every admitted direct index-partition edge, ConceptWeave resolves the already-complete key semantic arrays on parent and child and fails closed when any corresponding `collation()` coordinate differs. This occurs after modeled attribute mapping and before immutable digest construction.

This repair does not compare operator class as a proxy for PostgreSQL's operator-family check. The representation currently stores operator class but not operator family. Expression-tree, partial-predicate, and exclusion semantic equality remain open as well.

The initial collation test commit used `bigint/int8_ops` while supplying a collation. That fixture was unsuitable because integer keys are non-collatable in PostgreSQL. The ordinary-forward correction to `text/text_ops` preserves the intended causal contrast (`C` versus `POSIX`) and the matching positive control without changing production behavior.

## Acceptance

The corrected RED and causal source repair are committed evidence, not an executed Rust RED→GREEN claim. No Ready transition, merge, semantic publication, or release is authorized until one unchanged exact head passes repository-pinned Rust 1.98 and applicable hosted Product/security/dependency/review gates.

## Primary source

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/catalog/index.c`, `CompareIndexInfo()`* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c
