# Product / Technical Gap Baseline

**Current snapshot:** 2026-09-11

This is the code-current ConceptWeave gap baseline for this branch. Exact SHA, run, review and PR coordinates are evidence snapshots, not mutable dependencies. Live protected branches and owner PRs supersede recorded coordinates when they advance. Earlier snapshots remain historical evidence and do not become current acceptance by inheritance. GitHub's computed `mergeable` flag is mechanical state, never acceptance evidence.

The procedural self-application lane now has source-shaped strict raw-byte JSON admission, lossless Draft 2020-12 candidate mapping, semantic/topology validation, and a first canonical revision-envelope/context-binding seam. Review `5172140328` identified that bare-candidate admission could not bind `proposal_id` or `base_model_ref` because the repository-owned `procedural-revision-proposal` envelope never entered Rust. Source-level RED contract `569ad8f87b08f5be5f92e36d8f9b5657f38a45b3` requires revision-envelope mapping and distinct expected proposal/base/scope coordinates. Production repair `fb95147d53b3f93b9fa7c1d571263eba5a6633ae` maps the canonical envelope, preserves `decision_authority: none`, and rejects proposal/base/candidate-scope mismatch before deterministic validation. Test successor `bcb40b39e9a0d69340ad4c6f4ee09cbbfef89f8d` strengthens mismatched proposal/base/tenant and invalid authority/state/partition/origin/shape witnesses. None of these commits has native Rust 1.98 or hosted Product acceptance; CodeRabbit success alone is not native evidence.

Review `5172576660` then found that PRD/TRD/ARCHITECTURE/ADR-PG still described the procedural lane as schema-only or wholly planned Rust work. Documentation repairs `c6305efcd1872a59eaa6fa73a1b2977088edc887` -> `7e7cda6f20894dd59b2b7af75c761384eeb90b31` -> `2aae1f27021a826e7cc84757e256a687542eb114` -> `a2058142bfa90ddf86b6f11f19fdab9541c5d740` now distinguish source-shaped private Rust implementation from native/hosted acceptance and make the Keyverse-authentication/ConceptWeave-resource-authorization split explicit. This is authority-document repair, not Rust Product GREEN.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic release, and the canonical client contract for consuming those releases. It does not copy foreign domain truth.

- `semantic-data-portal`: catalog, governance and consumption surfaces.
- `context-graph-contracts`: released interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/model/provider routing and capability ownership.
- Noema: execution-local procedural projection, lifecycle and runtime-authorization integration.
- Keyverse: identity/credential/authentication trust authority; ConceptWeave retains proposal/base resource authorization.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and current prerequisites

ConceptWeave protected `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; it is still repository bootstrap only and has no protected immutable ConceptWeave semantic release.

Central protected `.github/main` remains `cb0872c9a20d5584703dffacca65c096fc034c6c`, merged from `.github#1938` on 2026-09-10. Its protected required contexts include CodeQL language detection and compatibility analysis, queue/security/dependency checks, Noema/review/bootstrap/coverage and OpenCode gates.

### Product-CI bootstrap #35

Exact head `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN non-Draft and mechanically mergeable on protected ConceptWeave `main`.

The branch repairs Draft suppression in Product validation and replaces registry-resolved `npx --yes ajv-cli` execution with lockfile-pinned `ajv@8.20.0` plus repository-owned in-process validation. Exact-head central evidence remains mixed: Security Scan `34434790791` SUCCESS, SAST Semgrep `34434790777` SUCCESS, CodeQL PR `34434790860` FAILURE. The CodeQL run detected languages successfully, failed compatibility while waiting for authenticated current-head settlement, and only later completed the dispatch job. This remains a stable downstream reproduction of the central settlement/rollout defect; no no-op push, manual rerun or Ready/Draft toggle is corrective evidence.

### Central CodeQL owner stack

`.github#2040@6706c231ab06a3c91c43fdb5b989cfcd79fff593` remains historical-base preservation input rather than current protected acceptance evidence.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` is OPEN Draft and mechanically mergeable, but still based on historical `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`. It preserves one post-matrix wake coordinator, exact PR/head/base/run identity and stricter `{base_ref, base_sha}` binding. Its own body records the deterministic bootstrap defect: `repository_dispatch` executes the protected default-branch handler, so a combined client+handler protocol change cannot self-bootstrap when protected main does not yet emit the new run identity.

`.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains OPEN Draft, mechanically mergeable and stacked on #2051. It preserves complete-failed-job-set validation plus one atomic run-wide wake; fixture-backed GREEN does not substitute for protected-default runtime evidence.

The owner repair order remains: land a versioned backward-compatible protected handler while preserving the currently protected client; ordinary/non-force reconcile #2051/#2056 onto current `.github/main`; switch the client only after the compatible handler exists; obtain one fresh unchanged-head terminal generation; remove any purpose-complete legacy handler only after protected migration and in-flight drain. Old-base hosted evidence does not transfer after ancestry repair.

## Current ConceptWeave roots

- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft. Product bootstrap #35 is its direct prerequisite. Foundation Product definition and `scripts/check_ci_contract.py` must be repaired together after #35 integrates.
- Client Consumption #5: `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft on Foundation. It owns immutable semantic-release admission/diff/compatibility/supersession, not foreign business truth or physical execution.
- Source Observation #6: `6c1157efad5d2e4258330d6485ff2ea980ae331b`, OPEN Draft on #5. Its next source slice is PostgreSQL representation/versioning/authorization before transport.
- Research Intake #9: `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft on Foundation. `ClassificationReport` remains private and constructor-bound.
- Pending-source #40: `4efe15c6318d8cb65c52a974a2c105363a4c82a5`, OPEN Draft on #9.
- Golden-set #10: `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4`, OPEN Draft from historical #9 ancestry; preserve valid deltas and repair by ordinary semantic reconciliation rather than closure.
- Procedural authoring #43: `2c6d3037acdeae2b152c335ee2f60a59b4472831`, OPEN Draft on Foundation. It owns local draft/revision schemas, locked AJV fixture validation and Draft-safe Product/CI-contract source repair.
- Procedural self-application #44: source/test successor `bcb40b39e9a0d69340ad4c6f4ee09cbbfef89f8d`; authority-document successor `a2058142bfa90ddf86b6f11f19fdab9541c5d740`; OPEN Draft on #43. It adds inferred self-application profiles, strict checked-in artifact/provenance admission, unpublished Rust semantic/topology projection, strict raw-byte JSON admission, canonical Draft mapping and private revision-envelope/context binding. None promotes parsed coordinates to authentication, approval, publication or runtime authority.

## Source Observation P0 — PostgreSQL representation before transport

Current snapshot framing remains `conceptweave.postgres_schema_snapshot.v2`; current evidence is table/column/constraint oriented and `ColumnObservation` still carries human-readable `data_type` rather than exact qualified type identity. Do not silently redefine v2.

Before transport, the owner lane still needs a frozen historical-v2 witness and explicit v3/equivalent representation for relation kind/comment, first-class indexes and schema-scoped domain/enum/type coordinates; exact authorized schema allowlist checks; identity-stable input ordering; digest material for enum ordering and material domain semantics; and rejection of fake table-scoped type coordinates. Only after this representation/version/authorization slice is exact-head Rust GREEN should concrete PostgreSQL transport add least-privilege key+binding resolution, stale-binding rejection before source I/O, one non-resetting budget, explicit `REPEATABLE READ READ ONLY`, bounded rows/bytes/concurrency, cancellation cleanup and complete-or-fail snapshot construction.

## Procedural semantic engineering — self-application state

The checked-in profiles remain inferred Draft artifacts, KO/EN authored labels only, with runtime activation disabled. Shape, provenance, transport recognition, schema mapping, semantic projection, revision binding or topology validation never grants execution or publication authority.

Fourteen defects/authority gaps now have explicit finding/RED -> repair lineages on #44:

1. **Self-consistency was not immutable provenance.** `86c74f66098ae83df935b11ad250f3a02ff8a732` -> `a963a1f24902347f7ec8aca9fd9bee8837986ce7` resolves exact local-Git `commit:path`, recorded blob identity/type, bounded source bytes and SHA-256.
2. **Object existence was not authored-history membership.** `a7e6b821262671c0f72974bfc86ff8b6fd935227` -> `e22647e6f45f7244552c0e2d74f86dcc8ac0e85c` requires source commit ancestry from exact checked-out HEAD and scrubs ambient `GIT_*` overrides.
3. **Ordinary JSON parsing collapsed duplicate object members.** Review `5163384790` -> `898cd641e60426541b567d02ba090b20249c88b2` adds bounded duplicate-decoded-key admission for the self-application manifest/profiles.
4. **A Git blob was not necessarily a regular source file.** Review `5163577072` -> RED `3fd1873bcbb29d74308131bde98a8503561d72d6` -> repair `aab84a9d72686fc4de8c43ba4d9e1548c3cad9da` -> tests `484db0083c9604cd41112c4a35ae9c31f3d667cc` binds tree mode/object/path and rejects symlinks.
5. **Duplicate rejection could amplify attacker content into logs.** Review `5164233960` -> RED `4fac2eb62f5484c20bce44c5abfc57b93592a596` -> `10799a65325319c6ff337814b23d9d7a48d82fcd` emits only bounded duplicate fingerprints/length.
6. **Local Git integrity was named as generic source authentication.** Review `5164920106` -> `c476ce244a6e6774a8b1d12c7b38613deaada2db` -> `abb8d7f98973d6ba77799b9bbc69d82353f48454` narrows the result to local-Git provenance; manifest authentication remains not established.
7. **Repository validator definitions themselves admitted ambiguous JSON.** Review `5165560999` -> `49bfbc90d985c3cfaee8f12b44981741d6b41eb9` -> `7c15ec12740854261cb9d931e804304c53031477` -> `7b536a62c428c8260e8712ba25714e55e03476a2` routes schemas/fixtures through strict duplicate-key admission before AJV.
8. **Rust projection discarded schema-significant semantics.** Review `5166120254` -> RED `564ea965e2dcf89b84af66d646ad2b7c6b9d1889` -> production `502e2719877a5d2945a86faaa7c9f557729280d3` -> test successor `8029153a86d1bfa959064ce0a6eddd7b928c552f` preserves procedure kind, eight locale slots, semantic/tool references and edge condition/guidance/pitfalls without resolving foreign authority.
9. **Production Rust assumed strict JSON had happened elsewhere.** Review `5169924981` -> scaffold/regression `b8f9e68532ba3a286c3e2d1a9ba3817e8e080ca9` / `07efa642ade33af2bcd37aa270c93a2dc5d7d6f8` -> `1100972a1bfbe106d877a40a4fd38732cbe5d969` -> tests `04bc5c28865c40db2c67c5f8e568ff679f304cb2` adds the private std-only strict recognizer.
10. **An `&str` entry could not own the wire boundary.** Review `5170467082` -> RED `a6c10bfd9219736c325c8195059c2e978e8f491a` -> `d4a89af78153137f74eed1e6a6a2b64580b4d4a1` -> tests `ce6b064abeb1db2184d7ecf6ea2d4756d500bbb6` bounds raw bytes before UTF-8 and rejects malformed UTF-8 without lossy conversion.
11. **`semantic_refs` absence collapsed with invalid present-empty input.** Review `5171000125` -> `f9534b3a98341ff57505f0fc49529caf3b9f26ff` -> `7c607d8691fbe045dc419d21b2aff4777bad7043` -> `d48e2731b390ae26bcba484cdafd087dc1d13b92` preserves `None` versus present nonempty arrays.
12. **Strict transport was not canonical Draft schema admission.** Review `5171563060` -> behavioral RED `d8e93f752801967c6708bfbea69253db459e811e` -> source/test `e779caeab6bd0e3a1bf26ef834e3a3b2a3677306` maps the bounded canonical Draft before semantic/topology validation and rejects unknown/missing/wrong-typed or schema-invalid members.
13. **Bare-candidate admission could not bind revision/base identity.** Review `5172140328` -> source-level RED `569ad8f87b08f5be5f92e36d8f9b5657f38a45b3` -> production repair `fb95147d53b3f93b9fa7c1d571263eba5a6633ae` -> test successor `bcb40b39e9a0d69340ad4c6f4ee09cbbfef89f8d`. The private revision mapper admits the repository-owned envelope, preserves `proposal_state: proposed`, `decision_authority: none`, `evidence_partition: training`, validates bounded training/rejected references and rationale, and compares proposal ID, exact base artifact coordinates and candidate model/tenant/task/domain-owner scope with a distinct `ProceduralRevisionExpectation`. Mismatch fails with a fixed non-echoing `revision_context_mismatch` diagnostic before deterministic candidate validation. The expectation is explicitly **not** an authentication receipt.
14. **Authoritative design docs lagged the implemented private source boundary.** Review `5172576660` found PRD schema-only wording, TRD's obsolete AJV-CLI/future-Rust description, ADR-PG's wholly-planned Rust section and architecture's stale Keyverse/input-schema framing. `c6305efc...` -> `7e7cda6f...` -> `2aae1f27...` -> `a2058142...` reconciles those documents with the exact source-shaped Rust seams while preserving the absence of native/hosted acceptance and the identity-vs-resource-authorization owner split.

The Rust modules remain deliberately unpublished. There is no exact-current native Rust 1.98 fmt/strict Clippy/tests/rustdoc/release/owned coverage or hosted Product acceptance for the newer lineages. The current runtime cannot establish compiler evidence and the Product workflow is absent from protected ConceptWeave main, so source-shaped RED/repair must not be promoted to exact-head GREEN.

The next procedural owner gap is the **authenticated application request adapter**, but it cannot be implemented against mutable Keyverse source. A production adapter must consume an immutable released Keyverse trust contract to obtain RP identity/subject/tenant evidence, apply ConceptWeave-owned authorization for the exact proposal/base resource, construct the private expectation without copying authority from the untrusted revision payload, and fail closed on stale/replayed/mismatched context before any governed transition. The domain crate remains provider/network/database free. Released semantic/tool-contract authenticity, ACL/capability resolution, independent evaluation, steward decision binding, immutable/CAS publication, revocation/rollback and released Noema projection remain separate later gates.

## Quality, security and release invariants

- Owned production doc/rustdoc, tests and edge-case coverage remain 100% requirements; coverage denominators are not weakened to manufacture GREEN.
- Math/psychometrics/data/performance/security hot paths remain Rust-first. Synthetic data is unit-test evidence only.
- Provider/model routing stays behind released contextual-orchestrator APIs; proposed/inferred model output never becomes authoritative before steward validation/publication.
- Published semantic truth is immutable; correction produces a new release plus supersession evidence.
- External DTOs cross explicit ACLs; product/domain truth stays with its canonical owner. No generic cross-domain source copy or cross-service SQL.
- Purpose-bound PII, least privilege, structured audit evidence, recovery and reproducibility are release criteria.
- Material buyer-facing web/API paths require realistic async+k6/E2E p95 <= 20 ms where applicable; bottlenecks are profiled rather than hidden by sample reduction or unrealistic warm-up.
- Material UI requires product-specific reusable composition plus normal/loading/empty/error/permission/responsive/interaction/a11y and KO/EN/JA/ZH/VI/ES/DE/FR layout evidence before completion claims.

No release is currently authorized. A release-ready protected head must still execute version/CHANGELOG/tag/package, immutable semantic release, SBOM/provenance/signing, reproducibility and rollback evidence.

## Current execution order

1. Land a versioned backward-compatible central protected CodeQL handler, then ordinary/non-force reconcile #2051/#2056 onto current `.github/main`, reproduce the stable consumer RED and obtain exact-current terminal GREEN plus qualifying independent review.
2. Re-run #35 unchanged exact head against that protected repair; require terminal required checks, zero valid unresolved findings and qualifying independent review before normal merge.
3. Restack Foundation #1 onto the protected Product bootstrap non-force and repair Product workflow plus executable CI-contract guard as one prerequisite; obtain fresh exact-head evidence.
4. Restack dependent #5/#9/#43 and descendants normally and reset evidence. #44 preserves all fourteen repair/authority lineages but inherits no acceptance from predecessor heads.
5. On #44, obtain native Rust 1.98 evidence for transport, semantic projection, Draft mapping and revision binding before widening/exporting them. Then consume an immutable released Keyverse trust artifact through an application-layer versioned port/ACL; do not place credentials/providers in `conceptweave-domain`.
6. After Keyverse identity admission, apply ConceptWeave-owned exact proposal/base resource authorization, reject stale/replayed/mismatched context, resolve only released semantic/tool artifact coordinates through canonical owner contracts/ACLs, then progress independent evaluation, steward decision and immutable publication.
7. Independently progress Source Observation from frozen-v2/versioned-representation/type-authorization RED; do not add transport first.
8. Repair Golden-set #10 against current #9 semantically, then propagate descendants without closure or whole-tree ours/theirs shortcuts.
9. Continue ontology discovery, semantic-layer discovery, alignment/matching, deterministic validation, governance persistence, review/publication, client explain/match/resolve/validate/query-plan contracts, multilingual evaluation, observability/recovery and buyer-path performance only through their canonical owner seams.

Force push, destructive rebase, self-approval, review dismissal, gate weakening, blind/manual rerun, synthetic status, mutable supplier dependency, premature semantic publication and runtime activation are not acceptance mechanisms.
