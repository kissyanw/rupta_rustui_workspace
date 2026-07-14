# RCPTA 项目汇报讲稿

适配 PPT：`doc/rcpta_project_report_template_style_polished.pptx`

## 开场

各位老师、同学好，我今天汇报的项目是 RCPTA 两阶段静态分析与 Cast Site 诊断工具。RCPTA 的全称是 Rust Class Pointer Type Analysis，它面向的是 Rust 中通过宏和类型系统模拟出来的 OO-style DSL 程序。简单来说，这类程序在源码层面具有 class、extends、interface、mixin 以及 checked cast 等面向对象语义，但编译到 Rust MIR 之后，这些语义会被展开成大量局部变量、临时引用、泛型调用、wrapper API 和标准库容器操作。因此，普通 MIR 层指针分析虽然能看到底层数据流，却很难直接回答源码层的问题，比如某个 class 对象可能流向哪里，某个 receiver 可能有哪些动态类型，或者某个 downcast 是否一定安全。

所以，本项目的目标不是重新做一套 Rust 类型系统，而是在已有 RUPTA MIR 分析基础上，增加一层面向 DSL class 语义的抽象。RCPTA 最终输出的不只是 points-to 集合，还包括 ClassPAG、ClassPTS、Class Call Graph、动态类型集合、继承关系图以及 cast safety 日志。这样一来，工具既能解释对象流，也能为 cast erase 或 checked cast 优化提供静态证据。

## Slide 1：项目总体结果

首先看标题页，这里列出了项目最终状态的几个关键数字。旧 DSL 的 51 个入口现在已经全部可以完成分析；新 Lite Class DSL 的三套原始测试套件保留了 69 个 RCPTA 分析结果；专门面向 cast risk detection 的 Cast Risk Matrix 包含 90 个高价值 cast 场景；最后，所有这些矩阵场景最终都没有 unknown 诊断。

这里我想强调的是，RCPTA 的核心贡献不是单纯“跑通了测试”，而是把 MIR 层复杂展开恢复成源码层可解释的对象流、动态类型集合和 cast 安全分类。特别是 0 unknown 这个结果比较关键，因为 unknown 通常意味着源 points-to 为空、目标类型缺失，或者继承关系无法判定。对于 cast erase 这类下游优化来说，unknown 基本不能作为可用证据。因此，unknown 清零说明当前高价值 cast 场景中的主要建模缺口已经补齐。

## Slide 2：项目问题与目标

接下来这一页说明为什么需要 RCPTA。Rust DSL 源码里看起来是 class 对象、继承、接口、多态调用和 downcast，但经过宏展开和 MIR 编译后，会变成 clone、move、引用临时变量、Option / Result、Vec、HashMap 以及 runtime helper 调用。如果直接看普通 points-to 结果，就很难对应回源码里的类对象，也很难定位某个 cast site 的真实源对象集合。

因此，RCPTA 的目标可以分成四步。第一步是构造 ClassPtr、ClassObj 和 ClassPAG，把 MIR 中与类对象相关的路径提升为类级指针和类级对象。第二步是在 ClassPAG 上求解 ClassPTS。第三步是从 ClassPTS 投影出动态类型集合。最后一步是结合 class、interface、mixin 的类型关系，对 checked cast 输出 safe、may-unsafe、must-unsafe 或 unknown。

换句话说，RCPTA 不是替代 Rust 编译器的静态类型检查，而是补充运行时 downcast 相关的静态证据。Rust 类型检查能保证静态类型合法，但 downcast 是否成功仍然取决于运行时动态类型，而这正是 RCPTA 要恢复和解释的信息。

## Slide 3：两阶段技术路线

项目整体经历了两个阶段。第一阶段主要围绕旧 Rust class DSL 建立类级分析基线，也就是打通 ClassPAG、ClassPTS 和 Class Call Graph。在这一阶段，我们用 animal、shape、vehicle 和 rcpta_full 四套旧 DSL 测试验证基本对象流和调用关系。当时有 44 个入口能够成功产出结果，但 vehicle 中 7 个 property test 入口会因为 proptest 包装和外部依赖展开触发栈溢出。

随后进入第二阶段前半部分，目标是把旧 DSL 从“部分入口可分析”推进到“四套件全入口可分析”。这一阶段做了 proptest 入口下钻、外部 framework callee 过滤、wrapper 和 container 传播增强，并增加了动态类型集合和 cast risk detection。最终旧 DSL 51 个入口全部成功，非空 points-to 比例约 97.35%，并且完成了 253 个旧 DSL cast site 的诊断。

第二阶段后半部分则迁移到新的 Lite Class DSL。这里一方面保留了三套原始 Lite DSL 测试结果，用于证明 RCPTA 能够适配新 DSL 的真实测试程序；另一方面专门构造了 90 入口 Cast Risk Matrix，用来集中验证高价值 cast site 的诊断能力。最终结果是 0 unknown 和 0 空源诊断。

## Slide 4：当前 RCPTA 架构

这一页是当前工具的整体架构图，可以从左到右理解。输入层包括 Rust DSL 源码、测试入口、class/interface/mixin 声明以及 CRc wrapper。提取层通过 cargo-pta driver 接入 rustc，提取 MIR、函数信息和 DSL metadata。中间的 RCPTA Core Analysis 是核心部分，负责 ClassPAG 构建、语义摘要、ClassPTS 求解和 cast-aware propagation。

在核心分析之后，Cast Safety Diagnosis 会消费 class_pts、type-info、inheritance graph、cast sites 和 pre-cast PTS。最后输出 class_pag、class_pts、class_cg、type-info、inheritance_graph 和 cast_safety.log 等可审计产物。

这张图里最重要的是 Stage-2 semantic enhancements。第二阶段大量工作都集中在这些语义增强上，例如 wrapper payload bridge、container value flow，以及 interface / mixin view 穿过 wrapper 和 container 的传播。它们的作用是保证对象流不会在 Option、Result、Vec、HashMap 这些常见 API 中断开。

## Slide 5：核心抽象：ClassPAG 与 ClassPTS

为了让 MIR 层数据流回到源码层 class 语义，RCPTA 引入了两个核心抽象。第一个是 ClassPtr，它表示“可能持有 DSL 类对象引用”的路径。它可以是局部变量、参数、返回值，也可以是字段、wrapper payload、container element slot、map value slot，甚至可以是 cast 源和 cast 结果。

第二个是 ClassObj，它表示基于分配点抽象出来的类级对象。每个 ClassObj 都记录它对应的动态 class type。因此，在 ClassPTS 求解完成后，动态类型集合就可以直接从对象集合投影出来。

ClassPAG 的边主要包括 Alloc、Assign、Cast、Load / Store、CallArg 和 CallRet。Alloc 对应对象创建；Assign 对应 clone、move 或 payload 传播；Load / Store 对应字段读写；CallArg / CallRet 对应跨函数参数和返回值传播。这里有一个很关键的实现点：Cast 边会保留 pre-cast PTS 快照。这样 cast safety 判断使用的是 cast 前的源对象集合，而不是 cast 后已经被目标类型过滤过的结果，从而避免自污染。

## Slide 6：Cast Safety 分类逻辑

在 cast safety 分类时，RCPTA 需要三类证据。第一类是 Source，也就是 cast 前源指针的 points-to 集合和动态类型集合。第二类是 Target，也就是目标静态类型。第三类是 Relation，也就是 class 的 extends*、interface 的 implements* 和 mixin 的 with* 关系。

基于这些证据，分类规则比较直接。如果所有源动态类型都满足目标类型，就输出 safe，这可以视为 must-safe。如果部分满足、部分不满足，就输出 may-unsafe，表示当前需要保留运行时检查。如果所有源动态类型都不满足目标类型，就输出 must-unsafe，表示这个 cast 在当前抽象下必然失败。如果源 points-to 或目标类型证据不足，则输出 unknown。

RCPTA 的诊断结果是可解释的。它不仅输出 safe 或 unsafe，还会给出源动态类型集合、目标静态类型、满足集合和不满足集合。因此我们可以审计为什么某个 cast 被判为 may-unsafe 或 must-unsafe。

## Slide 7：第二阶段关键语义增强

接下来这一页总结第二阶段的关键语义增强。第一类是 wrapper 摘要。RCPTA 支持 Option / Result unwrap，支持 ok_or、map、and_then，也支持 or、or_else、unwrap_or_else 这类 fallback combinator。最后一轮还补上了 as_ref 引用 wrapper payload bridge。

第二类是 container 和 iterator 摘要。对于 Vec，工具会建立 element summary；对于 iter、into_iter、next、find 和 collect，会建立迭代元素和结果元素之间的传播；对于 HashMap 和 BTreeMap，会建模 value slot。这样类对象经过容器 API 后仍然能够回到 ClassPTS。

第三类是入口与外部依赖处理。旧 DSL vehicle 的 property test 入口曾经因为 proptest 包装层和外部测试框架展开导致栈溢出。因此第二阶段增加了 proptest wrapper drill-down，把业务 closure 补种为分析根，同时过滤与 DSL 类对象流无关的外部 framework callee。

最后是 cast-aware propagation。工具会记录 cast 前源 points-to，并在传播时进行目标类型过滤，避免 cast 后结果影响 cast 本身的安全判断。需要说明的是，当前主验证路径仍然是上下文不敏感、流不敏感，所以在字段覆盖或多个对象实例合流的场景中，工具会保守地产生 may-unsafe。

## Slide 8：旧 DSL 测试结果

这一页是旧 DSL 的结果。第一阶段有 44 / 51 个入口成功，失败主要集中在 vehicle 的 7 个 prop_* 入口。第二阶段修复后，旧 DSL 四套件 51 个入口全部成功。

从规模上看，第二阶段旧 DSL 中 942 个类级指针里有 917 个具有非空 points-to，非空比例约 97.35%。cast risk detection 共记录 253 个源码级 cast site，其中 217 个 safe、36 个 unsafe，36 个 unsafe 中有 18 个是 must-unsafe，并且没有 boundary unknown。

这说明 RCPTA 在旧 DSL 上完成了从原型到稳定工具的提升。同时，vehicle 的接口多态 receiver 类型也能够恢复，说明工具不只是处理简单 downcast，也能够处理接口集合和多态调用。

## Slide 9：新 Lite Class DSL 三套原始测试套件

接下来是新 Lite Class DSL 的结果。这里需要特别强调，这一页不是旧 DSL 的 animal、shape、vehicle 结果，而是来自 `lite_class_dsl/analysis_results/rcpta` 的新 DSL 分析结果。

三套新 Lite DSL 原始测试套件分别是 animal_hierarchy、shape_hierarchy 和 vehicle_hierarchy。它们主要服务 Lite DSL 本体开发和兼容性回归，不是专门为 RCPTA 精度边界设计的 benchmark。当前保留的分析入口分别是 30、20 和 19 个，合计 69 个。

从聚合规模看，这 69 个入口一共包含 6928 个 ClassPtr，789 个 ClassObj / Alloc，1173 条 Assign 边，979 条 Cast 边，以及 2136 个具有类型信息的指针。cast 日志中有 65 条 safe cast record、31 条 must-unsafe record、44 个 no-cast entry，unknown 为 0，也没有 empty points-to 源。

因此，这一页的结论是：RCPTA 已经能够在新 Lite DSL 的真实测试程序上构建 ClassPAG、恢复动态类型集合，并对其中出现的 checked cast 给出确定诊断。

## Slide 10：Cast Risk Matrix

在证明新 Lite DSL 兼容性之后，我们进一步构造了 Cast Risk Matrix。它是专门面向 cast erase 和 cast risk detection 的最终验证矩阵，而不是普通业务测试集。

这个矩阵共有 90 个 public entry，最终工具输出为 43 个 safe、40 个 may-unsafe、7 个 must-unsafe 和 0 个 unknown。覆盖的场景包括本地对象创建、helper return、多级继承 downcast、字段 store-load、two-holder precision、clone alias chain、Option / Result unwrap、map、and_then、fallback、as_ref，以及 Vec、iterator、collect、HashMap 和 BTreeMap value flow。

最终检查中，入口数和结果目录数都是 90，没有 boundary-unknown，也没有 empty points-to cast source。也就是说，前面 wrapper、container 和 as_ref 等建模修复都已经体现在这个矩阵结果里。

## Slide 11：典型修复案例：as_ref 引用 Wrapper

这一页用 as_ref 作为典型案例，说明第二阶段的修改不是简单增加测试，而是解决了真实的对象流断裂问题。

问题在于，`Option<CRc<T>>::as_ref()` 会生成 `Option<&CRc<T>>`，`Result<CRc<T>, E>::as_ref()` 会生成 `Result<&CRc<T>, &E>`。旧模型没有把 destination 引用 wrapper payload 接回 receiver payload。因此，当程序写成 `as_ref().unwrap().clone()` 后再进入 cast 时，cast 源 points-to 会变成空。

修复策略是把 receiver payload 接到 destination payload，同时保留 receiver holder 已有对象流。这样对象可以穿过 `as_ref().unwrap().clone()` 继续到达 cast 源。修复后，Option / Result safe 场景可以被证明，interface wrapper 的 may-unsafe 可以被解释，mixin wrapper 的 must-unsafe 也可以被解释。最终 Cast Risk Matrix 中没有 unknown。

## Slide 12：当前成果与工程产物

这一页总结当前工具产物。能力上，RCPTA 已经支持类级对象流恢复、字段流分析、函数参数和返回值传播、wrapper / container 摘要、动态类型集合推断，以及 class / interface / mixin 类型关系解析。

输出上，工具会为每个入口生成 class_pag.txt、class_pts.txt、class_cg.txt、type-info.txt、inheritance_graph.txt、cast_safety.log 和对应的 analysis_results 目录。这些结果都可以人工审计。

因此，阶段性结论是：旧 DSL 四套件已经全入口可分析，新 Lite DSL 原始套件兼容性通过，90 个 cast risk matrix 场景完成分类，unknown 和空源问题已经清零。

## Slide 13：精度审计与不精确来源

最后一页回答一个关键问题：工具反馈和真实情况之间有什么出入。

我们按 90 个 Cast Risk Matrix 场景的源码意图进行了审计。真实语义上，45 个场景是 must-safe，38 个场景是 may-unsafe，7 个场景是 must-unsafe。工具输出则是 43 个 safe、40 个 may-unsafe 和 7 个 must-unsafe。因此 exact 三分类正确 88 / 90，总体准确率 97.8%。同时，must-unsafe 没有错误，unknown 也是 0。

两个错误都属于 unsafe 假阳性，也就是场景真实安全，但工具因为保守合流报成 may-unsafe。第一个来源是 two_holder 精度问题：两个 holder 的字段内容被合并，导致从 dog_holder 中读取对象时被混入 cat_holder 的 Cat。第二个来源是 field overwrite：由于流不敏感分析不会 kill 旧写入，先写 Cat 再写 Dog 的场景中，真实最后读取应为 Dog，但静态分析会保留 Cat 和 Dog，因此报 may-unsafe。

所以最后的结论是：safe 输出当前可以视为 must-safe，未发现错误；must-unsafe 输出与真实必失败场景一致；may-unsafe 是保守告警，包含真实风险和少量假阳性。如果后续服务 cast erase，最适合优先消费 safe 和 must-unsafe 结果，而 may-unsafe 应保留运行时检查或进入更精细分析。

## 结束语

总结来说，RCPTA 已经完成了从旧 DSL 类级 points-to 分析原型，到新 Lite Class DSL cast site 诊断工具的迁移。当前工具能够稳定恢复 class 对象流和动态类型集合，并在高价值 cast 场景上给出 0 unknown、97.8% exact 三分类准确率的诊断结果。剩余不精确主要来自有意识选择的上下文不敏感和流不敏感策略，表现为少量 may-unsafe 假阳性，而不是 safe 或 must-unsafe 的错误判断。

## 可能问答准备

### Q1：RCPTA 和普通 Rust 类型检查有什么区别？

Rust 类型检查验证静态类型合法性，但 checked downcast 的成败取决于运行时动态类型。RCPTA 关注的是对象流和动态类型集合，目标是静态证明某些 downcast 一定安全、一定失败或可能失败。

### Q2：为什么需要 ClassPAG，而不是直接用底层 PAG？

底层 MIR PAG 会包含大量临时变量、引用路径、标准库实现细节和宏展开代码。ClassPAG 把这些信息提升为源码层 class 对象、字段、wrapper payload、container element 等抽象，使结果可解释，也能直接服务 cast site 诊断。

### Q3：为什么 may-unsafe 不等于工具错误？

may-unsafe 是保守静态分析的预期结果。它表示源动态类型集合中既有满足目标类型的对象，也有不满足的对象。在上下文不敏感和流不敏感条件下，某些真实安全路径会因为对象集合合流而进入 may-unsafe，这是假阳性，但不是 unsound。

### Q4：当前最大的不精确来源是什么？

主要是上下文不敏感、对象不敏感和流不敏感。同一函数不同调用点的对象流可能合并；同一字段在不同对象实例上的内容可能被保守合并；字段多次写入时旧写入不会被 kill。这些选择换来的是实现稳定、分析可控和更高覆盖率。

### Q5：为什么说 unknown 清零很重要？

unknown 通常表示源 points-to 为空、目标类型缺失或类型关系不可判定。对于 cast erase 来说，unknown 不能提供可用优化证据。当前 Cast Risk Matrix 中 unknown 为 0，说明高价值场景中的主要建模缺口已经补齐。

### Q6：safe / must-unsafe 能否直接用于优化？

从当前矩阵结果看，safe 和 must-unsafe 没有发现错误。safe 可以作为消除运行时检查的候选；must-unsafe 可以用于识别必失败路径。但实际接入优化 pass 前仍应保留工程级 guard，例如只对结果完整、类型关系完整、无 unknown 的入口启用。
