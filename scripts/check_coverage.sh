#!/usr/bin/env bash
set -euo pipefail

coverage_toolchain="${COVERAGE_TOOLCHAIN:-nightly-2026-08-20}"

cargo "+${coverage_toolchain}" llvm-cov \
  --workspace \
  --branch \
  --json \
  --output-path coverage.json

jq '.data[0].totals' coverage.json
jq -r '
  .data[0].files[]
  | select(
      .summary.lines.percent != 100
      or .summary.functions.percent != 100
      or .summary.regions.percent != 100
      or .summary.branches.percent != 100
    )
  | "COVERAGE_GAP file=\(.filename) lines=\(.summary.lines.percent) functions=\(.summary.functions.percent) regions=\(.summary.regions.percent) branches=\(.summary.branches.percent)"
' coverage.json

jq -r '
  .data[0].files[]
  | .filename as $file
  | (.branches // [])[]
  | select((.[4] // 0) == 0 or (.[5] // 0) == 0)
  | "BRANCH_GAP file=\($file) start=\(.[0]):\(.[1]) end=\(.[2]):\(.[3]) true_count=\(.[4]) false_count=\(.[5])"
' coverage.json

jq -e '
  .data[0].totals.lines.percent == 100 and
  .data[0].totals.functions.percent == 100 and
  .data[0].totals.regions.percent == 100 and
  .data[0].totals.branches.percent == 100
' coverage.json >/dev/null
