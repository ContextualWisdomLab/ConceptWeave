# Changelog

All notable ConceptWeave changes through exact `4256f8dc361ae8ed00980799d524a261b161b1a8` are preserved losslessly at `docs/archive/CHANGELOG-through-4256f8dc.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-4256f8dc.md`. Earlier release-era and Source Observation history remains under `docs/archive/`. This active changelog records the current Source Observation delta without deleting archived history.

## Unreleased

### Added

- PostgreSQL ordinary `EXCLUDE` transform-converter evidence independently preserves every nonzero converter function's raw same-row `pg_proc.proleakproof` after exact converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, and raw `prosecdef` identity.
- `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation` records raw leakproof classification for one exact converter direction.
- `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot` requires complete same-generation leakproof coverage for every exact converter direction in the security-definer predecessor and issues exact provenance receipts.

### Correctness

- Equal converter signature, implementation, owner, ACL, local configuration, and invoker/definer context no longer imply equal leakproof identity. PostgreSQL can switch a function with `ALTER FUNCTION ... [NOT] LEAKPROOF` without changing those predecessor facts.
- Leakproof provenance locations now encode transform-type schema/name components independently. Valid PostgreSQL quoted identifiers such as `(payload.domain, json)` and `(payload, domain.json)` can no longer collapse to the same raw `schema.type` location string.
- The location repair preserves the existing leakproof digest domain and typed receipt lookup identity; it changes only the provenance/error string representation for separator-capable identifiers.
- Both leakproof Boolean states remain representable; Source Observation does not invent a leakproof-only admission policy.
- Missing or extra converter directions, duplicate coordinates, converter-function binding drift, zero positions, and unknown receipt coordinates fail closed.

### Retained

- The complete pre-location-repair decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-4256f8dc.md` and `docs/archive/CHANGELOG-through-4256f8dc.md`.
- All prior ordinary-EXCLUDE authority remains in force, including exact constraint/backing-index identity, operator/function facts, target-function auxiliary facts, selected transform rows, converter definition, owner, ACL, `proconfig`, `prosecdef`, `proleakproof`, operator-family/strategy, backing-index controls, and exact v3 source-content-generation binding.

### Acceptance

- Leakproof finding review `5235830453` produced structural source/compile RED `ce86f3c7e84054a0c58e38f81a8b230c3e2653b3`, production successor `14ebe48407ab448586f68dbce8b2cd55b32ea5ba`, and public composition `5a8229d6d04e3d4498cac61b0603eacc2e4398bc`.
- Provenance-location review `5236499737` produced realistic collision RED `008c99a2e82105046df0b9ed7061bc6cb931ebb9` and minimum causal repair `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81`.
- PostgreSQL/NIST/APA rationale and TRACEABILITY, including PostgreSQL 18 quoted-identifier lexical authority, are current in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-leakproof-integrity.md`.
- Canonical `.github#2106` current-base reconciliation finding `5237380716` was repaired ordinary-forward at exact `0e9412f93bb9d4a08a689f62a563dc666ae91f88` by adopting protected `.github/main@4e56ff0fd10e8d56e9af273881a7eb65d7e4f513`'s OpenCode contract-test blob unchanged. The effective owner delta is eight paths, 57 ahead / 0 behind and mergeable. Exact-head CodeQL PR `35236110056`, SAST Semgrep `35236109832`, Security Scan `35236109955`, Python Security `35236109883`, and Runtime Quality `35236110143` are queued/nonterminal; qualifying independent current-head approval remains absent.
- No executed Rust GREEN is claimed. Repository-pinned Rust 1.98 native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter raw `proleakproof` in the same exact `pg_proc` generation as definition/owner/ACL/`proconfig`/`prosecdef` and all retained ordinary-EXCLUDE facts.
- Remaining converter strictness, volatility, parallel safety, planner support/cost, and other independently mutable auxiliary fields stay review-gated until this successor has exact-head native/hosted acceptance plus bounded differential evidence.