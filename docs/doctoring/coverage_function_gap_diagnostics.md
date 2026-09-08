# Owned production function coverage normalization

Status: `COVERAGE_GATE_SCOPE_REPAIRED_PENDING_EXECUTION`

## Problem

Exact-head local revalidation on PR #40 at `26d088f948ebef01dad112313391b80cda7a0a71` completed the frozen Rust/nightly coverage run with raw LLVM totals of 3039 regions / 38 missed, 211 functions / 2 missed, 1913 lines / 15 missed and 212 branches / 0 missed. The repository-normalized owner source-region and branch gates were 100%.

The remaining failure came from `scripts/check_coverage.sh` asserting `.data[0].totals.functions.percent == 100`. That value is the raw LLVM function total, not the repository's owned-production source-function denominator. This contradicted `AGENTS.md`, which requires a normalized owner function/region/branch gate and 100% owned production coverage.

The distinction matters for Rust coverage. `cargo-llvm-cov` excludes integration-test paths such as `tests/` from reports by default but does not automatically exclude unit-test functions inside production source files; its documented workaround for unit-test code is a coverage attribute. LLVM coverage mapping is also emitted per instrumented function, so multiple compiled instantiations can map back to the same source function. ConceptWeave already avoids using raw region multiplicity as its acceptance denominator by filtering test-only mappings and grouping regions by source coordinates. Keeping a raw global function percentage as the only function gate made the three coverage dimensions inconsistent.

Primary references:

- `cargo-llvm-cov` README, “Exclude file from coverage” / “Exclude code from coverage”: https://github.com/taiki-e/cargo-llvm-cov/blob/main/README.md
- LLVM, *Code Coverage Mapping Format*, “Advanced Concepts” and “Mapping Region”: https://llvm.org/docs/CoverageMappingFormat.html

## Rejected alternatives

- Lower the 100% threshold — rejected; the owned production threshold remains 100%.
- Add or broaden `coverage(off)` on unit-test modules or production helpers — rejected; the gate should identify the correct denominator rather than manufacture coverage through suppression.
- Omit targets, reduce the test sample, or ignore raw misses — rejected; raw LLVM totals and every zero-count raw function remain visible diagnostic evidence.
- Guess which two raw functions are harmless from aggregate percentages — rejected; exact raw names remain diagnostic data and any normalized production gap still fails closed.

## Decision

Commit `922804492d581bcdd50a6ae47c5e33ab51420d05` repairs the acceptance denominator without changing production source or the 100% requirement.

`scripts/check_coverage.sh` now:

1. prints every raw zero-count LLVM function as `RAW_FUNCTION_GAP`;
2. derives `source-functions.json` from non-test mappings using the same `5tests` and `/tests/` ownership boundary already used for normalized regions;
3. groups compiled function records by their source-region identity and sums execution counts, so repeated codegen/monomorphized records for one owned source function do not create a second source-function obligation;
4. reports a zero-count normalized owner function as `FUNCTION_GAP` with source location and contributing raw names; and
5. requires every normalized owner source function to have a positive execution count.

Raw totals therefore remain observable and auditable, but test harness functions or duplicate codegen records no longer substitute for the owned-production acceptance denominator. A genuine owned production source function with zero execution still fails the gate.

## Verification performed in this repair

The changed shell script passes `bash -n` locally. The normalization jq expression was also executed against a synthetic LLVM-shaped JSON fixture containing:

- two raw instantiations with the same production source regions, one executed and one zero-count;
- a distinct zero-count production source function;
- a zero-count unit-test function whose mangled identity contains `5tests`; and
- a zero-count integration-test function attributed only to `/tests/`.

The two production instantiations collapsed to one covered source function, test-only entries were excluded from the owned denominator, and the separate zero-count production function remained a failing `FUNCTION_GAP`. This verifies the scope transformation itself; it is not an exact-head Rust coverage GREEN claim.

## Evidence boundary

The exact `26d088...` run remains historical evidence for the original raw/function-scope discrepancy. It does not transfer to `922804...`, whose coverage script changed. The new exact successor still needs a frozen nightly coverage execution. If `FUNCTION_GAP` is emitted, repair the named owned behavior with a deterministic test or the least-widening production/test refactor. If only `RAW_FUNCTION_GAP` remains while normalized owner functions, regions and branches are all covered, preserve the raw diagnostics but do not relabel test/codegen multiplicity as uncovered production behavior.

## Acceptance

On one unchanged exact successor, require:

- locked Rust 1.98 workspace tests;
- `cargo fmt --all -- --check`;
- all-target strict Clippy;
- warnings-denied rustdoc and release build;
- normalized owned production function coverage 100%;
- normalized owned production source-region coverage 100%;
- normalized owned production branch coverage 100%;
- applicable hosted checks; and
- qualifying independent review.

No protected merge, immutable release, Zotero mutation, semantic publication, or approval authority follows from this tooling repair alone.
