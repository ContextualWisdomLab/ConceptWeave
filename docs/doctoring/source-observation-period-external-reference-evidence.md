# Source Observation doctoring — PERIOD external reference evidence

## Problem

A PostgreSQL 18 `FOREIGN KEY (..., PERIOD period_column)` does not derive temporal-reference authority from the local `pg_constraint.conperiod=true` row alone. PostgreSQL requires the referenced table to expose a PRIMARY KEY or UNIQUE constraint declared `WITHOUT OVERLAPS`; when referenced columns are supplied, ordinary reference eligibility also depends on a non-deferrable referenced key contract. ConceptWeave therefore cannot publish a governed PERIOD relationship while omitting the referenced temporal-key evidence that makes the source relationship valid.

The predecessor representation validated this requirement only when the referenced relation happened to be present in the bounded `relations` inventory. If the exact referenced schema/relation was absent, validation fell through and the local PERIOD FK was admitted. That creates two different authority standards for the same source fact: strong verification for in-snapshot references and trust-by-absence for out-of-snapshot references.

## Constraint

The Source Observation aggregate is evidence, not a PostgreSQL parser or a trust proxy for mutable source state. A positive temporal relationship must be self-supporting inside its immutable evidence boundary.

For a PERIOD FK, admission therefore requires one of two designs:

1. the referenced relation, exact referenced PK/UNIQUE columns, positive `WITHOUT OVERLAPS` period observation, and non-deferrable timing evidence are present in the same bounded snapshot; or
2. a future explicit referenced-key evidence family carries an independently captured immutable coordinate with equivalent source authority and provenance.

Until design (2) exists, absence of the referenced relation must fail closed with `constraint_period_reference`. Ordinary non-PERIOD outbound foreign keys remain relationship evidence and are not forced into this stronger temporal completeness rule.

## Rejected alternatives

Relying on `conperiod=true` of the local FK is insufficient because it records the local constraint's PERIOD declaration, not the referenced key's immutable identity, timing, or `WITHOUT OVERLAPS` evidence.

Inferring the referenced key from naming conventions, OIDs, index shape, reconstructed DDL, or PostgreSQL's historical creation-time validation is also rejected. OIDs are capture-local join coordinates, names are not semantic proof, and mutable source state cannot substitute for governed evidence retained in the release candidate.

Automatically widening the observation request to the referenced schema is rejected because source admission and least-privilege scope belong to the authorized request/policy boundary. The adapter may propose a broader follow-up observation, but it cannot silently expand authorization.

## Primary authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. A PERIOD foreign key requires at least one non-PERIOD equality column, a range or multirange PERIOD column, and a referenced PRIMARY KEY or UNIQUE constraint declared `WITHOUT OVERLAPS`; explicit referenced columns must satisfy the referenced-key eligibility rules.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. `conperiod` records `WITHOUT OVERLAPS` for primary/unique keys and `PERIOD` for foreign keys, while `conkey` and `confkey` retain local and referenced column coordinates.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5186323924`
- Behavioral RED: `crates/conceptweave-observation/tests/constraint_period_external_reference_contract.rs`
- RED commit: `d692773a5050b6d5486c40a97d5d8589601fe995`
- Production owner: `crates/conceptweave-observation/src/lib.rs::canonicalize_constraint_periods`
- Required failure field: `constraint_period_reference`
- Positive retained contract: `constraint_period_reference_timing_contract.rs`

## Acceptance

The focused RED is GREEN only when a positive PERIOD FK whose referenced relation is missing from the bounded snapshot fails closed, while the existing fully observed referenced `WITHOUT OVERLAPS` + `NOT DEFERRABLE` path remains accepted and ordinary non-PERIOD outbound FK representation is unchanged. Any future external referenced-key evidence family must be domain-separated, digest-material, provenance-bearing, authorization-bounded, and must not use mutable sibling state or cross-service SQL as authority.
