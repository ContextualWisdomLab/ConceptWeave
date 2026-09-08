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

- A sampled hashing hotspot does not prove buffering improves end-to-end performance. Keep realistic escaped-text inputs unchanged, record terminal results, and discard an optimization without demonstrated benefit. Shared-host elapsed times are observations, not controlled causal benchmarks.

- Validate cited commits against the named repository before counting source evidence. A syntactically valid SHA or a successful lookup in another repository does not establish the citation; reject a repository-scoped 404/422 and quarantine dependent audit claims until rebound and reverified.

- Count repository audit coverage by canonical repository identity, not audit sections or visits. Search the existing inventory before adding a candidate; repeated inspection of the same owner or revision updates evidence without increasing the unique denominator. Withdraw an unsupported KPI until the complete identity reconciliation is verified.

- Update this section when work establishes a reusable, verified lesson. Keep transient run IDs, counts and incidents in the Gap baseline or doctoring evidence; never record credentials or private research content here.
- Automate publication when the protected release path permits it. Reuse the organization's canonical release workflow through a thin caller; verify its contract before wiring credentials. A configured registry secret is not evidence of package readiness, registry ownership, successful publication or deployment.
- Publish only the verified, protected source revision with an immutable version and artifact provenance. Do not release a draft stack to bypass its missing foundation, checks or independent review. Serialize release/deploy operations without cancelling an in-flight publication, and verify the registry artifact after publication before claiming delivery.
- Query secret names/access metadata only when needed; never retrieve or print values. An empty repository secret listing does not establish whether organization or environment secrets are available. Do not introduce a Python package merely because a PyPI credential exists.
- Perform actual screenshot-based Visual Inspection alongside accessibility inspection for affected user journeys. Record the inspected revision, view and state, distinguish untested states, and keep private library screenshots out of public evidence. Accessibility text alone does not establish layout correctness.
- A full-text write scope must carry a complete source-resolution aggregate and re-run the owner validator immediately before authority callbacks. Matching report metadata alone is insufficient; missing or stale pending-source decisions must fail closed without issuing Zotero write authority.
- The release-mode classification CLI can read a live Local API snapshot without mutation; keep its `0600` report in `/tmp`, publish only aggregate counts/hash, and preserve pending source keys in every downstream review.
- The bounded release-mode full-text capture can retain private response evidence while exposing only status/count/digest aggregates; never treat a successful capture or HTTP 200 response as reviewed meaning, approval, atomicity or write authority.
- `--full-text-worksheet` and `--bound-full-text-review` can prepare blank, capture-bound evidence offline; preserve their separate `0600` artifacts and never downcast the view into a metadata decision patch.
- Title-keyword aggregates are useful discovery queues only; they must never overwrite deterministic dispositions, clear abstentions or stand in for steward review and ontology authority.
- A Zotero full-text response can report equal indexed/total page counters while containing no text. Check content availability and completeness separately, preserve unresolved sources, and require the bound capture/review path before counting a classification as reviewed.
- Re-query the same CI run after an interrupted or truncated observation. A queued run is neither a failure nor proof of execution; a superseded cancelled run must not be presented as current evidence. Unit fixtures that manufacture a receipt and its expected digest do not prove independent producer authentication or live publication integration.
- If a rate-limit summary conflicts with a failed request, inspect that request's rate-limit resource, remaining count and reset headers. A successful summary or GraphQL query does not prove a REST run lookup is available; retain the same run handle and avoid repeated requests until its reset.
- The shell may export `RUSTUP_TOOLCHAIN=stable`, overriding `rust-toolchain.toml` and causing a Rust 1.97/1.98 mismatch. For ConceptWeave verification, invoke the pinned toolchain explicitly (`rustup run 1.98.0 cargo ...`) and retain the environment override as evidence; do not weaken the repository pin.
- A GitHub ruleset can require central workflows even when `gh pr checks --required` reports no required checks. Treat the ruleset and its exact workflow verdicts as authoritative; a `MERGEABLE` or clean leaf check set does not prove protected readiness.
- Central model-review HTTP 429s and missing exact-head OpenCode/CodeQL receipts are control-plane failures. Keep the product head stable, record the failed job and exact SHA, and wait for the canonical owner repair instead of replaying stale runs or manufacturing approval.
- For host-dependent path discovery, keep production admission policy deterministic and unit-test the canonicalization helper with both an existing and a guaranteed-missing path. Do not make coverage depend on whether `/tmp` happens to resolve on the runner.
