# Match owned production function regions; slot 5 indexes filenames.
  [
    .data[0].functions[]
    | select(.name | (contains("5tests") or contains("20internal_model_tests")) | not)
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
    | select(.file | startswith($root) and contains("/src/"))
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
