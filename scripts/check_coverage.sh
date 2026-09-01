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

jq -e '
  .data[0].totals.lines.percent == 100 and
  .data[0].totals.functions.percent == 100 and
  .data[0].totals.regions.percent == 100 and
  .data[0].totals.branches.percent == 100
' coverage.json >/dev/null
