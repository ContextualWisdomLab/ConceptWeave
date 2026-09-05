#!/usr/bin/env bash
set -euo pipefail

coverage_toolchain="${COVERAGE_TOOLCHAIN:-nightly-2026-08-20}"
trap 'rm -f coverage.json source-branches.json' EXIT

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
    )
  | "COVERAGE_GAP file=\(.filename) lines=\(.summary.lines.percent) functions=\(.summary.functions.percent) regions=\(.summary.regions.percent)"
' coverage.json

jq '
  [
    .data[0].files[]
    | .filename as $file
    | (.branches // [])[]
    | {
        file: $file,
        line_start: .[0],
        column_start: .[1],
        line_end: .[2],
        column_end: .[3],
        true_count: .[4],
        false_count: .[5]
      }
  ]
  | sort_by(.file, .line_start, .column_start, .line_end, .column_end)
  | group_by([.file, .line_start, .column_start, .line_end, .column_end])
  | map({
      file: .[0].file,
      line_start: .[0].line_start,
      column_start: .[0].column_start,
      line_end: .[0].line_end,
      column_end: .[0].column_end,
      true_count: (map(.true_count) | add),
      false_count: (map(.false_count) | add)
    })
' coverage.json > source-branches.json

jq '
  {
    count: (length * 2),
    covered: ([.[] | (.true_count > 0), (.false_count > 0) | select(.)] | length),
    notcovered: ([.[] | (.true_count == 0), (.false_count == 0) | select(.)] | length)
  }
  | .percent = (if .count == 0 then 100 else (.covered * 100 / .count) end)
' source-branches.json

jq -r '
  .[]
  | select(.true_count == 0 or .false_count == 0)
  | "BRANCH_GAP file=\(.file) start=\(.line_start):\(.column_start) end=\(.line_end):\(.column_end) true_count=\(.true_count) false_count=\(.false_count)"
' source-branches.json

jq -e '
  .data[0].totals.lines.percent == 100 and
  .data[0].totals.functions.percent == 100 and
  .data[0].totals.regions.percent == 100
' coverage.json >/dev/null

jq -e 'all(.[]; .true_count > 0 and .false_count > 0)' source-branches.json >/dev/null
