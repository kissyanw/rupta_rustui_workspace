# Entry Validation and Value Report

- Suite: `vehicle_hierarchy`
- Entry: `test_maintainable_interface_polymorphism`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/vehicle_hierarchy/test_maintainable_interface_polymorphism_entry2_rerun`

## Source Changes in this Entry

- Source changed: **No**
- Files changed: `(none)`

## Observed Result

- `class_pag`:
  - 集合迭代元素接收者 `local_58.as_variant#1.0.1` 建图完整。
  - 到 `interfacesMaintainable::{perform_maintenance,check_condition}::param_1` 的 `CallArg` 边完整。
- `class_pts`:
  - 接口形参均覆盖 4 对象：`obj_0,obj_1,obj_2,obj_3`。
  - 具体类 `car/motorcycle/bicycle/truck` 对应形参均为单对象，语义正确。
  - 8 个 local 临时为空：`local_101/117/133/149/225/241/257/273`。
- `type-info`:
  - 接口形参类型范围正确：`Bicycle, Car, Motorcycle, Truck`。
  - 与上述 8 个 local 对应出现 8 个 `(none)`。
- `cast_safety.log`:
  - 4 个 cast 点均 `safe`。

## Validation Verdict

- Status: **Pass**
- Confidence: **Medium-High**
- Notes:
  - 主调用链与接口多态语义正确。
  - 仍有“中间临时 local 空洞”噪声，模式与 Entry #1 一致。
