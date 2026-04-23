# Entry #14 Validation Notes

- Entry: `prop_multiple_mixin_independent_conversion`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multiple_mixin_independent_conversion`

## Source Changes in this Entry

- Source changed: **No**
- Files changed: (none)

## Expected Semantics

This entry validates independent repeated mixin conversions on the same object:
1. `Duck` repeatedly converts to `Flyable` and `Swimmable`
2. `FlyingFish` repeatedly converts to `Flyable` and `Swimmable`

Each conversion path should stay identity-consistent: repeated `cast_mixin` calls should point to the same runtime object and preserve field-read consistency.

## Observed Result

- `class_pag`:
  - two concrete objects allocated in entry scope: `Duck` (`obj_0`) and `FlyingFish` (`obj_1`)
  - expected repeated cast chains are present for both families:
    - `Duck -> Flyable` and `Duck -> Swimmable` (twice each)
    - `FlyingFish -> Flyable` and `FlyingFish -> Swimmable` (twice each)
- `class_pts`:
  - all 18 pointers are non-empty (`ptrs_with_objs: 18/18`)
  - all Duck-side pointers map to `obj_0`
  - all FlyingFish-side pointers map to `obj_1`
  - no cross-family contamination
- `type-info`:
  - all 18 pointers have inferred types (`ptrs_with_types: 18/18`)
  - Duck-side pointers infer `Duck`
  - FlyingFish-side pointers infer `FlyingFish`
  - no `(none)` entries
- `cast_safety.log`:
  - all entry-level cast sites are `cast is safe`
  - includes lines `1285/1289/1293/1297` and `1347/1351/1355/1359` in `main.rs`

## Interpretation

The analysis is consistent with the entry's property intent:
- repeated mixin conversions do not introduce spurious aliasing or type drift
- both object families remain independent and precise under rcpta propagation

## Conclusion

Entry #14 passes precision expectation; no new rcpta code change is necessary.

## Next

Next candidate entry in source order is:
- `test_polymorphic_collection`

(Per collaboration rule, ask user confirmation before proceeding.)
