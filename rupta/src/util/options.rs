// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Analysis options.

use itertools::Itertools;

use clap::{Arg, Command};
use clap::error::ErrorKind;
use rustc_tools_util::VersionInfo;


use crate::pta::PTAType;

const RUPTA_USAGE: &str = r#"pta [OPTIONS] INPUT -- [RUSTC OPTIONS]"#;

/// The version information from Cargo.toml.
fn version() -> &'static str {
    let version_info = rustc_tools_util::get_version_info!();
    let version = format!("v{}.{}.{}", version_info.major, version_info.minor, version_info.patch);
    Box::leak(version.into_boxed_str())
}

/// Creates the clap::Command metadata for argument parsing.
fn make_options_parser() -> Command<'static> {
    // We could put this into lazy_static! with a Mutex around, but we really do not expect
    // to construct this more then once per regular program run.
    let parser = Command::new("rupta")
        .no_binary_name(true)
        .override_usage(RUPTA_USAGE)
        .version(version())
        .arg(Arg::new("entry-func-name")
            .long("entry-func")
            .takes_value(true) 
            .help("The name of entry function from which the pointer analysis begins."))
        .arg(Arg::new("entry-func-id")
            .long("entry-id")
            .takes_value(true)
            .value_parser(clap::value_parser!(u32))
            .help("The def_id of entry function from which the pointer analysis begins."))
        .arg(Arg::new("pta-type")
            .long("pta-type")
            .takes_value(true)
            .value_parser(["andersen", "ander", "callsite-sensitive", "cs"])
            .default_value("callsite-sensitive")
            .help("The type of pointer analysis.")
            .long_help("Andersen and callsite-sensitive pointer analyses are supported now."))
        .arg(Arg::new("context-depth")
            .long("context-depth")
            .takes_value(true)
            .value_parser(clap::value_parser!(u32))
            .default_value("1")
            .help("The context depth limit for a context-sensitive pointer analysis."))
        .arg(Arg::new("no-cast-constraint")
            .long("no-cast-constraint")
            .takes_value(false)
            .hide(true)
            .help("Disable the cast optimization that constrains an object cast from a simple pointer type."))
        .arg(Arg::new("stack-filtering")
            .long("stack-filtering")
            .takes_value(false)
            .help("Enable stack filtering in pointer analysis."))
        .arg(Arg::new("analyze-only")
            .long("analyze-only")
            .takes_value(false)
            .help("Stop compilation after pointer analysis finishes."))
        .arg(Arg::new("dump-stats")
            .long("dump-stats")
            .takes_value(false)
            .help("Dump the statistics of the analysis results."))
        .arg(Arg::new("call-graph-output")
            .long("dump-call-graph")
            .takes_value(true)
            .help("Dump the call graph in DOT format to the output file."))
        .arg(Arg::new("pts-output")
            .long("dump-pts")
            .takes_value(true)
            .help("Dump points-to results to the output file."))
        .arg(Arg::new("mir-output")
            .long("dump-mir")
            .takes_value(true)
            .help("Dump the mir of reachable functions to the output file."))
        .arg(Arg::new("unsafe-stats-output")
            .long("dump-unsafe-stats")
            .takes_value(true)
            .help("Dump the statistics of unsafe functions in the analyzed program."))
        .arg(Arg::new("dyn-calls-output")
            .long("dump-dyn-calls")
            .takes_value(true)
            .hide(true)
            .hide(true)
            .help("Dump resolved dynamic callsites with their corresponding call targets.")
            .long_help("Including both calls on dynamic trait objects and calls via function pointers"))
        .arg(Arg::new("type-indices-output")
            .long("dump-type-indices")
            .takes_value(true)
            .hide(true)
            .help("Dump type indices for debugging."))
        .arg(Arg::new("class-level-mode")
            .long("class-level-mode")
            .takes_value(false)
            .help("Enable class-level analysis mode (filters out non-class related information)."))
        .arg(Arg::new("class-info-output")
            .long("dump-class-info")
            .takes_value(true)
            .help("Dump class-level information (constructors, instances, etc.) to the output file."))
        .arg(Arg::new("class-call-graph-output")
            .long("dump-class-call-graph")
            .takes_value(true)
            .help("Dump class call graph (only class method calls, filters DSL internal details) to the output file."))
        .arg(Arg::new("class-type-system-output")
            .long("dump-class-type-system")
            .takes_value(true)
            .help("Dump class type system information (classes, fields, methods, instances, references) to the output file."))
        .arg(Arg::new("class-pag-output")
            .long("dump-class-pag")
            .takes_value(true)
            .help("Dump rcpta class-level PAG (ClassPAG: ptrs, objs, assign/alloc/load/store/call edges) to the output file."))
        .arg(Arg::new("class-pts-output")
            .long("dump-class-pts")
            .takes_value(true)
            .help("Dump rcpta class-level points-to sets (each ptr -> set of class heap objs after propagation) to the output file."))
        .arg(Arg::new("type-info-output")
            .long("dump-type-info")
            .takes_value(true)
            .help("Dump inferred type ranges per class pointer (each ptr -> set of class types) to the output file."))
        .arg(Arg::new("inheritance-graph-output")
            .long("dump-inheritance-graph")
            .takes_value(true)
            .help("Dump DSL type relation graph (extends/mixin/interface) with direct edges and transitive closure."))
        .arg(Arg::new("cast-safety-log-output")
            .long("dump-cast-safety-log")
            .takes_value(true)
            .help("Dump cast safety decisions as `file:line:col cast is safe/unsafe`."))
        .arg(Arg::new("INPUT")
            .multiple(true)
            .help("The input file to be analyzed.")
        );
    parser
}

#[derive(Clone, Debug)]
pub struct AnalysisOptions {
    pub entry_func: String,
    pub entry_def_id: Option<u32>,
    pub pta_type: PTAType,
    // options for context-sensitive analysis
    pub context_depth: u32,
    // options for handling cast propagation
    pub cast_constraint: bool,
    pub stack_filtering: bool,
    pub analyze_only: bool,
    
    pub dump_stats: bool,
    pub call_graph_output: Option<String>,
    pub pts_output: Option<String>,
    pub mir_output: Option<String>,
    pub type_indices_output: Option<String>,
    pub dyn_calls_output: Option<String>,
    pub unsafe_stat_output: Option<String>,
    pub func_ctxts_output: Option<String>, 
    
    // Class-level analysis options
    pub class_level_mode: bool,
    pub class_info_output: Option<String>,
    pub class_type_system_output: Option<String>,
    pub class_call_graph_output: Option<String>,
    /// rcpta: dump ClassPAG (class-level pointer flow graph). Author: Yan Wang, Date: 2026-02-02
    pub class_pag_output: Option<String>,
    /// rcpta: dump class-level PTS (ptr -> set of objs after propagation on ClassPAG).
    pub class_pts_output: Option<String>,
    /// rcpta: dump ptr -> set of class types inferred from ClassPTSResult (PTS then obj_id -> obj.class_type).
    pub type_info_output: Option<String>,
    /// Dump static DSL inheritance/conversion graph (direct + closure) filtered to entry-related types.
    pub inheritance_graph_output: Option<String>,
    /// Dump cast safety decisions (source loc + safe/unsafe).
    pub cast_safety_log_output: Option<String>,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            entry_func: String::new(),
            entry_def_id: None,
            pta_type: PTAType::CallSiteSensitive,
            context_depth: 1,
            cast_constraint: true,
            stack_filtering: true,
            analyze_only: false,
            dump_stats: true,
            call_graph_output: None,
            pts_output: None,
            mir_output: None,
            type_indices_output: None,
            dyn_calls_output: None,
            unsafe_stat_output: None,
            func_ctxts_output: None,
            class_level_mode: false,
            class_info_output: None,
            class_call_graph_output: None,
            class_type_system_output: None,
            class_pag_output: None,
            class_pts_output: None,
            type_info_output: None,
            inheritance_graph_output: None,
            cast_safety_log_output: None,
        }
    }
}

impl AnalysisOptions {
    /// Parses options from a list of strings. Any content beyond the leftmost `--` token
    /// will be returned (excluding this token).
    pub fn parse_from_args(&mut self, args: &[String], from_env: bool) -> Vec<String> {
        let mut pta_args_end = args.len();
        let mut rustc_args_start = 0;
        if let Some((p, _)) = args.iter().find_position(|s| s.as_str() == "--") {
            pta_args_end = p;
            rustc_args_start = p + 1;
        }
        let pta_args = &args[0..pta_args_end];
        let matches = if !from_env && rustc_args_start == 0 {
            // The arguments may not be intended for RUPTA and may get here via some tool, so do not 
            // report errors here, but just assume that the arguments were not meant for RUPTA.
            match make_options_parser().try_get_matches_from(pta_args.iter())
            {
                Ok(matches) => {
                    // Looks like these are RUPTA options after all and there are no rustc options.
                    rustc_args_start = args.len();
                    matches
                }
                Err(e) => match e.kind() {
                    ErrorKind::DisplayHelp => {
                        eprintln!("{e}");
                        return args.to_vec();
                    }
                    ErrorKind::UnknownArgument => {
                        // Just send all of the arguments to rustc.
                        // Note that this means that RUPTA options and rustc options must always
                        // be separated by --. I.e. any RUPTA options present in arguments list
                        // will stay unknown to RUPTA and will make rustc unhappy.
                        return args.to_vec();
                    }
                    _ => {
                        e.exit();
                    }
                },
            }
        } else {
            // This will display error diagnostics for arguments that are not valid for RUPTA.
            match make_options_parser().try_get_matches_from(pta_args.iter()) {
                Ok(matches) => {
                    if rustc_args_start == 0 {
                        rustc_args_start = args.len();
                    }
                    matches
                }
                Err(e) => {
                    e.exit();
                }
            }
        };

        if let Some(s) = matches.get_one::<String>("entry-func-name") {
            self.entry_func = s.clone();
        }
        self.entry_def_id = matches.get_one::<u32>("entry-func-id").cloned();

        if matches.contains_id("pta-type") {
            self.pta_type = match matches.get_one::<String>("pta-type").unwrap().as_str() {
                "andersen" | "ander" => PTAType::Andersen,
                "callsite-sensitive" | "cs" => PTAType::CallSiteSensitive,
                _ => unreachable!(),
            }
        }
        
        if let Some(depth) = matches.get_one::<u32>("context-depth") {
            self.context_depth = *depth;
        }

        self.cast_constraint = !matches.contains_id("no-cast-constraint");
        self.stack_filtering = matches.contains_id("stack-filtering");
        self.analyze_only = matches.contains_id("analyze-only");
        
        self.dump_stats = matches.contains_id("dump-stats");
        self.call_graph_output = matches.get_one::<String>("call-graph-output").cloned();
        self.pts_output = matches.get_one::<String>("pts-output").cloned();
        self.mir_output = matches.get_one::<String>("mir-output").cloned();
        self.unsafe_stat_output = matches.get_one::<String>("unsafe-stats-output").cloned();
        self.dyn_calls_output = matches.get_one::<String>("dyn-calls-output").cloned();
        self.type_indices_output = matches.get_one::<String>("type-indices-output").cloned();
        
        // Class-level analysis options
        self.class_level_mode = matches.contains_id("class-level-mode");
        self.class_info_output = matches.get_one::<String>("class-info-output").cloned();
        self.class_call_graph_output = matches.get_one::<String>("class-call-graph-output").cloned();
        self.class_type_system_output = matches.get_one::<String>("class-type-system-output").cloned();
        self.class_pag_output = matches.get_one::<String>("class-pag-output").cloned();
        self.class_pts_output = matches.get_one::<String>("class-pts-output").cloned();
        self.type_info_output = matches.get_one::<String>("type-info-output").cloned();
        self.inheritance_graph_output = matches.get_one::<String>("inheritance-graph-output").cloned();
        self.cast_safety_log_output = matches.get_one::<String>("cast-safety-log-output").cloned();

        // If the user provide the input source code file path before the `--` token, 
        // add it to the rustc arguments.
        let mut rustc_args = args[rustc_args_start..].to_vec();
        if let Some(input) = matches.get_many::<String>("INPUT") {
            rustc_args.extend(input.cloned())
        } 
        
        rustc_args
    }

}
