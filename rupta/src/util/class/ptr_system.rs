// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Path-to-ClassPAG pointer-id helpers.
//!
//! The old eager pointer system lived here. The active rcpta pipeline now uses
//! `rcpta::ClassPAG` plus `solve_class_pts`; this module only keeps the shared
//! path naming helper used while building ClassPAG edges.

// ==================== Helper Functions ====================

/// Generate a simplified string representation of a Path for ClassPAG.
/// This creates a human-readable identifier independent of RUPTA's Path abstraction.
/// For Option::Some.0 (path = base + [Downcast(1), Field(0)]), returns the base's id so the
/// Option-holder local (e.g. downcast_to_eagle) is used consistently instead of base.as_variant#1.0.
///
/// When `param_slots` is `Some(s)` (e.g. 1 + num_params from MIR), LocalVariable(ordinal) with
/// ordinal < s is treated as return/params so CallArg formals (param_1, ...) match callee body ptr_ids.
pub fn path_to_class_ptr_id(
    path: &crate::mir::path::Path,
    func_name: Option<&str>,
    param_slots: Option<usize>,
) -> String {
    use crate::mir::path::{PathEnum, PathSelector};
    
    match &path.value {
        PathEnum::LocalVariable { func_id: _, ordinal } => {
            // In callee body, use param_N/ret for parameter slots so they match CallArg formals.
            if let (Some(fn_name), Some(s)) = (func_name, param_slots) {
                if *ordinal < s {
                    if *ordinal == 0 {
                        return format!("{}::ret", fn_name);
                    }
                    return format!("{}::param_{}", fn_name, ordinal);
                }
            }
            if let Some(fn_name) = func_name {
                format!("{}::local_{}", fn_name, ordinal)
            } else {
                format!("local_{}", ordinal)
            }
        }
        PathEnum::Parameter { func_id: _, ordinal } => {
            if let Some(fn_name) = func_name {
                format!("{}::param_{}", fn_name, ordinal)
            } else {
                format!("param_{}", ordinal)
            }
        }
        PathEnum::ReturnValue { func_id: _ } => {
            if let Some(fn_name) = func_name {
                format!("{}::ret", fn_name)
            } else {
                "ret".to_string()
            }
        }
        PathEnum::HeapObj { func_id: _, location } => {
            format!("heap_{:?}", location)
        }
        PathEnum::QualifiedPath { base, projection } => {
            // Option<CRc<T>>.Some.0: use the Option-holder local as the pointer id for consistency.
            if projection.len() == 2 {
                if let PathSelector::Downcast(1) = projection[0] {
                    if let PathSelector::Field(0) = projection[1] {
                        return path_to_class_ptr_id(base, func_name, param_slots);
                    }
                }
            }
            let base_id = path_to_class_ptr_id(base, func_name, param_slots);
            // Simplify projection to field name if possible
            let proj_str = projection.iter()
                .map(|sel| format!("{:?}", sel))
                .collect::<Vec<_>>()
                .join(".");
            format!("{}.{}", base_id, proj_str)
        }
        PathEnum::OffsetPath { base, offset } => {
            let base_id = path_to_class_ptr_id(base, func_name, param_slots);
            format!("{}.ofs({})", base_id, offset)
        }
        _ => {
            format!("{:?}", path)
        }
    }
}
