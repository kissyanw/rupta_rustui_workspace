# Entry #2 Validation Notes

- Entry: `test_downcast_animal_to_dog_success`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_animal_to_dog_success`

## Expected Semantics

Scenario is single-object flow:
1. allocate `Dog`
2. upcast `Dog -> Animal`
3. downcast `Animal -> Dog` via `try_into_subtype`
4. unwrap and assign

All class pointers in this entry should resolve to the same Dog object.

## Observed Result

- `class_pag`: 1 object (`Dog`), edges = `alloc + cast + cast + assign`.
- `class_pts`: all 4 pointers point to `obj_0`.
- `type-info`: all 4 pointers infer `Dog`.
- No `(none)` entries.

## Conclusion

Entry #2 passes precision expectation; no new rcpta bug exposed in this case.

## Next

Proceed to entry #3: `test_downcast_animal_to_eagle_through_bird`.
