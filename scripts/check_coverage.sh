#!/usr/bin/env bash
set -euo pipefail

coverage_toolchain="${COVERAGE_TOOLCHAIN:-nightly-2026-08-20}"

cargo "+${coverage_toolchain}" llvm-cov \
  --workspace \
  --branch \
  --json \
  --output-path coverage.json

jq -e '
  .data[0].totals.lines.percent == 100 and
  .data[0].totals.functions.percent == 100 and
  .data[0].totals.regions.percent == 100 and
  .data[0].totals.branches.percent == 100
' coverage.json >/dev/null
