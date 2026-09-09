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
- Treat `ClassificationReport` as a trusted Research Intake aggregate: keep snapshot identity and retained inventory private and constructor-bound, expose only immutable accessors, and test public mutation attempts with compile-fail documentation. Preserve defensive malformed-state tests inside the owner module instead of adding public corruption helpers.
- Published semantic truth is immutable; correction uses supersession/new release.
- Public Rust APIs require beginner-readable documentation.
- Owned production coverage target is 100% line/function/region/branch where tooling exposes it.
- Never suppress deprecation warnings; fix causes.
- Never force-push shared branches, self-approve, fabricate checks, or weaken branch protection.

## Operational lessons

- Do not compile test-only endpoint overrides into `coverage_nightly`: that changes the measured production artifact. Normalize cfg-varying function records by declaration origin plus normalized function identity while the independent native 100% function gate remains authoritative, and freeze both same-function merging and same-origin function separation with a synthetic contract fixture.
- When report publication fails, preserve both the operation error and any temporary-cleanup error. Tests must capture the exact operation-specific temporary path from the removal boundary and verify whether it was removed or retained; prefix scans and destination-only assertions are insufficient.
