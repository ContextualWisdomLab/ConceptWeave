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
- Canonical `.github#2106` has merged and is no longer the open workflow-owner prerequisite. The active owner successor is `.github#2040@609be40b7be3a53ac5a8baf2b48b3af5ad7da237` against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`; fresh compare is diverged, 163 ahead / 244 behind with merge base `fb17ef556f94f673234aa557254ae52779e9a7b0`.
- Owner review `5238816675` preserves #2040 commit `a9b18b4b24980c7ceb8b8cc0d143a24db20c90bf`, which removes source-neutral same-tree scheduler restamps for last-push approval and zero-job Actions startup failure. Current-base reconciliation must not reintroduce wake/no-op commits.
- Fresh owner review `5239422933` found that predecessor `b1542f0e...` had not inherited protected main's private-consumer CodeQL read grants. Structural RED `2a7e1252a57fd2aee934bb9c531b4cbecfc49f66` pins both job contracts; minimal production repair `9f435af94de5e02897b8d132105dea5f03b33547` adds only `pull-requests: read` and `statuses: read` to `analyze-head` and `dispatch-current-head`, preserving the v2 producer, base-bound evidence, rerun/job-set, OIDC/app-token, and no-source-neutral-restamp behavior.
- Current repository-identity P1 remains RED at `609be40b...`: protected main rejects repository owner/name components containing `..` or ending in `.`, while the branch production constant is still permissive and its exact regression contract rejects `ContextualWisdomLab/..`, `ContextualWisdomLab/repository.`, `ContextualWisdomLab./repository`, and `../repository`.
- Child `d278f972a018f813d11a5684d4ba16c32bb6f45c` attempted the one-line repository-identity repair, but exact diff review found collateral deletion of non-obvious scheduler rationale comments. Ordinary-forward repair `609be40b7be3a53ac5a8baf2b48b3af5ad7da237` restored the predecessor scheduler-core blob byte-for-byte. Current-head owner review `5241404620` records the failed-minimality check; no source GREEN or approval is claimed.
- Exact `.github#2040@609be40b...` currently has no PR-triggered workflow generation and no qualifying independent current-head approval. No owner GREEN is claimed.
- Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`: SAST Semgrep `34434790777` and Security Scan `34434790791` are terminal success, while CodeQL PR `34434790860` is terminal failure. Its actions compatibility consumer enforced `VERDICT_STATE=pending` before the later dispatch job completed successfully; this remains central owner-settlement evidence rather than a leaf-source defect or justification for a manual rerun.
- No executed Rust GREEN is claimed. Repository-pinned Rust 1.98 native validation, hosted terminal checks, workspace/doc tests, release build, rustdoc, and owned coverage remain open.
- The bounded PostgreSQL 18 differential must independently capture converter raw `proleakproof` in the same exact `pg_proc` generation as definition/owner/ACL/`proconfig`/`prosecdef` and all retained ordinary-EXCLUDE facts.
- Remaining converter strictness, volatility, parallel safety, planner support/cost, and other independently mutable auxiliary fields stay review-gated until this successor has exact-head native/hosted acceptance plus bounded differential evidence.
