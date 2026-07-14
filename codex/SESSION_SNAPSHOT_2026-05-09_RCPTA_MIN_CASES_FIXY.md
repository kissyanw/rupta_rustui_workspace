# SESSION SNAPSHOT - 2026-05-09 - RCPTA MIN CASES (FIXY)

## 1) 目标与当前状态
- 目标：先在 `rcpta_min_cases` 两个小案例上打通容器语义，再回到 entry5。
- 当前状态：**编译错误已修复**，但 `min_index` 输出仍不正常（调用侧结构边已建，PTS 仍空）。

## 2) 本轮关键结论
- `&drivables[0]` 的 MIR 真实形态已确认：
  - `_3 = &_1`
  - `_2 = <Vec<_> as Index<usize>>::index(move _3, const 0)`
- `resolve_call` 确实命中 `build_drivables` 和 `std::ops::Index::index`。
- `class_pag` 里调用侧链路已存在：
  - `build_drivables::ret.deref.index -> test...::local_1.$elem`
  - `build_drivables::ret.deref.index -> test...::local_2`
  - `test...::local_2 -> interfacesDrivable::drive::param_1`
- 但 `class_pts` 仍空的最早断点是：
  - `build_drivables::ret.deref.index -> (none)`
  - 上游 `slice::into_vec<...>::ret.deref.index -> (none)`
  - 同时 `build_drivables::local_45.deref.index -> obj_0..obj_3` 是非空。

## 3) 关键定位
- 断链不是调用侧读取（index/read）问题。
- 断链是 **build_drivables 内部“容器构造元素 -> 返回容器元素”** 未传通。
- 目前图上有从 `slice::into_vec::ret.*` 到 `build_drivables::ret.*` 的边，但 `slice::into_vec::ret.*` 本身空。

## 4) 已实施的语义扩展（本轮）
- 引入统一容器元素槽位命名（builder 内）：`$elem`
  - `base_ptr -> base_ptr.$elem`
  - `func::ret.$elem`
- return/bind/read(index) 已接入该语义：
  - return bind: callee ret.$elem/ret.deref.index -> caller base.$elem
  - index read: container base 映射到 dst ref
- 相关主要修改文件：
  - `rupta/src/builder/fpag_builder.rs`
  - `rupta/src/mir/analysis_context.rs`

## 5) 编译错误修复
- 修复了 `class_name` move 后复用导致的 `E0382`。
- `cargo build --bin pta` 已通过。

## 6) 最新验证产物（重点）
- 产物目录：
  - `analysis_results/rcpta/vehicle_hierarchy/min_index_drive_target_fixY_2026-05-09/`
- 关键文件：
  - `class_pag.txt`
  - `class_pts.txt`
  - `analysis.log`
- 关键观测：
  - `class_pag` 有结构边；
  - `class_pts` 中 `ret.*` / `local_2` / `drive::param_1` 全空。

## 7) 下一轮优先修复建议
优先做最小、可控、可验证的一条：
1. 在 `build_drivables` 返回路径处，按真实返回源把 `local_45.deref.index` 直接并入 `ret.$elem`/`ret.deref.index`（不依赖 `slice::into_vec` 内部 ret 建模）。
2. 若需要，可新增一个小型语义操作 `ContainerConstructToReturn`，限制触发条件：
   - 当前函数返回 `Vec<DSL>`
   - 函数内存在已填充的 `*.deref.index` summary
3. 先只在 min cases 验证：
   - `build_drivables::ret.deref.index` 非空
   - `test...::local_2` 非空
   - `interfacesDrivable::drive::param_1` 非空

## 8) 当前代码变更概览
- 已修改：
  - `rupta/src/builder/fpag_builder.rs`
  - `rupta/src/mir/analysis_context.rs`
- 当前工作树还有历史变更（非本轮新增）存在，请续修时注意只聚焦上述文件逻辑。

## 9) 用户授权（必须继承到下一轮）
- 用户已明确授权：
  - 允许扩展抽象域（不仅限于现有抽象）。
  - 允许扩展语义操作建模（不仅限于现有操作）。
- 续修原则：
  1. 不要被“仅在现有抽象内修补”束缚；当语义表达力不足时，应主动提出并实施抽象扩展。
  2. 扩展前先做简短提案（新增概念、语义操作、潜在精度/复杂度影响）。
  3. 先在 `rcpta_min_cases` 验证扩展有效，再带回 `entry5`。
  4. 优先追求可复用、通用语义，不做 entry 特判式补丁。
