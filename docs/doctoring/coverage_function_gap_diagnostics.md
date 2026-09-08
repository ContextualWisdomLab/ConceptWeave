# Raw function coverage gap diagnostics

Status: `COVERAGE_RED_DIAGNOSTIC_REPAIRED_PENDING_EXECUTION`

## Problem

PR #40's frozen coverage evidence on `4763b595182dc6a165bdfed74ccbb54ed5d0f289` reports 211 raw LLVM functions with 2 missed while the repository-normalized source-region and branch aggregations are complete. `scripts/check_coverage.sh` correctly fails unless `.data[0].totals.functions.percent == 100`, but before `544999aaaa886c17c3c3ddcad01af7776b399bad` it printed only aggregate/file percentages. The two zero-count function identities were therefore not present in the failure output needed for a causal repair.

## Constraint

The raw-function threshold is an acceptance contract, not a metric to normalize away. Coverage exclusions, broader `coverage(off)`, target omission, sample reduction, or threshold weakening are rejected. A refactor must not be selected from generic/monomorphization speculation before the exact zero-count functions are observable.

## Decision

Commit `544999aaaa886c17c3c3ddcad01af7776b399bad` adds a read-only diagnostic before the unchanged raw-function assertion:

```text
FUNCTION_GAP name=<LLVM function name> files=<attributed source files>
```

It reads `.data[0].functions[]`, emits entries whose raw `count == 0`, and leaves every existing region, branch and function gate unchanged.

## Evidence boundary

The preceding frozen run remains RED: 3039 regions / 38 missed, 211 functions / 2 missed, 1913 lines / 15 missed, 212 branches / 4 missed. Normalized source-region and branch aggregation is 100%. This note does not transfer that run into an exact-head GREEN claim for the diagnostic successor.

The automation execution environment used for this repair has no Rust toolchain, and exact `544999aaaa886c17c3c3ddcad01af7776b399bad` has no pull-request workflow run. Therefore the diagnostic is source-reviewed but not yet executed here.

## Acceptance

Run the frozen `scripts/check_coverage.sh` on one unchanged successor. Preserve the emitted `FUNCTION_GAP` lines as exact evidence, identify whether each missed function is owned behavior, a duplicated generic/codegen boundary, or another instrumentation artifact, then make the least-widening causal source/test repair. Completion still requires raw function coverage 100%, normalized source-region/branch 100%, Rust 1.98 workspace/fmt/all-target Clippy/warnings-denied rustdoc/release, applicable hosted checks and qualifying independent review.