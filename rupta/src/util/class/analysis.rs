// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class-level analysis utilities for DSL programs.
//! 
//! This module provides utilities to identify and analyze class-related operations
//! in programs that use the classes DSL macro.

use std::rc::Rc;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use log::*;
use crate::mir::function::FunctionReference;

use super::type_system::ClassTypeSystem;

/// Information about a class constructor call
#[derive(Debug, Clone)]
pub struct ClassConstructor {
    /// The class name (e.g., "Point")
    pub class_name: String,
    /// The full function name
    pub func_name: String,
    /// Whether this is the wrapper constructor or the data constructor
    pub is_wrapper: bool,
}

/// Identifies if a function is a class constructor (new operation)
pub fn identify_class_constructor(func_ref: &Rc<FunctionReference>) -> Option<ClassConstructor> {
    let func_name = func_ref.to_string();
    
    // Pattern 1: _classes::_Point::{impl#0}::new (wrapper constructor)
    // Pattern 2: _classes::_Point::data::{impl#0}::new (data constructor)
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        if func_name.contains("::new") {
            let is_wrapper = !func_name.contains("::data::");
            return Some(ClassConstructor {
                class_name,
                func_name,
                is_wrapper,
            });
        }
    }
    
    None
}

/// Extracts class name from a function name
/// 
/// Examples:
/// - "simple_new::_classes::_Point::{impl#0}::new" -> Some("Point")
/// - "simple_new::_classes::_Point::data::{impl#0}::new" -> Some("Point")
pub fn extract_class_name_from_func(func_name: &str) -> Option<String> {
    // Pattern: _classes::_ClassName::...
    if let Some(start) = func_name.find("_classes::_") {
        let after_prefix = &func_name[start + 11..]; // "_classes::_" is 11 chars
        if let Some(end) = after_prefix.find("::") {
            return Some(after_prefix[..end].to_string());
        }
    }
    None
}

/// Canonical name for a class method: strips `::{impl#N}::` and `::data::` so impl wrapper and
/// method body (e.g. _Holder::{impl#1}::get_and_wrap vs _Holder::data::get_and_wrap) share the same prefix.
/// Used for rcpta ClassPAG so we only have one set of param/ret pointers per source-level method (no duplicate param_1/ret).
pub fn canonical_class_method_name(func_name: &str) -> String {
    let mut s = func_name.to_string();
    // Strip ::{impl#N}:: so impl wrapper and trait default share one prefix.
    while let Some(start) = s.find("::{impl#") {
        let after = &s[start + 8..];
        if let Some(close) = after.find('}') {
            if after.get(close..).map_or(false, |r| r.starts_with("}::")) {
                s = format!("{}::{}", &s[..start], &after[close + 3..]);
                continue;
            }
        }
        break;
    }
    // Strip ::data:: so the concrete body (data::method) and impl wrapper share one prefix.
    s = s.replace("::data::", "::");
    s
}

/// Extracts the method name from a DSL function path (e.g. "crate::_classes::_Entity::{impl#0}::apply_twice" -> "apply_twice").
/// Used for rcpta: deduplicate callees by (class_name, method_name) so we only build func PAG once per source-level method.
pub fn extract_method_name_from_func(func_name: &str) -> Option<String> {
    let after_last = func_name.rsplit("::").next()?;
    let method = after_last.split('<').next()?.to_string();
    if method.is_empty() {
        None
    } else {
        Some(method)
    }
}

/// Checks if a function is class-related (belongs to the classes DSL system)
pub fn is_class_related(func_ref: &Rc<FunctionReference>) -> bool {
    let func_name = func_ref.to_string();
    func_name.contains("_classes::_")
}

/// Whether the **caller** is a "source-level allocation context" for rcpta.
/// Only when this is true should we create a ClassObj in ClassPAG for a class constructor Call.
/// Author: Yan Wang, Date: 2026-02-02
///
/// Returns false (do not create rcpta ClassObj) when the call is from:
/// - A class constructor body (caller name contains `_classes::_` and `::new`) — internal Call
/// - DSL internal helpers (e.g. `classes::ptr::` / `into_raw`) — not user-visible allocation
pub fn is_source_level_allocation_caller(caller_func_name: &str) -> bool {
    if caller_func_name.contains("_classes::_") && caller_func_name.contains("::new") {
        return false; // inside a class constructor
    }
    if caller_func_name.contains("classes::ptr::") || caller_func_name.contains("::into_raw") {
        return false; // DSL internal (into_raw etc.)
    }
    true
}

/// Substrings that appear in rustc `def_path_str` for DSL type-view conversions (class / mixin / supertype).
/// Use `::method` style so we do not match e.g. `get_into_super` (would match bare `into_super`).
///
/// Covers macros in `classes` DSL (`into_super`, `into_superclass`, `try_into_subtype`, `cast_mixin`, …)
/// and `classes::ptr::RcDyn::*` delegates. Test suites: `shape_hierarchy`, `animal_hierarchy`, `rcpta_full_hierarchy`, `vehicle_hierarchy`, etc.
pub const DSL_CLASS_CAST_CALLEE_MARKERS: &[&str] = &[
    "::try_into_subtype",
    "::try_into_superclass",
    "::try_into_supertype",
    "::try_cast_mixin",
    "::try_downcast_ty",
    "::try_as_superclass",
    "::try_to_supertype",
    "::try_as_subclass",
    "::cast_mixin_unchecked",
    "::into_superclass_unchecked",
    "::into_superclass",
    "::into_supertype",
    "::cast_mixin",
    "::mixin_to_impl",
    "::into_super",
];

/// True when the **caller** function is a DSL-generated cast/mixin shim (body delegates to `RcDyn` / ptr).
/// Cast edges must be recorded at the user callsite, not inside these wrappers.
pub fn is_dsl_cast_shim_caller(caller_func_name: &str) -> bool {
    DSL_CLASS_CAST_CALLEE_MARKERS
        .iter()
        .any(|m| caller_func_name.contains(m))
}

/// Whether the **caller** is a "source-level cast context" for rcpta.
/// Only when this is true should we add Cast edge and ClassPtr in ClassPAG for a class cast Call.
/// Author: Yan Wang, Date: 2026-02-02
///
/// Returns false (do not add rcpta Cast/ClassPtr) when the call is from:
/// - A DSL cast shim body (same markers as [`identify_class_cast_method`]: upcast/downcast/mixin/…)
/// - Other DSL internal (e.g. classes::ptr::, _delegate_ctor) — not user-visible cast
pub fn is_source_level_cast_caller(caller_func_name: &str) -> bool {
    if is_dsl_cast_shim_caller(caller_func_name) {
        return false;
    }
    if caller_func_name.contains("classes::ptr::") || caller_func_name.contains("_delegate_ctor") {
        return false; // DSL internal
    }
    true
}

/// Whether the **caller** is "source-level context" (user code, not DSL internal).
/// Internal DSL trait methods (e.g. downgrade_from implementing into_superclass) are not source-level:
/// we should not model their params/ret or body edges in ClassPAG.
pub fn is_internal_dsl_trait_method(func_name: &str) -> bool {
    func_name.contains("downgrade_from")
        || func_name.contains("upgrade_from")
        // Macro-generated mixin helper trait impl body (`MixinHasImpl::to_impl`) is internal:
        // we already model `mixin_to_impl` at user callsite, so keeping this body adds orphan
        // param/local/ret pointers (no CallArg/CallRet flow) and pollutes class_pts/type-info.
        || (func_name.contains("::_classes::_")
            && func_name.contains("::to_impl<")
            && func_name.contains("classes::class::Virtual"))
}

/// Use for rcpta ClassPAG when we only want to record edges/pointers from user-visible code
/// (e.g. Assign: only record when not inside ctor body, cast impl, classes::ptr::, etc.).
///
/// Currently we exclude core/std/alloc and only DSL-internal paths under classes
/// (classes::ptr::, classes::vtable::, classes::class::) so we don't create ptrs or edges
/// inside DSL runtime. User code in the classes crate (e.g. tests, impls like get_and_wrap)
/// is included so that callee bodies get Load/Cast/Assign edges in ClassPAG.
/// We also exclude internal trait methods (downgrade_from, upgrade_from) so we do not model
/// their param/ret pointers or body edges at all.
pub fn is_source_level_context(caller_func_name: &str) -> bool {
    if caller_func_name.starts_with("core::")
        || caller_func_name.starts_with("std::")
        || caller_func_name.starts_with("alloc::")
    {
        return false;
    }
    // Exclude only DSL runtime internals, not the whole classes crate (so callee bodies are analyzed).
    if caller_func_name.starts_with("classes::ptr::")
        || caller_func_name.starts_with("classes::vtable::")
        || caller_func_name.starts_with("classes::vtable")
        || caller_func_name.starts_with("classes::class::")
    {
        return false;
    }
    // Do not model pointers/edges inside internal trait method bodies (e.g. downgrade_from).
    if is_internal_dsl_trait_method(caller_func_name) {
        return false;
    }
    is_source_level_allocation_caller(caller_func_name) && is_source_level_cast_caller(caller_func_name)
}

/// Whether the callee is an implementation of `core::clone::Clone::clone` (trait method).
/// Returns true for both the trait method in core and any impl in other crates (e.g. DSL's
/// `impl Clone for RcDyn<C>` in classes). Used so rcpta recognizes DSL-extended clone as Assign.
pub fn is_impl_of_core_clone_trait<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool {
    let Some(trait_did) = tcx.trait_of_item(def_id) else {
        return false;
    };
    let crate_name = tcx.crate_name(trait_did.krate).to_string();
    let path = tcx.def_path_str(trait_did);
    (crate_name == "core" || crate_name == "std") && path.contains("Clone")
}

/// Whether the callee is Option::unwrap (core::option::Option::unwrap or std::option::Option::unwrap).
/// Used for Class PAG: unwrap() on Option<CRc<T>> should add Assign(option_inner_ptr, lhs_ptr).
pub fn is_option_unwrap<'tcx>(tcx: TyCtxt<'tcx>, callee_def_id: DefId) -> bool {
    let path = tcx.def_path_str(callee_def_id);
    (path.contains("option::Option") || path.contains("core::option") || path.contains("std::option"))
        && (path.contains("unwrap") || path.contains("expect"))
}

/// Whether the callee is GetSet::cell_option_set (DSL late field write).
/// Used for Class PAG: when building a callee body (e.g. set_item), add Store(base, field, value).
pub fn is_getset_cell_option_set<'tcx>(tcx: TyCtxt<'tcx>, callee_def_id: DefId) -> bool {
    let path = tcx.def_path_str(callee_def_id);
    path.contains("get_set::GetSet") && path.contains("cell_option_set")
}

/// Whether the callee is GetSet::cell_option_get (DSL late field read).
/// Used for Class PAG: when building a callee body, add Load(base, field, dst).
pub fn is_getset_cell_option_get<'tcx>(tcx: TyCtxt<'tcx>, callee_def_id: DefId) -> bool {
    let path = tcx.def_path_str(callee_def_id);
    path.contains("get_set::GetSet") && path.contains("cell_option_get")
}

/// Whether the callee is the DSL vtable function (classes::ptr::vtable<Type>).
/// Used for rcpta: when we see a vtable call in a callee (e.g. get_and_wrap), resolve it to all
/// methods of the class so we add static_dispatch to get_item, into_superclass, etc., and build their PAG.
pub fn is_dsl_vtable_call<'tcx>(tcx: TyCtxt<'tcx>, callee_def_id: DefId) -> bool {
    let path = tcx.def_path_str(callee_def_id);
    path.contains("vtable") && (path.contains("classes::ptr") || path.contains("classes::"))
}

/// Whether the type is Option<T> where T contains a DSL class (e.g. Option<CRc<Eagle>>).
/// Used for rcpta: when we see dst = move src with this type, record dst -> src so unwrap() can
/// resolve receiver (a copy of the Option) to the original Option holder and find Option.Some.0.
pub fn type_is_option_containing_dsl_class<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    use rustc_middle::ty::GenericArgKind;
    if let TyKind::Adt(def, args) = ty.kind() {
        let path_str = tcx.def_path_str(def.did());
        if (path_str.contains("option::Option") || path_str.contains("core::option") || path_str.contains("std::option"))
            && !args.is_empty()
        {
            if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                return extract_dsl_class_from_wrapper(tcx, inner_ty, None).is_some();
            }
        }
    }
    false
}

/// Whether the callee is a DSL class type-view conversion (same object, different static type / mixin / interface view).
/// rcpta: [`crate::builder::special_function_handler::handle_class_cast_call`]. Author: Yan Wang, Date: 2026-02-02
///
/// Matches [`DSL_CLASS_CAST_CALLEE_MARKERS`] on the callee path, and only when the callee lives under
/// `classes::ptr` or `_classes::_` (user class shims or `RcDyn` runtime).
///
/// For `CRc<T> -> CRc<Interface>` via `Into::into`, see `StdConvertInto` handling in `fpag_builder::resolve_call`
/// (callee is `core::convert`, not `_classes::_`).
pub fn identify_class_cast_method(func_ref: &Rc<FunctionReference>) -> bool {
    let name = func_ref.to_string();
    let is_cast_name = DSL_CLASS_CAST_CALLEE_MARKERS.iter().any(|m| name.contains(m));
    let is_classes = name.contains("classes::ptr") || name.contains("_classes::_");
    is_cast_name && is_classes
}

/// Class name string for a DSL class type (e.g. "Dog", "Animal").
/// Returns None if `ty` is not a DSL class type (path contains `_classes::_`).
pub fn class_name_of_dsl_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<String> {
    if let TyKind::Adt(adt_def, _) = ty.kind() {
        let path = tcx.def_path_str(adt_def.did());
        if let Some(start) = path.find("_classes::_") {
            let after = &path[start + 11..];
            let end = after.find("::").unwrap_or(after.len());
            return Some(after[..end].to_string());
        }
    }
    None
}

/// Information about a getter or setter method
#[derive(Debug, Clone)]
pub struct GetterSetter {
    /// The class name (e.g., "Container")
    pub class_name: String,
    /// The field name (e.g., "point")
    pub field_name: String,
    /// Whether this is a getter (true) or setter (false)
    pub is_getter: bool,
    /// The full function name
    pub func_name: String,
}

/// Identifies if a function is a getter or setter method
/// 
/// A getter/setter method must have the exact pattern: get_<field_name> or set_<field_name>
/// where field_name contains no underscores (single word). Methods like get_internal_point
/// or get_point_sum are NOT getters, they are regular class methods.
/// 
/// Examples:
/// - "simple_load_store::_classes::_Container::{impl#0}::get_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: true })
/// - "simple_load_store::_classes::_Container::{impl#0}::set_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: false })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_internal_point" -> None (not a getter, has underscore in field name)
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point_sum" -> None (not a getter, has underscore in field name)
pub fn identify_getter_setter(func_ref: &Rc<FunctionReference>) -> Option<GetterSetter> {
    let func_name = func_ref.to_string();
    if func_name.contains("set_item") {
        eprintln!("[rcpta] identify_getter_setter checking: {}", func_name);
    }

    // Only user-crate getters/setters; exclude DSL runtime (e.g. GetSet::cell_option_get/set).
    // Those live in classes:: and would otherwise be mis-identified via _classes::_ in generic args.
    if func_name.starts_with("classes::") {
        return None;
    }

    // Pattern: _classes::_ClassName::{impl#N}::get_field_name or set_field_name
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        if func_name.contains("set_item") {
             eprintln!("[rcpta] identify_getter_setter: class_name={}", class_name);
        }
        // Check for get_ prefix: get_X -> field X (allow underscores in X, e.g. get_entity_id -> entity_id).
        // Actual field check is done in flush (get_field_index) so we don't treat get_point_sum as getter.
        if let Some(get_start) = func_name.find("::get_") {
            let after_get = &func_name[get_start + 6..]; // "::get_" is 6 chars
            let field_name_end = after_get.find("::").unwrap_or(after_get.len());
            let field_name = after_get[..field_name_end].to_string();
            if !field_name.is_empty() {
                return Some(GetterSetter {
                    class_name,
                    field_name,
                    is_getter: true,
                    func_name,
                });
            }
        }
        
        // Check for set_ prefix: set_X -> field X (allow underscores in X).
        if let Some(set_start) = func_name.find("::set_") {
            if func_name.contains("set_item") {
                 eprintln!("[rcpta] identify_getter_setter: found ::set_ at {}", set_start);
            }
            let after_set = &func_name[set_start + 6..]; // "::set_" is 6 chars
            let field_name_end = after_set.find("::").unwrap_or(after_set.len());
            let field_name = after_set[..field_name_end].to_string();
            
            if func_name.contains("set_item") {
                 eprintln!("[rcpta] identify_getter_setter: field_name={}", field_name);
            }

            if !field_name.is_empty() {
                return Some(GetterSetter {
                    class_name,
                    field_name,
                    is_getter: false,
                    func_name,
                });
            }
        }
    }
    
    None
}

/// Information about a class method call
#[derive(Debug, Clone)]
pub struct ClassMethod {
    /// The class name (e.g., "Point", "Container")
    pub class_name: String,
    /// The method name (e.g., "sum_coords", "distance_to")
    pub method_name: String,
    /// The full function name
    pub func_name: String,
}

/// Identifies if a function is a class method (not a constructor, getter, or setter)
/// 
/// Examples:
/// - "simple_method_call::_classes::_Point::{impl#0}::sum_coords" -> Some(ClassMethod { class_name: "Point", method_name: "sum_coords" })
/// - "simple_method_call::_classes::_Container::{impl#0}::distance_to" -> Some(ClassMethod { class_name: "Container", method_name: "distance_to" })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_internal_point" -> Some(ClassMethod { class_name: "Container", method_name: "get_internal_point" })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point_sum" -> Some(ClassMethod { class_name: "Container", method_name: "get_point_sum" })
/// - "simple_method_call::_classes::_Point::{impl#0}::new" -> None (constructor)
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point" -> None (getter)
/// - "simple_method_call::_classes::_Container::{impl#0}::set_point" -> None (setter)
pub fn identify_class_method(func_ref: &Rc<FunctionReference>) -> Option<ClassMethod> {
    identify_class_method_impl(func_ref, None)
}

/// Like identify_class_method but only treats get_/set_ as getter/setter when the class has that field.
/// e.g. Entity::get_id is a normal method (Entity has entity_id, not id); Container::get_point is a getter.
pub fn identify_class_method_with_type_system(
    func_ref: &Rc<FunctionReference>,
    type_system: &ClassTypeSystem,
) -> Option<ClassMethod> {
    identify_class_method_impl(func_ref, Some(type_system))
}

fn identify_class_method_impl(
    func_ref: &Rc<FunctionReference>,
    type_system: Option<&ClassTypeSystem>,
) -> Option<ClassMethod> {
    let func_name = func_ref.to_string();
    let is_target = func_name.contains("get_and_wrap") || func_name.contains("get_id");

    // Must be in _classes::_ namespace
    let class_name = match extract_class_name_from_func(&func_name) {
        Some(c) => c,
        None => {
            if is_target {
                log::info!(
                    "rcpta identify_class_method: get_and_wrap/get_id not in _classes::_ func={}",
                    func_name
                );
            }
            return None;
        }
    };

    // Must be a wrapper method (not data::) when type_system is None (legacy).
    // When type_system is Some (rcpta): allow ::data:: so the concrete impl (the one with MIR
    // that we actually call) is recognized and added to static_dispatch_callsites / callee body build.
    if type_system.is_none() && func_name.contains("::data::") {
        return None;
    }

    // Must not be a constructor
    if func_name.contains("::new") {
        return None;
    }

    // Must not be a getter or setter for a field that actually exists on the class.
    // When type_system is provided: only exclude if class has that field (e.g. get_point for Container.point).
    // When type_system is None: exclude any getter/setter pattern (legacy).
    // So Entity::get_id is a class method (Entity has entity_id, not id); get_point for Container is getter.
    if let Some(gs) = identify_getter_setter(func_ref) {
        let exclude = match type_system {
            None => true,
            Some(ts) => ts.get_field_index(&gs.class_name, &gs.field_name).is_some(),
        };
        if is_target {
            log::info!(
                "rcpta identify_class_method: getter_check class={} field={} exclude={}",
                gs.class_name, gs.field_name, exclude
            );
        }
        if exclude {
            return None;
        }
    }

    // Extract method name. Two shapes in MIR/DefPath:
    // (1) With {impl#}: ...::_classes::_ClassName::{impl#N}::method_name or ...::data::{impl#0}::method_name
    // (2) Without {impl#}: ...::_classes::_ClassName::ClassName::<generic>::method_name (e.g. get_and_wrap, get_id)
    //    — rustc sometimes does not emit {impl#} for the impl ClassName<V> { fn method } block.
    // Reject type fragments (e.g. "Entity_Tagged>", "Virtual>>") by requiring method_name to be a plain identifier.
    let method_name = if let Some(impl_start) = func_name.find("::{impl#") {
        let after_impl = &func_name[impl_start + 7..]; // "::{impl#" is 7 chars
        if let Some(impl_end) = after_impl.find("}::") {
            let after_impl_end = &after_impl[impl_end + 3..]; // "}::" is 3 chars
            let m = after_impl_end.to_string();
            if !m.is_empty() && !m.contains("::") && !m.contains('>') && m != "new" {
                Some(m)
            } else {
                if is_target {
                    log::info!(
                        "rcpta identify_class_method: impl# branch rejected m=\"{}\"",
                        m
                    );
                }
                None
            }
        } else {
            if is_target {
                log::info!(
                    "rcpta identify_class_method: impl# branch no }}:: in after_impl=\"{}\"",
                    after_impl
                );
            }
            None
        }
    } else {
        // Fallback for (2): last path segment only if it is a plain method identifier.
        let last = func_name.rsplit("::").next().unwrap_or("");
        if last.is_empty()
            || last == "new"
            || last.contains('<')
            || last.contains('>')
            || last.contains('[')
            || last.contains(']')
            || !last.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            if is_target {
                log::info!(
                    "rcpta identify_class_method: fallback rejected func={} last_segment=\"{}\" (empty/new/bad_char)",
                    func_name, last
                );
            }
            None
        } else {
            Some(last.to_string())
        }
    };
    if let Some(method_name) = method_name {
        return Some(ClassMethod {
            class_name,
            method_name,
            func_name,
        });
    }
    if is_target {
        log::info!(
            "rcpta identify_class_method: returned None (method_name extraction failed) func={}",
            func_name
        );
    }
    None
}

/// Checks if a heap object path represents a class instance
/// This function recursively checks the base path for QualifiedPath and OffsetPath
/// to find the underlying HeapObj
pub fn is_class_instance_heap_obj(
    acx: &crate::mir::analysis_context::AnalysisContext,
    path: &crate::mir::path::PathEnum,
) -> bool {
    use crate::mir::path::{Path, PathEnum};
    
    // Check if this is directly a HeapObj
    if let PathEnum::HeapObj { func_id, location } = path {
        let path_rc = Path::new_heap_obj(*func_id, *location);
        return acx.class_instance_heap_objs.contains(&path_rc);
    }
    
    // For QualifiedPath and OffsetPath, recursively check the base
    let base_path = match path {
        PathEnum::QualifiedPath { base, .. } => Some(base.clone()),
        PathEnum::OffsetPath { base, .. } => Some(base.clone()),
        _ => None,
    };
    
    if let Some(base) = base_path {
        // Recursively check the base path
        return is_class_instance_heap_obj(acx, &base.value);
    }
    
    false
}

/// Checks if a type is a DSL class type
/// 
/// DSL class types are structs defined in the `_classes::_` module.
/// Examples:
/// - `Point<ClassMarker, Virtual>` (wrapper type)
/// - `Point<RcDyn<Point>, Virtual>` (instance type)
/// - `Container<ClassMarker, Virtual>`
/// 
/// This function checks if the type's DefId path contains `_classes::_`.
pub fn is_dsl_class_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    if let TyKind::Adt(adt_def, _args) = ty.kind() {
        // Get the DefId's path string representation
        let def_id = adt_def.did();
        let path = tcx.def_path_str(def_id);
        let is_class = path.contains("_classes::_");
        debug!("is_dsl_class_type: ty={:?}, path={}, result={}", ty, path, is_class);
        is_class
    } else {
        debug!("is_dsl_class_type: ty={:?}, not an Adt, result=false", ty);
        false
    }
}

/// Recursively unwraps wrapper types (ManuallyDrop, Rc, Dyn, etc.) to extract
/// the inner DSL class type, if any.
/// 
/// This function handles the case where DSL class types are wrapped in:
/// - `ManuallyDrop<T>` -> unwraps to T
/// - `Rc<T>` -> unwraps to T
/// - `Dyn<T>` -> unwraps to T
/// - `Cell<T>` -> unwraps to T
/// - `Option<T>` -> unwraps to T
/// 
/// If `field_index` is provided, it will directly access that field after unwrapping
/// to the struct type, instead of traversing all fields.
/// 
/// Returns the inner DSL class type if found, or None if no DSL class type
/// is found after unwrapping all wrappers.
pub fn extract_dsl_class_from_wrapper<'tcx>(
    tcx: TyCtxt<'tcx>, 
    ty: Ty<'tcx>,
    field_index: Option<usize>,
) -> Option<Ty<'tcx>> {
    use rustc_middle::ty::GenericArgKind;
    use crate::util::type_util;
    
    let mut current_ty = ty;
    let mut depth = 0;
    const MAX_DEPTH: usize = 10; // Prevent infinite recursion
    
    while depth < MAX_DEPTH {
        // First, check if current_ty is already a DSL class type
        if is_dsl_class_type(tcx, current_ty) {
            debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} at depth {}", current_ty, depth);
            return Some(current_ty);
        }
        
        // Also check if this is a struct with DSL class type fields
        // For example, Container::data::Container has a point field of type Cell<Option<CRc<Point>>>
        if let TyKind::Adt(adt_def, _args) = current_ty.kind() {
            if adt_def.is_struct() {
                let variant = adt_def.variants().iter().next();
                if let Some(variant) = variant {
                    if let Some(field_idx) = field_index {
                        // If field_index is provided, directly access that specific field
                        if field_idx < variant.fields.len() {
                            let field_ty = type_util::get_field_type(tcx, current_ty, field_idx);
                            debug!("extract_dsl_class_from_wrapper: checking field {} of {:?}", 
                                   field_idx, current_ty);
                            // Recursively check this specific field type
                            if let Some(class_ty) = extract_dsl_class_from_wrapper(tcx, field_ty, None) {
                                debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} in field {} of {:?}", 
                                       class_ty, field_idx, current_ty);
                                return Some(class_ty);
                            }
                        } else {
                            debug!("extract_dsl_class_from_wrapper: field index {} out of bounds for {:?}", 
                                   field_idx, current_ty);
                        }
                    } else {
                        // Fallback: check a limited number of fields if no field_index provided
                        let max_fields_to_check = 5;
                        for (field_idx, _field) in variant.fields.iter().enumerate().take(max_fields_to_check) {
                            let field_ty = type_util::get_field_type(tcx, current_ty, field_idx);
                            // Recursively check this field type
                            if let Some(class_ty) = extract_dsl_class_from_wrapper(tcx, field_ty, None) {
                                debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} in field {} of {:?}", 
                                       class_ty, field_idx, current_ty);
                                return Some(class_ty);
                            }
                        }
                    }
                }
            }
        }
        
        // Try to unwrap various wrapper types
        match current_ty.kind() {
            // ManuallyDrop<T> -> T
            TyKind::Adt(adt_def, args) => {
                let def_id = adt_def.did();
                let path_str = tcx.def_path_str(def_id);
                
                if path_str == "std::mem::ManuallyDrop" || path_str == "core::mem::ManuallyDrop" {
                    // ManuallyDrop has a single type parameter (index 0)
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Rc<T> -> T (including classes::Rc which is a re-export of alloc::rc::Rc)
                else if path_str == "alloc::rc::Rc" || path_str == "std::rc::Rc" || path_str == "classes::Rc" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        debug!("extract_dsl_class_from_wrapper: unwrapping Rc<{:?}>", inner_ty);
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Cell<T> -> T
                else if path_str == "core::cell::Cell" || path_str == "std::cell::Cell" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Option<T> -> T
                else if path_str == "core::option::Option" || path_str == "std::option::Option" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Dyn<T> (from classes::ptr::Dyn)
                else if path_str.contains("classes::ptr::Dyn") {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // RcDyn<T> (from classes::ptr::RcDyn) - unwrap to field 0 type (ManuallyDrop<Rc<Dyn<T::Data>>>)
                // Note: We use field type instead of generic arg because RcDyn's internal structure
                // is { data: ManuallyDrop<Rc<Dyn<C::Data>>>, vtable: ... }
                else if path_str.contains("classes::ptr::RcDyn") {
                    // Get field 0's type (the actual data field)
                    let field_ty = type_util::get_field_type(tcx, current_ty, 0);
                    debug!("extract_dsl_class_from_wrapper: unwrapping RcDyn, field 0 type = {:?}", field_ty);
                    current_ty = field_ty;
                    depth += 1;
                    continue;
                }
            }
            // &T -> T
            TyKind::Ref(_, inner_ty, _) => {
                current_ty = *inner_ty;
                depth += 1;
                continue;
            }
            // *const T or *mut T -> T
            TyKind::RawPtr(inner_ty, _) => {
                current_ty = *inner_ty;
                depth += 1;
                continue;
            }
            _ => {
                // No more wrappers to unwrap
                break;
            }
        }
        
        // If we didn't continue, break the loop
        break;
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_class_name() {
        assert_eq!(
            extract_class_name_from_func("simple_new::_classes::_Point::{impl#0}::new"),
            Some("Point".to_string())
        );
        assert_eq!(
            extract_class_name_from_func("simple_new::_classes::_Point::data::{impl#0}::new"),
            Some("Point".to_string())
        );
        assert_eq!(
            extract_class_name_from_func("simple_new::main"),
            None
        );
    }
}
