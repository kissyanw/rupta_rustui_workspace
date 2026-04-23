# Entry #8 Validation Notes

- Entry: `test_downcast_fish_shark_to_salmon_failure`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_fish_shark_to_salmon_failure`

## Expected Semantics

Scenario flow:
1. allocate `Shark`
2. upcast `Shark -> Fish`
3. attempt downcast `Fish -> Salmon` (expected failure)

Since runtime object is `Shark`, inferred runtime type range should not include `Salmon`.

## Observed Result

- `class_pag`: conversion chain is present; downcast destination pointer has static class type `[Salmon]`.
- `class_pts`: all tracked pointers point to the same object `obj_0`.
- `type-info`: all tracked pointers infer `Shark`; no `Salmon` appears in dynamic type ranges.
- `cast_safety.log`: marks `Fish(Shark) -> Salmon` cast as unsafe with reason `Shark` is not subtype of `Salmon`.

## Interpretation

This is expected behavior:
- `class_pag` static pointer typing and `type-info` dynamic inference remain consistent for failure-downcast.
- `cast_safety` output is aligned with inferred dynamic types.

## Conclusion

Entry #8 passes precision expectation for failure-downcast scenario; no rcpta code change is necessary.

## Next

Next in order is entry #9: `test_downcast_does_not_panic`.
(Per collaboration rule, ask user confirmation before proceeding.)
