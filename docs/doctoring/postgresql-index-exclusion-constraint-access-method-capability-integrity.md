# PostgreSQL 18 ordinary EXCLUDE access-method capability integrity

## Decision

ConceptWeave must preserve an independently observed PostgreSQL index-access-method exclusion capability for every ordinary `pg_constraint.contype = 'x'` constraint and must reject governed evidence when that capability is false, missing, duplicated, or bound to a different backing index/access method.

The accepted source fact is the boolean returned by `pg_indexam_has_property(am_oid, 'can_exclude')` for the access method resolved from the exact `pg_constraint.conindid` backing index. The access-method name already retained by the v3 index observation remains a separate fact. Neither value may be synthesized from the other.

## Problem

Before review `5224291789`, the Source Observation stack retained:

- exact ordinary EXCLUDE constraint identity and resolved `conindid` backing-index coordinate;
- `pg_index.indisexclusion` and the other backing-index role bits;
- the backing index access-method name;
- operator-class/operator-family identity and resolved exclusion operator/procedure/strategy semantics.

Those facts were not sufficient to prove that PostgreSQL would permit the observed access method to implement an exclusion constraint. An impossible source tuple such as an ordinary EXCLUDE backed by an index whose access method has no tuple-at-a-time scan callback could therefore receive governed evidence if the caller supplied otherwise coherent index/operator observations.

This is not a built-in-method naming problem. PostgreSQL permits extension index access methods, so a hard-coded `gist`/`spgist`/`btree`/`hash` allowlist would incorrectly reject valid extension AMs and would duplicate provider truth.

## PostgreSQL authority

Pinned PostgreSQL authority for this stack is `postgres/postgres@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1` (`REL_18_STABLE`).

`DefineIndex` rejects an exclusion constraint when `amRoutine->amgettuple == NULL`, with the error that the access method does not support exclusion constraints. PostgreSQL exposes the same capability through the stable SQL information function property `can_exclude`; the pinned `amutils.c` implementation returns whether `routine->amgettuple` is present for `AMPROP_CAN_EXCLUDE`.

The PostgreSQL 18 `CREATE TABLE` documentation states the same requirement: an `EXCLUDE` index access method must support `amgettuple`, and specifically notes that GIN cannot be used. PostgreSQL 18 system-information documentation defines `pg_indexam_has_property(..., 'can_exclude')` as the AM-level property indicating support for exclusion constraints.

## Chosen contract

`IndexExclusionConstraintAccessMethodCapabilitySnapshot` is a domain-separated successor over the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE predecessor chain.

For each predecessor ordinary EXCLUDE coordinate it requires exactly one `IndexExclusionConstraintAccessMethodCapabilityObservation` containing:

- the exact ordinary EXCLUDE coordinate;
- the exact resolved `conindid` backing-index coordinate;
- the independently observed access-method name;
- the independently observed `can_exclude` boolean.

Snapshot construction rebinds the ordinary EXCLUDE predecessor, proves that the supplied backing-index coordinate is the exact constraint backing index, resolves that index in the bounded v3 source snapshot, requires the observed access-method name to equal the index's independently retained access-method name, and requires `can_exclude = true`.

The successor digest includes the predecessor digest plus both coordinates, the access-method name, and the capability boolean. Existing v3/index/constraint digest domains are unchanged. Provenance is issued at `/access-method-exclusion-capability` beneath the ordinary EXCLUDE coordinate.

## Rejected alternatives

A built-in access-method allowlist was rejected because extension AMs can validly support exclusion constraints. Inferring capability from `pg_index.indisexclusion`, operator-family presence, or exclusion operator/procedure/strategy observations was rejected because those are different source facts and would turn validation into circular reconstruction. Rewriting the existing ordinary EXCLUDE digest was rejected because already-issued predecessor identities must remain immutable.

## Contract evidence

Review finding: `5224291789` on predecessor `9cc1e8a2076cf329c043a4a77705cce4862b1faf`.

Ordinary-forward implementation lineage:

- structural source/compile contract: `507f72fe3727a307c3bb23e3d14c8328a12b8be3`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_access_method_capability_contract.rs`;
- production successor: `a918215bfe29f242ce4684840b03b7de13fe1164`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_access_method_capability.rs`;
- public composition: `f4dd77a3abb80180d6e571754b7a627f23854a28`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

The focused contract covers false capability, access-method binding drift, missing evidence, duplicate evidence, and an extension-AM positive control. The positive control intentionally uses a non-core access-method name and succeeds only because explicit `can_exclude=true` evidence is supplied.

No executed Rust RED/GREEN is claimed by this document. The structural RED commit references public types that did not yet exist at that commit. Native and hosted acceptance still require one unchanged exact head to pass the repository-pinned Rust 1.98 gates and the applicable central workflow/review/security gates.

## Live differential requirement

The PostgreSQL 18 live differential must resolve the exact `conindid` backing index, read its `pg_class.relam`, resolve the access-method name independently, and query `pg_indexam_has_property(relam, 'can_exclude')` in the same bounded source-observation operation. The extractor must not map method names to capability locally.

Controls must include at least:

- a supported built-in method with `can_exclude=true`;
- GIN with `can_exclude=false` as the negative AM control (without attempting to fabricate a valid EXCLUDE DDL row);
- an extension/index-AM fixture when available, proving that acceptance is capability-based rather than name-based.

The live differential should distinguish “the AM reports false” from inability to resolve the backing index or AM. Missing/NULL evidence is not equivalent to false and must fail closed at the adapter boundary rather than being normalized.

## Traceability

- Owner bounded context: ConceptWeave Source Observation / relation-index semantic evidence.
- Exact PR: `ContextualWisdomLab/ConceptWeave#46`.
- Production module: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_access_method_capability.rs`.
- Focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_access_method_capability_contract.rs`.
- PostgreSQL source authority: `src/backend/commands/indexcmds.c`, `src/backend/utils/adt/amutils.c` at pinned commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: System information functions and operators*. https://www.postgresql.org/docs/18/functions-info.html

PostgreSQL Global Development Group. (2025). *PostgreSQL source: indexcmds.c* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/commands/indexcmds.c

PostgreSQL Global Development Group. (2025). *PostgreSQL source: amutils.c* (REL_18_STABLE, commit `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`). https://github.com/postgres/postgres/blob/3d2e8573e9cb91bd2b545184f4f9b326d237bcd1/src/backend/utils/adt/amutils.c
