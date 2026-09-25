# Changelog

The preceding active Source Observation surface through converter-planner-support head `913577072165cb09fbf52bbdb2e1216fa2b21005` is preserved losslessly at `docs/archive/CHANGELOG-through-91357707.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-91357707.md`. Earlier history remains under `docs/archive/`. This active changelog records the transform-converter planner-cost delta.

## Unreleased

### Added

- Added exact PostgreSQL ordinary-`EXCLUDE` transform-converter planner-cost evidence for same-row `pg_proc.procost`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation`, immutable source receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot` as a successor over converter planner-support evidence.
- Added focused doctoring/TRACEABILITY at `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-cost-integrity.md`, grounded in PostgreSQL 18 `pg_proc`, `CREATE FUNCTION`, and Function Optimization Information.

### Correctness

- Converter definition, owner, `EXECUTE` ACL, nullable `proconfig`, security-definer context, leakproof classification, strictness, volatility, parallel safety, and planner-support identity no longer imply equal planner-cost state. Distinct positive finite `pg_proc.procost` `float4` values produce distinct successor digests by raw bits.
- Cost observations exactly cover the predecessor converter-direction inventory and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, non-positive/non-finite costs, and unknown receipts fail closed.
- ConceptWeave records the PostgreSQL catalog value and does not infer language defaults or impose a product-specific planner-cost threshold.
- Cost provenance preserves the quoted-qualified-type collision repair by percent-encoding transform schema and type-name components independently before the canonical separator.

### Test and repair evidence

- Review `5250941523` identifies missing converter `pg_proc.procost` at prior exact head `913577072165cb09fbf52bbdb2e1216fa2b21005`.
- RED `c876bb3907f9d33da3973196937fd854ac3aae85` adds the executable converter-cost contract before public converter-cost types exist.
- Production `f82ca656ac238c1ce7015cddd327169abd7ce054` adds converter cost observation/snapshot/receipt behavior.
- Public-composition successor `5032b9cbeacd784f6b36d1393476a24b00f7529a` exports the new successor through `index_partition.rs`.
- The focused contract covers exact `float4` bit preservation, digest separation, rejection of zero/negative/non-finite costs, exact direction completeness, extra/duplicate coordinates, converter-function binding drift, blank identifiers, zero positions, exact receipt lookup, and quoted-type provenance collision safety.
- No native/hosted GREEN is claimed. Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, exact `prosupport` absence/identity, and collision-safe quoted-identifier provenance.
- The named converter-function fact sequence reviewed in this lane now includes definition, owner, ACL, configuration, security-definer, leakproof, strictness, volatility, parallel safety, planner support, and cost. This is not a claim that every possible future PostgreSQL/catalog semantic gap is exhausted; further facts still require evidence-driven review rather than inference.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa`; CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their exact-head acceptance evidence is independent and does not transfer to #2040.
- Source-fix #2175 remains `6e5f75dd40a428d174f691e35e210d08a29eb270` and cannot self-modify `.github/` or `scripts/ci/`.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.