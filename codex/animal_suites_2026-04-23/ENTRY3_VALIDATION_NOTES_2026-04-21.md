# Entry #3 Validation Notes

- Entry: `test_downcast_animal_to_eagle_through_bird`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_downcast_animal_to_eagle_through_bird`

## Expected Semantics

Scenario flow:
1. allocate `Eagle`
2. upcast `Eagle -> Bird -> Animal`
3. downcast `Animal -> Bird -> Eagle`
4. unwrap and assign

All class pointers in this entry should remain identity-equivalent to the original Eagle object.

## Observed Result

- `class_pag`: 1 object (`Eagle`), Cast/Assign chain matches conversion sequence.
- `class_pts`: all 8 pointers point to `obj_0`.
- `type-info`: all 8 pointers infer `Eagle`.
- No `(none)` entries in `class_pts/type-info`.

## Conclusion

Entry #3 passes precision expectation; no new rcpta bug exposed.

## Next

Proceed to entry #4: `test_downcast_animal_to_shark_through_fish`.
