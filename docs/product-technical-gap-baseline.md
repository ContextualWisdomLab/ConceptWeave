# Product / Technical Gap Baseline

**Snapshot:** 2026-09-05

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot. Because this documentation update creates a Foundation successor, the Foundation SHA below is the exact pre-refresh head; PR metadata must be refreshed to the resulting successor SHA.

## September 6 source inventory checkpoint

The existing #9 owner now retains every nonbibliographic metadata record and derives unresolved ancestry rather than silently discarding standalone sources. [Source-scope doctoring](doctoring/zotero_source_scope.md) binds committed REDs, final source `1e95d6eb979e66ecb7dae4f81f18a6b0a91b7624`, **47 tests / 10 unfiltered suites**, strict checks and the unchanged coverage gate. The earlier inventory executable at `48c3525` genuinely reads 8,326 records into 3,715 unchanged bibliographic proposals plus 4,611 other records, with exactly the four previously audited standalone identities pending. A later shared-reader guard also rejects blank identities; no actual final-guard executable replay is implied.

The earlier source findings are repaired locally, not yet propagated into root #39. Required downstream restoration/identity accounting, pending-source reconciliation, approval binding and full-library completion gates remain open; neither zero pending keys nor successful classification grants semantic or write authority. Current native Visual Inspection was attempted but the Mac is locked, so no new screenshot was verified. Historical source scope, authentic worksheet decisions/independent approvals 0/3,715, plus four unresolved sources remain distinct. This checkpoint does not refresh every historical PR coordinate below or imply protected merge/release.

## Historical protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

The active roots observed immediately before this baseline refresh are:

1. Foundation PR #1 — pre-refresh exact head `5cdd319b9425989e632149b243a3308dd630c0ae`, Draft/open/mergeable. The current Foundation makes Product CI draft-aware while keeping Ready/non-Draft quality requirements intact. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`.
2. Product-CI bootstrap PR #35 — exact head `daa543ce2cc2b2eb6d35a7265abcf2a7466e7381`, open/non-Draft/mergeable. It adds only the pull-request form of Product CI so #1 can later be marked Ready without a no-op commit. Exact-head CodeQL PR, Security Scan and SAST Semgrep remain queued; `Security Scan / Detect changed scope` is pre-runner with no steps and no runner assignment, and no independent submitted review exists yet.
3. Client Consumption PR #5 — exact head `cbb9cda0c93d8b762195423834f1d6a27dbfa613`, Draft/open/mergeable. The current source retains language-neutral semantic-release admission, integrity, compatibility, diff/resolution and supersession validation. Previously valid review findings are source-repaired, but current protected evidence remains independently required.
4. Source Observation PR #6 — exact head `d255f5c08a621024809c7e076989eccf0662a330`, Draft/open/mergeable. PostgreSQL targeted `ON DELETE SET NULL (...)` / `SET DEFAULT (...)` column provenance and registry/ACL-resolved source identity are source-repaired. The next P0 slice is the concrete bounded read-only PostgreSQL adapter.
5. Zotero Research Classification root PR #9 — exact head `cda546672cd95b5f8bed7024f70e4e6b39a134c8`, Draft/open/mergeable. The dependent research/write-back stack remains proposal/review oriented and does not elevate local classifier output to semantic authority.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and the public Draft 2020-12 semantic-candidate schema enforce compatible publication-state/truth-status semantics. Hosted exact-head Product evidence still requires the bootstrap workflow on protected `main`. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | BLOCKED_OWNER | #35's exact-head central runs are still queued before useful execution; `Detect changed scope` has no runner assignment or steps. Queueing blocks this validation lane only and is not a reason to stop Source Observation or other repository-owned work. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

Protected central source is `.github/main@b5efbc2762e472e4a380b0503b1f050f76fbb008` at this snapshot. This is evidence only, not a mutable ConceptWeave dependency.

- The current central source includes queue/admission and changed-scope/review-runtime repairs already integrated through ordinary protected history.
- `.github#1873@41935494aa234eb458f1cc08f006daaa278b9760` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains an exact consumer canary for current runner admission and Dependency Review behavior. Its central workflows are queued, so no protected recovery or dependency-review success is inferred from repository settings alone.

## September 6 source-scope admission checkpoint

PR #12 successor runtime `fc0465e` now uses the shared inventory/audit validator and required v2 scope binding before independent duplicate governance. The existing exact candidate-membership receipt already covers duplicate selection; it is not an unprotected authority gap in this consumer. RED `4656d6b` reproduced three retained-source admission bypasses, now rejected. Current local result: 87 tests/17 suites, independent duplicate review 12/12, pinned coverage 153/153 functions, 1,294/1,294 normalized regions and 220/220 normalized branches. Raw coverage remains 1,649/1,658 lines, 2,516/2,536 regions, 200/220 branches. See the Proposed ADR 0006 amendment for history, compatibility and remaining gates. Later restored-report/worksheet/write integration is still required; current root PR #39 has not adopted these changes.

PR #11 local successor checkpoint `23178a9768aa216692d77918d56357ae1269535c` normally inherits #10 `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4` while preserving prior #11 `1dc032598b41a35d52c09d8690c871e07365d7e3`. The stable isolated baseline passed 60 tests/15 suites; an earlier run contaminated by merge timing is invalid evidence. RED `ebdd852` reproduced forged derived audit and ambiguous provenance totals. Extracted audit computation now counts unique parent/child identities and is recomputed before governance. Final local tests passed 75/15 suites including two doctests; independent integrity review passed 17 tests with no new regression. Strict Clippy passed at unchanged runtime `935e035`. Pinned coverage is still running; no inherited coverage claim or remote PR update is made here.

The pinned coverage run subsequently finished with exit 0: 140/140 functions, 1,101/1,101 normalized owned regions, 182/182 normalized branches. Raw lines 1,502/1,505, regions 2,332/2,343 and branches 177/182 remain below 100%. The coverage script and exclusions are unchanged; the earlier pending sentence records chronology, not current execution state.

Residual duplicate-owner gap: changing duplicate candidates and their matching audit count is not bound by the v2 proposal receipt; derived count consistency does not authenticate duplicate proposals. No duplicate merge/write authority is granted. Fresh native screenshot and accessibility state both showed 3,719 items in the library view, with list rows and attachment icons rendered. The earlier Mac-lock limitation is no longer current. No screenshot or bibliographic identities are committed; this view check does not establish full-library reclassification.

Follow-up runtime `8ccb0d5b3d7705786b6c40c3bcf5a10ff32046d9` adds explicit legacy receipt/empty/orphan/cycle/blank-identity regressions and removes only checks proven redundant after shared admission. Local workspace tests: 71/14 unfiltered suites including two doctests; strict Clippy, warnings-denied rustdoc, release build, format and diff checks passed. Independent read-only review reran 15 integrity tests and found no actionable defect. Unchanged pinned coverage gate passed: functions 136/136, normalized owned regions 1,037/1,037 and branches 174/174. Raw lines 1,448/1,451, regions 2,268/2,279 and branches 169/174 remain below 100%; no raw full-coverage claim is made. Logs: `/tmp/conceptweave-admission-coverage-baseline.log`, `/tmp/conceptweave-admission-coverage-green.log`, `/tmp/conceptweave-admission-boundary-tests.log`.

PR #10 source `f6735b585022aac1c8ceff86c150d9b64fd77ec2` repairs evaluation admission and independently approved scope binding after normal integration of producer `51c7df6d03f072449422fd58ca24b2f9d6026f07`. RED `3f2cf55` failed three new integrity tests; GREEN passed 68 tests/14 unfiltered suites and strict Clippy. See the Proposed [ADR 0006 amendment](adr/0006-zotero-research-intake.md). This local result does not establish hosted GREEN, protection-compliant merge, release, downstream adoption or full coverage.

Remaining work: mandatory adoption by restoration, worksheet, duplicate and write consumers without empty defaults or weakened full-text binding. Genuine reviewed decisions and approvals remain 0/3,715 bibliographic proposals, with four additional standalone sources unresolved. Native visual inspection remains unverified at this checkpoint because the Mac was locked. No UI change or new utility repository was needed.

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local registry/credential resolution; explicit read-only session/transaction; exact schema allowlist; total operation and statement deadlines; cancellation plus row/byte/concurrency budgets; complete immutable snapshot or fail closed; source-disappearance handling; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
5. **LLM Proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

### PR #15 execution-scope and uncertainty repair (2026-09-06)

Baseline `45e9c493` passed 87 tests/19 result suites; normal merge `2887f70`
preserves that executor and parent `eb8eaa4`, passing 109 tests/19 suites.
RED `e8b4c06` failed three receipt-binding tests, repaired by `b91ad9f` retaining
the proposal/source digest across dry-run, applied, preflight and partial failure.

Independent review then found a root causal-completion defect: matching before
metadata could hide a delayed request, while matching after or unrelated newer
metadata could invent applied/rollback status. Existing scenarios were preserved
with corrected expectations in RED `646a10c` (two failures). Runtime `c09d101`
removes those inferences and retains the exact submitted request plus optional
observation. Only earlier directly verified operations retain applied/inverse
status. Test `e169630` compares the complete captured request and missing fields.
Independent static review found no remaining actionable owner defect; it is not
GitHub approval. Original and later executor paths both require propagation.

Final source: 109 passing tests/19 result suites including three doctests, strict
all-target Clippy, warnings-denied rustdoc and unchanged coverage gate passed.
Coverage: 185/185 functions, 1770/1770 source-normalized regions, 320/320 normalized
branches. Raw LLVM is 1998/2050 lines, 2934/3014 regions and 280/320 branches, not
100%. Evidence logs: `/tmp/conceptweave-pr15-causal-final.log`, `-clippy-final.log`,
`-rustdoc.log`, and `-coverage.log` share the `conceptweave-pr15-causal` prefix.

No real library write, paper decision, approval or release occurred. Subsequent
execution/recovery/full-text consumers must inherit unknown-request semantics,
retain exact request and earlier receipt fields, and reject retry/rollback of an
unknown original write even when its inverse list is empty. Protected integration,
descendant adoption, live write/rollback and actual reclassification remain open.

### PR #13 source-scope integration checkpoint (2026-09-06)

Normal merge `5df57a7` preserves write-plan head `b41217b` and source-scope
parent `3d2c252`. Two inherited test inputs lacked the optional tag type added
by write planning; explicitly retaining `None` restores their original manual-tag
semantics. Rust 1.98.0 workspace verification passed 94 tests across 18 result
suites, including two documentation tests. The initial shell selected Rust 1.97.1;
the verified invocation is `cargo +1.98.0 test --workspace`.

Native Zotero visual inspection again showed 3,719 items and attachment icons.
This is display evidence, not approved reclassification or write evidence.
Write-plan source-scope validation and proposal-bound approval remain open;
this local integration is not a hosted-check, protected-merge, or release claim.

### PR #13 write-scope repair checkpoint (2026-09-06)

The preceding integration-only gap is locally repaired by `c348278` with tests
`dcde49e` (two RED failures) and `1fe3d7d` (binding/independent-receipt coverage).
Shared report admission now precedes authority, and the required v2 proposal
binding is retained in the plan. Legacy item/metadata errors retain precedence.
Independent static review found no blocking issue; it is not GitHub approval.

Rust 1.98.0 workspace: 97 passing tests, 18 result suites including two doctests;
strict all-target Clippy, warnings-denied rustdoc and unchanged coverage gate pass.
Coverage: 169/169 functions, 1533/1533 source-normalized regions and 292/292
normalized branches. Raw LLVM remains 1831/1852 lines, 2739/2777 regions and
255/292 branches, not 100%. Logs use `/tmp/conceptweave-pr13-write-scope-` with
`final.log`, `clippy.log`, `rustdoc.log` and `coverage.log` suffixes.

PRD/TRD/ADR 0007 and DDD views retain separate metadata, duplicate-membership,
full-text and execution authority. No real decisions, approvals or Zotero writes
occurred. Protected merge, release, descendant adoption and actual reclassification
remain open; no local evidence is transferred to a remote or later head.

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
