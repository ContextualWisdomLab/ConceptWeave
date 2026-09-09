# Product / Technical Gap Baseline

**Snapshot:** 2026-09-09

This file records code-current ConceptWeave product and technical gaps. Exact SHA/run coordinates are evidence snapshots, not mutable dependencies. Live protected branch, PR, issue, review, and workflow state wins when it advances. The immediately preceding #40 ontology-evidence reconciliation is `525bdbd4a8297197b5c0a632730eff8bfa351ff9`; this baseline update is a docs-only successor, so predecessor executable evidence does not transfer to the resulting head.

Historical evidence is preserved rather than overwritten:

- `docs/evidence/historical-product-technical-gap-baseline-pr9-0c935d8.md` retains the complete pre-Foundation-restack Research Intake baseline;
- `docs/evidence/historical-product-technical-gap-baseline-pr40-cc802740.md` retains the complete pre-current-restack pending-source baseline.

These files are dated evidence only. This file is the active gap baseline.

## Canonical ownership

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic release, and the canonical client contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production model/provider routing. Product-domain truth remains with its owner. Consumers use released/versioned `semantic_release`/contract/ACL coordinates; source copies, cross-service SQL, and mutable sibling heads are invalid integration paths.

## Protected / Foundation / CI prerequisites

Protected ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; no immutable semantic release exists.

Foundation #1 is OPEN Draft/mergeable at `60f14a6e85a83d56c2eea43b34d52b3366bb1735`. Product CI still cannot materialize from protected `main` until #35 is normally integrated.

Product-CI bootstrap #35 remains OPEN/non-Draft/mergeable at `22709ec9b4d969bf67ec74db402813e74d11f7ca`: Security Scan `34204381232` and SAST Semgrep `34204381260` succeeded; CodeQL PR `34204381235` is terminal failure attributable to the central CodeQL owner path rather than a ConceptWeave source defect.

Central owner `.github#2051@558693e0333e48012beea142f739bc634b0674a7` is OPEN/non-Draft/mergeable on protected `.github/main@7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`, but acceptance remains blocked by the required CodeQL bootstrap path. Existing owner ancestry preserves changed-base wake binding and removes head-only commit-status authority. The remaining runtime incompatibility is between the successor reader and the protected-default-branch `repository_dispatch` handler: the successor requires `base_ref@base_sha` in dispatch/verdict identity, while protected main still emits the predecessor `head/base_sha/required_run_id` title.

Required CodeQL run `34332431435` has now reproduced that mismatch automatically through attempt **24**, triggered by `github-actions[bot]` without a manual or no-op rerun. Attempt 24 remains associated with #2051, head `558693e...`, and `main@7fd571db...`: Detect job `102469023536` succeeded; actions reader `102469024012` and python reader `102469024822` failed in `Read current-head CodeQL dispatch verdict`; run-level dispatcher `102469807882` then started another protected-default-branch dispatch. This is evidence-admission/liveness RED, not a new CodeQL security finding.

Independent protected-handler evidence is `.github#2061@a04052a86298eb05201449379e8349b32e85df7a` dispatch `34347282529`, which executes from `main@7fd571db...` with predecessor title `...#2061@a04052a.../7fd571db.../34345594932`; #2061 does not modify CodeQL. Earlier #2051 dispatch `34341848212` separately showed successful dispatch validation, CodeQL analysis, and Medium+ SARIF gates while only the legacy wake failed.

The required central repair is transitional and fail-closed: preserve unique `{repository, PR, head, base_ref, base_sha, required_run_id}` as the trust root; admit predecessor-title evidence only when the current required run's unique PR association independently proves that exact identity; distinguish failed SARIF/security evidence from predecessor wake-only liveness failure; reject every base/ref/run/language ambiguity; remove the compatibility path after the corrected handler is protected-main current.

Central #2056 remains OPEN/non-Draft/mergeable at `69ae472562c93cc17674af5e2085a58947d3fab8` on #2051. It preserves the complete-failed-job-set / single atomic wake delta. Existing `CHANGES_REQUESTED` reviews are not dismissed and include earlier-head conflict/behavior findings; current local evidence does not become protected acceptance. Preserve this stacked delta after the #2051 bootstrap repair rather than flattening or discarding it.

Semantic Data Portal #73 and Contextual Orchestrator #1094 remain owner-path dependencies only where their released contracts are actually needed. A mutable/open owner PR, free-gateway recovery proposal, or observed local behavior is not a released ConceptWeave dependency and must not be copied into this repository.

## Research Intake #9

Current Research Intake authority after Foundation restack is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft/mergeable on Foundation `60f14a6...`.

Foundation baseline movement made former #9 head `0c935d805f01cdf548156743c20544ab4596f8c3` non-mergeable because both branches carried valid baseline deltas. Ordinary two-parent merge `18dfe1397b7bc665b3259fa938da1aeebe4c0a86` preserved the Research Intake source/test ancestry and adopted Foundation. The old baseline was archived byte-for-byte before current #9 baseline successor `a67d9d66...` was written.

Owner source repair `9d2c7d612d9b4f38f350720a9e3f2558aa655278` remains material: test-only Local API endpoint selection is not compiled into the measured production artifact; cfg-varying functions are normalized by declaration origin plus normalized identity without collapsing distinct functions; report publication preserves both primary operation failure and exact temporary-cleanup failure without rolling back an already-published final file.

Pre-restack exact `0c935d8...` passed locked Rust 1.98 workspace tests, fmt, strict all-target/all-feature Clippy, warnings-denied rustdoc/release and frozen coverage at native functions 195/195; normalized functions 106/106, regions 1,187/1,187, branches 78/78. That is historical exact-source evidence only. Current `a67d9d66...` is a docs/restack successor and needs fresh exact-current execution, hosted checks, and qualifying independent review.

`ClassificationReport` snapshot/inventory/provenance state remains private and constructor-bound with read-only accessors. This is ordinary aggregate encapsulation, not cryptographic authenticity or peer authentication. Local classification/replay never grants semantic authority, approval, publication or Zotero write permission.

Read-only Local API census on 2026-09-09 observed Zotero `10.0.1`, API v3/schema 44, 8,326 items, 3,715 classifier proposals and 4,611 unclassified items. The private `0600` report SHA-256 is `5b2d81bddf09475de76ce18efa9fb5b486c44b794c9db6b697d183da5c268b48`; it contains 3,658 `needs_steward_review`, 56 `adjacent_evidence`, one `semantic_consumption_bridge`, four pending source keys, and 49 duplicate candidates. These are workload/reconciliation KPIs, not precision, recall, review completion, semantic authority, or Zotero write authority.

The read-only replay was repeated at 18:46 KST and was byte-identical. Title-only ontology discovery produced overlapping workload queues, not semantic coverage or adoption evidence. No private title, item key, or Zotero mutation is published here.

Ontology-library cultivation is bounded by [the Rust candidate matrix](doctoring/ontology_rust_library_candidate_matrix.md). Sophia, Oxigraph, and shacl-rust remain `Proposed`, not dependencies. The 2026-09-09 primary-source reconciliation records Sophia and shacl-rust by exact source revision; records that GitHub marks Oxigraph `v0.5.11` release metadata `immutable=false`, so any evaluation receipt must pin the resolved commit `df37a5c98e2497135cdd4cfce01a049b78ca6740` plus the published source-tarball SHA-256 rather than the tag name alone; and records shacl-rust's upstream statement that `source_constraint` remains unimplemented, making constraint-level diagnostic provenance an explicit fail-closed unsupported feature. A named RDF/OWL/SKOS/SHACL fixture and protected release evidence remain mandatory before adoption.

## Pending-source resolution #40

Current #40 is OPEN Draft/mergeable on exact #9 `a67d9d66...`. Pre-current-restack head `cc802740420dd4402ae2295e270af21530a682dd` became non-mergeable after #9 advanced; ordinary two-parent merge `64bd69e61dedf85a49455dcb0745fe98822a2d20` preserved #40 source-resolution code and adopted #9. Evidence-retention repair `8f5cb2802158b2d1681fc609afcd5dbe019464ae` restored the omitted #9 historical baseline. Subsequent movement through `525bdbd4a8297197b5c0a632730eff8bfa351ff9` is documentation/research only; this baseline commit is another docs-only successor.

The preserved #40 domain delta includes:

- reconstruction/report-binding RED `a3cd9b0d68b4ecc28322860d04a23867412d611f` -> repair `453878f236cd46ae5389575719f37077302da70e`;
- strict stored-wire unknown-field RED `a7b2588baa727ce973323c900e1d00b9c76425b2` -> `2760ebd494ee9d8f25d8768bb57e877ed3a3a678`;
- pending-set integrity RED `13e70410c4285bf037e9b1ca49fd50654f2829dc` -> `9fee299ea1d068a0c9c829865d5fe647d37c0951`;
- trusted-construction RED `cefc2e084f0a746d6a8a40880eef0fdfef417472` -> `f39fc7f874e01ae2fc043d6211ce24fd863930b8`;
- retained-inventory ambiguity RED `e30e519c44270450213f06fcf895dc0dcfc1c25c` -> `1799b2a1815dbfb2c57a8eb1bee992158c3cdb44`, with typed `AmbiguousInventory` pinned at `aa021414843037f598e3585108504e4323bc832e`;
- blank snapshot-identity RED `995837726b448d13c1b88576e2124c5c4809e1a9` -> typed fail-closed repair `7c3280ef7b8223ed685377d3861a7a22a933d2e5`;
- reachable output-path panic RED `bd847fd1b1a85e701eb0a87be4d965d2bb0eb50f` -> fallible `InvalidInput` repair `bdb1b7cce53efb42113685de82aa86d009f54b86`;
- owned branch-scope repair `14d610cf3909eebb075637409068ef76b6665aed`, native function gate repair `675230ae72325c78de2bcb9b1ad572be0f0dc1ab`, and nonempty normalized-denominator repair `124c44eb26411a7fdd58158746ed3504a459d690`.

Before the new #9 restack, exact `cc802740...` had frozen Rust 1.98 evidence: locked tests passed; native LLVM functions 224/224; normalized owned-production functions 129/129, regions 1,371/1,371, branches 102/102. Remaining raw LLVM deficits were diagnostic source-embedded test/generic-instantiation instrumentation; no unexecuted production symbol was identified. Merge and later docs/evidence successors invalidate transfer of that execution. The current docs successor therefore requires fresh Rust 1.98 workspace, fmt, strict Clippy, warnings-denied rustdoc/release, native function 100%, normalized function/region/branch 100%, hosted required checks and qualifying independent review. GitHub exposes no pull-request workflow run on `525bdbd...`; its sole commit status is CodeRabbit `Review skipped: draft pull request`, which is not acceptance evidence.

Stored restoration remains untrusted JSON -> private `StoredSourceResolutionWire` -> strict wire validation -> equality against the supplied `ClassificationReport` -> `prepare_source_resolution_review` -> constructor-bound `SourceResolutionReview`. Admission rejects blank snapshot identity, absent/blank server identity, duplicate/unknown/missing decisions, blank reasons, missing/ambiguous retained inventory, stale source coordinates, and duplicate/noncanonical/blank pending keys. A restored review is steward-workflow evidence only; it is not semantic publication or approval authority.

## Full-text/write terminal #39

#39 remains independently advancing downstream on #38 and is not a mutable dependency of Foundation. Fresh PR metadata during this baseline update reports exact head `aca2fe603477453fee071679a8aefef0cd784dd3`, OPEN Draft/mergeable on #38 `7678236ed3ec467e93b97bb2ad7ad26b3dc0e5b9`.

Its earlier broad raw-region RED was re-attributed by serialized pinned-nightly source evidence at `e78b616c1946005b48c75585639464b42a1462f8`: native functions 451/451, zero uncovered production-normalized regions out of 4,752, and zero uncovered production-normalized branch outcomes out of 784. Remaining raw LLVM deficits are test/generic instrumentation diagnostics, not an identified production-symbol miss. Later docs/coordination movement does not transfer that execution to `aca2fe...`; exact-current execution, hosted checks, and qualifying independent review remain required. Do not invent source work or weaken coverage merely to alter raw diagnostics.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner and released-contract consumer boundary are defined; foreign product truth remains outside ConceptWeave. |
| Foundation / Product CI | BLOCKED_OWNER | Central `.github#2051` bootstrap compatibility repair -> exact central checks/review -> protected integration -> fresh #35 CodeQL/review -> Foundation Product evidence. |
| Research Intake | RESTACKED_EXECUTION_RESET | #9 is mergeable on current Foundation; source repairs retained, current execution/hosted/review must be re-established. |
| Pending-source resolution | DOCS_SUCCESSOR_EXECUTION_RESET | #40 is mergeable and source-resolution delta is retained; current docs/evidence successors require fresh exact-current execution/hosted/review. |
| Full-text/write | DOWNSTREAM_DRAFT | Production coverage attribution is repaired; current lifecycle evidence remains incomplete. |
| Source Observation | P0_GAP | Concrete bounded read-only PostgreSQL adapter remains incomplete. |
| Ontology / semantic-layer generation | P0_GAP | Candidate-library evidence is bounded, but deterministic discovery, alignment, validation, review and publication engines remain incomplete. |
| LLM integration | OWNER_BOUND | Production model calls must use released `contextual-orchestrator`; outputs remain proposed/inferred until steward publication. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback are not executed on protected head. |

## P0 commercial / semantic gaps

1. Concrete Rust PostgreSQL Source Observation adapter with registry/credential ACL, read-only stable snapshot, exact schema admission, non-resetting operation budgets, cancellation, row/byte/concurrency limits, source-disappearance handling and frozen-fixture conformance.
2. Complete observed PostgreSQL domains/enums/indexes/comments/quoted identifiers/cross-schema collisions as evidence without importing source business truth.
3. Deterministic ontology candidate discovery for terms/concepts/taxonomy/non-taxonomic relations with exact receipts and abstention.
4. Semantic-layer candidate discovery for dimensions/measures/grain/units/relationships/physical mappings and deterministic calculation contracts.
5. Retrieval/pruning/structural alignment first; bounded optional LLM assistance through released `contextual-orchestrator`; OAEI-style evaluation and reproducibility.
6. RDF/OWL/SKOS/SHACL plus semantic-layer validation, bounded reasoning, conflict/duplicate/consistency detection and explicit unsupported-feature failure.
7. PostgreSQL 3NF candidate/evidence/validation/review/release/supersession receipts, idempotency/UPSERT/lock design, transactional outbox and only semantically necessary temporal history.
8. Keyverse identity context and product-owned auth UX for steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
9. Versioned OWL/RDFS/SKOS/SHACL/JSON-LD and explicitly version-bound exports; draft/incubating formats cannot be presented as final standards.
10. Complete language-neutral client release/supersession/provenance/signature/compatibility/diff/resolution contracts while consumers retain physical authorization/execution.
11. KO/EN/JA/ZH/VI/ES/DE/FR semantic labels plus CJK/font/text-expansion verification when UI or published labels become material; ontology labels remain separate from the versioned UI translation ledger.
12. Structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.
13. When buyer-facing web/API paths materialize, measure async+k6/E2E p95 <=20 ms where applicable without sample shrinking or unrealistic warm-cache exclusions; profile query/I/O/runtime/render/GC and move genuine hot paths Rust-first if the budget is missed.

## DDD / governance constraints

Maintain explicit Subdomain/Bounded Context/Context Map/UL/Aggregate/Entity/VO/Domain Service/Repository/Event/Invariant alignment across code/API/DB/tests. External DTOs cross ACLs; no generic domain `utils/helpers/services/common` buckets. Source observations are evidence rather than source-system business truth, and relational constraints are not semantic authority. Published semantic truth is immutable; corrections create a new release plus supersession evidence. Production LLM output remains proposed/inferred. Purpose-bound PII, least privilege, CSAP/SOC2 evidence readiness, deterministic receipts, recovery and auditability are release criteria.

## Current merge/release rule

No Foundation, #35, #9, #40, #39, or downstream semantic publication is authorized. The closest owner prerequisite remains central `.github#2051@558693e...`: its source repairs are present, but the protected-default-branch evidence-schema bootstrap incompatibility continues to reproduce automatically and qualifying exact-current review is incomplete. After a bounded central repair reaches exact GREEN and normal protected central integration, #35 needs fresh exact-head CodeQL/review, then Foundation needs exact-current Product/central evidence. #9/#40/#39 must separately establish their own exact-current executable, hosted, and review evidence after head movement.

No force push, destructive rebase, self-approval, review dismissal, fail-open substitution, provider bypass, no-op trigger, synthetic status, live Zotero mutation, premature semantic publication or release is acceptance evidence.
