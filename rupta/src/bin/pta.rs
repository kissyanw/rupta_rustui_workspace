// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! The main routine of `rupta`.
//! 
//! Implemented as a stub that invokes the rust compiler with a call back to execute 
//! pointer analysis during rust compilation.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_session;

use log::*;
use std::env;

use rupta::pta::PTACallbacks;
use rupta::util;
use rupta::util::options::AnalysisOptions;

/// Raise process stack limit to avoid overflow on large crates (e.g. vehicle_hierarchy).
/// Skip if RCPTA_SKIP_STACK_LIMIT=1. Tries 1024, 512, 256, 128 MiB until setrlimit succeeds.
fn maybe_raise_stack_limit() {
    if std::env::var_os("RCPTA_SKIP_STACK_LIMIT").is_some() {
        return;
    }
    #[cfg(unix)]
    {
        use libc::{rlimit, setrlimit, RLIMIT_STACK, RLIM_INFINITY};
        let mb_values = [1024_u64, 512, 256, 128];
        for &stack_mb in &mb_values {
            let limit = stack_mb * 1024 * 1024;
            let rlim = rlimit { rlim_cur: limit, rlim_max: RLIM_INFINITY };
            if unsafe { setrlimit(RLIMIT_STACK, &rlim) } == 0 {
                return;
            }
            let rlim = rlimit { rlim_cur: limit, rlim_max: limit };
            if unsafe { setrlimit(RLIMIT_STACK, &rlim) } == 0 {
                return;
            }
        }
    }
}

fn main() {
    maybe_raise_stack_limit();

    let early_dcx =
        rustc_session::EarlyDiagCtxt::new(rustc_session::config::ErrorOutputType::default());

    // Initialize loggers.
    if env::var("RUSTC_LOG").is_ok() {
        rustc_driver::init_rustc_env_logger(&early_dcx);
    }
    // Initialize env_logger if RUST_LOG or PTA_LOG is set
    if env::var("RUST_LOG").is_ok() || env::var("PTA_LOG").is_ok() {
        // Use RUST_LOG if set, otherwise use PTA_LOG with default level
        let log_filter = env::var("RUST_LOG").unwrap_or_else(|_| {
            env::var("PTA_LOG").unwrap_or_else(|_| "rupta=debug".to_string())
        });
        let e = env_logger::Env::default()
            .filter_or("RUST_LOG", log_filter)
            .write_style_or("RUST_LOG_STYLE", env::var("PTA_LOG_STYLE").unwrap_or_else(|_| "always".to_string()));
        env_logger::Builder::from_env(e).init();
    }

    // Get any options specified via the PTA_FLAGS environment variable
    let mut options = AnalysisOptions::default();
    let pta_flags = env::var("PTA_FLAGS").unwrap_or_default();
    eprintln!("[rupta][pta] raw PTA_FLAGS={}", pta_flags);
    let pta_args: Vec<String> = serde_json::from_str(&pta_flags).unwrap_or_default();
    eprintln!("[rupta][pta] parsed PTA_FLAGS args={:?}", pta_args);
    let rustc_args = options.parse_from_args(&pta_args[..], true);
    if options.class_pag_output.is_some() {
        eprintln!("[rupta][pta-v2] PTA_FLAGS: will dump class_pag to {:?}", options.class_pag_output);
    }
    // Debug: write to temp file so we can see pta ran even when stderr is captured by Cargo
    let debug_log = std::env::temp_dir().join("rupta_wrapper_debug.log");
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&debug_log)
        .and_then(|mut f| {
            use std::io::Write;
            writeln!(f, "[rupta] pta main() running, entry_func={:?}, class_pag_output={:?}", options.entry_func, options.class_pag_output)
        });

    // Let arguments supplied on the command line override the environment variable.
    let mut args = env::args_os()
        .enumerate()
        .map(|(i, arg)| {
            arg.into_string().unwrap_or_else(|arg| {
                early_dcx.early_fatal(format!("Argument {i} is not valid Unicode: {arg:?}"))
            })
        })
        .collect::<Vec<_>>();

    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we remove it if present.
    if args.len() > 1 && std::path::Path::new(&args[1]).file_stem() == Some("rustc".as_ref()) {
        args.remove(1);
    }

    let mut rustc_command_line_arguments = options.parse_from_args(&args[1..], false);
    info!("PTA Options: {:?}", options);

    let exit_code = rustc_driver::catch_with_exit_code(move || {
        // Add back the binary name
        rustc_command_line_arguments.insert(0, args[0].clone());

        // Add rustc arguments supplied via the MIRAI_FLAGS environment variable
        rustc_command_line_arguments.extend(rustc_args);

        let sysroot: String = "--sysroot".into();
        if !rustc_command_line_arguments.iter().any(|arg| arg.starts_with(&sysroot)) {
            // Tell compiler where to find the std library and so on.
            // The compiler relies on the standard rustc driver to tell it, so we have to do likewise.
            rustc_command_line_arguments.push(sysroot);
            rustc_command_line_arguments.push(util::find_sysroot());
        }

        let always_encode_mir: String = "always-encode-mir".into();
        if std::env::var_os("RCPTA_NO_ALWAYS_ENCODE_MIR").is_none()
            && !rustc_command_line_arguments.iter().any(|arg| arg.ends_with(&always_encode_mir))
        {
            // Tell compiler to emit MIR into crate for every function with a body.
            rustc_command_line_arguments.push("-Z".into());
            rustc_command_line_arguments.push(always_encode_mir);
        }
        debug!("rustc command line arguments: {:?}", rustc_command_line_arguments);
        
        let mut callbacks = PTACallbacks::new(options);
        rustc_driver::run_compiler(&rustc_command_line_arguments, &mut callbacks);
    });

    std::process::exit(exit_code);
}
