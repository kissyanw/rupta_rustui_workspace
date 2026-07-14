# Entry #5 Validation Notes

- Entry: `test_downcast_animal_dog_to_cat_failure`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_animal_dog_to_cat_failure`

## Expected Semantics

Scenario flow:
1. allocate `Dog`
2. upcast `Dog -> Animal`
3. attempt downcast `Animal -> Cat` (expected failure)

Since dynamic object is Dog, no Cat object should appear in inferred runtime type range.

## Observed Result

- `class_pag`: destination pointer for downcast target has static class type `[Cat]` (as declaration target), and cast edges are present.
- `class_pts`: all tracked pointers point to the same `Dog` object (`obj_0`).
- `type-info`: all tracked pointers infer `Dog`; no `Cat` appears in inferred runtime ranges.
- No `(none)` entries in `class_pts/type-info`.

## Interpretation

This is acceptable and expected:
- `class_pag` pointer annotation reflects static target type (`Cat`).
- `type-info` reflects dynamic reachable object type (`Dog`).

## Conclusion

Entry #5 passes precision expectation for failure-downcast scenario; no immediate rcpta bug exposed here.

## Next

Next in order is entry #6: `test_downcast_animal_eagle_to_penguin_failure`.
(Per latest collaboration rule, ask user confirmation before starting each next entry.)
