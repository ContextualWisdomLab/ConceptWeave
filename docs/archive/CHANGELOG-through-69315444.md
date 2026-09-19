# Changelog

The immediately preceding Source Observation/recovery-validation decision surface is preserved under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Unreleased

### Added

- Retained the converter-function `pg_init_privs` successor: exact row absence/presence, `privtype` (`i`/`e`), complete source-order object-level initial EXECUTE ACL, immutable receipt/snapshot, exact converter binding, and raw converter-root lineage remain authoritative.
- The initial-privilege material retains the complete source-order grant vector after digest construction, including repeated ACL entries when they exist in the source array. `grants()` exposes the observed ACL sequence and multiplicity rather than a normalized set.
- Resolved ACL role identity now retains the exact same-generation PostgreSQL role OID privately alongside its readable role name. PostgreSQL `AclItem` is OID-based and `pg_authid` stores OID and `rolname` separately, so equal names after role recreation no longer alias the earlier ACLITEM identity.
- The initial-privilege material digest is versioned to `.material.v2`. Resolved grantee/grantor variants commit both role OID and role name, while PUBLIC and unresolved variants remain explicitly tagged.
- Added log-safe grant semantics accessors for PUBLIC shape, resolved grantee role, unresolved-grantee presence, resolved grantor role, unresolved-grantor presence, and grant option. Numeric resolved OIDs remain private identity material; raw dangling OIDs remain purpose-bound to receipt-backed recovery validation.
- Retained hostile dangling-role identity coverage for catalog states where an initial ACL still refers to a role OID after the role no longer exists.
- Retained canonical sorted/deduplicated unresolved grantee/grantor OID sets inside initial-privilege material so recovery evidence preserves the exact missing PostgreSQL role identities rather than only aggregate counts. These remediation sets do not reorder or deduplicate the source ACL itself.
- Added purpose-bound `unresolved_grantee_oids()` and `unresolved_grantor_oids()` accessors on non-forgeable recovery-validation evidence. `unresolved_grantee_count()` / `unresolved_grantor_count()` are projections of those canonical identity sets.
- Added custom grant/material/validation `Debug` implementations that deliberately omit numeric role OIDs while retaining readable names, PUBLIC/unresolved shape, counts, immutable digest/provenance, and explicit typed recovery accessors.
- Retained the canonical initial-privilege recovery validator through the relation/index-partition public surface. Row absence and fully resolved material validate ready; any unresolved grantee or grantor identity blocks readiness.
- Retained complete non-secret receipt provenance in every verdict: source registry identity, connection-policy binding, source-content digest, extractor revision, observation timestamp, and canonical converter location. Present rows additionally carry the immutable material digest and exact canonical dangling-role sets.
- `matches_source_receipt()` compares complete receipt provenance, material absence/presence identity, and exact dangling-role identity sets.
- Added focused contracts proving equal unresolved counts with different OIDs remain distinguishable and repeated OIDs canonicalize deterministically in remediation sets.
- Updated initial-privilege traceability with PostgreSQL 18's ACL-array ordering, multiplicity, and OID/name identity boundaries.

### Correctness

- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` now requires nonzero exact OIDs for every resolved role identity instead of accepting a resolved role name alone. The OIDs are private and participate in equality/order/digest identity without becoming routine product API.
- Equal resolved role names with different grantee or grantor OIDs now produce distinct material digests. Drop/recreate role events therefore cannot collapse into a name-only ACL identity.
- PUBLIC remains a grantee-only identity. OID zero is rejected wherever a resolved or unresolved role identity is required.
- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial` retains the exact source-order grant sequence so `observe -> validate -> review` can inspect PUBLIC/resolved-role/grant-option/unresolved-kind semantics without falsifying PostgreSQL ACL ordering.
- Removed the material constructor's `grants.sort()` normalization. PostgreSQL 18 treats ACL-array order as significant for client utilities such as `pg_dump`, because grants with grant option may need to precede dependent grants.
- Removed source-level duplicate ACL-entry rejection. PostgreSQL 18 `check_acl()` validates array shape/nullability but does not impose a uniqueness invariant, and `aclnewowner()` explicitly handles ACLs in which duplicate entries can arise. Source Observation preserves repeated identical ACLITEM occurrences in exact array order and hashes their multiplicity.
- Derived dangling-role remediation OID sets remain sorted/deduplicated and do not rewrite the source ACL.
- Recovery readiness is derived only from private validation evidence produced by the canonical validator; callers cannot manufacture a public `Ready` variant.
- Validation consumes an owner-issued source receipt rather than detached ACL material, preventing evidence replay across converter directions, source snapshots, policies, extractors, or observation epochs.
- Present recovery evidence carries `Some(material.digest())`; row absence carries `None`. Both states retain complete receipt binding, and present damaged rows retain exact missing-role identity sets.
- Raw dangling OIDs are available only through explicit typed recovery accessors and are omitted from routine grant/material/validation `Debug` output.
- A real role named decimal text such as `"16424"` remains distinct from unresolved raw OID `16424`.
- Current `pg_proc.proacl`, converter-function `deptype='e'`, complete `deptype='x'` sets, security labels, exact `pg_init_privs` baseline, immutable raw converter root, and transform-object `deptype='e'` remain separate facts.

### Test and repair evidence

- Initial dangling-role representability finding review: `5254640920`; structural RED `94e98da8ebb31f1429b44b20cce7b96ea16e1332`; production causal repair `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- Diagnostic-projection finding review: `5254787838`; structural RED `6d5796fd2e2b1aa6d1df70ee380a76b24731f04a`; production repair `fde6d3a8776b75e3dd014713b671734b9226a0fe`.
- Validation-boundary finding review: `5254918439`; structural RED `21817d01f0aaa69902699eb21109c2a057401a8c`; production repair `97ff3903278bafbd2133a88ded475ca481e8b1b7`; public composition `da84d2afc99d41f7af9273cd192047704d779a47`.
- Non-forgeable validation-evidence finding review: `5254963033`; structural RED `eeb1692ed8e537b761b7acefbdc825ec242d4b38`; production repair `5206290cc8b7dd7128ef849d1acb315a2334e7fc`.
- Detached-material replay finding review: `5255023556`; structural RED `8734883edcb145933030cccd7177cff95abde772`; receipt-input repair `9c7e7e8a45eb1a75e43e6a5fe35c5e6f8dec6de2`.
- Complete-receipt finding review: `5255032673`; structural RED `c5f9686ec16c27311abeefd858b0d317ac3c4d6d`; production repair `b466a6fcb207061684e104a51010fed6a0bbca55`.
- Exact dangling-role remediation finding review: `5255447654`; structural RED `f23aefdcb8416c9828d2a46c12fcf3b6190fcd47`; material repair `b7b9e3d1b17a840436fbde7420f88a9c9539335f`; validation repair `57b5b209e9ade1f84c1c036a2bca81e7dacbac73`; rustdoc/log-safety follow-up `a49243baddbc3142286de4c38c29bd5dd06cbdcb`; canonical-set contract `dd8d5adf65638ae5c5e2f61686b37d14df17062c`.
- Grant-level raw-OID log-safety finding review: `5255638361`; structural RED `3626a658d45b59802f632845708bf8ec378296cb`; production causal repair `cfe4eba0e68eca6d4f4cf6b50f18478e61d14c23`.
- Complete-ACL readability finding review: `5255771158`; structural RED `3d03f9b9d3ce27ae2d96732ec03780a345493afc`; production causal repair `d406375f8989dae5dc06c50c1f4a776408a00bd8`.
- Source-order integrity finding review: `5255912747`; structural RED `13e51561cac125f2a942e7ca0d0dd39433040426`; production causal repair `966e2136c15aa049f72acfebf2c1a478d09c66ef`. The RED is a source-level contract and is not claimed as an executed failing run.
- ACL-multiplicity finding review: `5256059436`; corrected structural RED `9747763c4b2370185bd18039b00e8ceaa9df206c`; production causal repair `55f828f6f6065fac89ae63f521d72d807f875c5b`. Earlier test-only `3ece5938...` is superseded and is not evidence.
- Resolved-role OID identity finding review: `5256559272` at exact pre-finding head `a82dced2a154e1ff105864309a74559255898bbc`; structural RED `b1c8fef6a526b22566b65d8f7962c3af0ebd2031`; production causal repair `52e1c12d5e592e624d8d55badbdc24c52b428a00`; retained-contract constructor adoption `2780f5edb6c086a3d13b0dc47d6ca4b22963eec9`; dangling-role adaptation `a7274b7de87f904b8aae7fbdf0208ed8b3066d36`; recovery-fixture adaptation `8ba4e304fa4b24cb4e49ae31474985b400aeddc7`.
- The OID-identity structural RED is source-level evidence of the missing contract, not an executed failing CI run. Source/documentation repair itself is likewise not native or hosted GREEN evidence.
- Exact-head Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and PostgreSQL 18 bounded live differential remain gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, source-order and multiplicity-preserving initial-privilege baselines, immutable raw converter-root lineage, and independent transform-object extension membership.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Stable central owner lanes remain `.github#2279` (GitHub API URL/redirect authority), `.github#2271` (CodeQL dispatch repository identity), `.github#2268` (queue-health repository identity), and `.github#2040` (scheduler reconciliation), with product bootstrap #35 downstream. Their mutable heads, runs, reviews, and mergeability are live owner evidence and are not versioned as ConceptWeave product truth.