// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta: Author: Yan Wang, Date: 2026-02-02

//! Class-level points-to set computation on ClassPAG.
//!
//! Store/Load are **constraints**: when obj flows to base, we materialize store (src -> obj.field)
//! and load (obj.field -> dst). PTS and materialized edges are computed together until fixpoint.

use std::collections::{HashMap, HashSet};

use super::ClassPAG;
use crate::util::class::dsl_inheritance_graph::{build_graph_from_dsl_sources, EdgeKind, NodeKind};

/// Result of class-level points-to analysis: ptr_id -> set of obj_id.
pub type ClassPTS = HashMap<String, HashSet<String>>;

/// Store edge materialized when obj flows to base: src_ptr -> (obj, field).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializedStoreEdge {
    pub src_ptr_id: String,
    pub obj_id: String,
    pub field: String,
}

/// Load edge materialized when obj flows to base: (obj, field) -> dst_ptr.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializedLoadEdge {
    pub obj_id: String,
    pub field: String,
    pub dst_ptr_id: String,
}

/// Result of solve: PTS plus edges materialized from store/load constraints.
#[derive(Debug, Clone)]
pub struct ClassPTSResult {
    pub pts: ClassPTS,
    /// For each cast edge (src_ptr_id, dst_ptr_id), snapshot of src objects before applying that cast edge.
    pub cast_src_before_pts: HashMap<(String, String), HashSet<String>>,
    pub materialized_stores: Vec<MaterializedStoreEdge>,
    pub materialized_loads: Vec<MaterializedLoadEdge>,
}

fn compute_reachable_nodes(adj: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut work = std::collections::VecDeque::new();
    visited.insert(start.to_string());
    work.push_back(start.to_string());
    while let Some(cur) = work.pop_front() {
        if let Some(nexts) = adj.get(&cur) {
            for n in nexts {
                if visited.insert(n.clone()) {
                    work.push_back(n.clone());
                }
            }
        }
    }
    visited
}

fn is_subtype_via_extends(extends_adj: &HashMap<String, Vec<String>>, sub: &str, sup: &str) -> bool {
    if sub == sup {
        return true;
    }
    compute_reachable_nodes(extends_adj, sub).contains(sup)
}

fn implements_interface(
    extends_adj: &HashMap<String, Vec<String>>,
    direct_impl: &HashMap<String, HashSet<String>>,
    concrete: &str,
    iface: &str,
) -> bool {
    let anc = compute_reachable_nodes(extends_adj, concrete);
    for a in &anc {
        if let Some(ifs) = direct_impl.get(a) {
            for i in ifs {
                if i == iface {
                    return true;
                }
                if compute_reachable_nodes(extends_adj, i).contains(iface) {
                    return true;
                }
            }
        }
    }
    false
}

fn has_mixin_view(
    extends_adj: &HashMap<String, Vec<String>>,
    direct_with: &HashMap<String, HashSet<String>>,
    concrete: &str,
    mixin: &str,
) -> bool {
    let anc = compute_reachable_nodes(extends_adj, concrete);
    for a in &anc {
        if let Some(ms) = direct_with.get(a) {
            for m in ms {
                if m == mixin {
                    return true;
                }
            }
        }
    }
    false
}

/// Runs propagation on the ClassPAG until fixpoint.
/// Store/Load are constraints: for each obj in pts[base], we update content[(obj, field)] and
/// create pointer obj.field; materialized store (src -> obj.field) and load (obj.field -> dst)
/// are recorded after convergence.
pub fn solve_class_pts(pag: &ClassPAG) -> ClassPTSResult {
    let mut pts: ClassPTS = HashMap::new();
    let mut cast_src_before_pts: HashMap<(String, String), HashSet<String>> = HashMap::new();
    // (obj_id, field) -> set of obj_id that may be stored in this field
    let mut content: HashMap<(String, String), HashSet<String>> = HashMap::new();

    // Build DSL relation graph once for cast-edge compatibility filtering.
    let dsl_graph = build_graph_from_dsl_sources();
    let mut extends_adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut direct_impl: HashMap<String, HashSet<String>> = HashMap::new();
    let mut direct_with: HashMap<String, HashSet<String>> = HashMap::new();
    for e in &dsl_graph.edges {
        match e.kind {
            EdgeKind::Extends => {
                extends_adj.entry(e.src.clone()).or_default().push(e.dst.clone());
            }
            EdgeKind::Implements => {
                direct_impl.entry(e.src.clone()).or_default().insert(e.dst.clone());
            }
            EdgeKind::With => {
                direct_with.entry(e.src.clone()).or_default().insert(e.dst.clone());
            }
            EdgeKind::MixinOn => {}
        }
    }

    // Initialize: every pointer has an entry; Alloc gives initial points-to.
    for ptr_id in pag.ptr_ids() {
        pts.entry(ptr_id.clone()).or_default();
    }
    for (ptr_id, obj_id) in pag.iter_alloc_edges() {
        pts.get_mut(&ptr_id).unwrap().insert(obj_id);
    }

    // Iterate until fixpoint.
    loop {
        let mut changed = false;

        // Assign: dst may point to whatever src points to
        for (src, dst) in pag.iter_assign_edges() {
            let src_set = pts.get(&src).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(dst.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // Cast: filtered propagation by destination static type (precision-improving).
        for (src, dst) in pag.iter_cast_edges() {
            let src_set = pts.get(&src).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                cast_src_before_pts
                    .entry((src.clone(), dst.clone()))
                    .or_default()
                    .extend(src_set.iter().cloned());
                let dst_ty = pag.get_ptr(&dst).map(|p| p.class_type.clone());
                let dst_kind = dst_ty
                    .as_ref()
                    .map(|t| dsl_graph.nodes.get(t).copied().unwrap_or(NodeKind::Unknown));
                let mut filtered = HashSet::new();
                for obj_id in &src_set {
                    let Some(obj) = pag.get_obj(obj_id) else {
                        continue;
                    };
                    let keep = if let (Some(dt), Some(dk)) = (&dst_ty, dst_kind) {
                        match dk {
                            NodeKind::Class | NodeKind::Unknown => {
                                is_subtype_via_extends(&extends_adj, &obj.class_type, dt)
                            }
                            NodeKind::Interface => {
                                implements_interface(&extends_adj, &direct_impl, &obj.class_type, dt)
                            }
                            NodeKind::Mixin => {
                                has_mixin_view(&extends_adj, &direct_with, &obj.class_type, dt)
                            }
                        }
                    } else {
                        true
                    };
                    if keep {
                        filtered.insert(obj_id.clone());
                    }
                }
                if filtered.is_empty() {
                    continue;
                }
                let d = pts.entry(dst.clone()).or_default();
                let prev = d.len();
                d.extend(filtered);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // CallArg: actual -> formal
        for e in pag.call_arg_edges() {
            let src_set = pts.get(&e.actual_ptr_id).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(e.formal_ptr_id.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // CallRet: formal_ret -> actual_ret (actual_ret may not be in pag.ptr_ids() if from caller not yet in PAG)
        for e in pag.call_ret_edges() {
            let src_set = pts.get(&e.formal_ret_ptr_id).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(e.actual_ret_ptr_id.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // Store constraint: base.field <- src  =>  for each obj in pts[base], content[(obj, field)] += pts[src]
        // (obj.field pointer is created when content is updated; pts[obj.field] synced below)
        for e in pag.iter_store_edges() {
            let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
            let src_objs = pts.get(&e.src_ptr_id).cloned().unwrap_or_default();
            if !base_objs.is_empty() && !src_objs.is_empty() {
                for obj in &base_objs {
                    let key = (obj.clone(), e.field.clone());
                    let c = content.entry(key).or_default();
                    let prev = c.len();
                    c.extend(src_objs.clone());
                    if c.len() > prev {
                        changed = true;
                    }
                }
            }
        }

        // Sync obj.field pointers: pts[obj.field] = content[(obj, field)] for each (obj, field) in content
        for ((obj, field), objs_set) in &content.clone() {
            let obj_field_id = format!("{}.{}", obj, field);
            let d = pts.entry(obj_field_id.clone()).or_default();
            let prev = d.len();
            d.extend(objs_set.iter().cloned());
            if d.len() > prev {
                changed = true;
            }
        }

        // Load constraint: base.field -> dst  =>  for each obj in pts[base], pts[dst] += content[(obj, field)]
        for e in pag.iter_load_edges() {
            let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
            if !base_objs.is_empty() {
                let mut to_add = HashSet::new();
                for obj in &base_objs {
                    let key = (obj.clone(), e.field.clone());
                    if let Some(c) = content.get(&key) {
                        to_add.extend(c.iter().cloned());
                    }
                }
                if !to_add.is_empty() {
                    let d = pts.entry(e.dst_ptr_id.clone()).or_default();
                    let prev = d.len();
                    d.extend(to_add);
                    if d.len() > prev {
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    // After fixpoint: materialize store/load edges from constraints using final pts
    let mut materialized_stores = Vec::new();
    for e in pag.iter_store_edges() {
        let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
        for obj in base_objs {
            materialized_stores.push(MaterializedStoreEdge {
                src_ptr_id: e.src_ptr_id.clone(),
                obj_id: obj,
                field: e.field.clone(),
            });
        }
    }
    let mut materialized_loads = Vec::new();
    for e in pag.iter_load_edges() {
        let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
        for obj in base_objs {
            materialized_loads.push(MaterializedLoadEdge {
                obj_id: obj,
                field: e.field.clone(),
                dst_ptr_id: e.dst_ptr_id.clone(),
            });
        }
    }

    ClassPTSResult {
        pts,
        cast_src_before_pts,
        materialized_stores,
        materialized_loads,
    }
}
