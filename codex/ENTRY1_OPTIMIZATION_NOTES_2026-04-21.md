# Entry #1 优化记录（面向验收 Presentation）

- 入口函数: `prop_multilevel_upcast_preserves_identity`
- 测试文件: `/home/wy/rupta_rustdsl_workspace/rustdsl/classes/tests/animal_hierarchy/main.rs`
- 分析配置: `context-depth 1`
- 输出目录: `/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/animal_hierarchy/prop_multilevel_upcast_preserves_identity`

## 1. 问题定义（可复现）

初始现象：
- `class_pag` 中存在 CallArg 边，但部分 `actual` 指针在 `class_pts/type-info` 里是 `(none)`。
- 典型噪声 id：`local_104`、`local_219`、以及多个方法体中的 `...::local_5`。

业务影响：
- 类型范围出现不必要空洞，削弱 rcpta 精度与可解释性。
- 后续 cast 检测插件会受上游空洞影响（当前虽不优化插件，但上游质量必须先稳定）。

## 2. 根因定位证据

证据 A：边-点不一致
- 通过检查发现 CallArg 的 `actual` 出现在边中，但未有效参与对象传播。

证据 B：receiver 链路丢失
- MIR 中存在 `Deref::deref` 链和 `to_supertype` 返回临时。
- 这些临时若不别名回原始 receiver/base，会导致传播断链。

证据 C：ptr-id 命名空间不统一
- 局部使用 `path_to_class_ptr_id(..., None)`，而调用/参数路径使用 `param_slots` 方案。
- 同一语义实体被编码成不同 id，导致 alias 命中失败。

## 3. 修复方案（实现细节）

实现文件：
- `/home/wy/rupta_rustdsl_workspace/rupta/src/builder/fpag_builder.rs`

修复 1：CallArg actual canonical 化 + 注册
- 构造 CallArg 前先对 `actual_ptr_id` 做 `get_canonical_rcpta_ptr`。
- 强制将 `actual_ptr` 注册进 `class_pag`。

修复 2：`resolve_call` 增加 `Deref::deref` 归一
- 识别 `...ops::deref::Deref...::deref`。
- 记录 `destination ref -> base_path`。
- 建立 `destination -> canonical_base` alias。

修复 3：`resolve_call` 增加 `to_supertype` 归一
- 识别 `callee_def_path.contains("to_supertype")`。
- 将返回临时别名到 receiver-base。

修复 4：统一 ptr-id 编码到 `param_slots`
- 上述 deref/to_supertype 逻辑统一使用 `param_slots = Some(1 + self.mir.arg_count)`。
- 与函数参数/调用边命名对齐，提升 alias 命中率。

## 4. 前后对比（验收可用）

修复前：
- `type-info/class_pts` 存在多处 `(none)`（含 `local_104/local_219/local_5`）。

修复后：
- `type-info.txt` 无 `(none)`。
- `class_pts.txt` 无 `(none)`。
- 统计结果：
  - `ptrs: 20`
  - `ptrs_with_types: 20`
  - `ptrs_with_objs: 20`

代表性结果：
- `dogDog::*::param_1 -> Dog`
- `birdBird::make_sound::param_1 -> Eagle`
- `fishFish::make_sound::param_1 -> Shark`
- `animalAnimal::make_sound::param_1 -> Dog, Eagle, Shark`

## 5. 复现与验证步骤（演示可直接用）

```bash
cd /home/wy/rupta_rustdsl_workspace/rupta
cargo build

cd /home/wy/rupta_rustdsl_workspace
./run_rcpta.sh compile rustdsl/classes/tests/animal_hierarchy/main.rs
./run_rcpta.sh \
  rustdsl/classes/tests/animal_hierarchy/main.rs \
  prop_multilevel_upcast_preserves_identity \
  analysis_results/rcpta/animal_hierarchy/prop_multilevel_upcast_preserves_identity \
  --analyze-only \
  --context-depth 1
```

验收检查点：
- `analysis_results/.../type-info.txt` 不出现 `(none)`。
- `analysis_results/.../class_pts.txt` 不出现 `(none)`。

## 6. 风险与边界

风险：
- `Deref/to_supertype` 的归一规则属于语义增强，需关注是否在其它 entry 产生过度合并。

边界：
- 当前优化针对 entry1 暴露路径完成。
- 尚需在后续 entry（尤其 downcast/chain 场景）继续回归验证。

## 7. 后续计划（presentation 可作为 Roadmap）

1. 跑 entry2：`test_downcast_animal_to_dog_success`。
2. 继续沿“现象 -> 证据 -> 修复 -> 前后对比”模板固化每个 entry。
3. 在完成 `animal_hierarchy` 后汇总统一验收报告（可直接转 PPT）。
