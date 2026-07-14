# SESSION SNAPSHOT - 2026-05-10 - CONTAINER MODELING (ENTRY1)

## 1. Scope and Objective
- Focus entry: `test_drivable_interface_polymorphism` (Vehicle suites), one-entry-at-a-time workflow.
- Goal in this phase: clarify and improve container-related RCPTA modeling precision without changing flow-insensitive framework.
- User要求：先明确语义和建模策略，再实现；并区分内部建模与对外dump可读性。

## 2. Re-run and Artifact Status
- Re-ran and overwrote:
  - `analysis_results/rcpta/vehicle_hierarchy/test_drivable_interface_polymorphism`
- Key files updated at `2026-05-10 16:41`:
  - `analysis.log`, `class_pag.txt`, `class_pts.txt`, `type-info.txt`, `inheritance_graph.txt`, `cast_safety.log`, `mir.txt`.

## 3. Entry1 Result Summary
- Main-path behavior is correct:
  - `interfacesDrivable::{drive,stop,turn}::param_1` inferred as `Bicycle, Car, Motorcycle, Truck`.
  - 4 cast sites in `main.rs:73-76` are `safe`.
- Observed anomalies:
  - `slice::into_vec<...>::ret.$elem -> (none)`
  - `slice::into_vec<...>::ret.deref.index -> (none)`
  - `tests::...::local_52 -> (none)`, `tests::...::local_57 -> (none)`

## 4. Root Cause Discussion (Container)

### 4.1 Why callee ret pointers are empty
- Important clarification:
  - `slice::into_vec::ret.$elem / ret.deref.index` are synthetic summary pointers created at call-modeling time (builder), not directly mapped source-level variables.
- Current dataflow shape:
  - Builder creates edges `callee_ret_elem -> caller_dst_elem`.
  - But callee ret summary node is often not fed by internal std/alloc callee-body modeling.
  - Result: callee ret stays empty.
  - Caller can still become non-empty via fallback edges from caller-local summaries.
- Therefore:
  - Not propagation engine failure.
  - It is a summary-anchor feeding gap for std container callee.

### 4.2 Why iterator locals appear as empty pointers
- MIR has real iterator state locals like `Enumerate<Iter<...>>` (`_52`, `_57`).
- These may be registered as class pointers under broad source-level wrapper matching, but they are state carriers, not direct class-object holders.
- They can remain `(none)` without affecting final interface/cast conclusions, but they reduce output readability.

## 5. MIR Sites Identified for Entry1 Container Semantics
- Vec initialization (`vec![...]` lowered):
  - source: `main.rs:72-77`
  - MIR `slice::into_vec`: `mir.txt:1376`
- Iteration chain:
  - `Deref`: `mir.txt:542`
  - `slice::iter`: `mir.txt:547`
  - `Iterator::enumerate`: `mir.txt:551`
  - `IntoIterator::into_iter`: `mir.txt:555`
  - `Iterator::next`: `mir.txt:565`
  - `Some((idx, item))` extraction: `mir.txt:578-579`
- Indexing access:
  - source: `main.rs:124/129/134/139`
  - MIR `Index::index`: `mir.txt:587`

## 6. Agreed Modeling Direction
- Abstract domain agreement:
  - Track meaningful element carriers: `$elem`, `deref.index`, `as_variant#...`.
  - Do not treat iterator state objects as user-level class object pointers in default output.
- Operational principle:
  - Detect specific MIR semantic triggers.
  - Build corresponding pointer nodes/edges in ClassPAG.
  - Preserve flow-insensitive propagation framework.

## 7. Modeling Table (Agreed Output)
- Need two-layer policy:
  1. Internal modeling: keep synthetic summary pointers for sound propagation.
  2. External dump: hide/mark non-source-level synthetic pointers by default.
- Specific categories covered in discussion:
  - `into_vec`, `iter`, `enumerate`, `next`, `Option::Some` extract, `index`, `unwrap/expect`, cast sites.

## 8. Key Decision
- User明确认可：
  - Should patch container modeling gaps.
  - Even if synthetic pointers are useful internally, they should not pollute default `class_pag/class_pts/type-info` as source-level concepts.

## 9. Pending Implementation Plan (Post-Review)
1. Fix `into_vec` main semantic chain so callee ret summary is fed (fallback remains only backup).
2. Unify iterator/next/index element transfer on shared element-summary channel.
3. Add dump-layer filtering/marking for internal synthetic pointers (with optional debug switch to show all).
4. Re-run entry1 and compare before/after precision + readability.

## 10. Notes
- No algorithmic shift to flow-sensitive analysis.
- One-entry-at-a-time validation remains the execution policy for Vehicle suites.
