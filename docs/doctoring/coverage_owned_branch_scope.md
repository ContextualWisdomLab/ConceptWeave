# Owned-production branch coverage scope

Status: repaired in source, executable Rust coverage pending.

## Problem

`llvm-cov export` exposes branch records at the file level. ConceptWeave's coverage script already removed Rust unit-test functions from the normalized function and region denominators by rejecting mangled names containing the `tests` module segment, and it removed integration-test files under `/tests/`. The branch path did neither: every entry in `.data[0].files[].branches` entered `source-branches.json`, including branches emitted by inline `#[cfg(test)]` modules located in `src/*.rs`.

That made the three normalized denominators inconsistent. A branch in test implementation could fail the gate even when every owned-production branch outcome was covered. The failure is conservative rather than a false GREEN, but it violates the stated `owned production function/region/branch` contract and encourages test-only coverage work instead of causal production coverage.

## Reality RED

A minimal LLVM-shaped JSON fixture used one production function mapped to `/repo/crates/foo/src/lib.rs` lines 10–20 and one inline test function in the same file mapped to lines 100–110. The production branch at line 15 had both outcomes covered; the test branch at line 105 had only the true outcome covered.

The predecessor branch aggregation admitted both file-level branch records and therefore failed:

```text
old_gate=FAIL
```

This is a gate-normalization RED, not a claim about the current Rust workspace coverage total. Synthetic data is used only for this deterministic contract test; it is not substituted for product coverage evidence.

## Causal repair

Commit `14d610cf3909eebb075637409068ef76b6665aed` keeps LLVM's native function gate unchanged and keeps the existing non-test function/region normalization. Before aggregating file-level branches, it now requires each branch source span to be contained by at least one already-normalized non-test production region from `source-regions.json`.

On the same fixture the normalized branch set contains only the production branch and the gate passes:

```text
fixed_gate=PASS
```

The repair does not change counters, thresholds, runtime targets, sample size, coverage attributes, or the native LLVM function denominator. It only prevents inline test implementation from entering the repository's additional owned-production branch denominator.

## Why this mapping is defensible

LLVM documents branch regions as source ranges with separate true and false counters, and its JSON export exposes functions, regions, branches, and summaries. Rust 1.98's coverage mapping represents a `BranchRegion` with a `CoverageSpan` plus true/false counters, while per-function coverage mapping carries source-file coordinates. Using the already accepted non-test production regions as the admission map therefore preserves source-location provenance instead of guessing from a file path alone.

Rejected alternatives:

- ignoring every branch in `src/*.rs` that happens to be near a `#[cfg(test)]` marker: text parsing would duplicate Rust syntax semantics and be brittle around nested modules and macros;
- excluding all branches from files that contain tests: that would remove production obligations and weaken the gate;
- removing branch coverage or lowering the threshold: that would directly violate the 100% owned-production acceptance contract;
- treating the historical 212/212 branch result as current GREEN: the denominator changed and must be re-executed on one unchanged exact head.

## Acceptance

The source repair is not executable GREEN for PR #40. A successor head must still run the frozen coverage toolchain and show native LLVM function 100% plus normalized owned-production function, region, and branch 100%, together with locked Rust 1.98 workspace tests, formatting, all-target strict Clippy, warnings-denied rustdoc/release, hosted checks, and qualifying independent review. Any production branch that cannot be mapped to a non-test production source region must be investigated against the LLVM export before changing the denominator again.

## References

LLVM Project. (2026). *llvm-cov - emit coverage information*. LLVM documentation. https://llvm.org/docs/CommandGuide/llvm-cov.html

LLVM Project. (2026). *LLVM code coverage mapping format*. LLVM documentation. https://llvm.org/docs/CoverageMappingFormat.html

Rust Project. (2026). *BranchRegion in rustc_codegen_llvm::coverageinfo::ffi* (rustc 1.98). Rust compiler documentation. https://doc.rust-lang.org/stable/nightly-rustc/rustc_codegen_llvm/coverageinfo/ffi/struct.BranchRegion.html
