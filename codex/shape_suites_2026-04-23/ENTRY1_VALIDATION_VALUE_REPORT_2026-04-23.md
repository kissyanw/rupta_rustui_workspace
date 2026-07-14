# Entry Validation and Value Report

- Suite: `shape_hierarchy`
- Entry: `test_create_all_shapes`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/shape_hierarchy/test_create_all_shapes_entry1_rerun`

## Source Changes in this Entry

- Source changed: **No**
- Files changed: `(none)`

## Observed Result

- `class_pag`: only alloc edges, no call/cast path in this entry.
- `class_pts`: `ptrs_with_objs = 7/7`
- `type-info`: `ptrs_with_types = 7/7`
- `(none)` in `class_pts + type-info`: `0`
- `cast_safety.log`: empty (no cast checks in this entry)

## Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Notes:
  - Result matches entry语义：仅创建各类 shape 实例并验证基础属性。
