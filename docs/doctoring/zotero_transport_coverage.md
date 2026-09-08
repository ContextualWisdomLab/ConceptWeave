# Zotero Local API transport coverage

Status: **Source/test repaired — exact-head execution pending**  
Owner lane: Research Intake PR #9  
Pre-RED source: `943d7495b89c330df109414b547bb097d24ec6ee`  
Reality RED: `2e0be0902d48034cf130efb1b1bab27705a7f06b`  
Minimal source repair: `c47947579c947f3426d89601eee93c80ca9ce018`  
Coverage-branch test successor: `b9db5557fecae9b4998dc006b3ebf297f5e6da77`

## Problem

ConceptWeave's owned-production coverage contract requires the Local API admission and evidence path to remain executable under the coverage gate. The production `read_local_snapshot()` reaches `fetch_local_page()` through the same real loopback adapter used by `metadata_transport` regressions. Before the repair, four production helpers were nevertheless hidden from nightly coverage with `#[cfg_attr(coverage_nightly, coverage(off))]`:

- `fetch_local_page`;
- `header_u64`;
- `header_string`;
- `optional_header`.

Those helpers do more than an unavoidable raw socket syscall. They build the version-pinned Local API request, parse required and optional contract headers, read the bounded body, deserialize provider JSON and perform provider-shaped Zotero object-key admission before a page enters the immutable snapshot reader. Excluding the whole seam allowed the owned 100% gate to ignore provider-contract parsing regressions.

## RED and causal repair

Commit `2e0be0902d48034cf130efb1b1bab27705a7f06b` added `production_transport_helpers_remain_in_owned_coverage` to `crates/conceptweave-zotero/src/tests/metadata_transport.rs`. The regression fails while any of the four named helpers is immediately annotated `coverage(off)`.

Commit `c47947579c947f3426d89601eee93c80ca9ce018` is the minimal production repair. Its one-file diff removes exactly those four coverage exclusions and changes no Local API request, response, classification, snapshot or governance behavior.

Commit `b9db5557fecae9b4998dc006b3ebf297f5e6da77` follows with executable branch pressure on the real production adapter. It retains the existing proxy-isolation, API-version, body-bound and canonical-object-key cases and adds:

- missing and malformed required numeric header rejection;
- optional `Zotero-Server-ID` absence on a successful response;
- malformed JSON rejection through `read_local_snapshot()`;
- direct present/missing/malformed/opaque header-helper boundaries, including a non-text header value that `optional_header` must reject.

The source-contract RED therefore has a causal source repair and newly exposed branch tests in ordinary ancestry. This is not yet an executable GREEN claim because the Rust/LLVM coverage gate has not run on the unchanged successor.

## Preserved boundary

The repair preserves the loopback-only Local API endpoint, `Zotero-API-Version: 3`, environment-proxy denial, bounded response/body handling, exact provider key validation, snapshot/version/schema/server consistency, whole-run resource budgets and fail-closed errors. It does not add Zotero mutation, semantic classification authority, provider fallback, a second transport owner or a coverage threshold exception.

## Acceptance

This finding remains open until one unchanged successor head demonstrates:

1. the policy RED passes because the four production helpers are no longer excluded;
2. loopback transport regressions pass for success and exposed error branches;
3. locked Rust 1.98 workspace tests and formatting;
4. all-target Clippy and warnings-denied rustdoc/release;
5. owned production function, normalized-region and branch coverage at 100%;
6. applicable hosted exact-head checks.

If coverage reports a remaining transport branch, repair the missing deterministic regression rather than reintroducing `coverage(off)`. No predecessor coverage result, Draft review skip, deterministic classification result or zero pending-source count transfers to this acceptance.

## Traceability

| Evidence | Exact binding |
| --- | --- |
| Owned-production coverage contract | `AGENTS.md` on PR #9 current ancestry: 100% line/function/region/branch where tooling exposes it |
| Pre-RED transport source | `crates/conceptweave-zotero/src/lib.rs` at `943d7495b89c330df109414b547bb097d24ec6ee` |
| Reality RED | `crates/conceptweave-zotero/src/tests/metadata_transport.rs` at `2e0be0902d48034cf130efb1b1bab27705a7f06b` |
| Minimal production repair | `crates/conceptweave-zotero/src/lib.rs` at `c47947579c947f3426d89601eee93c80ca9ce018` |
| Exposed-branch regressions | `crates/conceptweave-zotero/src/tests/metadata_transport.rs` at `b9db5557fecae9b4998dc006b3ebf297f5e6da77` |
| Review owner | PR #9 thread `PRRT_kwDOUKg5E86fSz80`, reopened on 2026-09-08 |