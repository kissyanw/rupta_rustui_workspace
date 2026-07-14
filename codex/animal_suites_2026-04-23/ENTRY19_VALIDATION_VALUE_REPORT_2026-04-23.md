# Entry Validation and Value Report

- Entry: `prop_type_distinctiveness`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_type_distinctiveness_entry19_rerun`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 验证不同动物类型在多态调用中是否能保持“类型可区分性”而不被混淆。
- User/business impact if not fixed:
  - 若类型区分丢失，会导致精度下降并增加后续分析误判风险。

## 2) Scope and Change Type

- Affected scenario(s):
  - 9 种动物对象在 `Animal/Bird/Fish` 多态调用中的类型区分
  - 多条 `make_sound/move_action` 调用链的独立性
- Source changed: **No**
- Files changed:
  - `(none)`
- Why this change is needed:
  - 该 entry 是精度“区分能力”验证点，用于确认当前 rcpta 不发生类型塌缩。

## 3) Expected Semantics

1. `animalAnimal::*::param_1` 应呈现 9 种具体类型联合。
2. `birdBird::*::param_1` 与 `fishFish::*::param_1` 应保持分类范围，不出现空集或不相关污染。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - `alloc: 9`, `call_arg: 30`，路径覆盖完整。
  - 无异常断链迹象。
- `class_pts`:
  - `ptrs_with_objs: 23/23`，无空 points-to。
  - `animalAnimal::*::param_1` 覆盖 `obj_0..obj_8`（9 对象）。
  - `birdBird`、`fishFish` 形参集合与分类语义一致。
- `type-info`:
  - `ptrs_with_types: 23/23`，无 `(none)`。
  - `animalAnimal` 形参为 9 类型联合；
  - 各 local 与对应具体类型一一匹配（如 `local_4 -> Dog`, `local_11 -> Cat`, `local_52 -> FlyingFish`）。
- `cast_safety.log`:
  - 文件为空（本 entry 无 cast 检查点），符合场景特征。

## 5) Precision Metrics (Required)

Use FP/FN if available. If not, fill proxy metrics and mark as proxy.

- FP (false positives): `N/A` (proxy-only in this phase)
- FN (false negatives): `N/A` (proxy-only in this phase)
- Precision (%): `N/A`
- Recall (%): `N/A`
- F1 (%): `N/A`

Proxy metrics (when FP/FN are not directly measurable):
- `(none)` count in key outputs (`class_pts + type-info`): `0`
- Empty points-to pointers: `0`（`23-23`）
- Type-complete pointers: `23/23`（`100%`）

## 6) Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Regression check result:
  - 未观察到回归；类型区分能力保持稳定。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_type_distinctiveness_entry19_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_type_distinctiveness_entry19_rerun/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_type_distinctiveness_entry19_rerun/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 当前 rcpta 在类型区分场景下保持了精确且稳定的传播结果。
- Measurable benefit:
  - 空洞指标维持 0，类型完整率维持 100%。
- Practical value:
  - 证明当前方案在“高区分度”场景可用，为后续 suites 汇总指标提供稳定基线。

## 8) Next Action

- Next candidate entry:
  - `test_conversion_chain_object_identity`
- Need follow-up source changes:
  - `No`（先继续下一 entry 验证）
