# Entry Validation and Value Report

- Entry: `prop_polymorphic_method_call_correctness`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_polymorphic_method_call_correctness_entry18_rerun`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 验证跨多种动物类型的多态方法调用是否被 rcpta 持续、准确跟踪。
- User/business impact if not fixed:
  - 若该场景精度不足，会在核心多态调用链产生错误空洞，影响整体分析可信度。

## 2) Scope and Change Type

- Affected scenario(s):
  - `Animal` 基类上的统一多态调用
  - `Bird/Fish` 分类上的多态调用与具体子类调用交叉场景
- Source changed: **No**
- Files changed:
  - `(none)`
- Why this change is needed:
  - 该 entry 用于确认前序优化（特别是迭代/调用传播）在大规模多态调用下保持稳定。

## 3) Expected Semantics

1. `animalAnimal::{make_sound,move_action,describe}::param_1` 应覆盖 9 个具体动物对象。
2. `birdBird::*::param_1` 与 `fishFish::*::param_1` 应保持分类语义，不应出现空集。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - 图结构完整：`assign: 7`, `cast: 16`, `alloc: 9`, `call_arg: 75`。
  - 未见关键调用链断裂。
- `class_pts`:
  - `ptrs_with_objs: 56/56`，无空 points-to。
  - `animalAnimal::*::param_1` 覆盖 `obj_0..obj_8`（9 对象全覆盖）。
  - `birdBird`、`fishFish` 形参均为非空且对象范围合理。
- `type-info`:
  - `ptrs_with_types: 56/56`，无 `(none)`。
  - `animalAnimal::*::param_1 -> Cat,Dog,Duck,Eagle,FlyingFish,Ostrich,Penguin,Salmon,Shark`。
  - 分类/具体类型对应关系与源码语义一致。
- `cast_safety.log`:
  - 本 entry 16 个 cast 点均为 `cast is safe`。

## 5) Precision Metrics (Required)

Use FP/FN if available. If not, fill proxy metrics and mark as proxy.

- FP (false positives): `N/A` (proxy-only in this phase)
- FN (false negatives): `N/A` (proxy-only in this phase)
- Precision (%): `N/A`
- Recall (%): `N/A`
- F1 (%): `N/A`

Proxy metrics (when FP/FN are not directly measurable):
- `(none)` count in key outputs (`class_pts + type-info`): `0`
- Empty points-to pointers: `0`（`56-56`）
- Type-complete pointers: `56/56`（`100%`）

## 6) Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Regression check result:
  - 未观察到回归；多态调用路径保持稳定精度。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_polymorphic_method_call_correctness_entry18_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_polymorphic_method_call_correctness_entry18_rerun/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_polymorphic_method_call_correctness_entry18_rerun/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 在高覆盖多态调用场景中，rcpta 维持了完整的对象传播与类型推断。
- Measurable benefit:
  - 关键输出空洞为 0，类型完整率维持 100%。
- Practical value:
  - 证明当前优化方案可支撑更复杂 entry，而不引入调用传播退化。

## 8) Next Action

- Next candidate entry:
  - `prop_type_distinctiveness`
- Need follow-up source changes:
  - `No`（先继续下一 entry 验证）
