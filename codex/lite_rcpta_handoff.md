# Lite RCPTA Handoff

当前工作区：`/home/wy/rupta_rustdsl_workspace`

## Project Background

正在把旧版 `rcpta` 静态分析工具从 old `rustdsl/classes` DSL 迁移到 `lite_class_dsl`。

`rcpta` 位于：

```text
/home/wy/rupta_rustdsl_workspace/rupta
```

它建立在 RUPTA 的 Rust MIR 指针分析框架之上。目标是支持 lite class DSL 编写的程序：建模 class-level pointer/object，识别构造、upcast/downcast/interface/mixin cast，并根据 points-to 类型范围判断 cast 安全性。

## Old RCPTA Architecture

主要路径：

```text
ClassPAG -> solve_class_pts -> type-info/cast-safety
```

核心文件：

```text
rupta/src/rcpta/class_pag.rs
rupta/src/rcpta/class_pts.rs
rupta/src/util/class/analysis.rs
rupta/src/util/class/dsl_inheritance_graph.rs
rupta/src/util/class/cast_safety_log.rs
rupta/src/builder/fpag_builder.rs
rupta/src/builder/special_function_handler.rs
```

旧 DSL 已经过一次瘦身，仍能分析 old `rustdsl/classes` 测试程序。

## Lite Minimum Test Program

路径：

```text
/home/wy/rupta_rustdsl_workspace/lite_cast_erase/test_programs/min_class_downcast/
```

源码：

```text
src/main.rs
```

结构：

```text
Animal extends Object
Dog extends Animal
Cat extends Animal
```

`main` 创建：

```rust
let animal: CRc<Animal> = Dog::new();
```

然后：

```rust
animal.downcast_rc::<Dog>().is_ok()
animal.downcast_rc::<Cat>().is_ok()
```

期望：

```text
Dog downcast safe
Cat downcast must-unsafe
```

## Completed First Stage Migration

### 1. Lite Type Recognition

修改：

```text
rupta/src/util/class/analysis.rs
```

已支持识别 lite 展开后的类型：

```text
__Dog::__CDog
dyn __Dog::IDog
Rc<dyn __Animal::IAnimal>
Result<Rc<dyn __Dog::IDog>, DowncastError<_>>
```

这些类型会被归一到源层类名：

```text
Dog / Animal / Cat
```

修过一个类名 bug：

```text
不能把 __Dog 错切成 og；
只有 __CDog / __DDog / __SDog / __VDog / __ADog 这类才剥 C/D/S/V/A 前缀。
```

### 2. Lite Inheritance Graph Parsing

修改：

```text
rupta/src/util/class/dsl_inheritance_graph.rs
```

除 old `rustdsl/classes/tests` 外，也扫描：

```text
lite_cast_erase/test_programs
lite_class_dsl/oop_rs/tests
```

支持解析：

```rust
#[class(extends(Animal))]
type Dog = class<...>
```

目前能得到：

```text
Dog -> Animal -> Object
```

### 3. Lite Constructor / Upcast / Downcast Modeling

修改：

```text
rupta/src/builder/special_function_handler.rs
```

支持 lite constructor，例如：

```text
<(dyn __Dog::IDog + 'static)>::new()
```

支持 lite downcast：

```text
IDowncast::downcast_rc
IDowncast::downcast_ref
```

修改：

```text
rupta/src/builder/fpag_builder.rs
```

建模 MIR Unsize cast：

```text
Rc<__Dog::__CDog> -> Rc<dyn __Animal::IAnimal>
```

作为 ClassPAG cast edge。

给普通函数调用也加 class-level CallArg/CallRet：

```text
main::local_4 -> must_succeed_downcast::param_1
main::local_1 -> must_fail_downcast::param_1
```

给普通 wrapper 函数内的 lite downcast 做 summary：

如果 callee 函数体里有：

```text
IDowncast::downcast_rc::<Target>
```

在 caller 侧生成 synthetic dst：

```text
must_succeed_downcast::downcast_Dog
must_fail_downcast::downcast_Cat
```

并添加 ClassPAG cast site。

### 4. Cast Safety Output

修改：

```text
rupta/src/util/class/cast_safety_log.rs
```

允许 lite 测试路径和 `src/main.rs` 输出 cast safety。

当前 `min_class_downcast` 输出：

```text
src/main.rs:25:5 cast is safe
src/main.rs:29:5 cast is unsafe
  unsafe_kind: must-unsafe
  types: src_dynamic_types={Dog} dst_static_type=Cat dst_kind=class
  classification: satisfied_types={} unsatisfied_types={Dog}
  reason: the following src types do not satisfy extends* (class subtype) to dst: {Dog}
```

## Verification Command

因为 Cargo 可能复用 fingerprint 输出，验证前需要 `touch` 测试源文件强制重编译。

当前可靠命令：

```bash
cargo build --manifest-path rupta/Cargo.toml --bin cargo-pta --bin pta

touch lite_cast_erase/test_programs/min_class_downcast/src/main.rs

PTA_FLAGS='["--entry-func","main","--dump-class-pag","analysis_results/rcpta/lite_min_class_downcast/class_pag.txt","--dump-class-pts","analysis_results/rcpta/lite_min_class_downcast/class_pts.txt","--dump-type-info","analysis_results/rcpta/lite_min_class_downcast/type-info.txt","--dump-inheritance-graph","analysis_results/rcpta/lite_min_class_downcast/inheritance_graph.txt","--dump-cast-safety-log","analysis_results/rcpta/lite_min_class_downcast/cast_safety.log"]' \
PTA_CRATE=min_class_downcast \
PTA_TARGET_KIND=bin \
RUSTC_WRAPPER=/home/wy/rupta_rustdsl_workspace/rupta/target/debug/cargo-pta \
RUSTFLAGS='-Z always_encode_mir -C prefer-dynamic' \
cargo check --manifest-path lite_cast_erase/test_programs/min_class_downcast/Cargo.toml --bin min_class_downcast --verbose
```

## Output Directory

因为命令是在测试 crate 工作目录下由 Cargo 执行，结果落在：

```text
/home/wy/rupta_rustdsl_workspace/lite_cast_erase/test_programs/min_class_downcast/analysis_results/rcpta/lite_min_class_downcast/
```

关键输出：

```text
class_pag.txt
class_pts.txt
type-info.txt
inheritance_graph.txt
cast_safety.log
```

## Current Verified Result

`type-info.txt` 关键行：

```text
main::local_1 -> Dog
main::local_2 -> Dog
main::local_4 -> Dog
must_succeed_downcast::param_1 -> Dog
must_fail_downcast::param_1 -> Dog
must_succeed_downcast::downcast_Dog -> Dog
must_fail_downcast::downcast_Cat -> (none)
```

`cast_safety.log`：

```text
src/main.rs:25:5 cast is safe
src/main.rs:29:5 cast is unsafe
  unsafe_kind: must-unsafe
  types: src_dynamic_types={Dog} dst_static_type=Cat dst_kind=class
```

## Modified Files In This Stage

```text
rupta/src/util/class/analysis.rs
rupta/src/util/class/dsl_inheritance_graph.rs
rupta/src/util/class/cast_safety_log.rs
rupta/src/builder/fpag_builder.rs
rupta/src/builder/special_function_handler.rs
```

## Notes

- 工作区有很多历史 dirty/untracked 文件，不要随意 revert。
- `lite_cast_erase/test_programs/min_class_downcast/src/main.rs` 可能显示 modified/untracked，是因为验证时 `touch` 过，内容没有必要改。
- Cargo 有时输出 `attempt to write a readonly database`，这是 Cargo cache last-use 写入警告，分析仍成功。
- 如果使用 `cargo-pta pta` 外层命令，可能因为 Cargo fingerprint 复用旧 stdout 或 `PTA_FLAGS` 造成误读；目前最稳的是上面的 `RUSTC_WRAPPER + PTA_FLAGS` 方式。
- 当前只是第一阶段：`min_class_downcast` 已跑通。

## Suggested Next Tests

下一步建议扩展 lite 测试：

1. subclass chain downcast
2. sibling fail
3. union/branch source type range，形成 may-safe/may-unsafe
4. interface downcast
5. mixin downcast
6. field store/load 后再 downcast
7. 底层 `oop_rs` API 与 surface syntax 混用
