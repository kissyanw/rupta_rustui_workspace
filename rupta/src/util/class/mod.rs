// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class analysis module
//! 
//! This module contains all utilities for analyzing DSL class syntax,
//! including class identification, type system, call graph, and pointer system.

pub mod analysis;
pub mod call_graph;
pub mod ptr_system;
pub mod type_system;
pub mod dsl_inheritance_graph;
pub mod cast_safety_log;

// Re-export commonly used items for convenience
pub use analysis::*;
pub use call_graph::ClassCallGraph;
pub use type_system::ClassTypeSystem;
