# Source Observation: UNIQUE null-comparison evidence

Date: 2026-09-06. Scope: [PR #6](https://github.com/ContextualWisdomLab/ConceptWeave/pull/6), [review finding](https://github.com/ContextualWisdomLab/ConceptWeave/pull/6#pullrequestreview-5124531466), Proposed ADR 0004 and its single-use-capability refinement ADR 0006. This report records local contract verification, not a protected merge, live database observation or semantic approval.

## Problem and source evidence

A data architect comparing two revisions of the same unique constraint must be able to detect a change in which rows it admits. PostgreSQL 18 documents that `pg_index.indnullsnotdistinct=false` treats nulls as distinct, whereas `true` treats them as equal for uniqueness. Its constraint catalog identifies the supporting index with `conindid` (PostgreSQL Global Development Group, 2026a, 2026b, 2026c). These facts support retaining the observed boolean; they do not establish a business key, require a particular Rust type, or authorize semantic publication.

The predecessor retained only constraint name and ordered columns, so equal coordinates could conceal different source behavior. The repair preserves three states: unknown, observed distinct, and observed not-distinct. The existing constructor keeps unknown evidence. A consuming builder records an observed boolean without mutating the original value. The canonical digest reuses the existing optional-boolean encoder and moves its framing domain to v2. No driver, database query, general index abstraction or new dependency is added.

The Source Observation owner explicitly handed this bounded repair to an isolated worktree starting at `e3c415600300b6c2d5b852c457ea6ab2e5222e08`, retaining base `fcf36c8a99f015b963c9f812787df127ac2e2f9e`. The Zotero lane remains separate. PostgreSQL unique deferrability, period and wider index semantics were not silently added to this handoff.

## Executed evidence and root repairs

| Exact source coordinate | Executed result | Interpretation |
| --- | --- | --- |
| `e3c415600300b6c2d5b852c457ea6ab2e5222e08` | Workspace baseline: 125 passed, one failed | An existing Client documentation contract lacked the detached-artifact verification explanation in the Gap baseline. No new UNIQUE test was present. |
| `38efc2704b28b6a92c3de695bd8853c34f0af30a` | Two focused Client documentation tests passed | Restore the explanation, not Client runtime or its test threshold. |
| `c50b821798886f7fc4e9a0908ea87ad82d9a498a` test delta | Compiler RED: missing builder/getter, four E0599 errors | Formatting-only worktree changes also existed during this run; do not label it a clean exact-head execution. `27bf48490063942ce1eac670cdebed1d5ce7a78d` retains that pinned-formatter cleanup. |
| `bab6984221808c8ece1d7f3ff1aa57b7ff66ead7` | Clean functional RED: one selected test failed | False and true observations held the same digest despite unequal typed values. |
| `8b5b73889c705f92a5b48e8d8aaa050ee28cb0b5` | Clean framing RED: four passed, two failed | The semantic collision and old framing domain both remained visible. |
| `6c23924f7b27f820f85445abb90cd792021b076d` | Six digest tests and 128 workspace tests passed | Optional evidence now affects snapshot and receipt identity. Strict Clippy and the unchanged coverage gate still failed; test success was not full quality acceptance. |
| `dd2d17708c126974a019f0d1535aee4798132e0a` | Strict Clippy passed | Test-only async wrappers and custom no-op wakers were replaced by standard facilities, plus the suggested slice membership simplification. No warning was suppressed. |
| `7be2707c49ee3a4c9a359317bf04df035ad8fe43` | 132 workspace tests passed; normalized coverage still missed one region and one branch outcome | Added actual missing/unsafe binding, final-policy deadline, expired-capability, malformed digest and cumulative UTF-8 boundary checks. The remaining gap was the unreachable checked-add overflow arm. |
| `e3ac294b976d35f113fe9b920060f62c4a28f57f` | 132 tests across 42 suites, including two doctests; fmt, strict Clippy, warnings-denied rustdoc, release build and unchanged coverage gate passed | Compare the next name with the remaining byte allowance before accumulation. Existing ceilings, typed failure and pre-authorization ordering remain unchanged. |

The functional RED collision was `sha256:afe7306100e50e986daef592c8e1a7ccc6432f855966bba12d2a88448a93272c` for both observed values. The independent v2 empty-snapshot vector is `sha256:81fc16da60127e6574a183cd63077a7136791767240c0868de64b5cbf5bf879e`, calculated with standard SHA-256 over the length-prefixed v2 domain and zero-table frame. Neither digest denotes a live database capture or private paper.

The byte guard's safety follows from its invariant: the accumulated total begins at zero and never exceeds the validated cap. A new length must fit in `cap - accumulated` before addition, keeping both arithmetic operations bounded. This is equivalent admission with fewer branches, not a coverage exclusion or a larger resource allowance.

## Reproduction and metric boundary

Use the repository-pinned Rust 1.98.0 and the existing coverage toolchain. No system Python installation or additional dependency is required.

```sh
cargo +1.98.0 test -p conceptweave-observation --test snapshot_digest_integrity --locked
cargo +1.98.0 test --workspace --locked
cargo +1.98.0 fmt --all --check
cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo +1.98.0 doc --workspace --no-deps --locked
cargo +1.98.0 build --workspace --release --locked
COVERAGE_TOOLCHAIN=nightly-2026-08-20 scripts/check_coverage.sh
uv run --no-project --python 3.14 python scripts/check_ci_contract.py
actionlint .github/workflows/product.yml
```

At the runtime source coordinate above, the unchanged coverage gate reports 228/228 functions, 2026/2026 source-coordinate-normalized regions and 194/194 normalized branch outcomes. Raw LLVM totals remain 1807/1825 lines, 2192/2206 regions and 188/194 branches. The difference is disclosed; raw 100% coverage is not claimed and the coverage script/thresholds were not changed. The existing public-contract lane additionally compiled all three schemas, validated all twelve JSON fixtures with their expected valid/invalid outcomes, and checked accepted/rejected supersession semantics using the installed tools.

## Compatibility, risks and next acceptance

The v2 framing domain changes every new snapshot digest, including snapshots without a unique constraint. Earlier v1 receipts remain immutable historical evidence; do not retrofit missing observations or rehash them in place. Future serialized admission must explicitly bind its framing version and reject unsupported versions. No migration or version-negotiation API is claimed here.

Local fixture evidence does not prove a PostgreSQL extractor reads the right supporting index, a hosted Product check ran, a source system admitted a real operation, or a semantic steward approved a proposal. Keep PR #6 Draft and both ADRs Proposed until prerequisites, current-head independent review and protected checks are satisfied. The concrete adapter must later observe the supporting index under the exact source-policy binding, read-only transaction and remaining operation budget, with frozen anonymized conformance evidence. No Zotero item, private full-text capture, provider route, semantic truth or publication state changed in this repair.

## References (APA 7)

PostgreSQL Global Development Group. (2026a). *Constraints*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/ddl-constraints.html

PostgreSQL Global Development Group. (2026b). *pg_constraint*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026c). *pg_index*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-index.html
