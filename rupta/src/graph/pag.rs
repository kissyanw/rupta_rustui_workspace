// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

use log::*;
use petgraph::graph::{DefaultIx, EdgeIndex, NodeIndex};
use petgraph::Graph;
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::Ty;

use super::func_pag::FuncPAG;
use crate::builder::fpag_builder;
use crate::mir::analysis_context::AnalysisContext;
use crate::mir::call_site::CallSiteS;
use crate::mir::function::{FuncId, GenericArgE};
use crate::mir::path::{PathEnum, ProjectionElems};
use crate::util::bit_vec::Idx;
use crate::util::chunked_queue::{self, ChunkedQueue};
use crate::util::class::analysis as class_analysis;

// Unique identifiers for graph node and edges.
pub type PAGNodeId = NodeIndex<DefaultIx>;
pub type PAGEdgeId = EdgeIndex<DefaultIx>;

impl Idx for PAGNodeId {
    #[inline]
    fn new(idx: usize) -> Self {
        NodeIndex::new(idx)
    }

    #[inline]
    fn index(self) -> usize {
        self.index()
    }
}

pub trait PAGPath: Clone + PartialEq + Eq + Hash + Debug {
    type FuncTy;

    fn new_parameter(func: Self::FuncTy, ordinal: usize) -> Self;
    fn new_return_value(func: Self::FuncTy) -> Self;
    fn new_aux_local_path<'tcx>(
        acx: &mut AnalysisContext<'tcx, '_>,
        func: Self::FuncTy,
        ty: Ty<'tcx>,
    ) -> Self;
    fn value(&self) -> &PathEnum;
    fn append_projection(&self, projection_elems: &ProjectionElems) -> Self;
    fn add_offset(&self, offset: usize) -> Self;
    fn dyn_ptr_metadata(&self) -> Self;
    fn remove_cast(&self) -> Self;
    fn cast_to<'tcx>(&self, acx: &mut AnalysisContext<'tcx, '_>, ty: Ty<'tcx>) -> Option<Self>;
    fn type_variant<'tcx>(&self, acx: &mut AnalysisContext<'tcx, '_>, ty: Ty<'tcx>) -> Option<Self>;
    fn regularize(&self, acx: &mut AnalysisContext) -> Self;
    fn try_eval_path_type<'tcx>(&self, acx: &mut AnalysisContext<'tcx, '_>) -> Ty<'tcx>;
    fn set_path_rustc_type<'tcx>(&self, acx: &mut AnalysisContext<'tcx, '_>, ty: Ty<'tcx>);
    fn has_been_cast(&self, acx: &AnalysisContext) -> bool;
    fn concretized_heap_type<'tcx>(&self, acx: &AnalysisContext<'tcx, '_>) -> Option<Ty<'tcx>>;
    fn flatten_fields<'tcx>(self, acx: &mut AnalysisContext<'tcx, '_>) -> Vec<(usize, Self, Ty<'tcx>)>;
    fn get_containing_func(&self) -> Option<Self::FuncTy>;
}

pub struct PAGNode<P: PAGPath> {
    path: P,
}

impl<P: PAGPath> PAGNode<P> {
    pub fn new(path: P) -> Self {
        PAGNode { path }
    }

    /// Returns the path of the node.
    pub fn path(&self) -> &P {
        &self.path
    }
}

pub struct PAGEdge {
    pub kind: PAGEdgeEnum,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PAGEdgeEnum {
    /// Statements that create a reference or a raw pointer to a place.
    AddrPAGEdge,
    /// Statements that create a value by direct assignment, including Move and Copy statements.
    DirectPAGEdge,
    /// Statements that create a value by loading the value pointed by a pointer.
    /// e.g. `_2 = (*_3), _2 = (*_3).0.1`.
    LoadPAGEdge(ProjectionElems),
    /// Statements that store a value to a pointer's pointee.
    /// e.g. `(*_1) = _2, (*_1).0.1 = _2`.
    StorePAGEdge(ProjectionElems),
    /// Similar to GetElementPtr instruction in llvm ir, get an element's address from
    /// a pointed-to object, e.g. `_2 = &((*_3).0.1)`
    GepPAGEdge(ProjectionElems),
    /// Cast a  pointer to another type
    CastPAGEdge,
    /// Statements that offset a pointer.
    OffsetPAGEdge,
}

type EdgeMap = HashMap<PAGNodeId, BTreeSet<PAGEdgeId>>;

pub struct PAG<P: PAGPath> {
    /// The graph structure capturing assignment relations between nodes.
    pub(crate) graph: Graph<PAGNode<P>, PAGEdge>,
    /// A map from values to node id.
    pub(crate) values: HashMap<P, PAGNodeId>,
    /// Maintains a func_pag for each function, so that in context sensitive
    /// analysis we only need to process each function for once.
    pub(crate) func_pags: HashMap<FuncId, FuncPAG>,
    /// Maps each function to a set of related promoted functions.
    pub(crate) promoted_funcs_map: HashMap<FuncId, HashSet<FuncId>>,
    /// Maps each function to a set of related static functions.
    pub(crate) involved_static_funcs_map: HashMap<FuncId, HashSet<FuncId>>,
    // Iterated in pointer analysis. When new function pags are constructed, we
    // put the new addr_edges into this queue to help active new constraints.
    pub(crate) addr_edges_queue: ChunkedQueue<PAGEdgeId>,

    pub(crate) addr_in_edges: EdgeMap,
    pub(crate) addr_out_edges: EdgeMap,
    pub(crate) direct_in_edges: EdgeMap,
    pub(crate) direct_out_edges: EdgeMap,
    pub(crate) load_in_edges: EdgeMap,
    pub(crate) load_out_edges: EdgeMap,
    pub(crate) store_in_edges: EdgeMap,
    pub(crate) store_out_edges: EdgeMap,
    pub(crate) gep_in_edges: EdgeMap,
    pub(crate) gep_out_edges: EdgeMap,
    pub(crate) cast_in_edges: EdgeMap,
    pub(crate) cast_out_edges: EdgeMap,
    pub(crate) offset_in_edges: EdgeMap,
    pub(crate) offset_out_edges: EdgeMap,
}

impl<P: PAGPath> PAG<P> {
    /// Constructor
    pub fn new() -> Self {
        PAG {
            graph: Graph::<PAGNode<P>, PAGEdge>::new(),
            values: HashMap::new(),
            func_pags: HashMap::new(),
            promoted_funcs_map: HashMap::new(),
            involved_static_funcs_map: HashMap::new(),
            addr_edges_queue: ChunkedQueue::new(),

            addr_in_edges: EdgeMap::new(),
            addr_out_edges: EdgeMap::new(),
            direct_in_edges: EdgeMap::new(),
            direct_out_edges: EdgeMap::new(),
            load_in_edges: EdgeMap::new(),
            load_out_edges: EdgeMap::new(),
            store_in_edges: EdgeMap::new(),
            store_out_edges: EdgeMap::new(),
            gep_in_edges: EdgeMap::new(),
            gep_out_edges: EdgeMap::new(),
            cast_in_edges: EdgeMap::new(),
            cast_out_edges: EdgeMap::new(),
            offset_in_edges: EdgeMap::new(),
            offset_out_edges: EdgeMap::new(),
        }
    }

    /// Returns a reference to the pag graph.
    #[inline]
    pub fn graph(&self) -> &Graph<PAGNode<P>, PAGEdge> {
        &self.graph
    }

    /// Return an iterator for the `address_of edges`.
    pub fn addr_edge_iter(&self) -> chunked_queue::IterCopied<PAGEdgeId> {
        self.addr_edges_queue.iter_copied()
    }

    /// Returns the path for the given node_id.
    pub fn node_path(&self, node_id: PAGNodeId) -> &P {
        self.graph.node_weight(node_id).unwrap().path()
    }

    /// Returns the node for the given node_id.
    pub fn get_node(&self, node_id: PAGNodeId) -> &PAGNode<P> {
        self.graph.node_weight(node_id).unwrap()
    }

    /// Returns the node for the given node_id.
    pub fn get_node_mut(&mut self, node_id: PAGNodeId) -> &mut PAGNode<P> {
        self.graph.node_weight_mut(node_id).unwrap()
    }

    /// Returns the node_id for the given path.
    pub fn get_node_id(&self, path: &P) -> Option<PAGNodeId> {
        match self.values.get(path) {
            Some(id) => Some(*id),
            None => None,
        }
    }

    /// Returns the edge for the given edge_id.
    pub fn get_edge(&self, edge_id: PAGEdgeId) -> &PAGEdge {
        self.graph.edge_weight(edge_id).unwrap()
    }

    /// Adds a new node to the pag.
    pub fn add_node(&mut self, path: P) {
        if let Entry::Vacant(e) = self.values.entry(path.clone()) {
            let node = PAGNode::new(path);
            let node_id = self.graph.add_node(node);
            e.insert(node_id);
        }
    }

    /// Helper function to get a node or insert a new
    /// node if it does not exist in the map.
    pub fn get_or_insert_node(&mut self, path: &P) -> PAGNodeId {
        match self.values.entry(path.clone()) {
            Entry::Occupied(o) => o.get().to_owned(),
            Entry::Vacant(v) => {
                let node = PAGNode::new(path.clone());
                let node_id = self.graph.add_node(node);
                *v.insert(node_id)
            }
        }
    }

    /// Returns true if the edge from `src` to `dst` of the `kind` exists.
    pub fn has_edge(&self, src: &P, dst: &P, kind: &PAGEdgeEnum) -> bool {
        match (self.values.get(src), self.values.get(dst)) {
            (Some(src_id), Some(dst_id)) => self.contains_edge(*src_id, *dst_id, kind),
            _ => false,
        }
    }

    /// Returns true if the edge from `src` to `dst` of the `kind` exists.
    pub fn contains_edge(&self, src: PAGNodeId, dst: PAGNodeId, kind: &PAGEdgeEnum) -> bool {
        for edge in self.graph.edges_connecting(src, dst) {
            if &edge.weight().kind == kind {
                return true;
            }
        }
        return false;
    }

    #[inline]
    pub fn add_incoming_addr_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.addr_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_addr_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.addr_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_direct_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.direct_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_direct_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.direct_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_load_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.load_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_load_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.load_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_store_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.store_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_store_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.store_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_gep_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.gep_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_gep_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.gep_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_cast_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.cast_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_cast_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.cast_out_edges.entry(node_id).or_default().insert(out_edge);
    }
    #[inline]
    pub fn add_incoming_offset_edge(&mut self, node_id: PAGNodeId, in_edge: PAGEdgeId) {
        self.offset_in_edges.entry(node_id).or_default().insert(in_edge);
    }
    #[inline]
    pub fn add_outgoing_offset_edge(&mut self, node_id: PAGNodeId, out_edge: PAGEdgeId) {
        self.offset_out_edges.entry(node_id).or_default().insert(out_edge);
    }

    /// Adds an edge from `src` to `dst` according to the edge type.
    /// Returns the edge id if this edge is newly added to the graph.
    pub fn add_edge(&mut self, src: &P, dst: &P, kind: PAGEdgeEnum) -> Option<PAGEdgeId> {
        match kind {
            PAGEdgeEnum::AddrPAGEdge => self.add_addr_edge(src, dst),
            PAGEdgeEnum::DirectPAGEdge => self.add_direct_edge(src, dst),
            PAGEdgeEnum::LoadPAGEdge(..) => self.add_load_edge(src, dst, kind),
            PAGEdgeEnum::StorePAGEdge(..) => self.add_store_edge(src, dst, kind),
            PAGEdgeEnum::GepPAGEdge(..) => self.add_gep_edge(src, dst, kind),
            PAGEdgeEnum::CastPAGEdge => self.add_cast_edge(src, dst),
            PAGEdgeEnum::OffsetPAGEdge => self.add_offset_edge(src, dst),
        }
    }

    pub fn add_addr_edge(&mut self, src: &P, dst: &P) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &PAGEdgeEnum::AddrPAGEdge) {
            let edge = PAGEdge {
                kind: PAGEdgeEnum::AddrPAGEdge,
            };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);
            self.addr_edges_queue.push(edge_id);

            self.add_outgoing_addr_edge(src_id, edge_id);
            self.add_incoming_addr_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    pub fn add_direct_edge(&mut self, src: &P, dst: &P) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &PAGEdgeEnum::DirectPAGEdge) {
            let edge = PAGEdge {
                kind: PAGEdgeEnum::DirectPAGEdge,
            };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_direct_edge(src_id, edge_id);
            self.add_incoming_direct_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    ///author:wy
    ///date:2025-10-29
    ///why field-related edge need kind info?
    pub fn add_load_edge(&mut self, src: &P, dst: &P, kind: PAGEdgeEnum) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &kind) {
            let edge = PAGEdge { kind };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_load_edge(src_id, edge_id);
            self.add_incoming_load_edge(dst_id, edge_id);
            return Some(edge_id);
        }
        None
    }

    pub fn add_store_edge(&mut self, src: &P, dst: &P, kind: PAGEdgeEnum) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &kind) {
            let edge = PAGEdge { kind };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_store_edge(src_id, edge_id);
            self.add_incoming_store_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    pub fn add_gep_edge(&mut self, src: &P, dst: &P, kind: PAGEdgeEnum) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &kind) {
            let edge = PAGEdge { kind };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_gep_edge(src_id, edge_id);
            self.add_incoming_gep_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    pub fn add_cast_edge(&mut self, src: &P, dst: &P) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &PAGEdgeEnum::CastPAGEdge) {
            let edge = PAGEdge {
                kind: PAGEdgeEnum::CastPAGEdge,
            };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_cast_edge(src_id, edge_id);
            self.add_incoming_cast_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    pub fn add_offset_edge(&mut self, src: &P, dst: &P) -> Option<PAGEdgeId> {
        let src_id = self.get_or_insert_node(src);
        let dst_id = self.get_or_insert_node(dst);
        if !self.contains_edge(src_id, dst_id, &PAGEdgeEnum::OffsetPAGEdge) {
            let edge = PAGEdge {
                kind: PAGEdgeEnum::OffsetPAGEdge,
            };
            let edge_id = self.graph.add_edge(src_id, dst_id, edge);

            self.add_outgoing_offset_edge(src_id, edge_id);
            self.add_incoming_offset_edge(dst_id, edge_id);

            return Some(edge_id);
        }
        None
    }

    /// Given two paths, add direct edge between them if they are both of pointer type or add direct
    /// edges between their pointer type fields if any. Return the edges added.
    /// author:wy
    /// date:2025-10-29
    /// add direct edges recursively for pointer type fields
    /// 
    /// For DSL class types (like Point, Container), they are treated as pointers because
    /// they semantically represent references to heap objects, even though they are not
    /// traditional Rust pointer types.
    pub fn add_new_direct_edges<'tcx>(
        &mut self,
        acx: &mut AnalysisContext<'tcx, '_>,
        src: &P,
        dst: &P,
        ty: Ty<'tcx>,
    ) -> Vec<PAGEdgeId> {
        let mut added_edges = Vec::new();
        let mut add_new_direct_edge = |src: &P, dst: &P| {
            if let Some(edge_id) = self.add_direct_edge(src, dst) {
                added_edges.push(edge_id);
            }
        };

        if ty.is_any_ptr() {
            add_new_direct_edge(src, dst);
        } else if class_analysis::is_dsl_class_type(acx.tcx, ty) {
            // DSL class types should be treated as pointers for propagation purposes.
            // In DSL semantics, class instances are heap objects that should propagate
            // through function calls, assignments, etc.
            add_new_direct_edge(src, dst);
        } else {
            let ptr_projs = acx.get_pointer_projections(ty);
            let ptr_projs = unsafe { &*(ptr_projs as *const Vec<(ProjectionElems, Ty<'_>)>) };
            for (ptr_proj, _ptr_ty) in ptr_projs {
                let src_field = src.append_projection(ptr_proj);
                let dst_field = dst.append_projection(ptr_proj);
                add_new_direct_edge(&src_field, &dst_field);
            }
        }
        added_edges
    }

    #[inline]
    pub fn get_func_pag(&self, func_id: &FuncId) -> Option<&FuncPAG> {
        self.func_pags.get(func_id)
    }

    pub fn build_func_pag(&mut self, acx: &mut AnalysisContext<'_, '_>, func_id: FuncId) -> bool {
        if acx.special_functions.contains(&func_id) {
            debug!("build_func_pag skip (special): func_id {:?}", func_id);
            return false;
        }
        let def_id = acx.get_function_reference(func_id).def_id;
        let name = acx.get_function_reference(func_id).to_string();

        // Mitigation for large test harnesses (e.g. proptest-heavy entries in vehicle_hierarchy):
        // When enabled, only analyze MIR for the current crate. Skips non-local functions, which
        // can otherwise explode reachable code and trigger rustc stack overflow during analysis.
        if std::env::var_os("RCPTA_LOCAL_ONLY").is_some() && !def_id.is_local() {
            info!("rcpta: build_func_pag skip (non-local): name={} def_id={:?}", name, def_id);
            return false;
        }
        if !acx.tcx.is_mir_available(def_id) {
            info!(
                "rcpta: build_func_pag skip (no MIR): name={} def_id={:?}",
                name, def_id
            );
            return false;
        }

        if self.func_pags.contains_key(&func_id) {
            return true;
        }

        let gen_args = &acx.get_function_reference(func_id).generic_args;
        if std::env::var_os("RCPTA_TRACE_INIT").is_some() || std::env::var_os("RCPTA_TRACE_FPAG").is_some() {
            let name = acx.get_function_reference(func_id).to_string();
            eprintln!("[rupta][trace] build_func_pag: begin func_id={:?} name={}", func_id, name);
        }
        // Build function pags for promoted functions
        if let Some(promoted_funcs) = self.promote_constants(acx, def_id, gen_args) {
            self.promoted_funcs_map.insert(func_id, promoted_funcs);
        }

        // Build pag for this function.
        let mut fpag = FuncPAG::new(func_id);
        let mir = acx.tcx.optimized_mir(def_id);
        let num_blocks = mir.basic_blocks.len();
        info!(
            "rcpta: build_func_pag MIR ok: name={} def_id={:?} basic_blocks={}",
            name, def_id, num_blocks
        );
        let mut builder = fpag_builder::FuncPAGBuilder::new(acx, func_id, mir, &mut fpag);
        builder.build();
        if std::env::var_os("RCPTA_TRACE_INIT").is_some() || std::env::var_os("RCPTA_TRACE_FPAG").is_some() {
            let name = acx.get_function_reference(func_id).to_string();
            eprintln!("[rupta][trace] build_func_pag: built ok func_id={:?} name={}", func_id, name);
        }

        // Build function pags for static variables encountered in this function.
        let mut static_funcs = HashSet::new();
        for static_variable in &fpag.static_variables_involved {
            if let PathEnum::StaticVariable { def_id } = static_variable.value {
                if let Some(static_func) = self.build_static_pag(acx, def_id) {
                    static_funcs.insert(static_func);
                }
            }
        }
        if !static_funcs.is_empty() {
            self.involved_static_funcs_map.insert(func_id, static_funcs);
        }

        self.func_pags.insert(func_id, fpag);
        true
    }

    /// rcpta: Build PAG for every callee in the call graph (same pointer modeling as entry).
    /// Call after process_reach_funcs. Skip callees already in processed (do not re-push, or queue never empties).
    pub fn build_all_callee_pags(&mut self, acx: &mut AnalysisContext<'_, '_>) {
        let entry_name = acx.tcx.def_path_str(acx.entry_point);
        let prop_mode = entry_name.contains("property_tests::") && entry_name.contains("prop_");
        let worklist: Vec<FuncId> = self.func_pags.keys().copied().collect();
        let mut processed = std::collections::HashSet::new();
        worklist.iter().for_each(|f| { processed.insert(*f); });
        let mut queue = worklist;
        while let Some(func_id) = queue.pop() {
            let callees: Vec<FuncId> = self
                .func_pags
                .get(&func_id)
                .map(|fpag| {
                    fpag.static_dispatch_callsites
                        .iter()
                        .map(|(_, callee_id)| *callee_id)
                        .collect()
                })
                .unwrap_or_default();
            for callee_id in callees {
                if processed.contains(&callee_id) {
                    continue;
                }
                let callee_name = acx.get_function_reference(callee_id).to_string();
                if prop_mode
                    && (callee_name.contains("proptest::")
                        || callee_name.contains("regex_syntax::")
                        || callee_name.contains("rusty_fork::")
                        || callee_name.contains("tempfile::"))
                {
                    processed.insert(callee_id);
                    continue;
                }
                info!("rcpta: build_all_callee_pags trying callee={}", callee_name);
                if self.build_func_pag(acx, callee_id) {
                    processed.insert(callee_id);
                    queue.push(callee_id);
                    info!("rcpta: build_all_callee_pags built ok: callee={}", callee_name);
                } else {
                    processed.insert(callee_id);
                    info!("rcpta: build_all_callee_pags build failed/skip: callee={}", callee_name);
                }
            }
        }
    }

    ///author:wy
    ///date:2025-10-29
    ///def_id: func referencedef id
    ///acx.tcx.instance_mir(def) returns the mir for initialization procedure of the static variable
    pub fn build_static_pag(&mut self, acx: &mut AnalysisContext<'_, '_>, def_id: DefId) -> Option<FuncId> {
        if !acx.tcx.is_mir_available(def_id) {
            warn!("Unavailable mir for static: {:?}", def_id);
            return None;
        }
        let static_variable_ty = acx.tcx.type_of(def_id).skip_binder();
        if !static_variable_ty.is_any_ptr() && acx.get_pointer_projections(static_variable_ty).is_empty() {
            return None;
        }

        let func_id = acx.get_func_id(def_id, acx.tcx.mk_args(&[]));
        if !self.func_pags.contains_key(&func_id) {
            // Build function pags for promoted functions
            if let Some(promoted_funcs) = self.promote_constants(acx, def_id, &vec![]) {
                self.promoted_funcs_map.insert(func_id, promoted_funcs);
            }

            // Build function pags for the static variable/static function
            let mut fpag = FuncPAG::new(func_id);
            // fix 045
            let def = rustc_middle::ty::InstanceKind::Item(def_id);
            let mir = acx.tcx.instance_mir(def);
            let mut builder = fpag_builder::FuncPAGBuilder::new(acx, func_id, mir, &mut fpag);
            builder.build();
            self.func_pags.insert(func_id, fpag);
        }
        Some(func_id)
    }

    ///author:wy
    ///date:2025-10-29
    ///def_id: function reference def id
    ///how to build pag for promoted constants?
    pub fn promote_constants<'tcx>(
        &mut self,
        acx: &mut AnalysisContext<'tcx, '_>,
        def_id: DefId,
        gen_args: &Vec<GenericArgE<'tcx>>,
    ) -> Option<HashSet<FuncId>> {
        let mut promoted_func_ids = HashSet::new();
        //get promoted constants related to the static variable
        for (ordinal, constant_mir) in acx.tcx.promoted_mir(def_id).iter().enumerate() {
            let func_id = acx.get_promoted_id(def_id, gen_args.clone(), ordinal.into());
            promoted_func_ids.insert(func_id);
            if !self.func_pags.contains_key(&func_id) {
                let mut fpag = FuncPAG::new(func_id);
                let mut builder = fpag_builder::FuncPAGBuilder::new(acx, func_id, constant_mir, &mut fpag);
                builder.build();
                self.func_pags.insert(func_id, fpag);
            }
        }
        if promoted_func_ids.is_empty() {
            None
        } else {
            Some(promoted_func_ids)
        }
    }
}

///author:wy
///date:2025-10-29
///add inter-procedural edges for function calls and take recursive operations for fields of pointer types into consideration
impl<P: PAGPath> PAG<P>
where
    P::FuncTy: Into<FuncId> + Copy,
{
    /// Adds direct edges from the arguments to the parameters and from the return value to the destination value.
    pub fn add_inter_procedural_edges(
        &mut self,
        acx: &mut AnalysisContext<'_, '_>,
        callsite: &CallSiteS<P::FuncTy, P>,
        callee: P::FuncTy,
    ) -> Vec<PAGEdgeId> {
        if !acx
            .tcx
            .is_mir_available(acx.get_function_reference(callee.into()).def_id)
        {
            return vec![];
        }

        let mut added_edges = Vec::new();

        // add arg --> param edges.
        for (i, arg) in callsite.args.iter().enumerate() {
            // The source path can be a constant, we did not cache the type information for constants.
            let arg_type = arg.try_eval_path_type(acx);
            let param = PAGPath::new_parameter(callee, i + 1);
            
            // Debug: Check if this is a DSL class type
            if class_analysis::is_dsl_class_type(acx.tcx, arg_type) {
                debug!("DSL class arg propagation: arg={:?} -> param={:?}, type={:?}", 
                       arg, param, arg_type);
            }
            
            added_edges.extend(self.add_new_direct_edges(acx, arg, &param, arg_type));
        }

        // add ret --> dst edge
        // why use destination's type here instead of ret type?
        let ret = PAGPath::new_return_value(callee);
        let ret_type = callsite.destination.try_eval_path_type(acx);
        
        // Debug: Check if return type is DSL class type
        if class_analysis::is_dsl_class_type(acx.tcx, ret_type) {
            debug!("DSL class ret propagation: ret={:?} -> dst={:?}, type={:?}", 
                   ret, callsite.destination, ret_type);
        }
        
        let new_edges = self.add_new_direct_edges(acx, &ret, &callsite.destination, ret_type);
        added_edges.extend(new_edges);

        added_edges
    }
}
