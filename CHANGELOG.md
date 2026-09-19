# Changelog

The Source Observation decision surface before converter-function initial-privilege evidence is preserved at `docs/archive/CHANGELOG-through-71c86b19.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-71c86b19.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Added `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType`, `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant`, `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial`, observation, immutable snapshot, and receipt as a successor to converter-function security-label evidence.
- The observation preserves exact absence versus presence of the same-generation converter-function `pg_init_privs` row, exact `privtype` (`i`/`e`), and a canonical privacy-preserving identity for the complete object-level initial EXECUTE ACL.
- Added hostile contracts covering absent versus extension-provided baselines, `initdb` versus extension `privtype`, deterministic ACL ordering, duplicate ACL rejection, blank resolved roles, completeness, duplicate coordinates, exact converter binding, immutable raw converter-root propagation, collision-safe provenance, and exact receipt lookup.
- Restacked transform-object extension membership on the converter initial-privilege successor so recovery/dump semantics cannot disappear from the final transform-object lifecycle digest.
- Added `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-integrity.md` with PostgreSQL 18 `pg_init_privs`, extension packaging, GRANT, and `pg_dump` primary-source traceability.

### Correctness

- PostgreSQL initial privileges are now represented separately from current `pg_proc.proacl`. Two converter functions may have the same current ACL while carrying different initial privilege baselines, which changes the GRANT/REVOKE reconstruction PostgreSQL emits for extension dump/restore.
- Row absence remains distinct from a present `pg_init_privs` row. Present rows distinguish `privtype='i'` from `privtype='e'` even when their initial ACLs are otherwise identical.
- The initial ACL is canonicalized only for deterministic set identity. PUBLIC versus named-role grantee, exact same-generation grantor identity, and grant option remain identity-bearing; duplicate ACL entries fail closed.
- The successor retains complete converter coordinates, direction, exact converter schema/function binding, same-generation source metadata, and the immutable raw `converter_snapshot_digest`.
- Transform-object extension membership now uses the initial-privilege snapshot as its digest predecessor while still proving that predecessor's immutable raw converter root equals the separately supplied raw transform-converter snapshot before checking complete direction/function binding.
- Converter current ACL, `deptype='e'`, complete `deptype='x'` sets, security labels, initial privilege baselines, and transform-object `deptype='e'` remain separate Source Observation facts. Extension package/control/update scripts, role-membership policy, package inventory, and application metadata remain outside ConceptWeave.

### Test and repair evidence

- Initial-privilege finding review: `5254405649`.
- Structural RED: `6a16b6c314e9ba6f9b8a2038055713c62e4cefab`, which referenced the not-yet-existing public initial-privilege contract.
- Production successor: `c94caa8a1da7088c819b802a87588ba303edcf55`.
- Public composition: `1ba7f86bb62c399f334b823ed51bad651e398f24`.
- Transform-object production restack: `50dddc811bacc301fc36f0d7b5bddd3a6a51fd18`.
- Transform-object contract restack: `34c8611f0c75723bdebc82e0e84fa67a92856604`.
- Retained direction/function/raw-root hostile lineage restack: `a1c35b715e56f5a62813ba60e65d0ba6a80ba418`.
- Focused primary-source doctoring: `df88a83cb3606ac89bd0b2ec47fa6d4abae0e123`.
- The structural RED was real at its commit because the public initial-privilege types did not yet exist. Later source commits satisfy that compile-level contract; source repair itself is not native or hosted GREEN evidence.
- No native or hosted GREEN is claimed after source or documentation movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain exact-head gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, immutable raw converter-root lineage, and independent transform-object extension membership.
- Source Observation stores initial privilege state as exact external recovery truth. Later validation/publication may impose authorization policy, but observation must not reconstruct `pg_init_privs` from current ACL, extension membership, package state, or names.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads, checks, reviews, and protected-main relationships must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
