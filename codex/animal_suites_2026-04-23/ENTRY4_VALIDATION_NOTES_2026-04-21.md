# Entry #4 Validation Notes

- Entry: `test_downcast_animal_to_shark_through_fish`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_animal_to_shark_through_fish`

## Expected Semantics

Scenario flow:
1. allocate `Shark`
2. upcast `Shark -> Fish -> Animal`
3. downcast `Animal -> Fish -> Shark`
4. unwrap and assign

All class pointers in this entry should resolve to the same Shark object.

## Observed Result

- `class_pag`: 1 object (`Shark`), Cast/Assign chain matches expected conversion path.
- `class_pts`: all 8 pointers point to `obj_0`.
- `type-info`: all 8 pointers infer `Shark`.
- No `(none)` entries in `class_pts/type-info`.

## Conclusion

Entry #4 passes precision expectation; no new rcpta bug exposed.

## Next

Proceed to entry #5: `test_downcast_animal_dog_to_cat_failure`.
