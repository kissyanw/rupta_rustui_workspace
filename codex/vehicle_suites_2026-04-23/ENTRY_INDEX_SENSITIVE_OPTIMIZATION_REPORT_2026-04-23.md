# Entry Impact Report: Index Access Modeling (element-sensitive, index-insensitive)

- Date: `2026-04-23`
- Scope: `vehicle_hierarchy` suite（基于已完成的 Entry #1 / #2 复测）
- rcpta config: `--analyze-only --context-depth 1`
- Source change:
  - `/home/wy/rupta_rustdsl_workspace/rupta/src/builder/fpag_builder.rs`
  - 新增 `Index::index` 调用建模：对 `v[idx]` 的返回值指针执行“元素敏感、索引不敏感”传播

## Change Summary

- 触发条件：
  - callee def-path 匹配 `::ops::index::Index...::index`
  - 且处于 source-level context
- 建模策略：
  - 从 `index` 返回值推导元素 DSL 类类型（优先 `destination` 的 `&T -> T`，回退到函数返回类型）
  - 为 `destination` 创建/确保 ClassPtr
  - 在同一函数作用域内筛选与元素类型兼容的 class pointers（含子类型兼容）
  - 添加 `Assign(src -> destination)`，不区分具体索引值（index-insensitive）

## Before vs After (Entry #1)

- Entry: `test_drivable_interface_polymorphism`
- Before output:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/vehicle_hierarchy/test_drivable_interface_polymorphism_entry1_rerun`
- After output:
  - `/home/wy/rupta_rustdsl_workspace/test_drivable_interface_polymorphism_entry1_index_sensitive_v2`

- 关键变化（之前为空的 index 结果指针）：
  - `tests::test_drivable_interface_polymorphism#1::local_138`
  - `tests::test_drivable_interface_polymorphism#1::local_154`
  - `tests::test_drivable_interface_polymorphism#1::local_170`
  - `tests::test_drivable_interface_polymorphism#1::local_186`

- 修复前：
  - `class_pts`: 上述 4 个均为 `(none)`
  - `type-info`: 上述 4 个均为 `(none)`
- 修复后：
  - `class_pts`: 上述 4 个均为 `obj_0,obj_1,obj_2,obj_3`
  - `type-info`: 上述 4 个均为 `Bicycle, Car, Motorcycle, Truck`

## Before vs After (Entry #2)

- Entry: `test_maintainable_interface_polymorphism`
- Before output:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/vehicle_hierarchy/test_maintainable_interface_polymorphism_entry2_rerun`
- After output:
  - `/home/wy/rupta_rustdsl_workspace/test_maintainable_interface_polymorphism_entry2_index_sensitive_v2`

- 关键变化（之前为空的 index 结果指针）：
  - `tests::test_maintainable_interface_polymorphism#1::local_101`
  - `tests::test_maintainable_interface_polymorphism#1::local_117`
  - `tests::test_maintainable_interface_polymorphism#1::local_133`
  - `tests::test_maintainable_interface_polymorphism#1::local_149`
  - `tests::test_maintainable_interface_polymorphism#1::local_225`
  - `tests::test_maintainable_interface_polymorphism#1::local_241`
  - `tests::test_maintainable_interface_polymorphism#1::local_257`
  - `tests::test_maintainable_interface_polymorphism#1::local_273`

- 修复前：
  - `class_pts`: 上述 8 个均为 `(none)`
  - `type-info`: 上述 8 个均为 `(none)`
- 修复后：
  - `class_pts`: 上述 8 个均为 `obj_0,obj_1,obj_2,obj_3`
  - `type-info`: 上述 8 个均为 `Bicycle, Car, Motorcycle, Truck`

## Quantitative Effect

- Entry #1:
  - `class_pts` `(none)` 数量：`4 -> 0`（减少 `100%`）
  - `type-info` `(none)` 数量：`4 -> 0`（减少 `100%`）
- Entry #2:
  - `class_pts` `(none)` 数量：`8 -> 0`（减少 `100%`）
  - `type-info` `(none)` 数量：`8 -> 0`（减少 `100%`）
- Combined (Entry #1 + #2):
  - `class_pts` `(none)`：`12 -> 0`
  - `type-info` `(none)`：`12 -> 0`

## Interpretation

- 该修复直接命中 vehicle 套件中 `Vec` 下标访问链路导致的 “actual 非空但局部 index 结果指针空” 问题。
- 修复后，`Index::index` 返回值可正确承接容器元素候选对象，随后流向 interface formal 的 CallArg 语义更一致。
- 该策略满足“element-sensitive but index-insensitive”：保留元素类型敏感性，不引入具体索引值追踪。
