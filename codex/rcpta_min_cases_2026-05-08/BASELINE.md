# Baseline

Date: 2026-05-08

Entries:
- min_iter_next_unwrap_drive_target_2026-05-08
- min_index_drive_target_2026-05-08_fix

Observed:
- `build_drivables::local_45.deref.index` has objects `{obj_0,obj_1,obj_2,obj_3}`.
- Caller-side refs are empty:
  - `test_rcpta_min_iter_next_unwrap_drive#1::local_5`, `local_6`
  - `test_rcpta_min_index_drive#1::local_2`
- `interfacesDrivable::drive::param_1` is `(none)` in both entries.

Implication:
- Chain breaks between container element abstraction and caller local reference variables.
