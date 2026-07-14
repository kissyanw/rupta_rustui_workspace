# Animal Suites 总结报告（可追溯版）

- 范围：`rustdsl/classes/tests/animal_hierarchy/main.rs` 全部 22 个 entry
- 日期：`2026-04-23`
- 分析配置：`--context-depth 1`
- 结论类型：分为“高置信（有完整证据链）”与“中置信（早期记录不完整，基于日志回放）”

## 1. 总体完成度

- 已测试 entry：`22/22`
- 记录覆盖：`ENTRY1 ... ENTRY22` 全部有文档
- 批次划分：
  - 批次 A（早期）：Entry 1-5（2026-04-21）
  - 批次 B（中期）：Entry 6-15（2026-04-22）
  - 批次 C（后期）：Entry 16-22（2026-04-23，已采用新版 value report）

## 2. 源码修复主线（可还原）

可明确还原的 rcpta 源码修复点：

1. Entry #1（高置信）
   - 文件：`rupta/src/builder/fpag_builder.rs`
   - 修复：CallArg actual canonical、`deref/to_supertype` 归一、ptr-id 编码统一
   - 效果：日志记录为 `type-info/class_pts` 从有 `(none)` 到无 `(none)`，并达到 `ptrs_with_types=20/20`, `ptrs_with_objs=20/20`

2. Entry #11（高置信）
   - 文件：`rupta/src/util/class/analysis.rs`
   - 修复：
     - `::mixin_to_impl` 纳入 cast marker
     - `Option::expect` 纳入 unwrap 同类处理
   - 效果：原本 entry 关键 cast unsafe（空 points-to）转为 safe

3. Entry #13（高置信）
   - 文件：`rupta/src/util/class/analysis.rs`
   - 修复：过滤宏生成 `::to_impl<classes::class::Virtual>` 内部 helper，避免污染 source-level 输出
   - 效果：`(none)` 噪声清零（记录为 `18/18` 完整）

4. Entry #15（高置信）
   - 文件：`rupta/src/builder/fpag_builder.rs`
   - 修复：`Iterator::next` 的 `Option<(idx,&Class)>` 元素传播建图
   - 效果：迭代元素到 `CallArg` 形参传播恢复，循环路径 `(none)` 清除

5. Entry #16（高置信，后续增强）
   - 文件：`rupta/src/builder/fpag_builder.rs`
   - 修复：补强 `next` item 类型提取与兼容匹配
   - 效果（有严格前后对比）：`(none)` 从 `16 -> 0`，`ptrs_with_objs 14/22 -> 22/22`

## 3. 三批次验证结果

### 批次 A（Entry 1-5）

- 结论：通过，且 Entry #1 发生关键修复并解决空洞。
- 可追溯性：中到高
  - Entry #1：高（有“问题-修复-前后结果”）
  - Entry #2-5：中（旧模板，量化粒度较粗）

### 批次 B（Entry 6-15）

- 结论：总体通过；Entry #11/#13/#15 有实质修复并在日志中体现效果。
- 可追溯性：中到高
  - Entry #11/#13/#14/#15：高（包含较明确语义与结果）
  - Entry #6-10/#12：中（部分缺少统一 before/after 指标）

### 批次 C（Entry 16-22）

- 结论：`Pass=5`（16-20），`Partial Pass=2`（21-22）
- `Partial Pass` 原因：
  - 剩余 `(none)` 集中在内部 mixin wrapper 形参（`...::::...`），非测试主调用路径
  - 未出现主路径语义错误
- 可追溯性：高（全部使用新版 value report，含指标）

## 4. 量化指标（当前可严格统计部分）

> 说明：早期批次（A/B）缺少统一机器可比指标，本节给出“严格可比”的 C 批次与关键修复点指标。

### 4.1 批次 C（Entry 16-22）聚合

- 聚合类型完整率（`Σptrs_with_types / Σptrs`）：
  - `192 / 201 = 95.5%`
- 其中 Pass 子集（Entry 16-20）：
  - `157 / 157 = 100%`
- Partial Pass 子集（Entry 21-22）：
  - 主要受内部 wrapper `(none)` 影响（非主路径）

### 4.2 关键修复点（可前后对比）

- Entry #16：
  - `(none)`：`16 -> 0`
  - `ptrs_with_objs`：`14/22 -> 22/22`
  - `ptrs_with_types`：`14/22 -> 22/22`

- Entry #1（来自历史优化记录）：
  - 修复后 `ptrs_with_objs=20/20`, `ptrs_with_types=20/20`
  - 旧记录显示修复前存在多处 `(none)`，但未保留统一数字口径

## 5. 对“早期修复真实效果”的可还原性评估

- 能还原（高置信）：
  - Entry #1/#11/#13/#15/#16 的修复点、改动文件、修复后行为变化

- 部分能还原（中置信）：
  - Entry #2-10/#12 的“通过结论”可以确认
  - 但缺少统一 `before -> after` 量化（尤其 FP/FN 或统一 `(none)` 计数）

- 不能严格还原（低置信）：
  - 早期某些 entry 的“精确数值收益”无法做到逐项可重复比对（因为当时未按新模板记录）

## 6. 最终结论

1. `animal_hierarchy` 的 22 个 entry 已全部跑通并完成记录。
2. rcpta 的主要精度问题已在关键 entry（1/11/13/15/16）被修复并验证。
3. 当前残余问题集中于内部 wrapper 形参噪声（Entry 21/22），不影响主调用语义正确性。
4. 若下一阶段目标是“全量输出零 `(none)`”，建议单独立项处理内部 wrapper 去噪策略；若目标是“主路径语义正确”，当前已达到可用状态。
