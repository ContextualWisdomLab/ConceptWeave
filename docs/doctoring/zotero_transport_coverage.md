# Zotero Local API transport coverage

Status: **Reality RED — source repair pending**  
Owner lane: Research Intake PR #9  
Pre-RED source: `943d7495b89c330df109414b547bb097d24ec6ee`  
Reality RED: `2e0be0902d48034cf130efb1b1bab27705a7f06b`

## Problem

ConceptWeave's owned-production coverage contract requires the Local API admission and evidence path to remain executable under the coverage gate. The production `read_local_snapshot()` already reaches `fetch_local_page()` through the real loopback adapter used by `metadata_transport` regressions, but four helpers are still hidden from nightly coverage with `#[cfg_attr(coverage_nightly, coverage(off))]`:

- `fetch_local_page`;
- `header_u64`;
- `header_string`;
- `optional_header`.

Those helpers do more than an unavoidable raw socket syscall. They build the version-pinned Local API request, parse required/optional contract headers, read the bounded body, deserialize provider JSON and perform provider-shaped Zotero object-key admission before a page enters the immutable snapshot reader. Excluding the whole seam permits the owned 100% gate to stay green while provider-contract parsing regresses.

The existing loopback suite already reaches this production adapter for proxy isolation, API-version request headers, exact/oversized body limits, malformed UTF-8/truncation, and canonical Zotero object-key admission. The correct boundary is therefore narrower than these four helpers.

## Reality RED

Commit `2e0be0902d48034cf130efb1b1bab27705a7f06b` adds `production_transport_helpers_remain_in_owned_coverage` to `crates/conceptweave-zotero/src/tests/metadata_transport.rs`.

The regression reads the production source with `include_str!("../lib.rs")` and fails while any of the four named helpers is still immediately annotated `coverage(off)`. This is a policy RED, not a substitute for execution coverage: removing the annotations alone is insufficient unless the resulting exact head also reaches 100% owned function/normalized-region/branch coverage through executable tests.

The previously resolved production-reader coverage review was reopened because current source still violates the accepted coverage boundary.

## Minimal repair

Remove `coverage(off)` only from `fetch_local_page`, `header_u64`, `header_string`, and `optional_header`. Preserve all existing Local API behavior: loopback-only ACL, `Zotero-API-Version: 3`, no environment proxy, bounded response/body handling, exact provider key validation, snapshot/version/schema/server consistency, resource budgets and fail-closed errors.

After exposing the helpers, add or retain explicit loopback cases until every newly visible function/normalized-region/branch is exercised. In particular, required-header absence/malformed numeric values, optional server ID, HTTP failure, JSON failure, object-key failure and successful page admission must not rely on a coverage exclusion.

Do not broaden this repair into semantic classification, Zotero mutation, provider fallback or a second transport owner. A genuinely unavoidable raw process or host-filesystem discovery shim may remain narrowly excluded only when its deterministic policy is extracted and covered separately.

## Acceptance

This finding remains open until one unchanged successor head demonstrates:

1. the policy RED passes because the four production helpers are no longer excluded;
2. loopback transport regressions cover success and exposed error branches;
3. locked Rust 1.98 workspace tests and formatting;
4. all-target Clippy and warnings-denied rustdoc/release;
5. owned production function, normalized-region and branch coverage at 100%;
6. applicable hosted exact-head checks.

No predecessor coverage result, Draft review skip, deterministic classification result or zero pending-source count transfers to this acceptance.

## Traceability

| Evidence | Exact binding |
| --- | --- |
| Owned-production coverage contract | `AGENTS.md` on PR #9 current ancestry: 100% line/function/region/branch where tooling exposes it |
| Pre-RED transport source | `crates/conceptweave-zotero/src/lib.rs` at `943d7495b89c330df109414b547bb097d24ec6ee` |
| Existing real-adapter regressions | `crates/conceptweave-zotero/src/tests/metadata_transport.rs` at `943d7495b89c330df109414b547bb097d24ec6ee` |
| Reality RED | same test module at `2e0be0902d48034cf130efb1b1bab27705a7f06b` |
| Review owner | PR #9 thread `PRRT_kwDOUKg5E86fSz80`, reopened on 2026-09-08 |
