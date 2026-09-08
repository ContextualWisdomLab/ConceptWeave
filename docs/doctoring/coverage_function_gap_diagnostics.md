# Function-coverage denominator integrity

Status: `REALITY_RED_NATIVE_FUNCTION_GATE_REPAIR_PENDING`

## Problem

PR #40 exact head `0327a77759ecedb3786d8d7bbc1c4cad652ea85b` inherits a coverage-gate change that stopped requiring LLVM's native function total to be 100% and instead accepts a repository-derived `source-functions.json` denominator grouped by identical source-region sets. Historical frozen execution at `26d088f948ebef01dad112313391b80cda7a0a71` reported 211 LLVM functions / 2 missed, while the repository-normalized source-region and branch gates were 100%.

The prior doctoring described `.data[0].totals.functions` as a raw instantiation count. That description is not consistent with LLVM's documented coverage semantics. LLVM/Clang and rustc both define **function coverage** as the percentage of functions executed at least once, with a function considered executed when any instantiation executes. **Instantiation coverage** is a separate statistic. LLVM also implements instantiation grouping explicitly in `CoverageMapping::getInstantiationGroups`; the JSON exporter emits both `functions` and `instantiations` summaries.

Therefore the two missed native functions cannot be dismissed as duplicate monomorphized records merely because individual `functions[]` entries can contain codegen/instantiation detail. They may still prove to be test-only functions, instrumentation artifacts, or a Rust coverage defect, but that requires exact symbol/source evidence. Replacing the native function gate before establishing that evidence creates a possible false-GREEN path and violates the repository rule against weakening a gate to obtain 100%.

Primary references checked on 2026-09-09:

- LLVM Project. *Source-based Code Coverage*. Function coverage counts a function as executed when any instantiation executes; instantiation coverage is reported separately: https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
- Rust Project. *Instrument-based Code Coverage*. Rust documents the same function-versus-instantiation distinction: https://github.com/rust-lang/rust/blob/main/src/doc/rustc/src/instrument-coverage.md
- LLVM Project. `CoverageMapping::getInstantiationGroups` / `FunctionInstantiationSetCollector`. LLVM groups records belonging to the same source function before producing function-level coverage summaries: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ProfileData/Coverage/CoverageMapping.cpp
- LLVM Project. `CoverageExporterJson.cpp`. The JSON summary exposes distinct `functions` and `instantiations` metrics: https://github.com/llvm/llvm-project/blob/main/llvm/tools/llvm-cov/CoverageExporterJson.cpp
- rust-lang/rust issue #137524. Rust coverage currently has known cross-instantiation region-reporting defects, reinforcing the need to preserve raw/native evidence rather than infer the missing-function cause from aggregate percentages alone: https://github.com/rust-lang/rust/issues/137524

## Reality RED

The current `scripts/check_coverage.sh` prints every zero-count raw record as `RAW_FUNCTION_GAP`, derives a normalized source-function view, and fails on normalized source-function/region/branch gaps. It no longer fails when `.data[0].totals.functions.percent < 100` so long as the custom source-region grouping is positive.

That is a real acceptance-path RED against the stated 100% function requirement: a native function miss can pass the final shell predicates before its identity has been shown to be outside owned production. The existing synthetic fixture is insufficient to justify this replacement because it assumes that identical source-region sets are necessarily duplicate instantiations of one source function; LLVM has its own instantiation-group semantics and the fixture does not prove equivalence to them.

## Least-widening repair

Fail closed while preserving the useful diagnostics:

1. keep `RAW_FUNCTION_GAP` output and the repository-normalized `source-functions.json` view;
2. keep normalized owner source-function, source-region, and branch gates at 100%;
3. restore LLVM native function coverage `== 100%` as an additional acceptance predicate until the two exact native misses are identified and a principled owned-production filter is demonstrated;
4. do not add `coverage(off)`, omit targets, reduce the test sample, lower thresholds, or relabel a native miss as test-only from its aggregate count alone; and
5. on the next frozen run, capture the exact missed function names/source attribution and distinguish unit-test-only code, a genuine production function, and an upstream instrumentation/reporting defect before changing the denominator again.

This is intentionally stricter than the preceding head. It cannot manufacture GREEN by reclassifying evidence.

## Verification boundary

This doctoring commit is the RED/decision record, not executable GREEN. The source repair must be a normal successor. After the repair, one unchanged exact successor still needs:

- locked Rust 1.98 workspace tests;
- `cargo fmt --all -- --check`;
- all-target strict Clippy;
- warnings-denied rustdoc and release build;
- LLVM native function coverage 100% while this conservative gate is in force;
- normalized owned-production source-function coverage 100%;
- normalized owned-production source-region coverage 100%;
- normalized owned-production branch coverage 100%;
- applicable hosted checks; and
- qualifying independent review.

No protected merge, immutable release, Zotero mutation, semantic publication, or approval authority follows from this tooling finding.
