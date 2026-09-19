# Changelog

The Source Observation decision surface before the dangling-role diagnostic-projection repair is preserved at `docs/archive/CHANGELOG-through-8348be96.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-8348be96.md`. The earlier pre-dangling-role surface remains under the `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the converter-function `pg_init_privs` successor: exact row absence/presence, `privtype` (`i`/`e`), complete object-level initial EXECUTE ACL, immutable receipt/snapshot, exact converter binding, and raw converter-root lineage remain authoritative.
- Retained hostile dangling-role identity coverage for catalog states where an initial ACL still refers to a role OID after the role no longer exists.
- Added privacy-preserving dangling-role diagnostic projections: `unresolved_grantee_count()` and `unresolved_grantor_count()` expose whether deterministic validation has recovery damage to act on without publishing raw dangling OID values.
- Updated `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` with PostgreSQL 18 catalog-source authority, BUG #19483/#19513, and the diagnostic-projection repair trace.

### Correctness

- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` continues to preserve nonzero dangling grantee/grantor OIDs as raw unresolved identities rather than requiring role lookup or stringifying the OID as a role name.
- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial` now exposes unresolved grantee/grantor counts derived from the canonical initial ACL. Before this repair, the digest committed to dangling OIDs but downstream `validate`/recovery diagnostics could not determine from the immutable observation whether unresolved role references existed.
- Derived unresolved-reference counts do not enter the digest. Existing resolved and unresolved ACL digest identity remains byte-for-byte stable; raw dangling OIDs remain committed inside the privacy-preserving digest rather than becoming a public diagnostic data surface.
- Resolved role names and unresolved OIDs remain separate identity namespaces. A real role named `"16424"` does not alias raw dangling OID `16424`.
- PUBLIC remains a grantee-only identity. OID zero is rejected by unresolved-role constructors and is never treated as a dangling role.
- Current `pg_proc.proacl`, converter-function `deptype='e'`, complete `deptype='x'` sets, security labels, exact `pg_init_privs` baseline, immutable raw converter root, and transform-object `deptype='e'` remain separate facts.

### Test and repair evidence

- Initial dangling-role representability finding review: `5254640920`; structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332`; production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- Diagnostic-projection finding review: `5254787838` at exact pre-finding head `8348be96316f625525b3d89abb732e952c0240e5`.
- Diagnostic structural RED: `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`; the dangling-role contract referenced unresolved-reference count accessors before production exposed them.
- Diagnostic production causal repair: `fde6d3a8776b75e3dd014713b671734b9226a0fe`.
- The focused contract requires resolved-only `0/0`, dangling grantee `1/0`, dangling grantor `0/1`, both dangling `1/1`, and PUBLIC with dangling grantor `0/1`.
- Pre-diagnostic archives were created ordinary-forward as `docs/archive/CHANGELOG-through-8348be96.md` and `docs/archive/product-technical-gap-baseline-through-8348be96.md`.
- Source/documentation repair itself is not native or hosted GREEN evidence. Exact-head Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and PostgreSQL 18 bounded live differential remain gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, initial-privilege baselines, immutable raw converter-root lineage, and independent transform-object extension membership.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads, checks, reviews, workflow inventories, and protected-main relationships must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
