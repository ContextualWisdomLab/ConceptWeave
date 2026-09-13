#!/usr/bin/env bash
set -euo pipefail

coverage_toolchain="${COVERAGE_TOOLCHAIN:-nightly-2026-08-20}"
trap 'rm -f coverage.json source-functions.json source-branches.json source-regions.json' EXIT

normalize_branches() {
  local coverage_path="$1"
  local production_regions_path="$2"
  local output_path="$3"

  jq --slurpfile production_regions "$production_regions_path" '
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
        } as $branch
      | select(any($production_regions[0][];
          .file == $branch.file
          and (
            .line_start < $branch.line_start
            or (
              .line_start == $branch.line_start
              and .column_start <= $branch.column_start
            )
          )
          and (
            .line_end > $branch.line_end
            or (
              .line_end == $branch.line_end
              and .column_end >= $branch.column_end
            )
          )
        ))
      | $branch
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
  ' "$coverage_path" > "$output_path"
}

normalize_functions() {
  local coverage_path="$1"
  local output_path="$2"

  jq '
    [
      .data[0].functions[]
      | select(.name | contains("5tests") | not)
      | . as $function
      | [
          .regions[]
          | {
              file: $function.filenames[.[5]],
              line_start: .[0],
              column_start: .[1],
              line_end: .[2],
              column_end: .[3]
            }
          | select(.file | contains("/tests/") | not)
        ]
        | sort_by(.file, .line_start, .column_start, .line_end, .column_end) as $source_regions
      | select($source_regions | length > 0)
      | {
          origin: $source_regions[0],
          symbol_identity: ($function.name | sub("Cs[[:alnum:]_]+_19conceptweave_"; "19conceptweave_")),
          source_regions: $source_regions,
          raw_name: $function.name,
          count: $function.count
        }
    ]
    | sort_by(.origin, .symbol_identity)
    | group_by([.origin, .symbol_identity])
    | map({
        source_regions: ([.[].source_regions[]] | unique | sort_by(.file, .line_start, .column_start, .line_end, .column_end)),
        raw_names: (map(.raw_name) | unique),
        count: (map(.count) | add)
      })
  ' "$coverage_path" > "$output_path"
}

check_branch_normalization_contract() {
  (
    local contract_dir
    contract_dir=$(mktemp -d "${TMPDIR:-/tmp}/conceptweave-coverage-contract.XXXXXX")
    trap 'rm -rf -- "${contract_dir:?}"' EXIT

    jq -n '{data:[{files:[{filename:"/repo/src/lib.rs",branches:[[10,1,20,1,1,1,0,0,0],[30,5,30,10,1,0,0,0,0]]}]}]}' > "$contract_dir/coverage.json"
    jq -n '[{file:"/repo/src/lib.rs",line_start:10,column_start:1,line_end:20,column_end:1,count:1},{file:"/repo/src/decoy.rs",line_start:30,column_start:5,line_end:30,column_end:10,count:1}]' > "$contract_dir/source-regions.json"
    normalize_branches "$contract_dir/coverage.json" "$contract_dir/source-regions.json" "$contract_dir/source-branches.json"
    jq -e '. == [{file:"/repo/src/lib.rs",line_start:10,column_start:1,line_end:20,column_end:1,true_count:1,false_count:1}]' "$contract_dir/source-branches.json" >/dev/null
  )
}

check_function_normalization_contract() {
  (
    local contract_dir
    contract_dir=$(mktemp -d "${TMPDIR:-/tmp}/conceptweave-function-contract.XXXXXX")
    trap 'rm -rf -- "${contract_dir:?}"' EXIT

    jq -n '{data:[{functions:[
      {name:"_RNvCsAAAA_19conceptweave_zotero9read_page",count:0,filenames:["/repo/src/lib.rs"],regions:[[10,1,20,1,0,0,0,0]]},
      {name:"_RNvCsBBBB_19conceptweave_zotero9read_page",count:1,filenames:["/repo/src/lib.rs"],regions:[[10,1,20,1,1,0,0,0],[12,1,12,8,1,0,0,0]]},
      {name:"_RNvCsCCCC_19conceptweave_zotero10other_here",count:1,filenames:["/repo/src/lib.rs"],regions:[[10,1,20,1,1,0,0,0]]},
      {name:"crate::other",count:1,filenames:["/repo/src/lib.rs"],regions:[[30,1,35,1,1,0,0,0]]},
      {name:"crate::5tests::helper",count:0,filenames:["/repo/src/lib.rs"],regions:[[40,1,45,1,0,0,0,0]]}
    ]}]}' > "$contract_dir/coverage.json"
    normalize_functions "$contract_dir/coverage.json" "$contract_dir/source-functions.json"
    jq -e '
      length == 3
      and any(.[];
        .count == 1
        and .raw_names == ["_RNvCsAAAA_19conceptweave_zotero9read_page", "_RNvCsBBBB_19conceptweave_zotero9read_page"]
      )
      and any(.[];
        .count == 1
        and .raw_names == ["_RNvCsCCCC_19conceptweave_zotero10other_here"]
      )
      and any(.[]; .count == 1 and .raw_names == ["crate::other"])
    ' "$contract_dir/source-functions.json" >/dev/null
  )
}

check_branch_normalization_contract
check_function_normalization_contract

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

# Preserve every zero-count LLVM function record as diagnostic evidence. LLVM's
# native function summary already considers a function covered when any
# instantiation executes; instantiation coverage is a separate metric. The
# repository-normalized source-function view below is therefore an additional
# owned-production scope check, not a replacement for the native function gate.
jq -r '
  .data[0].functions[]
  | select(.count == 0)
  | "RAW_FUNCTION_GAP name=\(.name) files=\(.filenames | join(","))"
' coverage.json

normalize_functions coverage.json source-functions.json

jq '
  {
    count: length,
    covered: ([.[] | select(.count > 0)] | length),
    notcovered: ([.[] | select(.count == 0)] | length)
  }
  | .percent = (if .count == 0 then 100 else (.covered * 100 / .count) end)
' source-functions.json

jq -r '
  .[]
  | select(.count == 0)
  | .source_regions[0] as $origin
  | "FUNCTION_GAP file=\($origin.file) start=\($origin.line_start):\($origin.column_start) names=\(.raw_names | join(","))"
' source-functions.json

jq '
  [
    .data[0].functions[]
    | select(.name | contains("5tests") | not)
    | .filenames as $files
    | .regions[]
    | {
        file: $files[.[5]],
        line_start: .[0],
        column_start: .[1],
        line_end: .[2],
        column_end: .[3],
        count: .[4]
      }
    | select(.file | contains("/tests/") | not)
  ]
  | sort_by(.file, .line_start, .column_start, .line_end, .column_end)
  | group_by([.file, .line_start, .column_start, .line_end, .column_end])
  | map({
      file: .[0].file,
      line_start: .[0].line_start,
      column_start: .[0].column_start,
      line_end: .[0].line_end,
      column_end: .[0].column_end,
      count: (map(.count) | add)
    })
' coverage.json > source-regions.json

jq '
  {
    count: length,
    covered: ([.[] | select(.count > 0)] | length),
    notcovered: ([.[] | select(.count == 0)] | length)
  }
  | .percent = (if .count == 0 then 100 else (.covered * 100 / .count) end)
' source-regions.json

jq -r '
  .[]
  | select(.count == 0)
  | "REGION_GAP file=\(.file) start=\(.line_start):\(.column_start) end=\(.line_end):\(.column_end)"
' source-regions.json

# LLVM's file-level branch list also contains branches emitted by inline
# #[cfg(test)] modules that live under src/*.rs. Bind each normalized branch to
# a non-test production region so function, region, and branch scope agree.
normalize_branches coverage.json source-regions.json source-branches.json

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

# Fail closed on LLVM's native function coverage as well as the repository's
# additional owned-production normalization. Do not infer that a native miss is
# test-only or duplicate codegen until exact symbol/source evidence proves it.
jq -e '.data | all(.totals.functions.percent == 100)' coverage.json >/dev/null
jq -e 'length > 0 and all(.[]; .count > 0)' source-functions.json >/dev/null
jq -e 'length > 0 and all(.[]; .count > 0)' source-regions.json >/dev/null
jq -e 'length > 0 and all(.[]; .true_count > 0 and .false_count > 0)' source-branches.json >/dev/null
