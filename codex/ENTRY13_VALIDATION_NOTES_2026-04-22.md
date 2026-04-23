# Entry #13 Validation Notes

- Entry: `prop_mixin_bidirectional_conversion`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_bidirectional_conversion`

## Source Changes in this Entry

- Source changed: **Yes**
- Files changed:
  - `rupta/src/util/class/analysis.rs`
- Why:
  - Treat macro-generated `::to_impl<classes::class::Virtual>` helper context as internal DSL trait method context,
    so shim-local pointers are not materialized into entry-level `class_pts/type-info`.

## Expected Semantics

This entry validates bidirectional mixin conversion correctness:
1. `Eagle -> Flyable -> Animal -> Eagle`
2. `Duck -> Swimmable -> Animal -> Duck`
3. `FlyingFish -> Flyable -> Animal -> FlyingFish`

All round-trip chains should preserve identity and field accessibility, and downcasts back to concrete impl types should be safe.

## Observed Result

- `class_pag`:
  - test entry contains expected cast chains for all three round trips:
    - `impl -> mixin`
    - `mixin -> animal`
    - `animal -> impl`
  - 3 concrete objects allocated in entry context: `Eagle`, `Duck`, `FlyingFish`
- `class_pts` / `type-info` on entry-local pointers:
  - Eagle chain pointers consistently map to `obj_0` / `Eagle`
  - Duck chain pointers consistently map to `obj_1` / `Duck`
  - FlyingFish chain pointers consistently map to `obj_2` / `FlyingFish`
  - no cross-object contamination on these entry-local chains
- `cast_safety.log`:
  - entry-level cast points are all `cast is safe`
    - `main.rs:1104`, `1106`, `1108`
    - `main.rs:1157`, `1159`, `1161`
    - `main.rs:1216`, `1218`, `1220`
- `class_pts` / `type-info`:
  - no `(none)` entries (`ptrs_with_objs: 18/18`, `ptrs_with_types: 18/18`)
  - internal `mixins::*::to_impl<...>` helper pointers no longer appear in this entry output

## Note on Precision Cleanup

To remove non-source-level noise, rcpta now treats macro-generated mixin helper
`::to_impl<classes::class::Virtual>` as internal DSL trait method context, so those
shim-local `param/local/ret` pointers are not materialized in class-level outputs.

## Conclusion

Entry #13 meets precision expectation for bidirectional mixin conversion after the
internal helper filtering fix; no additional rcpta change is necessary for this entry.

## Next

Next candidate entry in source order is:
- `prop_multiple_mixin_independent_conversion`

(Per collaboration rule, ask user confirmation before proceeding.)
