# PostgreSQL ordinary EXCLUDE constraint-name integrity

## Decision

ConceptWeave rejects an ordinary PostgreSQL `contype='x'` EXCLUDE observation when its exact relation-local `conname` collides with another constraint row already represented in the bounded source snapshot. The check covers `RelationObservation::constraints()` (PRIMARY KEY, UNIQUE, FOREIGN KEY, CHECK) and the separately observed PostgreSQL 18 first-class NOT NULL family. An unobserved NOT NULL family is not inferred. The same constraint name on a different relation remains valid.

This is source-integrity validation, not naming policy. ConceptWeave does not rename, normalize, deduplicate, or select a preferred owner constraint.

## Problem

`IndexExclusionConstraintSnapshot` already rejected duplicate EXCLUDE coordinates inside its own family, but it did not compare an ordinary EXCLUDE `conname` with constraint families represented by the predecessor snapshot. A caller could therefore combine, for example, a CHECK row named `bookings_no_overlap` with an ordinary EXCLUDE row of the same name on `public.bookings`, receive a governed successor digest, and describe a PostgreSQL catalog state that cannot exist.

A second bounded review found the same gap for PostgreSQL 18 first-class NOT NULL constraints, which are intentionally represented outside `RelationObservation::constraints()`. Limiting the repair to PK/UQ/FK/CHECK would therefore have left the catalog invariant only partially enforced.

## PostgreSQL authority

Pinned primary source: PostgreSQL `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`, `src/include/catalog/pg_constraint.h`.

PostgreSQL documents `conname` as unique among the constraints of one relation or domain and enforces the relation case with the unique catalog index over `(conrelid, contypid, conname)`. For relation constraints, `contypid` is zero, so two rows belonging to the same relation cannot share a `conname`, regardless of whether the rows are CHECK, NOT NULL, PRIMARY KEY, UNIQUE, FOREIGN KEY, or EXCLUDE. The same name on different relations is legal because `conrelid` differs.

APA 7 reference:

PostgreSQL Global Development Group. (2026). *pg_constraint catalog definition* (REL_18_STABLE, commit 3d2e8573e9cb91bd2b545184f4f9b326d237bcd1) [Source code]. PostgreSQL.

## Alternatives considered

1. **Keep family-local uniqueness only.** Rejected because it admits a catalog tuple PostgreSQL itself prevents.
2. **Rename or normalize colliding constraints.** Rejected because source identifiers are evidence, not a ConceptWeave naming namespace; mutation would hide the source contradiction.
3. **Infer NOT NULL constraints from `attnotnull`.** Rejected because the PostgreSQL 18 first-class NOT NULL family has independent catalog state and ConceptWeave already distinguishes observed from unobserved family evidence.
4. **Reject the same name globally across the schema.** Rejected because PostgreSQL permits the same `conname` on different relations.

## Ordinary-forward lineage

- finding review `5221675071` on exact predecessor `0de651ee57f823638a416677239a9ef4dbac3c84`;
- CHECK↔EXCLUDE behavioral contract `0237a677302ad15121555d575dc09252348cc1c0`;
- minimal source repair restored ordinary-forward after an overly broad formatting-only intermediate delta at `a6e53cd459e61efaf61ec94e55cd9128aa5ec3fc`;
- bounded follow-up review `5221753670` identified the separately observed NOT NULL owner family;
- expanded NOT NULL behavioral contract with duplicate fixture helpers removed at `50b69ea481dd0acdb303c91376976d8f130b942f`;
- source repair covering both base relation constraints and explicitly observed first-class NOT NULL constraints `a5c0ce6624eb53d767a881d0fee49ba44841c5ef`;
- contract-only formatting churn repaired forward without changing semantics at `864cd3bb003c496435110caeacfcf4bf1bb831de`, leaving the turn-start comparison focused on the new fixtures and assertions rather than a file-wide rewrite.

The production change is intentionally before EXCLUDE digest/receipt issuance. It does not alter predecessor or ordinary-EXCLUDE digest algorithms; it only prevents impossible source combinations from entering that identity domain.

## Behavioral contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_partition_contract.rs` now covers:

- same relation: CHECK and ordinary EXCLUDE with the same `conname` → `DuplicateConstraintName`;
- same relation: explicitly observed PostgreSQL 18 NOT NULL and ordinary EXCLUDE with the same `conname` → `DuplicateConstraintName`;
- different relations: the same textual constraint name remains admissible;
- existing ordinary EXCLUDE partition parentage, inheritance-state, completeness, and receipt tests remain retained.

The NOT NULL fixture carries an exact parent/child `conparentid`-equivalent linkage and direct partition-parent relation evidence so the collision test does not rely on an incoherent NOT NULL setup.

## Validation status

The contract and causal repair are source-current. No executed Rust RED or GREEN is claimed: the current execution host does not provide the repository-pinned Rust 1.98 toolchain (`cargo`, `rustc`, `rustfmt`, `rustup`). Exact-head native and hosted acceptance therefore remain required after all documentation/currentization commits settle.

The PostgreSQL 18 bounded live differential should include relation-wide `pg_constraint` uniqueness over the captured ordinary EXCLUDE rows and all constraint families captured in the same observation. The adapter must preserve the distinction between an explicitly observed-empty family and an unobserved family; it must not fabricate NOT NULL rows solely from `attnotnull` to satisfy this check.

## Effect and follow-up

The repair closes one cross-family catalog-integrity gap before governed identity is minted. It does not establish complete `pg_constraint` coverage. Bounded review should continue for materially relevant catalog state, while the next executable gate remains one unchanged #46 head through Rust 1.98 native/hosted GREEN and a real PostgreSQL 18 differential before parent adoption or publication.