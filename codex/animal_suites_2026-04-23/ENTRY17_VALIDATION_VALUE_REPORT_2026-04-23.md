# Entry Validation and Value Report

- Entry: `test_polymorphic_method_calls`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_method_calls_entry17_rerun`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 验证不同类别对象在直接调用与多态调用中的方法分派是否被 rcpta 正确跟踪。
- User/business impact if not fixed:
  - 若分派链不准，会导致类型推断偏差，影响后续安全判断与问题定位可信度。

## 2) Scope and Change Type

- Affected scenario(s):
  - `Dog` 直接调用与 `Animal` 上转后调用
  - `Eagle` 的 `Bird`/`Animal` 多态调用
  - `Shark` 的 `Fish`/`Animal` 多态调用
- Source changed: **No**
- Files changed:
  - `(none)`
- Why this change is needed:
  - 本 entry 用于确认 Entry #16 优化后没有引入新的多态调用回归。

## 3) Expected Semantics

1. `dogDog::*::param_1` 仅指向 `Dog`，`birdBird::*::param_1` 仅指向 `Eagle`，`fishFish::*::param_1` 仅指向 `Shark`。
2. `animalAnimal::*::param_1` 应聚合 `Dog/Eagle/Shark` 三类对象。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - `CallArg` 链完整覆盖 direct + polymorphic 调用路径（35 条 call_arg）。
  - `assign/cast/alloc` 与源码场景一致（`assign: 2`, `cast: 5`, `alloc: 3`）。
- `class_pts`:
  - 全部指针非空（`ptrs_with_objs: 24/24`）。
  - `dog/bird/fish` 对应形参分别映射单对象（`obj_0/obj_1/obj_2`），`animal` 聚合三对象。
- `type-info`:
  - 全部指针有类型（`ptrs_with_types: 24/24`）。
  - `animalAnimal::*::param_1 -> Dog,Eagle,Shark`，其余形参类型精确且无污染。
- `cast_safety.log`:
  - 5 个 cast 点全部 `cast is safe`。

## 5) Precision Metrics (Required)

Use FP/FN if available. If not, fill proxy metrics and mark as proxy.

- FP (false positives): `N/A` (proxy-only in this phase)
- FN (false negatives): `N/A` (proxy-only in this phase)
- Precision (%): `N/A`
- Recall (%): `N/A`
- F1 (%): `N/A`

Proxy metrics (when FP/FN are not directly measurable):
- `(none)` count in key outputs (`class_pts + type-info`): `0`（当前结果）
- Empty points-to pointers: `0`（`24-24`）
- Type-complete pointers: `24/24`（`100%`）

## 6) Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Regression check result:
  - 未观察到 Entry #16 优化导致的回归。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_method_calls_entry17_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_method_calls_entry17_rerun/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_method_calls_entry17_rerun/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 验证通过，说明当前 rcpta 在多态方法调用链路上保持了稳定精度。
- Measurable benefit:
  - 关键输出无空洞，类型完整率维持 100%。
- Practical value:
  - 支撑后续在更复杂 entry 上继续迭代，而无需回退本轮优化。

## 8) Next Action

- Next candidate entry:
  - `test_complex_polymorphic_scenarios`
- Need follow-up source changes:
  - `No`（先继续下一 entry 验证）
