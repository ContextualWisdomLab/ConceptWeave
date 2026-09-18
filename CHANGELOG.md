# Changelog

The preceding active Source Observation surface through converter-kind head `09f4b9c9d1be4da2979c0a01958014a1aa129f42` is preserved losslessly at `docs/archive/CHANGELOG-through-09f4b9c9.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-09f4b9c9.md`. Earlier history remains under `docs/archive/`. This active changelog records the remaining explicit structural transform-converter predicates from PostgreSQL 18 `check_transform_function()`.

## Unreleased

### Added

- Added exact same-row transform-converter set-return evidence for raw `pg_proc.proretset=false`.
- Added exact same-row transform-converter argument-count evidence for raw `pg_proc.pronargs=1`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation`/receipt/snapshot over converter-kind evidence and `IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation`/receipt/snapshot over converter-return-set evidence.
- Added focused doctoring/TRACEABILITY for both source facts under `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-{return-set,argument-count}-integrity.md`.

### Correctness

- A normalized scalar return type no longer substitutes for raw `proretset`: set-returning converter rows fail closed even when other converter coordinates look valid.
- A normalized one-`internal`-argument value object no longer substitutes for raw `pronargs`: converter rows fail closed unless the same-generation catalog count is exactly `1`.
- Both successors exactly cover the predecessor converter-direction inventory and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, blank identifiers, zero positions, invalid structural values, and unknown receipts fail closed.
- Quoted transform-type provenance remains collision-safe through independent component percent-encoding.
- The base converter contract continues to require the resolved single argument type to be `pg_catalog.internal`, preserving PostgreSQL's final explicit structural predicate without rewriting predecessor digest domains.

### Test and repair evidence

- Return-set finding review `5251492933` -> RED `e92a8657a20878621da883a4326237f1d899bc89` -> production `c6d11432cbac681a937449ea41e2c49e26f24409` -> public composition `40fc0e46b8a9e8c1018245540fc788f47a9132c0`.
- Argument-count finding review `5251508944` -> RED `4f81f92f54edb622ba933617ef6a598d7a4a1d16` -> production `65dabb7df539c2a80455f8b4aec5c024117b8a58` -> final public composition `c7419477a8538551a573482f78013ad226923899`.
- The first argument-count composition attempt `cb43e7fc8ba28516dc96a6e3d61a8df9a71c9a24` accidentally duplicated one public re-export outside the intended stanza. Ordinary-forward `c7419477...` removed it immediately. Net comparison from return-set public head `40fc0e46...` to `c7419477...` is ahead-only and leaves only the new production module, its contract, and four intended `index_partition.rs` lines.
- Focused contracts cover structural-value rejection, direction completeness, extra/duplicate coordinates, binding drift, blank identifiers, zero positions, exact receipt lookup, successor digest-domain separation, public composition, and quoted-type provenance collision safety.
- No native/hosted GREEN is claimed. Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, and collision-safe quoted-identifier provenance.
- The explicit structural predicates in PostgreSQL 18 `check_transform_function()` are now represented as source facts: `prokind='f'`, `proretset=false`, `pronargs=1`, and the existing exact `internal` argument-type boundary. This is not a universal semantic-completeness claim; any next successor must come from fresh source/catalog review.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa`; CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their exact-head acceptance evidence is independent and does not transfer to #2040.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.