# Entry #7 Validation Notes

- Entry: `test_downcast_bird_eagle_to_duck_failure`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_bird_eagle_to_duck_failure`

## Expected Semantics

Scenario flow:
1. allocate `Eagle`
2. upcast `Eagle -> Bird`
3. attempt downcast `Bird -> Duck` (expected failure)

Since runtime object is `Eagle`, inferred runtime type range should not include `Duck`.

## Observed Result

- `class_pag`: conversion chain is present; downcast destination pointer has static class type `[Duck]`.
- `class_pts`: all tracked pointers point to the same object `obj_0`.
- `type-info`: all tracked pointers infer `Eagle`; no `Duck` appears in dynamic type ranges.
- `cast_safety.log`: marks `Bird(Eagle) -> Duck` cast as unsafe with reason `Eagle` is not subtype of `Duck`.

## Interpretation

This is expected behavior:
- `class_pag` static pointer typing and `type-info` dynamic inference are consistent with failure-downcast semantics.
- `cast_safety` diagnostics match inferred dynamic types.

## Conclusion

Entry #7 passes precision expectation for failure-downcast scenario; no rcpta code change is necessary.

## Next

Next in order is entry #8: `test_downcast_fish_shark_to_salmon_failure`.
(Per collaboration rule, ask user confirmation before proceeding.)
