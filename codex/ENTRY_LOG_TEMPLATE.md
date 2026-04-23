# Entry Validation Notes Template

- Entry: `<entry_func_name>`
- Config: `context-depth 1`
- Output dir: `<analysis_output_dir>`

## Source Changes in this Entry

- Source changed: **Yes/No**
- Files changed:
  - `<path/to/file>` (if any)
- Why:
  - `<brief reason>` (if changed)

## Expected Semantics

1. `<expected flow 1>`
2. `<expected flow 2>`

## Observed Result

- `class_pag`:
  - `<key graph observations>`
- `class_pts`:
  - `<non-empty/empty and object mapping>`
- `type-info`:
  - `<inferred dynamic type quality>`
- `cast_safety.log`:
  - `<safe/unsafe summary>`

## Interpretation

- `<whether behavior matches semantics>`

## Conclusion

`<pass/fail and whether rcpta change needed>`

## Next

Next candidate entry in source order is:
- `<next_entry>`

(Per collaboration rule, ask user confirmation before proceeding.)
