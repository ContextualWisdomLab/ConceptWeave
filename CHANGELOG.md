# Changelog

All notable ConceptWeave changes through exact `51820de79986f83e8417efd7196159f59f4be1fb` are preserved losslessly at `docs/archive/CHANGELOG-through-51820de7.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-51820de7.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence now independently preserves every nonzero converter function's raw `pg_proc.prosecdef` after exact converter definition, owner, object-level `EXECUTE` ACL, and nullable `proconfig` identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation` records raw invoker/definer state for one exact converter direction.
- `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot` requires complete same-generation security-context coverage for every exact converter direction in the configuration predecessor and issues exact provenance receipts.

### Correctness

- Equal converter signature, implementation, owner, ACL, and local configuration no longer imply equal execution privilege context. PostgreSQL can switch a function between `SECURITY INVOKER` and `SECURITY DEFINER` without changing those predecessor facts.
- Both Boolean states remain representable; Source Observation does not invent an invoker-only admission policy.
- `prosecdef` is not inferred from owner, ACL, `proconfig`, language, definition material, or caller identity.
- Missing or extra converter directions, duplicate coordinates, converter-function binding drift, zero positions, and unknown receipt coordinates fail closed.

### Retained

- The complete pre-security-context decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-51820de7.md` and `docs/archive/CHANGELOG-through-51820de7.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity, operator/function facts, target-function auxiliary facts, selected transform rows, converter definition, owner, ACL and `proconfig` identity, operator-family/strategy, backing-index controls, and exact v3 source-content-generation binding.

### Acceptance

- Finding review `5235250993` produced structural source/compile RED `96aeb1b07cf32a225db250c570a5a6ff120221be`, production successor `6bfa27f7c20e74dfa870515850bcb04424dad4fe`, and public composition `4652d52ad538bf17ba21bd1d8c8ad3b678a5d0d4`.
- PostgreSQL/NIST/APA rationale and TRACEABILITY are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-security-definer-integrity.md`.
- No executed Rust RED/GREEN is claimed on the moving exact head. Repository-pinned Rust 1.98 is absent from the current execution host; native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter raw `prosecdef` in the same generation as converter definition/owner/ACL/`proconfig` and all retained ordinary-EXCLUDE facts.
- Converter `pg_proc.proleakproof` is the next identified independently mutable converter-function auxiliary surface after security-context acceptance and live differential.
