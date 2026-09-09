# AGENTS.md — ConceptWeave

Read the organization `ContextualWisdomLab/.github` master context and product goal directive before material work. Live GitHub state and this repository's accepted ADRs override remembered chat state.

## Product boundary

ConceptWeave owns automatic, evidence-bound **Semantic Model Engineering**. Do not turn it into a semantic catalog, lineage engine, generic LLM gateway, browser crawler, or another product's system of record.

## Development rules

- Apply DDD continuously; maintain the Context Map and Ubiquitous Language.
- Rust 1.98.0 is the production baseline for core logic. Production mathematical/vector/model-scoring computation, if introduced, remains Rust-first.
- `conceptweave-domain` has no provider/network/database dependencies.
- External products and providers enter through versioned ports and Anti-Corruption Layers.
- LLM work uses `contextual-orchestrator`; model output is proposal evidence, never semantic authority.
- No direct cross-service application-table SQL.
- New database objects, when introduced, use descriptive two-or-more-word `snake_case` names and 3NF by default.
- Preserve source evidence, truth status, and publication state separately.
- Keep Zotero full-text captures separate from metadata reports and approval receipts; restored captures require bounded verification, and local HTTP continuity is not peer authentication.
- Full-text review views are read-only evidence; never strip their outer binding into a metadata-only patch and claim full-text decision or approval provenance.
- Full-text decisions use the separate blank-start worksheet and atomic exact-view application; reverify the capture/report relation through finalization and whole-envelope governance. No reviewed-set downcast grants Zotero write authority.
- Offline full-text commands reuse the private-file boundary and pass completed-view bytes unchanged into atomic validation. Finalized files await external approval verification; no CLI command issues approval or writes Zotero.
- Full-text writes require a complete typed review, explicit destinations and mode; finish both local validation paths before real authority verification. Keep scope bindings through opaque execution/recovery receipts. Unknown original writes cannot become empty successful rollbacks, and serialized audit files are not executable authority.
- Persisted source-resolution wire DTOs must reject undeclared fields at every object boundary; silently discarding newer fields can turn an untrusted artifact into a falsely trusted review.
- Source-resolution review coordinates are constructor-bound and exposed through read-only accessors; callers must not mutate aggregate fields after admission.
- Delayed original-write observations retain the exact submitted request and complete earlier receipt; matching metadata does not prove causal completion or authorize retry/rollback.
- Published semantic truth is immutable; correction uses supersession/new release.
- Public Rust APIs require beginner-readable documentation.
- Owned production coverage target is 100% line/function/region/branch where tooling exposes it.
- Never suppress deprecation warnings; fix causes.
- Never force-push shared branches, self-approve, fabricate checks, or weaken branch protection.

## Operational lessons to maintain

- The classification report intentionally projects only classifier metadata, so it cannot diagnose attachment `contentType`, `linkMode`, path or URL state. When acquisition triage needs those fields, query the exact report-bound attachment set read-only and emit aggregate counts only; never infer attachment availability from missing projected fields.

- Use the capture-bound `assess_full_text_availability` API or `--full-text-availability` command for paper-level acquisition denominators. Verify the whole report/capture first, resolve bounded ancestor chains, and keep response counts, paper counts, unresolved records and review status distinct; do not replace this contract with ad hoc JSON queries.

- Use the pinned optimized build for full-library capture on this host; preserve the same byte/time limits and report binding. A debug capture exceeded its budget while the optimized run completed. Do not infer which budget failed from legacy shared error text, or infer unchanged evidence from equal item counts/snapshot identifiers alone.

- Full-library report output must be a new direct child of the system temp directory. A nested temporary directory is intentionally rejected by the private-file boundary; choose a unique direct-child filename without weakening canonical-parent or `0600` checks.

- A sampled hashing hotspot does not prove buffering improves end-to-end performance. Keep realistic escaped-text inputs unchanged, record terminal results, and discard an optimization without demonstrated benefit. Shared-host elapsed times are observations, not controlled causal benchmarks.

- Validate cited commits against the named repository before counting source evidence. A syntactically valid SHA or a successful lookup in another repository does not establish the citation; reject a repository-scoped 404/422 and quarantine dependent audit claims until rebound and reverified.

- Count repository audit coverage by canonical repository identity, not audit sections or visits. Search the existing inventory before adding a candidate; repeated inspection of the same owner or revision updates evidence without increasing the unique denominator. Withdraw an unsupported KPI until the complete identity reconciliation is verified.
- A table-row count is only a reconciliation lower bound: prose-only audits, aliases and repeated revisions require URL-level repository joins before any unique-coverage denominator is published.

- Update this section when work establishes a reusable, verified lesson. Keep transient run IDs, counts and incidents in the Gap baseline or doctoring evidence; never record credentials or private research content here.
- Automate publication when the protected release path permits it. Reuse the organization's canonical release workflow through a thin caller; verify its contract before wiring credentials. A configured registry secret is not evidence of package readiness, registry ownership, successful publication or deployment.
- Publish only the verified, protected source revision with an immutable version and artifact provenance. Do not release a draft stack to bypass its missing foundation, checks or independent review. Serialize release/deploy operations without cancelling an in-flight publication, and verify the registry artifact after publication before claiming delivery.
- Failed-publication tests must assert the operation-specific temporary path is absent after cleanup; checking only the returned error and destination state can leave sensitive serialized evidence behind while still appearing green.
- Query secret names/access metadata only when needed; never retrieve or print values. An empty repository secret listing does not establish whether organization or environment secrets are available. Do not introduce a Python package merely because a PyPI credential exists.
- Perform actual screenshot-based Visual Inspection alongside accessibility inspection for affected user journeys. Record the inspected revision, view and state, distinguish untested states, and keep private library screenshots out of public evidence. Accessibility text alone does not establish layout correctness.
- A full-text write scope must carry a complete source-resolution aggregate and re-run the owner validator immediately before authority callbacks. Matching report metadata alone is insufficient; missing or stale pending-source decisions must fail closed without issuing Zotero write authority.
- Keep that aggregate intact from the blank worksheet through view application, finalization, evaluation, and write admission; a pending report must not be unconditionally rejected when an exact report-bound resolution is available.
- The release-mode classification CLI can read a live Local API snapshot without mutation; keep its `0600` report in `/tmp`, publish only aggregate counts/hash, and preserve pending source keys in every downstream review.
- The release report stores `classified_items` and `unclassified_items` as arrays, not scalar counts. Compute public aggregates from their lengths after checking the JSON shape; never let a failed ad hoc aggregation alter the private report or be presented as a classification result.
- The bounded release-mode full-text capture can retain private response evidence while exposing only status/count/digest aggregates; never treat a successful capture or HTTP 200 response as reviewed meaning, approval, atomicity or write authority.
- `--full-text-worksheet` and `--bound-full-text-review` can prepare blank, capture-bound evidence offline; preserve their separate `0600` artifacts and never downcast the view into a metadata decision patch.
- Full-text worksheets are not metadata `StewardReviewWorksheet` inputs: `--review-batch` and `--review-progress` must reject them. Use `--bound-full-text-review` for capture-bound batches, and keep the metadata progress contract on its own worksheet type.
- Title-keyword aggregates are useful discovery queues only; they must never overwrite deterministic dispositions, clear abstentions or stand in for steward review and ontology authority.
- Discovery KPI matching must declare case semantics per signal: case-insensitive `ontology` is distinct from case-sensitive `OWL`/`RDF`; otherwise `OWL` can overcount ordinary words such as `knowledge`.
- Keep classifier matched-phrase counts separate from title-only discovery counts; overlapping signals are evidence queues, not additive coverage or steward decisions.
- A Zotero full-text response can report equal indexed/total page counters while containing no text. Check content availability and completeness separately, preserve unresolved sources, and require the bound capture/review path before counting a classification as reviewed.
- Validate the report's pending-source key sequence itself before building a trusted resolution aggregate: reject duplicates and noncanonical ordering even when the supplied decisions appear complete.
- To exercise report-bound canonicalization, use at least two valid pending sources and reverse the stored resolution order; a single-entry fixture cannot cover the mismatch branch.
- Re-query the same CI run after an interrupted or truncated observation. A queued run is neither a failure nor proof of execution; a superseded cancelled run must not be presented as current evidence. Unit fixtures that manufacture a receipt and its expected digest do not prove independent producer authentication or live publication integration.
- If a rate-limit summary conflicts with a failed request, inspect that request's rate-limit resource, remaining count and reset headers. A successful summary or GraphQL query does not prove a REST run lookup is available; retain the same run handle and avoid repeated requests until its reset.
- The shell may export `RUSTUP_TOOLCHAIN=stable`, overriding `rust-toolchain.toml` and causing a Rust 1.97/1.98 mismatch. For ConceptWeave verification, invoke the pinned toolchain explicitly (`rustup run 1.98.0 cargo ...`) and retain the environment override as evidence; do not weaken the repository pin.
- A GitHub ruleset can require central workflows even when `gh pr checks --required` reports no required checks. Treat the ruleset and its exact workflow verdicts as authoritative; a `MERGEABLE` or clean leaf check set does not prove protected readiness.
- Central model-review HTTP 429s and missing exact-head OpenCode/CodeQL receipts are control-plane failures. Keep the product head stable, record the failed job and exact SHA, and wait for the canonical owner repair instead of replaying stale runs or manufacturing approval.
- For host-dependent path discovery, keep production admission policy deterministic and unit-test the canonicalization helper with both an existing and a guaranteed-missing path. Do not make coverage depend on whether `/tmp` happens to resolve on the runner.
- Once an absolute output path has a canonicalized parent, the root-only path has already failed the parent check; encode that invariant directly instead of carrying an unreachable file-name error arm into production coverage.
- For numeric conversions whose failure is impossible at a supported target width, use a target-width helper: keep checked conversion and fail-closed handling on narrower targets, while avoiding untestable error arms on wider targets. Record the boundary rather than weakening the conversion contract.
- A documentation-only push still changes the PR head and invalidates prior hosted evidence. Re-query the exact head, ruleset, review and checks after pushing; local coverage can be reused only as local evidence, with raw per-file gaps kept separate from normalized aggregate gates.
- External ontology-library scans are candidate evidence only: cite the official upstream revision, verify release/license/provenance and conformance in a bounded matrix, and keep the candidate behind a versioned ACL until ConceptWeave authority and consumer contracts are proven.
- When a stacked PR is merged externally, re-query both PR `headRefName` and `baseRefName` before pushing a successor; a branch name may now represent the merged successor, and only an exact fast-forward/non-force update preserves the intended delta.
- Automated review comments are evidence tied to their reviewed commit, not evergreen instructions; re-read the current exact head before fixing or closing a finding, and record stale-finding corrections without claiming approval.
- Pending-source admission must reject ambiguous retained inventory rather than silently taking the first match; preserve the typed failure and regression before any trusted review aggregate is built.
- A green workspace test run does not satisfy the owned coverage gate; retain raw and normalized per-file/branch deficits as RED evidence and repair tests or documented platform seams without lowering thresholds.
### Reusable coverage lesson

When a coverage gate exposes test-only branches, remove unreachable platform guards or branchy assertion wrappers only when the behavioral assertion remains equivalent. Re-run both the full locked suite and the frozen coverage script; normalized branch recovery does not satisfy the raw repository gate when source functions, lines, or regions remain uncovered.

- Compare raw coverage gaps with the frozen repository gate before editing. Direct `cargo llvm-cov` can report intentionally uncalled rejection-test callbacks, coverage-off entrypoints or fixture-only unreachable arms; classify those lines first and never hide them by weakening the owned threshold.
- Concurrent frozen coverage runs share `coverage.json` and normalized intermediates in the worktree. For a decisive rerun, use a fresh `CARGO_TARGET_DIR`, run only one coverage command, and retain raw line/region/branch deficits as RED even when native functions and normalized predicates pass.
- Raw LLVM file totals include source-embedded test and generic-instantiation instrumentation. Preserve them as diagnostics, but locate production gaps with the frozen function-symbol normalization: an empty normalized uncovered set is evidence that the reported raw deficit is not an unexecuted production region; it does not by itself make a PR protected-ready.

- Upstream repository license metadata and Cargo package license declarations are separate evidence. Record both at immutable head/release coordinates and resolve any divergence before proposing a dependency or utility repository.
- Discovery KPIs must name their denominator: the 142/3,715 ontology queue rate is steward workload screening only, never precision, recall, approval coverage or semantic authority.
- A byte-identical Local API replay proves deterministic snapshot/report continuity only; it never substitutes for steward decisions, publication authority or a Zotero write receipt.
- Discovery queues must be checked against the bound replay for non-blank, unique, sorted keys and exact digests before steward use; queue integrity still grants no semantic or Zotero write authority.
- A merged upstream owner pull request is source-integration evidence only. Require an immutable release coordinate or artifact plus current consumer adoption and re-execution before transferring checks or marking a protected consumer prerequisite repaired.
- A historical GitHub check can remain visible after its workflow definition is removed, while `gh run view` returns workflow-level 404. Preserve the exact run/job URL and SHA, inspect check-run metadata and the rendered PR Checks page, and do not reinterpret the unavailable log as a passing or rerunnable workflow.
- OIDC issuer, audience, JWKS coordinates, timeout, role mapping, and demo-header policy belong in the catalog owner's versioned configuration boundary, not request-path environment reads. A local owner test seam may override that boundary; ConceptWeave still requires a released versioned catalog contract before adoption.
- Verified OIDC roles must reach every catalog policy action, not just initial create/search branches. A regenerated dependency lockfile repairs declared-package drift but is local owner evidence until its successor completes protected checks.
