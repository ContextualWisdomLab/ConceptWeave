# Changelog

All notable ConceptWeave changes through exact `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` are preserved losslessly at `docs/archive/CHANGELOG-through-1153c0b6.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-1153c0b6.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence now independently preserves every nonzero converter function's raw same-row `pg_proc.proleakproof` after exact converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, and raw `prosecdef` identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation` records raw leakproof classification for one exact converter direction.
- `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot` requires complete same-generation leakproof coverage for every exact converter direction in the security-definer predecessor and issues exact provenance receipts.

### Correctness

- Equal converter signature, implementation, owner, ACL, local configuration, and invoker/definer context no longer imply equal leakproof identity. PostgreSQL can switch a function with `ALTER FUNCTION ... [NOT] LEAKPROOF` without changing those predecessor facts.
- Both Boolean states remain representable; Source Observation does not invent a leakproof-only admission policy.
- `proleakproof` is not inferred from `prosecdef`, owner, ACL, `proconfig`, language, definition material, caller identity, or observed plan shape.
- Missing or extra converter directions, duplicate coordinates, converter-function binding drift, zero positions, and unknown receipt coordinates fail closed.

### Retained

- The complete pre-leakproof decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-1153c0b6.md` and `docs/archive/CHANGELOG-through-1153c0b6.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity, operator/function facts, target-function auxiliary facts, selected transform rows, converter definition, owner, ACL, `proconfig`, `prosecdef`, operator-family/strategy, backing-index controls, and exact v3 source-content-generation binding.

### Acceptance

- Finding review `5235830453` produced structural source/compile RED `ce86f3c7e84054a0c58e38f81a8b230c3e2653b3`, production successor `14ebe48407ab448586f68dbce8b2cd55b32ea5ba`, and public composition `5a8229d6d04e3d4498cac61b0603eacc2e4398bc`.
- PostgreSQL/NIST/APA rationale and TRACEABILITY are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-leakproof-integrity.md` at `e5006f422f326f818440ed9a37c033c9dd0534fc`; the leakproof decision surface was first currentized at `aa54935b00de82350f871c684a5fe76b8efb1d19`.
- Canonical `.github/main` then advanced to `4fda7f504e58f72f0d9120c83b7da2b5cc824f25` while owner PR #2106 remained at `65a71c5965d0e2153464419272c9ccf019b0a9ad`; active gap state was ordinary-forward corrected at `f89a24613bdfec3d94395ccb4b04fc4d69d1b7e6` to mark canonical owner restack open instead of transferring predecessor-head acceptance.
- No executed Rust RED/GREEN is claimed on the moving exact head. Repository-pinned Rust 1.98 native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter raw `proleakproof` in the same exact `pg_proc` generation as definition/owner/ACL/`proconfig`/`prosecdef` and all retained ordinary-EXCLUDE facts.
- Remaining converter strictness, volatility, parallel safety, planner support/cost, and other independently mutable auxiliary fields stay review-gated until this successor has exact-head native/hosted acceptance plus bounded differential evidence.
