# RCPTA 项目汇报配套文档

适配 PPT：`doc/rcpta_project_report_template_style_polished.pptx`

本文档用于配合最终版 PPT 进行项目汇报，建议汇报时长为 12-15 分钟。整体叙述主线是：

1. 为什么普通 Rust MIR 指针分析不能直接解释 Rust OO-style DSL 的对象流和 cast site。
2. RCPTA 如何在 RUPTA 基础上建立 ClassPAG / ClassPTS / 动态类型集合 / cast safety 诊断。
3. 项目如何经历旧 DSL 基线、旧 DSL 增强、新 Lite Class DSL 适配三个阶段。
4. 最终测试结果、cast risk matrix 结果、精度口径和不精确来源。

## 汇报开场

本项目的目标是构建 RCPTA，也就是 Rust Class Pointer Type Analysis。它面向 Rust 中用宏和类型系统模拟出来的 OO-style DSL 程序，恢复源码层面的 class、interface、mixin、对象流和 checked cast 语义。

普通 MIR 层分析能看到局部变量、临时引用、投影路径和标准库调用，但很难直接回答“这个 class 对象可能流到哪里”“这个 receiver 可能是什么动态类型”“这个 downcast 是否一定安全”这类源码级问题。RCPTA 在 RUPTA 的 MIR 分析基础上增加一层 class 语义抽象，并输出可审计的分析产物。

## Slide 1：标题页

这一页给出项目最终状态的四个关键数字：

- 旧 DSL 51 个入口全部可分析。
- 新 Lite Class DSL 三套原始测试套件保留 69 个 RCPTA 分析结果。
- Cast Risk Matrix 包含 90 个高价值 cast 场景。
- 最终没有 unknown 诊断。

讲述重点：

RCPTA 的核心贡献不是只跑通测试，而是把 MIR 层复杂展开恢复成可解释的对象流、动态类型集合和 cast 安全分类。这里的 “0 unknown” 尤其重要，因为它说明当前高价值 cast site 中已经没有因为空 points-to 或边界信息不足而无法诊断的场景。

## Slide 2：项目问题与目标

这一页说明为什么需要 RCPTA。

分析难点：

- Rust DSL 源码里看起来是 class 对象、继承、接口和 downcast。
- 编译到 MIR 后会变成 clone、move、引用临时变量、Option / Result、容器 API 和宏展开代码。
- 如果直接看普通 points-to，很难对应回源码里的类对象和 cast site。

RCPTA 目标：

- 构造 ClassPtr、ClassObj 和 ClassPAG。
- 在 ClassPAG 上求解 ClassPTS。
- 从 ClassPTS 投影动态类型集合。
- 结合 class / interface / mixin 关系输出 safe、may-unsafe、must-unsafe 或 unknown。

可以强调：RCPTA 的定位是源码级可解释静态分析工具，不是替代 Rust 编译器类型检查，而是补充运行时 downcast / cast erase 优化所需的静态证据。

## Slide 3：两阶段技术路线

这一页解释项目不是一次性完成，而是两阶段推进。

Stage 1：旧 DSL 基线

- 建立 ClassPAG、ClassPTS、ClassCG。
- 在 animal、shape、vehicle、rcpta_full 四套旧 DSL 测试中验证基本对象流和调用关系。
- 第一阶段有 44 / 51 个入口成功，vehicle 的 property test 入口存在栈溢出和外部依赖噪声问题。

Stage 2A：旧 DSL 增强

- 解决 proptest 入口下钻。
- 增强 wrapper / container 传播。
- 增加动态类型集合和 cast risk detection。
- 旧 DSL 51 / 51 个入口全部成功，非空 points-to 比例约 97.35%，共诊断 253 个旧 DSL cast site。

Stage 2B：Lite DSL 适配

- 面向新的 Lite Class DSL 和 cast erase 需求适配分析。
- 保留三套原始 Lite DSL 测试结果。
- 构造专门的 90 入口 Cast Risk Matrix。
- 最终 0 unknown / 0 空源诊断。

## Slide 4：当前 RCPTA 架构

这一页看图讲整体 pipeline：

1. 输入层：Rust DSL 源码、测试入口、class/interface/mixin/CRc wrapper。
2. 提取层：cargo-pta driver 接入 rustc，提取 MIR、函数信息和 DSL metadata。
3. RCPTA 核心分析：ClassPAG 构建、语义摘要、ClassPTS 求解、cast-aware propagation。
4. Cast Safety Diagnosis：消费 class_pts、type-info、inheritance graph、cast sites 和 pre-cast PTS。
5. 输出层：class_pag、class_pts、class_cg、type-info、inheritance graph、cast_safety.log。

讲述重点：

Stage-2 semantic enhancements 是当前版本最关键的工程增量，包括 wrapper payload bridge、container value flow、interface / mixin view propagation。它们保证对象流不会在 Option / Result / Vec / HashMap 等常见 API 中断开。

## Slide 5：核心抽象：ClassPAG 与 ClassPTS

这一页讲技术核心。

ClassPtr：

- 表示“可能持有 DSL 类对象引用”的路径。
- 可以是局部变量、参数、返回值、字段、wrapper payload、container element slot、map value slot、cast 源或 cast 结果。

ClassObj：

- 基于分配点抽象对象。
- 每个对象记录动态 class type。
- 动态类型集合就是从 ClassObj 的 class type 投影得到。

ClassPAG 边：

- Alloc：对象创建。
- Assign：clone、move、payload 传播。
- Cast：类型视图转换或 checked cast。
- Load / Store：字段读写。
- CallArg / CallRet：跨函数传播。

关键解释：

Cast 边会保留 pre-cast PTS 快照。这样 cast safety 判断用的是 cast 前源对象集合，而不是 cast 后已经被目标类型过滤过的结果，避免自污染。

## Slide 6：Cast Safety 分类逻辑

这一页说明分类规则。

输入证据：

- Source：pre-cast source PTS 和动态类型集合。
- Target：目标静态类型。
- Relation：class extends*、interface implements*、mixin with*。

分类规则：

- safe：所有源动态类型都满足目标类型，可视为 must-safe。
- may-unsafe：部分源动态类型满足，部分不满足，需要保留运行时检查。
- must-unsafe：所有源动态类型都不满足，表示必然失败路径。
- unknown：源 points-to 或目标类型证据不足，是建模缺口。

讲述重点：

RCPTA 的诊断结果是可解释的。它不仅输出 safe / unsafe，还会输出源动态类型集合、目标静态类型、满足集合和不满足集合。因此可以审计为什么某个 cast 被判为 may-unsafe 或 must-unsafe。

## Slide 7：第二阶段关键语义增强

这一页讲第二阶段具体做了什么。

Wrapper 摘要：

- Option / Result unwrap。
- ok_or、map、and_then。
- or、or_else、unwrap_or_else。
- as_ref 引用 wrapper payload bridge。

Container / Iterator：

- Vec element summary。
- iter / into_iter / next / find。
- collect result element slot。
- HashMap / BTreeMap value slot。

入口与外部依赖：

- proptest wrapper drill-down。
- 业务 closure 补种为分析根。
- 外部 framework callee 过滤。

Cast-aware propagation：

- 记录 cast 前源 points-to。
- 目标类型过滤传播，避免 cast 后自污染。

当前边界：

主验证路径仍是上下文不敏感、流不敏感。因此字段覆盖、多个对象实例合流等场景会保守地产生 may-unsafe。

## Slide 8：旧 DSL 测试结果

这一页汇报旧 DSL 结果。

第一阶段：

- 44 / 51 个入口成功。
- vehicle 中 7 个 prop_* 入口因为 proptest 包装和外部依赖展开触发栈溢出。

第二阶段：

- 51 / 51 个入口全部成功。
- 917 / 942 个类级指针具有非空 points-to。
- 非空比例约 97.35%。
- 共诊断 253 个旧 DSL cast site。
- 217 safe，36 unsafe，其中 18 must-unsafe，0 boundary unknown。

重点解释：

这一页证明 RCPTA 在旧 DSL 上从“原型可行”推进到“四套件全入口可分析”。同时，vehicle 的接口多态 receiver 类型能够恢复，说明 RCPTA 不只是处理简单 downcast，也能处理接口集合和多态调用。

## Slide 9：新 Lite Class DSL 三套原始测试套件

这一页要特别说明：这些不是旧 DSL 的 animal / shape / vehicle 结果，而是新 Lite Class DSL 的结果。

数据路径：

`lite_class_dsl/analysis_results/rcpta`

入口数量：

- Lite animal_hierarchy：30。
- Lite shape_hierarchy：20。
- Lite vehicle_hierarchy：19。
- 合计：69。

聚合规模：

- ClassPtr：6928。
- ClassObj / Alloc：789。
- Assign edges：1173。
- Cast edges：979。
- Typed ptrs：2136。

Cast 日志：

- Safe cast records：65。
- Must-unsafe records：31。
- No-cast entries：44。
- Unknown：0。
- 未出现 empty points-to 源。

讲述口径：

这三套新 Lite DSL 测试主要服务 DSL 本体开发和兼容性回归，不是专门为 RCPTA 精度边界设计的 benchmark。但它们证明 RCPTA 可以在新 Lite DSL 真实测试程序上构建 ClassPAG、恢复动态类型集合，并对其中出现的 checked cast 给出确定诊断。

## Slide 10：Cast Risk Matrix

这一页汇报专门为 cast risk detection 构造的数据集。

总规模：

- 90 个 public entries。
- 43 safe。
- 40 may-unsafe。
- 7 must-unsafe。
- 0 unknown。

覆盖场景：

- local object、helper return、多级继承 downcast。
- field store-load、two-holder precision、clone alias chain。
- Option / Result unwrap、map、and_then、fallback、as_ref。
- Vec / iterator / collect、HashMap / BTreeMap value。
- interface / mixin 通过 wrapper / container 后进入 downcast_ref。

收尾质量：

- 入口数和结果目录数均为 90。
- 没有 boundary-unknown。
- 没有 empty points-to cast source。

讲述重点：

Cast Risk Matrix 是最终能力验证的核心数据集。三套原始 Lite DSL 测试证明兼容性，Cast Risk Matrix 则专门验证 RCPTA 对高价值 cast site 的诊断能力。

## Slide 11：典型修复案例：as_ref 引用 Wrapper

这一页讲一个具体修复点，帮助评委理解工具改动不是只堆测试。

问题形态：

`Option<CRc<T>>::as_ref()` 会生成 `Option<&CRc<T>>`。

`Result<CRc<T>, E>::as_ref()` 会生成 `Result<&CRc<T>, &E>`。

旧模型的问题：

destination 引用 wrapper payload 没有接回 receiver payload。后续 `as_ref().unwrap().clone()` 后进入 cast 时，cast 源 points-to 为空。

修复策略：

- 把 receiver payload 接到 destination payload。
- 保留 receiver holder 已有对象流。
- 支持 `as_ref().unwrap().clone()` 后继续进入 cast。

验证结果：

- Option / Result safe 场景可证明。
- interface wrapper may-unsafe 可解释。
- mixin wrapper must-unsafe 可解释。
- 最终矩阵无 unknown。

## Slide 12：当前成果与工程产物

这一页总结当前工具能力和输出。

分析能力：

- 类级对象流恢复。
- 字段流、函数参数/返回值传播。
- wrapper / container 摘要。
- 动态类型集合推断。
- class / interface / mixin 类型关系解析。

输出产物：

- `class_pag.txt`
- `class_pts.txt`
- `class_cg.txt`
- `type-info.txt`
- `inheritance_graph.txt`
- `cast_safety.log`
- per-entry `analysis_results`

汇报结论：

- 旧 DSL 四套件全入口可分析。
- Lite DSL 原始套件兼容性通过。
- 90 个 cast risk matrix 场景完成分类。
- unknown 和空源问题清零。

## Slide 13：精度审计与不精确来源

这一页是最后的收束，重点回答“工具诊断和真实情况有什么出入”。

按 90 个 Cast Risk Matrix 场景的源码意图审计：

- 真实 must-safe：45。
- 真实 may-unsafe：38。
- 真实 must-unsafe：7。

工具输出：

- 43 safe。
- 40 may-unsafe。
- 7 must-unsafe。

精度口径：

- exact 三分类正确：88 / 90。
- 总体准确率：97.8%。
- unsafe 假阳性：2。
- must-unsafe 错误：0。
- unknown：0。

假阳性解释：

false positive 指真实安全，但工具因为保守合流报为 may-unsafe。

两个典型来源：

1. two_holder 精度问题：两个 holder 的字段内容被合并，导致 dog_holder 中读取到的对象集合被混入 cat_holder 的 Cat。
2. field overwrite 问题：流不敏感分析会保留旧写入。例如先写 Cat 再写 Dog，真实最后读取应为 Dog，但静态分析会把 Cat 和 Dog 都保留，因此报 may-unsafe。

总结口径：

safe 输出当前可视为 must-safe，未发现错误；must-unsafe 输出与真实必失败场景一致；may-unsafe 是保守告警，包含真实风险和少量假阳性。因此，如果后续服务 cast erase，最适合优先消费 safe 和 must-unsafe 结果，而 may-unsafe 应保留运行时检查或进入更精细分析。

## 可能问答准备

### Q1：RCPTA 和普通 Rust 类型检查有什么区别？

Rust 类型检查验证静态类型合法性，但 checked downcast 的成败取决于运行时动态类型。RCPTA 关注的是对象流和动态类型集合，目标是静态证明某些 downcast 一定安全、一定失败或可能失败。

### Q2：为什么需要 ClassPAG，而不是直接用底层 PAG？

底层 MIR PAG 会包含大量临时变量、引用路径、标准库实现细节和宏展开代码。ClassPAG 把这些信息提升为源码层 class 对象、字段、wrapper payload、container element 等抽象，使结果可解释，也能直接服务 cast site 诊断。

### Q3：为什么 may-unsafe 不等于工具错误？

may-unsafe 是保守静态分析的预期结果。它表示源动态类型集合中既有满足目标类型的对象，也有不满足的对象。在上下文不敏感和流不敏感条件下，某些真实安全路径会因为对象集合合流而进入 may-unsafe，这是假阳性，但不是 unsound。

### Q4：当前最大的不精确来源是什么？

主要是上下文不敏感、对象不敏感和流不敏感：

- 同一函数不同调用点的对象流可能合并。
- 同一字段在不同对象实例上的内容可能被保守合并。
- 字段多次写入时旧写入不会被 kill。

这些选择换来的是实现稳定、分析可控和更高覆盖率。

### Q5：为什么说 unknown 清零很重要？

unknown 通常表示源 points-to 为空、目标类型缺失或类型关系不可判定。对于 cast erase 来说，unknown 不能提供可用优化证据。当前 Cast Risk Matrix 中 unknown 为 0，说明高价值场景中的主要建模缺口已经补齐。

### Q6：safe / must-unsafe 能否直接用于优化？

从当前矩阵结果看，safe 和 must-unsafe 没有发现错误。safe 可以作为消除运行时检查的候选；must-unsafe 可以用于识别必失败路径。但实际接入优化 pass 前仍应保留工程级 guard，例如只对结果完整、类型关系完整、无 unknown 的入口启用。

## 结束语

RCPTA 当前已经完成从旧 DSL 类级 points-to 分析原型，到新 Lite Class DSL cast site 诊断工具的迁移。最终结果表明：工具可以稳定恢复 class 对象流和动态类型集合，并在高价值 cast 场景上给出 0 unknown、97.8% exact 三分类准确率的诊断结果。剩余不精确主要来自有意识选择的上下文不敏感和流不敏感策略，表现为少量 may-unsafe 假阳性，而不是 safe 或 must-unsafe 的错误判断。
