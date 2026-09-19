# Changelog

The immediately preceding Source Observation/recovery-validation decision surface is preserved at `docs/archive/CHANGELOG-through-440d5e3f.md`; its matching product/technical surface is `docs/archive/product-technical-gap-baseline-through-440d5e3f.md`. Earlier dangling-role surfaces remain under the `through-8348be96` and `through-5daf2a57` archives. Focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the converter-function `pg_init_privs` successor: exact row absence/presence, `privtype` (`i`/`e`), complete object-level initial EXECUTE ACL, immutable receipt/snapshot, exact converter binding, and raw converter-root lineage remain authoritative.
- Retained hostile dangling-role identity coverage for catalog states where an initial ACL still refers to a role OID after the role no longer exists.
- Retained canonical sorted/deduplicated unresolved grantee/grantor OID sets inside initial-privilege material so recovery evidence preserves the exact missing PostgreSQL role identities rather than only aggregate counts.
- Added purpose-bound `unresolved_grantee_oids()` and `unresolved_grantor_oids()` accessors on non-forgeable recovery-validation evidence. `unresolved_grantee_count()` / `unresolved_grantor_count()` are projections of those canonical identity sets.
- Added custom material and validation `Debug` implementations that deliberately omit raw dangling OIDs while retaining counts, immutable digest/provenance, and explicit typed recovery accessors.
- Retained the canonical initial-privilege recovery validator through the relation/index-partition public surface. Row absence and fully resolved material validate ready; any unresolved grantee or grantor identity blocks readiness.
- Retained complete non-secret receipt provenance in every verdict: source registry identity, connection-policy binding, source-content digest, extractor revision, observation timestamp, and canonical converter location. Present rows additionally carry the immutable material digest and exact canonical dangling-role sets.
- `matches_source_receipt()` now compares complete receipt provenance, material absence/presence identity, and exact dangling-role identity sets.
- Added focused contracts proving equal unresolved counts with different OIDs remain distinguishable and repeated OIDs canonicalize deterministically.
- Updated both initial-privilege doctoring documents with the exact-role remediation boundary and acceptance requirements.

### Correctness

- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` continues to preserve nonzero dangling grantee/grantor OIDs as raw unresolved identities rather than requiring role lookup or stringifying the OID as a role name.
- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial` retains exact canonical dangling-role sets without changing the existing ACL/material digest framing. The Source Observation digest continues to commit the original ACL identities and does not add derived diagnostic fields.
- Recovery readiness is derived only from private validation evidence produced by the canonical validator; callers cannot manufacture a public `Ready` variant.
- Validation consumes an owner-issued source receipt rather than detached ACL material, preventing evidence replay across converter directions, source snapshots, policies, extractors, or observation epochs.
- Present recovery evidence carries `Some(material.digest())`; row absence carries `None`. Both states retain the complete owner-issued receipt binding, and present damaged rows retain exact missing-role identity sets.
- Equal ACL material intentionally retains equal material identity while validation provenance remains distinct when receipt provenance differs.
- Distinct dangling role identities remain distinguishable even when their aggregate unresolved counts are equal. Repeated dangling role identities canonicalize to sorted unique sets for deterministic recovery work.
- Raw dangling OIDs are available only through explicit typed recovery accessors and are omitted from routine material/validation `Debug` output.
- Resolved role names and unresolved OIDs remain separate identity namespaces. A real role named `"16424"` does not alias raw dangling OID `16424`.
- PUBLIC remains a grantee-only identity. OID zero is rejected by unresolved-role constructors and is never treated as a dangling role.
- Current `pg_proc.proacl`, converter-function `deptype='e'`, complete `deptype='x'` sets, security labels, exact `pg_init_privs` baseline, immutable raw converter root, and transform-object `deptype='e'` remain separate facts.

### Test and repair evidence

- Initial dangling-role representability finding review: `5254640920`; structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332`; production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- Diagnostic-projection finding review: `5254787838`; structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`; production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe`.
- Validation-boundary finding review: `5254918439`; structural RED `21817d01f0aaa69902699eb21109c2a057401a8c`; production repair `97ff3903278bafbd2133a88ded475ca481e8b1b7`; public composition `da84d2afc99d41f7af9273cd192047704d779a47`.
- Non-forgeable validation-evidence finding review: `5254963033`; structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38`; production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc`.
- Detached-material replay finding review: `5255023556`; structural RED `8734883edcb145933030cccd7177cff95abde772`; receipt-input repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2`.
- Complete-receipt finding review: `5255032673`; structural RED `c5f9686ec16c27311abeefd858b0d317ac3c4d6d`; production repair `b466a6fcb207061684e104a51010fed6a0bbca55`.
- Exact dangling-role remediation finding review: `5255447654` at exact pre-finding head `440d5e3f1cd12011047025c499aec76792c95c83`; structural RED `f23aefdcb8416c9828d2a46c12fcf3b6190fcd47`; material repair `b7b9e3d1b17a840436fbde7420f88a9c9539335f`; validation repair `57b5b209e9ade1f84c1c036a2bca81e7dacbac73`; rustdoc/log-safety follow-up `a49243baddbc3142286de4c38c29bd5dd06cbdcb`; canonical-set contract `dd8d5adf65638ae5c5e2f61686b37d14df17062c`.
- The focused validation contract now requires exact receipt-field retention, source/location-bound row absence, exact present-material digest binding, exact dangling-role identity sets, equal-count/different-identity separation, canonical sorting/deduplication, damaged-state blocking, and routine debug redaction.
- Source/documentation repair itself is not native or hosted GREEN evidence. Exact-head Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and PostgreSQL 18 bounded live differential remain gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, initial-privilege baselines, immutable raw converter-root lineage, and independent transform-object extension membership.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Stable central owner lanes remain `.github#2279` (GitHub API URL/redirect authority), `.github#2271` (CodeQL dispatch repository identity), `.github#2268` (queue-health repository identity), and `.github#2040` (scheduler reconciliation), with product bootstrap #35 downstream. Their mutable heads, runs, reviews, and mergeability are live owner evidence and are not versioned as ConceptWeave product truth.
