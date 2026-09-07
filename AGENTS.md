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

- Update this section when work establishes a reusable, verified lesson. Keep transient run IDs, counts and incidents in the Gap baseline or doctoring evidence; never record credentials or private research content here.
- Automate publication when the protected release path permits it. Reuse the organization's canonical release workflow through a thin caller; verify its contract before wiring credentials. A configured registry secret is not evidence of package readiness, registry ownership, successful publication or deployment.
- Publish only the verified, protected source revision with an immutable version and artifact provenance. Do not release a draft stack to bypass its missing foundation, checks or independent review. Serialize release/deploy operations without cancelling an in-flight publication, and verify the registry artifact after publication before claiming delivery.
- Query secret names/access metadata only when needed; never retrieve or print values. An empty repository secret listing does not establish whether organization or environment secrets are available. Do not introduce a Python package merely because a PyPI credential exists.
- Perform actual screenshot-based Visual Inspection alongside accessibility inspection for affected user journeys. Record the inspected revision, view and state, distinguish untested states, and keep private library screenshots out of public evidence. Accessibility text alone does not establish layout correctness.
- A Zotero full-text response can report equal indexed/total page counters while containing no text. Check content availability and completeness separately, preserve unresolved sources, and require the bound capture/review path before counting a classification as reviewed.
- Re-query the same CI run after an interrupted or truncated observation. A queued run is neither a failure nor proof of execution; a superseded cancelled run must not be presented as current evidence. Unit fixtures that manufacture a receipt and its expected digest do not prove independent producer authentication or live publication integration.
- If a rate-limit summary conflicts with a failed request, inspect that request's rate-limit resource, remaining count and reset headers. A successful summary or GraphQL query does not prove a REST run lookup is available; retain the same run handle and avoid repeated requests until its reset.
