# Function-coverage denominator integrity

Status: `NATIVE_FUNCTION_GATE_REPAIRED_PENDING_EXECUTION`

## Problem

PR #40 exact predecessor `0327a77759ecedb3786d8d7bbc1c4cad652ea85b` inherited a coverage-gate change that stopped requiring LLVM's native function total to be 100% and instead accepted a repository-derived `source-functions.json` denominator grouped by identical source-region sets. Historical frozen execution at `26d088f948ebef01dad112313391b80cda7a0a71` reported 211 LLVM functions / 2 missed, while the repository-normalized source-region and branch gates were 100%.

The prior doctoring described `.data[0].totals.functions` as a raw instantiation count. That description is not consistent with LLVM's documented coverage semantics. LLVM/Clang and rustc both define **function coverage** as the percentage of functions executed at least once, with a function considered executed when any instantiation executes. **Instantiation coverage** is a separate statistic. LLVM also implements instantiation grouping explicitly in `CoverageMapping::getInstantiationGroups`; the JSON exporter emits both `functions` and `instantiations` summaries.

Therefore the two missed native functions cannot be dismissed as duplicate monomorphized records merely because individual `functions[]` entries can contain codegen/instantiation detail. They may still prove to be test-only functions, instrumentation artifacts, or a Rust coverage defect, but that requires exact symbol/source evidence. Replacing the native function gate before establishing that evidence created a possible false-GREEN path and conflicted with the repository rule against weakening a gate to obtain 100%.

Primary references checked on 2026-09-09:

- LLVM Project. *Source-based Code Coverage*. Function coverage counts a function as executed when any instantiation executes; instantiation coverage is reported separately: https://clang.llvm.org/docs/SourceBasedCodeCoverage.html
- Rust Project. *Instrument-based Code Coverage*. Rust documents the same function-versus-instantiation distinction: https://github.com/rust-lang/rust/blob/main/src/doc/rustc/src/instrument-coverage.md
- LLVM Project. `CoverageMapping::getInstantiationGroups` / `FunctionInstantiationSetCollector`. LLVM groups records belonging to the same source function before producing function-level coverage summaries: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ProfileData/Coverage/CoverageMapping.cpp
- LLVM Project. `CoverageExporterJson.cpp`. The JSON summary exposes distinct `functions` and `instantiations` metrics: https://github.com/llvm/llvm-project/blob/main/llvm/tools/llvm-cov/CoverageExporterJson.cpp
- rust-lang/rust issue #137524. Rust coverage currently has known cross-instantiation region-reporting defects, reinforcing the need to preserve native evidence rather than infer the missing-function cause from aggregate percentages alone: https://github.com/rust-lang/rust/issues/137524

## Reality RED

Doctoring commit `4bf195853c9c3a22a9cf435dbff85a53ae566390` records the verified RED. At that head, `scripts/check_coverage.sh` printed every zero-count raw record as `RAW_FUNCTION_GAP`, derived a normalized source-function view, and failed on normalized source-function/region/branch gaps, but no longer failed when LLVM's native function percentage was below 100%.

That was a real acceptance-path RED against the stated 100% function requirement: a native function miss could pass the final shell predicates before its identity had been shown to be outside owned production. The earlier synthetic fixture was insufficient to justify replacing LLVM's function metric because it assumed that identical source-region sets were necessarily duplicate instantiations of one source function; LLVM has its own instantiation-group semantics and the fixture did not prove equivalence to them.

## Causal repair

Commit `675230ae72325c78de2bcb9b1ad572be0f0dc1ab` restores the fail-closed boundary without discarding the useful normalization diagnostics:

1. every zero-count raw function record remains visible as `RAW_FUNCTION_GAP`;
2. the repository-derived `source-functions.json` view remains an additional owned-production check;
3. normalized owner source-function, source-region, and branch gates remain at 100%; and
4. LLVM native function coverage is again required to equal 100% via `.data | all(.totals.functions.percent == 100)`.

No `coverage(off)` expansion, target omission, sample reduction, threshold reduction, or synthetic success was introduced. A local jq contract check verified that the restored predicate accepts all-100 export objects and fails when any export object reports 99% function coverage. This verifies the predicate itself, not the Rust coverage result.

## Remaining RCA

The historical 211 / 2 native-function deficit is deliberately RED again. The next frozen coverage run must print the exact zero-count function names and source attribution. Only after those two misses are identified may the denominator be reconsidered. If they are unit-test-only functions, introduce a principled test-code ownership boundary with a regression that cannot exclude production functions accidentally. If either is owned production behavior, add deterministic coverage. If evidence shows an upstream instrumentation/reporting defect, preserve the raw/native result and document the exact toolchain reproducer rather than weakening the gate.

## Verification boundary

This repair is not executable GREEN. One unchanged exact successor still needs:

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

No protected merge, immutable release, Zotero mutation, semantic publication, or approval authority follows from this tooling repair alone.
