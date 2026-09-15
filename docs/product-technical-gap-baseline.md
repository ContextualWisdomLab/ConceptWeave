# Product / Technical Gap Baseline

**Snapshot:** 2026-09-16

This file is the code-current authority for the active ConceptWeave Source Observation lane. Detailed history through exact `b1de4de974ac2b96495d054060ac48cb1cfe7f52` is preserved losslessly in `docs/archive/product-technical-gap-baseline-through-b1de4de9.md`; focused decisions after that point remain in `docs/doctoring/`. Exact SHAs, review IDs, run IDs, and statuses are evidence coordinates only. Execution or review evidence from an earlier head never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA truth; `contextual-orchestrator` owns production LLM routing. Product-domain truth and Ubiquitous Language remain with their canonical owners. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack and single-writer boundary

- Protected/default ConceptWeave `main` remains repository acceptance authority; the active Source Observation branch is not release authority.
- #46 `codex/pr6-v3-index-evidence` is the active Draft Source Observation writer stacked on #45 exact `6b2a8f555725dc79f60432afbc492d6005290a4a`.
- #45 and #6 must not duplicate, partially cherry-pick, or independently reimplement this Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is terminal GREEN.
- Product bootstrap #35 remains the repository-owned Product `pull_request` prerequisite while protected ConceptWeave `main` lacks that workflow.
- Central reusable workflow ownership remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, locally mutate, or wake that owner lane.

## Current Source Observation authority

All source-repaired relation-partition/index evidence archived through `b1de4de9...` remains retained: rowtype/`atttypmod`, topology/validity/uniqueness/access method, mapped key/`INCLUDE`, operator-family/exclusion semantics, canonical expression/predicate and relation-`Var` equality, exact collation catalog coordinates and database encoding, PostgreSQL-18 provider/field/encoding shape, material/effective database-default definitions, provider-version presence/drift/coherence, copied `ucs_basic`, libc encoding-independent C/POSIX rows, and provider-`d` delegation. No issued digest domain is rewritten by the current repair.

### Database-default libc C-UTF8 encoding binding

Review `5213788571` found that `DatabaseDefaultCollationDefinitionObservation::validate_database_encoding()` treated every libc database-default definition as database-encoding compatible. That admitted source-unreachable governed evidence such as PostgreSQL database encoding LATIN1 with libc `datcollate` or `datctype` equal to `C.UTF-8`/`C.utf8`.

PostgreSQL 18 `CreateDatabase()` executes `check_encoding_locale_matches(encoding, dbcollate, dbctype)`. `pg_get_encoding_from_locale()` treats `C`/`POSIX` as SQL_ASCII-compatible and otherwise obtains the actual codeset from the host locale implementation. An available C-UTF8 locale resolves to UTF8, so LATIN1 is rejected. PostgreSQL separately retains the explicit superuser SQL_ASCII exception; that source-reachable catalog state must not be rejected.

- Source RED: `9e9379359406787d0e9cf7b964e7800f63055496`, `database_default_libc_c_utf8_database_encoding_contract.rs`.
- Initial minimal production repair: `336c42aaadeb729a65f2343bf3501826c9478343`.
- Source-review correction retaining PostgreSQL's SQL_ASCII exception: contract `68d4a6064822999ee4b1544c965476431fb8bd1f`, production `d690f3f94f71cae8a6e137693383ef6271d0614f`.
- Focused doctoring currentization: `fd6ade25d4ee3a5270d23f65b2d34c90d3039c76`, `docs/doctoring/postgresql-database-default-libc-c-utf8-encoding-integrity.md`.
- CHANGELOG currentization: `d135772a3dd471e8c90edc370a5b4ae58e780d52`.
- PostgreSQL authority: `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`.

Current invariant: for provider `c`, if either raw `datcollate` or `datctype` is case-insensitive `C.UTF-8`/`C.utf8`, the bounded PostgreSQL database encoding must be UTF8 (`6`) or the explicit source-reachable SQL_ASCII (`0`) path. LATIN1 is rejected. `C` and `POSIX` remain encoding-independent controls. Arbitrary libc locale names are not assigned a codeset by string parsing; PostgreSQL obtains that truth from the runtime/OS, so the remaining general libc compatibility proof belongs to concrete transport/live differential evidence.

## Current state

**INDEX_DATABASE_DEFAULT_LIBC_C_UTF8_ENCODING_BINDING_SOURCE_REPAIRED / POSTGRESQL_DATABASE_DEFAULT_LIBC_GENERAL_ENCODING_DIFFERENTIAL_OPEN / POSTGRESQL_COLLATION_DEFINITION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_DEFAULT_COLLATION_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_EXPRESSION_EXTRACTOR_DIFFERENTIAL_OPEN / POSTGRESQL_ATTTYPMOD_ADAPTER_DIFFERENTIAL_OPEN / POSTGRESQL_DATABASE_ENCODING_ADAPTER_DIFFERENTIAL_OPEN / ACCEPTANCE_PENDING**.

The complete archived Source Observation state through `b1de4de9...` remains authoritative unless explicitly superseded by a focused decision record. This concise baseline is not a deletion of earlier contracts or evidence; the archive is the retained history and this file is the current decision surface.

## Acceptance boundary

No executed Rust RED/GREEN or hosted Product acceptance is claimed after these ordinary-forward head moves. One unchanged exact #46 representation head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new libc C-UTF8 database-encoding contract plus every retained Source Observation focused contract, workspace/doc tests, release build, owned production rustdoc/test/edge-case coverage, and applicable hosted Product/security/dependency/review gates. Any head movement resets exact-head acceptance.

Protected ConceptWeave `main` still requires repository-owned Product PR workflow convergence through #35. Central workflow-owner work remains in `ContextualWisdomLab/.github`; ConceptWeave must not copy, wake, or locally weaken that owner contract.

## Next causal work

1. Converge the canonical central workflow owner and obtain compatible fresh unchanged-head acceptance for Product bootstrap #35; land #35 normally on protected/default ConceptWeave `main` only when required gates are terminal GREEN.
2. Obtain one unchanged #46 representation head with repository-pinned Rust 1.98 native GREEN plus applicable hosted Product/security/dependency/review terminal GREEN.
3. Only after that representation gate, extend #46 ordinary-forward with the concrete PostgreSQL 18 extractor/live differential. For the new seam, prove UTF8 + libc C-UTF8 acceptance, LATIN1 + libc C-UTF8 rejection/non-creation, the authorized SQL_ASCII + C-UTF8 path, and C/POSIX cross-encoding controls. For arbitrary libc locale names, observe runtime codeset compatibility instead of inferring it from spelling. All retained built-in/ICU/provider-`d`, version, copied-`ucs_basic`, expression, and `atttypmod` differentials remain required.
4. Only after the complete #46 child is terminal GREEN may its full delta flow ordinary/non-force into #45, followed by fresh #45 acceptance and #6 propagation. Semantic publication, version/tag/package/SBOM/provenance/reproducibility/rollback, and immutable release remain later gates.

No force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflow, manual/no-op rerun, gate weakening, partial parent adoption, predecessor-evidence transfer, or premature publication/release is authorized.
