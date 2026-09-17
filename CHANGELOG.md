# Changelog

All notable ConceptWeave changes through exact `b5904f6ede97bf433665a45e7da910fc7433b7f8` are preserved losslessly at `docs/archive/CHANGELOG-through-b5904f6e.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-b5904f6e.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence now independently preserves every nonzero converter function's nullable `pg_proc.proconfig` after exact converter definition, owner, and object-level `EXECUTE` ACL identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial` distinguishes `proconfig IS NULL` from explicit arrays and domain-hashes exact array length, entry order, and entry bytes without retaining raw GUC values in receipts.
- `IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot` requires complete same-generation configuration coverage for every exact converter direction in the access-control predecessor and issues exact provenance receipts.

### Correctness

- Equal converter signature, implementation, owner, and ACL no longer imply equal runtime/security configuration identity. PostgreSQL `ALTER FUNCTION ... SET/RESET` can change function-local settings without changing those predecessor facts.
- Configuration is not inferred from current/session GUCs, reconstructed DDL, target-function settings, converter owner, or converter ACL.
- Missing or extra converter directions, duplicate coordinates, converter-function binding drift, zero positions, and unknown receipt coordinates fail closed.
- Raw `NULL`, explicit empty arrays, entry order, and exact entry bytes remain distinguishable source states.

### Retained

- The complete pre-converter-configuration decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-b5904f6e.md` and `docs/archive/CHANGELOG-through-b5904f6e.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity, operator/function facts, target-function auxiliary facts, selected transform rows, converter definition, owner and ACL identity, operator-family/strategy, backing-index controls, and exact v3 source-content-generation binding.

### Acceptance

- Finding review `5235145397` produced structural source/compile RED `b99338232954258f93fd43253e61cd58c76e295b`, production successor `f863ec28ac0d910befbd3c416e9a0ab4797d300c`, and public composition `07053742f96f253cf3b76d1563d500a0e17be232`.
- PostgreSQL/NIST/APA rationale and TRACEABILITY are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-configuration-integrity.md`.
- No executed Rust RED/GREEN is claimed on the moving exact head. Repository-pinned Rust 1.98 native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter `proconfig` raw nullable array identity in the same generation as converter definition/owner/ACL and all retained ordinary-EXCLUDE facts.
- Converter `pg_proc.prosecdef` is the next identified independently mutable converter-function auxiliary surface after configuration acceptance and live differential.
