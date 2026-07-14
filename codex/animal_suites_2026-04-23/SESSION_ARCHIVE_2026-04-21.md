# RCPTA 会话快速恢复归档（面向下次续做）

- 初始归档时间: 2026-04-21 20:28:42 CST
- 最后更新时间: 2026-04-21 21:26 CST
- 工作区: `/home/wy/rupta_rustdsl_workspace`
- 提交基线: `b781549`
- 当前阶段目标: 只做 `rcpta` 精度优化，暂不进入 cast unsafety plugin 优化。

## 1. 本轮会话目标与边界

用户目标：
- 让 Codex 快速熟悉 rcpta，并按 entry 逐个优化精度。
- 以真实测试入口驱动：观察 `class_pag/class_pts/type-info` 与源码语义差异，定位并修复。

明确边界：
- 固定 `context-depth 1`。
- 暂不优化 cast unsafety plugin 判定逻辑。
- 采用小步快跑：每修一处立即重跑验证。

新增协作规则（本轮后期用户明确）：
- 每个 entry 开始前，需要先征求用户同意再继续下一个 entry。

## 2. 决策脉络（为什么这样做）

1. 先系统熟悉 rcpta 链路。
2. 再确认如何稳定调用指定 entry。
3. 选 `animal_hierarchy` 第一个 entry 做试点闭环。
4. 用“结果噪声（none）-> 根因 -> 修复 -> 回归”方式建立可复制优化模板。
5. 用连续 entry 验证（成功/失败 downcast 混合）检查是否回归。

选择该路径的原因：
- entry 驱动最贴近用户真实使用方式。
- 能快速暴露建图层（CallArg/Alias/Ref 映射）问题。
- 修复收益可立刻在 `type-info/class_pts` 上量化。

## 3. rcpta 快速知识卡（供下次秒恢复）

核心模块：
- `rupta/src/rcpta/class_ptr.rs`
- `rupta/src/rcpta/class_obj.rs`
- `rupta/src/rcpta/class_pag.rs`
- `rupta/src/rcpta/class_pts.rs`

关键拼接点：
- `rupta/src/mir/analysis_context.rs`（`class_pag` + alias/ref maps）
- `rupta/src/builder/fpag_builder.rs`（MIR -> ClassPAG 主喂图点）
- `rupta/src/builder/special_function_handler.rs`（alloc/cast 等）
- `rupta/src/util/results_dumper.rs`（PAG/PTS/type-info 输出）

## 4. 标准执行命令（已验证）

```bash
cd /home/wy/rupta_rustdsl_workspace

# 目标测试一次编译
./run_rcpta.sh compile rustdsl/classes/tests/animal_hierarchy/main.rs

# 分析指定入口（固定 context-depth=1）
./run_rcpta.sh \
  rustdsl/classes/tests/animal_hierarchy/main.rs \
  <entry_func_name> \
  analysis_results/rcpta/animal_hierarchy/<entry_func_name> \
  --analyze-only \
  --context-depth 1
```

重要操作约束：
- `run_rcpta.sh` 不会自动重建 `rupta`。
- 修改 `rupta` 源码后必须先执行：
```bash
cd /home/wy/rupta_rustdsl_workspace/rupta
cargo build
```

## 5. 当前状态快照（截至今天收尾）

已完成 entry：
1. `prop_multilevel_upcast_preserves_identity`（Entry #1）
2. `test_downcast_animal_to_dog_success`（Entry #2）
3. `test_downcast_animal_to_eagle_through_bird`（Entry #3）
4. `test_downcast_animal_to_shark_through_fish`（Entry #4）
5. `test_downcast_animal_dog_to_cat_failure`（Entry #5）

结论摘要：
- Entry #1 做了实质修复（见修复日志），并将 `none` 噪声清零。
- Entry #2~#5 验证通过，未暴露新的必须修复的 rcpta bug。
- Entry #5 中“指针静态类型 Cat 但 type-info 推断 Dog”被判定为预期行为（静态目标 vs 动态对象类型）。

下一步待做：
- Entry #6: `test_downcast_animal_eagle_to_penguin_failure`
- 开始前先征求用户同意（遵循新增协作规则）。

## 6. 已产出日志与文档索引

- 会话归档：
  - `codex/SESSION_ARCHIVE_2026-04-21.md`
- 修复日志：
  - `codex/ENTRY1_OPTIMIZATION_NOTES_2026-04-21.md`
- 验证日志：
  - `codex/ENTRY2_VALIDATION_NOTES_2026-04-21.md`
  - `codex/ENTRY3_VALIDATION_NOTES_2026-04-21.md`
  - `codex/ENTRY4_VALIDATION_NOTES_2026-04-21.md`
  - `codex/ENTRY5_VALIDATION_NOTES_2026-04-21.md`

## 7. 下次会话启动脚本（建议）

1. 读本文件 + Entry1 修复日志。
2. `cd rupta && cargo build`（若有新改动）。
3. 问用户是否同意执行 Entry #6。
4. 执行 Entry #6 分析并按同模板记录验证/修复。
