# Product / Technical Gap Baseline

**Snapshot:** 2026-09-09

This file records code-current ConceptWeave product and technical gaps. Exact SHA/run coordinates are evidence snapshots, not mutable dependencies. Live protected branch, PR, issue, review, and workflow state wins when it advances. This documentation update follows pending-source restack/evidence-retention head `8f5cb2802158b2d1681fc609afcd5dbe019464ae`; the resulting docs-only commit becomes the new #40 head, so prior exact-head execution does not transfer to it.

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

Central owner `.github#2051@558693e0333e48012beea142f739bc634b0674a7` is OPEN/non-Draft/BLOCKED on protected `.github/main@7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`; the earlier `70e8c1fcf19b2e56578e021e0b4d84a808104b24` remains the P1 repair coordinate below. Two P1 identity defects have committed RED->repair lineages:

1. changed-base wake RED `901af9f024836eadd10c6c98affbee037ffecd58` -> production `66a15d856c251f1db2f91cb3d4a2fa66afd8f48c`, with base-aware fixtures `f9d46984e1ef35341e9535af245da8e6ab9c061e`; the single post-matrix coordinator revalidates exact PR/head/base and exact required-run pull-request association before one bounded failed-job rerun;
2. stale head-only terminal-verdict RED `cb164402518e948e6f88366b3f5187d790fb94b8` -> `70e8c1fcf19b2e56578e021e0b4d84a808104b24`; required verdict admission and pending-language suppression use immutable dispatch title `<repo>#<PR>@<head>/<base>/<required_run_id>` plus exact language-job conclusion, while commit statuses remain observability only.

Current exact central workflows remain queued/in flight and qualifying independent current-head approval is absent. Keep #35 stable until normal protected central integration; do not manufacture leaf evidence with a no-op push or blind/manual rerun. After central integration, #35 still requires fresh authenticated exact-head CodeQL and independent approval.

Central #2056 at `69ae472562c93cc17674af5e2085a58947d3fab8` is a six-commit stacked successor whose merge base is #2051 current `558693e...`; it is OPEN/non-Draft/UNSTABLE with `CHANGES_REQUESTED`. It is repair evidence, not a replacement owner coordinate or protected integration. Keep #2051 as the current dependency until #2056 clears its own checks/review and is normally merged.

## Research Intake #9

Current Research Intake authority after Foundation restack is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft/mergeable on Foundation `60f14a6...`.

Foundation baseline movement made former #9 head `0c935d805f01cdf548156743c20544ab4596f8c3` non-mergeable because both branches carried valid baseline deltas. Ordinary two-parent merge `18dfe1397b7bc665b3259fa938da1aeebe4c0a86` preserved the Research Intake source/test ancestry and adopted Foundation. The old baseline was archived byte-for-byte before current #9 baseline successor `a67d9d66...` was written.

Owner source repair `9d2c7d612d9b4f38f350720a9e3f2558aa655278` remains material: test-only Local API endpoint selection is not compiled into the measured production artifact; cfg-varying functions are normalized by declaration origin plus normalized identity without collapsing distinct functions; report publication preserves both primary operation failure and exact temporary-cleanup failure without rolling back an already-published final file.

Pre-restack exact `0c935d8...` passed locked Rust 1.98 workspace tests, fmt, strict all-target/all-feature Clippy, warnings-denied rustdoc/release and frozen coverage at native functions 195/195; normalized functions 106/106, regions 1,187/1,187, branches 78/78. That is historical exact-source evidence only. Current `a67d9d66...` is a docs successor and needs fresh exact-current execution/hosted/review evidence.

`ClassificationReport` snapshot/inventory/provenance state remains private and constructor-bound with read-only accessors. This is ordinary aggregate encapsulation, not cryptographic authenticity or peer authentication. Local classification/replay never grants semantic authority, approval, publication or Zotero write permission.

Read-only Local API census on 2026-09-09: Zotero `10.0.1`, API v3/schema 44 observed 8,326 items and reconciled 3,715 classifier proposals plus 4,611 unclassified items. The private `0600` report has SHA-256 `5b2d81bddf09475de76ce18efa9fb5b486c44b794c9db6b697d183da5c268b48`; it contains 3,658 `needs_steward_review`, 56 `adjacent_evidence`, one `semantic_consumption_bridge`, four pending source keys, and 49 duplicate candidates. The serialized report deliberately does not expose a snapshot digest, so its private artifact hash identifies this replay only. These are workload and reconciliation KPIs, never precision, recall, review completion, semantic authority, or Zotero write authority.

## Pending-source resolution #40

Pre-current-restack head `cc802740420dd4402ae2295e270af21530a682dd` was OPEN Draft and became non-mergeable after #9 advanced to `a67d9d66...`. This was a valid wrong-base/intervening-delta finding, not a reason to close the dependent PR.

The conflict was repaired without force or destructive rebase. Ordinary two-parent merge `64bd69e61dedf85a49455dcb0745fe98822a2d20` has parents `cc802740420dd4402ae2295e270af21530a682dd` and current #9 `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`. A first tree reconciliation preserved #40's source-resolution code and adopted the current #9 baseline, but comparison exposed that #9's archived historical baseline file had been omitted. That omission was immediately repaired at `8f5cb2802158b2d1681fc609afcd5dbe019464ae`; compare against #9 now reports the base as the merge base with `behind_by=0`, retains the #9 archive, and adds only #40's genuine code/doctoring/test delta plus its own historical baseline archive. GitHub reports #40 mergeable again.

The preserved #40 domain delta includes:

- reconstruction/report-binding RED `a3cd9b0d68b4ecc28322860d04a23867412d611f` -> repair `453878f236cd46ae5389575719f37077302da70e`;
- strict stored-wire unknown-field RED `a7b2588baa727ce973323c900e1d00b9c76425b2` -> `2760ebd494ee9d8f25d8768bb57e877ed3a3a678`;
- pending-set integrity RED `13e70410c4285bf037e9b1ca49fd50654f2829dc` -> `9fee299ea1d068a0c9c829865d5fe647d37c0951`;
- trusted-construction RED `cefc2e084f0a746d6a8a40880eef0fdfef417472` -> `f39fc7f874e01ae2fc043d6211ce24fd863930b8`;
- retained-inventory ambiguity RED `e30e519c44270450213f06fcf895dc0dcfc1c25c` -> `1799b2a1815dbfb2c57a8eb1bee992158c3cdb44`, with typed `AmbiguousInventory` pinned at `aa021414843037f598e3585108504e4323bc832e`;
- blank snapshot-identity RED `995837726b448d13c1b88576e2124c5c4809e1a9` -> typed fail-closed repair `7c3280ef7b8223ed685377d3861a7a22a933d2e5` plus corrected regression/integration/doctoring lineage;
- reachable output-path panic RED `bd847fd1b1a85e701eb0a87be4d965d2bb0eb50f` -> fallible `InvalidInput` repair `bdb1b7cce53efb42113685de82aa86d009f54b86`;
- owned branch-scope repair `14d610cf3909eebb075637409068ef76b6665aed`, native function gate repair `675230ae72325c78de2bcb9b1ad572be0f0dc1ab`, and nonempty normalized-denominator repair `124c44eb26411a7fdd58158746ed3504a459d690`.

Before the new #9 restack, exact `cc802740...` had a frozen Rust 1.98 coverage execution: locked tests passed; native LLVM functions 224/224; normalized owned-production functions 129/129, regions 1,371/1,371, branches 102/102. Remaining raw LLVM line/region/branch deficits were diagnostic source-embedded test/generic-instantiation instrumentation; no unexecuted production symbol was identified. The new merge and docs/evidence successors invalidate transfer of that execution. Current exact #40 therefore requires fresh Rust 1.98 workspace, fmt, strict Clippy, warnings-denied rustdoc/release, native function 100%, normalized function/region/branch 100%, hosted required checks and qualifying independent review.

Stored restoration remains untrusted JSON -> private `StoredSourceResolutionWire` -> strict wire validation -> equality against the supplied `ClassificationReport` -> `prepare_source_resolution_review` -> constructor-bound `SourceResolutionReview`. Admission rejects blank snapshot identity, absent/blank server identity, duplicate/unknown/missing decisions, blank reasons, missing/ambiguous retained inventory, stale source coordinates, and duplicate/noncanonical/blank pending keys. A restored review is steward-workflow evidence only; it is not semantic publication or approval authority.

## Full-text/write terminal #39

#39 remains an independently advancing downstream writer on #38 and is not pinned to a mutable SHA here. Latest observed during this run was `d986861db29b45ef176108317bca910b655d4ada`, OPEN Draft/mergeable.

Its broad raw-region RED at `2c09c0690edc5d3be363c40763c20f0d97adf2bd` was re-attributed by a serialized pinned-nightly run at `e78b616c1946005b48c75585639464b42a1462f8`: native functions 451/451, zero uncovered production-normalized regions out of 4,752, zero uncovered production-normalized branch outcomes out of 784. Remaining raw LLVM deficits are test/generic instrumentation diagnostics, not an identified production-symbol miss. Do not invent source work or weaken coverage to alter raw diagnostics. Exact-current execution/hosted/review evidence remains required after later docs/coordination successors.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | Canonical owner and released-contract consumer boundary are defined; foreign product truth remains outside ConceptWeave. |
| Foundation / Product CI | BLOCKED_OWNER | Central `.github#2051` exact checks/review -> protected integration -> fresh #35 exact CodeQL/review -> Foundation Product evidence. |
| Research Intake | RESTACKED_EXECUTION_RESET | #9 is mergeable on current Foundation; source repairs retained, current execution/hosted/review must be re-established. |
| Pending-source resolution | RESTACKED_EXECUTION_RESET | #40 is mergeable after ordinary #9 integration and archive-retention repair; all source-resolution delta retained, current exact execution/hosted/review reset. |
| Full-text/write | DOWNSTREAM_DRAFT | Production coverage attribution is repaired; current lifecycle evidence remains incomplete. |
| Source Observation | P0_GAP | Concrete bounded read-only PostgreSQL adapter remains incomplete. |
| Ontology / semantic-layer generation | P0_GAP | Deterministic evidence-bound discovery, alignment, validation, review and publication engines remain incomplete. |
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

No Foundation, #35, #9, #40, or downstream semantic publication is authorized. The closest owner prerequisite remains central `.github#2051@558693e...`: source repair is present, but current terminal checks and qualifying independent review are pending. After normal protected central integration, #35 needs fresh exact-head CodeQL/review, then Foundation needs exact-current Product/central evidence. #9/#40 have been normally restacked without force and must now re-establish exact-head execution after movement.

No force push, destructive rebase, self-approval, review dismissal, fail-open substitution, provider bypass, no-op trigger, synthetic status, live Zotero mutation, premature semantic publication or release is acceptance evidence.
