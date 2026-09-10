# Product / Technical Gap Baseline

**Current snapshot:** 2026-09-10

This is the code-current ConceptWeave gap baseline for this branch. Exact SHA, run, review and PR coordinates are evidence snapshots, not mutable dependencies. Live protected branches and owner PRs supersede a recorded coordinate when they advance. Earlier snapshots remain in Git history as historical evidence; they are not current authority. GitHub's computed `mergeable` flag is volatile mechanical state and is never acceptance evidence.

This refresh follows procedural self-application source head `898cd641e60426541b567d02ba090b20249c88b2`. The commit containing this file is a documentation-only successor, so source-head local evidence does not automatically become exact-current whole-head evidence.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic release, and the canonical client contract for consuming those releases. It does not copy foreign domain truth.

- `semantic-data-portal`: catalog, governance and consumption surfaces.
- `context-graph-contracts`: released interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/model/provider routing and capability ownership.
- Noema: execution-local procedural projection, lifecycle and runtime-authorization integration.
- Keyverse: identity/credential authority.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and current prerequisites

ConceptWeave protected `main` is `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`. It remains repository bootstrap only; there is no protected immutable ConceptWeave semantic release yet.

Central protected `.github/main` is `cb0872c9a20d5584703dffacca65c096fc034c6c`, merged from `.github#1938` on 2026-09-10.

### Product-CI bootstrap #35

Exact head `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN non-Draft on protected ConceptWeave `main`.

The branch repairs two bootstrap defects: Draft PRs are no longer suppressed from Product validation, and dynamic `npx --yes ajv-cli` execution is replaced by lockfile-pinned `ajv@8.20.0` plus a repository-owned in-process validator.

Current central required-workflow evidence remains mixed:

- Security Scan `34434790791`: SUCCESS.
- SAST Semgrep `34434790777`: SUCCESS.
- CodeQL PR `34434790860`: FAILURE.
- CodeQL language detection succeeded; compatibility analysis failed waiting for authenticated current-head settlement; a later dispatch job succeeded after the compatibility job had already failed.

This is a stable consumer reproduction of the central settlement gap. Do not manufacture a leaf GREEN through no-op source pushes, manual reruns or Ready/Draft toggles while the same central contract is protected.

### Central CodeQL owner stack

`.github#2040@6706c231ab06a3c91c43fdb5b989cfcd79fff593` remains diverged from protected `cb0872c...`: **144 commits ahead / 32 behind**, merge base `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` is **18 ahead / 32 behind** current protected main and remains OPEN Draft. It preserves one post-matrix wake coordinator, exact PR/head/base/run identity, stricter `{base_ref, base_sha}` binding and versioned rollout work.

`.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains stacked on #2051 and preserves complete-failed-job-set validation plus one atomic run-wide wake. It must adopt a repaired exact #2051 successor rather than independently attaching to protected main.

Owner repair must preserve authenticated repository/PR/head/base/required-run/language/job/SARIF/artifact evidence, boundedly converge the complete terminal job set, and perform one run-wide wake before producing terminal receipt. Old-base hosted evidence does not transfer after ancestry repair.

Required central order: ordinary non-force adoption of protected `cb0872c...` -> reproduce current consumer RED -> minimal settlement repair -> reconcile valid #2051/#2056 identity/wake deltas -> exact-current central GREEN and qualifying independent review -> normal protected integration -> fresh unchanged-head #35 acceptance.

## Current ConceptWeave roots

- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft. Product bootstrap #35 is its direct prerequisite. Foundation's Product definition and `scripts/check_ci_contract.py` must be repaired together after #35 integrates so Draft validation and executable contract agree.
- Client Consumption #5: `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft on Foundation. It owns deterministic immutable semantic-release admission/diff/compatibility/supersession, not physical execution or foreign business truth.
- Source Observation #6: `6c1157efad5d2e4258330d6485ff2ea980ae331b`, OPEN Draft on #5. Its next source slice is PostgreSQL representation/versioning/authorization before transport.
- Research Intake #9: `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft on Foundation. `ClassificationReport` remains private and constructor-bound with read-only accessors.
- Pending-source #40: `4efe15c6318d8cb65c52a974a2c105363a4c82a5`, OPEN Draft on #9.
- Golden-set #10: `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4`, OPEN Draft from historical #9 ancestry; it remains a semantic reconciliation finding rather than a closure candidate.
- Procedural authoring #43: `2c6d3037acdeae2b152c335ee2f60a59b4472831`, OPEN Draft on Foundation. It owns local draft/revision schemas, locked AJV fixture validation and Draft-safe Product/CI-contract source repair; it is not a production self-modifying runtime.
- Procedural self-application #44: source head `898cd641e60426541b567d02ba090b20249c88b2`, OPEN Draft on #43. It adds inferred self-application profiles, a Rust topology/evidence-membership candidate, local-Git source-provenance verification and strict duplicate-key checked-in artifact admission.

## Source Observation P0 — PostgreSQL 18 representation before transport

Current snapshot framing is `conceptweave.postgres_schema_snapshot.v2`; current evidence is table/column/constraint oriented and `ColumnObservation` still carries human-readable `data_type` rather than exact qualified type identity. Do not silently redefine v2.

Required RED before representation source:

- frozen historical v2 fixture reproduces original v2 digest and coordinate meaning;
- expanded evidence uses an explicit v3/equivalent digest/receipt family;
- relation kind/comment, first-class index semantics and schema-scoped domain/enum evidence alter successor identity when material facts differ;
- type-only schemas are checked directly against the exact authorized schema allowlist before immutable side effects;
- same-name types in different schemas remain distinct and columns bind to an exact qualified source-type coordinate rather than display text or `search_path`;
- enum ordering and material domain base/default/null/collation/check semantics are digest material;
- input-order permutations of the same complete evidence remain identity-stable;
- fake table-scoped type coordinates fail closed.

Only after this representation/version/authorization slice is exact-head Rust GREEN may the concrete PostgreSQL adapter be admitted. Transport then requires least-privilege key+binding resolution, stale-binding rejection before source I/O, one non-resetting budget, explicit `REPEATABLE READ READ ONLY`, bounded rows/bytes/concurrency, cancellation cleanup and complete-or-fail snapshot construction.

## Procedural semantic engineering — self-application state

The checked-in profiles remain inferred Draft artifacts, KO/EN authoring only, with runtime activation disabled. Shape or topology validation never grants execution or publication authority.

Three artifact-admission defects now have explicit RED -> repair evidence on #44:

1. **Self-consistency was not immutable provenance.** RED `86c74f66098ae83df935b11ad250f3a02ff8a732` showed coordinated source-digest/profile-reference/profile-digest rewriting could remain internally consistent. GREEN `a963a1f24902347f7ec8aca9fd9bee8837986ce7` resolves exact local-Git `commit:path`, requires the recorded blob identity/type, bounds source bytes and checks exact SHA-256.
2. **Object existence was not authored-history membership.** RED `a7e6b821262671c0f72974bfc86ff8b6fd935227` reproduced a dangling/orphan source commit whose object remained locally. GREEN `e22647e6f45f7244552c0e2d74f86dcc8ac0e85c` verifies repository top-level identity, strips ambient `GIT_*` overrides and requires each source commit to be an ancestor of exact checked-out `HEAD` before path/blob/byte verification.
3. **Ordinary JSON parsing collapsed duplicate object members before admission.** Review `5163384790` records the unchanged-head RED: Node 22.16 last-wins parsing discarded duplicate `model_id`/nested scope evidence before AJV or provenance logic could observe it. Source head `898cd641e60426541b567d02ba090b20249c88b2` adds `scripts/parse_strict_json.mjs`, bounded to 2 MiB and depth 128, rejects duplicate decoded member names at every object depth including escaped-equivalent names, and then delegates ordinary syntax/value parsing to `JSON.parse`. `check_semantic_engineering_profiles.mjs` now uses this reader for the manifest and both checked-in profiles. Six local Node contract tests cover valid JSON, top-level/nested/escaped-equivalent duplicates, malformed/trailing input, excessive nesting and oversize. This is checked-in artifact admission only, not the future Rust production transport.

The manifest deliberately remains `source_authentication: not_established_by_manifest`: metadata cannot authenticate itself. The external local-Git verifier establishes only checked-in design-source provenance. It does not establish factual truth, independent evaluation, steward approval, publication or runtime activation.

Remaining procedural admission gaps include a production Rust transport/domain adapter with duplicate-key preservation/rejection, profile-to-domain conformance, authenticated expected tenant/task/base/revision rather than equality to caller-provided values, annotation semantics, released semantic/tool-contract resolution, independent evaluation, retained rejected revisions, steward decision binding, immutable/CAS publication, revocation/rollback and released Noema projection.

The Rust topology candidate has 20 native contract tests in source, but no Rust compiler is available in the present local execution environment and no exact-current Product generation has been observed for this stacked child. Do not promote those tests or predecessor results to whole-head GREEN. The module remains deliberately non-public until native evidence and the semantic admission boundary are complete.

## Research-stack root repair

Golden-set #10 remains a repair finding. Preserve valid golden-label/approval/evaluation source/test/fixture work while adopting current #9 through ordinary non-force semantic integration. Do not reopen #9's private trusted `ClassificationReport` fields to satisfy historical tests. Corruption cases belong behind a test-only or explicitly untrusted wire seam, and raw provider provenance must remain backward compatible. After repaired #10 exact-head verification, descendants adopt the repaired successor and reset evidence.

## Quality, security and release invariants

- Owned production doc/rustdoc, tests and edge-case coverage remain 100% requirements; raw diagnostic counters never justify inventing code or weakening denominators.
- Math/psychometrics/data/performance/security hot paths remain Rust-first. Synthetic data is unit-test evidence only.
- Provider/model routing stays behind released contextual-orchestrator APIs; proposed/inferred model output never becomes authoritative before steward validation/publication.
- Published semantic truth is immutable; correction produces a new release plus supersession evidence.
- External DTOs cross explicit ACLs; product/domain truth stays with its canonical owner. No generic cross-domain source copy or cross-service SQL.
- Purpose-bound PII, least privilege, structured audit evidence, recovery and reproducibility are release criteria, not post-release paperwork.
- Material buyer-facing web/API paths require realistic async+k6/E2E p95 <= 20 ms where applicable; genuine bottlenecks are profiled rather than hidden by sample reduction or unrealistic cache warm-up.
- Material UI requires product-specific component composition, normal/loading/empty/error/permission/responsive/interaction/a11y states and locale validation across KO/EN/JA/ZH/VI/ES/DE/FR before completion claims.

No release is currently authorized. Release-ready protected head must still execute version/CHANGELOG/tag/package, immutable semantic release, SBOM/provenance/signing, reproducibility and rollback evidence.

## Current execution order

1. Central `.github` owners adopt protected `cb0872c...` non-force, repair terminal settlement with the stable #35 consumer RED, reconcile #2051/#2056 valid deltas, obtain exact-current GREEN/review and integrate normally.
2. Re-run #35 unchanged exact head against the protected central repair; require terminal required checks, zero valid findings and qualifying independent review before normal merge.
3. Restack Foundation #1 onto protected Product bootstrap non-force and repair its Product workflow plus executable CI-contract guard as one parent prerequisite; obtain fresh Foundation exact-head evidence.
4. Restack dependent #5/#9/#43 and descendants normally and reset evidence. #44 retains the three artifact-admission RED/repair lineages.
5. On #44, progress the Rust production transport/domain admission from executable REDs; do not treat the JS artifact reader as production domain validation.
6. Independently progress Source Observation from frozen-v2/versioned-representation/type-authorization RED; do not add transport first.
7. Repair Golden-set #10 against current #9 semantically, then propagate descendants without closure or whole-tree ours/theirs shortcuts.
8. Continue ontology discovery, semantic-layer discovery, alignment/matching, deterministic validation, governance persistence, review/publication, client explain/match/resolve/validate/query-plan contracts, multilingual evaluation, observability/recovery and buyer-path performance only through their canonical owner seams.

Force push, destructive rebase, self-approval, review dismissal, gate weakening, blind/manual rerun, synthetic status, mutable supplier dependency, live Zotero mutation, premature semantic publication and runtime activation are not acceptance mechanisms.
