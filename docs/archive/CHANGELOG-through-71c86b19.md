# Changelog

The Source Observation decision surface before converter-function security-label evidence is preserved at `docs/archive/CHANGELOG-through-106cfb3f.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-106cfb3f.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Added `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel`, `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation`, and immutable snapshot/receipt as a successor to converter-function auto-extension dependency evidence.
- The observation preserves the complete same-generation PostgreSQL `pg_seclabel` provider/label map for each exact FROM SQL / TO SQL converter function. Empty and populated maps, multiple providers, exact provider identity, and exact provider-owned label text remain distinct.
- Added focused hostile contracts covering MAC-labeled versus unlabeled functions, deterministic provider ordering, raw empty/whitespace-bearing label text, blank/duplicate provider rejection, completeness, duplicate coordinates, exact converter binding, immutable raw converter-root propagation, collision-safe provenance, and exact receipt lookup.
- Restacked transform-object extension membership on the converter security-label successor so label-based security state cannot disappear from the final transform-object lifecycle digest.
- Added `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-security-label-integrity.md` with PostgreSQL 18 `SECURITY LABEL`, `pg_seclabel`, and `sepgsql` primary-source traceability.

### Correctness

- PostgreSQL security labels are now represented separately from owner, EXECUTE ACL, `SECURITY DEFINER`, leakproofness, extension membership, and `DEPENDS ON EXTENSION` edges. Label-based MAC therefore no longer collapses into discretionary privilege state.
- Provider order is canonicalized only for deterministic set/map identity. Label text is not trimmed, parsed, or interpreted because PostgreSQL delegates validity and meaning to the registered provider. Duplicate provider rows fail closed because PostgreSQL permits at most one label per provider per object.
- The security-label successor retains complete converter coordinates, direction, exact converter schema/function binding, same-generation source metadata, and the immutable raw `converter_snapshot_digest`.
- Transform-object extension membership now uses the security-label snapshot as its digest predecessor while still proving that predecessor's immutable raw converter root equals the separately supplied raw transform-converter snapshot before checking complete direction/function binding.
- Converter-function `deptype='e'`, converter-function `deptype='x'`, converter-function security labels, and transform-object `deptype='e'` remain separate Source Observation facts. Provider policy, SELinux policy, extension-owned metadata, package inventory, and application metadata remain outside ConceptWeave.

### Test and repair evidence

- Security-label finding review: `5254227769`.
- Structural RED: `49059a31e9445f364fca190dba5b049a440e78e1`, which referenced the not-yet-existing public security-label contract.
- Production successor: `58abc98906919ca803adb4bbe3f197df61ffc37c`.
- Public composition: `0b1a513c6c4ca50353160b986e70a088381360e6`.
- Transform-object production restack: `03d434c88e1251d6d28977be4805738c374d183b`.
- Transform-object contract restack: `e764a73751bb88fd362fbc5af4948691d8dd02aa`.
- Retained direction/function/raw-root hostile lineage restack: `c52fce2d4ea782e83bae4b31636335ad8c417a06`.
- Duplicate-provider test correction: `f4c0ce73237062b2a28b2f0ee69731d72ab6853b`.
- Focused primary-source doctoring: `f93c699a0da1be463593b2aff1bcbf565a1924bc`.
- The structural RED was real at its commit because the public security-label types did not yet exist. Later source commits satisfy that compile-level contract; source repair itself is not native or hosted GREEN evidence.
- No native or hosted GREEN is claimed after source or documentation movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain exact-head gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, immutable raw converter-root lineage, and independent transform-object extension membership.
- Source Observation stores provider/label state as raw external truth. Later validation may reject a label or require a known provider, but observation must not normalize or reinterpret provider-owned label semantics.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads, checks, reviews, and protected-main relationships must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
