# Changelog

The Source Observation decision surface before the dangling-role diagnostic-projection repair is preserved at `docs/archive/CHANGELOG-through-8348be96.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-8348be96.md`. The earlier pre-dangling-role surface remains under the `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the converter-function `pg_init_privs` successor: exact row absence/presence, `privtype` (`i`/`e`), complete object-level initial EXECUTE ACL, immutable receipt/snapshot, exact converter binding, and raw converter-root lineage remain authoritative.
- Retained hostile dangling-role identity coverage for catalog states where an initial ACL still refers to a role OID after the role no longer exists.
- Added privacy-preserving dangling-role diagnostic projections: `unresolved_grantee_count()` and `unresolved_grantor_count()` expose whether deterministic validation has recovery damage to act on without publishing raw dangling OID values.
- Added the canonical initial-privilege recovery validator through the relation/index-partition public surface. Row absence and fully resolved material validate ready; any unresolved grantee or grantor reference blocks readiness.
- Made the public recovery verdict non-forgeable by replacing constructible enum variants with a public struct whose evidence fields are private and whose production construction path is the canonical validator.
- Bound recovery validation to the owner-issued `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt`: every verdict now carries the exact source snapshot digest and canonical converter location, while present rows additionally carry their immutable material digest.
- Kept row absence material-free (`material_digest() == None`) without leaving successful absence validation unbound; the source digest plus canonical location identify the exact absent observation that was validated.
- Added focused replay-resistance contracts proving byte-identical ACL material remains distinct validation evidence across converter locations and source generations.
- Updated `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privilege-recovery-validation.md` with the receipt-binding finding, RED, causal repair, and provenance invariants.
- Updated `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` with PostgreSQL 18 catalog-source authority, BUG #19483/#19513, and the diagnostic-projection repair trace.

### Correctness

- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` continues to preserve nonzero dangling grantee/grantor OIDs as raw unresolved identities rather than requiring role lookup or stringifying the OID as a role name.
- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial` exposes unresolved grantee/grantor counts derived from the canonical initial ACL without changing source digest identity.
- Recovery readiness is derived only from private validation evidence produced by the canonical validator; callers cannot manufacture a public `Ready` variant.
- The validator no longer accepts detached `Option<&...InitialPrivilegeMaterial>`. It accepts an owner-issued source receipt, preventing a valid result from being replayed as evidence for another converter direction or source snapshot.
- Present recovery evidence carries `Some(material.digest())`; row absence carries `None`. Both states carry the receipt source digest and canonical observation location.
- Equal ACL material intentionally retains equal material identity, while validation provenance remains distinct when source generation or converter location differs.
- Derived unresolved-reference counts and the validation verdict do not enter the Source Observation digest. Existing resolved and unresolved ACL digest identity remains stable; raw dangling OIDs remain committed inside the privacy-preserving digest rather than becoming a public diagnostic data surface.
- Resolved role names and unresolved OIDs remain separate identity namespaces. A real role named `"16424"` does not alias raw dangling OID `16424`.
- PUBLIC remains a grantee-only identity. OID zero is rejected by unresolved-role constructors and is never treated as a dangling role.
- Current `pg_proc.proacl`, converter-function `deptype='e'`, complete `deptype='x'` sets, security labels, exact `pg_init_privs` baseline, immutable raw converter root, and transform-object `deptype='e'` remain separate facts.

### Test and repair evidence

- Initial dangling-role representability finding review: `5254640920`; structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332`; production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- Diagnostic-projection finding review: `5254787838` at exact pre-finding head `8348be96316f625525b3d89abb732e952c0240e5`; structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`; production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe`.
- Validation-boundary finding review: `5254918439` at exact pre-finding head `b88836369e5481f8f91eda86003af979f28bc7c9`; structural RED `21817d01f0aaa69902699eb21109c2a057401a8c`; production repair `97ff3903278bafbd2133a88ded475ca481e8b1b7`; public composition `da84d2afc99d41f7af9273cd192047704d779a47`.
- Non-forgeable validation-evidence finding review: `5254963033`; structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38`; production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc`.
- Source-receipt replay finding review: `5255023556` at exact pre-finding head `83b23116bad4106c2fe1c71953fb2453175dad0c`.
- Source-receipt structural RED: `8734883edcb145933030cccd7177cff95abde772`; it requires equal material at different converter locations and generations to retain distinct validation provenance and requires row absence to remain receipt-bound.
- Source-receipt production repair: `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2`; validation now consumes owner-issued receipt provenance and carries source digest + canonical location without widening raw-OID diagnostics.
- The focused diagnostic contract requires resolved-only `0/0`, dangling grantee `1/0`, dangling grantor `0/1`, both dangling `1/1`, and PUBLIC with dangling grantor `0/1`.
- The focused validation contract requires source- and location-bound row absence, exact present-material digest binding, unresolved dimensions to block readiness, and public diagnostics to omit raw OID values.
- Source/documentation repair itself is not native or hosted GREEN evidence. Exact-head Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and PostgreSQL 18 bounded live differential remain gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, initial-privilege baselines, immutable raw converter-root lineage, and independent transform-object extension membership.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads, checks, reviews, workflow inventories, and protected-main relationships must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
