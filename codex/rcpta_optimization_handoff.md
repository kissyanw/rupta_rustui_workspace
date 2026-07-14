# RCPTA Optimization Handoff

Workspace:

```text
/home/wy/rupta_rustdsl_workspace
```

Use this file to start a new Codex context for continuing the RCPTA cast-risk / cast-erase optimization task.

## Current Goal

Continue expanding high-value lite DSL cast scenarios, then update `rcpta` only when a new test exposes a real modeling gap.

The practical target is:

- discover all relevant checked cast sites expressible in lite DSL surface syntax;
- classify them as `safe`, `may-unsafe`, or `must-unsafe`;
- make `safe` sites explainable from non-empty points-to/type flow;
- eventually provide enough proof to replace high-cost checked casts with lower-cost cast syntax.

The user prefers incremental iterations:

- each iteration should focus on one technical direction;
- data first, rcpta changes second;
- explain the core change after each iteration;
- do not introduce flow-sensitive analysis yet;
- flow-insensitive false positives are acceptable only when clearly explained.

## Main Dataset

Primary dataset:

```text
lite_cast_erase/test_programs/cast_risk_matrix/
```

Important files:

```text
lite_cast_erase/test_programs/cast_risk_matrix/src/main.rs
lite_cast_erase/test_programs/cast_risk_matrix/README.md
```

This directory is currently untracked in git but is central to the task.

## Covered Scenario Families

The matrix currently covers:

- local allocation/upcast then downcast;
- helper return then downcast;
- chained downcast through multiple inheritance levels;
- interface view and mixin view checked casts;
- must-unsafe sibling / wrong concrete target;
- branch join may-unsafe;
- local branch join may-unsafe;
- `Vec<CRc<Base>>` element source;
- `Option<CRc<Base>>::unwrap()` source;
- holder field store/load;
- helper store/load;
- `Vec<CRc<Holder>>` and `Option<CRc<Holder>>`;
- helper returns `Vec<CRc<Holder>>` / `Option<CRc<Holder>>`;
- helper returns interface/mixin views;
- `Result<CRc<Base>, E>::unwrap()` source;
- `Option<Result<CRc<Base>, E>>` double unwrap;
- `Result<Option<CRc<Base>>, E>` double unwrap;
- `Option<CRc<Base>>::ok_or(...).unwrap()`.

Every new high-value family should include safe and may-unsafe pairs when possible.

## Most Recent Iteration

Focus:

```text
Option/Result fallible API boundary around cast sources
```

New helpers in `cast_risk_matrix/src/main.rs`:

```rust
fn dog_result() -> Result<CRc<Animal>, &'static str>
fn choose_animal_result(flag: bool) -> Result<CRc<Animal>, &'static str>
fn dog_option_result() -> Option<Result<CRc<Animal>, &'static str>>
fn choose_animal_option_result(flag: bool) -> Option<Result<CRc<Animal>, &'static str>>
fn dog_result_option() -> Result<Option<CRc<Animal>>, &'static str>
fn choose_animal_result_option(flag: bool) -> Result<Option<CRc<Animal>>, &'static str>
fn dog_option() -> Option<CRc<Animal>>
fn choose_animal_option(flag: bool) -> Option<CRc<Animal>>
```

New entries:

```text
proven_safe_helper_result_unwrap_source_downcast
may_unsafe_helper_result_unwrap_source_downcast
proven_safe_option_result_double_unwrap_downcast
may_unsafe_option_result_double_unwrap_downcast
proven_safe_result_option_double_unwrap_downcast
may_unsafe_result_option_double_unwrap_downcast
proven_safe_option_ok_or_unwrap_downcast
may_unsafe_option_ok_or_unwrap_downcast
```

Expected and verified results:

- all `proven_safe_*` entries are `cast is safe`;
- all `may_unsafe_*` entries are `may-unsafe` with `src_dynamic_types={Cat, Dog}`.

## Why The Recent Tests Initially Failed

The object flow was not lost at allocation or upcast. It was lost inside enum wrapper payload propagation.

Example:

```rust
fn dog_result() -> Result<CRc<Animal>, &'static str> {
    Ok(dog_as_animal())
}

let animal = dog_result().unwrap();
let dog = animal.downcast_rc::<Dog>().unwrap();
```

Before the fix, callee-side payload existed:

```text
dog_result::ret.as_variant#0.0 -> Dog
```

But the call site only had:

```text
dog_result::ret -> caller::local_2
```

It was missing:

```text
dog_result::ret.as_variant#0.0 -> caller::local_2.as_variant#0.0
```

Therefore `Result::unwrap()` read an empty `Ok.0`, and the later downcast source had empty points-to.

The harder case was:

```rust
dog_option_result().unwrap().unwrap()
```

MIR builds:

```text
_1 = Result::Ok(_2)
_0 = Option::Some(_1)
```

Needed payload flow:

```text
_1.Ok.0 -> _0.Some.0.Ok.0
Option<Result<T>>::unwrap(): opt.Some.0.Ok.0 -> result.Ok.0
```

Both were missing before this iteration.

## Recent RCPTA Changes

Primary file:

```text
rupta/src/builder/fpag_builder.rs
```

Added enum payload helpers around line ~79:

```text
rcpta_enum_payload_selector_for_type
rcpta_enum_payload_inner_type
rcpta_enum_payload_routes_for_type
add_rcpta_enum_payload_call_ret
```

Key modeling changes:

- Helper return payload call-ret:

```text
callee::ret.Ok.0 -> caller::dst.Ok.0
callee::ret.Some.0 -> caller::dst.Some.0
callee::ret.Some.0.Ok.0 -> caller::dst.Some.0.Ok.0
```

- Enum aggregate nested payload propagation:

```text
result_local.Ok.0 -> option.Some.0.Ok.0
```

- `Option<Result<T>>::unwrap()` nested payload bridge:

```text
option.Some.0.Ok.0 -> result.Ok.0
```

- `Option::ok_or/ok_or_else` semantic summary:

```text
option.Some.0 -> result.Ok.0
```

These changes do not alter cast safety classification. They only make object/type flow reach the cast source pointer.

## Important Existing RCPTA State

Several earlier changes are already present in `rupta/src/builder/fpag_builder.rs` and related files:

- lite constructor/upcast/downcast modeling;
- unsize cast as ClassPAG cast;
- plain helper call ClassPAG arg/ret summaries;
- `Access::set/get` as same-object views;
- field getter/setter class type inference;
- `Vec` / iterator / element summaries;
- `Option` / `Result` unwrap payload support;
- interface/mixin cast classification;
- synthetic downcast summaries for helper functions containing downcast sites.

Do not revert unrelated existing changes. The worktree is dirty with many user/prior modifications.

## Verification Commands

Build rcpta:

```zsh
cargo build --manifest-path rupta/Cargo.toml --bin cargo-pta --bin pta
```

Check dataset crate:

```zsh
cargo check --offline --manifest-path lite_cast_erase/test_programs/cast_risk_matrix/Cargo.toml
```

Run rcpta for one entry:

```zsh
entry=proven_safe_helper_result_unwrap_source_downcast
out="/home/wy/rupta_rustdsl_workspace/analysis_results/rcpta/cast_risk_matrix/${entry}"
mkdir -p "$out"
PTA_FLAGS="[\"--entry-func\",\"$entry\",\"--analyze-only\",\"--dump-class-pag\",\"$out/class_pag.txt\",\"--dump-class-pts\",\"$out/class_pts.txt\",\"--dump-type-info\",\"$out/type-info.txt\",\"--dump-inheritance-graph\",\"$out/inheritance_graph.txt\",\"--dump-cast-safety-log\",\"$out/cast_safety.log\"]" \
PTA_CRATE=cast_risk_matrix PTA_TARGET_KIND=bin \
RUSTC_WRAPPER=/home/wy/rupta_rustdsl_workspace/rupta/target/debug/cargo-pta \
RUSTFLAGS='-Z always_encode_mir -C prefer-dynamic' \
cargo check --offline --manifest-path lite_cast_erase/test_programs/cast_risk_matrix/Cargo.toml --verbose
```

Use absolute output paths. Relative paths may be resolved under the test crate directory by rustc/cargo.

Useful regression entries:

```text
proven_safe_helper_result_unwrap_source_downcast
may_unsafe_helper_result_unwrap_source_downcast
proven_safe_option_result_double_unwrap_downcast
may_unsafe_option_result_double_unwrap_downcast
proven_safe_result_option_double_unwrap_downcast
may_unsafe_result_option_double_unwrap_downcast
proven_safe_option_ok_or_unwrap_downcast
may_unsafe_option_ok_or_unwrap_downcast
proven_safe_helper_return_option_holder_field_downcast
may_unsafe_helper_return_option_holder_field_downcast
proven_safe_helper_interface_view_downcast_ref
may_unsafe_helper_interface_view_downcast_ref
unknown_should_fix_option_unwrap_source_downcast
unknown_should_fix_vec_element_downcast
```

## Suggested Next Iteration Directions

Pick one direction per iteration. Do not keep all future iterations in the same direction.

High-value remaining scenario ideas:

1. Cast source after method-returned class reference:
   - class method returns `CRc<Base>`;
   - caller downcasts;
   - safe/may-unsafe variants through overriding or helper methods.

2. Cast source after closure/function argument propagation:
   - pass `CRc<Base>` through a plain function/closure;
   - return or store then downcast.

3. Interface/mixin plus fallible wrappers:
   - `Result<CRc<Interface>, E>::unwrap().downcast_ref::<Concrete>()`;
   - `Option<CRc<Base>>::ok_or(...).unwrap().downcast_ref::<Mixin>()`.

4. Must-unsafe expansion:
   - add more must-unsafe pairs for containers, helper returns, interface/mixin targets.

5. Replacement-oriented syntax audit:
   - inspect `oop_rs` unchecked APIs;
   - map safe cast site shapes to possible low-cost replacements;
   - do not implement rewriting until runtime APIs and semantics are confirmed.

## Iteration Termination Standard

Stop the current "lite DSL adaptation and high-value cast-site expansion" phase when:

- `cast_risk_matrix` covers roughly 8-10 major high-value source families;
- each family has safe/may-unsafe pairs where meaningful;
- all checked cast sites in entries appear in `cast_safety.log`;
- safe sites have non-empty, explainable points-to/type flow;
- may-unsafe sites are caused by real mixed dynamic types or known flow-insensitive limits;
- unexplained empty PTS does not appear in ordinary object-flow paths;
- core regression entries remain stable across several iterations.

After that, move to larger dataset construction and Rust-MIR-oriented redesign.

## Current Known Limitations

- The solver is still flow-insensitive.
- Some may-unsafe results are expected when a later store overwrites an earlier unsafe store.
- Do not introduce flow-sensitive analysis yet unless the user explicitly approves that direction.
- The current dataset is still synthetic and much smaller than real Rust MIR workloads.

## Communication Preference

The user wants concise but concrete summaries:

- what scenario was added;
- why it matters for cast optimization;
- what rcpta gap appeared, if any;
- what precise propagation/modeling change fixed it;
- how it was verified.
