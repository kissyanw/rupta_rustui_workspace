# RCPTA 会话快照（2026-04-22）

- 快照时间: 2026-04-22 21:06 CST（已合并本次对话进展）
- 工作区: `/home/wy/rupta_rustdsl_workspace`
- 当前主线: 继续按 entry 驱动优化 `rcpta` 指针分析精度
- 固定配置: `--context-depth 1`

## 1. 本轮确认的优化边界（重要）

用户已明确：
- 当前阶段只优化 `rcpta` 本体精度（PAG/PTS/type-info）。
- `cast_safety` 结果仅作为辅助观察，不作为本阶段主要优化目标。
- safety detection plugin 本身后续单独优化（当前不进入该插件判定逻辑优化）。

执行准则：
- 优先依据 `class_pts` / `type-info` 与源码语义的一致性判断问题。
- 对明显属于 safety detection plugin 的问题，只记录不改插件逻辑。
- 每个 entry 验证日志需显式记录“是否改动源码 + 改动文件 + 改动原因”（若无改动也要写 `No`）。

## 2. Entry 进度状态

已完成验证：
1. Entry #1 `prop_multilevel_upcast_preserves_identity`（历史已修复）
2. Entry #2 `test_downcast_animal_to_dog_success`
3. Entry #3 `test_downcast_animal_to_eagle_through_bird`
4. Entry #4 `test_downcast_animal_to_shark_through_fish`
5. Entry #5 `test_downcast_animal_dog_to_cat_failure`
6. Entry #6 `test_downcast_animal_eagle_to_penguin_failure`
7. Entry #7 `test_downcast_bird_eagle_to_duck_failure`
8. Entry #8 `test_downcast_fish_shark_to_salmon_failure`
9. Entry #9 `test_downcast_does_not_panic`
10. Entry #10 `prop_downcast_type_safety`
11. Entry #11 `test_mixin_reference_back_conversion`
12. Entry #12 `prop_mixin_reference_access_integrity`
13. Entry #13 `prop_mixin_bidirectional_conversion`
14. Entry #14 `prop_multiple_mixin_independent_conversion`
15. Entry #15 `test_polymorphic_collection`

下一项待做：
- Entry #16 `test_polymorphic_collection_by_category`

## 3. 本轮新增 rcpta 修复（Entry #11 期间）

文件：
- `rupta/src/util/class/analysis.rs`

修复点：
1. 将 `::mixin_to_impl` 纳入 `DSL_CLASS_CAST_CALLEE_MARKERS`，让 mixin->impl 路径按 class cast 建图。
2. `Option` 取值识别从仅 `unwrap` 扩展为 `unwrap || expect`，打通 `try_into_subtype(...).expect(...)` 路径的指针流。

效果（Entry #11）：
- 回转链关键指针从 `(none)` 变为具体类型（如 Eagle/Duck/FlyingFish）。
- `main.rs` 中 mixin 回转链对应行（796/797/865/866/935/936）已为 safe。
- 仍存在宏内部 `classes/src/macros/mod.rs:1171` 的 unsafe 噪声，按当前边界仅记录不作为本阶段目标。

## 3.1 本次对话新增的两处精度改动（重点）

改动 A（Entry #13 期间）：
- 文件：`rupta/src/util/class/analysis.rs`
- 内容：将宏生成 `::to_impl<classes::class::Virtual>` helper 识别为 internal DSL trait context，不再作为 source-level 指针/边建图。
- 影响：清除 `mixins*::to_impl<...>` 悬空指针噪声；`prop_mixin_bidirectional_conversion` 中原有 8 个 `(none)` 清零。

改动 B（Entry #15 期间）：
- 文件：`rupta/src/builder/fpag_builder.rs`
- 内容：新增 `Iterator::next`（返回 `Option<(idx, &Class)>`）建图规则，对 `Option.Some.0.1`（循环元素）补充保守 Assign 回流。
- 影响：打通容器迭代元素到 `CallArg` 形参传播；`test_polymorphic_collection` 中
  `local_134.as_variant#1.0.1` 与 `Animal::{make_sound,move_action,describe}::param_1` 由 `(none)` 变为完整非空联合。

补充验证（同次对话）：
- 重新跑 `prop_polymorphic_method_call_correctness` 后，历史上 4 个空指针也已消失：
  - `birdBird::{make_sound,move_action}::param_1`
  - `fishFish::{make_sound,move_action}::param_1`

## 4. 产物与记录索引

- 会话归档（旧）:
  - `codex/SESSION_ARCHIVE_2026-04-21.md`
- 本次快照:
  - `codex/SESSION_SNAPSHOT_2026-04-22.md`
- Entry 记录（新增）:
  - `codex/ENTRY6_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY7_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY8_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY9_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY10_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY11_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY12_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY13_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY14_VALIDATION_NOTES_2026-04-22.md`
  - `codex/ENTRY15_VALIDATION_NOTES_2026-04-22.md`
- 日志模板（新增）:
  - `codex/ENTRY_LOG_TEMPLATE.md`

## 5. 续做建议（下次直接执行）

1. 继续 Entry #16 分析：
   - `./run_rcpta.sh rustdsl/classes/tests/animal_hierarchy/main.rs test_polymorphic_collection_by_category analysis_results/rcpta/animal_hierarchy/test_polymorphic_collection_by_category --analyze-only --context-depth 1`
2. 验证重点只看 `class_pag/class_pts/type-info` 语义一致性。
3. 若 `cast_safety` 异常但 PTS/type-info 正确，记录为插件侧待后续处理。
4. 每个 entry 记录必须包含 `Source Changes in this Entry` 区块，明确是否改源码与改动原因。
