# SESSION SNAPSHOT — 2026-05-10 (cast risk detection / inheritance parsing)

## 1) 本轮目标与范围
- 从 vehicle 精度问题切换到 `cast risk detection` 优化。
- 聚焦四项：
  - 指针分析结果可用性（作为判定输入）
  - 类型推断质量（type-info）
  - 类继承关系解析（inheritance graph）
  - cast 安全判定逻辑（safe / unsafe + 原因）
- 约束：
  - **不改流不敏感框架**
  - **暂不处理上游大范围建图精度问题**

## 2) 已完成的关键修复

### 2.1 cast_safety source-level 去噪
- 文件：`rupta/src/util/class/cast_safety_log.rs`
- 改动：过滤 DSL 内部位置（如 `rustdsl/classes/src/macros/...`）的 cast site。
- 效果：`cast_safety.log` 只保留 source-level（tests）cast，避免噪声误导验证。

### 2.2 unsafe 原因细分
- 文件：`rupta/src/util/class/cast_safety_log.rs`
- 新增分类：
  - `unsafe_kind: must-unsafe`
  - `unsafe_kind: may-unsafe`
  - `unsafe_kind: boundary-unknown-src`
  - `unsafe_kind: boundary-missing-dst-type`
- 增强诊断：输出 `satisfied_types` / `unsatisfied_types`。

### 2.3 Cast 边传播改为“类型过滤传播”
- 文件：`rupta/src/rcpta/class_pts.rs`
- 原逻辑：`Cast` 与 `Assign` 等价传播。
- 新逻辑：`Cast` 对 `PTS(src)` 先按目标类型兼容规则过滤，再传播到 `dst`：
  - class 目标：`extends*`
  - interface 目标：`implements*`
  - mixin 目标：`with*`

### 2.4 修复 pre_cast_pts 方案缺陷（核心）
- 问题：全局 `pre_cast_pts`（完全禁 cast）会导致过度保守，丢失“历史 cast 链路”信息。
- 最终方案：
  - 删除全局禁-cast并行解。
  - 在 `solve_class_pts` 中按 cast 边记录“cast 前 src 快照”：
    - `cast_src_before_pts: HashMap<(src_ptr_id, dst_ptr_id), HashSet<obj_id>>`
  - `cast_safety` 对每个 cast site 优先使用该 `(src,dst)` 快照做判定。
  - 缺失时 fallback 到 `pts[src]`。
- 意义：既避免“本次 cast 自污染”，又保留此前传播信息。

### 2.5 继承图解析增强
- 文件：`rupta/src/util/class/dsl_inheritance_graph.rs`
- 原方式：全文件多组正则 + 短 lookahead（脆弱）。
- 新方式：声明驱动解析：
  - 统一锚定 `pub [abstract] class ... {` / `pub mixin ... {`
  - 解析头部子句 `extends / implements / with / on`（支持多行）
  - 改进 `abstract class` 的 class/interface 判别逻辑
- 不改变闭包规则定义（`extends* / implements* / with*`）。

## 3) 已确认的方法论共识
- `safe` 可解释为 `must-safe`。
- `unsafe` 覆盖 `may-unsafe` / `must-unsafe` / 边界不确定。
- cast 判定与 cast 传播必须分离：
  - 判定看 cast 前 src 证据
  - 传播对 dst 应用过滤

## 4) 编译状态
- 多轮改动后均通过：
  - `cargo build --bin pta` 成功。

## 5) 批量验证执行（animal suites 前 10 entries）

### 5.1 输出目录
- `/home/wy/rupta_rustdsl_workspace/analysis_results/2026_5_10/animal_hierarchy/`

### 5.2 已执行 entries（10个）
1. `prop_multilevel_upcast_preserves_identity`
2. `test_downcast_animal_to_dog_success`
3. `test_downcast_animal_to_eagle_through_bird`
4. `test_downcast_animal_to_shark_through_fish`
5. `test_downcast_animal_dog_to_cat_failure`
6. `test_downcast_animal_eagle_to_penguin_failure`
7. `test_downcast_bird_eagle_to_duck_failure`
8. `test_downcast_fish_shark_to_salmon_failure`
9. `test_downcast_does_not_panic`
10. `prop_downcast_type_safety`

### 5.3 结果摘要（cast_safety）
- `prop_multilevel_upcast_preserves_identity`: `safe=5, unsafe=0`
- `test_downcast_animal_to_dog_success`: `safe=2, unsafe=0`
- `test_downcast_animal_to_eagle_through_bird`: `safe=4, unsafe=0`
- `test_downcast_animal_to_shark_through_fish`: `safe=4, unsafe=0`
- `test_downcast_animal_dog_to_cat_failure`: `safe=1, unsafe=1`
- `test_downcast_animal_eagle_to_penguin_failure`: `safe=2, unsafe=1`
- `test_downcast_bird_eagle_to_duck_failure`: `safe=1, unsafe=1`
- `test_downcast_fish_shark_to_salmon_failure`: `safe=1, unsafe=1`
- `test_downcast_does_not_panic`: `safe=5, unsafe=5`
- `prop_downcast_type_safety`: `safe=16, unsafe=8`

### 5.4 验证结论
- 指针分析/类型推断：关键输出文件均生成，且场景级结论与期望一致。
- 继承图：关键闭包关系（如 `Dog->Animal`、`Eagle->Bird->Animal`、`Shark->Fish->Animal`、`with*` 关系）已命中。
- cast risk detection：成功/失败场景的 safe/unsafe 分布符合测试语义。

## 6) 当前状态
- 在既定约束（不改流不敏感、暂不处理上游大问题）下，本轮目标已阶段性达成。
- 当前 cast 判定链路较此前更“语义一致 + 可解释 + 可审计”。
