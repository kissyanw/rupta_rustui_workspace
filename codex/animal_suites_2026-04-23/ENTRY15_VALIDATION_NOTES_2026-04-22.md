# Entry #15 Validation Notes

- Entry: `test_polymorphic_collection`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection`

## Source Changes in this Entry

- Source changed: **Yes**
- Files changed:
  - `rupta/src/builder/fpag_builder.rs`
- Why:
  - Add iterator `next` modeling for `Option<(idx, &Class)>` item extraction, so loop-element pointer
    (e.g. `...as_variant#1.0.1`) receives points-to from collection items and can propagate to `CallArg` formals.

## Expected Semantics

This entry validates polymorphic collection behavior:
1. build `Vec<CRc<Animal>>` from 9 concrete animal instances
2. iterate collection and call `make_sound`, `move_action`, `describe`

Expected rcpta behavior:
- collection element pointer should map to the union of 9 concrete objects
- call arguments to `Animal` methods should carry non-empty dynamic type ranges

## Observed Result

- `class_pag`:
  - object allocations and upcast chains are present for all 9 animals
  - a loop element pointer is materialized as:
    - `tests::test_polymorphic_collection#1::local_134.as_variant#1.0.1  [Animal]`
  - rcpta now adds conservative iterator-item assign edges:
    - `local_69/71/73/77/81/85/89/93/97 -> local_134.as_variant#1.0.1`
  - `CallArg` edges exist from this pointer to:
    - `animalAnimal::make_sound::param_1`
    - `animalAnimal::move_action::param_1`
    - `animalAnimal::describe::param_1`
- `class_pts` / `type-info`:
  - all entry-local pointers are non-empty and type-complete
  - loop item pointer and polymorphic method formals now infer the full 9-type union:
    - `Cat, Dog, Duck, Eagle, FlyingFish, Ostrich, Penguin, Salmon, Shark`
- `cast_safety.log`:
  - all cast sites in this entry are reported `cast is safe`

## Interpretation

The iterator-element propagation gap is fixed for this entry:
- values in polymorphic collection now flow to iterated element pointer
- CallArg propagation into `Animal` method formals is no longer empty

## Conclusion

Entry #15 now meets precision expectation after the iterator `next` modeling fix; no residual `(none)` remains on the polymorphic loop path.

## Next

Next candidate entry in source order is:
- `test_polymorphic_collection_by_category`

(Per collaboration rule, ask user confirmation before proceeding.)
