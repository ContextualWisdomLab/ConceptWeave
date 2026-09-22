# Product / Technical Gap Baseline

**Snapshot:** 2026-09-09

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes a recorded coordinate when it advances. This refresh was authored on Foundation pre-refresh head `b538470c963e6524ddc0c3f652a46a4fc8265150`; the resulting commit is the new Foundation exact head and resets exact-head execution evidence for that PR.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic release, and the canonical client contract for consuming those releases. It does not duplicate foreign domain truth.

- `semantic-data-portal`: catalog, governance, discovery and consumption surfaces.
- `context-graph-contracts`: interop contract owner.
- `enterprise-architecture-core`: enterprise-architecture truth owner.
- `contextual-orchestrator`: production LLM routing/provider/capability owner.
- consuming products: tenant/purpose authorization and physical execution remain local to the consumer.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and active prerequisites

Protected/default ConceptWeave `main` is `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`. It is the repository bootstrap only; there is no immutable ConceptWeave semantic release yet.

### Foundation PR #1

Pre-refresh exact head: `b538470c963e6524ddc0c3f652a46a4fc8265150`, OPEN Draft/mergeable. This documentation refresh creates a successor, so predecessor checks/reviews do not transfer. Product CI still cannot execute from protected `main` until bootstrap #35 is normally integrated.

### Product-CI bootstrap PR #35

Exact head `22709ec9b4d969bf67ec74db402813e74d11f7ca`, OPEN non-Draft/mergeable on protected `main`.

- Security Scan `34204381232`: success.
- SAST Semgrep `34204381260`: success.
- CodeQL PR `34204381235`: terminal failure.

The ConceptWeave CodeQL dispatch completed, but compatibility admission ended pending an authenticated terminal verdict. Current evidence attributes this to the central CodeQL owner path, not to a ConceptWeave source defect. Do not manufacture a fresh result by no-op push or blind/manual rerun while the corrected central owner remains unmerged.

### Central CodeQL owner `.github#2051`

Current source authority: `.github#2051@70e8c1fcf19b2e56578e021e0b4d84a808104b24`, OPEN non-Draft/mergeable on protected `.github/main@7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`.

The original sibling-rerun race is replaced by one post-matrix `wake-required-codeql` coordinator. Two distinct identity defects now have committed RED->repair lineages:

1. **Wake/base identity.** Changed-base RED `901af9f024836eadd10c6c98affbee037ffecd58`; base-aware fixtures `f9d46984e1ef35341e9535af245da8e6ab9c061e`; production repair `66a15d856c251f1db2f91cb3d4a2fa66afd8f48c`. The coordinator revalidates live PR/head/base and exact required-run pull-request metadata before one bounded failed-job rerun.
2. **Terminal-verdict identity.** RED `cb164402518e948e6f88366b3f5187d790fb94b8`; production repair `70e8c1fcf19b2e56578e021e0b4d84a808104b24`. Required verdict admission and pending-language suppression no longer trust head-only `codeql-dispatch/<language>` commit statuses. They bind to immutable dispatch title `CodeQL Scan Dispatch <repo>#<PR>@<head>/<base>/<required_run_id>` plus the exact `CodeQL dispatch scan (<language>)` job conclusion. Commit statuses are observability only.

Current-head source inspection confirms the second repair preserves the first repair and the single coordinator. Acceptance is still incomplete: CodeQL PR `34330189823`, Python Security `34330189894`, Agent Review Runtime Quality CI `34330189849`, Security Scan `34330190193`, and SAST Semgrep `34330190098` remain queued/in flight, and formal reviews observed before `70e8c1f...` are historical. Normal protected central integration requires terminal exact-head checks, zero valid unresolved findings, and qualifying independent review.

After central integration, #35 must obtain a fresh authenticated exact-head CodeQL result and independent approval before normal merge. Only then can Foundation obtain Product evidence from protected `main`.

## Current semantic-engineering roots

### Research Intake PR #9

Exact head `0c935d805f01cdf548156743c20544ab4596f8c3`, OPEN Draft/mergeable on Foundation. Owner repair `9d2c7d612d9b4f38f350720a9e3f2558aa655278` keeps test-only Local API endpoint selection out of measured production, normalizes cfg-varying function identity, and preserves both primary report-publication failure and temporary-cleanup failure without rolling back a published final file.

Local exact-head evidence recorded for `0c935d8...`: locked Rust 1.98 workspace, fmt, strict all-target/all-feature Clippy, warnings-denied rustdoc/release; frozen coverage native functions 195/195, normalized owned functions 106/106, regions 1,187/1,187, branches 78/78. Hosted exact-head checks and qualifying independent review are absent, so #9 remains Draft and no semantic authority is promoted.

`ClassificationReport` provenance/snapshot/inventory state is private and constructor-bound with read-only accessors. This is an owner-controlled Rust aggregate boundary; it is not cryptographic authenticity or peer authentication.

### Pending-source resolution PR #40

Exact head `cc802740420dd4402ae2295e270af21530a682dd`, OPEN Draft/mergeable on #9 exact `0c935d8...`. Normal non-force merge `c47f8239cd4191b6948d8fb7adb69c048d559ee7` adopted the material #9 owner delta while retaining #40 report binding, strict stored-wire validation, private trusted construction, canonical pending-set admission, retained-inventory uniqueness, snapshot identity, and source-resolution authority boundaries.

A frozen Rust 1.98 coverage run has executed on current exact head `cc802740...`: locked tests passed; native LLVM functions 224/224; production-normalized functions 129/129, regions 1,371/1,371, branches 102/102. Raw LLVM line/region/branch totals include source-embedded test and generic-instantiation instrumentation and remain diagnostic. No unexecuted production symbol was identified. This advances only the current local coverage lane; exact-current fmt/strict Clippy/rustdoc/release, hosted required checks, and qualifying independent approval are still required. No Zotero write, semantic publication, or approval authority is added.

### Full-text/write terminal PR #39

#39 is an independently advancing downstream writer on #38; Foundation does not pin its mutable head. The latest observed coordinate during this refresh is `d986861db29b45ef176108317bca910b655d4ada`, OPEN Draft/mergeable.

The broad raw-region RED recorded at `2c09c0690edc5d3be363c40763c20f0d97adf2bd` was followed by a serialized pinned-nightly attribution run at `e78b616c1946005b48c75585639464b42a1462f8`: native functions 451/451, zero uncovered production-normalized regions out of 4,752, and zero uncovered production-normalized branch outcomes out of 784. Raw LLVM lines/regions/branches remain below total because of source-embedded test and generic-instantiation instrumentation; no production-symbol miss was identified. Do not invent production code or weaken a gate merely to change raw diagnostics. Subsequent observed commits through `d986861...` are documentation/coordination successors, so exact-current execution, hosted checks, and independent review must still be refreshed before lifecycle actions.

The live Local API replay remains acquisition/observation evidence only: 8,326 observed records, 3,715 bibliographic proposals, 4,611 retained non-bibliographic records, four pending sources, and zero classification failures were recorded; full-text availability reported 3,203 papers with nonempty text, 440 without attachment ancestry, 34 unmanifested attachments, 38 captured-empty, 471 needing review without text, and two unbound nonempty records. These counts do not create steward decisions, approval, semantic authority, publication, or Zotero write permission.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership and owner/consumer boundaries; foreign truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | Publication-state/truth-status semantics and immutable/supersession direction are defined; no protected semantic release exists. |
| Source Observation | ACTIVE_CHILD | Immutable source evidence and bounded observation contracts exist; concrete production PostgreSQL adapter remains a P0 gap. |
| Research Intake / source resolution | LOCAL_GREEN_HOSTED_PENDING | #9 and #40 have current local source/coverage evidence within their stated scope; hosted checks and independent current-head review remain missing. |
| Product CI | BLOCKED_OWNER | #35 leaf CodeQL failure waits on accepted central `.github#2051@70e8c1f...` and then a fresh #35 exact-head run. |
| Quality gate | ACTIVE_PR | Rust 1.98, unsafe forbidden, public docs, exact checkout, fmt, strict Clippy, tests, rustdoc, owned production coverage, schema fixtures, lock freshness, and clean-tree checks. Head movement resets exact-head evidence. |
| Security / review | PENDING_EXACT_HEAD | Scanner/reviewer status contexts are not substitutes for authenticated terminal checks and qualifying formal review. |
| Standards / research | ACTIVE | Doctoring/TRACEABILITY must bind authoritative standards/primary research to exact implementation/module/API/test evidence and identify contradictions. |
| UI / multilingual | NOT_YET_MATERIALIZED | When material UI appears, require reusable composition, token/Figma identity, normal/loading/empty/error/permission/responsive/interaction/a11y E2E, and KO/EN/JA/ZH/VI/ES/DE/FR label/font/text-expansion verification. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback must be executed on the exact protected release head. |

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local registry/credential resolution; explicit read-only session/transaction; exact schema admission; total operation/statement deadlines; cancellation plus row/byte/concurrency budgets; complete immutable snapshot or fail closed; source-disappearance handling; deterministic replay against a frozen anonymized fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; relational structure alone is not business authority.
5. **LLM proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox, temporal history only where domain semantics require it, explicit idempotency/UPSERT/lock design.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound exports; draft/incubating formats cannot be represented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation and explain/query-plan contracts while downstream products retain physical authorization/execution.
12. **CWL integration** — only released/versioned semantic-release/contract/ACL seams to sibling owners; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.
15. **Buyer-path performance** — when a buyer-facing web/API path is materialized, measure async+k6/E2E p95 <= 20 ms where applicable without sample shrinking or unrealistic cache warm-up; profile query/I/O/runtime/render/GC and move genuine hot paths Rust-first when the budget is missed.

## DDD and governance fitness constraints

- Maintain explicit Subdomain/Bounded Context/Context Map/UL/Aggregate/Entity/VO/Domain Service/Repository/Event/Invariant alignment across code, API, DB and tests.
- No generic domain `utils/helpers/services/common` buckets.
- External DTOs cross Anti-Corruption Layers; adapters stay outside the core domain model.
- Source Observation facts are evidence, not source-system business truth. Relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- Published semantic truth is immutable; correction creates a new release and supersession evidence instead of in-place overwrite.
- Production LLM output remains proposed/inferred until steward validation/publication. Provider/model/key discovery belongs to `contextual-orchestrator` released APIs and schemas.
- Purpose-bound PII, least privilege, CSAP/SOC2 evidence readiness, deterministic receipts, recovery and auditability are release criteria rather than post-release documentation work.

## Current merge/release rule

No Foundation, #35, #9, #40, or downstream semantic publication is authorized by this snapshot. The closest prerequisite is central `.github#2051@70e8c1f...`: source repair is present, but exact-head required workflows and qualifying independent review are still pending. After its normal protected integration, #35 requires a fresh authenticated exact-head CodeQL run and independent approval; Foundation then requires its own exact-current Product and central evidence. Local predecessor evidence never transfers merely because a successor is docs-only.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, provider bypass, no-op trigger, synthetic status, live Zotero mutation, premature semantic publication, or release is acceptance evidence.