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
- Published semantic truth is immutable; correction uses supersession/new release.
- Public Rust APIs require beginner-readable documentation.
- Owned production coverage target is 100% line/function/region/branch where tooling exposes it.
- Never suppress deprecation warnings; fix causes.
- Never force-push shared branches, self-approve, fabricate checks, or weaken branch protection.
