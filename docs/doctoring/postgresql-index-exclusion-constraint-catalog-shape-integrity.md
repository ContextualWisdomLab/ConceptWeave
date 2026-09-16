# PostgreSQL ordinary EXCLUDE catalog-family shape integrity

## Decision status

Proposed source-integrity repair for the active Draft Source Observation lane. This note does not claim executed Rust GREEN, hosted acceptance, live PostgreSQL differential evidence, or release readiness.

## Problem

ConceptWeave already governs ordinary PostgreSQL `contype = 'x'` EXCLUDE identity, backing-index linkage, partition parentage/inheritance, timing, enforcement, validation, `NO INHERIT`, temporal-period exclusion, `conkey`, `conexclop`, backing-index role/immediacy, relation-local name integrity, and independently resolved constraint namespace. The bounded predecessor still allowed a malformed extractor/catalog tuple to carry payload that belongs to a different `pg_constraint` family while collapsing into the same governed ordinary-EXCLUDE identity.

PostgreSQL 18 stores mutually exclusive family state in one catalog row. For a normal table EXCLUDE constraint:

- `contype` is `x`;
- `conrelid` is nonzero and `contypid` is zero;
- `confrelid` is zero;
- `confupdtype`, `confdeltype`, and `confmatchtype` use the non-FK space sentinel;
- `confkey`, `conpfeqop`, `conppeqop`, `conffeqop`, and `confdelsetcols` are foreign-key-only and therefore NULL;
- `conbin` is CHECK-only and therefore NULL.

Dedicated successors already own exact ordinary-EXCLUDE `conkey` and `conexclop`, so this repair must not duplicate or reinterpret those arrays.

Without an explicit family-shape boundary, an adapter bug that selected an ordinary EXCLUDE coordinate but retained domain/FK/CHECK residue could produce governed evidence indistinguishable from a source-reachable row.

## Constraints

1. Keep ConceptWeave as the canonical owner of source observation and governed semantic evidence; do not copy foreign product truth.
2. Do not rewrite any issued predecessor digest domain.
3. Do not infer family-shape fields from `contype`; the adapter must supply the independently observed raw sentinels/presence state.
4. Do not use capture-time nonzero OIDs as stable semantic identity. The only OID-level facts needed here are whether `conrelid`/`contypid`/`confrelid` are zero versus nonzero; exact supported relation/index identities remain resolved elsewhere.
5. Keep `conkey` and `conexclop` in their existing exact ordered successors.
6. Preserve temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` in their dedicated owner families.

## Alternatives considered

### Trust the extractor's `WHERE contype = 'x'`

Rejected. A row discriminator does not prove the remaining selected fields were read from the same row or that family-incompatible payload was not retained during mapping. ConceptWeave's governed evidence boundary should reject impossible cross-family tuples rather than rely on query intent.

### Normalize incompatible payload to EXCLUDE defaults

Rejected. Clearing domain/FK/CHECK residue would erase source-integrity evidence and make malformed capture indistinguishable from a valid row.

### Add the omitted fields directly to the existing EXCLUDE identity digest

Rejected because predecessor digests are immutable once issued. The repair must be a domain-separated successor.

### Duplicate `conkey` and `conexclop` into the same shape object

Rejected. Both already have dedicated exact-value successors with their own completeness, backing-index, and semantic checks. Duplicating them would create competing owner truth.

## Chosen repair

Review `5222790284` on exact predecessor `fbb935ea1b7e6c17c81ebd9106cc8d6d20b55b5b` records the P1 finding.

The ordinary-forward lineage is:

- structural source/compile RED contract `531da2eb704806694da2dfe6fdc2003f19d89c56`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_catalog_shape_contract.rs`;
- production `IndexExclusionConstraintCatalogShapeSnapshot` `7fb3e5ac8cc22c0164aeb6c9bee800b78dd4c84a`;
- public composition `3b8982eba610c97d47936d8e8f63441ed79183bc`;
- contract/API currentization `18e76bfb54b777ec6193b02718f23b3b04c751ee`;
- focused edge-branch coverage `f6862d7d52eb6bed02a2e06ea0410cc722613256`.

`IndexExclusionConstraintCatalogShapeObservation` retains raw `contype`, presence of relation/domain ownership, the three fixed foreign-action characters, presence of all FK-only payload families, and CHECK-expression presence. It admits only the PostgreSQL 18 ordinary-EXCLUDE shape and fails closed otherwise. `IndexExclusionConstraintCatalogShapeSnapshot` requires exactly one observation for every predecessor ordinary EXCLUDE coordinate, binds those raw facts into a new digest domain, and issues coordinate-specific provenance.

The source/compile RED commit was created before the production type existed, so it is structurally RED. No executed compiler failure is claimed.

## Invariants

For every governed ordinary EXCLUDE coordinate:

```text
contype == 'x'
conrelid != 0
contypid == 0
confrelid == 0
confupdtype == ' '
confdeltype == ' '
confmatchtype == ' '
confkey IS NULL
conpfeqop IS NULL
conppeqop IS NULL
conffeqop IS NULL
confdelsetcols IS NULL
conbin IS NULL
```

The dedicated `conkey` and `conexclop` successors remain mandatory separate evidence. The family-shape successor does not claim catalog completeness beyond these bounded fields.

## Test and edge-case traceability

`index_exclusion_constraint_catalog_shape_contract.rs` covers:

- positive exact family shape and provenance;
- non-`x` discriminator rejection;
- missing relation owner (`conrelid = 0`);
- domain-owner residue (`contypid != 0`);
- FK action-code residue;
- FK-only variable payload residue;
- CHECK-expression residue;
- complete one-to-one predecessor inventory;
- duplicate coordinate rejection;
- unknown provenance location.

`index_exclusion_constraint_catalog_shape_edge_contract.rs` exercises each foreign-action sentinel and each FK-only payload slot independently so short-circuit validation cannot hide an uncovered branch.

## Risks and controls

The new successor currently depends on the adapter reporting raw sentinel/presence state truthfully. A live PostgreSQL differential is therefore still required. It must select the family fields from the same bounded `pg_constraint` row as the already retained ordinary-EXCLUDE state rather than deriving emptiness from the constraint kind.

A boolean presence representation deliberately avoids treating capture-time nonzero OIDs as semantic identifiers. If a future buyer requirement needs forensic retention of exact raw OIDs, that must be a separate provenance artifact with explicit lifecycle and portability rules rather than silently entering the semantic identity digest.

## Live PostgreSQL 18 differential

Create ordinary EXCLUDE rows, including partition parent/child examples, and read in one bounded capture:

- `contype`, `conrelid`, `contypid`, `conindid`, `conparentid`, `confrelid`;
- `confupdtype`, `confdeltype`, `confmatchtype`;
- `conislocal`, `coninhcount`, `connoinherit`, `condeferrable`, `condeferred`, `conenforced`, `convalidated`, `conperiod`;
- `conkey`, `confkey`, `conpfeqop`, `conppeqop`, `conffeqop`, `confdelsetcols`, `conexclop`, `conbin`;
- independently resolved `connamespace` and supporting `pg_index` evidence.

Use CHECK, FK, domain CHECK, temporal key, and ordinary EXCLUDE rows as controls so family-specific nonempty payload is demonstrated rather than assumed. ConceptWeave must admit only the ordinary EXCLUDE shape for this successor and leave each control with its canonical family owner.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: `src/include/catalog/pg_constraint.h`, REL_18_STABLE*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_constraint.h

## Acceptance boundary

This repair is not GREEN until one unchanged exact #46 head passes repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, hosted Product/security/dependency/review gates, and the bounded PostgreSQL 18 live differential. Any head movement resets exact-head acceptance evidence.
