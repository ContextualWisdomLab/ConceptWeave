# Changelog

The preceding active Source Observation surface through converter-cost head `ef4657347a605a1a5acbfac8500fadf8142ae9b9` is preserved losslessly at `docs/archive/CHANGELOG-through-ef465734.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-ef465734.md`. Earlier history remains under `docs/archive/`. This active changelog records the transform-converter routine-kind delta.

## Unreleased

### Added

- Added exact PostgreSQL ordinary-`EXCLUDE` transform-converter routine-kind evidence for same-row `pg_proc.prokind`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation`, immutable source receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot` as a successor over converter-cost evidence.
- Added focused doctoring/TRACEABILITY at `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-kind-integrity.md`, grounded in PostgreSQL 18 `pg_proc`, `CREATE TRANSFORM`, and `functioncmds.c::check_transform_function()`.

### Correctness

- Converter coordinate/signature/definition and the previously governed owner, ACL, configuration, security, planner-support, and cost facts no longer imply routine kind. Every converter direction now binds raw same-generation `pg_proc.prokind` and admits only PostgreSQL normal-function kind `f`.
- Kind observations exactly cover the predecessor converter-direction inventory and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, non-function kinds, and unknown receipts fail closed.
- Kind provenance preserves the quoted-qualified-type collision repair by percent-encoding transform schema and type-name components independently before the canonical separator.

### Test and repair evidence

- Review `5251421078` identifies the missing converter `pg_proc.prokind` structural fact at prior exact head `ef4657347a605a1a5acbfac8500fadf8142ae9b9`.
- RED `f60aa85fbf36ffb8ab6354afc4826d347ba8ea44` adds the executable converter-kind contract before public converter-kind types exist.
- Production `b75b3f008c0baeeae1b5e5eb3fd421b14e0d1205` adds converter kind observation/snapshot/receipt behavior.
- Public-composition successor `75028d1c8c18a490f793f8fcde5061ec39c52158` exports the new successor through `index_partition.rs`.
- Edge-contract successor `98149497113369b1f8a95c7f00bb0800a854a56f` additionally pins successor-domain separation and both blank schema/name branches.
- No native/hosted GREEN is claimed. Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, and collision-safe quoted-identifier provenance.
- PostgreSQL's independent `proretset=false` transform-converter invariant is not silently folded into `prokind`; it is the next explicit structural fact for review.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa`; CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their exact-head acceptance evidence is independent and does not transfer to #2040.
- Source-fix #2175 remains `6e5f75dd40a428d174f691e35e210d08a29eb270` and cannot self-modify `.github/` or `scripts/ci/`.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.