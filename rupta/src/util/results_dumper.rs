// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

use log::*;
use petgraph::visit::EdgeRef;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

/// Ensures the parent directory of the given file path exists before creating the file.
/// Avoids "No such file or directory" when output path is e.g. analysis_results/rcpta/.../file.txt
/// and cargo-pta runs with a different cwd.
fn ensure_parent_dir(file_path: &str) {
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
}

use crate::graph::pag::{PAGNodeId, PAG, PAGPath};
use crate::graph::call_graph::{CallGraph, CGFunction, CGCallSite, CSCallGraph};
use crate::mir::call_site::{BaseCallSite, CallType};
use crate::mir::context::{Context, ContextId};
use crate::mir::function::FuncId;
use crate::mir::analysis_context::AnalysisContext;
use crate::mir::path::{Path, PathEnum};
use crate::pta::DiffPTDataTy;
use crate::pta::strategies::context_strategy::ContextStrategy;
use crate::pts_set::points_to::PointsToSet;
use crate::util;
use crate::util::class;
use crate::util::class::analysis;
use crate::util::class::{ClassCallGraph, ClassTypeSystem, ClassPtrSystem};
use crate::util::class::dsl_inheritance_graph::dump_inheritance_graph_from_entry_types;
use crate::util::class::cast_safety_log::dump_cast_safety_log;
use crate::rcpta::{solve_class_pts, ClassPAG, ClassPTSResult};

pub fn dump_results<P: PAGPath, F, S>(
    acx: &AnalysisContext, 
    call_graph: &CallGraph<F, S>, 
    pt_data: &DiffPTDataTy, 
    pag: &PAG<P>, 
) where
    F: CGFunction + Into<FuncId>,
    S: CGCallSite + Into<BaseCallSite>,
    <P as PAGPath>::FuncTy: Ord + std::fmt::Debug + Into<FuncId> + Copy
{
    // dump points-to results
    if let Some(pts_output) = &acx.analysis_options.pts_output {
        info!("Dumping points-to results...");
        dump_ci_pts(acx, pt_data, pag, pts_output);
        // dump_pts(pt_data, pag, pts_output);
    }

    // dump call graph
    if let Some(cg_output) = &acx.analysis_options.call_graph_output {
        let cg_path = std::path::Path::new(cg_output);
        info!("Dumping call graph...");
        dump_call_graph(acx, call_graph, cg_path);
    }

    // dump mir for reachable functions
    if let Some(mir_output) = &acx.analysis_options.mir_output {
        info!("Dumping functions' mir...");
        dump_mir(acx, call_graph, mir_output);
    }

    // dump type indices
    // Note: the type indices map is not used to store all the types.
    if let Some(ti_output) = &acx.analysis_options.type_indices_output {
        let ti_path = std::path::Path::new(ti_output);
        info!("Dumping type indices...");
        dump_type_index(acx, ti_path);
    }

    // dump dynamically resolved calls
    if let Some(dyn_calls_output) = &acx.analysis_options.dyn_calls_output {
        info!("Dumping dynamically resolved calls...");
        dump_dyn_calls(acx, call_graph, dyn_calls_output);
    }

    // dump class-level information
    if let Some(class_info_output) = &acx.analysis_options.class_info_output {
        info!("Dumping class-level information...");
        dump_class_info(acx, call_graph, class_info_output);
    }

    // dump class call graph
    if let Some(class_call_graph_output) = &acx.analysis_options.class_call_graph_output {
        info!("Dumping class call graph...");
        dump_class_call_graph(&acx.class_call_graph, class_call_graph_output);
    }

    // dump class type system
    if let Some(class_type_system_output) = &acx.analysis_options.class_type_system_output {
        info!("Dumping class type system...");
        dump_class_type_system(acx, &acx.class_type_system, class_type_system_output);
    }
    
    // dump class pointer system
    if let Some(class_ptr_system_output) = &acx.analysis_options.class_ptr_system_output {
        info!("Dumping class pointer system...");
        dump_class_ptr_system(&acx.class_ptr_system, class_ptr_system_output);
    }

    // dump DSL inheritance graph (direct edges + transitive closure), filtered to entry-related types
    if let Some(path) = &acx.analysis_options.inheritance_graph_output {
        info!("Dumping DSL inheritance graph (direct + closure)...");
        dump_inheritance_graph_from_entry_types(&acx.class_type_system, path);
    }

    // dump rcpta ClassPAG and/or class PTS. Run solver once so PAG can show materialized store/load.
    let class_pag_output = acx.analysis_options.class_pag_output.as_ref();
    let class_pts_output = acx.analysis_options.class_pts_output.as_ref();
    let type_info_output = acx.analysis_options.type_info_output.as_ref();
    let cast_safety_log_output = acx.analysis_options.cast_safety_log_output.as_ref();
    if class_pag_output.is_some()
        || class_pts_output.is_some()
        || type_info_output.is_some()
        || cast_safety_log_output.is_some()
    {
        let result = solve_class_pts(&acx.class_pag);
        if let Some(path) = class_pag_output {
            info!("Dumping rcpta ClassPAG...");
            dump_class_pag(&acx.class_pag, path, Some(&result));
        }
        if let Some(path) = class_pts_output {
            info!("Dumping rcpta class PTS...");
            dump_class_pts_from_result(&acx.class_pag, &result, path);
        }
        if let Some(path) = type_info_output {
            info!("Dumping rcpta inferred type ranges (Type-info)...");
            dump_type_info_from_result(&acx.class_pag, &result, path);
        }
        if let Some(path) = cast_safety_log_output {
            info!("Dumping cast safety log...");
            dump_cast_safety_log(&acx.class_type_system, &acx.class_pag, &result, path);
        }
    }
}


pub fn dump_call_graph<F, S>(
    acx: &AnalysisContext, 
    call_graph: &CallGraph<F, S>, 
    dot_path: &std::path::Path
) where 
    F: CGFunction + Into<FuncId>,
    S: CGCallSite + Into<BaseCallSite>,
{
    let ci_call_graph = to_ci_call_graph(call_graph);
    ci_call_graph.to_dot(acx, dot_path);
}

pub fn dump_type_index(acx: &AnalysisContext, index_path: &std::path::Path) {
    if let Some(parent) = index_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut output = String::new();
    for (i, ty) in acx.type_cache.type_list().iter().enumerate() {
        output.push_str(&format!("{}: {:?}\n", i, ty));
    }
    match std::fs::write(index_path, output) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write index file: {:?}", e),
    };
}

pub fn dump_pts<P: PAGPath>(pt_data: &DiffPTDataTy, pag: &PAG<P>, pts_path: &String) {
    let pts_map = &pt_data.propa_pts_map;
    let mut pts_writer = BufWriter::new(match &pts_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(pts_path);
            Box::new(File::create(pts_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });
    for (node, pts) in pts_map {
        if pts.is_empty() {
            continue;
        }
        let var = pag.node_path(*node);
        pts_writer
            .write_all(format!("{:?} ==> {{ ", var).as_bytes())
            .expect("Unable to write data");
        for pointee in pts {
            pts_writer
                .write_all(format!("{:?} ", pag.node_path(pointee)).as_bytes())
                .expect("Unable to write data");
        }
        pts_writer
            .write_all("}\n".as_bytes())
            .expect("Unable to write data");
    }
}

pub fn dump_pts_for<P: PAGPath>(pt_data: &DiffPTDataTy, pag: &PAG<P>, node_id: PAGNodeId) {
    let path = pag.node_path(node_id);
    println!("Processing node: {:?}, {:?}", node_id, path);
    let pts = pt_data.propa_pts_map.get(&node_id);
    if pts.is_some() {
        let pts = pts.unwrap();
        let mut str = String::new();
        for node in pts {
            str.push_str(&format!("{:?}, ", pag.node_path(node)));
        }
        println!("Points-to: {}", str);
    }
}


/// Helper function to check if a node's points-to set contains a class instance
/// by recursively following PTS relationships.
fn pointees_contain_class_instance<P: PAGPath>(
    acx: &AnalysisContext,
    pt_data: &DiffPTDataTy,
    pag: &PAG<P>,
    node_id: PAGNodeId,
    visited: &mut HashSet<PAGNodeId>,
    max_depth: usize,
) -> bool {
    // Avoid infinite recursion
    if max_depth == 0 {
        return false;
    }
    
    // Avoid cycles
    if visited.contains(&node_id) {
        return false;
    }
    visited.insert(node_id);
    
    // Get the path for this node
    let path = pag.node_path(node_id);
    let path_enum = path.value();
    
    // First, check if the path itself is a class instance (direct check)
    if class::analysis::is_class_instance_heap_obj(acx, path_enum) {
        return true;
    }
    
    // Check if this node's pointees contain a class instance
    if let Some(pointees) = pt_data.get_propa_pts(node_id) {
        for pointee_node_id in pointees.iter() {
            // Recursively check if the pointee is or points to a class instance
            if pointees_contain_class_instance(
                acx,
                pt_data,
                pag,
                pointee_node_id,
                visited,
                max_depth - 1,
            ) {
                return true;
            }
        }
    }
    
    false
}

pub fn dump_ci_pts<P: PAGPath>(acx: &AnalysisContext, pt_data: &DiffPTDataTy, pag: &PAG<P>, grouped_pts_path: &String) {
    // Build mapping from PathEnum to NodeId for efficient lookup
    let mut path_to_node: HashMap<PathEnum, PAGNodeId> = HashMap::new();
    let mut grouped_pts: BTreeMap<FuncId, HashMap<&PathEnum, HashSet<&PathEnum>>> = BTreeMap::new();
    let pts_map = &pt_data.propa_pts_map;
    let mut pts_writer = BufWriter::new(match &grouped_pts_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(grouped_pts_path);
            Box::new(File::create(grouped_pts_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });
    for (node, pts) in pts_map {
        if pts.is_empty() {
            continue;
        }
        let var = pag.node_path(*node);
        let value = var.value();
        // Build PathEnum to NodeId mapping
        path_to_node.insert(value.clone(), *node);
        if let Some(func_id) = path_func_id(value) {
            let pts_map = grouped_pts.entry(func_id).or_default();
            let tmp_pts = pts_map.entry(value).or_default();
            for pointee in pts {
                tmp_pts.insert(pag.node_path(pointee).value());
            }
        }
    }
    for (func_id, pts_map) in grouped_pts {
        pts_writer
            .write_all(format!("{:?} - {:?}\n", func_id, acx.get_function_reference(func_id).to_string()).as_bytes())
            .expect("Unable to write data");
        for (pt, pts) in pts_map {
            // Check if this pointer's pointees contain a class instance
            // (either directly or through propagation)
            let mut visited = HashSet::new();
            let is_class_ref = pts.iter().any(|pointee| {
                if let Some(pointee_node_id) = path_to_node.get(pointee) {
                    visited.clear();
                    pointees_contain_class_instance(acx, pt_data, pag, *pointee_node_id, &mut visited, 10)
                } else {
                    // Fallback: check path structure if node_id not found
                    class::analysis::is_class_instance_heap_obj(acx, pointee)
                }
            });
            
            let pt_label = if is_class_ref {
                format!("{:?} [CLASS_REF]", pt)
            } else {
                format!("{:?}", pt)
            };
            
            pts_writer
                .write_all(format!("\t{} ({:?}) ==> {{ ", pt_label, pts.len()).as_bytes())
                .expect("Unable to write data");
            for pointee in pts {
                let is_class_instance = if let Some(pointee_node_id) = path_to_node.get(pointee) {
                    let mut visited = HashSet::new();
                    pointees_contain_class_instance(acx, pt_data, pag, *pointee_node_id, &mut visited, 10)
                } else {
                    // Fallback: check path structure if node_id not found
                    class::analysis::is_class_instance_heap_obj(acx, pointee)
                };
                let pointee_label = if is_class_instance {
                    format!("{:?} [CLASS_INSTANCE]", pointee)
                } else {
                    format!("{:?}", pointee)
                };
                pts_writer
                    .write_all(format!("{} ", pointee_label).as_bytes())
                    .expect("Unable to write data");
            }
            pts_writer
                .write_all("}\n".as_bytes())
                .expect("Unable to write data");
        }
    }
}

pub fn dump_mir<F: CGFunction + Into<FuncId>, S: CGCallSite>(
    acx: &AnalysisContext, 
    call_graph: &CallGraph<F, S>, 
    mir_path: &String
) {
    // let mut mir_writer = Box::new(File::create(mir_path).expect("Unable to create file")) as Box<dyn Write>;
    let mut mir_writer = match &mir_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(mir_path);
            Box::new(File::create(mir_path).expect("Unable to create file")) as Box<dyn Write>
        }
    };
    let mut visited_func = HashSet::new();
    for func in call_graph.reach_funcs_iter() {
        let func_id = func.into();
        if visited_func.contains(&func_id) {
            continue;
        }
        visited_func.insert(func_id);
        let def_id = acx.get_function_reference(func_id).def_id;
        let func_name = acx.get_function_reference(func_id).to_string();
        mir_writer
            .write_all(format!("[{:?} - {:?}]\n", func_id, func_name).as_bytes())
            .expect("Unable to write data");
        if !acx.tcx.is_mir_available(def_id) {
            mir_writer.write_all(("Mir is unavailable\n").as_bytes()).expect("Unable to write data");
        } else {
            rustc_middle::mir::write_mir_pretty(acx.tcx, Some(def_id), mir_writer.as_mut()).unwrap();
        }
        mir_writer.write_all("\n".as_bytes()).expect("Unable to write data");
    }
}

pub fn dump_dyn_calls<F: CGFunction, S: CGCallSite>(
    acx: &AnalysisContext, 
    call_graph: &CallGraph<F, S>, 
    dyn_calls_path: &String
) where
    F: Into<FuncId>,
    S: Into<BaseCallSite>,
{
    let mut dyn_dispatch_calls: HashMap<BaseCallSite, HashSet<FuncId>> = HashMap::new();
    let mut fnptr_calls: HashMap<BaseCallSite, HashSet<FuncId>> = HashMap::new();
    let mut dyn_fntrait_calls: HashMap<BaseCallSite, HashSet<FuncId>> = HashMap::new();
    for (callsite, call_edges) in &call_graph.callsite_to_edges {
        let callsite_type = call_graph.get_callsite_type(&(*callsite).into()).unwrap();
        match callsite_type {
            CallType::DynamicDispatch => {
                let callees = dyn_dispatch_calls.entry((*callsite).into()).or_default();
                for edge_id in call_edges {
                    let callee_id = call_graph.get_callee_id_of_edge(*edge_id).unwrap();
                    callees.insert(callee_id.into());
                }
            }
            CallType::FnPtr => {
                let callees = fnptr_calls.entry((*callsite).into()).or_default();
                for edge_id in call_edges {
                    let callee_id = call_graph.get_callee_id_of_edge(*edge_id).unwrap();
                    callees.insert(callee_id.into());
                }
            }
            CallType::DynamicFnTrait => {
                let callees = dyn_fntrait_calls.entry((*callsite).into()).or_default();
                for edge_id in call_edges {
                    let callee_id = call_graph.get_callee_id_of_edge(*edge_id).unwrap();
                    callees.insert(callee_id.into());
                }
            }
            _ => {}
        }
    }
    dump_dyn_calls_(
        acx,
        dyn_dispatch_calls,
        fnptr_calls,
        dyn_fntrait_calls,
        dyn_calls_path,
    );
}

fn dump_dyn_calls_(
    acx: &AnalysisContext,
    dyn_dispatch_calls: HashMap<BaseCallSite, HashSet<FuncId>>,
    fnptr_calls: HashMap<BaseCallSite, HashSet<FuncId>>,
    dyn_fntrait_calls: HashMap<BaseCallSite, HashSet<FuncId>>,
    dyn_calls_path: &String,
) {
    let mut dyn_calls_writer = BufWriter::new(match &dyn_calls_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(dyn_calls_path);
            Box::new(File::create(dyn_calls_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    dyn_calls_writer
        .write_all(format!("#Dynamic dispatch calls:\n").as_bytes())
        .expect("Unable to write data");
    for (callsite, callees) in dyn_dispatch_calls {
        let caller_func_ref = acx.get_function_reference(callsite.func);
        dyn_calls_writer
            .write_all(
                format!(
                    "\tcallsite: {:?}, {:?}, callee: \n",
                    caller_func_ref.to_string(),
                    callsite.location
                )
                .as_bytes(),
            )
            .expect("Unable to write data");
        for callee in callees {
            dyn_calls_writer
                .write_all(format!("\t\t{:?}\n", acx.get_function_reference(callee).to_string()).as_bytes())
                .expect("Unable to write data");
        }
    }
    dyn_calls_writer
        .write_all(format!("#Fnptr calls:\n").as_bytes())
        .expect("Unable to write data");
    for (callsite, callees) in fnptr_calls {
        let caller_func_ref = acx.get_function_reference(callsite.func);
        dyn_calls_writer
            .write_all(
                format!(
                    "\tcallsite: {:?}, {:?}, callee: \n",
                    caller_func_ref.to_string(),
                    callsite.location
                )
                .as_bytes(),
            )
            .expect("Unable to write data");
        for callee in callees {
            dyn_calls_writer
                .write_all(format!("\t\t{:?}\n", acx.get_function_reference(callee).to_string()).as_bytes())
                .expect("Unable to write data");
        }
    }
    dyn_calls_writer
        .write_all(format!("#Dynamic Fn* Trait calls:\n").as_bytes())
        .expect("Unable to write data");
    for (callsite, callees) in dyn_fntrait_calls {
        let caller_func_ref = acx.get_function_reference(callsite.func);
        dyn_calls_writer
            .write_all(
                format!(
                    "\tcallsite: {:?}, {:?}, callee: \n",
                    caller_func_ref.to_string(),
                    callsite.location
                )
                .as_bytes(),
            )
            .expect("Unable to write data");
        for callee in callees {
            dyn_calls_writer
                .write_all(format!("\t\t{:?}\n", acx.get_function_reference(callee).to_string()).as_bytes())
                .expect("Unable to write data");
        }
    }
}

pub fn dump_func_contexts(acx: &AnalysisContext, call_graph: &CSCallGraph, ctx_strategy: &impl ContextStrategy, func_ctxts_path: &String) {
    let mut func_ctxts_writer = BufWriter::new(match &func_ctxts_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(func_ctxts_path);
            Box::new(File::create(func_ctxts_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    let mut func_ctxts_map: HashMap<FuncId, HashSet<ContextId>> = HashMap::new();
    for cs_func in call_graph.reach_funcs_iter() {
        func_ctxts_map.entry(cs_func.func_id).or_default().insert(cs_func.cid);
    }
    
    // Sort and print the func_ctxts_map
    let mut sorted_func_ctxts: Vec<(&FuncId, &HashSet<ContextId>)> = func_ctxts_map.iter().collect();
    sorted_func_ctxts.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    for (func_id, ctxts) in sorted_func_ctxts {
        let func_ref = acx.get_function_reference(*func_id);
        let has_self_parameter = util::has_self_parameter(acx.tcx, func_ref.def_id);
        let has_self_ref_parameter = util::has_self_ref_parameter(acx.tcx, func_ref.def_id);
        let ctxts: HashSet<Rc<Context<_>>> = ctxts.iter().map(|ctxt_id| ctx_strategy.get_context_by_id(*ctxt_id)).collect();
        func_ctxts_writer
            .write_all(
                format!(
                    "{:?}, has_self_param: {:?}, has_self_ref_param: {:?}, #ctxts: {:?} \n",
                    func_ref.to_string(),
                    has_self_parameter,
                    has_self_ref_parameter,
                    ctxts.len()
                )
                .as_bytes(),
            )
            .expect("Unable to write data");
        func_ctxts_writer.write_all(format!("\t{:?}\n", ctxts).as_bytes()).expect("Unable to write data");
    }
}

pub fn dump_most_called_funcs<W: Write>(acx: &AnalysisContext, call_graph: &CallGraph<FuncId, BaseCallSite>, stat_writer: &mut BufWriter<W>) {
    let edge_references = call_graph.graph.edge_references();
    let mut call_times_map: HashMap<FuncId, u32> = HashMap::new();
    for edge_ref in edge_references {
        let target = edge_ref.target();
        let callee_id = call_graph.graph.node_weight(target).unwrap().func;
        let count = call_times_map.entry(callee_id).or_insert(0);
        *count += 1;
    }
    let mut vec: Vec<_> = call_times_map.into_iter().collect();
    vec.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    stat_writer
        .write_all("Top-100 called functions: \n".as_bytes())
        .expect("Unable to write data");
    for i in 0..100 {
        let (func_id, called_times) = vec.get(i).unwrap();
        let func_ref = acx.get_function_reference(*func_id);
        stat_writer
            .write_all(format!("\t{:?}: {:?}\n", func_ref.to_string(), called_times).as_bytes())
            .expect("Unable to write data");
    }
}



fn path_func_id(value: &PathEnum) -> Option<FuncId> {
    match value {
        PathEnum::LocalVariable { func_id, .. } 
        | PathEnum::Parameter { func_id, .. } 
        | PathEnum::ReturnValue { func_id } 
        | PathEnum::Auxiliary { func_id, .. } 
        | PathEnum::HeapObj { func_id, .. } => Some(*func_id),
        PathEnum::Constant 
        | PathEnum::StaticVariable { .. } 
        | PathEnum::PromotedConstant { .. } => {
            None
        }
        PathEnum::QualifiedPath { base, .. } 
        | PathEnum::OffsetPath { base, .. } => path_func_id(&base.value),
        PathEnum::Function(..) 
        | PathEnum::PromotedArgumentV1Array 
        | PathEnum::PromotedStrRefArray 
        | PathEnum::Type(..) => None,
    }
}

fn to_ci_call_graph<F, S>(
    call_graph: &CallGraph<F, S>, 
) -> CallGraph<FuncId, BaseCallSite> where 
    F: CGFunction + Into<FuncId>,
    S: CGCallSite + Into<BaseCallSite>,
{
    let mut ci_call_graph = CallGraph::new();
    for (callsite, edges) in &call_graph.callsite_to_edges {
        let ci_callsite: BaseCallSite = callsite.clone().into();
        for edge in edges {
            let (from_id, to_id) = call_graph.graph.edge_endpoints(*edge).unwrap();
            let from_func = call_graph.graph.node_weight(from_id).unwrap().func;
            let to_func = call_graph.graph.node_weight(to_id).unwrap().func;
            ci_call_graph.add_edge(ci_callsite, from_func.into(), to_func.into());
        }
    }
    ci_call_graph
}

/// Dumps class-level information (constructors, instances, etc.)
pub fn dump_class_info<F: CGFunction + Into<FuncId>, S: CGCallSite>(
    acx: &AnalysisContext,
    call_graph: &CallGraph<F, S>,
    class_info_path: &String,
) {
    let mut class_info_writer = BufWriter::new(match &class_info_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(class_info_path);
            Box::new(File::create(class_info_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    // Collect all class constructors
    let mut class_constructors: Vec<(FuncId, class::analysis::ClassConstructor)> = Vec::new();
    let mut visited_funcs = std::collections::HashSet::new();

    for func in call_graph.reach_funcs_iter() {
        let func_id: FuncId = func.into();
        if visited_funcs.contains(&func_id) {
            continue;
        }
        visited_funcs.insert(func_id);

        let func_ref = acx.get_function_reference(func_id);
        if let Some(constructor) = class::analysis::identify_class_constructor(&func_ref) {
            class_constructors.push((func_id, constructor));
        }
    }

    // Write class-level information
    class_info_writer
        .write_all(b"# Class-Level Analysis Results\n")
        .expect("Unable to write data");
    class_info_writer
        .write_all(b"# This file contains class-related information extracted from the analysis\n\n")
        .expect("Unable to write data");

    if class_constructors.is_empty() {
        class_info_writer
            .write_all(b"No class constructors found.\n")
            .expect("Unable to write data");
        return;
    }

    let total_constructors = class_constructors.len();

    // Group by class name
    let mut by_class: std::collections::BTreeMap<String, Vec<(FuncId, class::analysis::ClassConstructor)>> = 
        std::collections::BTreeMap::new();
    for (func_id, constructor) in class_constructors {
        by_class.entry(constructor.class_name.clone())
            .or_default()
            .push((func_id, constructor));
    }

    // Write information for each class
    for (class_name, constructors) in by_class {
        class_info_writer
            .write_all(format!("## Class: {}\n\n", class_name).as_bytes())
            .expect("Unable to write data");

        for (func_id, constructor) in constructors {
            let func_ref = acx.get_function_reference(func_id);
            class_info_writer
                .write_all(format!("### Constructor: {:?}\n", func_id).as_bytes())
                .expect("Unable to write data");
            class_info_writer
                .write_all(format!("  Function: {}\n", func_ref.to_string()).as_bytes())
                .expect("Unable to write data");
            class_info_writer
                .write_all(format!("  Type: {}\n", 
                    if constructor.is_wrapper { "Wrapper" } else { "Data Constructor" }
                ).as_bytes())
                .expect("Unable to write data");
            class_info_writer
                .write_all(b"\n")
                .expect("Unable to write data");
        }
    }

    class_info_writer
        .write_all(format!("\n# Total: {} class constructor(s) found\n", 
            total_constructors).as_bytes())
        .expect("Unable to write data");
}

/// Dumps class call graph (only class method calls, filters DSL internal details)
pub fn dump_class_call_graph(
    class_call_graph: &ClassCallGraph,
    output_path: &String,
) {
    let mut writer = BufWriter::new(match &output_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    // Write DOT format
    let dot_content = class_call_graph.to_dot();
    writer.write_all(dot_content.as_bytes())
        .expect("Unable to write class call graph");

    // Also write text format for readability
    let text_content = class_call_graph.to_text();
    writer.write_all(b"\n\n")
        .expect("Unable to write separator");
    writer.write_all(text_content.as_bytes())
        .expect("Unable to write class call graph text");

    // Write statistics
    let stats = class_call_graph.stats();
    writer.write_all(format!("\n\n{}\n", stats).as_bytes())
        .expect("Unable to write statistics");
}

/// Dumps class type system information
pub fn dump_class_type_system(
    _acx: &AnalysisContext,
    class_type_system: &ClassTypeSystem,
    output_path: &String,
) {
    use std::collections::HashMap;
    
    let mut writer = BufWriter::new(match &output_path[..] {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    writer.write_all(b"Class Type System Report\n")
        .expect("Unable to write header");
    writer.write_all(b"=======================\n\n")
        .expect("Unable to write separator");

    // 1. Class Definitions
    writer.write_all(b"## Class Definitions\n\n")
        .expect("Unable to write section header");
    
    let mut class_names: Vec<_> = class_type_system.get_all_classes().keys().collect();
    class_names.sort();
    
    for class_name in class_names {
        let class_info = class_type_system.get_class(class_name).unwrap();
        
        writer.write_all(format!("### Class: {}\n", class_name).as_bytes())
            .expect("Unable to write class name");
        
        // Inheritance
        if let Some(parent) = &class_info.parent {
            writer.write_all(format!("  Parent: {}\n", parent).as_bytes())
                .expect("Unable to write parent");
        }
        if !class_info.subclasses.is_empty() {
            let mut subclasses: Vec<_> = class_info.subclasses.iter().collect();
            subclasses.sort();
            writer.write_all(format!("  Subclasses: {}\n", subclasses.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")).as_bytes())
                .expect("Unable to write subclasses");
        }
        
        // Fields
        if !class_info.fields.is_empty() {
            writer.write_all(b"  Fields:\n").expect("Unable to write fields header");
            let mut fields: Vec<_> = class_info.fields.values().collect();
            fields.sort_by_key(|f| f.index);
            for field in fields {
                let type_info = field.class_type.as_ref()
                    .map(|t| format!(" -> {}", t))
                    .unwrap_or_else(|| "".to_string());
                writer.write_all(format!("    [{}] {}: index={}{}\n", 
                    field.index, field.name, field.index, type_info).as_bytes())
                    .expect("Unable to write field");
            }
        }
        
        // Methods
        if !class_info.methods.is_empty() {
            writer.write_all(b"  Methods:\n").expect("Unable to write methods header");
            let mut methods: Vec<_> = class_info.methods.iter().collect();
            methods.sort();
            for method in methods {
                writer.write_all(format!("    - {}\n", method).as_bytes())
                    .expect("Unable to write method");
            }
        }
        
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 2. Class Instances
    writer.write_all(b"## Class Instances\n\n")
        .expect("Unable to write section header");
    
    let instances = class_type_system.get_all_instances();
    if instances.is_empty() {
        writer.write_all(b"  (No class instances found)\n\n")
            .expect("Unable to write empty message");
    } else {
        // Group by class name
        let mut by_class: HashMap<String, Vec<&Rc<Path>>> = HashMap::new();
        for (path, class_name) in instances {
            by_class.entry(class_name.clone())
                .or_insert_with(Vec::new)
                .push(path);
        }
        
        let mut class_names: Vec<_> = by_class.keys().collect();
        class_names.sort();
        
        for class_name in class_names {
            writer.write_all(format!("  {}:\n", class_name).as_bytes())
                .expect("Unable to write class name");
            let paths = &by_class[class_name];
            for path in paths {
                writer.write_all(format!("    - {:?}\n", path).as_bytes())
                    .expect("Unable to write instance path");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 3. Class References
    writer.write_all(b"## Class References\n\n")
        .expect("Unable to write section header");
    
    let references = class_type_system.get_all_references();
    if references.is_empty() {
        writer.write_all(b"  (No class references found)\n\n")
            .expect("Unable to write empty message");
    } else {
        // Group by class name
        let mut by_class: HashMap<String, Vec<(&Rc<Path>, bool)>> = HashMap::new();
        for (path, (class_name, is_direct)) in references {
            by_class.entry(class_name.clone())
                .or_insert_with(Vec::new)
                .push((path, *is_direct));
        }
        
        let mut class_names: Vec<_> = by_class.keys().collect();
        class_names.sort();
        
        for class_name in class_names {
            writer.write_all(format!("  {}:\n", class_name).as_bytes())
                .expect("Unable to write class name");
            let refs = &by_class[class_name];
            for (path, is_direct) in refs {
                let ref_type = if *is_direct { "direct" } else { "indirect" };
                writer.write_all(format!("    - {:?} ({})\n", path, ref_type).as_bytes())
                    .expect("Unable to write reference path");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 4. Statistics
    writer.write_all(b"## Statistics\n\n")
        .expect("Unable to write section header");
    let stats = class_type_system.stats();
    writer.write_all(format!("{}\n", stats).as_bytes())
        .expect("Unable to write statistics");
}

pub fn dump_class_ptr_system(class_ptr_system: &ClassPtrSystem, output_path: &str) {
    let mut writer = BufWriter::new(match output_path {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });
    
    writer.write_all(b"# Class Pointer System\n\n")
        .expect("Unable to write header");
    
    // 1. All Pointers
    writer.write_all(b"## All Pointers\n\n")
        .expect("Unable to write section header");
    let ptrs = class_ptr_system.get_all_ptrs();
    if ptrs.is_empty() {
        writer.write_all(b"  (No pointers found)\n\n")
            .expect("Unable to write empty message");
    } else {
        for ptr in ptrs {
            writer.write_all(format!("  - {}\n", ptr).as_bytes())
                .expect("Unable to write pointer");
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }
    
    // 2. All Objects
    writer.write_all(b"## All Objects\n\n")
        .expect("Unable to write section header");
    let objs = class_ptr_system.get_all_objs();
    if objs.is_empty() {
        writer.write_all(b"  (No objects found)\n\n")
            .expect("Unable to write empty message");
    } else {
        for obj in objs {
            writer.write_all(format!("  - {}\n", obj).as_bytes())
                .expect("Unable to write object");
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }
    
    // 3. Points-to Relationships
    writer.write_all(b"## Points-to Relationships\n\n")
        .expect("Unable to write section header");
    let pts = class_ptr_system.get_all_points_to();
    if pts.is_empty() {
        writer.write_all(b"  (No points-to relationships found)\n\n")
            .expect("Unable to write empty message");
    } else {
        for (ptr, objs) in pts {
            writer.write_all(format!("  {} -> {{\n", ptr).as_bytes())
                .expect("Unable to write pointer");
            for obj in objs {
                writer.write_all(format!("    {}\n", obj).as_bytes())
                    .expect("Unable to write object");
            }
            writer.write_all(b"  }\n").expect("Unable to write closing brace");
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }
    
    // 4. Statistics
    writer.write_all(b"## Statistics\n\n")
        .expect("Unable to write section header");
    let stats = class_ptr_system.stats();
    writer.write_all(format!("{}\n", stats).as_bytes())
        .expect("Unable to write statistics");
}

/// Normalizes a function path to a key for grouping: strip leading crate segment so that
/// `rcpta_full_hierarchy::entry_complex_call_chain_demo` and `entry_complex_call_chain_demo` become the same key.
fn normalize_func_key(name: &str) -> String {
    if let Some(i) = name.find("::") {
        name[i + 2..].to_string()
    } else {
        name.to_string()
    }
}

/// Extracts the function scope from a pointer id (e.g. `main::local_1` -> `main`, `Holder::get_and_wrap::param_1` -> `Holder::get_and_wrap`).
/// Used to group edges by the function body they belong to.
/// Result is normalized (leading crate stripped) so it matches caller_from_call_site and all edges for one function land in one section.
fn func_scope_from_ptr_id(ptr_id: &str) -> String {
    let base = ptr_id.split('.').next().unwrap_or(ptr_id);
    let scope = if let Some(i) = base.rfind("::param_") {
        base[..i].to_string()
    } else if let Some(i) = base.rfind("::ret") {
        let after = base.get(i + 5..).unwrap_or("");
        if after.is_empty() || !after.chars().next().map_or(false, |c| c.is_ascii_alphanumeric() || c == '_') {
            base[..i].to_string()
        } else {
            base.to_string()
        }
    } else if let Some(i) = base.rfind("::local_") {
        base[..i].to_string()
    } else {
        base.to_string()
    };
    normalize_func_key(&scope)
}

/// Strips "::{impl#N}::" and "::data::" from a function scope so trait method and impl wrapper
/// map to the same key (one section per source-level method).
fn collapse_impl_and_data_in_scope(scope: &str) -> String {
    let mut s = scope.to_string();
    // Remove ::{impl#0}::, ::{impl#1}::, etc.
    while let Some(off) = s.find("::{impl#") {
        if let Some(close) = s[off..].find("}::") {
            let end = off + close + 3;
            s = format!("{}{}", &s[..off], &s[end..]);
        } else {
            break;
        }
    }
    s = s.replace("::data::", "::");
    s
}

/// rcpta: Canonical section key so that one source-level class method gets one section (merge impl#0 / impl#1 / data::).
/// If scope is a DSL class method (has _classes::_ and we can get class+method), returns "ClassName::method_name".
/// Otherwise normalizes scope by collapsing {impl#N} and data:: so "Entity::{impl#1}::chain_with" and "Entity::chain_with" map to the same key.
fn canonical_section_key_for_scope(scope: &str) -> String {
    if let Some(class) = analysis::extract_class_name_from_func(scope) {
        if let Some(method) = analysis::extract_method_name_from_func(scope) {
            return format!("{}::{}", class.trim_start_matches('_'), method);
        }
    }
    collapse_impl_and_data_in_scope(scope)
}

/// Extracts the caller function from a call_site id.
/// Format is `[crate::]func_name:bbN[M]` (e.g. `rcpta_full_hierarchy::entry_complex_call_chain_demo:bb8[1]`).
/// Returns the function name normalized (leading crate stripped) so it matches func_scope_from_ptr_id and all edges land in one section.
fn caller_from_call_site(call_site: &str) -> String {
    let before_bb = if let Some(i) = call_site.rfind(':') {
        &call_site[..i]
    } else {
        call_site
    };
    normalize_func_key(before_bb)
}

/// Extracts callee scope from a formal ptr id (e.g. `entityEntity::with_partner::param_1` -> `entityEntity::with_partner`,
/// `entityEntity::with_partner::ret` -> `entityEntity::with_partner`). Used to detect "wrapper -> same-name body" calls.
fn callee_scope_from_formal_ptr_id(id: &str) -> Option<String> {
    if id.ends_with("::ret") {
        return Some(id[..id.len() - 5].to_string());
    }
    if let Some(i) = id.rfind("::param_") {
        return Some(id[..i].to_string());
    }
    None
}

/// True iff this CallArg/CallRet should be hidden from ClassPAG output: only when the call is
/// "impl wrapper calling the same-named method body" (MIR artifact, not visible in source).
fn is_wrapper_to_same_method_edge(caller_scope: &str, callee_scope: Option<&str>) -> bool {
    if !caller_scope.contains("::{impl#") {
        return false;
    }
    let Some(callee) = callee_scope else { return false };
    canonical_section_key_for_scope(caller_scope) == canonical_section_key_for_scope(callee)
}

/// Shortens ClassPAG pointer/call_site ids for human-readable output.
/// - Strips leading crate (first path segment).
/// - Collapses _classes::_Entity -> Entity, {impl#0} -> removed, ::data:: -> ::
fn short_class_pag_name(id: &str) -> String {
    let mut s = id.to_string();
    // Strip crate prefix: "crate::rest" -> "rest"
    if let Some(i) = s.find("::") {
        s = s[i + 2..].to_string();
    }
    // _classes::_ClassName -> ClassName
    s = s.replace("::_classes::_", "");
    while let Some(off) = s.find("::_") {
        let rest = &s[off + 3..];
        if rest.starts_with(|c: char| c.is_ascii_uppercase()) {
            s = format!("{}::{}", &s[..off + 2], rest);
        } else {
            break;
        }
    }
    // Strip any ::{impl#N}:: so impl wrapper and body display the same (no duplicate pointer lines in PTS).
    while let Some(off) = s.find("::{impl#") {
        let rest = &s[off + 8..];
        if let Some(close) = rest.find('}') {
            if rest.get(close..).map_or(false, |r| r.starts_with("}::")) {
                s = format!("{}::{}", &s[..off], &rest[close + 3..]);
                continue;
            }
        }
        break;
    }
    s = s.replace("::data::", "::");
    s
}

#[inline]
fn is_internal_rcpta_container_ptr_id(id: &str) -> bool {
    // Internal summary pointers used for container bridging.
    // Keep them in analysis/propagation but hide from default dumps.
    id.ends_with("::ret.$elem") || id.ends_with("::ret.deref.index")
}

fn collect_hidden_ptr_ids(
    class_pag: &ClassPAG,
    solver_result: Option<&ClassPTSResult>,
) -> HashSet<String> {
    let mut hidden: HashSet<String> = HashSet::new();
    for id in class_pag.ptr_ids() {
        if is_internal_rcpta_container_ptr_id(id) {
            hidden.insert(id.clone());
        }
    }

    // Hide iterator-state temporaries from default dumps.
    // These locals are not source-level class references; they are MIR state carriers
    // (e.g. Enumerate<Iter<...>>), often mis-typed into class pointers by conservative modeling.
    if let Some(result) = solver_result {
        let mut assign_preds: HashMap<String, HashSet<String>> = HashMap::new();
        let mut assign_succs: HashMap<String, HashSet<String>> = HashMap::new();
        let mut in_cast: HashSet<String> = HashSet::new();
        let mut in_callarg: HashSet<String> = HashSet::new();
        let mut in_callret: HashSet<String> = HashSet::new();
        let mut in_alloc: HashSet<String> = HashSet::new();
        let mut in_load_store: HashSet<String> = HashSet::new();

        for (src, dst) in class_pag.iter_assign_edges() {
            assign_preds
                .entry(dst.clone())
                .or_default()
                .insert(src.clone());
            assign_succs.entry(src).or_default().insert(dst);
        }
        for (src, dst) in class_pag.iter_cast_edges() {
            in_cast.insert(src);
            in_cast.insert(dst);
        }
        for e in class_pag.call_arg_edges() {
            in_callarg.insert(e.actual_ptr_id.clone());
            in_callarg.insert(e.formal_ptr_id.clone());
        }
        for e in class_pag.call_ret_edges() {
            in_callret.insert(e.formal_ret_ptr_id.clone());
            in_callret.insert(e.actual_ret_ptr_id.clone());
        }
        for (ptr, _) in class_pag.iter_alloc_edges() {
            in_alloc.insert(ptr);
        }
        for e in class_pag.iter_load_edges() {
            in_load_store.insert(e.base_ptr_id.clone());
            in_load_store.insert(e.dst_ptr_id.clone());
        }
        for e in class_pag.iter_store_edges() {
            in_load_store.insert(e.base_ptr_id.clone());
            in_load_store.insert(e.src_ptr_id.clone());
        }

        let mut iterator_state_candidates: HashSet<String> = HashSet::new();
        for id in class_pag.ptr_ids() {
            let is_plain_local = id.contains("::local_")
                && !id.contains(".as_variant#")
                && !id.ends_with(".$elem")
                && !id.ends_with(".deref.index");
            if !is_plain_local {
                continue;
            }
            let pts_empty = result
                .pts
                .get(id)
                .map(|s| s.is_empty())
                .unwrap_or(true);
            let has_structural_role = in_cast.contains(id)
                || in_callarg.contains(id)
                || in_callret.contains(id)
                || in_alloc.contains(id)
                || in_load_store.contains(id);
            if pts_empty && !has_structural_role {
                iterator_state_candidates.insert(id.clone());
            }
        }

        // Fixed-point: hide iterator-state candidates that have no semantic-source predecessor.
        // A node is hidden when all its assign predecessors are already hidden/candidates and
        // no predecessor carries concrete points-to objects.
        let mut changed = true;
        while changed {
            changed = false;
            for id in iterator_state_candidates.iter() {
                if hidden.contains(id) {
                    continue;
                }
                let preds = assign_preds.get(id);
                let has_semantic_pred = preds
                    .map(|preds| {
                        preds.iter().any(|p| {
                            if hidden.contains(p) {
                                return false;
                            }
                            if iterator_state_candidates.contains(p) {
                                return false;
                            }
                            result
                                .pts
                                .get(p)
                                .map(|s| !s.is_empty())
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false);
                if !has_semantic_pred {
                    hidden.insert(id.clone());
                    changed = true;
                }
            }
        }
    }
    hidden
}

#[inline]
fn should_dump_ptr_id(id: &str, hidden_ptrs: &HashSet<String>) -> bool {
    !hidden_ptrs.contains(id)
}

#[inline]
fn should_dump_ptr_edge(src: &str, dst: &str, hidden_ptrs: &HashSet<String>) -> bool {
    should_dump_ptr_id(src, hidden_ptrs) && should_dump_ptr_id(dst, hidden_ptrs)
}

/// Dumps rcpta ClassPAG (class-level pointer flow graph): ptrs, objs, assign/alloc/load/store/call edges.
/// When solver_result is Some, also dumps obj-level materialized Store/Load and obj.field pointers.
/// Author: Yan Wang, Date: 2026-02-02
pub fn dump_class_pag(class_pag: &ClassPAG, output_path: &str, solver_result: Option<&ClassPTSResult>) {
    let mut writer = BufWriter::new(match output_path {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    writer.write_all(b"# rcpta ClassPAG (Class-level Pointer Assignment Graph)\n\n")
        .expect("Unable to write header");

    let hidden_ptrs = collect_hidden_ptr_ids(class_pag, solver_result);

    // 1. Pointers (build-time ptrs + obj.field ptrs from solver when available)
    writer.write_all(b"## Pointers\n\n").expect("Unable to write section header");
    let mut ptr_ids: Vec<_> = class_pag.ptr_ids().cloned().collect();
    if let Some(result) = solver_result {
        for id in result.pts.keys() {
            if class_pag.get_ptr(id).is_none() {
                ptr_ids.push(id.clone());
            }
        }
    }
    ptr_ids.retain(|id| should_dump_ptr_id(id, &hidden_ptrs));
    ptr_ids.sort();
    if ptr_ids.is_empty() {
        writer.write_all(b"  (none)\n\n").expect("Unable to write");
    } else {
        for id in &ptr_ids {
            if let Some(ptr) = class_pag.get_ptr(id) {
                let short = short_class_pag_name(&ptr.id);
                writer.write_all(format!("  {}  [{}]\n", short, ptr.class_type).as_bytes())
                    .expect("Unable to write pointer");
            } else if solver_result.is_some() {
                // obj.field pointer (materialized during PTS)
                let short = short_class_pag_name(id);
                let field = id.rsplit('.').next().unwrap_or("?");
                writer.write_all(format!("  {}  [field: {}]\n", short, field).as_bytes())
                    .expect("Unable to write pointer");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 2. Objects
    writer.write_all(b"## Objects\n\n").expect("Unable to write section header");
    let mut obj_ids: Vec<_> = class_pag.obj_ids().cloned().collect();
    obj_ids.sort();
    if obj_ids.is_empty() {
        writer.write_all(b"  (none)\n\n").expect("Unable to write");
    } else {
        for id in &obj_ids {
            if let Some(obj) = class_pag.get_obj(id) {
                let short_site = short_class_pag_name(&obj.alloc_site.to_string());
                writer
                    .write_all(format!("  {}  {}  @{}\n", obj.id, obj.class_type, short_site).as_bytes())
                    .expect("Unable to write object");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 3. Group edges by function body (same func -> same section)
    use crate::rcpta::{LoadEdge, StoreEdge};
    type FuncEdges = (
        Vec<(String, String)>,
        Vec<(String, String)>,
        Vec<(String, String)>,
        Vec<LoadEdge>,
        Vec<StoreEdge>,
        Vec<(String, usize, String, String)>,
        Vec<(String, String, String)>,
    );
    let mut by_func: BTreeMap<String, FuncEdges> = BTreeMap::new();
    let empty_edges = || (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

    for (src, dst) in class_pag.iter_assign_edges() {
        if !should_dump_ptr_edge(&src, &dst, &hidden_ptrs) {
            continue;
        }
        let scope = func_scope_from_ptr_id(&dst);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).0.push((src, dst));
    }
    for (src, dst) in class_pag.iter_cast_edges() {
        if !should_dump_ptr_edge(&src, &dst, &hidden_ptrs) {
            continue;
        }
        let scope = func_scope_from_ptr_id(&dst);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).1.push((src, dst));
    }
    for (ptr_id, obj_id) in class_pag.iter_alloc_edges() {
        if !should_dump_ptr_id(&ptr_id, &hidden_ptrs) {
            continue;
        }
        let scope = func_scope_from_ptr_id(&ptr_id);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).2.push((ptr_id, obj_id));
    }
    for e in class_pag.iter_load_edges() {
        if !should_dump_ptr_id(&e.base_ptr_id, &hidden_ptrs)
            || !should_dump_ptr_id(&e.dst_ptr_id, &hidden_ptrs)
        {
            continue;
        }
        let scope = func_scope_from_ptr_id(&e.dst_ptr_id);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).3.push(e);
    }
    for e in class_pag.iter_store_edges() {
        if !should_dump_ptr_id(&e.base_ptr_id, &hidden_ptrs)
            || !should_dump_ptr_id(&e.src_ptr_id, &hidden_ptrs)
        {
            continue;
        }
        let scope = func_scope_from_ptr_id(&e.base_ptr_id);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).4.push(e);
    }
    // Only hide CallArg/CallRet when call is "impl wrapper -> same-named method body" (MIR artifact).
    // Do not hide real calls from method bodies (e.g. chain_with -> with_partner).
    let call_arg = class_pag.call_arg_edges();
    for e in call_arg.iter() {
        let scope = caller_from_call_site(&e.call_site);
        let callee_scope = callee_scope_from_formal_ptr_id(&e.formal_ptr_id);
        if is_wrapper_to_same_method_edge(&scope, callee_scope.as_deref()) {
            continue;
        }
        if !should_dump_ptr_id(&e.actual_ptr_id, &hidden_ptrs)
            || !should_dump_ptr_id(&e.formal_ptr_id, &hidden_ptrs)
        {
            continue;
        }
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).5.push((e.call_site.clone(), e.arg_idx, e.actual_ptr_id.clone(), e.formal_ptr_id.clone()));
    }
    let call_ret = class_pag.call_ret_edges();
    for e in call_ret.iter() {
        let scope = caller_from_call_site(&e.call_site);
        let callee_scope = callee_scope_from_formal_ptr_id(&e.formal_ret_ptr_id);
        if is_wrapper_to_same_method_edge(&scope, callee_scope.as_deref()) {
            continue;
        }
        if !should_dump_ptr_id(&e.formal_ret_ptr_id, &hidden_ptrs)
            || !should_dump_ptr_id(&e.actual_ret_ptr_id, &hidden_ptrs)
        {
            continue;
        }
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges).6.push((e.call_site.clone(), e.formal_ret_ptr_id.clone(), e.actual_ret_ptr_id.clone()));
    }

    // rcpta: ensure every function that has any ptr in ClassPAG gets a section (even if no edges yet).
    for ptr_id in class_pag.ptr_ids() {
        if !should_dump_ptr_id(ptr_id, &hidden_ptrs) {
            continue;
        }
        let scope = func_scope_from_ptr_id(ptr_id);
        let func = canonical_section_key_for_scope(&scope);
        by_func.entry(func).or_insert_with(empty_edges);
    }

    // Sort edge lists within each function for stable output
    for (_, (assign, cast, alloc, load, store, _, _)) in by_func.iter_mut() {
        assign.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        cast.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        alloc.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        load.sort_by(|a, b| {
            a.base_ptr_id.cmp(&b.base_ptr_id)
                .then_with(|| a.field.cmp(&b.field))
                .then_with(|| a.dst_ptr_id.cmp(&b.dst_ptr_id))
        });
        store.sort_by(|a, b| {
            a.base_ptr_id.cmp(&b.base_ptr_id)
                .then_with(|| a.field.cmp(&b.field))
                .then_with(|| a.src_ptr_id.cmp(&b.src_ptr_id))
        });
    }

    let mut total_assign = 0usize;
    let mut total_cast = 0usize;
    let mut total_alloc = 0usize;
    let mut total_load = 0usize;
    let mut total_store = 0usize;
    let total_call_arg: usize = by_func.values().map(|(_, _, _, _, _, call_arg_f, _)| call_arg_f.len()).sum();
    let total_call_ret: usize = by_func.values().map(|(_, _, _, _, _, _, call_ret_f)| call_ret_f.len()).sum();

    // 4. Per-function sections: assign, cast, alloc, load, store, call_arg, call_ret
    writer.write_all(b"## Edges by function\n\n").expect("Unable to write section header");
    for (func_name, (assign, cast, alloc, load, store, call_arg_f, call_ret_f)) in &by_func {
        // Skip sections with no edges (e.g. interface get_id with only formals, no body edges).
        if assign.is_empty() && cast.is_empty() && alloc.is_empty() && load.is_empty() && store.is_empty() && call_arg_f.is_empty() && call_ret_f.is_empty() {
            continue;
        }
        total_assign += assign.len();
        total_cast += cast.len();
        total_alloc += alloc.len();
        total_load += load.len();
        total_store += store.len();

        // Skip internal DSL trait methods (e.g. downgrade_from for into_superclass); not user-facing class methods.
        if analysis::is_internal_dsl_trait_method(func_name) {
            continue;
        }

        let short_func = short_class_pag_name(func_name);
        writer.write_all(format!("### {}\n\n", short_func).as_bytes()).expect("Unable to write func header");

        writer.write_all(b"  Assign (src -> dst):\n").expect("Unable to write");
        if assign.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for (src, dst) in assign {
                writer.write_all(format!("    {} -> {}\n", short_class_pag_name(src), short_class_pag_name(dst)).as_bytes()).expect("Unable to write assign edge");
            }
        }
        writer.write_all(b"  Cast (src -> dst):\n").expect("Unable to write");
        if cast.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for (src, dst) in cast {
                writer.write_all(format!("    {} -> {}  [cast]\n", short_class_pag_name(src), short_class_pag_name(dst)).as_bytes()).expect("Unable to write cast edge");
            }
        }
        writer.write_all(b"  Alloc (ptr -> obj):\n").expect("Unable to write");
        if alloc.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for (ptr_id, obj_id) in alloc {
                writer.write_all(format!("    {} -> {}\n", short_class_pag_name(ptr_id), obj_id).as_bytes()).expect("Unable to write alloc edge");
            }
        }
        writer.write_all(b"  Load (base.field -> dst):\n").expect("Unable to write");
        if load.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for e in load {
                writer.write_all(format!("    {}.{} -> {}\n", short_class_pag_name(&e.base_ptr_id), e.field, short_class_pag_name(&e.dst_ptr_id)).as_bytes()).expect("Unable to write load edge");
            }
        }
        writer.write_all(b"  Store (base.field <- src):\n").expect("Unable to write");
        if store.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for e in store {
                writer.write_all(format!("    {}.{} <- {}\n", short_class_pag_name(&e.base_ptr_id), e.field, short_class_pag_name(&e.src_ptr_id)).as_bytes()).expect("Unable to write store edge");
            }
        }
        writer.write_all(b"  CallArg (call_site [arg idx] actual -> formal):\n").expect("Unable to write");
        if call_arg_f.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for (call_site, arg_idx, actual, formal) in call_arg_f {
                writer.write_all(format!("    {} [arg {}] {} -> {}\n", short_class_pag_name(call_site), arg_idx, short_class_pag_name(actual), short_class_pag_name(formal)).as_bytes()).expect("Unable to write call arg edge");
            }
        }
        writer.write_all(b"  CallRet (call_site: formal_ret -> actual_ret):\n").expect("Unable to write");
        if call_ret_f.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            for (call_site, formal_ret, actual_ret) in call_ret_f {
                writer.write_all(format!("    {}  {} -> {}\n", short_class_pag_name(call_site), short_class_pag_name(formal_ret), short_class_pag_name(actual_ret)).as_bytes()).expect("Unable to write call ret edge");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 5. Materialized Store/Load (after PTS): obj-level edges when base flows to obj
    if let Some(result) = solver_result {
        writer.write_all(b"## Materialized Store/Load (after PTS)\n\n").expect("Unable to write section header");
        writer.write_all(b"  Store (src -> obj.field):\n").expect("Unable to write");
        if result.materialized_stores.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            let mut stores: Vec<_> = result.materialized_stores.iter().collect();
            stores.sort_by(|a, b| (a.src_ptr_id.as_str(), a.obj_id.as_str(), a.field.as_str()).cmp(&(b.src_ptr_id.as_str(), b.obj_id.as_str(), b.field.as_str())));
            for e in stores {
                let obj_field = format!("{}.{}", e.obj_id, e.field);
                writer.write_all(format!("    {} -> {}\n", short_class_pag_name(&e.src_ptr_id), obj_field).as_bytes()).expect("Unable to write");
            }
        }
        writer.write_all(b"  Load (obj.field -> dst):\n").expect("Unable to write");
        if result.materialized_loads.is_empty() {
            writer.write_all(b"    (none)\n").expect("Unable to write");
        } else {
            let mut loads: Vec<_> = result.materialized_loads.iter().collect();
            loads.sort_by(|a, b| (a.obj_id.as_str(), a.field.as_str(), a.dst_ptr_id.as_str()).cmp(&(b.obj_id.as_str(), b.field.as_str(), b.dst_ptr_id.as_str())));
            for e in loads {
                let obj_field = format!("{}.{}", e.obj_id, e.field);
                writer.write_all(format!("    {} -> {}\n", obj_field, short_class_pag_name(&e.dst_ptr_id)).as_bytes()).expect("Unable to write");
            }
        }
        writer.write_all(b"\n").expect("Unable to write newline");
    }

    // 6. Statistics
    writer.write_all(b"## Statistics\n\n").expect("Unable to write section header");
    let (total_load_dump, total_store_dump) = if let Some(r) = solver_result {
        (total_load + r.materialized_loads.len(), total_store + r.materialized_stores.len())
    } else {
        (total_load, total_store)
    };
    writer
        .write_all(
            format!(
                "  ptrs: {}  objs: {}  assign: {}  cast: {}  alloc: {}  load: {}  store: {}  call_arg: {}  call_ret: {}\n",
                ptr_ids.len(),
                class_pag.num_objs(),
                total_assign,
                total_cast,
                total_alloc,
                total_load_dump,
                total_store_dump,
                total_call_arg,
                total_call_ret,
            )
            .as_bytes(),
        )
        .expect("Unable to write statistics");
}

/// Dumps rcpta class-level points-to sets from a precomputed ClassPTSResult (used when solver was already run for ClassPAG dump).
pub fn dump_class_pts_from_result(
    class_pag: &ClassPAG,
    result: &ClassPTSResult,
    output_path: &str,
) {
    let pts = &result.pts;
    let mut writer = BufWriter::new(match output_path {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });
    writer
        .write_all(b"# rcpta Class-level Points-to Sets (PTS)\n\n")
        .expect("Unable to write header");
    writer
        .write_all(b"# For each pointer, the set of class heap objects it may point to after propagation on ClassPAG.\n\n")
        .expect("Unable to write description");
    writer.write_all(b"## Pointer -> Objects\n\n").expect("Unable to write section header");
    let hidden_ptrs = collect_hidden_ptr_ids(class_pag, Some(result));
    let mut ptr_ids: Vec<_> = pts.keys().cloned().collect();
    ptr_ids.retain(|id| should_dump_ptr_id(id, &hidden_ptrs));
    ptr_ids.sort();
    for ptr_id in &ptr_ids {
        let objs = pts.get(ptr_id).unwrap();
        let short_ptr = short_class_pag_name(ptr_id);
        if objs.is_empty() {
            writer
                .write_all(format!("  {}  ->  (none)\n", short_ptr).as_bytes())
                .expect("Unable to write pts line");
        } else {
            let mut obj_list: Vec<_> = objs.iter().cloned().collect();
            obj_list.sort();
            let objs_str = obj_list.join(", ");
            writer
                .write_all(format!("  {}  ->  {}\n", short_ptr, objs_str).as_bytes())
                .expect("Unable to write pts line");
        }
    }
    writer.write_all(b"\n## Statistics\n\n").expect("Unable to write section header");
    let total_ptrs = ptr_ids.len();
    let ptrs_with_objs = ptr_ids
        .iter()
        .filter(|id| pts.get(*id).map(|s| !s.is_empty()).unwrap_or(false))
        .count();
    writer
        .write_all(format!("  ptrs: {}  ptrs_with_objs: {}\n", total_ptrs, ptrs_with_objs).as_bytes())
        .expect("Unable to write statistics");
}

/// Dumps rcpta inferred type ranges (Type-info).
///
/// Principle: heap object `obj_id` has a source-level class type `class_type`.
/// After PTS propagation, each pointer `ptr_id` may point to a set of heap objects `PTS(ptr_id)`,
/// hence its type range is the corresponding set of class types:
///   TypeRange(ptr) = { class_type(obj) | obj in PTS(ptr) }.
pub fn dump_type_info_from_result(class_pag: &ClassPAG, result: &ClassPTSResult, output_path: &str) {
    let pts = &result.pts;
    let mut writer = BufWriter::new(match output_path {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => {
            ensure_parent_dir(output_path);
            Box::new(File::create(output_path).expect("Unable to create file")) as Box<dyn Write>
        }
    });

    writer
        .write_all(b"# Type-info (inferred type ranges per class pointer)\n\n")
        .expect("Unable to write header");
    writer
        .write_all(b"# For each pointer, infer the set of source-level class types it may refer to, using PTS.\n\n")
        .expect("Unable to write description");

    writer
        .write_all(b"## Pointer -> Type Range\n\n")
        .expect("Unable to write section header");

    let hidden_ptrs = collect_hidden_ptr_ids(class_pag, Some(result));
    let mut ptr_ids: Vec<_> = pts.keys().cloned().collect();
    ptr_ids.retain(|id| should_dump_ptr_id(id, &hidden_ptrs));
    ptr_ids.sort();

    for ptr_id in &ptr_ids {
        let objs = pts.get(ptr_id).unwrap();
        let short_ptr = short_class_pag_name(ptr_id);
        if objs.is_empty() {
            writer
                .write_all(format!("  {}  ->  (none)\n", short_ptr).as_bytes())
                .expect("Unable to write type-info line");
            continue;
        }

        let mut type_set: HashSet<String> = HashSet::new();
        for obj_id in objs {
            if let Some(obj) = class_pag.get_obj(obj_id) {
                type_set.insert(obj.class_type.clone());
            }
        }

        if type_set.is_empty() {
            writer
                .write_all(format!("  {}  ->  (none)\n", short_ptr).as_bytes())
                .expect("Unable to write type-info line");
        } else {
            let mut type_list: Vec<_> = type_set.into_iter().collect();
            type_list.sort();
            let types_str = type_list.join(", ");
            writer
                .write_all(format!("  {}  ->  {}\n", short_ptr, types_str).as_bytes())
                .expect("Unable to write type-info line");
        }
    }

    writer.write_all(b"\n## Statistics\n\n").expect("Unable to write section header");
    let total_ptrs = ptr_ids.len();
    let ptrs_with_types = ptr_ids
        .iter()
        .filter(|id| pts.get(*id).map(|s| !s.is_empty()).unwrap_or(false))
        .count();
    writer
        .write_all(format!("  ptrs: {}  ptrs_with_types: {}\n", total_ptrs, ptrs_with_types).as_bytes())
        .expect("Unable to write statistics");
}

/// Dumps rcpta class-level points-to sets: for each pointer, the set of class heap objects it may point to (after propagation on ClassPAG).
/// Author: Yan Wang, Date: 2026-02-02
pub fn dump_class_pts(class_pag: &ClassPAG, output_path: &str) {
    let result = solve_class_pts(class_pag);
    dump_class_pts_from_result(class_pag, &result, output_path);
}
