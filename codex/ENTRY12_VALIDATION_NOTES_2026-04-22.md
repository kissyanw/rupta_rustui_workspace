# Entry #12 Validation Notes

- Entry: `prop_mixin_reference_access_integrity`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_reference_access_integrity`

## Expected Semantics

This entry validates mixin reference field-access integrity:
1. `Eagle -> Flyable` access should preserve `max_altitude`
2. `Penguin -> Swimmable` access should preserve `swim_speed`
3. `Duck -> Flyable/Swimmable` access should preserve both fields
4. `Shark -> Swimmable` access should preserve `swim_speed`
5. `FlyingFish -> Flyable/Swimmable` access should preserve both fields

Runtime dynamic types should remain at concrete object types and not drift to unrelated classes.

## Observed Result

- `class_pag`:
  - 5 allocated objects: `Eagle`, `Penguin`, `Duck`, `Shark`, `FlyingFish`
  - cast edges correctly model `impl -> mixin` paths (`Flyable`/`Swimmable`)
  - no unexpected load/store/call propagation in this entry
- `class_pts`:
  - all 19 pointers have non-empty points-to (`ptrs_with_objs: 19/19`)
  - scenario-local mapping is preserved:
    - Eagle chain -> `obj_0`
    - Penguin chain -> `obj_1`
    - Duck chain -> `obj_2`
    - Shark chain -> `obj_3`
    - FlyingFish chain -> `obj_4`
- `type-info`:
  - all 19 pointers have inferred dynamic types (`ptrs_with_types: 19/19`)
  - inferred types stay concrete and precise (`Eagle/Penguin/Duck/Shark/FlyingFish`)
  - no `(none)` entries
- `cast_safety.log`:
  - all relevant cast points in this entry are `cast is safe`
  - no `cast is unsafe` entries

## Interpretation

The result is aligned with mixin-reference access semantics:
- static pointer typing in `class_pag` and dynamic type inference in `type-info` are consistent
- no cross-object contamination in `class_pts`
- no new rcpta precision regression is exposed

## Conclusion

Entry #12 passes precision expectation; no rcpta code change is necessary.

## Next

Next candidate entry in source order is:
- `prop_mixin_bidirectional_conversion`

(Per collaboration rule, ask user confirmation before proceeding.)
