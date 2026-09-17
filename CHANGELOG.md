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

### Acceptance and canonical-owner coordination

- Leakproof finding review `5235830453` produced structural source/compile RED `ce86f3c7e84054a0c58e38f81a8b230c3e2653b3`, production successor `14ebe48407ab448586f68dbce8b2cd55b32ea5ba`, and public composition `5a8229d6d04e3d4498cac61b0603eacc2e4398bc`.
- Provenance-location review `5236499737` produced realistic collision RED `008c99a2e82105046df0b9ed7061bc6cb931ebb9` and minimum causal repair `3ce7e0bcf6d6a3a0935465147e2eaf5fd3a63e81`.
- PostgreSQL/NIST/APA rationale and TRACEABILITY, including PostgreSQL 18 quoted-identifier lexical authority, are current in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-leakproof-integrity.md`.
- Canonical `.github#2106` has merged and is no longer the open workflow-owner prerequisite. The active owner successor is `.github#2040@b1542f0e21d6709d273af04b80f595a7d0fcdaf7` against protected `.github/main@3449d0020ffac86315ecccfb9d5a1dd3bf421834`; fresh compare is diverged, 156 ahead / 232 behind with merge base `fb17ef556f94f673234aa557254ae52779e9a7b0`.
- Owner review `5238816675` preserves both the focused CodeQL v2 producer repair and #2040 commit `a9b18b4b24980c7ceb8b8cc0d143a24db20c90bf`, which removes source-neutral same-tree scheduler restamps for last-push approval and zero-job Actions startup failure. Protected main still carries those restamp paths, so current-base reconciliation must not reintroduce wake/no-op commits.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`: SAST Semgrep `34434790777` and Security Scan `34434790791` are terminal success, while CodeQL PR `34434790860` is terminal failure. Its actions compatibility consumer enforced `VERDICT_STATE=pending` before the later dispatch job completed successfully; this remains central owner-settlement evidence rather than a leaf-source defect or justification for a manual rerun.
- No executed Rust GREEN is claimed. Repository-pinned Rust 1.98 native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter raw `proleakproof` in the same exact `pg_proc` generation as definition/owner/ACL/`proconfig`/`prosecdef` and all retained ordinary-EXCLUDE facts.
- Remaining converter strictness, volatility, parallel safety, planner support/cost, and other independently mutable auxiliary fields stay review-gated until this successor has exact-head native/hosted acceptance plus bounded differential evidence.
