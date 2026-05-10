use crate::util::class::ClassTypeSystem;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    Extends,
    Implements,
    With,
    MixinOn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Class,
    Interface,
    Mixin,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub src: String,
    pub dst: String,
    pub kind: EdgeKind,
}

#[derive(Debug, Default)]
pub struct DslTypeGraph {
    pub nodes: HashMap<String, NodeKind>,
    pub edges: Vec<Edge>,
}

fn ensure_parent_dir(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
}

fn split_ident_list(s: &str) -> Vec<String> {
    // Keep it robust: tokens may contain whitespace/newlines or trailing punctuation.
    // We only care about extracting Rust identifier-like names.
    let ident_re = Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut out = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(m) = ident_re.captures(part) {
            out.push(m.get(1).unwrap().as_str().to_string());
        }
    }
    out
}

fn clause_segment(header: &str, keyword: &str, stop_keywords: &[&str]) -> Option<String> {
    let idx = header.find(keyword)?;
    let rest = &header[idx + keyword.len()..];
    let mut toks = Vec::new();
    for tok in rest.split_whitespace() {
        let trimmed = tok.trim_matches(|c: char| c == ',' || c == ';');
        if stop_keywords.iter().any(|stop| trimmed == *stop) {
            break;
        }
        toks.push(tok);
    }
    Some(toks.join(" "))
}

fn abstract_class_has_struct_section(content: &str, decl_end: usize) -> bool {
    let lookahead_end = std::cmp::min(content.len(), decl_end + 8192);
    let lookahead = &content[decl_end..lookahead_end];
    let struct_pos = lookahead.find("struct {").or_else(|| lookahead.find("struct{"));
    let fn_pos = lookahead.find("\n    pub fn")
        .or_else(|| lookahead.find("\n        pub fn"))
        .or_else(|| lookahead.find("\npub fn"))
        .or_else(|| lookahead.find("\n    fn "))
        .or_else(|| lookahead.find("\n        fn "));
    match (struct_pos, fn_pos) {
        (Some(s), Some(f)) => s < f,
        (Some(_), None) => true,
        _ => false,
    }
}

fn collect_rs_files_recursively(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files_recursively(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

pub fn build_graph_from_dsl_sources() -> DslTypeGraph {
    // We intentionally parse from the DSL *source definitions* in tests, not from
    // rcpta's pointer/type-info outputs.
    let rupta_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = rupta_manifest_dir
        .parent()
        .expect("rupta manifest must have a parent workspace root");
    let tests_root = workspace_root.join("rustdsl/classes/tests");

    let mut files = Vec::new();
    collect_rs_files_recursively(&tests_root, &mut files);

    // Declaration-driven parser:
    // - robust to multiline headers
    // - avoids global regex over large function bodies/comments
    let re_decl = Regex::new(
        r"(?s)\bpub\s+(?:(abstract)\s+)?(class|mixin)\s+([A-Za-z_][A-Za-z0-9_]*)\b(.*?)\{",
    )
    .unwrap();

    let mut graph = DslTypeGraph::default();
    let mut edges = Vec::new();
    for file in files {
        let content = match fs::read_to_string(&file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for cap in re_decl.captures_iter(&content) {
            let is_abstract = cap.get(1).is_some();
            let decl_kind = cap.get(2).map(|m| m.as_str()).unwrap_or_default();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or_default().to_string();
            let header_tail = cap.get(4).map(|m| m.as_str()).unwrap_or_default();
            let decl_end = cap.get(0).map(|m| m.end()).unwrap_or(0);

            match decl_kind {
                "mixin" => {
                    graph.nodes.entry(name.clone()).or_insert(NodeKind::Mixin);
                    if let Some(on_seg) = clause_segment(header_tail, "on", &[]) {
                        for on_t in split_ident_list(&on_seg) {
                            edges.push(Edge {
                                src: name.clone(),
                                dst: on_t,
                                kind: EdgeKind::MixinOn,
                            });
                        }
                    }
                }
                "class" => {
                    let node_kind = if is_abstract {
                        if abstract_class_has_struct_section(&content, decl_end) {
                            NodeKind::Class
                        } else {
                            NodeKind::Interface
                        }
                    } else {
                        NodeKind::Class
                    };
                    graph.nodes.entry(name.clone()).or_insert(node_kind);

                    if let Some(ext_seg) = clause_segment(header_tail, "extends", &["implements", "with"]) {
                        for parent in split_ident_list(&ext_seg) {
                            edges.push(Edge {
                                src: name.clone(),
                                dst: parent,
                                kind: EdgeKind::Extends,
                            });
                        }
                    }
                    if let Some(impl_seg) = clause_segment(header_tail, "implements", &["extends", "with"]) {
                        for iface in split_ident_list(&impl_seg) {
                            edges.push(Edge {
                                src: name.clone(),
                                dst: iface,
                                kind: EdgeKind::Implements,
                            });
                        }
                    }
                    if let Some(with_seg) = clause_segment(header_tail, "with", &["extends", "implements"]) {
                        for mixin in split_ident_list(&with_seg) {
                            edges.push(Edge {
                                src: name.clone(),
                                dst: mixin,
                                kind: EdgeKind::With,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Ensure nodes mentioned only in edges are registered.
    for e in &edges {
        graph.nodes.entry(e.src.clone()).or_insert(NodeKind::Unknown);
        graph.nodes.entry(e.dst.clone()).or_insert(NodeKind::Unknown);
    }

    graph.edges = edges;
    graph
}

fn compute_reachable_nodes(
    adj: &HashMap<String, Vec<String>>,
    start_nodes: &HashSet<String>,
) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut work = std::collections::VecDeque::new();
    for s in start_nodes {
        if visited.insert(s.clone()) {
            work.push_back(s.clone());
        }
    }
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

pub fn dump_inheritance_graph_from_entry_types(
    class_type_system: &ClassTypeSystem,
    output_path: &String,
) {
    let graph = build_graph_from_dsl_sources();

    // Entry-related types: use what rcpta discovered syntactically in reachable code.
    // This is NOT `type-info.txt`, it is a set of DSL type names registered by analysis.
    let involved: HashSet<String> = class_type_system
        .get_all_classes()
        .keys()
        .cloned()
        .collect();

    // Only keep nodes that exist in the parsed DSL graph.
    let involved_in_graph: HashSet<String> = involved
        .iter()
        .filter(|n| graph.nodes.contains_key(*n))
        .cloned()
        .collect();

    // =============== Build "safe" relations ===============
    // For type-conversion safety, we do NOT blindly take reachability closure over all edges.
    // Instead:
    // - extends* is safe for upcast/view conversion
    // - implements* is safe when the concrete class (or any superclass) declares it implements
    //   the interface, and interfaces can themselves extend other interfaces.
    // - with* is safe when the concrete class (or any superclass) declares it uses that mixin.
    // - mixin_on is kept only as a direct relation; it is NOT treated as guaranteeing the mixin
    //   view is present on every `on` target instance at runtime.

    let mut extends_adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut direct_impl: HashMap<String, HashSet<String>> = HashMap::new();
    let mut direct_with: HashMap<String, HashSet<String>> = HashMap::new();

    for e in &graph.edges {
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
            EdgeKind::MixinOn => {
                // intentionally not included in safe closures
            }
        }
    }

    // Closure nodes:
    // - all extends ancestors/descendants reachable from involved types via extends
    // - plus interfaces implemented by any class in the extends-closure
    // - plus mixins selected by any class in the extends-closure
    let extends_closure_nodes = compute_reachable_nodes(&extends_adj, &involved_in_graph);

    let mut closure_nodes: HashSet<String> = extends_closure_nodes.clone();
    for n in &extends_closure_nodes {
        if let Some(ifs) = direct_impl.get(n) {
            for i in ifs {
                closure_nodes.insert(i.clone());
                // interface extends chain is also relevant for view conversion safety
                let mut start = HashSet::new();
                start.insert(i.clone());
                let reach = compute_reachable_nodes(&extends_adj, &start);
                closure_nodes.extend(reach);
            }
        }
        if let Some(ms) = direct_with.get(n) {
            for m in ms {
                closure_nodes.insert(m.clone());
                // mixins might not extend, but keep it safe/consistent if they do
                let mut start = HashSet::new();
                start.insert(m.clone());
                let reach = compute_reachable_nodes(&extends_adj, &start);
                closure_nodes.extend(reach);
            }
        }
    }

    // Prepare direct edges filtered to closure nodes.
    let mut direct_edges: Vec<Edge> = Vec::new();
    for e in &graph.edges {
        if closure_nodes.contains(&e.src) && closure_nodes.contains(&e.dst) {
            direct_edges.push(e.clone());
        }
    }
    direct_edges.sort_by(|a, b| {
        let ka = format!("{:?}", a.kind);
        let kb = format!("{:?}", b.kind);
        (a.src.cmp(&b.src)).then(ka.cmp(&kb)).then(a.dst.cmp(&b.dst))
    });

    // =============== Safe closure edges ===============
    // closure edges carry a kind tag so consumers can map them to conversion safety.
    let mut closure_edges: HashSet<(String, String, EdgeKind)> = HashSet::new();

    // extends*: any reachable via extends edges
    for src in &closure_nodes {
        let mut start = HashSet::new();
        start.insert(src.clone());
        let reach = compute_reachable_nodes(&extends_adj, &start);
        for dst in reach {
            if dst != *src {
                closure_edges.insert((src.clone(), dst, EdgeKind::Extends));
            }
        }
    }

    // implements*: propagate implements along extends chain, then add interface-extends ancestors
    for src in &closure_nodes {
        // all super classes (including itself)
        let mut anc_start = HashSet::new();
        anc_start.insert(src.clone());
        let anc = compute_reachable_nodes(&extends_adj, &anc_start);

        let mut implemented_ifaces: HashSet<String> = HashSet::new();
        for a in &anc {
            if let Some(ifs) = direct_impl.get(a) {
                implemented_ifaces.extend(ifs.iter().cloned());
            }
        }

        // interface inheritance closure
        let mut expanded_ifaces: HashSet<String> = HashSet::new();
        for iface in &implemented_ifaces {
            let mut start = HashSet::new();
            start.insert(iface.clone());
            let reach = compute_reachable_nodes(&extends_adj, &start);
            expanded_ifaces.extend(reach);
        }

        for iface in expanded_ifaces {
            closure_edges.insert((src.clone(), iface, EdgeKind::Implements));
        }
    }

    // with*: propagate mixin selection along extends chain (do NOT use mixin_on)
    for src in &closure_nodes {
        let mut anc_start = HashSet::new();
        anc_start.insert(src.clone());
        let anc = compute_reachable_nodes(&extends_adj, &anc_start);

        let mut selected_mixins: HashSet<String> = HashSet::new();
        for a in &anc {
            if let Some(ms) = direct_with.get(a) {
                selected_mixins.extend(ms.iter().cloned());
            }
        }
        for m in selected_mixins {
            closure_edges.insert((src.clone(), m, EdgeKind::With));
        }
    }

    ensure_parent_dir(output_path);
    let mut writer = match output_path.as_str() {
        "stdout" => Box::new(std::io::stdout()) as Box<dyn Write>,
        _ => Box::new(fs::File::create(output_path).expect("Unable to create inheritance graph file"))
            as Box<dyn Write>,
    };

    writer
        .write_all(b"# DSL Type Relation Graph (direct edges + transitive closure)\n\n")
        .expect("write header");
    writer
        .write_all(b"## Node kinds\n\n")
        .expect("write node kinds header");

    let mut node_list: Vec<_> = closure_nodes.iter().cloned().collect();
    node_list.sort();
    for n in node_list {
        let kind = graph.nodes.get(&n).copied().unwrap_or(NodeKind::Unknown);
        let kind_str = match kind {
            NodeKind::Class => "class",
            NodeKind::Interface => "interface",
            NodeKind::Mixin => "mixin",
            NodeKind::Unknown => "unknown",
        };
        writer
            .write_all(format!("  {}  [{}]\n", n, kind_str).as_bytes())
            .expect("write node line");
    }

    writer
        .write_all(b"\n## Direct edges\n\n")
        .expect("write direct header");
    for e in direct_edges {
        let k = match e.kind {
            EdgeKind::Extends => "extends",
            EdgeKind::Implements => "implements",
            EdgeKind::With => "with",
            EdgeKind::MixinOn => "mixin_on",
        };
        writer
            .write_all(format!("  {} -[{}]-> {}\n", e.src, k, e.dst).as_bytes())
            .expect("write direct edge");
    }

    writer
        .write_all(b"\n## Closure edges (reachability)\n\n")
        .expect("write closure header");
    let mut closure_edge_list: Vec<_> = closure_edges.into_iter().collect();
    closure_edge_list.sort_by(|(a1, b1, k1), (a2, b2, k2)| {
        let ks1 = format!("{:?}", k1);
        let ks2 = format!("{:?}", k2);
        a1.cmp(a2).then(ks1.cmp(&ks2)).then(b1.cmp(b2))
    });
    for (src, dst, kind) in closure_edge_list {
        let k = match kind {
            EdgeKind::Extends => "extends*",
            EdgeKind::Implements => "implements*",
            EdgeKind::With => "with*",
            EdgeKind::MixinOn => "mixin_on*",
        };
        writer
            .write_all(format!("  {} -[{}]-> {}\n", src, k, dst).as_bytes())
            .expect("write closure edge");
    }

    writer.flush().expect("flush");
}
