# Product / Technical Gap Baseline

**Current snapshot:** 2026-09-11

This is the code-current ConceptWeave gap baseline for this branch. Exact SHA, run, review and PR coordinates are evidence snapshots, not mutable dependencies. Live protected branches and owner PRs supersede recorded coordinates when they advance. Earlier snapshots remain historical evidence and do not become current acceptance by inheritance. GitHub's computed `mergeable` flag is volatile mechanical state, never acceptance evidence.

This refresh adopts the procedural self-application sequence through semantic projection and the first private Rust transport-admission slice. Review `5169924981` identified that the Rust validator assumed strict JSON had already been established outside the production-language boundary. RED scaffold `b8f9e68532ba3a286c3e2d1a9ba3817e8e080ca9` plus regression `07efa642ade33af2bcd37aa270c93a2dc5d7d6f8` makes duplicate decoded members, malformed strings/surrogates/trailing data, depth/size bounds and fixed diagnostics explicit. Source repair `1100972a1bfbe106d877a40a4fd38732cbe5d969` implements a std-only strict JSON recognizer, and test successor `04bc5c28865c40db2c67c5f8e568ff679f304cb2` broadens grammar/escape/boundary cases. Native Rust/Product acceptance is still unproven for this lineage.

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

`.github#2040@6706c231ab06a3c91c43fdb5b989cfcd79fff593` remains a historical-base preservation input rather than current protected acceptance evidence.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` remains OPEN Draft on historical base `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`. It preserves one post-matrix wake coordinator, exact PR/head/base/run identity and stricter `{base_ref, base_sha}` binding, but also records the deterministic rollout/bootstrap defect: `repository_dispatch` executes the protected default-branch handler, so a combined PR-head client+handler protocol change cannot self-bootstrap when the protected handler does not yet emit the new run identity.

`.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains stacked on #2051 and preserves complete-failed-job-set validation plus one atomic run-wide wake. It must adopt a repaired exact #2051 successor rather than independently attaching to protected main.

The owner repair order is a versioned backward-compatible protected handler first, preserving the currently protected client; then ordinary non-force reconciliation of #2051/#2056 onto current protected truth; then one fresh unchanged-head terminal generation before legacy handler cleanup. Old-base hosted evidence does not transfer after ancestry repair.

## Current ConceptWeave roots

- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft. Product bootstrap #35 is its direct prerequisite. Foundation's Product definition and `scripts/check_ci_contract.py` must be repaired together after #35 integrates so Draft validation and executable contract agree.
- Client Consumption #5: `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft on Foundation. It owns deterministic immutable semantic-release admission/diff/compatibility/supersession, not physical execution or foreign business truth.
- Source Observation #6: `6c1157efad5d2e4258330d6485ff2ea980ae331b`, OPEN Draft on #5. Its next source slice is PostgreSQL representation/versioning/authorization before transport.
- Research Intake #9: `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft on Foundation. `ClassificationReport` remains private and constructor-bound with read-only accessors.
- Pending-source #40: `4efe15c6318d8cb65c52a974a2c105363a4c82a5`, OPEN Draft on #9.
- Golden-set #10: `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4`, OPEN Draft from historical #9 ancestry; it remains a semantic reconciliation finding rather than a closure candidate.
- Procedural authoring #43: `2c6d3037acdeae2b152c335ee2f60a59b4472831`, OPEN Draft on Foundation. It owns local draft/revision schemas, locked AJV fixture validation and Draft-safe Product/CI-contract source repair; it is not a production self-modifying runtime.
- Procedural self-application #44: current branch successor of Rust transport source/test `04bc5c28865c40db2c67c5f8e568ff679f304cb2`, OPEN Draft on #43. It adds inferred self-application profiles, strict checked-in artifact/provenance admission, an unpublished Rust semantic/topology projection, and a private strict JSON transport recognizer. None of these promote artifact coordinates or parsed input to authority.

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

The checked-in profiles remain inferred Draft artifacts, KO/EN authored labels only, with runtime activation disabled. Shape, provenance, transport recognition, semantic projection or topology validation never grants execution or publication authority.

Nine defects now have explicit RED/finding -> repair lineages on #44:

1. **Self-consistency was not immutable provenance.** RED `86c74f66098ae83df935b11ad250f3a02ff8a732` -> GREEN `a963a1f24902347f7ec8aca9fd9bee8837986ce7` resolves exact local-Git `commit:path`, requires recorded blob identity/type, bounds source bytes and checks exact SHA-256.
2. **Object existence was not authored-history membership.** RED `a7e6b821262671c0f72974bfc86ff8b6fd935227` -> GREEN `e22647e6f45f7244552c0e2d74f86dcc8ac0e85c` verifies repository top-level identity, strips ambient `GIT_*` overrides and requires each source commit to be an ancestor of exact checked-out `HEAD`.
3. **Ordinary JSON parsing collapsed duplicate object members before admission.** Review `5163384790` -> source `898cd641e60426541b567d02ba090b20249c88b2` adds the bounded duplicate-decoded-key reader used for the manifest and profiles before AJV/provenance.
4. **A Git blob was not necessarily a regular source file.** Review `5163577072` -> RED `3fd1873bcbb29d74308131bde98a8503561d72d6` -> repair `aab84a9d72686fc4de8c43ba4d9e1548c3cad9da` -> source/test `484db0083c9604cd41112c4a35ae9c31f3d667cc` binds tree mode/object/path and admits only `100644`/`100755`, rejecting `120000` symlinks. Synthetic Node slice: 2/2 passed.
5. **Fail-closed duplicate detection could amplify untrusted content into diagnostics.** Review `5164233960` -> RED `4fac2eb62f5484c20bce44c5abfc57b93592a596` -> GREEN `10799a65325319c6ff337814b23d9d7a48d82fcd` replaces raw duplicate names with a short SHA-256 fingerprint plus byte count. Local strict-JSON slice: 7/7 passed.
6. **Local repository integrity was exposed as generic source authentication.** Review `5164920106` -> RED `c476ce244a6e6774a8b1d12c7b38613deaada2db` -> GREEN `abb8d7f98973d6ba77799b9bbc69d82353f48454` narrows the receipt to `local_git_provenance_established`; the manifest remains `source_authentication: not_established_by_manifest`.
7. **Validator definitions themselves admitted ambiguous JSON.** Review `5165560999` -> RED wiring `49bfbc90d985c3cfaee8f12b44981741d6b41eb9` -> repair `7c15ec12740854261cb9d931e804304c53031477` -> source/test `7b536a62c428c8260e8712ba25714e55e03476a2` routes repository-owned schemas and fixtures through the same strict duplicate-key boundary before AJV. Focused Node seam passed with a deterministic AJV test double; this is not real-AJV whole-suite acceptance.
8. **The Rust projection discarded schema-significant semantics.** Review `5166120254` identified that two candidates with identical topology/evidence could collapse despite differing `procedure_kind`, semantic/tool-contract coordinates or authored edge semantics. RED `564ea965e2dcf89b84af66d646ad2b7c6b9d1889` adds explicit semantic contracts; production repair `502e2719877a5d2945a86faaa7c9f557729280d3` preserves and bounds `procedure_kind`, all eight locale slots, `semantic_refs`, conditional `tool_contract_ref`, and edge `condition`/`guidance`/`pitfalls`, while enforcing the tool-operation contract invariant without resolving foreign authority. Test successor `8029153a86d1bfa959064ce0a6eddd7b928c552f` carries the projection through existing topology fixtures. Successor `be82c216e51669ef48743aad5864ebe278d78ac8` only corrects a stale Rust test-module comment that still described the repaired candidate as topology-only.
9. **The production-language validator assumed strict transport parsing had happened elsewhere.** Review `5169924981` -> RED scaffold `b8f9e68532ba3a286c3e2d1a9ba3817e8e080ca9` -> regression `07efa642ade33af2bcd37aa270c93a2dc5d7d6f8` -> source repair `1100972a1bfbe106d877a40a4fd38732cbe5d969` -> test successor `04bc5c28865c40db2c67c5f8e568ff679f304cb2`. The private std-only recognizer enforces a 2 MiB input ceiling, 128-level object/array depth ceiling, exact JSON number/string/container grammar, valid UTF-16 escape pairing, duplicate member equality after escape decoding, and fixed error codes that do not echo untrusted member text. The tests cover escape-equivalent ASCII and surrogate-pair member names, malformed/trailing JSON, grammar boundaries and exact size/depth limits. This lineage is source-shaped only until native Rust execution is obtained.

The Rust modules remain deliberately unpublished. The new transport seam validates strict JSON grammar and duplicate-member determinism but does not deserialize or map the Draft 2020-12 procedural schema, authenticate expected scope, validate released artifact authenticity/ACL, evaluate semantics, approve a candidate, publish a release or authorize execution. No hosted workflow/check-run materialized for `07efa642...` or `04bc5c288...` in the current observation, and native Rust 1.98 fmt/strict Clippy/tests/rustdoc/release/owned coverage are not established here. Do not promote source-shaped RED/repair lineage into exact-head GREEN.

The next procedural source gap is lossless Rust transport-to-domain mapping of the canonical Draft 2020-12 shape. It must preserve schema-significant absent/present distinctions, reject unknown or schema-invalid fields before construction, and only then compare candidate scope against independently authenticated tenant/task/base/revision context. Released semantic/tool-contract authenticity, ACL/capability resolution, independent evaluation, retained rejected revisions, steward decision binding, immutable/CAS publication, revocation/rollback and released Noema projection remain separate later gates.

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

No release is currently authorized. A release-ready protected head must still execute version/CHANGELOG/tag/package, immutable semantic release, SBOM/provenance/signing, reproducibility and rollback evidence.

## Current execution order

1. Land a versioned backward-compatible central protected CodeQL handler, then ordinary/non-force reconcile #2051/#2056 onto current `.github/main`, reproduce the stable consumer RED and obtain exact-current terminal GREEN plus qualifying independent review.
2. Re-run #35 unchanged exact head against that protected repair; require terminal required checks, zero valid findings and qualifying independent review before normal merge.
3. Restack Foundation #1 onto protected Product bootstrap non-force and repair its Product workflow plus executable CI-contract guard as one parent prerequisite; obtain fresh Foundation exact-head evidence.
4. Restack dependent #5/#9/#43 and descendants normally and reset evidence. #44 preserves all nine repair lineages but does not inherit acceptance from any predecessor head.
5. On #44, first obtain native Rust 1.98 RED/GREEN evidence for the strict transport seam, then progress Draft 2020-12 transport-to-domain mapping and authenticated scope admission; do not repeat the repaired semantic projection or expand JS/local-Git checkers without a new valid finding.
6. Independently progress Source Observation from frozen-v2/versioned-representation/type-authorization RED; do not add transport first.
7. Repair Golden-set #10 against current #9 semantically, then propagate descendants without closure or whole-tree ours/theirs shortcuts.
8. Continue ontology discovery, semantic-layer discovery, alignment/matching, deterministic validation, governance persistence, review/publication, client explain/match/resolve/validate/query-plan contracts, multilingual evaluation, observability/recovery and buyer-path performance only through their canonical owner seams.

Force push, destructive rebase, self-approval, review dismissal, gate weakening, blind/manual rerun, synthetic status, mutable supplier dependency, live Zotero mutation, premature semantic publication and runtime activation are not acceptance mechanisms.
