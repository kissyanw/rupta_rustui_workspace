use crate::rcpta::{ClassPAG, ClassPTSResult};
use crate::util::class::dsl_inheritance_graph::{
    build_graph_from_dsl_sources, dump_inheritance_graph_from_entry_types, EdgeKind, NodeKind,
};
use crate::util::class::ClassTypeSystem;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

fn ensure_parent_dir(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
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

fn type_range_for_ptr(class_pag: &ClassPAG, result: &ClassPTSResult, ptr_id: &str) -> HashSet<String> {
    let mut out: HashSet<String> = HashSet::new();
    let Some(objs) = result.pts.get(ptr_id) else {
        return out;
    };
    for obj_id in objs {
        if let Some(obj) = class_pag.get_obj(obj_id) {
            out.insert(obj.class_type.clone());
        }
    }
    out
}

fn type_range_for_cast_site_src(
    class_pag: &ClassPAG,
    result: &ClassPTSResult,
    src_ptr_id: &str,
    dst_ptr_id: &str,
) -> HashSet<String> {
    let mut out: HashSet<String> = HashSet::new();
    if let Some(objs) = result
        .cast_src_before_pts
        .get(&(src_ptr_id.to_string(), dst_ptr_id.to_string()))
    {
        for obj_id in objs {
            if let Some(obj) = class_pag.get_obj(obj_id) {
                out.insert(obj.class_type.clone());
            }
        }
        return out;
    }
    type_range_for_ptr(class_pag, result, src_ptr_id)
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
    // implements* rule mirrors `dsl_inheritance_graph`:
    // - collect implements declared on concrete or any superclass (via extends*)
    // - expand each implemented interface via interface-extends chain (also uses extends edges)
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
    // with* rule mirrors `dsl_inheritance_graph`:
    // - collect mixins selected by concrete or any superclass (via extends*)
    // - DO NOT use mixin_on for safety
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

fn format_type_set(types: &HashSet<String>) -> String {
    let mut v: Vec<_> = types.iter().cloned().collect();
    v.sort();
    format!("{{{}}}", v.join(", "))
}

#[inline]
fn is_source_level_cast_loc(src_loc: &str) -> bool {
    // Keep only source-level cast checks in suites/user code.
    // Filter out DSL runtime/macro internals such as `rustdsl/classes/src/macros/mod.rs:*`.
    src_loc.starts_with("classes/tests/")
        || src_loc.contains("/rustdsl/classes/tests/")
}

/// Dumps one line per source-level class cast site:
/// `file:line:col cast is safe/unsafe`
pub fn dump_cast_safety_log(
    class_type_system: &ClassTypeSystem,
    class_pag: &ClassPAG,
    pts_result: &ClassPTSResult,
    output_path: &str,
) {
    // Build DSL graph once, and compute relations in-memory (no dependency on reading dumped files).
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

    // Also keep a side-effect dump of inheritance graph if user requested it separately.
    // (This is a no-op unless the caller enabled that option; included only to keep behavior stable.)
    let _ = class_type_system;
    let _ = dump_inheritance_graph_from_entry_types;

    ensure_parent_dir(output_path);
    let mut writer: Box<dyn Write> = match output_path {
        "stdout" => Box::new(std::io::stdout()),
        _ => Box::new(fs::File::create(output_path).expect("Unable to create cast safety log file")),
    };

    let mut sites = class_pag.cast_sites().to_vec();
    sites.sort_by(|a, b| a.src_loc.cmp(&b.src_loc).then(a.src_ptr_id.cmp(&b.src_ptr_id)).then(a.dst_ptr_id.cmp(&b.dst_ptr_id)));

    for site in sites {
        if !is_source_level_cast_loc(&site.src_loc) {
            continue;
        }
        let src_types = type_range_for_cast_site_src(
            class_pag,
            pts_result,
            &site.src_ptr_id,
            &site.dst_ptr_id,
        );
        let dst_ty = class_pag
            .get_ptr(&site.dst_ptr_id)
            .map(|p| p.class_type.clone());

        let (safe, diag): (bool, Option<String>) = if src_types.is_empty() {
            (
                false,
                Some(
                    "unsafe_kind: boundary-unknown-src\nreason: src pointer has empty points-to set (no inferred dynamic types)"
                        .to_string(),
                ),
            )
        } else if dst_ty.is_none() {
            (
                false,
                Some(
                    "unsafe_kind: boundary-missing-dst-type\nreason: dst pointer has no static class_type recorded in ClassPAG"
                        .to_string(),
                ),
            )
        } else {
            let dst_ty = dst_ty.unwrap();
            let dst_kind = dsl_graph.nodes.get(&dst_ty).copied().unwrap_or(NodeKind::Unknown);
            let mut bad: Vec<String> = Vec::new();
            let mut good: Vec<String> = Vec::new();
            for s in &src_types {
                let ok = match dst_kind {
                    NodeKind::Class | NodeKind::Unknown => is_subtype_via_extends(&extends_adj, s, &dst_ty),
                    NodeKind::Interface => implements_interface(&extends_adj, &direct_impl, s, &dst_ty),
                    NodeKind::Mixin => has_mixin_view(&extends_adj, &direct_with, s, &dst_ty),
                };
                if !ok {
                    bad.push(s.clone());
                } else {
                    good.push(s.clone());
                }
            }
            bad.sort();
            good.sort();

            if bad.is_empty() {
                (true, None)
            } else {
                let unsafe_kind = if good.is_empty() {
                    "must-unsafe"
                } else {
                    "may-unsafe"
                };
                let rule = match dst_kind {
                    NodeKind::Class | NodeKind::Unknown => "extends* (class subtype)",
                    NodeKind::Interface => "implements* (class implements interface via extends chain)",
                    NodeKind::Mixin => "with* (class has mixin view via extends chain; mixin_on is not used)",
                };
                let dst_kind_str = match dst_kind {
                    NodeKind::Class => "class",
                    NodeKind::Interface => "interface",
                    NodeKind::Mixin => "mixin",
                    NodeKind::Unknown => "unknown",
                };
                let mut diag = String::new();
                diag.push_str(&format!("unsafe_kind: {}\n", unsafe_kind));
                diag.push_str(&format!(
                    "types: src_dynamic_types={} dst_static_type={} dst_kind={}\n",
                    format_type_set(&src_types),
                    dst_ty,
                    dst_kind_str
                ));
                diag.push_str(&format!(
                    "classification: satisfied_types={{{}}} unsatisfied_types={{{}}}\n",
                    good.join(", "),
                    bad.join(", ")
                ));
                diag.push_str(&format!(
                    "reason: the following src types do not satisfy {} to dst: {{{}}}",
                    rule,
                    bad.join(", ")
                ));
                (false, Some(diag))
            }
        };

        let verdict = if safe { "safe" } else { "unsafe" };
        writer
            .write_all(format!("{} cast is {}\n", site.src_loc, verdict).as_bytes())
            .expect("write cast safety line");
        if let Some(diag) = diag {
            // Keep diagnostics on separate lines for readability and easy grepping.
            for line in diag.lines() {
                writer
                    .write_all(format!("  {}\n", line).as_bytes())
                    .expect("write cast safety diag line");
            }
        }
    }

    writer.flush().expect("flush cast safety log");
}
