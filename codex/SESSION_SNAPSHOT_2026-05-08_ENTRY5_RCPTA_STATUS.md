# SESSION SNAPSHOT — 2026-05-08 (entry#5 / rcpta)

## 1) 本轮任务目标
- 继续修复 vehicle suites 中 `prop_drivable_interface_polymorphism`（entry#5）分析时的栈溢出与结果失真。
- 目标不是 entry5 特判，而是做可泛化的 rcpta 修复：
  - 保持 proptest 入口可分析（不栈溢出）。
  - rcpta 输出（`class_cg / class_pag / class_pts / type-info`）应接近普通 test entry 的语义质量。

## 2) 已确认的关键事实
- 普通入口 `test_drivable_interface_polymorphism`（baseline）可稳定得到正确类型/对象流：
  - `interfacesDrivable::{drive,stop,turn}::param_1` 能拿到 `Car/Motorcycle/Bicycle/Truck`。
- proptest 入口 `prop_drivable_interface_polymorphism` 早期反复栈溢出，后续已通过入口下钻+工具链路屏蔽稳定跑通。
- 当前剩余核心问题：
  - entry#5 虽不再溢出，但 `interfacesDrivable::param_1` 在 `class_pts/type-info` 仍为 `(none)`。

## 3) 本轮已落地代码改动（当前工作树状态）
涉及文件：
- `rupta/src/mir/analysis_context.rs`
- `rupta/src/pta/context_sensitive.rs`
- `rupta/src/graph/pag.rs`
- `rupta/src/builder/fpag_builder.rs`

### 3.1 proptest 入口可分析链路（已生效）
- 在 `context_sensitive` 中加入 proptest entry 下钻与函数筛选：
  - 针对 `property_tests::prop_*` 入口补种业务 closure（`{closure#1}`）作为分析起点。
  - 对明显工具链/噪声 callee（`proptest::`, `regex_syntax::`, `rusty_fork::`, `tempfile::`）做阻断。
- 在 `pag::build_all_callee_pags` 阶段同步做 prop-mode callee 屏蔽，避免 stage3 再次爆炸。
- 结果：entry#5 能稳定完成 stage1~4，不再栈溢出。

### 3.2 ClassPAG 对象流补强尝试（仍未完全闭环）
- 在 `fpag_builder::visit_copy_or_move` 增加 source-level 类语义 copy/move 的 `ClassPAG Assign` 补边（泛化，不限 entry5）。
- 扩展 `Iterator::next` 的类对象建模：
  - 支持 `Option<(idx, Item)>` 与 `Option<Item>` 两种返回形态。
  - 改为使用 caller 侧 destination type（单态化）而不是 callee 泛型返回类型识别 item class。
- 当前效果：
  - 对象已能汇聚到 `local_324.deref.index`（可见四种 vehicle 对象）。
  - 但未成功桥接到 `local_56/local_161/...`，因此 `Drivable::param_1` 仍空。

## 4) 当前根因判断（最重要）
`Drivable::param_1` 为空不是因为 `CallArg` 缺失。

证据链：
- `class_pag.txt` 中存在 call_arg：
  - `local_56 -> interfacesDrivable::drive::param_1`
  - `local_161/164/167/170 -> interfacesDrivable::drive::param_1`
- 但 `class_pts.txt` 里这些 `local_*` 本身是 `(none)`。

因此真正断点在：
- 「容器/迭代器返回值（如 `Iterator::next` / `Index::index` 产出的 `Option<&Drivable>` / `&Drivable`）
  到 caller 本地变量」这段 ClassPAG 桥接不完整。

## 5) 栈溢出问题阶段性状态
- 状态：已阶段性解决（entry#5 可稳定跑完，无 stack overflow）。
- 代价：对 proptest 工具链 callee 进行了屏蔽；这属于通用降噪策略，不是 entry5 名称硬编码特判。

## 6) 结果目录清理（本轮已执行）
已在 `analysis_results/rcpta/vehicle_hierarchy` 清理旧的 entry5 试验目录，保留关键目录：
- 保留：
  - `prop_drivable_interface_polymorphism_entry5_drill_v10_iter_next_dst_ty_2026-05-08`（最新）
  - `test_drivable_interface_polymorphism_baseline_2026-05-07`
  - `test_drivable_interface_polymorphism_baseline_2026-05-08_regress_after_entry5_fix2`
  - 其他非 entry5 的 suites 目录
- 删除：v1~v9 等过时 entry5 调试目录与中间产物。

## 7) 新会话建议的直接执行路线
1. 先不改更多策略，精准补齐“返回值桥接”语义：
   - 在 `resolve_call` 路径为 `Iterator::next` / `Index::index` 等返回 `&ClassLike`、`Option<&ClassLike>` 的调用建立稳定 `ret -> dst` / `inner -> dst` ClassPAG 边。
2. 用最小验证集回归：
   - `test_drivable_interface_polymorphism`（baseline 不退化）
   - `prop_drivable_interface_polymorphism`（entry#5 不溢出且 `Drivable::param_1` 非空）
3. 通过后再批量跑 vehicle suites 所有 test entry func。

## 8) 你关心的最终验收标准（明确）
- entry#5：
  - 不栈溢出。
  - `interfacesDrivable::param_1` 在 `class_pts/type-info` 中恢复为四种 vehicle 对象类型（或等价语义集合）。
- 其他 vehicle test entries：
  - 结果不退化，`class_cg/class_pag/class_pts/type-info` 与程序语义一致。
