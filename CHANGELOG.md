# Changelog

All notable ConceptWeave changes through exact `d35374cba690503d45b96079df84b5d945d4df42` are preserved losslessly at `docs/archive/CHANGELOG-through-d35374cb.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-d35374cb.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence now independently preserves every nonzero converter function's `pg_proc.proacl` after exact converter definition and owner identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant` distinguishes `PUBLIC` and named role grantees while retaining same-generation grantor role and grant-option state.
- `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial` preserves raw `proacl IS NULL` versus explicit ACL state and reduces the canonical object-level `EXECUTE` grant set to a domain-separated digest.
- `IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot` requires complete ACL coverage for every exact converter direction in the owner predecessor and issues exact provenance receipts.

### Correctness

- Equal converter signature, implementation, and owner no longer imply equal authorization-source identity. `GRANT`/`REVOKE EXECUTE` can change function privileges without redefining those predecessor facts.
- Missing or extra converter directions, duplicate coordinates, converter-function binding drift, zero positions, malformed role identifiers, duplicate grants, and unknown receipt coordinates fail closed.
- ACL evidence is not inferred from target-function ACL, schema/transform ownership, session identity, `SECURITY DEFINER`, or role-membership closure.
- Runtime caller-specific effective permission remains outside Source Observation; this layer records the converter object's own ACL source facts.

### Retained

- The complete pre-converter-ACL decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-d35374cb.md` and `docs/archive/CHANGELOG-through-d35374cb.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity, operator/function facts, target-function auxiliary facts, selected transform rows, converter definition and owner identity, operator-family/strategy, backing-index controls, and exact v3 source-content-generation binding.

### Acceptance

- Finding review `5234509421` produced structural source/compile RED `8b65892785b5c4b1db0f30812d1a010fff6ac71f`, production successor `c6d8f4ca6c49ec3be44fd797df5f6ce9f8d83a0b`, and public composition `816621e01667b76e22fe7633a56bf50428d6c2f8`.
- No executed Rust RED/GREEN is claimed. Repository-pinned Rust 1.98 is unavailable in the current runtime, so `fmt`, strict workspace/all-target Clippy, focused/retained tests, workspace/doc tests, release build, rustdoc, and owned coverage remain open on the current exact head.
- The bounded PostgreSQL 18 differential must independently capture converter `proacl` raw NULL-state and canonical object-level `EXECUTE` grant evidence in the same generation as converter definition/owner and all retained ordinary-EXCLUDE facts.
- Nullable converter `proconfig` is the next identified independently mutable converter-function auxiliary surface after converter-ACL acceptance and live differential.
