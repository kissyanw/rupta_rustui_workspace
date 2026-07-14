# Entry Validation and Value Report

- Suite: `vehicle_hierarchy`
- Entry: `test_drivable_interface_polymorphism`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/vehicle_hierarchy/test_drivable_interface_polymorphism_entry1_rerun`

## Source Changes in this Entry

- Source changed: **No**
- Files changed: `(none)`

## Observed Result

- `class_pag`:
  - 车辆集合迭代元素接收者 `local_58.as_variant#1.0.1` 建图完整。
  - 到 `interfacesDrivable::{drive,stop,turn}::param_1` 的 `CallArg` 边完整。
- `class_pts`:
  - `interfacesDrivable::{drive,stop,turn}::param_1 -> obj_0,obj_1,obj_2,obj_3`（Car/Motorcycle/Bicycle/Truck）
  - `car/motorcycle/bicycle/truck::drive::param_1` 均为对应单对象，语义正确。
  - 4 个 local 临时为空：`local_138/154/170/186`（索引取值中间变量）。
- `type-info`:
  - 接口形参类型范围为 `Bicycle,Car,Motorcycle,Truck`，与语义一致。
  - 与上述 4 个 local 对应地出现 4 个 `(none)`。
- `cast_safety.log`:
  - 4 个 cast 点均 `safe`。

## Validation Verdict

- Status: **Pass**
- Confidence: **Medium-High**
- Notes:
  - 主调用链与接口多态语义正确。
  - 存在“中间临时 local 空洞”噪声，后续可在高价值 entry 中视情况收敛。
