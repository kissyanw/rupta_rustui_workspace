# Entry Validation and Value Report

- Entry: `test_polymorphic_collection_by_category`
- Date: `2026-04-23`
- Config: `context-depth 1`
- Output dir:
  - Before: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun`
  - After: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun_after_fix2`
- Owner: `Codex`

## 1) Problem Statement (Non-Technical)

- What issue is addressed in this entry:
  - 分类多态集合循环中的元素没有被 rcpta 识别为真实对象，导致后续方法调用参数为空。
- User/business impact if not fixed:
  - 分析结果会错误地显示关键多态调用“无对象/无类型”，降低 rcpta 在集合场景下的可信度。

## 2) Scope and Change Type

- Affected scenario(s):
  - `for (i, bird) in birds.iter().enumerate()`
  - `for (i, fish) in fishes.iter().enumerate()`
- Source changed: **Yes**
- Files changed:
  - `rupta/src/builder/fpag_builder.rs`
- Why this change is needed:
  - `Iterator::next` 的 item 类型提取在该 entry 下不稳定，导致 `Option.Some.0.1` 未建立有效 Assign 回流。

## 3) Expected Semantics

1. `Bird` 分类集合循环元素应传播 `Eagle/Penguin/Duck/Ostrich`。
2. `Fish` 分类集合循环元素应传播 `Shark/Salmon/FlyingFish`，并进一步传播到 `make_sound/move_action` 形参。

## 4) Observed Result (Technical Evidence)

- `class_pag`:
  - Before: `assign: 0`，循环元素无回流边。
  - After: `assign: 7`，新增：
    - `local_34/36/38/40 -> local_57.as_variant#1.0.1`
    - `local_127/129/131 -> local_148.as_variant#1.0.1`
- `class_pts`:
  - Before: `local_57.as_variant#1.0.1`、`local_148.as_variant#1.0.1` 与 `bird/fish/animal` 方法形参均为 `(none)`。
  - After:
    - `local_57.as_variant#1.0.1 -> obj_0,obj_1,obj_2,obj_3`
    - `local_148.as_variant#1.0.1 -> obj_4,obj_5,obj_6`
    - `birdBird::*::param_1` 与 `fishFish::*::param_1` 全部非空并与分类一致。
- `type-info`:
  - Before: 关键循环元素与方法形参类型范围为 `(none)`。
  - After:
    - Bird 形参：`Duck,Eagle,Ostrich,Penguin`
    - Fish 形参：`FlyingFish,Salmon,Shark`
    - Animal 形参：7 类联合
- `cast_safety.log`:
  - Before/After 均为 `cast is safe`（7 个 cast 点，无新增 unsafe）。

## 5) Precision Metrics (Required)

Use FP/FN if available. If not, fill proxy metrics and mark as proxy.

- FP (false positives): `N/A` (proxy-only in this phase)
- FN (false negatives): `N/A` (proxy-only in this phase)
- Precision (%): `N/A`
- Recall (%): `N/A`
- F1 (%): `N/A`

Proxy metrics (when FP/FN are not directly measurable):
- `(none)` count in key outputs (`class_pts + type-info`): `16 -> 0`
- Empty points-to pointers: `8 -> 0` (from `22-14` to `22-22`)
- Type-complete pointers: `14/22 -> 22/22` (`63.6% -> 100%`)

## 6) Validation Verdict

- Status: **Pass**
- Confidence: **High**
- Regression check result:
  - 未观察到本 entry 的语义回归；`cast_safety` 保持稳定。
- Evidence paths:
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun_after_fix2/class_pag.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun_after_fix2/class_pts.txt`
  - `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category_entry16_rerun_after_fix2/type-info.txt`

## 7) Value Summary (For Presentation)

- Optimization behavior:
  - 修复了分类多态集合迭代中的元素传播断链，恢复了“循环元素 -> 调用形参”的完整流。
- Measurable benefit:
  - 本 entry 关键空结果从 16 项降到 0，类型完整率从 63.6% 提升到 100%。
- Practical value:
  - 明显减少“误报为空”的分析噪声，提升多态集合场景下的定位效率与解释性。

## 8) Next Action

- Next candidate entry:
  - `test_polymorphic_method_calls`
- Need follow-up source changes:
  - `No`（先按新 entry 结果继续验证，若出现新断链再增量修复）
