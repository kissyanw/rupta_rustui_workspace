# Entry Validation and Value Report

- Suite: `shape_hierarchy`
- Entry: `test_polymorphism_with_collection`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/shape_hierarchy/test_polymorphism_with_collection_entry2_rerun`

## Source Changes in this Entry

- Source changed: **No**
- Files changed: `(none)`

## Observed Result

- `class_pag`:
  - collection iterator element ptr `local_78.as_variant#1.0.1` 正常建图
  - `CallArg` 到 `shapeShape::{area,perimeter,description}::param_1` 完整
- `class_pts`:
  - `shapeShape::{area,perimeter,description}::param_1` 均覆盖 7 个对象
  - `ptrs_with_objs = 29/29`
- `type-info`:
  - 3 个多态形参均推断 7 类型联合
  - `ptrs_with_types = 29/29`
- `(none)` in `class_pts + type-info`: `0`

## Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Notes:
  - 多态集合场景下的循环元素传播与形参传播均正确，无新增精度缺陷。
