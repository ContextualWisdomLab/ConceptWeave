# Match LLVM instantiation groups: main file and first region start location.
[
  .data[0].functions[]
  | select(.name | (contains("5tests") or contains("20internal_model_tests")) | not)
  | [.regions[] | select(.[7] == 1) | .[6]] as $expanded
  | ([range(0; .filenames | length)
      | select(. as $id | $expanded | index($id) | not)][0]) as $main
  | select($main != null)
  | ([.regions[] | select(.[5] == $main)][0]) as $entry
  | select($entry != null)
  | {
      file: .filenames[$main],
      line_start: $entry[0],
      column_start: $entry[1],
      count: .count
    }
  | select(.file | startswith($root) and contains("/src/"))
]
| sort_by(.file, .line_start, .column_start)
| group_by([.file, .line_start, .column_start])
| map(.[0] + {count: (map(.count) | add)})
