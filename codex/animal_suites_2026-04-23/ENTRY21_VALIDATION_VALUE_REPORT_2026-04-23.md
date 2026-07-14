# Entry Validation and Value Report

- Entry: `prop_mixin_methods_callable`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_methods_callable_entry21_rerun`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 验证 mixin 方法（`Feathered/Flyable/Scaled/Swimmable`）在多种具体类上的可调用性与类型传播。
- User/business impact if not fixed:
  - 若 mixin 调用链传播错误，会直接影响 mixin 场景下的精度与可解释性。

## 2) Scope and Change Type

- Affected scenario(s):
  - `Eagle/Penguin` 的 `preen_feathers`
  - `Shark/Salmon` 的 `shed_scales`
  - `Eagle/Duck/FlyingFish` 的 `fly`
  - `Penguin/Duck/Shark/Salmon/FlyingFish` 的 `swim`
- Source changed: **No**
- Files changed:
  - `(none)`
- Why this change is needed:
  - 本 entry 主要用于 mixin 调用路径验证与剩余噪声识别。

## 3) Expected Semantics

1. 测试中实际调用到的 mixin 形参应为非空并匹配对应对象类型。
2. 各 mixin 方法类型范围应与具体类语义一致，不出现跨类污染。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - `alloc: 12`, `call_arg: 25`，调用路径覆盖测试主流程。
- `class_pts`:
  - 主调用路径非空且语义正确（如 `mixinsFlyable::fly::param_1 -> obj_4,obj_5,obj_6`）。
  - 但存在 5 个 `(none)`：均为 `...::::...` 组合 wrapper 形参（未被测试直接调用的层）。
- `type-info`:
  - 主调用路径类型与预期一致（如 `mixinsSwimmable::swim::param_1 -> Duck,FlyingFish,Penguin,Salmon,Shark`）。
  - 同样存在 5 个 `(none)`，与 `class_pts` 对应。
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
- `(none)` count in key outputs (`class_pts + type-info`): `10`（`5 + 5`）
- Empty points-to pointers: `5`（`29-24`）
- Type-complete pointers: `24/29`（`82.8%`）

## 6) Validation Verdict

- Status: **Partial Pass**
- Confidence: **High**
- Regression check result:
  - 主调用语义无回归；残留空洞集中在未直接调用的组合 wrapper 层。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_methods_callable_entry21_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_methods_callable_entry21_rerun/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_mixin_methods_callable_entry21_rerun/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 主路径（测试实际调用的 mixin）精度可用，未出现对象/类型传播错误。
- Measurable benefit:
  - 主路径非空，但全量指标受 5 个内部 wrapper 空洞影响（完整率 82.8%）。
- Practical value:
  - 明确定位了剩余噪声区：`...::::...` 组合 wrapper 层，可作为后续定向去噪优化点。

## 8) Next Action

- Next candidate entry:
  - `prop_multiple_mixin_method_independence`
- Need follow-up source changes:
  - `Yes`（若目标是把全量空洞清零，建议下一轮对内部 wrapper 层做去噪/过滤策略）
