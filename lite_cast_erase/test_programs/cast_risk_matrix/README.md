# Cast Risk Matrix

Focused Lite Class DSL programs for rcpta cast-risk and cast-erase development.

The entries are grouped by the optimization decision that rcpta should report:

- `proven_safe_*`: checked cast can be statically proven safe and is a candidate for low-cost replacement.
- `must_unsafe_*`: checked cast is statically known to fail.
- `may_unsafe_*`: source may contain both satisfying and unsatisfying dynamic types.
- `unknown_should_fix_*`: useful surface-syntax pattern that should eventually be proven, but may expose a missing rcpta summary today.

Each public entry is zero-argument and intended to be passed to rcpta via `--entry-func`.

## Initial Entries

| Entry | Expected decision | Pattern |
| --- | --- | --- |
| `proven_safe_local_downcast` | safe | local concrete allocation, upcast, downcast |
| `proven_safe_helper_return_downcast` | safe | helper return object flow |
| `proven_safe_chained_result_unwrap_downcast` | safe | multilevel downcast through `Result::unwrap` |
| `proven_safe_interface_downcast_ref` | safe | `CRc<Interface>.downcast_ref::<Concrete>()` |
| `proven_safe_mixin_downcast_ref` | safe | class to mixin view through `downcast_ref` |
| `must_unsafe_sibling_downcast` | must-unsafe | sibling class target |
| `must_unsafe_interface_wrong_concrete_downcast_ref` | must-unsafe | interface source, wrong concrete target |
| `may_unsafe_branch_join_downcast` | may-unsafe | helper branch returns `Dog` or `Cat` |
| `may_unsafe_local_branch_join_downcast` | may-unsafe | local branch join then downcast |
| `unknown_should_fix_vec_element_downcast` | currently safe | homogeneous `Vec<CRc<Base>>` element |
| `unknown_should_fix_option_unwrap_source_downcast` | currently safe | `Option<CRc<Base>>::unwrap` source |
| `proven_safe_field_store_load_downcast` | safe | `holder.set().field(...)`, `holder.get().field()`, then downcast |
| `may_unsafe_field_branch_store_load_downcast` | may-unsafe | branch stores `Dog` or `Cat` into one field, then downcast |
| `must_unsafe_field_store_load_downcast` | must-unsafe | field stores `Cat`, then downcast to `Dog` |
| `proven_safe_two_holder_field_precision_downcast` | safe | two holders store different types; read only the `Dog` holder |
| `may_unsafe_field_overwrite_flow_insensitive_downcast` | may-unsafe | `Cat` store overwritten by `Dog`; current flow-insensitive field model keeps both |
| `proven_safe_field_helper_store_load_downcast` | safe | helper stores `Dog`, helper reads field, caller downcasts |
| `may_unsafe_field_helper_branch_store_load_downcast` | may-unsafe | helper stores branch result `Dog` or `Cat`, helper reads field |
| `proven_safe_method_return_field_downcast` | safe | user-defined class method returns stored `Dog` as `CRc<Base>`; caller downcasts |
| `may_unsafe_method_return_field_downcast` | may-unsafe | user-defined class method returns branch-stored `Dog` or `Cat`; caller downcasts |
| `proven_safe_function_arg_passthrough_downcast` | safe | plain function receives and returns `CRc<Base>` carrying one concrete type |
| `may_unsafe_function_arg_passthrough_downcast` | may-unsafe | plain function receives and returns branch-selected `Dog` or `Cat` as `CRc<Base>` |
| `proven_safe_closure_arg_passthrough_downcast` | safe | closure receives and returns `CRc<Base>` carrying one concrete type |
| `may_unsafe_closure_arg_passthrough_downcast` | may-unsafe | closure receives and returns branch-selected `Dog` or `Cat` as `CRc<Base>` |
| `proven_safe_vec_holder_field_downcast` | safe | `Vec<CRc<Holder>>` element read, field load, then downcast |
| `may_unsafe_vec_holder_field_downcast` | may-unsafe | branch-selected `Vec<CRc<Holder>>`, field load may see `Dog` or `Cat` |
| `proven_safe_option_holder_field_downcast` | safe | `Option<CRc<Holder>>::unwrap`, field load, then downcast |
| `may_unsafe_option_holder_field_downcast` | may-unsafe | branch-joined holder wrapped in `Option`, unwrap preserves both field flows |
| `proven_safe_helper_return_vec_holder_field_downcast` | safe | helper returns `Vec<CRc<Holder>>`; caller indexes, loads field, then downcasts |
| `may_unsafe_helper_return_vec_holder_field_downcast` | may-unsafe | helper returns branch-selected holder vector; caller field load may see `Dog` or `Cat` |
| `proven_safe_helper_return_option_holder_field_downcast` | safe | helper returns `Option<CRc<Holder>>`; caller unwraps, loads field, then downcasts |
| `may_unsafe_helper_return_option_holder_field_downcast` | may-unsafe | helper returns branch-joined holder in `Option`; caller unwrap preserves both field flows |
| `proven_safe_helper_interface_view_downcast_ref` | safe | helper returns `CRc<Interface>` from one concrete implementor; caller downcasts ref to that concrete |
| `may_unsafe_helper_interface_view_downcast_ref` | may-unsafe | helper returns interface view from either `RunnerDog` or `RunnerCat`; downcast to `RunnerDog` may fail |
| `proven_safe_vec_interface_view_downcast_ref` | safe | `Vec<CRc<Interface>>` element has one concrete implementor; ref downcast is safe |
| `may_unsafe_vec_interface_view_downcast_ref` | may-unsafe | branch-selected interface vector may contain `RunnerDog` or `RunnerCat` |
| `proven_safe_helper_mixin_view_downcast_ref` | safe | helper returns base class view of `TaggedDog`; caller downcasts ref to `Tagged` mixin |
| `may_unsafe_helper_mixin_view_downcast_ref` | may-unsafe | helper returns either `TaggedDog` or plain `Cat`; mixin downcast may fail |
| `proven_safe_vec_mixin_view_downcast_ref` | safe | `Vec<CRc<Base>>` element is known to have `Tagged` mixin |
| `may_unsafe_vec_mixin_view_downcast_ref` | may-unsafe | branch-selected base vector may contain a class without `Tagged` mixin |
| `proven_safe_helper_result_unwrap_source_downcast` | safe | helper returns `Result<CRc<Base>, E>`; caller unwraps then downcasts |
| `may_unsafe_helper_result_unwrap_source_downcast` | may-unsafe | helper returns `Result` containing branch-selected `Dog` or `Cat` |
| `proven_safe_option_result_double_unwrap_downcast` | safe | helper returns `Option<Result<CRc<Base>, E>>`; double unwrap preserves `Dog` payload |
| `may_unsafe_option_result_double_unwrap_downcast` | may-unsafe | nested `Option<Result<_>>` carries branch-selected `Dog` or `Cat` |
| `proven_safe_result_option_double_unwrap_downcast` | safe | helper returns `Result<Option<CRc<Base>>, E>`; double unwrap preserves `Dog` payload |
| `may_unsafe_result_option_double_unwrap_downcast` | may-unsafe | nested `Result<Option<_>>` carries branch-selected `Dog` or `Cat` |
| `proven_safe_option_ok_or_unwrap_downcast` | safe | `Option<CRc<Base>>::ok_or(...).unwrap()` preserves `Dog` payload |
| `may_unsafe_option_ok_or_unwrap_downcast` | may-unsafe | `ok_or(...).unwrap()` preserves branch-selected `Dog` or `Cat` |
| `proven_safe_option_map_passthrough_downcast` | safe | `Option<CRc<Base>>::map(|x| x).unwrap()` carries one concrete type through a closure |
| `may_unsafe_option_map_passthrough_downcast` | may-unsafe | `Option::map` closure carries branch-selected `Dog` or `Cat` |
| `proven_safe_result_map_passthrough_downcast` | safe | `Result<CRc<Base>, E>::map(|x| x).unwrap()` carries one concrete type through a closure |
| `may_unsafe_result_map_passthrough_downcast` | may-unsafe | `Result::map` closure carries branch-selected `Dog` or `Cat` |
| `proven_safe_option_and_then_passthrough_downcast` | safe | `Option<CRc<Base>>::and_then(|x| Some(x)).unwrap()` carries one concrete type through a closure-returned wrapper |
| `may_unsafe_option_and_then_passthrough_downcast` | may-unsafe | `Option::and_then` closure-returned wrapper carries branch-selected `Dog` or `Cat` |
| `proven_safe_result_and_then_passthrough_downcast` | safe | `Result<CRc<Base>, E>::and_then(|x| Ok(x)).unwrap()` carries one concrete type through a closure-returned wrapper |
| `may_unsafe_result_and_then_passthrough_downcast` | may-unsafe | `Result::and_then` closure-returned wrapper carries branch-selected `Dog` or `Cat` |
| `proven_safe_option_fallback_combinators_downcast` | safe | `Option` fallback family (`or`, `or_else`, `unwrap_or_else`) supplies only `Dog` before downcast |
| `may_unsafe_option_fallback_combinators_downcast` | may-unsafe | `Option` fallback family may supply `Dog` or `Cat` before downcast |
| `proven_safe_result_fallback_combinators_downcast` | safe | `Result` fallback family (`or`, `or_else`, `unwrap_or_else`) supplies only `Dog` before downcast |
| `may_unsafe_result_fallback_combinators_downcast` | may-unsafe | `Result` fallback family may supply `Dog` or `Cat` before downcast |
| `proven_safe_vec_iter_next_downcast` | safe | `Vec<CRc<Base>>::iter().next().unwrap().clone()` carries one concrete type |
| `may_unsafe_vec_iter_next_downcast` | may-unsafe | branch-selected vector element reaches `iter().next()` before downcast |
| `proven_safe_vec_into_iter_next_downcast` | safe | `Vec<CRc<Base>>::into_iter().next().unwrap()` carries one concrete type |
| `may_unsafe_vec_into_iter_next_downcast` | may-unsafe | branch-selected vector element reaches `into_iter().next()` before downcast |
| `proven_safe_vec_iter_map_next_downcast` | safe | iterator `map(|x| x.clone()).next()` preserves a `Dog` element |
| `may_unsafe_vec_iter_map_next_downcast` | may-unsafe | iterator `map` preserves branch-selected `Dog` or `Cat` |
| `proven_safe_vec_iter_find_downcast` | safe | iterator `find(...).unwrap().clone()` returns a known `Dog` element |
| `may_unsafe_vec_iter_find_downcast` | may-unsafe | iterator `find` may return branch-selected `Dog` or `Cat` |
| `proven_safe_vec_into_iter_collect_downcast` | safe | `into_iter().collect::<Vec<_>>()` preserves one concrete element before index/downcast |
| `may_unsafe_vec_into_iter_collect_downcast` | may-unsafe | collected vector may contain branch-selected `Dog` or `Cat` |
| `proven_safe_hashmap_get_downcast` | safe | `HashMap::insert` then `get(...).unwrap().clone()` preserves a `Dog` value |
| `may_unsafe_hashmap_get_downcast` | may-unsafe | `HashMap::get` may return branch-selected `Dog` or `Cat` |
| `must_unsafe_hashmap_get_downcast` | must-unsafe | `HashMap::get` returns only `Cat`, then downcast to `Dog` |
| `proven_safe_hashmap_values_next_downcast` | safe | `HashMap::values().next().unwrap().clone()` preserves a `Dog` value |
| `may_unsafe_hashmap_values_next_downcast` | may-unsafe | `HashMap::values().next()` may return branch-selected `Dog` or `Cat` |
| `proven_safe_btreemap_get_downcast` | safe | `BTreeMap::insert` then `get(...).unwrap().clone()` preserves a `Dog` value |
| `may_unsafe_btreemap_get_downcast` | may-unsafe | `BTreeMap::get` may return branch-selected `Dog` or `Cat` |
| `must_unsafe_btreemap_get_downcast` | must-unsafe | `BTreeMap::get` returns only `Cat`, then downcast to `Dog` |
| `proven_safe_option_as_ref_clone_downcast` | safe | `Option<CRc<Base>>::as_ref().unwrap().clone()` preserves a `Dog` payload |
| `may_unsafe_option_as_ref_clone_downcast` | may-unsafe | `Option::as_ref().unwrap().clone()` preserves branch-selected `Dog` or `Cat` |
| `proven_safe_result_as_ref_clone_downcast` | safe | `Result<CRc<Base>, E>::as_ref().unwrap().clone()` preserves a `Dog` payload |
| `may_unsafe_result_as_ref_clone_downcast` | may-unsafe | `Result::as_ref().unwrap().clone()` preserves branch-selected `Dog` or `Cat` |
| `proven_safe_alias_chain_clone_downcast` | safe | repeated `CRc<Base>::clone()` alias chain preserves a `Dog` source |
| `may_unsafe_alias_chain_clone_downcast` | may-unsafe | repeated `clone()` alias chain preserves branch-selected `Dog` or `Cat` |
| `proven_safe_option_interface_as_ref_clone_downcast_ref` | safe | `Option<CRc<Interface>>::as_ref().unwrap().clone()` then concrete ref downcast |
| `may_unsafe_option_interface_as_ref_clone_downcast_ref` | may-unsafe | `Option` wrapper around branch-selected interface implementor |
| `must_unsafe_option_interface_as_ref_clone_downcast_ref` | must-unsafe | `Option` wrapper contains only the wrong interface implementor |
| `proven_safe_result_mixin_as_ref_clone_downcast_ref` | safe | `Result<CRc<Base>, E>::as_ref().unwrap().clone()` then mixin ref downcast |
| `may_unsafe_result_mixin_as_ref_clone_downcast_ref` | may-unsafe | `Result` wrapper around branch-selected mixin/plain base object |
| `must_unsafe_result_mixin_as_ref_clone_downcast_ref` | must-unsafe | `Result` wrapper contains only a base object without the requested mixin |

The two `unknown_should_fix_*` entries are intentionally named as future pressure tests. They are already handled by the current rcpta build, so new harder variants should be added under this category.
