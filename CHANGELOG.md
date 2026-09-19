# Changelog

The Source Observation decision surface before converter-function auto-extension dependency evidence is preserved at `docs/archive/CHANGELOG-through-fb6b8e6e.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-fb6b8e6e.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Added `IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation` and its immutable snapshot/receipt as a successor to converter-function extension membership.
- The observation preserves the complete canonical set of same-generation PostgreSQL `pg_depend.deptype='x'` extension dependencies for each exact FROM SQL / TO SQL converter function direction. Empty, single, and multiple dependency sets remain distinct.
- Added a focused contract covering zero/one/multiple dependencies, deterministic set ordering, blank/duplicate extension names, completeness, duplicate coordinates, exact converter binding, immutable raw converter-root propagation, collision-safe provenance, and exact receipt lookup.
- Restacked transform-object extension membership on the auto-extension dependency successor so converter-function lifecycle evidence cannot disappear from the final transform-object successor digest.
- Added `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-auto-extension-dependency-integrity.md` with PostgreSQL 18 primary-source traceability.

### Correctness

- PostgreSQL extension membership (`pg_depend.deptype='e'`) and auto-extension dependency (`deptype='x'`) are now represented as different facts. A converter with no extension membership but one or more explicit `DEPENDS ON EXTENSION` edges no longer collapses into the same governed state as a converter with no such dependency.
- `ALTER FUNCTION ... DEPENDS ON EXTENSION` / `NO DEPENDS ON EXTENSION` can mutate these edges without changing converter schema/name identity or the `pg_transform` row. PostgreSQL permits multiple such dependencies, so the contract preserves a set rather than an `Option<String>`.
- Auto-extension dependency names are normalized by set ordering only; duplicate or blank names fail closed. Digest identity remains sensitive to exact set membership.
- The new successor retains `converter_snapshot_digest`, complete converter coordinates, direction, schema and function name from the converter-function extension-membership predecessor.
- Transform-object extension membership now uses the auto-extension dependency snapshot as its digest predecessor while still proving that predecessor's immutable raw converter root equals the separately supplied raw transform-converter snapshot before checking exact direction/function binding.
- Converter-function `deptype='e'`, converter-function `deptype='x'`, and transform-object `deptype='e'` remain separate owner facts. Extension-owned `extversion`, configuration, control/update scripts and package metadata are not copied into ConceptWeave.

### Test and repair evidence

- Immutable raw-root lineage retained: finding review `5253398088` -> behavioral RED `8ab7c431cdba2d0ba43a66313253b93b53b9e72a` -> source repair `7ee7847b629e4f2e5ec5a12f93f7da06f23f366c` -> coherence review `5253528602` -> docs currentization through `fb6b8e6e692449a6ca1d97e719045f6d387b7bb3`.
- Auto-extension lifecycle finding: review `5254049432` -> structural RED `3347a4c8fe2dab338c138cb4acd0ca0d7fc0e498` -> production snapshot `f52c34dbd385b8d2e50b54a67b125c1a35a777fa` -> public composition `55b0baf3bbb34d0ef24534d29c12a3814fa97176` -> transform-object contract restack `458bd76f193906999a81c583da30dff152d0f9f8` -> transform-object production restack `fcb0e2cdd8700ed56ee4d2a0b96bdf89c5bd4532` -> retained direction/function/raw-root lineage contract restack `60d90c3ed0b0793378707a39cd27397dba6ed3bb` -> focused doctoring currentization `98417ed8d92863fe3fddf97560b87bc37001a676`.
- The structural RED was real at its commit because the public auto-extension observation/snapshot types did not yet exist. Later source commits satisfy that compile-level contract; hosted/native GREEN has not been inferred from the repair itself.
- No native or hosted GREEN is claimed after source or documentation movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain exact-head gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, exact `deptype='e'` converter-function membership, immutable raw converter-root lineage, and independent transform-object extension membership.
- Source Observation records raw lifecycle state. Later validation may reject an unexpected dependency, but observation must not erase or reinterpret the `deptype='x'` edge.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads/checks/reviews must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
