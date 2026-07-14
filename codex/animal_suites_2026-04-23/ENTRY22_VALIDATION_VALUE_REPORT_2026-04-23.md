# Entry Validation and Value Report

- Entry: `prop_multiple_mixin_method_independence`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multiple_mixin_method_independence_entry22_rerun`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 验证多 mixin 方法（`fly`/`swim`）重复调用时相互独立、不互相污染。
- User/business impact if not fixed:
  - 若 mixin 调用链互相干扰，会造成类型混淆和行为解释错误。

## 2) Scope and Change Type

- Affected scenario(s):
  - `Duck` 的 `Flyable + Swimmable` 多次组合调用
  - `FlyingFish` 的 `Flyable + Swimmable` 多次组合调用
- Source changed: **No**
- Files changed:
  - `(none)`
- Why this change is needed:
  - 本 entry 用于检验 mixin 多方法调用独立性和对象传播稳定性。

## 3) Expected Semantics

1. `Bird_Flyable/Bird_Flyable_Swimmable` 形参应只接收 `Duck`。
2. `Fish_Flyable/Fish_Flyable_Swimmable` 形参应只接收 `FlyingFish`。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - `alloc: 4`, `call_arg: 21`，重复调用路径完整。
  - 主调用边都指向具体 wrapper 形参（`Bird_Flyable`, `Bird_Flyable_Swimmable`, `Fish_Flyable`, `Fish_Flyable_Swimmable`）。
- `class_pts`:
  - 主路径非空且分离：
    - `mixinsFlyable::::Bird_Flyable::fly::param_1 -> obj_0,obj_2`（Duck）
    - `mixinsFlyable::::Fish_Flyable::fly::param_1 -> obj_1,obj_3`（FlyingFish）
    - `mixinsSwimmable::::Bird_Flyable_Swimmable::swim::param_1 -> obj_0,obj_2`
    - `mixinsSwimmable::::Fish_Flyable_Swimmable::swim::param_1 -> obj_1,obj_3`
  - 仍有 5 个 `(none)`，集中在未直接喂流的内部组合 wrapper。
- `type-info`:
  - 主路径类型准确：
    - Bird 侧为 `Duck`
    - Fish 侧为 `FlyingFish`
  - 5 个 `(none)` 与 `class_pts` 对应。
- `cast_safety.log`:
  - 文件为空（本 entry 无 cast 检查点）。

## 5) Precision Metrics (Required)

Use FP/FN if available. If not, fill proxy metrics and mark as proxy.

- FP (false positives): `N/A` (proxy-only in this phase)
- FN (false negatives): `N/A` (proxy-only in this phase)
- Precision (%): `N/A`
- Recall (%): `N/A`
- F1 (%): `N/A`

Proxy metrics (when FP/FN are not directly measurable):
- `(none)` count in key outputs (`class_pts + type-info`): `10`（`5 + 5`）
- Empty points-to pointers: `5`（`15-10`）
- Type-complete pointers: `10/15`（`66.7%`）

## 6) Validation Verdict

- Status: **Partial Pass**
- Confidence: **High**
- Regression check result:
  - 按“实参有对象则形参必须有对象”标准，未发现违规（0 例）。
  - 剩余空洞来自未被入口直接调用的内部 wrapper 形参。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multiple_mixin_method_independence_entry22_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multiple_mixin_method_independence_entry22_rerun/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multiple_mixin_method_independence_entry22_rerun/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 主调用链精度稳定，Duck/FlyingFish 两族 mixin 调用保持独立。
- Measurable benefit:
  - 关键“有实参流入但形参为空”违规数为 0。
- Practical value:
  - 证明主路径可用；剩余改进点集中在内部 wrapper 去噪策略。

## 8) Next Action

- Next candidate entry:
  - `(animal_hierarchy suites entries finished)`
- Need follow-up source changes:
  - `Optional`（若目标是全量输出无 `(none)`，可专项处理内部 wrapper 形参去噪）
