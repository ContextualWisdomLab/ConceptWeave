# PostgreSQL replica-identity NOT NULL validation integrity

## Problem

ConceptWeave deliberately separates frozen column nullability (`pg_attribute.attnotnull`) from PostgreSQL 18 first-class `NOT NULL` constraint rows (`pg_constraint.contype = 'n'`). That distinction matters because `attnotnull = true` can coexist with a not-yet-validated NOT NULL constraint.

The base source-observation aggregate must remain valid when the first-class NOT NULL family was not observed. Once that family is explicitly observed, however, accepting `pg_index.indisreplident = true` on a key column whose matching NOT NULL row has `convalidated = false` would grant governed identity to a catalog state PostgreSQL rejects for `REPLICA IDENTITY USING INDEX`.

## Authoritative source trace

PostgreSQL `REL_18_STABLE` was pinned at commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc` for this decision.

In `src/backend/commands/tablecmds.c`, the `ALTER TABLE ... REPLICA IDENTITY USING INDEX` path first verifies that the candidate index is unique, immediate, expression-free, and non-partial. For each index key attribute it then requires `pg_attribute.attnotnull`, resolves the corresponding first-class NOT NULL constraint with `findNotNullConstraintAttnum()`, and rejects the candidate when that constraint has `convalidated = false`.

ConceptWeave already enforces the relation/index-side conditions in `validate_schema_relation_invariants()`. The missing seam is therefore the optional NOT NULL-family canonicalizer, where both the exact replica-identity key coordinates and source-authoritative `convalidated` evidence are available.

## Decision

When `with_observed_not_null_constraints()` is used, every key attribute of every `indisreplident = true` index must resolve to the matching captured NOT NULL constraint and that row must be validated. An observed `convalidated = false` row fails closed with the existing `index_replica_identity` invariant.

This check does **not** make first-class NOT NULL evidence mandatory for the base snapshot. If the adapter did not observe the family, ConceptWeave does not invent `convalidated` state. It also does not reject unvalidated NOT NULL constraints globally: PostgreSQL can represent them, and the validation requirement here belongs specifically to replica-identity eligibility.

## Alternatives rejected

Requiring the NOT NULL family for every replica-identity observation was rejected because it would collapse “unobserved” into “invalid” and break the existing optional-family contract.

Rejecting every unvalidated NOT NULL row was rejected because `NOT VALID` is itself source-authoritative PostgreSQL state for supported relation paths; the stricter prerequisite is imposed by replica identity, not by NOT NULL representation in general.

Adding the check to the base relation/index validator was rejected because `convalidated` is intentionally unavailable there. The canonical NOT NULL-family boundary is the first owner seam where both evidence families are present without source copying or inference.

## Verification contract

`crates/conceptweave-observation/tests/index_replica_identity_not_null_validation_contract.rs` records four boundaries:

- observed unvalidated NOT NULL on a replica-identity key is rejected;
- observed validated NOT NULL on a replica-identity key is accepted;
- observed unvalidated NOT NULL remains accepted when the index is not replica identity;
- replica identity remains accepted when the first-class NOT NULL family is unobserved.

Retained replica-identity, NOT NULL inheritance, digest, index, and temporal contracts must remain green on the same exact head before integration.

## Traceability

- Owner: `crates/conceptweave-observation/src/not_null_constraint.rs::canonicalize_not_null_constraints`
- Related owner: `crates/conceptweave-observation/src/lib.rs::validate_schema_relation_invariants`
- Focused contract: `crates/conceptweave-observation/tests/index_replica_identity_not_null_validation_contract.rs`
- PR: `ContextualWisdomLab/ConceptWeave#46`
- PostgreSQL owner: `postgres/postgres@051db7737c18b1c5d25cdc4ad508608c4b53fafc`, `src/backend/commands/tablecmds.c`

## Reference

PostgreSQL Global Development Group. (2026). *PostgreSQL source code: `tablecmds.c` (REL_18_STABLE, commit 051db7737c18b1c5d25cdc4ad508608c4b53fafc)*. GitHub.
