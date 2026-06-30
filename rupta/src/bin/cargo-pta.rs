#![feature(rustc_private)]

// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! This provides an implementation for the "cargo pta" subcommand.
//! 
//! The subcommand is the same as "cargo check" but with three differences:
//! 1) It implicitly adds the options "-Z always_encode_mir" to the rustc invocation.
//! 2) It calls `pta` rather than `rustc` for all the targets of the current package.
//! 3) It runs `cargo test --no-run` for test targets.

use cargo_metadata::Package;
use log::info;
use serde_json;
use std::env;
use std::ffi::OsString;
use std::ops::Index;
use std::path::Path;
use std::process::{Command, Stdio};

use rupta::util;

/// The help message for `cargo-pta`
const CARGO_PTA_HELP: &str = r#"Pointer analysis tool for Rust programs
Usage:
    cargo pta
"#;

/// Set the environment variable `PTA_BUILD_STD` to enable the building of std library when running pta.
const PTA_BUILD_STD: &str = "PTA_BUILD_STD";

/// Raise process stack limit so that spawned pta inherits it (avoids overflow on large crates).
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

pub fn main() {
    maybe_raise_stack_limit();

    if std::env::args().take_while(|a| a != "--").any(|a| a == "--help" || a == "-h") {
        println!("{}", CARGO_PTA_HELP);
        return;
    }

    match std::env::args().nth(1).as_ref().map(AsRef::<str>::as_ref) {
        Some(s) if s.ends_with("pta") => {
            // Get here for the top level cargo execution, i.e. "cargo pta".
            call_cargo();
        }
        Some(s) if s.ends_with("rustc") => {
            // 'cargo rustc ..' redirects here because RUSTC_WRAPPER points to this binary.
            // execute rustc with PTA applicable parameters for dependencies and call PTA
            // to analyze targets in the current package.
            call_rustc_or_pta();
        }
        Some(arg) => {
            eprintln!(
                "`cargo-pta` called with invalid first argument: {arg}; please only invoke this binary through `cargo pta`" 
            );
        }
        _ => {
            eprintln!("current args: {:?}", std::env::args());
            eprintln!("`cargo-pta` called without first argument; please only invoke this binary through `cargo pta`");
        }
    }
}

/// Read the toml associated with the current directory and
/// recursively execute cargo for each applicable package target/workspace member in the toml
fn call_cargo() {
    let manifest_path = get_arg_flag_value("--manifest-path").map(|m| Path::new(&m).canonicalize().unwrap());

    let mut cmd = cargo_metadata::MetadataCommand::new();
    if let Some(ref manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }

    let metadata = match cmd.exec() {
        Ok(metadata) => metadata,
        Err(err) => {
            eprintln!("Could not obtain Cargo metadata; likely an ill-formed manifest: {err}");
            std::process::exit(1);
        }
    };

    // If a binary is specified, analyze this binary only.
    if let Some(target) = get_arg_flag_value("--bin") {
        call_cargo_on_target(&target, "bin");
        return;
    }

    // If a test target is specified, analyze this test only (do not run cargo for every target).
    if let Some(target) = get_arg_flag_value("--test") {
        call_cargo_on_target(&target, "test");
        return;
    }

    if let Some(root) = metadata.root_package() {
        call_cargo_on_each_package_target(root);
        return;
    }

    // There is no root, this must be a workspace, so call_cargo_on_each_package_target on each workspace member
    for package_id in &metadata.workspace_members {
        let package = metadata.index(package_id);
        call_cargo_on_each_package_target(package);
    }
}

fn call_cargo_on_each_package_target(package: &Package) {
    let lib_only = has_arg_flag("--lib");
    for target in &package.targets {
        let kind_str = target
            .kind
            .first()
            .map(|k| k.to_string())
            .expect("bad cargo metadata: target::kind");
        if lib_only && kind_str != "lib" {
            continue;
        }
        call_cargo_on_target(&target.name, &kind_str);
    }
}

fn call_cargo_on_target(target: &String, kind: &str) {
    // Build a cargo command for target
    let mut cmd = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")));
    match kind {
        "bin" => {
            cmd.arg("check");
            if get_arg_flag_value("--bin").is_none() {
                cmd.arg("--bin").arg(target);
            }
        }
        "lib" => {
            cmd.arg("check");
            cmd.arg("--lib");
        }
        "test" => {
            cmd.arg("test");
            cmd.arg("--no-run");
        }
        _ => {
            return;
        }
    }
    cmd.arg("--verbose");

    let mut args = std::env::args().skip(2);
    // Add cargo args to cmd until first `--`.
    for arg in args.by_ref() {
        if arg == "--" {
            break;
        }
        if arg == "--lib" {
            continue;
        }
        cmd.arg(arg);
    }

    // Enable Cargo to compile the standard library from source code as part of a crate graph compilation.
    if env::var(PTA_BUILD_STD).is_ok() {
        cmd.arg("-Zbuild-std");

        if !has_arg_flag("--target") {
            let toolchain_target = toolchain_target().expect("could not get toolchain target");
            cmd.arg("--target").arg(toolchain_target);
        }
    }

    // Serialize the remaining args into an environment variable.
    let args_vec: Vec<String> = args.collect();
    eprintln!("[rupta][cargo-pta] PTA_FLAGS args={:?}", args_vec);
    cmd.env(
        "PTA_FLAGS",
        serde_json::to_string(&args_vec).expect("failed to serialize args"),
    );

    // Force cargo to recompile all dependencies with PTA friendly flags and prefer dynamic linkage
    let mut rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
    if !rustflags.trim().is_empty() {
        rustflags.push(' ');
    }
    rustflags.push_str("-Z always_encode_mir -C prefer-dynamic");
    cmd.env("RUSTFLAGS", rustflags);

    // Pass through RUST_LOG and RUST_LOG_STYLE environment variables for logging
    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        cmd.env("RUST_LOG", rust_log);
    }
    if let Ok(rust_log_style) = std::env::var("RUST_LOG_STYLE") {
        cmd.env("RUST_LOG_STYLE", rust_log_style);
    }
    // Also pass PTA_LOG for backward compatibility
    if let Ok(pta_log) = std::env::var("PTA_LOG") {
        cmd.env("PTA_LOG", pta_log);
    }

    // Replace the rustc executable through RUSTC_WRAPPER environment variable so that rustc
    // calls generated by cargo come back to cargo-pta.
    let path = std::env::current_exe().expect("current executable path invalid");
    cmd.env("RUSTC_WRAPPER", path);

    // Communicate the name of the root crate to the calls to cargo-pta that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("PTA_CRATE", target.replace('-', "_"));

    // Communicate the target kind of the root crate to the calls to cargo-pta that are invoked via
    // the RUSTC_WRAPPER setting.
    cmd.env("PTA_TARGET_KIND", kind);

    // Set the tool chain to be compatible with pta
    if let Some(toolchain) = option_env!("RUSTUP_TOOLCHAIN") {
        cmd.env("RUSTUP_TOOLCHAIN", toolchain);
    }

    // Execute cmd
    info!("cmd: {:?}", cmd);
    let exit_status = cmd
        .spawn()
        .expect("could not run cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc_or_pta() {
    if let Some(crate_name) = get_arg_flag_value("--crate-name") {
        if let Ok(pta_crate) = std::env::var("PTA_CRATE") {
            if crate_name.eq(&pta_crate) {
                if let Ok(kind) = std::env::var("PTA_TARGET_KIND") {
                    if let Some(t) = get_arg_flag_value("--crate-type") {
                        let crate_type = t.as_str();
                        let kind_matches = kind == crate_type
                            || (kind == "test" && crate_type == "bin");
                        if kind_matches {
                            call_pta();
                            return;
                        }
                    } else if kind == "test" {
                        call_pta();
                        return;
                    }
                }
            }
        }
    }
    call_rustc()
}

fn call_pta() {
    let mut path = std::env::current_exe().expect("current executable path invalid");
    let extension = path.extension().map(|e| e.to_owned());
    path.pop(); // remove the cargo_pta bit
    path.push("pta");
    if let Some(ext) = extension {
        path.set_extension(ext);
    }
    let mut cmd = Command::new(path);
    eprintln!("[rupta][cargo-pta] invoke pta at {:?}", cmd.get_program());
    if let Ok(v) = std::env::var("PTA_FLAGS") {
        cmd.env("PTA_FLAGS", v);
    } else {
        cmd.env_remove("PTA_FLAGS");
    }
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run pta")
        .wait()
        .expect("failed to wait for pta");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

fn call_rustc() {
    // todo: invoke the rust compiler for the appropriate tool chain?
    let mut cmd = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc")));
    cmd.args(std::env::args().skip(2));
    let exit_status = cmd
        .spawn()
        .expect("could not run rustc")
        .wait()
        .expect("failed to wait for rustc");

    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(-1))
    }
}

/// Determines whether a flag `name` is present before `--`.
/// For example, has_arg_flag("-v")
fn has_arg_flag(name: &str) -> bool {
    let mut args = std::env::args().take_while(|val| val != "--");
    args.any(|val| val == name)
}

/// Gets the value of `name`.
/// `--name value` or `--name=value`
fn get_arg_flag_value(name: &str) -> Option<String> {
    let mut args = std::env::args().take_while(|val| val != "--");
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => return None,
        };
        if !arg.starts_with(name) {
            continue;
        }
        // Strip `name`.
        let suffix = &arg[name.len()..];
        if suffix.is_empty() {
            // This argument is `name` and the next one is the value.
            return args.next();
        } else if let Some(arg_value) = suffix.strip_prefix('=') {
            return Some(arg_value.to_owned());
        }
    }
}

/// Returns the target of the toolchain, e.g. "x86_64-unknown-linux-gnu".
fn toolchain_target() -> Option<String> {
    let sysroot = util::find_sysroot();

    // get the supported rustup targets
    let output = String::from_utf8(
        Command::new("rustup")
            .arg("target")
            .arg("list")
            .stdout(Stdio::piped())
            .output()
            .expect("could not run 'rustup target list'")
            .stdout,
    )
    .unwrap();

    let target = output.lines().find_map(|line| {
        let target = line.split_whitespace().next().unwrap().to_owned();
        if sysroot.ends_with(&target) {
            Some(target)
        } else {
            None
        }
    });

    target
}
