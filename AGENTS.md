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
- Pending Zotero ancestry is resolved only through the typed source-resolution aggregate bound to the exact report item key/version/type/parent identity; never infer a parent, rewrite the report, or treat a resolution as write or approval authority.
- When the Local API exposes a server identity, source-resolution decisions must also match that `server_id`; a matching library version alone cannot prove the same Zotero source.
- Stored source-resolution artifacts must validate nested `server_id` and `library_version` against their envelope during deserialization; later report validation cannot make an already accepted mismatched artifact safe.
- Stored source-resolution artifacts must also reject blank `rule_revision`; constructor-valid data cannot be assumed valid after JSON restoration.
- Stored source-resolution artifacts must retain a canonical expected pending-key set and reject any restored decision-key set that differs; truncating JSON must not regain typed completeness.
- Stored source-resolution wire DTOs must deny undeclared fields at every persisted object boundary; silently discarding a newer field can turn an untrusted artifact into a falsely trusted review.
- Source-resolution admission is fail-closed when the report has no `server_id`; optional identity is acceptable for lower-authority classification, not for a typed resolution review.
- Missing or blank Local API `server_id` remains an admission error even when a report has no pending items; an empty resolution set must not yield a typed review without provider identity.
- Trusted source-resolution construction must also reject blank `zotero_version` or blank `rule_revision`; caller-supplied or mutated report metadata cannot manufacture an exact-snapshot review with incomplete provenance.
- Public review aggregates that cross a JSON artifact boundary must use owned fields and have a round-trip test; borrowed `&'static str` metadata is not an artifact contract.
- Derived deserialization must enforce the same non-blank provider identity invariant as the constructor path; JSON omission or `null` must not bypass admission checks.
- Coverage reports must distinguish raw LLVM instantiations from the repository's normalized owner function/region/branch gate; never report raw gaps as green or suppress them to manufacture 100%.
- A pinned coverage run can expose duplicate generic instantiations and newly added owner branches even when the ordinary workspace suite is green; record the exact denominator and add deterministic contract tests before claiming coverage recovery.
- Validate public report pending keys for nonblank content before map construction, and return `InvalidInput` whenever an admitted absolute output path has no final normal file-name component; canonicalizing its parent does not make `..` a valid file name.
- For numeric conversions whose failure is impossible at a supported target width, use a target-width helper: keep checked conversion and fail-closed handling on narrower targets, while avoiding untestable error arms on wider targets. Record the boundary rather than weakening the conversion contract.
- Visual Zotero item counts are presentation evidence only; reconcile them with the Local API snapshot and classify attachments before claiming a complete research population.
- Required shared-workflow checks can fail closed while an authenticated exact-head dispatch is still queued; record the dispatch receipt and queue evidence, then wait for the terminal verdict rather than manually rerunning or fabricating success.
- In a live Gap baseline, retain a historical repair SHA as lineage but label the separately observed current owner head and merge state; a later owner push must not silently turn historical evidence into a false current-status claim.
- The release-mode classification report projects no serialized snapshot digest. Keep its `0600` private artifact and record only its SHA-256 plus aggregate counts; that hash proves replay artifact continuity, never steward approval, semantic authority, or Zotero write permission.
- A Noema free-gateway HTTP 429 is an owner-side review-control-plane failure: retain the exact job/head and wait for a protected noema/contextual-orchestrator failover release; never add a consumer retry, paid fallback, synthetic verdict, or copied routing behavior.
- A healthy local replay and a rendered Zotero view do not authorize deployment; keep protected-main, independent review, immutable release, and live-runtime evidence as separate gates.
- Once a report is atomically published, cleanup failure must preserve the final artifact; never roll back by deleting a successfully published path.
- Post-publication cleanup errors must retain the original I/O kind while adding enough context to distinguish an already-published report from a pre-publication failure.
- Treat `ClassificationReport` as a trusted Research Intake aggregate: keep snapshot identity and retained inventory private and constructor-bound, expose only immutable accessors, and test public mutation attempts with compile-fail documentation. Preserve defensive malformed-state tests inside the owner module instead of adding public corruption helpers.
- Published semantic truth is immutable; correction uses supersession/new release.
- Public Rust APIs require beginner-readable documentation.
- Owned production coverage target is 100% line/function/region/branch where tooling exposes it.
- Never suppress deprecation warnings; fix causes.
- Never force-push shared branches, self-approve, fabricate checks, or weaken branch protection.
- Coverage assertions should inspect an already-returned error with direct kind comparisons; `matches!` guards create test-only branch obligations and can obscure the production coverage deficit.
- Keep test injection seams source-identical to production code. A `cfg(test)` branch or generic callback can create a separately uncovered production instance; prefer one private, non-generic boundary and exercise it through the public wrapper.
- Coverage normalization filters need an executable fixture that proves both sides of the boundary: owned production obligations remain present while inline test-only records are excluded. A computed empty denominator must not pass on prose evidence alone.
- Normalized coverage predicates must reject an empty function, region or branch denominator before checking per-record counts; `all([])` is vacuously true and can otherwise turn a broken mapping into false GREEN.
- After programmatic PR-body edits, inspect the rendered Markdown rather than trusting the stored string; escaped newline literals can join headings and evidence bullets while API fields still look correct.
- Do not compile test-only endpoint overrides into `coverage_nightly`: that changes the measured production artifact. Normalize cfg-varying function records by declaration origin plus normalized function identity while the independent native 100% function gate remains authoritative, and freeze both same-function merging and same-origin function separation with a synthetic contract fixture.
- When report publication fails, preserve both the operation error and any temporary-cleanup error. Tests must capture the exact operation-specific temporary path from the removal boundary and verify whether it was removed or retained; prefix scans and destination-only assertions are insufficient.
- GitHub's Mermaid viewer overlays controls on the diagram and the browser viewport can clip a tall graph. A diagram may render without an error while hiding its terminal node; use a top-to-bottom layout with bounded rank spacing for this baseline and confirm every node in an actual browser screenshot.
