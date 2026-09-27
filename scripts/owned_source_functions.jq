# Entry-region slot 5 identifies the function owner, including macro file tables.
[
  .data[0].functions[]
  | select(.name | contains("5tests") | not)
  | .regions[0] as $entry
  | {
      file: .filenames[$entry[5]],
      name: .name,
      count: .count
    }
  | select(.file | startswith($root) and contains("/src/"))
]
| sort_by(.file, .name)
| group_by([.file, .name])
| map({file: .[0].file, name: .[0].name, count: (map(.count) | add)})
