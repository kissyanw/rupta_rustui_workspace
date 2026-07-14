# Changelog

## 2026-05-08
- Initialized tracking directory and baseline/design docs.
- Added semantic bridge for Vec-return summary in `fpag_builder.rs`:
  - callee side: `func::ret.elem` gets flow from local `*.deref.index` summaries.
  - caller side: `callee::ret.elem -> caller::dst.deref.index`.
- Validation run (fixA):
  - `test_rcpta_min_index_drive`: still `param_1` empty.
  - `test_rcpta_min_iter_next_unwrap_drive`: needs isolated rerun due to parallel dump-path interference.
- Validation rerun (fixA2):
  - `test_rcpta_min_iter_next_unwrap_drive`: still empty (`local_5/local_6`, `interfacesDrivable::drive::param_1`).
  - `test_rcpta_min_index_drive`: still empty (`local_2`, `interfacesDrivable::drive::param_1`).
- Result: Vec-return summary bridge alone is insufficient; next step is to add callsite-local semantic operation that maps caller Vec local summary directly to returned `&Drivable` locals for `index` and `next+unwrap` patterns.

## 2026-05-08 (fixD/fixF) - Semantic op implementation status

- Implemented/adjusted in `rupta/src/builder/fpag_builder.rs`:
  - `IterNextUnwrapRefBridge` fallback path no longer hard-requires destination DSL type extraction.
  - `IndexRefBridge` fallback now can reuse destination pointer class type when direct type extraction fails.
  - `is_iter_next` / `is_index_index` recognizer switched to method-name-based matching from `def_path` tail token.
- Validation runs:
  - `min_index_drive_target_fixD_2026-05-08`
  - `min_iter_next_unwrap_drive_target_fixD_2026-05-08`
  - `min_index_drive_target_fixF_2026-05-08`
- Current result: both min cases still unresolved.
  - `build_drivables::local_45.deref.index` remains non-empty.
  - caller ref locals (`local_2` / `local_5` / `local_6`) remain `(none)`.
  - `interfacesDrivable::drive::param_1` remains `(none)`.
- Observation: new semantic bridge edges are not materialized in `class_pag.txt` yet, indicating branch trigger / edge injection is still not effective in current call modeling path.
