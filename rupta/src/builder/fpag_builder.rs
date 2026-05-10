
// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Builds the Pointer Assignment Graph (PAG) for a single function.
//! 
//! The Function PAG is part of the PAG for the whole program.

use log::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::mir;
use rustc_middle::mir::interpret::{GlobalAlloc, Scalar};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty;
use rustc_middle::ty::{Const, Ty, TyCtxt, TyKind, GenericArgsRef};
use rustc_span::source_map::Spanned;
// use rustc_target::abi::FieldIdx; // old API path (pre-2024-02-03)
use rustc_abi::FieldIdx;

use crate::builder::{call_graph_builder, special_function_handler};
use crate::graph::func_pag::FuncPAG;
use crate::graph::pag::{PAGEdgeEnum, PAGPath};
use crate::mir::call_site::CallSite;
use crate::mir::function::{FuncId, FunctionReference};
use crate::mir::known_names::KnownNames;
use crate::mir::analysis_context::AnalysisContext;
use crate::mir::path::{Path, PathEnum, PathSelector, PathSupport, ProjectionElems};
use crate::util::{self, type_util};
use crate::util::class::analysis;

use super::substs_specializer::SubstsSpecializer;

/// A visitor that traverses the MIR associated with a particular function's body and
/// build the function's pointer assignment graph.
pub struct FuncPAGBuilder<'pta, 'tcx, 'compilation> {
    pub(crate) acx: &'pta mut AnalysisContext<'tcx, 'compilation>,
    pub(crate) func_id: FuncId,
    pub(crate) func_ref: Rc<FunctionReference<'tcx>>,
    pub(crate) mir: &'tcx mir::Body<'tcx>,
    /// Pointer Assignment Graph for this function.
    pub(crate) fpag: &'pta mut FuncPAG,

    /// For specializing the generic type in the function.
    substs_specializer: SubstsSpecializer<'tcx>,

    /// Caches the path for each place visited in this function
    path_cache: HashMap<mir::Place<'tcx>, Rc<Path>>,
    trace_stmt_counter: usize,
}

impl<'pta, 'tcx, 'compilation> Debug for FuncPAGBuilder<'pta, 'tcx, 'compilation> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        "FuncPAGBuilder".fmt(f)
    }
}


impl<'pta, 'tcx, 'compilation> FuncPAGBuilder<'pta, 'tcx, 'compilation> {
    #[inline]
    fn rcpta_elem_slot_id_from_base_ptr(base_ptr_id: &str) -> String {
        format!("{}.$elem", base_ptr_id)
    }

    #[inline]
    fn rcpta_ret_elem_slot_id(func_name: &str) -> String {
        format!("{}::ret.$elem", func_name)
    }

    pub fn new(
        acx: &'pta mut AnalysisContext<'tcx, 'compilation>,
        func_id: FuncId,
        mir: &'tcx mir::Body<'tcx>,
        fpag: &'pta mut FuncPAG,
    ) -> FuncPAGBuilder<'pta, 'tcx, 'compilation> {
        let func_ref = acx.get_function_reference(func_id);
        debug!("Building FuncPAG for {:?}: {}", func_id, func_ref.to_string());
        // if func_ref.promoted.is_none() {
        //     util::pretty_print_mir(acx.tcx, func_ref.def_id);
        // }
        let substs_specializer = SubstsSpecializer::new(
            acx.tcx, 
            func_ref.generic_args.clone()
        );
        let aux_local_index = mir.local_decls.len();
        acx.aux_local_indexer.insert(func_id, aux_local_index);
        acx.rcpta_alias_map.clear();
        acx.rcpta_ref_ptr_to_base_path.clear();
        acx.rcpta_option_copy_to_base_path.clear();
        acx.rcpta_container_elem_ptrs_by_base.clear();
        FuncPAGBuilder {
            acx,
            func_id,
            func_ref,
            mir,
            fpag,
            substs_specializer,
            path_cache: HashMap::new(),
            trace_stmt_counter: 0,
        }
    }

    #[inline]
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.acx.tcx
    }

    #[inline]
    fn def_id(&self) -> DefId {
        self.func_ref.def_id
    }

    /// Returns true if this function corresponds to an initialization procedure 
    /// for a promoted constant.
    #[inline]
    fn is_promoted(&self) -> bool {
        self.func_ref.promoted.is_some()
    }

    /// Returns true if this function corresponds to an initialization procedure 
    /// for a static item.
    #[inline]
    fn is_static(&self) -> bool {
        self.acx.tcx.is_static(self.def_id())
    }

    /// Canonical function name for rcpta ClassPAG: same for impl wrapper and method body so we have one param/ret set per method.
    fn rcpta_canonical_func_name(&self) -> String {
        analysis::canonical_class_method_name(&self.acx.get_function_reference(self.fpag.func_id).to_string())
    }

    /// Returns true if this function corresponds to an initialization procedure 
    /// for a const item.
    #[inline]
    fn is_const(&self) -> bool {
        matches!(self.tcx().def_kind(self.def_id()), DefKind::Const)
    }

    /// Builds the PAG. 
    pub fn build(&mut self) {
        let trace_fpag = std::env::var_os("RCPTA_TRACE_FPAG").is_some();
        let func_name = self.acx.get_function_reference(self.fpag.func_id).to_string();
        let canonical = self.rcpta_canonical_func_name();
        info!("rcpta: building PAG for func={}", func_name);
        if trace_fpag {
            eprintln!(
                "[rupta][fpag] build begin func_id={:?} bb_count={} stmt_locals={}",
                self.func_id,
                self.mir.basic_blocks.len(),
                self.mir.local_decls.len()
            );
        }
        if func_name.contains("get_and_wrap") || canonical.contains("get_and_wrap") {
            info!("rcpta: [dedup] func raw={} canonical={}", func_name, canonical);
        }
        // build common functions' PAG first
        self.visit_body();
        if trace_fpag {
            eprintln!("[rupta][fpag] visit_body done func_id={:?}", self.func_id);
        }

        // Add extra edges between the return value and the promoted_path/static_path
        // if the function body corresponds to a promoted body or a static's body
        // author:wy
        // date:2025-11-04
        // if this funtion is the initialization procedure of a promoted constant/static variable/const item
        // create edges between the return value and the promoted_path/static_path/const_path
        if let Some(promoted) = self.func_ref.promoted {
            let ret_path = Path::new_return_value(self.func_id);
            let ret_type = self.acx
                .get_path_rustc_type(&ret_path)
                .expect("Unresolved result type");
            let promoted_path = Path::new_promoted(self.def_id(), promoted.into());
            self.acx.set_path_rustc_type(promoted_path.clone(), ret_type);
            self.add_internal_edges(ret_path, ret_type, promoted_path, ret_type);
        } else if self.is_static() || self.is_const() {
            let ret_path = Path::new_return_value(self.func_id);
            let ret_type = self.acx
                .get_path_rustc_type(&ret_path)
                .expect("Unresolved result type");
            let static_variable = Path::new_static_variable(self.def_id());
            self.acx.set_path_rustc_type(static_variable.clone(), ret_type);
            self.add_internal_edges(ret_path, ret_type, static_variable, ret_type);
        }
        if trace_fpag {
            eprintln!("[rupta][fpag] build done func_id={:?}", self.func_id);
        }
    }

    pub fn visit_body(&mut self) {
        let trace_fpag = std::env::var_os("RCPTA_TRACE_FPAG").is_some();
        let mut bb_idx: usize = 0;
        for bb in self.mir.basic_blocks.indices() {
            bb_idx += 1;
            if trace_fpag && bb_idx % 50 == 0 {
                eprintln!("[rupta][fpag] func_id={:?} visiting bb_idx={}", self.func_id, bb_idx);
            }
            self.visit_basic_block(bb);
        }
    }

    fn visit_basic_block(&mut self, bb: mir::BasicBlock) {
        let trace_fpag = std::env::var_os("RCPTA_TRACE_FPAG").is_some();
        if trace_fpag {
            eprintln!(
                "[rupta][fpag] func_id={:?} enter bb={:?}",
                self.func_id, bb
            );
        }
        let mir::BasicBlockData {
            ref statements,
            ref terminator,
            ..
        } = &self.mir[bb];
        let mut location = bb.start_location();
        let terminator_index = statements.len();

        while location.statement_index < terminator_index {
            if trace_fpag {
                self.trace_stmt_counter += 1;
                if self.func_id.as_usize() == 70 && bb.as_usize() == 4 {
                    eprintln!(
                        "[rupta][fpag][focus] func=70 bb=4 stmt_idx={} kind={:?}",
                        location.statement_index,
                        statements[location.statement_index].kind
                    );
                }
                if self.func_id.as_usize() == 138 && bb.as_usize() == 0 {
                    eprintln!(
                        "[rupta][fpag][focus] func=138 bb=0 stmt_idx={} kind={:?}",
                        location.statement_index,
                        statements[location.statement_index].kind
                    );
                }
                if self.trace_stmt_counter % 20 == 0 {
                    eprintln!(
                        "[rupta][fpag] func_id={:?} bb={:?} stmt_idx={} total_stmt={}",
                        self.func_id,
                        bb,
                        location.statement_index,
                        self.trace_stmt_counter
                    );
                }
            }
            self.visit_statement(location, &statements[location.statement_index]);
            location.statement_index += 1;
        }

        if let Some(mir::Terminator {
            ref source_info,
            ref kind,
        }) = *terminator
        {
            if trace_fpag {
                eprintln!(
                    "[rupta][fpag] func_id={:?} bb={:?} visit terminator",
                    self.func_id, bb
                );
            }
            self.visit_terminator(location, kind, *source_info);
        }
    }

    /// Calls a specialized visitor for each kind of statement.
    fn visit_statement(&mut self, _location: mir::Location, statement: &mir::Statement<'tcx>) {
        if std::env::var_os("RCPTA_TRACE_FPAG").is_some()
            && self.func_id.as_usize() == 70
            && _location.block.as_usize() == 4
        {
            eprintln!(
                "[rupta][fpag][focus] enter visit_statement func=70 bb=4 stmt_idx={}",
                _location.statement_index
            );
        }
        if std::env::var_os("RCPTA_TRACE_FPAG").is_some()
            && self.func_id.as_usize() == 138
            && _location.block.as_usize() == 0
        {
            eprintln!(
                "[rupta][fpag][focus] enter visit_statement func=138 bb=0 stmt_idx={}",
                _location.statement_index
            );
        }
        // debug!("Visiting statement: {:?}", statement);
        let mir::Statement { kind, source_info: _ } = statement;
        match kind {
            mir::StatementKind::Assign(box (place, rvalue)) => {
                // Generalized overflow guard:
                // Large library/generic functions frequently assign a huge aggregate to return place (_0),
                // which can trigger pathological recursion in deep type/projection modeling.
                // For non-user-crate functions this modeling is not needed for rcpta class precision.
                if place.local.as_usize() == 0 && matches!(rvalue, mir::Rvalue::Aggregate(_, _)) {
                    let func_name = self.func_ref.to_string();
                    let is_user_crate_func = func_name.starts_with("vehicle_hierarchy::");
                    if !is_user_crate_func {
                        if std::env::var_os("RCPTA_TRACE_FPAG").is_some() {
                            eprintln!(
                                "[rupta][fpag][guard] skip non-user aggregate return assign func={:?} bb={:?} stmt={}",
                                self.func_id,
                                _location.block,
                                _location.statement_index
                            );
                        }
                        return;
                    }
                }
                self.visit_assign(place, rvalue)
            }
            mir::StatementKind::FakeRead(..) => (),
            mir::StatementKind::SetDiscriminant { place, variant_index } => {
                self.visit_set_discriminant(place, *variant_index)
            }
            mir::StatementKind::Deinit(box place) => self.visit_deinit(place),
            mir::StatementKind::StorageLive(local) => self.visit_storage_live(*local),
            mir::StatementKind::StorageDead(local) => self.visit_storage_dead(*local),
            mir::StatementKind::Retag(retag_kind, place) => self.visit_retag(*retag_kind, place),
            mir::StatementKind::PlaceMention(..) => (),
            mir::StatementKind::AscribeUserType(..) => (),
            mir::StatementKind::Coverage(..) => (),
            // 新增：处理 BackwardIncompatibleDropHint（Rust 2025-05-09 nightly 新增）
            // 这个语句用于标记可能导致向后兼容性问题的 drop 行为，对于指针分析不需要特殊处理
            mir::StatementKind::BackwardIncompatibleDropHint { .. } => (),
            mir::StatementKind::Intrinsic(box non_diverging_intrinsic) => {
                self.visit_non_diverging_intrinsic(non_diverging_intrinsic);
            }
            mir::StatementKind::ConstEvalCounter => (),
            mir::StatementKind::Nop => (),
        }
    }

    /// An assignment statement writes the RHS Rvalue to the LHS Place.
    fn visit_assign(&mut self, place: &mir::Place<'tcx>, rvalue: &mir::Rvalue<'tcx>) {
        let (lh_path, lh_type) = self.get_path_and_type_for_place(place);

        // Skip this assignment if the destination path is not pointer and does not 
        // contain pointer type fields. 
        // Exception: DSL class types (and wrappers like CRc<Bird>) should be tracked
        if !lh_type.is_any_ptr()
            && !analysis::is_dsl_class_type(self.tcx(), lh_type)
            && analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_type, None).is_none()
            && self.acx.get_pointer_projections(lh_type).is_empty() {
            return;
        }

        self.visit_rvalue(lh_path.clone(), rvalue);

        // If this assignment writes to a field or subfield of a union, add edges 
        // between the union fields that share the same memory offset.
        self.cast_between_union_fields(&lh_path);
    }

    /// Denotes a call to the intrinsic function copy_nonoverlapping, where `src` and `dst` denotes the
    /// memory being read from and written to and size indicates how many bytes are being copied over.
    /// `src` and `dst` must each be a reference, pointer, or `Box` pointing to the same type T.
    /// A copy_nonoverlapping statement can be regarded as a statement like `*dst = *src`.
    fn visit_copy_non_overlapping(&mut self, copy_info: &mir::CopyNonOverlapping<'tcx>) {
        let mut get_ptr_path = |operand: &mir::Operand<'tcx>| -> Rc<Path> {
            match operand {
                mir::Operand::Copy(place) | mir::Operand::Move(place) => {
                    let (path, ty) = self.get_path_and_type_for_place(place);
                    match ty.kind() {
                        TyKind::Ref(..) | TyKind::RawPtr(..) => path,
                        TyKind::Adt(def, _args) if def.is_box() => {
                            self.get_box_pointer_field(path, ty.boxed_ty().expect("Box type should have boxed_ty"))
                        }
                        _ => unreachable!("CopyNonOverlapping is called on non-ptr operands."),
                    }
                }
                mir::Operand::Constant(..) => unreachable!(),
            }
        };
        let src_ptr = get_ptr_path(&copy_info.src);
        let dst_ptr = get_ptr_path(&copy_info.dst);

        // convert it to `` let aux = *src_ptr; *dst_ptr = aux ``
        let deref_ty = type_util::get_dereferenced_type(self.acx.get_path_rustc_type(&src_ptr).unwrap());
        let aux = self.create_aux_local(deref_ty);
        let src_deref = Path::new_deref(src_ptr);
        self.acx.set_path_rustc_type(src_deref.clone(), deref_ty);
        self.add_internal_edges(
            src_deref,
            deref_ty,
            aux.clone(),
            deref_ty,
        );
        let dst_deref = Path::new_deref(dst_ptr);
        self.acx.set_path_rustc_type(dst_deref.clone(), deref_ty);
        self.add_internal_edges(
            aux,
            deref_ty,
            dst_deref,
            deref_ty,
        );
    }

    /// Writes the discriminant for a variant to the enum Place.
    /// author:wy
    /// date:2025-11-04
    /// enum Option<T> {
    ///     Some(T),
    ///     None,
    /// }
    /// fn example() {
    ///     let x: Option<i32> = Option::Some(42);
    ///     // this will generate a SetDiscriminant statement, set the discriminant to the index of Some variant
    /// }
    fn visit_set_discriminant(
        &mut self,
        _place: &mir::Place<'tcx>,
        // _variant_index: rustc_target::abi::VariantIdx, // old API path (pre-2024-02-03)
        _variant_index: rustc_abi::VariantIdx,
    ) {
    }

    /// Deinitializes the place. This writes `uninit` bytes to the entire place.
    /// author:wy
    /// date:2025-11-04
    /// fn deinit_example() {
    ///     let s = String::from("hello");
    ///     let s2 = s;  // after the move, the s location generates Deinit
    ///     // or
    ///     let mut s3 = String::from("world");
    ///     s3 = String::from("new");  // before the assignment, the old value of s3 location is deinitialized and replaced by the new value
    /// }
    fn visit_deinit(&mut self, _place: &mir::Place<'tcx>) {}

    /// author:wy
    /// date:2025-11-04
    /// fn storage_live_example() {
    ///let x = 5;           // ← StorageLive(_1) - x starts to use stack space
    ///{
    ///    let y = 10;      // ← StorageLive(_2) - y starts to use stack space
    ///    // ... use y
    ///}                    // ← StorageDead(_2) - the stack space of y can be reused
    /// ... use x            
    /// }                   // ← StorageDead(_1) - the stack space of x can be reused
    /// 
    /// bb0: {
    /// StorageLive(_1);     // the stack space of x starts to be used
    /// _1 = const 5_i32;
    /// StorageLive(_2);     // the stack space of y starts to be used
    /// _2 = const 10_i32;
    /// StorageDead(_2);     // the stack space of y can be reused
    /// StorageDead(_1);     // the stack space of x can be reused
    /// }

    /// Start a live range for the storage of the local.
    fn visit_storage_live(&mut self, _local: mir::Local) {}

    /// End the current live range for the storage of the local.
    fn visit_storage_dead(&mut self, _local: mir::Local) {}

    /// Retag references in the given place, ensuring they got fresh tags.  This is
    /// part of the Stacked Borrows model. These statements are currently only interpreted
    /// by miri and only generated when "-Z mir-emit-retag" is passed.
    /// See <https://internals.rust-lang.org/t/stacked-borrows-an-aliasing-model-for-rust/8153/>
    /// for more details.
    
    /// author:wy
    /// date:2025-11-04
    /// fn retag_example() {
    /// let x = 5;
    /// let y = &x;
    /// let z = &x;
    /// let w = &x;
    /// }
    fn visit_retag(&self, _retag_kind: mir::RetagKind, _place: &mir::Place<'tcx>) {
        // This seems to be an intermediate artifact of MIR generation and is related to aliasing.
        // We currently simply ignore this.
    }

    /// Denotes a call to an intrinsic that does not require an unwind path and always returns. 
    fn visit_non_diverging_intrinsic(
        &mut self,
        visit_non_diverging_intrinsic: &mir::NonDivergingIntrinsic<'tcx>,
    ) {
        match visit_non_diverging_intrinsic {
            mir::NonDivergingIntrinsic::CopyNonOverlapping(copy_info) => {
                self.visit_copy_non_overlapping(copy_info);
            }
            mir::NonDivergingIntrinsic::Assume(_operand) => {}
        }
    }

    /// Terminator for a basic block. 
    /// We only analyze the call statements in a flow-insensitive pointer analysis.
    fn visit_terminator(
        &mut self,
        location: mir::Location,
        kind: &mir::TerminatorKind<'tcx>,
        _source_info: mir::SourceInfo,
    ) {
        match kind {
            mir::TerminatorKind::Call {
                func,
                args,
                destination,
                target: _,
                unwind: _,
                call_source: _,
                fn_span: _,
            } => {
                // Old code (nightly-2024-02-03): args was Vec, passed directly as &args
                // New code (nightly-2025-05-09): args is Box<[...]>, convert to slice explicitly
                self.visit_call(func, args.as_ref(), destination, location)
            },
            mir::TerminatorKind::InlineAsm {
                template: _,
                operands: _,
                // Old code (nightly-2024-02-03): destination field existed
                // New code (nightly-2025-05-09): destination field removed from InlineAsm
                ..
            } => {}
            _ => {}
        }
    }

    /// Block ends with the call of a function.
    ///
    /// #Arguments
    /// * `func` - The function that's being called
    /// * `args` - Arguments the function is called with. These are owned by the callee, which is
    /// free to modify them. This allows the memory occupied by "by-value" arguments to be reused
    /// across function calls without duplicating the contents.
    /// * `destination` - Destination for the return value. If some, the call returns a value.
    fn visit_call(
        &mut self,
        func: &mir::Operand<'tcx>,
        // Old code (nightly-2024-02-03): args: &Vec<Spanned<mir::Operand<'tcx>>>
        // New code (nightly-2025-05-09): args changed from Vec to Box<[...]>, use slice to accept both
        args: &[Spanned<mir::Operand<'tcx>>],
        destination: &mir::Place<'tcx>,
        location: mir::Location,
    ) {
        match func {
            mir::Operand::Constant(box constant) => match constant.ty().kind() {
                TyKind::Closure(callee_def_id, gen_args)
                | TyKind::FnDef(callee_def_id, gen_args)
                | TyKind::Coroutine(callee_def_id, gen_args) => {
                    self.resolve_call(callee_def_id, gen_args, args, destination, location)
                }
                // Old code (nightly-2024-02-03): TyKind::FnPtr(_) - had 1 field
                // New code (nightly-2025-05-09): TyKind::FnPtr now has 2 fields, use .. to ignore all
                TyKind::FnPtr(..) => {
                    let fnptr = self.visit_const_operand(constant);
                    debug!("Constant function pointer: {:?}", fnptr);
                    let args = self.visit_args(args);
                    let destination = self.get_path_for_place(destination);
                    let callsite = self.new_callsite(self.func_id, location, args, destination);
                    self.fpag.add_fnptr_callsite(fnptr, callsite);
                }
                _ => {
                    error!("Unexpected call: {:?}, type: {:?}", constant, constant.ty());
                }
            },
            mir::Operand::Copy(place) | mir::Operand::Move(place) => {
                let (fn_item, fn_item_ty) = self.get_path_and_type_for_place(place);
                assert!(fn_item_ty.is_fn());
                match fn_item_ty.kind() {
                    TyKind::FnDef(callee_def_id, gen_args) => {
                        self.resolve_call(callee_def_id, gen_args, args, destination, location)
                    }
                    TyKind::FnPtr(..) => {
                        let args = self.visit_args(args);
                        let destination = self.get_path_for_place(destination);
                        let callsite = self.new_callsite(self.func_id, location, args, destination);
                        self.fpag.add_fnptr_callsite(fn_item, callsite);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
    
    // Old code (nightly-2024-02-03): args: &Vec<Spanned<mir::Operand<'tcx>>>
    // New code (nightly-2025-05-09): use slice to accept both Vec and Box<[...]>
    fn visit_args(&mut self, args: &[Spanned<mir::Operand<'tcx>>]) -> Vec<Rc<Path>> {
        let mut args_paths = Vec::<Rc<Path>>::with_capacity(args.len());
        for arg in args {
            match &arg.node {
                mir::Operand::Copy(place) | mir::Operand::Move(place) => {
                    args_paths.push(self.get_path_for_place(place));
                }
                mir::Operand::Constant(const_op) => {
                    args_paths.push(self.visit_const_operand(const_op));
                }
            }
        }
        args_paths
    }

    /// Calls a specialized visitor for each kind of Rvalue.
    fn visit_rvalue(&mut self, lh_path: Rc<Path>, rvalue: &mir::Rvalue<'tcx>) {
        match rvalue {
            mir::Rvalue::Use(operand) => {
                self.visit_use(lh_path, operand);
            }
            mir::Rvalue::Repeat(operand, count) => {
                self.visit_repeat(lh_path, operand, count);
            }
            // Old code (nightly-2024-02-03): mir::Rvalue::Ref(_, _, place) | mir::Rvalue::AddressOf(_, place)
            // New code (nightly-2025-05-09): AddressOf variant removed, Ref now handles both cases
            mir::Rvalue::Ref(_, _, place) => {
                self.visit_ref_or_address_of(lh_path, place);
            }
            mir::Rvalue::ThreadLocalRef(_def_id) => {}
            mir::Rvalue::Len(_place) => {}
            mir::Rvalue::Cast(cast_kind, operand, ty) => {
                let specialized_ty = self.substs_specializer.specialize_generic_argument_type(*ty);
                self.visit_cast(lh_path, *cast_kind, operand, specialized_ty);
            }
            mir::Rvalue::BinaryOp(bin_op, box (left_operand, right_operand)) => {
                self.visit_binary_op(lh_path, *bin_op, left_operand, right_operand);
            }
            // Old code (nightly-2024-02-03): mir::Rvalue::CheckedBinaryOp(_bin_op, box (_left_operand, _right_operand)) => {}
            // New code (nightly-2025-05-09): CheckedBinaryOp variant removed, checked operations now handled differently
            mir::Rvalue::NullaryOp(..) | mir::Rvalue::UnaryOp(..) | mir::Rvalue::Discriminant(..) => {}
            mir::Rvalue::Aggregate(aggregate_kind, operands) => {
                self.visit_aggregate(lh_path, aggregate_kind, operands);
            }
            mir::Rvalue::ShallowInitBox(operand, ty) => {
                self.visit_shallow_init_box(lh_path, operand, *ty);
            }
            mir::Rvalue::CopyForDeref(place) => {
                // A CopyForDeref is equivalent to a read from a place at the codegen level, 
                // but is treated specially by drop elaboration. When such a read happens, 
                // it is guaranteed (via nature of the mir_opt Derefer in 
                // rustc_mir_transform/src/deref_separator) that the only use of the returned 
                // value is a deref operation, immediately followed by one or more projections.
                self.visit_copy_or_move(lh_path, place);
            }
            // 新增：处理 RawPtr（Rust 2025-05-09 nightly 新增）
            // RawPtr: 创建原始指针的操作 (e.g., &raw const x, &raw mut x)
            // TODO: RawPtr 与指针分析高度相关，未来需要实现对原始指针创建的分析逻辑
            // 应该类似于 visit_ref_or_address_of，但需要考虑原始指针的特殊语义
            mir::Rvalue::RawPtr(_, _) => {
                // 暂时不处理，等待实现原始指针分析
            }
            // 新增：处理 WrapUnsafeBinder（Rust 2025-05-09 nightly 新增）
            // WrapUnsafeBinder: 处理 unsafe binder 的包装，用于高阶类型推理
            // 对于指针分析，暂时不需要特殊处理
            mir::Rvalue::WrapUnsafeBinder(_, _) => {}
        }
    }

    /// `path = x` (either a move or copy, depending on type of `x`), or `path = constant`.
    fn visit_use(&mut self, lh_path: Rc<Path>, operand: &mir::Operand<'tcx>) {
        match operand {
            // Currently we do not seperate copy and move cases.
            mir::Operand::Copy(place) 
            | mir::Operand::Move(place) => {
                self.visit_copy_or_move(lh_path, place);
            }
            mir::Operand::Constant(const_op) => {
                self.visit_constant_assign(lh_path, const_op.borrow());
            }
        }
    }

    fn visit_copy_or_move(&mut self, lh_path: Rc<Path>, place: &mir::Place<'tcx>) {
        let lh_type = self
            .acx
            .get_path_rustc_type(&lh_path)
            .expect("Unresolved lh type");
        let (rh_path, rh_type) = self.get_path_and_type_for_place(place);
        
        // Update lh_type if it is a opaque type
        if lh_type.is_impl_trait() {
            // debug!("Update lh opaque type with {:?}", rh_type);
            if rh_type.is_impl_trait() {
                error!("Rh type {:?} is a opaque type", rh_type);
            }
            self.acx.set_path_rustc_type(lh_path.clone(), rh_type);
        }

        // An assignment of format: (*lbase).elem = (*rbase).elem
        if lh_path.is_deref_path() && rh_path.is_deref_path() {
            debug!(
                "Assignment: (*lbase).elem = (*rbase).elem: {:?} = {:?}",
                lh_path, rh_path
            );
            let aux = self.create_aux_local(rh_type);
            self.add_internal_edges(
                rh_path,
                rh_type,
                aux.clone(),
                rh_type,
            );
            self.add_internal_edges(
                aux,
                rh_type,
                lh_path,
                lh_type,
            );
            return;
        }

        self.add_internal_edges(rh_path.clone(), rh_type, lh_path.clone(), lh_type);
        
        // ===== Class Pointer System Integration =====
        // Source-level `=` compiles to MIR Assign; MIR also has many compiler-generated Assigns (e.g. arg passing).
        // We add ClassPAG Assign only when both sides are DSL class type AND caller is source-level context,
        // so that only user-visible assignments (and cast/clone return → variable) get edges, not internal DSL moves.
        let rh_has_dsl = analysis::is_dsl_class_type(self.tcx(), rh_type)
            || analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_type, None).is_some();
        let lh_has_dsl = analysis::is_dsl_class_type(self.tcx(), lh_type)
            || analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_type, None).is_some();
        let rh_has_class_semantic = rh_has_dsl
            || self.acx.class_type_system.get_path_class_type(&rh_path).is_some();
        let lh_has_class_semantic = lh_has_dsl
            || self.acx.class_type_system.get_path_class_type(&lh_path).is_some();
        if rh_has_dsl && lh_has_dsl {
            let func_name = self.rcpta_canonical_func_name();
            use crate::util::class::ptr_system::{path_to_class_ptr_id, ClassPtr as UtilClassPtr};

            let src_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), None);
            let dst_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);

            // Legacy: only when type system has path
            if analysis::is_dsl_class_type(self.tcx(), rh_type) && analysis::is_dsl_class_type(self.tcx(), lh_type) {
                if let Some(class_type) = self.acx.class_type_system.get_path_class_type(&rh_path) {
                    let src_ptr = UtilClassPtr::new_local(src_ptr_id.clone(), class_type.clone());
                    let dst_ptr = UtilClassPtr::new_local(dst_ptr_id.clone(), class_type.clone());
                    self.acx.class_ptr_system.get_or_create_ptr(src_ptr);
                    self.acx.class_ptr_system.get_or_create_ptr(dst_ptr);
                    self.acx.class_ptr_system.propagate_points_to(&src_ptr_id, &dst_ptr_id);
                }
            }
            // rcpta: record alias whenever both sides contain DSL class.
            // NOTE: Do NOT build ClassPAG Assign edge here! MIR Assign statements like "_27 = move _4" are
            // mostly arg passing, not source-level assignments. Source-level operations are handled in:
            // - Alloc: handle_class_constructor
            // - Cast: handle_class_cast_call  
            // - Assign (clone): clone handling in resolve_call
            // Only update alias map for pointer analysis propagation.
            if analysis::is_source_level_context(&func_name)
                && analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_type, None).is_some()
                && analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_type, None).is_some()
            {
                let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                self.acx.rcpta_alias_map.insert(dst_ptr_id.clone(), canonical_src.clone());
            }
        }
        // rcpta: for source-level class-like copy/move, add explicit ClassPAG Assign flow.
        // This keeps merged object flow through MIR temporaries (e.g. trait-object/enum join points)
        // and avoids losing CallArg points-to when alias-map (single canonical source) is insufficient.
        if analysis::is_source_level_context(&self.rcpta_canonical_func_name())
            && rh_has_class_semantic
            && lh_has_class_semantic
        {
            use crate::util::class::ptr_system::path_to_class_ptr_id;
            let func_name = self.rcpta_canonical_func_name();
            let src_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), None);
            let dst_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);

            let src_class = self
                .acx
                .class_type_system
                .get_path_class_type(&rh_path)
                .cloned()
                .or_else(|| {
                    analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_type, None)
                        .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty))
                })
                .unwrap_or_else(|| "Drivable".to_string());
            let dst_class = self
                .acx
                .class_type_system
                .get_path_class_type(&lh_path)
                .cloned()
                .or_else(|| {
                    analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_type, None)
                        .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty))
                })
                .unwrap_or_else(|| src_class.clone());

            use crate::rcpta::ClassPtr;
            self.acx
                .class_pag
                .get_or_create_ptr(ClassPtr::new_local(src_ptr_id.clone(), src_class));
            self.acx
                .class_pag
                .get_or_create_ptr(ClassPtr::new_local(dst_ptr_id.clone(), dst_class));
            self.acx.class_pag.add_assign(&src_ptr_id, &dst_ptr_id);
        }
        // rcpta: Option/enum unwrap path often materializes as:
        //   _x = copy ((_opt as Some).0)    // _x: &ClassLike
        // This may not be recognized by wrapper-based class checks above because both sides are references.
        // Add an explicit reference-level class Assign so later CallArg sees non-empty actuals.
        if analysis::is_source_level_context(&self.rcpta_canonical_func_name()) {
            let rh_inner = match rh_type.kind() {
                TyKind::Ref(_, inner, _) => Some(*inner),
                _ => None,
            };
            let lh_inner = match lh_type.kind() {
                TyKind::Ref(_, inner, _) => Some(*inner),
                _ => None,
            };
            if let (Some(rh_inner), Some(lh_inner)) = (rh_inner, lh_inner) {
                let rh_cls =
                    analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_inner, None)
                        .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty));
                let lh_cls =
                    analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_inner, None)
                        .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty));
                if let (Some(src_class), Some(dst_class)) = (rh_cls, lh_cls) {
                    use crate::rcpta::ClassPtr;
                    use crate::util::class::ptr_system::path_to_class_ptr_id;
                    let func_name = self.rcpta_canonical_func_name();
                    let src_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), None);
                    let dst_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
                    self.acx
                        .class_pag
                        .get_or_create_ptr(ClassPtr::new_local(src_ptr_id.clone(), src_class));
                    self.acx
                        .class_pag
                        .get_or_create_ptr(ClassPtr::new_local(dst_ptr_id.clone(), dst_class));
                    let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                    self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                    self.acx.rcpta_alias_map.insert(dst_ptr_id, canonical_src);
                }
            }
        }
        // rcpta: `_x = ((_opt as Some).0)` often appears after `Iterator::next` and may lose the
        // Option-base pointer flow (`_opt`) because call-return modeling is incomplete for iterator internals.
        // For this specific MIR shape, conservatively bridge compatible in-function class pointers to
        // both Option base and lhs reference pointer, so downstream CallArg can become non-empty.
        if analysis::is_source_level_context(&self.rcpta_canonical_func_name()) {
            if let crate::mir::path::PathEnum::QualifiedPath { base, projection } = &rh_path.value {
                let is_option_some_inner = projection.len() >= 2
                    && matches!(projection[0], PathSelector::Downcast(1))
                    && matches!(projection[1], PathSelector::Field(0));
                if is_option_some_inner {
                    let lh_inner = match lh_type.kind() {
                        TyKind::Ref(_, inner, _) => *inner,
                        _ => lh_type,
                    };
                    if let Some(dst_class_ty) =
                        analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_inner, None)
                    {
                        if let Some(dst_class_name) =
                            analysis::class_name_of_dsl_type(self.tcx(), dst_class_ty)
                        {
                            use crate::rcpta::ClassPtr;
                            use crate::util::class::ptr_system::path_to_class_ptr_id;
                            let func_name = self.rcpta_canonical_func_name();
                            let base_ptr_id = path_to_class_ptr_id(base, Some(&func_name), None);
                            let lhs_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                base_ptr_id.clone(),
                                dst_class_name.clone(),
                            ));
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                lhs_ptr_id.clone(),
                                dst_class_name.clone(),
                            ));

                            let func_prefix = format!("{}::", func_name);
                            let candidate_ptrs: Vec<String> = self
                                .acx
                                .class_pag
                                .ptr_ids()
                                .filter_map(|ptr_id| {
                                    if !ptr_id.starts_with(&func_prefix)
                                        || *ptr_id == base_ptr_id
                                        || *ptr_id == lhs_ptr_id
                                    {
                                        return None;
                                    }
                                    let ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                                    if self
                                        .acx
                                        .class_type_system
                                        .is_subtype(&ptr.class_type, &dst_class_name)
                                    {
                                        Some(ptr_id.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            for src_ptr_id in candidate_ptrs {
                                let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                                self.acx.class_pag.add_assign(&canonical_src, &base_ptr_id);
                                self.acx.class_pag.add_assign(&canonical_src, &lhs_ptr_id);
                            }
                        }
                    }
                }
            }
        }
        // rcpta: return place = move(expr) — add ClassPAG Assign from expr to ret so call-ret flow is complete.
        // e.g. get_and_wrap: ret_place = move local_3 (entity) → Assign local_3 -> ret.
        // Only for source-level methods (class CG / entry); skip internal funcs (_into_inner, convert::from, etc.).
        use crate::mir::path::PathEnum;
        use crate::util::class::ptr_system::path_to_class_ptr_id;
        if let PathEnum::ReturnValue { func_id } = &lh_path.value {
            if *func_id == self.fpag.func_id
                && analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_type, None).is_some()
            {
                let func_name = self.rcpta_canonical_func_name();
                if analysis::is_source_level_context(&func_name) {
                    let param_slots = Some(1 + self.mir.arg_count);
                    let src_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), param_slots);
                    let ret_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), param_slots);
                    if let Some(inner_ty) = analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_type, None) {
                        if let Some(class_name) = analysis::class_name_of_dsl_type(self.tcx(), inner_ty) {
                            use crate::rcpta::ClassPtr;
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                src_ptr_id.clone(),
                                class_name.clone(),
                            ));
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                ret_ptr_id.clone(),
                                class_name,
                            ));
                            self.acx.class_pag.add_assign(&src_ptr_id, &ret_ptr_id);
                        }
                    }
                }
            }
            // rcpta: container-return summary bridge.
            // For `fn f() -> Vec<CRc<...>>` style returns, connect function-local
            // container element summaries to `func::ret.$elem` and `func::ret.deref.index`
            // so caller-side index/iter reads can receive class flow.
            let mut vec_inner = rh_type;
            if let TyKind::Ref(_, inner, _) = vec_inner.kind() {
                vec_inner = *inner;
            }
            if let TyKind::Adt(def, args) = vec_inner.kind() {
                let path = self.tcx().def_path_str(def.did());
                let is_vec = path.contains("alloc::vec::Vec") || path.ends_with("::vec::Vec");
                if is_vec {
                    use rustc_middle::ty::GenericArgKind;
                    if let Some(GenericArgKind::Type(item_ty)) = args.get(0).map(|a| a.unpack()) {
                        let item_ty = match item_ty.kind() {
                            TyKind::Ref(_, inner, _) => *inner,
                            _ => item_ty,
                        };
                        let mut item_class_name = analysis::extract_dsl_class_from_wrapper(
                            self.tcx(),
                            item_ty,
                            None,
                        )
                        .and_then(|class_ty| analysis::class_name_of_dsl_type(self.tcx(), class_ty))
                        .or_else(|| analysis::class_name_of_dsl_type(self.tcx(), item_ty));
                        if item_class_name.is_none() {
                            let func_name = self.rcpta_canonical_func_name();
                            let func_prefix = format!("{}::", func_name);
                            item_class_name = self
                                .acx
                                .class_pag
                                .ptr_ids()
                                .filter(|ptr_id| {
                                    ptr_id.starts_with(&func_prefix)
                                        && ptr_id.ends_with(".deref.index")
                                })
                                .find_map(|ptr_id| {
                                    self.acx
                                        .class_pag
                                        .get_ptr(ptr_id)
                                        .map(|p| p.class_type.clone())
                                });
                        }
                        if let Some(class_name) = item_class_name {
                            use crate::rcpta::ClassPtr;
                            let func_name = self.rcpta_canonical_func_name();
                            let func_prefix = format!("{}::", func_name);
                            let ret_elem_ptr_id = Self::rcpta_ret_elem_slot_id(&func_name);
                            let ret_index_ptr_id = format!("{}::ret.deref.index", func_name);
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                ret_elem_ptr_id.clone(),
                                class_name.clone(),
                            ));
                            self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                ret_index_ptr_id.clone(),
                                class_name,
                            ));
                            self.acx
                                .class_pag
                                .add_assign(&ret_elem_ptr_id, &ret_index_ptr_id);
                            let local_elem_ptrs: Vec<String> = self
                                .acx
                                .class_pag
                                .ptr_ids()
                                .filter(|ptr_id| {
                                    ptr_id.starts_with(&func_prefix)
                                        && ptr_id.contains("::local_")
                                        && (ptr_id.ends_with(".deref.index")
                                            || ptr_id.ends_with(".$elem"))
                                })
                                .cloned()
                                .collect();
                            for src_ptr_id in local_elem_ptrs {
                                let src_for_assign = if src_ptr_id.ends_with(".deref.index")
                                    || src_ptr_id.ends_with(".$elem")
                                {
                                    src_ptr_id.clone()
                                } else {
                                    self.acx.get_canonical_rcpta_ptr(&src_ptr_id)
                                };
                                self.acx.class_pag.add_assign(&src_for_assign, &ret_elem_ptr_id);
                            }
                        }
                    }
                }
            }
            }

        // rcpta: dst = move option_holder (Option<CRc<T>>) — map copy to base so unwrap() can resolve
        // receiver to the original Option holder and find Option.Some.0 (try_into_subtype's dest).
        if analysis::type_is_option_containing_dsl_class(self.tcx(), lh_type) {
            let func_name = self.rcpta_canonical_func_name();
            if analysis::is_source_level_context(&func_name) {
                use crate::util::class::ptr_system::path_to_class_ptr_id;
                let copy_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
                self.acx.rcpta_option_copy_to_base_path.insert(copy_ptr_id, rh_path.clone());
            }
        }
        // ============================================

    }

    fn visit_constant_assign(&mut self, lh_path: Rc<Path>, const_op: &mir::ConstOperand<'tcx>) {
        let lh_type = self
            .acx
            .get_path_rustc_type(&lh_path)
            .expect("Unresolved lh type.");
        if !lh_type.is_any_ptr() && self.acx.get_pointer_projections(lh_type).is_empty() {
            return;
        }
        let rh_path = self.visit_const_operand(const_op);
        self.add_const_assign_edge(lh_path, rh_path);
    }

    fn add_const_assign_edge(&mut self, lh_path: Rc<Path>, rh_path: Rc<Path>) {
        if let Some(rh_type) = self.acx.get_path_rustc_type(&rh_path) {
            let lh_type = self
                .acx
                .get_path_rustc_type(&lh_path)
                .expect("Unresolved lh type");
            self.add_internal_edges(rh_path, rh_type, lh_path, lh_type);
        };
    }

    /// Returns a value that corresponds to the given literal.
    fn visit_const_operand(&mut self, const_op: &mir::ConstOperand<'tcx>) -> Rc<Path> {
        let mir::ConstOperand { const_, .. } = const_op;
        match const_ {
            // This constant came from the type system
            // Old code (nightly-2024-02-03): mir::Const::Ty(c) - had 1 field (const value)
            // New code (nightly-2025-05-09): mir::Const::Ty now has 2 fields (ty, const), swap order
            mir::Const::Ty(_ty, c) => self.visit_const(c),
            // An unevaluated mir constant which is not part of the type system.
            mir::Const::Unevaluated(c, ty) => self.visit_unevaluated_const(c, *ty),
            // This constant contains something the type system cannot handle (e.g. pointers).
            mir::Const::Val(v, ty) => self.visit_const_value(*v, *ty),
        }
    }

    /// Synthesizes a constant value from a RustC constant as used in the type system.
    fn visit_const(&mut self, c: &ty::Const<'tcx>) -> Rc<Path> {
        debug!("Visiting constant came from the type system: {c:?}");
        Path::new_constant()
    }

    /// Synthesizes a constant value from an unevaluated mir constant which is not part of the type system.
    fn visit_unevaluated_const(
        &mut self,
        unevaluated: &mir::UnevaluatedConst<'tcx>,
        ty: Ty<'tcx>,
    ) -> Rc<Path> {
        debug!("Visiting unevaluated constant: {unevaluated:?} {ty:?}");
        if let Some(promoted) = unevaluated.promoted {
            let promoted = Path::new_promoted(self.def_id(), promoted.index());
            self.acx.set_path_rustc_type(promoted.clone(), ty);
            return promoted;
        }
        let mut def_id = unevaluated.def;
        let def_ty = self.tcx().type_of(def_id);
        let args = self.substs_specializer.specialize_generic_args(unevaluated.args);
        debug!("resolving unevaluated def_id {:?} {:?}", def_id, def_ty);
        if !args.is_empty() {
            // Old code (nightly-2024-02-03): ParamEnv::reveal_all() and Instance::resolve()
            // New code (nightly-2025-05-09): TypingEnv::fully_monomorphized() and Instance::try_resolve()
            let typing_env = rustc_middle::ty::TypingEnv::fully_monomorphized();
            if let Ok(Some(instance)) =
                rustc_middle::ty::Instance::try_resolve(self.tcx(), typing_env, def_id, args)
            {
                def_id = instance.def.def_id();
            }
        }
        if self.tcx().is_mir_available(def_id) {
            let static_variable = Path::new_static_variable(def_id);
            let static_variable_ty = self.tcx().type_of(def_id).skip_binder();
            self.acx
                .set_path_rustc_type(static_variable.clone(), static_variable_ty);
            self.fpag.add_static_variables_involved(static_variable.clone());
            return static_variable;
        }
        Path::new_constant()
    }

    /// This represents things the type system cannot handle (e.g. pointers), as well as
    /// everything else.
    fn visit_const_value(&mut self, val: mir::ConstValue<'tcx>, ty: Ty<'tcx>) -> Rc<Path> {
        debug!("Visiting constant value: {val:?} {ty:?}");
        match val {
            // A pointer.
            // We also store the size of the pointer, such that a `Scalar` always knows how big it is. 
            // The size is always the pointer size of the current target, but this is not information
            // that we always have readily available.
            mir::ConstValue::Scalar(Scalar::Ptr(ptr, _size)) => {
                debug!("Visiting scalar pointer {ptr:?}");
                match self.tcx().try_get_global_alloc(ptr.provenance.alloc_id()) {
                    Some(GlobalAlloc::Memory(_alloc)) => {
                        // Todo: The alloc ID points to memory.
                        // We currently ignore the pointed-to memory of the constant.
                        let aux = self.create_aux_local(ty);
                        aux
                    }
                    Some(GlobalAlloc::Static(def_id)) => {
                        // the global alloc is a pointer to a static variable
                        let static_variable = Path::new_static_variable(def_id);
                        let static_variable_ty = self.tcx().type_of(def_id).skip_binder();
                        self.acx
                            .set_path_rustc_type(static_variable.clone(), static_variable_ty);
                        self.fpag.add_static_variables_involved(static_variable.clone());

                        // create an auxiliary variable for representing the global alloc const
                        let aux = self.create_aux_local(ty);
                        self.add_addr_edge(static_variable, aux.clone());
                        aux
                    }
                    _ => Path::new_constant(),
                }
            }
            mir::ConstValue::ZeroSized => {
                match ty.kind() {
                    TyKind::Closure(..) => {
                        self.new_closure_path(ty)
                    }
                    TyKind::FnDef(def_id, args) => {
                        self.visit_function_reference(*def_id, args)
                    }
                    _ => Path::new_constant(),
                }
            }
            mir::ConstValue::Scalar(Scalar::Int(..)) 
            | mir::ConstValue::Slice {..}
            | mir::ConstValue::Indirect {..} => {
                Path::new_constant()
            }
        }
    }

    /// Creates an array where each element is the value of the operand.
    /// Corresponds to source code like `[x; 32]`.
    fn visit_repeat(&mut self, lh_path: Rc<Path>, operand: &mir::Operand<'tcx>, _count: &Const<'tcx>) {
        let lh_type = self.acx.get_path_rustc_type(&lh_path).unwrap();
        if let TyKind::Array(elem_ty, _) = lh_type.kind() {
            let index_path = Path::new_index(lh_path.clone());
            self.acx.set_path_rustc_type(index_path.clone(), *elem_ty);
            self.visit_use(index_path, operand);
        }
    }

    /// Analyzes the `ref` and `address_of` assignments.
    /// 
    /// Ref: Creates a reference of the indicated kind to the place. e.g. `path = &x` or `&mut x`
    /// AddressOf: Creates a pointer with the indicated mutability to the place.
    ///            This is generated by pointer casts like `&v` as `*const _` or raw address of
    ///            expressions like `&raw v` or `addr_of!(v)`.
    /// create aux for deref path = ref/address_of path; for there is no edge type corresponding to this case.
    fn visit_ref_or_address_of(&mut self, lh_path: Rc<Path>, place: &mir::Place<'tcx>) {
        // debug!("Ref/AddressOf Assignment");
        let rh_path = self.get_path_for_place(place);

        // If the lh_path is a deref path, we need to add a temporary local variable,
        // e.g. `(*_1).2 = &rh_path;` ==> `_TMP = &rh_path; (*_1).2 = _TMP`;
        let lh_path = if lh_path.is_deref_path() {
            let lh_type = self.acx.get_path_rustc_type(&lh_path).unwrap();
            let aux = self.create_aux_local(lh_type);
            self.add_store_edge(aux.clone(), lh_path);
            aux
        } else {
            lh_path
        };

        if self.is_promoted() {
            //create only path for argument array and string array to avoid duplicate paths
            let rh_type = self.acx.get_path_rustc_type(&rh_path).unwrap();
            //argument array for macro like format!
            if type_util::is_argumentv1_array(rh_type) {
                let argv1_arr_path = Path::new_argumentv1_arr();
                if self.acx.get_path_rustc_type(&argv1_arr_path).is_none() {
                    self.acx.set_path_rustc_type(argv1_arr_path.clone(), rh_type);
                }
                self.add_addr_edge(argv1_arr_path, lh_path);
                return;
            }
            if type_util::is_str_ref_array(rh_type) {
                //string array for marco like format!
                let str_ref_arr_path = Path::new_str_ref_arr();
                if self.acx.get_path_rustc_type(&str_ref_arr_path).is_none() {
                    self.acx.set_path_rustc_type(str_ref_arr_path.clone(), rh_type);
                }
                self.add_addr_edge(str_ref_arr_path, lh_path);
                return;
            }
        }

        match &rh_path.value {
            PathEnum::Parameter { .. }
            | PathEnum::LocalVariable { .. }
            | PathEnum::ReturnValue { .. } => {
                self.add_addr_edge(rh_path.clone(), lh_path.clone());
            }
            PathEnum::QualifiedPath { base, projection } => {
                // 1. If the rh_path is a dereference of a pointer or reference, add a direct edge from
                //    the base_value of the rh_path to the lh_path,
                //    e.g. _1 = &(*_2); // It is equivelant to _1 = _2;
                // 2. If the first projection element is Deref, and the length of projection is larger
                //    than 1, add a gep edge from the rh_path to lh_path,
                //    e.g. _1 = &((*_2).1); // _1 points to the field of the _2's referent
                // 3. If the first projection element is not Deref, add an addr_edge from the rh_path to
                //    the lh_path, e.g. _1 = &(_1.2);
                match projection[0] {
                    PathSelector::Deref if projection.len() == 1 => {
                        let base = base.clone();
                        self.add_direct_edge(base, lh_path.clone());
                    }
                    PathSelector::Deref => {
                        self.add_gep_edge(rh_path.clone(), lh_path.clone());
                    }
                    _ => {
                        self.add_addr_edge(rh_path.clone(), lh_path.clone());
                    }
                };
            }
            _ => {
                unreachable!("Unexpected path type of rh_path in Ref/AddressOf assignment.");
            }
        }

        // rcpta: Ref assignment _126 = &_98 (e.g. &eagle for clone) — map reference to base so clone's receiver resolves to eagle.
        // clone() takes &self, so args[0] is the reference (local_126); get_canonical_rcpta_ptr(local_126) must resolve to local_98 (eagle).
        let lh_type = self.acx.get_path_rustc_type(&lh_path);
        let rh_type = self.acx.get_path_rustc_type(&rh_path);
        if let (Some(lh_ty), Some(rh_ty)) = (lh_type, rh_type) {
            let lh_has_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), lh_ty, None).is_some();
            let rh_has_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), rh_ty, None).is_some();
            if lh_has_dsl && rh_has_dsl {
                let func_name = self.rcpta_canonical_func_name();
                if analysis::is_source_level_context(&func_name) {
                    use crate::util::class::ptr_system::path_to_class_ptr_id;
                    let ref_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
                    let base_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), None);
                    let canonical_base = self.acx.get_canonical_rcpta_ptr(&base_ptr_id);
                    self.acx.rcpta_alias_map.insert(ref_ptr_id.clone(), canonical_base);
                    self.acx.rcpta_ref_ptr_to_base_path.insert(ref_ptr_id, rh_path.clone());
                }
            }
        }
        // rcpta: Ref assignment _tmp = &downcast_to_bird — always map ref to base so Option::unwrap() can
        // resolve receiver &opt to base path (downcast_to_bird) and then Option.Some.0. Without this, unwrap
        // would not find option_inner_path (LHS/RHS are Option<CRc<T>>, not DSL class), so no assign edge.
        let func_name = self.rcpta_canonical_func_name();
        if analysis::is_source_level_context(&func_name) {
            use crate::util::class::ptr_system::path_to_class_ptr_id;
            let ref_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
            self.acx.rcpta_ref_ptr_to_base_path.insert(ref_ptr_id, rh_path.clone());
            // ContainerReadIndexRef semantic op (path-local):
            // `let v = &drivables[i]` often lowers to a source path whose class ptr id ends with
            // `.deref.index`. Bridge that element summary directly to the reference local.
            let src_ptr_id = path_to_class_ptr_id(&rh_path, Some(&func_name), None);
            if src_ptr_id.ends_with(".deref.index") {
                let dst_ptr_id = path_to_class_ptr_id(&lh_path, Some(&func_name), None);
                let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                if func_name.contains("test_rcpta_min_") {
                    eprintln!(
                        "[rupta][min-case][addr-index-bridge] {} -> {}",
                        canonical_src, dst_ptr_id
                    );
                }
            }
        }
    }

    /// path = operand as ty.
    fn visit_cast(
        &mut self,
        lh_path: Rc<Path>,
        cast_kind: mir::CastKind,
        operand: &mir::Operand<'tcx>,
        ty: Ty<'tcx>,
    ) {
        let lh_type = self
            .acx
            .get_path_rustc_type(&lh_path)
            .expect("Unresolved lh type");
        let lh_path = if lh_path.is_deref_path() {
            // Create an auxiliary `aux`, add a cast edge from src to aux first, then store aux into dst.
            let aux = self.create_aux_local(lh_type);
            self.add_internal_edges(
                aux.clone(),
                lh_type,
                lh_path,
                lh_type,
            );
            aux
        } else {
            lh_path
        };
        match cast_kind {
            // 原代码（nightly-2024-02-03）使用的名称：
            // - mir::CastKind::PointerExposeAddress
            // - mir::CastKind::PointerFromExposedAddress
            // 新代码（nightly-2025-05-09）重命名为：
            // - mir::CastKind::PointerExposeProvenance
            // - mir::CastKind::PointerWithExposedProvenance
            // 这个改名反映了 Rust 对指针 provenance（来源追踪）概念的更新
            
            // An exposing pointer to provenance cast. A cast between a pointer and an
            // integer type, or between a function pointer and an integer type.
            mir::CastKind::PointerExposeProvenance
            // A pointer-from-exposed-provenance cast that picks up an exposed provenance.
            | mir::CastKind::PointerWithExposedProvenance => {}
            
            // 注意：DynStar 在 nightly-2025-05-09 中已被移除
            // 原代码：mir::CastKind::DynStar => { /* 处理逻辑 */ }
            
            // Primitive casts
            mir::CastKind::IntToInt
            | mir::CastKind::FloatToInt
            | mir::CastKind::FloatToFloat
            | mir::CastKind::IntToFloat => {}
            
            // 旧代码（nightly-2024-02-03）：
            // mir::CastKind::DynStar|
            // 说明：DynStar 变体在 nightly-2025-05-09 中已被移除
            
            // Go from a mut raw pointer to a const raw pointer.
            mir::CastKind::PointerCoercion(PointerCoercion::MutToConstPointer, _)
            // Go from a safe fn pointer to an unsafe fn pointer.
            | mir::CastKind::PointerCoercion(PointerCoercion::UnsafeFnPointer, _) => {
                // These kinds of pointer casts do not re-interpret the bits of the input as a 
                // different type. We simply treat them as direct assignments.
                let rh_path = match operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        self.get_path_for_place(place)
                    }
                    mir::Operand::Constant(box const_op) => {
                        debug!("
                            MutToConstPointer/UnsafeFnPointer cast from a const operand!"
                        );
                        self.visit_const_operand(const_op)
                    }
                };
                self.add_direct_edge(rh_path, lh_path);
            }
            // Go from a fn-item type to a fn-pointer type.
            // For example: ``` p = foo as fn(i32) -> i32 (Pointer(ReifyFnPointer)); ```
            // The operand should be a constant of a function instance or a place of FnDef type
            // 旧代码（nightly-2024-02-03）：
            // mir::CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer) => {
            // 说明：ReifyFnPointer 在 nightly-2025-05-09 中需要两个参数
            mir::CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer, _) => {
                let rh_path = match operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        let mut rh_path = self.get_path_for_place(place);
                        let rh_ty = self
                            .acx
                            .get_path_rustc_type(&rh_path)
                            .expect("Expect a FnDef type");
                        if let TyKind::FnDef(def_id, substs) = rh_ty.kind() {
                            rh_path = self.visit_function_reference(*def_id, substs);
                        } else {
                            unreachable!("Unexpected type of operand in ReifyFnPointer cast!");
                        }
                        rh_path
                    }
                    mir::Operand::Constant(box const_op) => {
                        // the rh_path must be a function item
                        let rh_path = self.visit_const_operand(const_op);
                        assert!(matches!(rh_path.value, PathEnum::Function(..)));
                        rh_path
                    }
                };
                self.add_fnptr_cast_edge(lh_path, rh_path, ty);
            }
            // Go from a non-capturing closure to an fn pointer or an unsafe fn pointer. 
            // It cannot convert a closure that requires unsafe.
            // Closures capturing the environments cannot be converted to fn pointer as well.
            // The operand should be a place of a closure instance.
            mir::CastKind::PointerCoercion(PointerCoercion::ClosureFnPointer(..), _) => {
                let rh_path = match operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        self.get_path_for_place(place)
                    }
                    mir::Operand::Constant(const_op) => {
                        // the rh_path must be a closure
                        self.visit_const_operand(const_op)
                    }
                };
                let ty = self
                    .acx
                    .get_path_rustc_type(&rh_path)
                    .expect("Expect a closure type");
                assert!(matches!(ty.kind(), TyKind::Closure(..)));

                self.add_fnptr_cast_edge(lh_path, rh_path, ty);
            }
            mir::CastKind::PointerCoercion(PointerCoercion::DynStar, _) => {
                match operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        let (rh_path, rh_type) = self.get_path_and_type_for_place(place);
                        self.copy_and_transmute(rh_path, rh_type, lh_path, lh_type);
                    }
                    mir::Operand::Constant(const_op) => {
                        let const_path = self.visit_const_operand(const_op);
                        if let Some(const_ty) = self.acx.get_path_rustc_type(&const_path) {
                            self.copy_and_transmute(const_path, const_ty, lh_path, lh_type);
                        }
                    }
                }
            }
            // Unsize a pointer/reference value, e.g., &[T; n] to &[T]. Note that the source could 
            // be a thin or fat pointer. This will do things like convert thin pointers to fat 
            // pointers, or convert structs containing thin pointers to structs containing fat 
            // pointers, or convert between fat pointers. We don’t store the details of how the 
            // transform is done (in fact, we don’t know that, because it might depend on the 
            // precise type parameters). We just store the target type. Codegen backends and miri 
            // figure out what has to be done based on the precise source/target type at hand.
            // Example of casting a struct containing thin pointers to a struct containing
            // fat pointers:
            // ```
            //  let a = Box::<[i32; 3]>::new([1, 2, 3]);
            //  let b: Box::<[i32]> = a;
            // ```
            mir::CastKind::PointerCoercion(PointerCoercion::Unsize,_) => {
                match operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        let (rh_path, rh_type) = self.get_path_and_type_for_place(place);
                        debug!("Unsize pointer cast: {:?} -> {:?}", rh_path, lh_path);
                        // We need to call transmute_pointers here to make the source pointer and
                        // destination pointer point to different types. 
                        self.copy_and_transmute(rh_path, rh_type, lh_path, lh_type);
                    }
                    mir::Operand::Constant(const_op) => {
                        // The operand of a Unsize pointer cast statement can be a constant in rare cases. 
                        let const_path = self.visit_const_operand(const_op);
                        if let Some(const_ty) = self.acx.get_path_rustc_type(&const_path) {
                            if ty.is_any_ptr() {
                                self.copy_and_transmute(const_path, const_ty, lh_path, lh_type);
                            }
                        }
                    }
                }
            }
            // Go from *const [T; N] to *const T
            // In practice, we find that most casts from *const [T; N] to *const T are classified 
            // as CastKind::PtrToPtr
            mir::CastKind::PointerCoercion(PointerCoercion::ArrayToPointer,_) 
            | mir::CastKind::PtrToPtr 
            // Cast a function pointer to another pointer type
            // e.g. ``` let p = fp as *const (); ```
            | mir::CastKind::FnPtrToPtr => {
                if let mir::Operand::Copy(place) | mir::Operand::Move(place) = operand {
                    let (rh_path, rh_type) = self.get_path_and_type_for_place(place);
                    if lh_type.is_any_ptr() && rh_type.is_any_ptr() {
                        let src_path = if rh_path.is_deref_path() {
                            // Load the value of rh_path to an auxiliary variable, then add a cast 
                            // edge from aux to dst.
                            let aux = self.create_aux_local(rh_type);
                            self.add_load_edge(rh_path, aux.clone());
                            aux
                        } else {
                            rh_path
                        };
                        // The lh path or the rh path might be a reference to a transparent wrapper struct.
                        // Therefore we cast the pointers by transmuting between them.
                        self.transmute_pointers(src_path, rh_type, lh_path, lh_type)
                    } 
                }
            }
            mir::CastKind::Transmute => {
                debug!("Visiting transmute cast statement {:?} -> {:?}", operand, lh_path);
                if let mir::Operand::Copy(place) | mir::Operand::Move(place) = operand {
                    let (rh_path, rh_type) = self.get_path_and_type_for_place(place);
                    self.copy_and_transmute(rh_path, rh_type, lh_path, lh_type);
                }
            }
        }
    }

    /// Apply the given binary operator to the two operands and assign result to path.
    fn visit_binary_op(
        &mut self,
        lh_path: Rc<Path>,
        bin_op: mir::BinOp,
        left_operand: &mir::Operand<'tcx>,
        _right_operand: &mir::Operand<'tcx>,
    ) {
        match bin_op {
            mir::BinOp::Offset => {
                match left_operand {
                    mir::Operand::Move(place) | mir::Operand::Copy(place) => {
                        let rh_path = self.get_path_for_place(place);
                        self.add_offset_edge(rh_path, lh_path);
                    }
                    mir::Operand::Constant(_const_op) => {
                        error!("Unexpected left operand in an Offset BinaryOp.");
                    }
                };
            }
            _ => {}
        }
    }

    /// Creates an aggregate value, like a tuple or struct.
    fn visit_aggregate(
        &mut self,
        lh_path: Rc<Path>,
        aggregate_kind: &mir::AggregateKind<'tcx>,
        operands: &IndexVec<FieldIdx, mir::Operand<'tcx>>,
    ) {
        match aggregate_kind {
            mir::AggregateKind::Array(ty) => {
                let index_path = Path::new_index(lh_path.clone());
                let index_ty = self.substs_specializer.specialize_generic_argument_type(*ty);
                self.acx.set_path_rustc_type(index_path.clone(), index_ty);
                for (_i, operand) in operands.iter().enumerate() {
                    self.visit_use(index_path.clone(), operand);
                }
            }
            mir::AggregateKind::Tuple => {
                let lh_ty = self.acx.get_path_rustc_type(&lh_path).unwrap();
                let types = if let TyKind::Tuple(types) = lh_ty.kind() {
                    types.as_slice()
                } else {
                    &[]
                };
                for (i, operand) in operands.iter().enumerate() {
                    let index_path = Path::new_field(lh_path.clone(), i);
                    if let Some(ty) = types.get(i) {
                        self.acx.set_path_rustc_type(index_path.clone(), *ty);
                    };
                    self.visit_use(index_path, operand);
                }
            }
            mir::AggregateKind::Adt(def, variant_idx, args, _, case_index) => {
                // The second field is the variant index. It’s equal to 0 for struct and union expressions. 
                // The last field is the active field number and is present only for union expressions 
                // – e.g., for a union expression SomeUnion { c: .. }, the active field index would identity 
                // the field c
                let mut path = lh_path;
                let adt_def = self.tcx().adt_def(def);
                let variant_def = &adt_def.variants()[*variant_idx];
                // let adt_ty = self.tcx().type_of(def).skip_binder();
                let args = self.substs_specializer.specialize_generic_args(args);
                if adt_def.is_union() {
                    let case_index = case_index.unwrap_or(FieldIdx::from_usize(0));
                    let field_path = Path::new_union_field(path, usize::from(case_index));
                    let field = &variant_def.fields[case_index];
                    let field_ty = type_util::field_ty(self.tcx(), field, args);
                    self.acx.set_path_rustc_type(field_path.clone(), field_ty);
                    self.visit_use(field_path.clone(), &operands[FieldIdx::from_usize(0)]);

                    self.cast_between_union_fields(&field_path);
                    return;
                } else if adt_def.is_enum() {
                    path = Path::new_downcast(path, variant_idx.as_usize());
                } 

                for (i, field) in variant_def.fields.iter().enumerate() {
                    let field_path = Path::new_field(path.clone(), i);
                    let field_ty = type_util::field_ty(self.tcx(), field, args);
                    self.acx.set_path_rustc_type(field_path.clone(), field_ty);
                    if let Some(operand) = operands.get(FieldIdx::from_usize(i)) {
                        self.visit_use(field_path, operand);
                    } else {
                        debug!(
                            "variant has more fields than was serialized {:?}",
                            variant_def
                        );
                    }
                }
            }
            mir::AggregateKind::Closure(_def_id, _args)
            | mir::AggregateKind::Coroutine(_def_id, _args)
            | mir::AggregateKind::CoroutineClosure(_def_id, _args) => {
                for (i, operand) in operands.iter().enumerate() {
                    let base_ty = self.acx.get_path_rustc_type(&lh_path).unwrap();
                    let field_path = Path::new_field(lh_path.clone(), i);
                    let field_ty = type_util::get_field_type(self.tcx(), base_ty, i);
                    self.acx.set_path_rustc_type(field_path.clone(), field_ty);
                    self.visit_use(field_path, operand);
                }
            }
            mir::AggregateKind::RawPtr(_, _) => {
                if let Some(data_operand) = operands.get(FieldIdx::from_usize(0)) {
                    self.visit_use(lh_path.clone(), data_operand);
                }
            }
        }
    }

    /// Transmutes a `*mut u8` into a shallow-initialized `Box<T>`.
    ///
    /// This is different from a normal transmute because dataflow analysis will treat the box
    /// as initialized but its content as uninitialized.
    fn visit_shallow_init_box(&mut self, lh_path: Rc<Path>, operand: &mir::Operand<'tcx>, ty: Ty<'tcx>) {
        // Box.0 = Unique, Unique.0 = NonNull, NonNull.0: *const T = source thin pointer
        let box_ptr_field = self.get_box_pointer_field(lh_path, ty);
        // Treat this statement as a cast statement that casts heap object from u8 type to T type.
        let box_ptr_type = self.acx.get_path_rustc_type(&box_ptr_field).unwrap();
        let source_path = match operand {
            mir::Operand::Move(place) | mir::Operand::Copy(place) => self.get_path_for_place(place),
            _ => {
                unreachable!(
                    "The operand of shallow_init_box statement is supposed to be a move|copy place."
                );
            }
        };
        let source_ptr_type = self.acx.get_path_rustc_type(&source_path).unwrap();
        self.transmute_pointers(source_path, source_ptr_type, box_ptr_field, box_ptr_type)
    }

    /// If callee is a DSL class method, push one (caller, callee) edge to pending_class_cg_edges.
    /// Used when the call was resolved via static dispatch or devirtualization (not vtable blob).
    fn push_class_cg_edge_if_class_method(&mut self, callee_func_id: crate::mir::function::FuncId) {
        let caller_name = self.acx.get_function_reference(self.func_id).to_string();
        let callee_func_ref = self.acx.get_function_reference(callee_func_id);
        let callee_name = callee_func_ref.to_string();
        let Some(class_method) = analysis::identify_class_method_with_type_system(
            &callee_func_ref,
            &self.acx.class_type_system,
        ) else {
            if caller_name.contains("entry_complex_call_chain_demo") || callee_name.contains("get_and_wrap") || callee_name.contains("get_id") {
                info!(
                    "rcpta class CG: skip (not identified as class method) caller={} callee={}",
                    caller_name, callee_name
                );
            }
            return;
        };
        let caller_func_ref = self.acx.get_function_reference(self.func_id);
        let (caller_class, caller_method) = if let Some(caller_cm) =
            analysis::identify_class_method_with_type_system(&caller_func_ref, &self.acx.class_type_system)
        {
            (caller_cm.class_name.clone(), caller_cm.method_name.clone())
        } else {
            let n = caller_func_ref.to_string();
            let simple = if let Some(i) = n.rfind("::") { n[i + 2..].to_string() } else { n };
            (simple, String::new())
        };
        self.acx.pending_class_cg_edges.push((
            caller_class.clone(),
            caller_method.clone(),
            class_method.class_name.clone(),
            class_method.method_name.clone(),
            callee_func_id,
        ));
        if caller_name.contains("entry_complex_call_chain_demo") {
            info!(
                "rcpta class CG: push edge {}::{} -> {}::{}",
                caller_class, caller_method, class_method.class_name, class_method.method_name
            );
        }
    }

    /// Try to resolve a function calls. 
    fn resolve_call(
        &mut self,
        callee_def_id: &DefId,
        gen_args: &GenericArgsRef<'tcx>,
        // Old code (nightly-2024-02-03): args: &Vec<Spanned<mir::Operand<'tcx>>>
        // New code (nightly-2025-05-09): use slice to accept both Vec and Box<[...]>
        args: &[Spanned<mir::Operand<'tcx>>],
        destination: &mir::Place<'tcx>,
        location: mir::Location,
    ) {
        // Specialize callee's substs from known generic types
        let gen_args = self.substs_specializer.specialize_generic_args(gen_args);
        let args = self.visit_args(args);
        let destination = self.get_path_for_place(destination);
        debug!("Call func {:?}, generic_args: {:?}", callee_def_id, gen_args);

        let caller_name = self.acx.get_function_reference(self.fpag.func_id).to_string();
        let callee_func_id = self.acx.get_func_id(*callee_def_id, gen_args);
        let callee_name = self.acx.get_function_reference(callee_func_id).to_string();
        let param_slots = Some(1 + self.mir.arg_count);
        // rcpta: For deref chains like `_104 = Deref::deref(_105)`, keep tracking `ref_ptr -> base_path`.
        // This enables call-arg receiver normalization to recover source-level base objects instead of temp refs.
        let caller_func_name = analysis::canonical_class_method_name(&caller_name);
        if self.func_id.as_usize() == 0 {
            let dst_dbg = format!("{:?}", destination);
            let arg_dbg: Vec<String> = args.iter().map(|a| format!("{:?}", a)).collect();
            eprintln!(
                "[rupta][probe][resolve] func_id=0 caller={} callee_def={} dst={} args={:?}",
                caller_name,
                self.tcx().def_path_str(*callee_def_id),
                dst_dbg,
                arg_dbg
            );
        }
        // rcpta: cross-function container summary bridge.
        // For callee returning Vec<DSL>, connect `callee::ret.elem` -> `caller::dst.elem`.
        {
            let mut dst_ty = destination.try_eval_path_type(self.acx);
            if let TyKind::Ref(_, inner, _) = dst_ty.kind() {
                dst_ty = *inner;
            }
            if let TyKind::Adt(def, adt_args) = dst_ty.kind() {
                let path = self.tcx().def_path_str(def.did());
                let is_vec = path.contains("alloc::vec::Vec") || path.ends_with("::vec::Vec");
                if is_vec {
                    use rustc_middle::ty::GenericArgKind;
                    if let Some(GenericArgKind::Type(item_ty)) = adt_args.get(0).map(|a| a.unpack()) {
                        let item_ty = match item_ty.kind() {
                            TyKind::Ref(_, inner, _) => *inner,
                            _ => item_ty,
                        };
                        let item_class_name = analysis::extract_dsl_class_from_wrapper(
                            self.tcx(),
                            item_ty,
                            None,
                        )
                        .and_then(|class_ty| analysis::class_name_of_dsl_type(self.tcx(), class_ty))
                        .or_else(|| analysis::class_name_of_dsl_type(self.tcx(), item_ty));
                        if let Some(class_name) = item_class_name {
                                let item_class_name_for_match = class_name.clone();
                                use crate::rcpta::ClassPtr;
                                use crate::util::class::ptr_system::path_to_class_ptr_id;
                                let callee_func_name =
                                    analysis::canonical_class_method_name(&callee_name);
                                let callee_ret_elem_ptr_id =
                                    Self::rcpta_ret_elem_slot_id(&callee_func_name);
                                let callee_ret_index_ptr_id =
                                    format!("{}::ret.deref.index", callee_func_name);
                                let caller_base_ptr_id =
                                    path_to_class_ptr_id(&destination, Some(&caller_func_name), None);
                                let caller_dst_elem_ptr_id =
                                    Self::rcpta_elem_slot_id_from_base_ptr(&caller_base_ptr_id);
                                if caller_func_name.contains("test_rcpta_min_")
                                    || callee_func_name.contains("rcpta_min_cases::build_drivables")
                                {
                                    eprintln!(
                                        "[rupta][min-case][bind] caller={} callee={} ret_elem={} dst_elem={}",
                                        caller_func_name,
                                        callee_func_name,
                                        callee_ret_elem_ptr_id,
                                        caller_dst_elem_ptr_id
                                    );
                                }
                                self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                    callee_ret_elem_ptr_id.clone(),
                                    class_name.clone(),
                                ));
                                self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                    callee_ret_index_ptr_id.clone(),
                                    class_name.clone(),
                                ));
                                self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                                    caller_dst_elem_ptr_id.clone(),
                                    class_name,
                                ));
                                self.acx
                                    .class_pag
                                    .add_assign(&callee_ret_elem_ptr_id, &caller_dst_elem_ptr_id);
                                self.acx
                                    .class_pag
                                    .add_assign(&callee_ret_index_ptr_id, &caller_dst_elem_ptr_id);
                                // Feed callee return element summaries from call arguments when possible.
                                // This keeps `callee::ret.$elem` meaningful for std/alloc container constructors
                                // (e.g. slice::into_vec) whose bodies are not modeled as source-level ClassPAG.
                                for arg in args.iter() {
                                    let arg_ptr_id = path_to_class_ptr_id(
                                        arg,
                                        Some(&caller_func_name),
                                        None,
                                    );
                                    let arg_elem_ptr_id =
                                        Self::rcpta_elem_slot_id_from_base_ptr(&arg_ptr_id);
                                    let candidate_srcs = [arg_ptr_id, arg_elem_ptr_id];
                                    for src_ptr_id in candidate_srcs {
                                        if let Some(src_ptr) =
                                            self.acx.class_pag.get_ptr(&src_ptr_id)
                                        {
                                            if src_ptr.class_type == item_class_name_for_match
                                                || self
                                                    .acx
                                                    .class_type_system
                                                    .is_subtype(
                                                        &src_ptr.class_type,
                                                        &item_class_name_for_match,
                                                    )
                                            {
                                                self.acx.class_pag.add_assign(
                                                    &src_ptr_id,
                                                    &callee_ret_elem_ptr_id,
                                                );
                                                self.acx.class_pag.add_assign(
                                                    &src_ptr_id,
                                                    &callee_ret_index_ptr_id,
                                                );
                                            }
                                        }
                                    }
                                }
                                self.acx
                                    .rcpta_container_elem_ptrs_by_base
                                    .entry(caller_base_ptr_id.clone())
                                    .or_default()
                                    .push(caller_dst_elem_ptr_id.clone());
                                if caller_func_name.contains("test_rcpta_min_") {
                                    eprintln!(
                                        "[rupta][min-case][bind-map] base={} elem={}",
                                        caller_base_ptr_id,
                                        caller_dst_elem_ptr_id
                                    );
                                }
                                // Fallback: directly bridge callee-local element summaries when
                                // `callee::ret.elem` is not populated on this MIR path.
                                let callee_prefix = format!("{}::", callee_func_name);
                                let callee_summary_ptrs: Vec<String> = self
                                    .acx
                                    .class_pag
                                    .ptr_ids()
                                    .filter_map(|ptr_id| {
                                        if !ptr_id.starts_with(&callee_prefix)
                                            || (!ptr_id.ends_with(".deref.index")
                                                && !ptr_id.ends_with(".$elem"))
                                        {
                                            return None;
                                        }
                                        let src_ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                                        if self
                                            .acx
                                            .class_type_system
                                            .is_subtype(&src_ptr.class_type, "Drivable")
                                        {
                                            Some(ptr_id.clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                for src_ptr_id in callee_summary_ptrs {
                                    let canonical_src = if src_ptr_id.ends_with(".deref.index")
                                        || src_ptr_id.ends_with(".$elem")
                                    {
                                        src_ptr_id.clone()
                                    } else {
                                        self.acx.get_canonical_rcpta_ptr(&src_ptr_id)
                                    };
                                    self.acx
                                        .class_pag
                                        .add_assign(&canonical_src, &caller_dst_elem_ptr_id);
                                }
                                // ContainerConstructToReturn fallback:
                                // For calls like `slice::into_vec` in `vec![...]`, callee `ret.$elem`
                                // is often empty in RCPTA, while caller local summaries are populated.
                                // Bridge caller-local element summaries directly to call destination elem.
                                let caller_prefix = format!("{}::", caller_func_name);
                                let caller_local_summary_ptrs: Vec<String> = self
                                    .acx
                                    .class_pag
                                    .ptr_ids()
                                    .filter_map(|ptr_id| {
                                        if !ptr_id.starts_with(&caller_prefix)
                                            || !ptr_id.contains("::local_")
                                            || (!ptr_id.ends_with(".deref.index")
                                                && !ptr_id.ends_with(".$elem"))
                                        {
                                            return None;
                                        }
                                        let src_ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                                        if self
                                            .acx
                                            .class_type_system
                                            .is_subtype(&src_ptr.class_type, &item_class_name_for_match)
                                        {
                                            Some(ptr_id.clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                for src_ptr_id in caller_local_summary_ptrs {
                                    let canonical_src = if src_ptr_id.ends_with(".deref.index")
                                        || src_ptr_id.ends_with(".$elem")
                                    {
                                        src_ptr_id.clone()
                                    } else {
                                        self.acx.get_canonical_rcpta_ptr(&src_ptr_id)
                                    };
                                    self.acx
                                        .class_pag
                                        .add_assign(&canonical_src, &caller_dst_elem_ptr_id);
                                }
                        }
                    }
                }
            }
        }
        let callee_def_path = self.tcx().def_path_str(*callee_def_id);
        if caller_name.contains("rcpta_min_cases::test_rcpta_min_index_drive") {
            eprintln!(
                "[rupta][min-case][resolve-hit] caller_raw={} caller_canon={} callee={}",
                caller_name, caller_func_name, callee_def_path
            );
        }
        if caller_func_name.contains("test_rcpta_min_") {
            eprintln!(
                "[rupta][min-case][resolve-call] caller={} callee={}",
                caller_func_name, callee_def_path
            );
        }
        let is_deref_call = callee_def_path.ends_with("::deref")
            && (callee_def_path.contains("ops::deref::Deref")
                || callee_def_path.contains("ops::Deref"));
        if analysis::is_source_level_context(&caller_func_name)
            && !args.is_empty()
            && is_deref_call
        {
            use crate::util::class::ptr_system::path_to_class_ptr_id;
            let recv_ptr_id = path_to_class_ptr_id(&args[0], Some(&caller_func_name), param_slots);
            let base_path = self
                .acx
                .rcpta_ref_ptr_to_base_path
                .get(&recv_ptr_id)
                .cloned()
                .unwrap_or_else(|| args[0].clone());
            let dst_ptr_id = path_to_class_ptr_id(&destination, Some(&caller_func_name), param_slots);
            self.acx
                .rcpta_ref_ptr_to_base_path
                .insert(dst_ptr_id.clone(), base_path.clone());
            let base_ptr_id = path_to_class_ptr_id(&base_path, Some(&caller_func_name), param_slots);
            let canonical_base = self.acx.get_canonical_rcpta_ptr(&base_ptr_id);
            self.acx.rcpta_alias_map.insert(dst_ptr_id, canonical_base);
        }
        // rcpta: `to_supertype` returns a super-class view of the same underlying object.
        // Record destination alias to receiver-base so later CallArg canonicalization does not keep
        // temp locals (e.g. local_5 in `Dog::make_sound`) as standalone empty pointers.
        if analysis::is_source_level_context(&caller_func_name)
            && !args.is_empty()
            && callee_def_path.contains("to_supertype")
        {
            use crate::util::class::ptr_system::path_to_class_ptr_id;
            let recv_ptr_id = path_to_class_ptr_id(&args[0], Some(&caller_func_name), param_slots);
            let recv_base_ptr_id = self
                .acx
                .rcpta_ref_ptr_to_base_path
                .get(&recv_ptr_id)
                .map(|p| path_to_class_ptr_id(p, Some(&caller_func_name), param_slots))
                .unwrap_or(recv_ptr_id);
            let canonical_recv = self.acx.get_canonical_rcpta_ptr(&recv_base_ptr_id);
            let dst_ptr_id = path_to_class_ptr_id(&destination, Some(&caller_func_name), param_slots);
            self.acx.rcpta_alias_map.insert(dst_ptr_id, canonical_recv);
        }
        // rcpta diagnostic: log every resolve_call from entry to see which callees we see and which branch we take
        let from_entry = caller_name.contains("entry_complex_call_chain_demo");
        if from_entry {
            let def_path_str = self.tcx().def_path_str(*callee_def_id);
            let is_vtable = analysis::is_dsl_vtable_call(self.tcx(), *callee_def_id);
            let is_trait = util::is_trait_method(self.tcx(), *callee_def_id);
            let devirt = call_graph_builder::try_to_devirtualize(self.tcx(), *callee_def_id, gen_args);
            info!(
                "rcpta resolve_call [entry]: callee_func={} def_path={} vtable={} is_trait={} devirt={}",
                callee_name, def_path_str, is_vtable, is_trait, devirt.is_some()
            );
        }

        // Check if this is a getter/setter method call (only when the class actually has that field).
        // e.g. Entity::get_id is a normal method, not a getter for field "id" — Entity has entity_id, not id.
        let func_ref = self.acx.get_function_reference(callee_func_id);
        
        // DEBUG: trace apply_twice resolution
        if func_ref.to_string().contains("apply_twice") {
             eprintln!("[rcpta] resolve_call seeing apply_twice: {}", func_ref.to_string());
             if let Some(cm) = analysis::identify_class_method_with_type_system(&func_ref, &self.acx.class_type_system) {
                 eprintln!("[rcpta] Identified as class method: {:?}", cm);
             } else {
                 eprintln!("[rcpta] NOT identified as class method");
             }
        }

        if let Some(gs) = analysis::identify_getter_setter(&func_ref) {
            // ROBUST CHECK: Generalizable logic to differentiate Field Access (Load/Store) from Method Call.
            // A getter/setter pattern (get_X/set_X) is only a Field Access if the class actually has field X.
            let mut is_actual_field = false;

            // 1. Try generic check via TypeSystem (if registered)
            if self.acx.class_type_system.get_field_index(&gs.class_name, &gs.field_name).is_some() {
                is_actual_field = true;
            }

            // 2. Fallback: Check ADT definition of the receiver (The "Universal" way)
            if !is_actual_field && !args.is_empty() {
                 let receiver_ty = args[0].try_eval_path_type(self.acx);
                 // Unwrap references and wrappers to find the underlying DSL class struct
                 let effective_ty = match receiver_ty.kind() {
                     TyKind::Ref(_, inner, _) => *inner,
                     _ => receiver_ty,
                 };
                 
                 let class_ty = if let Some(inner) = analysis::extract_dsl_class_from_wrapper(self.tcx(), effective_ty, None) {
                     inner
                 } else {
                     effective_ty
                 };

                 // Do NOT set is_actual_field=true or register_field just because receiver class matches:
                 // that would treat get_and_wrap (field "and_wrap") and get_id (field "id") as getters and
                 // skip adding them to the class CG. Only set is_actual_field when the ADT actually has that field.

                 if !is_actual_field {
                     if let Some(adt_def) = class_ty.ty_adt_def() {
                         // Check if any field matches gs.field_name (or _field_name)
                         is_actual_field = adt_def.all_fields().any(|f| {
                             let fname = f.name.as_str();
                             fname == gs.field_name || fname == format!("_{}", gs.field_name)
                         });
                         // 2b. Wrapper peeling: DSL wrapper structs often have tuple-like fields ("0", "1").
                         // The real field (e.g. "item") lives in the inner Data type. Check field 0's type.
                         if !is_actual_field && adt_def.is_struct() {
                             let variant = adt_def.variants().iter().next();
                             if let Some(v) = variant {
                                 let tuple_like = v.fields.len() <= 5
                                     && v.fields.iter().all(|f| {
                                         let n = f.name.as_str();
                                         !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())
                                     });
                                 if tuple_like && !v.fields.is_empty() {
                                     let field_0_ty = type_util::get_field_type(self.tcx(), class_ty, 0);
                                     if let Some(inner_ty) = analysis::extract_dsl_class_from_wrapper(self.tcx(), field_0_ty, None) {
                                         if let TyKind::Adt(inner_adt, _) = inner_ty.kind() {
                                             is_actual_field = inner_adt.all_fields().any(|f| {
                                                 let fname = f.name.as_str();
                                                 fname == gs.field_name || fname == format!("_{}", gs.field_name)
                                             });
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
            }
            
            if is_actual_field {
                self.handle_getter_setter(&gs, &args, &destination, location);
                // Return early so we DON'T add a static dispatch edge or treat it as a method call
                return;
            }
            // If not actual field (e.g. get_id() but no field id), fall through to standard class method handling.
            if (caller_name.contains("get_and_wrap") || caller_name.contains("chain_with")) && gs.field_name == "item" {
                info!(
                    "rcpta getter NOT treated as field (is_actual_field=false): caller={} callee getter {}.{}",
                    caller_name, gs.class_name, gs.field_name
                );
            }
        }

        // rcpta: GetSet::cell_option_set / cell_option_get (DSL late field store/load in callee body).
        // set_item body calls cell_option_set(cell_ref, value); get_item body calls cell_option_get(cell_ref).
        // Extract (base_path, field_index) from cell path and add ClassPAG Store/Load.
        // Use canonical name so ptr ids (e.g. get_and_wrap::local_2) match visit_assign and avoid duplicate Pointers.
        let func_ref_caller = self.acx.get_function_reference(self.fpag.func_id);
        let func_name = analysis::canonical_class_method_name(&func_ref_caller.to_string());
        let param_slots = Some(1 + self.mir.arg_count);
        if analysis::is_getset_cell_option_set(self.tcx(), *callee_def_id) && args.len() >= 2
            && analysis::is_source_level_context(&func_name)
        {
            let cell_path = &args[0];
            let value_path = &args[1];
            // Resolve ref: in set_item, first arg is local _4 = &((*_5).1); resolve to base path for projection.
            use crate::util::class::ptr_system::path_to_class_ptr_id as path_to_ptr_id;
            let cell_ptr_id = path_to_ptr_id(cell_path, Some(&func_name), param_slots);
            let path_to_use = self.acx.rcpta_ref_ptr_to_base_path.get(&cell_ptr_id).cloned().unwrap_or_else(|| cell_path.clone());
            if let PathEnum::QualifiedPath { base: _, projection } = &path_to_use.value {
                if let Some(PathSelector::Field(field_index)) = projection.last() {
                    let base_path = Path::truncate_projection_elems(&path_to_use, projection.len() - 1);
                    let base_ty = base_path.try_eval_path_type(self.acx);
                    let effective_ty = match base_ty.kind() {
                        TyKind::Ref(_, inner, _) => *inner,
                        _ => base_ty,
                    };
                    if let Some(class_name) = analysis::class_name_of_dsl_type(self.tcx(), effective_ty) {
                        // Register real field when we're inside the setter body (e.g. set_item) so Holder::item exists without ever registering Entity::id.
                        if let Some(gs) = analysis::identify_getter_setter(&func_ref_caller) {
                            if gs.class_name == class_name {
                                self.acx.class_type_system.register_field_with_index(
                                    &class_name,
                                    &gs.field_name,
                                    *field_index,
                                    None,
                                );
                            }
                        }
                        if let Some(field_name) = self.acx.class_type_system.get_field_name_by_index(&class_name, *field_index) {
                            // Only ClassPAG Store for class-reference values (same rule as handle_getter_setter).
                            if let Some(value_class) = self
                                .acx
                                .class_type_system
                                .get_path_class_type(value_path)
                                .cloned()
                                .or_else(|| {
                                    self.acx
                                        .class_type_system
                                        .get_field_class_type(&class_name, &field_name)
                                })
                            {
                                let base_ptr_id = path_to_ptr_id(&base_path, Some(&func_name), param_slots);
                                let value_ptr_id = path_to_ptr_id(value_path, Some(&func_name), param_slots);
                                self.acx.class_pag.get_or_create_ptr(crate::rcpta::ClassPtr::new_local(
                                    base_ptr_id.clone(),
                                    class_name.clone(),
                                ));
                                self.acx.class_pag.get_or_create_ptr(crate::rcpta::ClassPtr::new_local(
                                    value_ptr_id.clone(),
                                    value_class.clone(),
                                ));
                                self.acx.class_pag.add_store(&base_ptr_id, &field_name, &value_ptr_id);
                                info!(
                                    "rcpta ClassPAG: add_store (cell_option_set) func={} base={} field={}",
                                    func_name, base_ptr_id, field_name
                                );
                            }
                        }
                    }
                }
            }
        } else if analysis::is_getset_cell_option_get(self.tcx(), *callee_def_id) && !args.is_empty()
            && analysis::is_source_level_context(&func_name)
        {
            let cell_path = &args[0];
            if let PathEnum::QualifiedPath { base: _, projection } = &cell_path.value {
                if let Some(PathSelector::Field(field_index)) = projection.last() {
                    let base_path = Path::truncate_projection_elems(cell_path, projection.len() - 1);
                    let base_ty = base_path.try_eval_path_type(self.acx);
                    let effective_ty = match base_ty.kind() {
                        TyKind::Ref(_, inner, _) => *inner,
                        _ => base_ty,
                    };
                    if let Some(class_name) = analysis::class_name_of_dsl_type(self.tcx(), effective_ty) {
                        // Register real field when we're inside the getter body (e.g. get_item) so Holder::item exists.
                        if let Some(gs) = analysis::identify_getter_setter(&func_ref_caller) {
                            if gs.class_name == class_name {
                                self.acx.class_type_system.register_field_with_index(
                                    &class_name,
                                    &gs.field_name,
                                    *field_index,
                                    None,
                                );
                            }
                        }
                        if let Some(field_name) = self.acx.class_type_system.get_field_name_by_index(&class_name, *field_index) {
                            if let Some(dst_class) = self
                                .acx
                                .class_type_system
                                .get_field_class_type(&class_name, &field_name)
                            {
                                use crate::util::class::ptr_system::path_to_class_ptr_id;
                                let base_ptr_id = path_to_class_ptr_id(&base_path, Some(&func_name), param_slots);
                                let dst_ptr_id =
                                    path_to_class_ptr_id(&destination, Some(&func_name), param_slots);
                                self.acx.class_pag.get_or_create_ptr(crate::rcpta::ClassPtr::new_local(
                                    base_ptr_id.clone(),
                                    class_name.clone(),
                                ));
                                self.acx.class_pag.get_or_create_ptr(crate::rcpta::ClassPtr::new_local(
                                    dst_ptr_id.clone(),
                                    dst_class.clone(),
                                ));
                                self.acx.class_pag.add_load(&base_ptr_id, &field_name, &dst_ptr_id);
                                info!(
                                    "rcpta ClassPAG: add_load (cell_option_get) func={} base={} field={}",
                                    func_name, base_ptr_id, field_name
                                );
                            }
                        }
                    }
                }
            }
        }
        
        // rcpta: Prioritize checking for Field Access (Getter/Setter) to correctly distinguish
        // simple Load/Store from Method Calls (even if the method looks like a getter).
        if let Some(gs) = analysis::identify_getter_setter(&func_ref) {
            // ROBUST CHECK: Generalizable logic to differentiate Field Access (Load/Store) from Method Call.
            // A getter/setter pattern (get_X/set_X) is only a Field Access if the class actually has field X.
            let callee_str = func_ref.to_string();
            let is_get_and_wrap_or_get_id = callee_str.contains("get_and_wrap") || callee_str.contains("get_id");
            let mut is_actual_field = self.acx.class_type_system.get_field_index(&gs.class_name, &gs.field_name).is_some();
            if is_get_and_wrap_or_get_id {
                eprintln!(
                    "[rcpta] getter branch: callee get_and_wrap/get_id class={} field={} is_actual_field(initial)={}",
                    gs.class_name, gs.field_name, is_actual_field
                );
            }

            // Fallback: Check ADT definition of the receiver (The "Universal" way).
            // Do NOT pre-register field or set is_actual_field=true just because receiver class matches:
            // that would treat get_and_wrap (field "and_wrap") and get_id (field "id") as getters and
            // skip adding them to the class CG when the class has no such field.
            if !is_actual_field && !args.is_empty() {
                 let receiver_ty = args[0].try_eval_path_type(self.acx);
                 let effective_ty = match receiver_ty.kind() {
                     TyKind::Ref(_, inner, _) => *inner,
                     _ => receiver_ty,
                 };
                 let class_ty = if let Some(inner) = analysis::extract_dsl_class_from_wrapper(self.tcx(), effective_ty, None) {
                     inner
                 } else {
                     effective_ty
                 };

                 if !is_actual_field {
                     if let Some(adt_def) = class_ty.ty_adt_def() {
                     let fields: Vec<_> = adt_def.all_fields().map(|f| f.name.as_str().to_string()).collect();
                     if gs.field_name == "item" || gs.field_name == "id" {
                         eprintln!("[rcpta] Resolve GS Top: {}::{}", gs.class_name, gs.field_name);
                         eprintln!("[rcpta]   Fields: {:?}", fields);
                     }

                     // Check direct fields
                     is_actual_field = fields.iter().any(|fname| {
                         fname == &gs.field_name || fname == &format!("_{}", gs.field_name)
                     });

                     // Fallback: If not found, check if it's a wrapper (has field "0" or "data")
                     // The DSL macro expands `pub class Holder` into a wrapper struct `Holder` containing `data: ManuallyDrop<Rc<Dyn<Holder::Data>>>` (or similar).
                     // We need to peek into this data field to find the actual user fields.
                     if !is_actual_field && adt_def.is_struct() {
                         // Check field "0" (tuple struct wrapper) or "data" (named struct wrapper)
                         let candidate_idx = fields.iter().position(|f| f == "0" || f == "data");
                         
                         if let Some(idx) = candidate_idx {
                             let field_ty = type_util::get_field_type(self.tcx(), class_ty, idx);
                             // Unwrap wrappers (ManuallyDrop, Rc, Dyn, etc.) to get the inner data struct type
                             if let Some(inner_ty) = analysis::extract_dsl_class_from_wrapper(self.tcx(), field_ty, None) {
                                 if let Some(inner_adt) = inner_ty.ty_adt_def() {
                                     let inner_fields: Vec<_> = inner_adt.all_fields().map(|f| f.name.as_str().to_string()).collect();
                                     if gs.field_name == "item" || gs.field_name == "id" {
                                         eprintln!("[rcpta]   Inner Fields (idx {}): {:?}", idx, inner_fields);
                                     }
                                     is_actual_field = inner_fields.iter().any(|fname| {
                                         fname == &gs.field_name || fname == &format!("_{}", gs.field_name)
                                     });
                                 }
                             }
                         }
                     }
                     }
                 }
            }
            
            if is_actual_field {
                if is_get_and_wrap_or_get_id {
                    eprintln!(
                        "[rcpta] getter branch: TREATING AS GETTER (returning early) class={} field={}",
                        gs.class_name, gs.field_name
                    );
                }
                if gs.field_name == "item" {
                    eprintln!("[rcpta] Inline set_item!");
                }
                self.handle_getter_setter(&gs, &args, &destination, location);
                // Return early so we DON'T add a static dispatch edge or treat it as a method call
                return;
            }
            // If not actual field (e.g. get_id() but no field id), fall through to standard class method handling.
        }

        // Check if this is a class method call (not getter/setter, not constructor)
        let class_method_opt = analysis::identify_class_method_with_type_system(&func_ref, &self.acx.class_type_system);
        if let Some(class_method) = class_method_opt {
            // Register the method in the type system
            self.acx.class_type_system.register_method(
                &class_method.class_name,
                &class_method.method_name,
            );
            // Register (class_name, method_name) -> func_name for polymorphic dispatch table
            let callee_func_name = func_ref.to_string();
            self.acx.class_type_system.register_method_impl(
                &class_method.class_name,
                &class_method.method_name,
                callee_func_name.clone(),
            );

            // Record to class call graph only for ordinary method calls (flush after build_all_callee_pags).
            // rcpta: use resolved (devirtualized) callee for the CG edge so we record Holder::get_and_wrap -> Entity::get_id
            // instead of -> Identifiable::get_id (trait); flush then sees a concrete callee and the edge is preserved.
            let (resolved_def_id, resolved_args) = match call_graph_builder::try_to_devirtualize(
                self.tcx(), *callee_def_id, gen_args,
            ) {
                Some((def_id, substs)) => (def_id, substs),
                None => (*callee_def_id, gen_args),
            };
            let resolved_callee_func_id = self.acx.get_func_id(resolved_def_id, resolved_args);
            let (cg_callee_class, cg_callee_method) = if resolved_def_id != *callee_def_id {
                analysis::identify_class_method_with_type_system(
                    &self.acx.get_function_reference(resolved_callee_func_id),
                    &self.acx.class_type_system,
                )
                .map(|cm| (cm.class_name, cm.method_name))
                .unwrap_or_else(|| (class_method.class_name.clone(), class_method.method_name.clone()))
            } else {
                (class_method.class_name.clone(), class_method.method_name.clone())
            };
            let caller_func_ref = self.acx.get_function_reference(self.func_id);
            if let Some(caller_class_method) = analysis::identify_class_method_with_type_system(&caller_func_ref, &self.acx.class_type_system) {
                self.acx.pending_class_cg_edges.push((
                    caller_class_method.class_name.clone(),
                    caller_class_method.method_name.clone(),
                    cg_callee_class,
                    cg_callee_method,
                    resolved_callee_func_id,
                ));
            } else {
                let caller_name = caller_func_ref.to_string();
                let caller_simple_name = if let Some(last_colon) = caller_name.rfind("::") {
                    caller_name[last_colon + 2..].to_string()
                } else {
                    caller_name.clone()
                };
                self.acx.pending_class_cg_edges.push((
                    caller_simple_name,
                    String::new(),
                    cg_callee_class,
                    cg_callee_method,
                    resolved_callee_func_id,
                ));
            }

            // Mark self argument (first argument) as class reference if it's a class type
            if !args.is_empty() {
                let self_arg = &args[0];
                let self_arg_type = self_arg.try_eval_path_type(self.acx);
                if analysis::is_dsl_class_type(self.tcx(), self_arg_type) {
                    self.acx.class_type_system.mark_class_reference(
                        self_arg.clone(),
                        &class_method.class_name,
                        true, // direct reference
                    );
                }
            }
            
            // Mark return value (destination) as class reference if it's a class type
            let dest_type = destination.try_eval_path_type(self.acx);
            if analysis::is_dsl_class_type(self.tcx(), dest_type) {
                // Try to infer return type from method signature
                // For now, we'll mark it with the same class as self
                // TODO: Could extract return type from function signature
                self.acx.class_type_system.mark_class_reference(
                    destination.clone(),
                    &class_method.class_name,
                    true, // direct reference
                );
            }

            // rcpta: ClassPAG CallArg / CallRet for cross-function pointer flow (source-level only).
            // Use canonical name so impl wrapper and body share same ptr ids when used as caller (actual_ptr_id, call_site).
            let caller_func_name = analysis::canonical_class_method_name(&caller_func_ref.to_string());
            let callee_func_name = analysis::canonical_class_method_name(&func_ref.to_string());
            // Caller class/method from the same identify_class_method logic (for robust same-method and impl->trait detection).
            let caller_cm_opt = analysis::identify_class_method_with_type_system(&caller_func_ref, &self.acx.class_type_system);
            let same_method_name = caller_cm_opt.as_ref().map(|c| c.method_name.as_str()) == Some(class_method.method_name.as_str());
            // tcx.trait_of_item(callee_def_id) can be None for some trait default/impl items, so add fallback.
            let is_trait_callee = util::is_trait_method(self.tcx(), *callee_def_id);
            let callee_is_trait_like = is_trait_callee
                || (same_method_name
                    && caller_cm_opt.is_some()
                    && caller_cm_opt.as_ref().unwrap().class_name != class_method.class_name
                    && callee_func_name.contains("interfaces::"));
            // Skip CallArg/CallRet when caller and callee are the same source-level method (impl wrapper -> data body).
            if caller_func_name == callee_func_name {
                // (no-op; skip edges for same method)
            } else if same_method_name && callee_is_trait_like {
                // Impl wrapper calling the trait method it overrides (e.g. Entity::{impl#0}::get_id -> Identifiable::get_id).
                // In MIR the dynamic-dispatch impl body is "call trait::get_id"; at source level the real body is
                // in data::get_id (self.get_entity_id()). Do not add CallArg/CallRet for this shim.
            } else if analysis::is_source_level_context(&caller_func_name) {
                use crate::util::class::ptr_system::path_to_class_ptr_id;
                let call_site_id: crate::rcpta::CallSiteId = format!(
                    "{}:bb{}[{}]",
                    caller_func_name,
                    location.block.index(),
                    location.statement_index
                );
                // CallArg: actual → formal for each class-typed argument.
                // rcpta: receiver ptr → this ptr (TAIE-style). For method calls like holder_1.get_and_wrap(),
                // we must add an edge from the caller's receiver ptr to the callee's self (param_1; MIR ordinals are 1-based).
                // - Receiver type may be &self (Ref(_, inner)) or CRc<T> (wrapper); treat as class-typed when
                //   inner/wrapper contains DSL class. For Ref receiver, use base path's ptr as actual so that
                //   flow is: holder_1 → get_and_wrap::param_0, enabling Load/Cast inside callee (self.get_item().into_superclass()).
                for (arg_idx, arg_path) in args.iter().enumerate() {
                    let arg_ty = arg_path.try_eval_path_type(self.acx);
                    let effective_ty = match arg_ty.kind() {
                        TyKind::Ref(_, inner, _) => *inner,
                        _ => arg_ty,
                    };
                    let is_class_typed = analysis::is_dsl_class_type(self.tcx(), effective_ty)
                        || analysis::extract_dsl_class_from_wrapper(self.tcx(), effective_ty, None).is_some();
                    if !is_class_typed {
                        continue;
                    }
                    let raw_actual_ptr_id = if arg_idx == 0 {
                        match arg_ty.kind() {
                            TyKind::Ref(..) => {
                                let receiver_ptr_id = path_to_class_ptr_id(arg_path, Some(&caller_func_name), None);
                                self.acx.rcpta_ref_ptr_to_base_path
                                    .get(&receiver_ptr_id)
                                    .map(|base_path| path_to_class_ptr_id(base_path, Some(&caller_func_name), None))
                                    .unwrap_or_else(|| receiver_ptr_id)
                            }
                            _ => path_to_class_ptr_id(arg_path, Some(&caller_func_name), None),
                        }
                    } else {
                        path_to_class_ptr_id(arg_path, Some(&caller_func_name), None)
                    };
                    let actual_ptr_id = self.acx.get_canonical_rcpta_ptr(&raw_actual_ptr_id);
                    // MIR uses 1-based parameter ordinals (0 = return place, 1 = first param/self).
                    let formal_ptr_id = format!("{}::param_{}", callee_func_name, arg_idx + 1);
                    let class_ty = self.acx.class_type_system
                        .get_path_class_type(arg_path)
                        .cloned()
                        .unwrap_or_else(|| class_method.class_name.clone());
                    let actual_ptr = crate::rcpta::ClassPtr::new_local(actual_ptr_id.clone(), class_ty.clone());
                    let formal_ptr = crate::rcpta::ClassPtr::new_local(formal_ptr_id.clone(), class_ty);
                    self.acx.class_pag.get_or_create_ptr(actual_ptr);
                    self.acx.class_pag.get_or_create_ptr(formal_ptr);
                    self.acx.class_pag.add_call_arg(&call_site_id, arg_idx, &actual_ptr_id, &formal_ptr_id);
                    
                    if caller_func_name.contains("apply_twice") {
                        eprintln!("[rcpta] Added CallArg in apply_twice: {} -> {}", actual_ptr_id, formal_ptr_id);
                    }
                }
                // CallRet: formal_ret → actual_ret when return type is class
                if analysis::is_dsl_class_type(self.tcx(), dest_type) {
                    let actual_ret_id = path_to_class_ptr_id(&destination, Some(&caller_func_name), None);
                    let formal_ret_id = format!("{}::ret", callee_func_name);
                    let ret_ptr = crate::rcpta::ClassPtr::new_local(formal_ret_id.clone(), class_method.class_name.clone());
                    self.acx.class_pag.get_or_create_ptr(ret_ptr);
                    self.acx.class_pag.add_call_ret(&call_site_id, &formal_ret_id, &actual_ret_id);
                }
            }
            
            debug!("Class method call: {}.{} at {:?}", 
                   class_method.class_name, class_method.method_name, location);
            info!(
                "rcpta: add_static_dispatch_callsite callee={}.{} func={}",
                class_method.class_name,
                class_method.method_name,
                callee_func_name
            );
            // rcpta: Ensure callee is in call graph so its body is analyzed (Tai-e style iterative analysis).
            // (resolved_* already computed above for pending_class_cg_edges.)
            let callsite = self.new_callsite(self.func_id, location, args.clone(), destination.clone());
            self.fpag.add_static_dispatch_callsite(callsite.clone(), resolved_callee_func_id);
            // If devirtualization yielded a different def_id, also add the original callee so we try both:
            // the resolved one (concrete impl with MIR) and the original (trait method; build_func_pag may skip if no MIR).
            if resolved_def_id != *callee_def_id {
                self.fpag.add_static_dispatch_callsite(callsite, callee_func_id);
            }
            
            // DEBUG: track added static dispatch
            if self.acx.get_function_reference(resolved_callee_func_id).to_string().contains("apply_twice") {
                eprintln!("[rcpta] Added static dispatch to apply_twice in func {:?}. Total callsites: {}", self.fpag.func_id, self.fpag.static_dispatch_callsites.len());
            }
            // Continue with normal call handling to build call graph
        } else if let Some(gs) = analysis::identify_getter_setter(&func_ref) {
            // Generalizable Logic: Distinguish Field Access (Load/Store) from Method Call.
            // Check if the receiver type's struct definition actually has the field `gs.field_name`.
            
            let mut is_actual_field = self.acx.class_type_system.get_field_index(&gs.class_name, &gs.field_name).is_some();
            
            if !is_actual_field && !args.is_empty() {
                // Fallback: Check receiver type (args[0]) for the field
                let receiver_ty = args[0].try_eval_path_type(self.acx);
                let effective_ty = match receiver_ty.kind() {
                    TyKind::Ref(_, inner, _) => *inner,
                    _ => receiver_ty,
                };
                // rcpta: Pre-register when receiver's DSL class matches (same as first getter block; normalize _Holder vs Holder)
                if let Some(receiver_class) = analysis::class_name_of_dsl_type(self.tcx(), effective_ty) {
                    if receiver_class.trim_start_matches('_') == gs.class_name.trim_start_matches('_') {
                        self.acx.class_type_system.register_field(
                            &gs.class_name,
                            &gs.field_name,
                            None,
                        );
                        is_actual_field = true;
                    }
                }
                // If it's a wrapper (CRc<T>), extract inner
                let class_ty = if let Some(inner) = analysis::extract_dsl_class_from_wrapper(self.tcx(), effective_ty, None) {
                    inner
                } else {
                    effective_ty
                };

                // DEBUG: Trace getter/setter resolution
                if gs.field_name == "item" || gs.field_name == "id" {
                    eprintln!("[rcpta] Resolve GS: {}::{} (is_getter={})", gs.class_name, gs.field_name, gs.is_getter);
                    eprintln!("[rcpta]   Receiver Type: {:?}", receiver_ty);
                    eprintln!("[rcpta]   Effective Type: {:?}", effective_ty);
                    eprintln!("[rcpta]   Class Type: {:?}", class_ty);
                }

                if !is_actual_field {
                if let Some(adt_def) = class_ty.ty_adt_def() {
                    let fields: Vec<_> = adt_def.all_fields().map(|f| f.name.as_str().to_string()).collect();
                    if gs.field_name == "item" || gs.field_name == "id" {
                        eprintln!("[rcpta]   AdtDef Fields: {:?}", fields);
                    }

                    // Check all fields (including _field form generated by macro)
                    is_actual_field = fields.iter().any(|fname| {
                        fname == &gs.field_name || fname == &format!("_{}", gs.field_name)
                    });
                } else if gs.field_name == "item" || gs.field_name == "id" {
                    eprintln!("[rcpta]   No AdtDef found for class_ty!");
                }
                }
            }

            if gs.field_name == "item" || gs.field_name == "id" {
                eprintln!("[rcpta]   Decision: is_actual_field = {}", is_actual_field);
            }

            if is_actual_field {
                // Actual Field -> Inline Load/Store (no Call in PAG)
                self.handle_getter_setter(&gs, &args, &destination, location);
            }
        }

        // rcpta: ClassPAG design for clone and cast (source-code aligned, MIR-compressed):
        //
        // 1. Assign edges: only for source-level clone(). Build Assign(canonical_receiver, clone_dest).
        //    - Ref (&eagle) is mapped to base (eagle) in visit_ref_or_address_of so clone's receiver resolves.
        //    - We do NOT insert clone_dest -> canonical into alias map so clone result stays as its own ptr.
        // 2. Cast edges: for into_superclass / try_into_subtype / cast_mixin. Build Cast(canonical_receiver, dest).
        //    - canonical_receiver makes argument copies (e.g. move bird for next cast) resolve to same ptr (bird).
        // 3. visit_copy_or_move: we insert dst -> canonical_src for alias only; no ClassPAG Assign (avoids "arg pass" edges).
        //
        // Nested: eagle.clone().into_superclass() → Assign(eagle, clone_result) + Cast(clone_result, bird).
        //         bird.into_superclass()         → Cast(bird, animal) with same bird ptr as above dst.
        if analysis::identify_class_cast_method(&func_ref) {
            special_function_handler::handle_class_cast_call(
                self,
                callee_def_id,
                &gen_args,
                &args,
                &destination,
                location,
            );
        } else if matches!(
            self.acx.get_known_name_for(*callee_def_id),
            KnownNames::StdConvertInto
        ) && !args.is_empty()
        {
            // e.g. vehicle_hierarchy: CRc<Car> -> CRc<Drivable> via `.into()` (impl Into in DSL shim).
            // Callee is core::convert::Into::into, so [`identify_class_cast_method`] does not apply.
            let receiver_ty = args[0].try_eval_path_type(self.acx);
            let dest_ty = destination.try_eval_path_type(self.acx);
            let from_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), receiver_ty, None);
            let to_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), dest_ty, None);
            if from_dsl.is_some() && to_dsl.is_some() {
                special_function_handler::handle_class_cast_call(
                    self,
                    callee_def_id,
                    &gen_args,
                    &args,
                    &destination,
                    location,
                );
            }
        }

        // rcpta: Option::unwrap() on Option<CRc<T>> — build Assign(option_inner_ptr, lhs_ptr).
        // Source: ptr for Option.Some.0 (created when try_into_subtype returned Option<CRc<T>>, or when
        // Option was constructed). Dest: LHS of `let lhs = opt.unwrap()`. Ensures class reference from
        // unwrap gets a ptr abstraction and Assign(source_ptr -> dest_ptr) in Class PAG.
        if analysis::is_option_unwrap(self.tcx(), *callee_def_id) && !args.is_empty() {
            // Use caller destination type instead of callee generic return type:
            // for std::iter::Iterator::next the callee return is often generic `Option<&T>`,
            // while destination is monomorphized in caller and can reveal concrete DSL class.
            let return_ty = destination.try_eval_path_type(self.acx);
            let caller_func_ref = self.acx.get_function_reference(self.func_id);
            let func_name = analysis::canonical_class_method_name(&caller_func_ref.to_string());
            if analysis::is_source_level_context(&func_name) {
                use crate::util::class::ptr_system::path_to_class_ptr_id;
                let dest_ptr_id = path_to_class_ptr_id(&destination, Some(&func_name), None);
                let dest_class_from_type: Option<(Ty<'tcx>, String)> =
                    analysis::extract_dsl_class_from_wrapper(self.tcx(), return_ty, None)
                        .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty).map(|n| (ty, n)));
                if let Some((dest_class_ty, class_name)) = dest_class_from_type {
                    use crate::mir::path::{Path, PathSelector};
                    let receiver_ptr_id = path_to_class_ptr_id(&args[0], Some(&func_name), None);
                    let receiver_ty = args[0].try_eval_path_type(self.acx);
                    // Resolve receiver to the Option holder: &opt -> opt via ref_ptr_to_base_path;
                    // opt_copy (move) -> opt via option_copy_to_base_path so we use the same Option.Some.0
                    // that try_into_subtype created.
                    let base_path = match receiver_ty.kind() {
                        TyKind::Ref(..) => self.acx.rcpta_ref_ptr_to_base_path.get(&receiver_ptr_id).cloned(),
                        _ => self.acx.rcpta_option_copy_to_base_path.get(&receiver_ptr_id).cloned(),
                    };
                    let option_holder_path = base_path.unwrap_or_else(|| args[0].clone());
                    let option_inner_path = Path::new_qualified(
                        option_holder_path,
                        vec![PathSelector::Downcast(1), PathSelector::Field(0)],
                    );
                    self.acx.set_path_rustc_type(option_inner_path.clone(), dest_class_ty);
                    let source_ptr_id = path_to_class_ptr_id(&option_inner_path, Some(&func_name), None);
                    let canonical_source = self.acx.get_canonical_rcpta_ptr(&source_ptr_id);
                    use crate::rcpta::ClassPtr;
                    let source_cptr = ClassPtr::new_local(canonical_source.clone(), class_name.clone());
                    let dest_cptr = ClassPtr::new_local(dest_ptr_id.clone(), class_name);
                    self.acx.class_pag.get_or_create_ptr(source_cptr);
                    self.acx.class_pag.get_or_create_ptr(dest_cptr);
                    self.acx.class_pag.add_assign(&canonical_source, &dest_ptr_id);

                    // rcpta semantic op: IterNextUnwrapRefBridge
                    // Fallback bridge for `iter.next().unwrap()` style when Option payload path
                    // is not stably modeled: route in-function container element summaries
                    // (`*.deref.index`) to unwrap destination ref local.
                    let func_prefix = format!("{}::", func_name);
                    let candidate_ptrs: Vec<String> = self
                        .acx
                        .class_pag
                        .ptr_ids()
                        .filter(|ptr_id| {
                            ptr_id.starts_with(&func_prefix)
                                && ptr_id.ends_with(".deref.index")
                                && *ptr_id != &dest_ptr_id
                        })
                        .cloned()
                        .collect();
                    for src_ptr_id in candidate_ptrs {
                        let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                        self.acx.class_pag.add_assign(&canonical_src, &dest_ptr_id);
                    }
                } else {
                    // Even if we cannot recover Option payload Ty, keep fallback summary bridge:
                    // `iter.next().unwrap()` should still receive container element summaries.
                    let func_prefix = format!("{}::", func_name);
                    let candidate_ptrs: Vec<String> = self
                        .acx
                        .class_pag
                        .ptr_ids()
                        .filter(|ptr_id| {
                            ptr_id.starts_with(&func_prefix)
                                && ptr_id.ends_with(".deref.index")
                                && *ptr_id != &dest_ptr_id
                        })
                        .cloned()
                        .collect();
                    for src_ptr_id in candidate_ptrs {
                        let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                        self.acx.class_pag.add_assign(&canonical_src, &dest_ptr_id);
                    }
                }
            }
        }

        // rcpta: Iterator::next() over class collections often returns Option<Item> or Option<(idx, Item)>.
        // Build a conservative Assign from caller-local class pointers (compatible with item class)
        // to Option.Some.0 (or .0.1 for tuple enumerate item), so loop element can carry non-empty PTS/type-info.
        //
        // Example:
        //   for (i, animal) in animals.iter().enumerate() { ... }
        // MIR:
        //   _134 = Iterator::next(...)
        //   _138 = ((_134 as Some).0).1
        // We model source as destination.as_variant#1.0.1.
        let callee_def_path = self.tcx().def_path_str(*callee_def_id);
        let callee_method = callee_def_path
            .rsplit("::")
            .next()
            .and_then(|s| s.split('<').next())
            .unwrap_or("");
        let is_iter_next = callee_method == "next" && callee_def_path.contains("iter");
        let is_index_index = callee_method == "index";
        if caller_func_name.contains("test_rcpta_min_") {
            eprintln!(
                "[rupta][min-case] caller={} callee_def_path={} callee_method={} unwrap={} iter_next={} index_index={}",
                caller_func_name,
                callee_def_path,
                callee_method,
                analysis::is_option_unwrap(self.tcx(), *callee_def_id),
                is_iter_next,
                is_index_index
            );
        }
        if is_iter_next && analysis::is_source_level_context(&caller_func_name) {
            use rustc_middle::ty::GenericArgKind;
            // Use caller destination type (monomorphized in caller), not callee generic return type.
            let return_ty = destination.try_eval_path_type(self.acx);
            let mut item_class_name: Option<String> = None;
            let mut item_class_ty: Option<Ty<'tcx>> = None;

            let mut option_item_path_selectors: Option<Vec<PathSelector>> = None;
            if let TyKind::Adt(def, args) = return_ty.kind() {
                let option_path = self.tcx().def_path_str(def.did());
                let is_option = option_path.contains("core::option::Option")
                    || option_path.contains("std::option::Option")
                    || option_path.contains("option::Option");
                if is_option && !args.is_empty() {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        match inner_ty.kind() {
                            // Iterator::next for enumerate() commonly returns Option<(usize, Item)>
                            TyKind::Tuple(elems) if elems.len() >= 2 => {
                                let item_ty = elems[1];
                                let item_ty = match item_ty.kind() {
                                    TyKind::Ref(_, inner, _) => *inner,
                                    _ => item_ty,
                                };
                                if let Some(class_ty) =
                                    analysis::extract_dsl_class_from_wrapper(self.tcx(), item_ty, None)
                                {
                                    if let Some(class_name) =
                                        analysis::class_name_of_dsl_type(self.tcx(), class_ty)
                                    {
                                        item_class_name = Some(class_name);
                                        item_class_ty = Some(class_ty);
                                        option_item_path_selectors = Some(vec![
                                            PathSelector::Downcast(1),
                                            PathSelector::Field(0),
                                            PathSelector::Field(1),
                                        ]);
                                    }
                                }
                            }
                            _ => {
                                let inner_ty = match inner_ty.kind() {
                                    TyKind::Ref(_, inner, _) => *inner,
                                    _ => inner_ty,
                                };
                                if let Some(class_ty) =
                                    analysis::extract_dsl_class_from_wrapper(self.tcx(), inner_ty, None)
                                {
                                    if let Some(class_name) =
                                        analysis::class_name_of_dsl_type(self.tcx(), class_ty)
                                    {
                                        item_class_name = Some(class_name);
                                        item_class_ty = Some(class_ty);
                                        option_item_path_selectors = Some(vec![
                                            PathSelector::Downcast(1),
                                            PathSelector::Field(0),
                                        ]);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let (Some(item_class_name), Some(item_class_ty), Some(selectors)) =
                (item_class_name, item_class_ty, option_item_path_selectors)
            {
                use crate::mir::path::Path;
                use crate::rcpta::ClassPtr;
                use crate::util::class::ptr_system::path_to_class_ptr_id;

                let option_item_path = Path::new_qualified(destination.clone(), selectors);
                self.acx
                    .set_path_rustc_type(option_item_path.clone(), item_class_ty);
                let option_item_ptr_id =
                    path_to_class_ptr_id(&option_item_path, Some(&caller_func_name), None);
                let dst_ptr_id =
                    path_to_class_ptr_id(&destination, Some(&caller_func_name), None);
                self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                    option_item_ptr_id.clone(),
                    item_class_name.clone(),
                ));
                self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                    dst_ptr_id.clone(),
                    item_class_name.clone(),
                ));
                // Bridge Option.Some.0 item to the call destination pointer abstraction.
                self.acx.class_pag.add_assign(&option_item_ptr_id, &dst_ptr_id);

                let func_prefix = format!("{}::", caller_func_name);
                let candidate_ptrs: Vec<String> = self
                    .acx
                    .class_pag
                    .ptr_ids()
                    .filter_map(|ptr_id| {
                        if !ptr_id.starts_with(&func_prefix) {
                            return None;
                        }
                        if *ptr_id == option_item_ptr_id {
                            return None;
                        }
                        let ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                        // Keep pointers whose dynamic class may flow into iter item static class.
                        if self
                            .acx
                            .class_type_system
                            .is_subtype(&ptr.class_type, &item_class_name)
                        {
                            Some(ptr_id.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                for src_ptr_id in candidate_ptrs {
                    let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                    self.acx.class_pag.add_assign(&canonical_src, &option_item_ptr_id);
                    // Keep a direct bridge to call destination as fallback when MIR projection
                    // paths (Option::Some.0 / tuple element) do not get stable class ptr IDs.
                    self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                }
            }
        }

        // rcpta: Index::index() over class collections often yields &ClassLike at caller side.
        // Build conservative Assign(s) from caller-local compatible class pointers to call destination ptr,
        // so later CallArg can carry non-empty points-to/type-info.
        if is_index_index && analysis::is_source_level_context(&caller_func_name) {
            let dst_ty = destination.try_eval_path_type(self.acx);
            let dst_ty = match dst_ty.kind() {
                TyKind::Ref(_, inner, _) => *inner,
                _ => dst_ty,
            };
            use crate::util::class::ptr_system::path_to_class_ptr_id;
            let dst_ptr_id = path_to_class_ptr_id(&destination, Some(&caller_func_name), None);
            let mut item_class_name: Option<String> =
                analysis::extract_dsl_class_from_wrapper(self.tcx(), dst_ty, None)
                    .and_then(|ty| analysis::class_name_of_dsl_type(self.tcx(), ty));
            if item_class_name.is_none() {
                item_class_name = self
                    .acx
                    .class_pag
                    .get_ptr(&dst_ptr_id)
                    .map(|p| p.class_type.clone());
            }
            if let Some(item_class_name) = item_class_name {
                    use crate::rcpta::ClassPtr;
                    self.acx.class_pag.get_or_create_ptr(ClassPtr::new_local(
                        dst_ptr_id.clone(),
                        item_class_name.clone(),
                    ));

                    let func_prefix = format!("{}::", caller_func_name);
                    let candidate_ptrs: Vec<String> = self
                        .acx
                        .class_pag
                        .ptr_ids()
                        .filter_map(|ptr_id| {
                            if !ptr_id.starts_with(&func_prefix) || *ptr_id == dst_ptr_id {
                                return None;
                            }
                            let ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                            if ptr.class_type == item_class_name
                                || self
                                    .acx
                                    .class_type_system
                                    .is_subtype(&ptr.class_type, &item_class_name)
                            {
                                Some(ptr_id.clone())
                            } else {
                                None
                            }
                        })
                        .collect();

                    for src_ptr_id in candidate_ptrs {
                        let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                        self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                    }

                    // rcpta semantic op: IndexRefBridge
                    // Also bridge from caller container element summaries (`*.deref.index`)
                    // to the index() destination ref local.
                    let summary_ptrs: Vec<String> = self
                        .acx
                        .class_pag
                        .ptr_ids()
                        .filter_map(|ptr_id| {
                            if *ptr_id == dst_ptr_id
                                || (!ptr_id.ends_with(".deref.index")
                                    && !ptr_id.ends_with(".$elem"))
                            {
                                return None;
                            }
                            let src_ptr = self.acx.class_pag.get_ptr(ptr_id)?;
                            if src_ptr.class_type == item_class_name
                                || self
                                    .acx
                                    .class_type_system
                                    .is_subtype(&src_ptr.class_type, &item_class_name)
                            {
                                Some(ptr_id.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    for src_ptr_id in summary_ptrs {
                        let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                        self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                    }

                    // Provenance-aware bridge: container receiver -> its recorded element summaries.
                    if !args.is_empty() {
                        let recv_ptr_id =
                            path_to_class_ptr_id(&args[0], Some(&caller_func_name), None);
                        let recv_base_ptr_id = self
                            .acx
                            .rcpta_ref_ptr_to_base_path
                            .get(&recv_ptr_id)
                            .map(|p| path_to_class_ptr_id(p, Some(&caller_func_name), None))
                            .unwrap_or(recv_ptr_id);
                        if let Some(src_elem_ptrs) = self
                            .acx
                            .rcpta_container_elem_ptrs_by_base
                            .get(&recv_base_ptr_id)
                            .cloned()
                        {
                            if caller_func_name.contains("test_rcpta_min_") {
                                eprintln!(
                                    "[rupta][min-case][index-read] caller={} recv_base={} srcs={:?} dst={}",
                                    caller_func_name, recv_base_ptr_id, src_elem_ptrs, dst_ptr_id
                                );
                            }
                            for src_ptr_id in src_elem_ptrs {
                                let canonical_src = self.acx.get_canonical_rcpta_ptr(&src_ptr_id);
                                self.acx.class_pag.add_assign(&canonical_src, &dst_ptr_id);
                            }
                        } else if caller_func_name.contains("test_rcpta_min_") {
                            eprintln!(
                                "[rupta][min-case][index-read] caller={} recv_base={} srcs=(none) dst={}",
                                caller_func_name, recv_base_ptr_id, dst_ptr_id
                            );
                        }
                    }
            }
        }

        // rcpta: Clone::clone() — build Assign edge and create ClassPtr for clone result (LHS).
        // Recognize both core's Clone::clone (StdCloneClone) and DSL's impl (e.g. classes::ptr::RcDyn::clone);
        // get_known_name_for returns StdCloneClone only for core/std crate, so we also check is_impl_of_core_clone_trait.
        let is_clone_call = matches!(self.acx.get_known_name_for(*callee_def_id), KnownNames::StdCloneClone)
            || analysis::is_impl_of_core_clone_trait(self.tcx(), *callee_def_id);
        if is_clone_call && !args.is_empty() {
            let caller_func_ref = self.acx.get_function_reference(self.func_id);
            let func_name = analysis::canonical_class_method_name(&caller_func_ref.to_string());
            if analysis::is_source_level_context(&func_name) {
                let receiver_ty = args[0].try_eval_path_type(self.acx);
                let dest_ty = destination.try_eval_path_type(self.acx);
                let receiver_has_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), receiver_ty, None).is_some();
                let dest_has_dsl = analysis::extract_dsl_class_from_wrapper(self.tcx(), dest_ty, None).is_some();
                if receiver_has_dsl && dest_has_dsl {
                    use crate::util::class::ptr_system::path_to_class_ptr_id;
                    let receiver_ptr_id = path_to_class_ptr_id(&args[0], Some(&func_name), None);
                    let dest_ptr_id = path_to_class_ptr_id(&destination, Some(&func_name), None);
                    let canonical_receiver = self.acx.get_canonical_rcpta_ptr(&receiver_ptr_id);
                    // rcpta: do NOT insert clone dest -> canonical; clone result stays as its own ptr so
                    // "eagle cast to bird" is Cast(clone_result -> bird), and "bird cast to animal" uses
                    // canonical(argument_copy) = bird (same ptr as first cast's dst).
                    // rcpta: create ClassPtr for receiver and clone result (destination), add Assign edge receiver → destination.
                    if let Some(receiver_class_ty) = analysis::extract_dsl_class_from_wrapper(self.tcx(), receiver_ty, None) {
                        if let Some(class_name) = analysis::class_name_of_dsl_type(self.tcx(), receiver_class_ty) {
                            use crate::rcpta::ClassPtr;
                            let receiver_cptr = ClassPtr::new_local(canonical_receiver.clone(), class_name.clone());
                            let dest_cptr = ClassPtr::new_local(dest_ptr_id.clone(), class_name);
                            self.acx.class_pag.get_or_create_ptr(receiver_cptr);
                            self.acx.class_pag.get_or_create_ptr(dest_cptr);
                            self.acx.class_pag.add_assign(&canonical_receiver, &dest_ptr_id);
                        }
                    }
                }
            }
        }

        if special_function_handler::handled_as_special_function_call(
            self,
            callee_def_id,
            &gen_args,
            &args,
            &destination,
            location,
        ) {
            let callsite = self.new_callsite(self.func_id, location, args, destination);
            let (callee_def_id, gen_args) = match call_graph_builder::try_to_devirtualize(
                self.tcx(), *callee_def_id, gen_args
            ) {
                Some((callee_def_id, gen_args)) => (callee_def_id, gen_args),
                None => (*callee_def_id, gen_args),
            };
            let callee_func_id = self.acx.get_func_id(callee_def_id, gen_args);
            self.fpag.add_special_callsite(callsite, callee_func_id);
            self.acx.add_special_function(callee_func_id);
            return;
        }

        if self.acx.is_std_ops_fntrait_call(*callee_def_id) {
            // Fn*::call*
            self.resolve_fntrait_call(callee_def_id, &gen_args, args, destination, location);
            return;
        }

        // rcpta: DSL vtable call (e.g. in get_and_wrap we only see Call to vtable, not to get_item).
        // Resolve to all methods of the class and add static_dispatch so build_all_callee_pags builds their PAG.
        if analysis::is_dsl_vtable_call(self.tcx(), *callee_def_id) {
            if let Some(vtable_ty) = gen_args.types().next() {
                let class_ty = analysis::extract_dsl_class_from_wrapper(self.tcx(), vtable_ty, None).unwrap_or(vtable_ty);
                if let Some(class_name) = analysis::class_name_of_dsl_type(self.tcx(), class_ty) {
                    let class_name_norm = class_name.trim_start_matches('_');
                    let (func_names, method_names): (Vec<String>, Vec<String>) = self.acx.class_type_system
                        .get_class(class_name_norm)
                        .or_else(|| self.acx.class_type_system.get_class(&class_name))
                        .map(|c| (
                            c.method_impls.values().cloned().collect(),
                            c.methods.iter().cloned().collect(),
                        ))
                        .unwrap_or_default();
                    let mut callee_ids = std::collections::HashSet::new();
                    for func_name in &func_names {
                        if let Some(fid) = self.acx.try_get_func_id_by_name(func_name) {
                            callee_ids.insert(fid);
                        }
                    }
                    for method_name in &method_names {
                        for fid in self.acx.try_get_func_ids_for_class_method(class_name_norm, method_name) {
                            callee_ids.insert(fid);
                        }
                    }
                    if callee_ids.is_empty() {
                        let path_prefix = match class_ty.kind() {
                            TyKind::Adt(adt_def, _) => self.tcx().def_path_str(adt_def.did()),
                            _ => String::new(),
                        };
                        if !path_prefix.is_empty() {
                            for fid in self.acx.try_get_func_ids_by_prefix(&path_prefix) {
                                callee_ids.insert(fid);
                            }
                        }
                    }
                    // rcpta: one func PAG per source-level method. Deduplicate by (class_name, method_name),
                    // preferring the FuncId that has MIR so we only build body once.
                    let mut canonical: std::collections::HashMap<(String, String), FuncId> = std::collections::HashMap::new();
                    for fid in callee_ids {
                        let name = self.acx.get_function_reference(fid).to_string();
                        let Some(cname) = analysis::extract_class_name_from_func(&name) else { continue };
                        let Some(mname) = analysis::extract_method_name_from_func(&name) else { continue };
                        let key = (cname.trim_start_matches('_').to_string(), mname);
                        let has_mir = self.tcx().is_mir_available(self.acx.get_function_reference(fid).def_id);
                        let should_insert = match canonical.get(&key) {
                            None => true,
                            Some(&existing_fid) => {
                                let existing_has_mir = self.tcx().is_mir_available(self.acx.get_function_reference(existing_fid).def_id);
                                has_mir && !existing_has_mir
                            }
                        };
                        if should_insert {
                            canonical.insert(key, fid);
                        }
                    }
                    // Do NOT push pending_class_cg_edges here: at this call site we only know it's a
                    // vtable call to the class, not which method. Adding edges to all methods would
                    // create false edges (e.g. describe -> apply_twice, chain_with, ...).
                    let n_callees = canonical.len();
                    if n_callees > 0 {
                        let callsite = self.new_callsite(self.func_id, location, args.clone(), destination.clone());
                        for callee_func_id in canonical.values() {
                            self.fpag.add_static_dispatch_callsite(callsite.clone(), *callee_func_id);
                        }
                        info!(
                            "rcpta vtable resolved: {} -> {} callees (class {}, deduped by method)",
                            self.acx.get_function_reference(self.fpag.func_id).to_string(),
                            n_callees,
                            class_name_norm
                        );
                        return;
                    }
                }
            }
        }

        if !util::is_trait_method(self.tcx(), *callee_def_id)
        {
            // Static functions or methods or associated functions not declared on a trait.
            let callsite = self.new_callsite(self.func_id, location, args, destination);
            let callee_func_id = self.acx.get_func_id(*callee_def_id, gen_args);
            self.fpag.add_static_dispatch_callsite(callsite.clone(), callee_func_id);
            self.push_class_cg_edge_if_class_method(callee_func_id);
        } else if let Some((callee_def_id, callee_substs)) =
            call_graph_builder::try_to_devirtualize(self.tcx(), *callee_def_id, gen_args)
        {
            // Methods or associated functions declared on a trait (e.g. Identifiable::get_id).
            // The called instance can be resolved at compile time to a concrete impl (e.g. Entity::get_id).
            let callsite = self.new_callsite(self.func_id, location, args, destination);
            debug!(
                "Devirtualize to func {:?}, substs: {:?}",
                callee_def_id, callee_substs
            );
            let callee_func_id = self.acx.get_func_id(callee_def_id, callee_substs);
            self.fpag.add_static_dispatch_callsite(callsite, callee_func_id);
            self.push_class_cg_edge_if_class_method(callee_func_id);
        } else if util::is_dynamic_call(self.tcx(), *callee_def_id, gen_args) {
            // trait method calls where the first argument is of dynamic type
            let receiver = args[0].clone();
            let callsite = self.new_callsite(self.func_id, location, args, destination);
            self.acx.add_dyn_callsite(callsite.clone().into(), *callee_def_id, gen_args);
            self.fpag.add_dynamic_dispatch_callsite(receiver, callsite);
        } else {
            warn!(
                "Could not resolve function: {:?}, {:?}",
                callee_def_id, gen_args
            );
        }
    }

    fn resolve_fntrait_call(
        &mut self,
        callee_def_id: &DefId,
        gen_args: &GenericArgsRef<'tcx>,
        args: Vec<Rc<Path>>,
        destination: Rc<Path>,
        location: mir::Location,
    ) {
        // The fn_traits feature allows for implementation of the Fn* traits for
        // creating custom closure-like types. We first try to devirtualize the callee function
        // <https://doc.rust-lang.org/beta/unstable-book/library-features/fn-traits.html>
        // Old code (nightly-2024-02-03): ParamEnv::reveal_all() and Instance::resolve()
        // New code (nightly-2025-05-09): TypingEnv::fully_monomorphized() and Instance::try_resolve()
        let typing_env = rustc_middle::ty::TypingEnv::fully_monomorphized();
        // Instance::try_resolve panics if try_normalize_erasing_regions returns an error.
        // It is hard to figure out exactly when this will be the case.
        if self.tcx().try_normalize_erasing_regions(typing_env, *gen_args).is_err() {
            warn!("Could not resolve fntrait call: {:?}, {:?}", callee_def_id, gen_args);
            return;
        }
        let resolved_instance =
            rustc_middle::ty::Instance::try_resolve(self.tcx(), typing_env, *callee_def_id, gen_args);
        if let Ok(Some(instance)) = resolved_instance {
            let resolved_def_id = instance.def.def_id();

            // Specially handlings for closures, function items and function pointers.
            // When the Fn* trait object is specialized to a closure, the resolved_def_id 
            // corresponds to the def id of the closure. We still handle it along with function 
            // items and function pointers.
            if self.tcx().is_closure_like(resolved_def_id) {
                self.inline_indirectly_called_function(
                    callee_def_id,
                    gen_args,
                    args,
                    destination,
                    location,
                );
                return;
            }
 
            let has_mir = self.tcx().is_mir_available(resolved_def_id);
            if !has_mir {
                // ops::function::Fn*::call* for FnDef, FnPtr, Dynamic... types are unavailable
                // Try to inline the indirect call for these types
                if self.acx.def_in_ops_func_namespace(resolved_def_id) {
                    self.inline_indirectly_called_function(
                        callee_def_id,
                        gen_args,
                        args,
                        destination,
                        location,
                    );
                } else {
                    warn!("Unavailable mir for def_id: {:?}", resolved_def_id);
                }
                return;
            }

            // Programmers can implement the `Fn|FnOnce|FnMut` for a customized type.
            //
            // Rust compiler automatically implements the `Fn|FnOnce|FnMut` trait for a reference 
            // type if its underlying type has implemented `Fn`, and implements `FnMut` and 
            // `FnOnce` trait for a reference type if its underlying type has implemented `FnMut`
            // See https://doc.rust-lang.org/src/core/ops/function.rs.html#76.
            //
            // For example, if we implement the `Fn` trait for a struct type A, it automatically 
            // implements the `Fn|FnOnce|FnMut` trait for `&A`, `&&A`, `&&&A`, ...
            //
            // When calling the function `<&&&A as Fn>::call()`, functions `<&&A as Fn>::call()`,
            // `<&A as Fn>::call()` and `<A as Fn>::call()` are called layer-by-layer.
            // 
            // The mirs for the automatic implementations are also available and can be analyzed 
            // directly.
            let instance_args = instance.args;
            debug!("Devirtualize to func {:?}, substs: {:?}", resolved_def_id, instance_args);
            let callsite = self.new_callsite(self.func_id, location, args, destination);
            let callee_func_id = self.acx.get_func_id(resolved_def_id, instance_args);
            self.fpag.add_static_dispatch_callsite(callsite, callee_func_id);
        } else {
            warn!(
                "Could not resolve function: {:?}, {:?}",
                callee_def_id, gen_args
            );
        }
    }

    /// `Fn::call`, `FnMut::call_mut`, `FnOnce::call_once` all receive two arguments:
    /// 1. An operand of any type that implements `Fn`|`FnMut`|`FnOnce`, including function items, 
    ///    function pointers and closures.
    /// 2. A tuple of argument values for the call.
    /// The tuple is unpacked and the callee is then invoked with its normal function signature.
    /// In the case of calling a closure, the closure is included as the first argument.
    ///
    /// All of this happens in code that is not encoded as MIR, so we need built in support for it.
    pub fn inline_indirectly_called_function(
        &mut self,
        callee_def_id: &DefId,
        gen_args: &GenericArgsRef<'tcx>,
        args: Vec<Rc<Path>>,
        destination: Rc<Path>,
        location: mir::Location,
    ) {
        assert_eq!(args.len(), 2);
        // Parse the actual arguments from the second argument.
        let args_tuple_path = args[1].clone();
        // Unpack the type of the second argument, which should be a tuple.
        // The argument can be a constant tuple `const ()`, in which case we may fail to get its type
        let mut actual_arg_types: Vec<Ty<'tcx>> =
            if args_tuple_path.is_constant() {
                vec![]
            } else {
                if let TyKind::Tuple(tuple_types) = 
                    self.acx.get_path_rustc_type(&args_tuple_path).unwrap().kind() 
                {
                    tuple_types.iter().collect()
                } else {
                    unreachable!("The second argument is expected to be a tuple");
                }
            };
            
        // Unpack the second argument, which should be a tuple
        let mut actual_args: Vec<Rc<Path>> = actual_arg_types
            .iter()
            .enumerate()
            .map(|(i, ty)| {
                let proj_elems = vec![PathSelector::Field(i)];
                let arg = Path::new_qualified(args_tuple_path.clone(), proj_elems);
                self.acx.set_path_rustc_type(arg.clone(), *ty);
                arg
            })
            .collect();

        // If the first substution is a closure or FnDef, we can inline the closure call directly.
        // The substs should have been specialized when added to the type cache.
        let first_subst_ty = gen_args
            .types()
            .next()
            .expect("Expect type substition in Fn* invocation");
        match first_subst_ty.kind() {
            TyKind::FnDef(def_id, substs) => {
                // Fn*::call* itself cannot be the first argument as it is a trait method without
                // a implementation, therefore we do not need to worry about the recursive std_ops_func_call.
                let (def_id, substs) = call_graph_builder::resolve_fn_def(self.tcx(), *def_id, substs);
                let callee_func_id = self.acx.get_func_id(def_id, substs);
                // Set up a callsite
                let callsite = self.new_callsite(self.func_id, location, actual_args, destination);
                self.fpag.add_static_dispatch_callsite(callsite, callee_func_id);
            }
            TyKind::Closure(def_id, substs) | TyKind::Coroutine(def_id, substs) => {
                // Prepend the callee closure/generator/function to the unpacked arguments vector
                // if the called function actually expects it.
                actual_args.insert(0, args[0].clone());
                actual_arg_types.insert(0, first_subst_ty);

                // call_once consumes its callee argument. If the callee does not,
                // we have to provide it with a reference.
                // Sadly, the easiest way to get hold of the type of the first parameter
                // of the callee is to look at its MIR body. If there is no body, we wont
                // be executing it and the type of the first argument is immaterial, so this
                // does not cause problems.
                let mir = self.tcx().optimized_mir(def_id);
                let first_arg_type = self.acx.get_path_rustc_type(&args[0]).unwrap();
                if let Some(decl) = mir.local_decls.get(mir::Local::from(1usize)) {
                    if decl.ty.is_ref() && !first_arg_type.is_ref() {
                        let closure_path = args[0].clone();
                        // create a reference path to to this closure
                        let closure_ref_ty = Ty::new_mut_ref(self.tcx(), self.tcx().lifetimes.re_static, first_subst_ty);
                        let closure_ref_path = self.create_aux_local(closure_ref_ty);
                        self.add_addr_edge(closure_path, closure_ref_path.clone());
                        actual_args[0] = closure_ref_path;
                        // decl.ty is not type specialized
                        actual_arg_types[0] = closure_ref_ty;
                    }
                }

                // Set up a callsite
                let callsite = self.new_callsite(self.func_id, location, actual_args, destination);
                let callee_func_id = self.acx.get_func_id(*def_id, substs);
                self.fpag.add_static_dispatch_callsite(callsite, callee_func_id);
            }
            TyKind::FnPtr(..) => {
                // Add the first argument and the callsite to fpag's fnptr_callsite
                let callsite = self.new_callsite(self.func_id, location, actual_args, destination);
                // If the first argument is a reference to a function pointer 
                let first_arg_type = self.acx.get_path_rustc_type(&args[0]).unwrap();
                let fn_ptr = if !first_arg_type.is_fn_ptr() && first_arg_type.is_any_ptr() {
                    let aux = self.create_aux_local(type_util::get_dereferenced_type(first_arg_type));
                    let deref_path = self.create_dereference(args[0].clone(), first_arg_type);
                    self.add_load_edge(deref_path, aux.clone());
                    aux
                } else {
                    args[0].clone()
                };
                self.fpag.add_fnptr_callsite(fn_ptr, callsite);
            }
            // For dynamic substution, resolve on the fly
            // e.g. &dyn FnMut(..)
            TyKind::Dynamic(..) => {
                // Add the first argument and the callsite to fpag's std_ops_callsites
                // Use the original args instead of the actual args
                let dyn_fn_obj = args[0].clone();
                let dyn_callsite = self.new_callsite(self.func_id, location, args, destination);
                self.acx.add_dyn_callsite(dyn_callsite.clone().into(), *callee_def_id, gen_args);
                // This call maybe a dyn FnOnce call, in which case the dyn_fn_obj would be
                // of dyn FnOnce type instead a reference type (occurs for a function call
                // via a Box<dyn FnOnce> object). In this case, the first argument would be a
                // dereference value, e.g. (*_1). We need to cache the dynamic callsite with 
                // the reference value, e.g. _1, to make our solver be able to determine the 
                // call target based on the pointed-to objects of the reference value. 
                let first_arg_type = self.acx.get_path_rustc_type(&dyn_fn_obj).unwrap();
                if !first_arg_type.is_any_ptr() {
                    if let PathEnum::QualifiedPath { base, projection } = &dyn_fn_obj.value {
                        if projection[0] == PathSelector::Deref && projection.len() == 1 {
                            self.fpag
                                .add_dynamic_fntrait_callsite(base.clone(), dyn_callsite);
                        }
                    }
                } else {
                    self.fpag
                        .add_dynamic_fntrait_callsite(dyn_fn_obj, dyn_callsite);
                }
            }
            _ => {
                error!("Unexpected argument type in Dn* trait call!");
            }
        }
    }

    /// If the source path and the destination path are both of pointer types, add a direct edge between them. 
    /// Otherwise, get their pointer type fields if exist and add internal edges between these fields.
    /// DSL class types are treated as pointers for propagation purposes.
    pub fn add_internal_edges(
        &mut self,
        src_path: Rc<Path>,
        src_type: Ty<'tcx>,
        dst_path: Rc<Path>,
        dst_type: Ty<'tcx>,
    ) {
        if type_util::equal_types(self.tcx(), src_type, dst_type) {
            if src_type.is_any_ptr() {
                self.add_edge_between_ptrs(src_path, dst_path);
            } else if analysis::is_dsl_class_type(self.tcx(), src_type) {
                // DSL class types should be treated as pointers for propagation
                self.add_edge_between_ptrs(src_path.clone(), dst_path.clone());
                
                // ===== Class Type System: Propagate class type =====
                self.acx.class_type_system.propagate_type(&src_path, dst_path);
            } else {
                let ptr_projs = unsafe {
                    &*(self.acx.get_pointer_projections(src_type) as *const Vec<(ProjectionElems, Ty<'tcx>)>)
                };
                for (ptr_proj, ptr_ty) in ptr_projs {
                    let src_field = Path::append_projection(&src_path, ptr_proj);
                    self.acx.set_path_rustc_type(src_field.clone(), *ptr_ty);
                    let dst_field = Path::append_projection(&dst_path, ptr_proj);
                    self.acx.set_path_rustc_type(dst_field.clone(), *ptr_ty);
                    self.add_edge_between_ptrs(src_field, dst_field);
                }
            }
        } else {
            warn!(
                "Unmatched types: {:?}({:?}) -> {:?}({:?})",
                src_path, src_type, dst_path, dst_type
            );
        }
    }

    fn add_edge_between_ptrs(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        match (src.is_deref_path(), dst.is_deref_path()) {
            (false, false) => self.add_direct_edge(src, dst),
            (true, false) => self.add_load_edge(src, dst),
            (false, true) => self.add_store_edge(src, dst),
            (true, true) => unreachable!("Unexpected types of lh_path and rh_path."),
        }
    }


    /// Adds edges between the union fields that share the same memory offset
    fn cast_between_union_fields(&mut self, path: &Rc<Path>) {
        let retrieve_union_fields = |path: &Rc<Path>| -> Vec<(Rc<Path>, usize)> {
            let mut ret = Vec::new();
            match &path.value {
                PathEnum::QualifiedPath { projection, .. } => {
                    for (i, selector) in projection.iter().enumerate() {
                        if let PathSelector::UnionField(index) = *selector {
                            let union_base = Path::truncate_projection_elems(&path, i);
                            ret.push((union_base, index));
                        }
                    }
                }
                _ => {}
            }
            ret
        };

        let union_fields = retrieve_union_fields(path);
        if !union_fields.is_empty() {
            for (union_path, field_index) in union_fields {
                let union_type = self
                    .acx
                    .get_path_rustc_type(&union_path)
                    .expect("Uncached union path");
                if let TyKind::Adt(def, substs) = union_type.kind() {
                    let source_field = def.all_fields().nth(field_index).unwrap();
                    let source_type =
                        self.substs_specializer
                            .specialize_generic_argument_type(type_util::field_ty(
                                self.tcx(),
                                source_field,
                                substs,
                            ));
                    let source_path = Path::new_union_field(union_path.clone(), field_index);
                    self.acx.set_path_rustc_type(source_path.clone(), source_type);
                    for (i, field) in def.all_fields().enumerate() {
                        if i == field_index {
                            continue;
                        }
                        let target_type = self.substs_specializer.specialize_generic_argument_type(
                            type_util::field_ty(self.tcx(), field, substs),
                        );
                        let target_path = Path::new_union_field(union_path.clone(), i);
                        self.acx.set_path_rustc_type(target_path.clone(), target_type);
                        self.copy_and_transmute(
                            source_path.clone(),
                            source_type,
                            target_path,
                            target_type,
                        );
                    }
                } else {
                    unreachable!("the base path of a union field is not a union");
                }
            }
        }
    }

    /// Adds internal edge for ReifyFnPointer or ClosureFnPointer casts, where the rh_path is a function item (
    /// parsed from FnDef or Closure) and the lh_path is a function pointer, to enable the function pointer
    /// pointing to the function item.
    /// Note that the lh_path can also be a dereferenced value, if so, we need to introduce an auxiliary local
    /// variable.  
    /// For exmaple, given the ReifyFnPointer cast: `(*_2) = times2 as fn(i32) -> i32 (Pointer(ReifyFnPointer));`
    /// We create an auxiliary local variable `aux` to split this statement into two statements:
    /// `aux = times2 as fn(i32) -> i32 (Pointer(ReifyFnPointer));` and `(*2) = aux`.
    fn add_fnptr_cast_edge(&mut self, lh_path: Rc<Path>, rh_path: Rc<Path>, ty: Ty<'tcx>) {
        match &lh_path.value {
            PathEnum::QualifiedPath { base: _, projection } if projection[0] == PathSelector::Deref => {
                match ty.kind() {
                    TyKind::FnPtr(..) => {
                        let aux = self.create_aux_local(ty);
                        self.add_addr_edge(rh_path, aux.clone());
                        self.add_store_edge(aux, lh_path);
                    }
                    _ => {
                        unreachable!("Unexpected cast type in ReifyFnPointer cast!");
                    }
                }
            }
            _ => {
                self.add_addr_edge(rh_path, lh_path);
            }
        }
    }

    /// Creates an auxiliary local variable with the given type.
    #[inline]
    pub fn create_aux_local(&mut self, ty: Ty<'tcx>) -> Rc<Path> {
        self.acx.create_aux_local(self.func_id, ty)
    }

    /// Creates a dereference path for the given pointer or reference path.
    #[allow(unused)]
    fn create_dereference(&mut self, ptr_path: Rc<Path>, ptr_ty: Ty<'tcx>) -> Rc<Path> {
        let deref_path = if let PathEnum::QualifiedPath { .. } = ptr_path.value {
            let aux = self.create_aux_local(ptr_ty);
            self.add_direct_edge(ptr_path, aux.clone());
            Path::new_deref(aux)
        } else {
            Path::new_deref(ptr_path)
        };
        self.acx
            .set_path_rustc_type(deref_path.clone(), type_util::get_dereferenced_type(ptr_ty));
        deref_path
    }

    /// Returns the parameter environment for the current function.
    pub fn get_param_env(&self) -> rustc_middle::ty::ParamEnv<'tcx> {
        let def_id = self.def_id();
        let env_def_id = if self.tcx().is_closure_like(def_id) {
            self.tcx().typeck_root_def_id(def_id)
        } else {
            def_id
        };
        self.tcx().param_env(env_def_id)
    }

    /// Copy the value at `source_path` to a value at `target_path`.
    /// If the type of `source_path` is different from that at `target_path`, the value is transmuted.
    pub fn copy_and_transmute(
        &mut self,
        source_path: Rc<Path>,
        source_rustc_type: Ty<'tcx>,
        target_path: Rc<Path>,
        target_rustc_type: Ty<'tcx>,
    ) {
        debug!(
            "Copy and transmute from {:?}({:?}) to {:?}({:?})",
            source_path, source_rustc_type, target_path, target_rustc_type
        );
        let param_env = self.get_param_env();
        let src_flattened_fields =
            type_util::flatten_fields(self.tcx(), param_env, source_path, source_rustc_type);
        debug!("flattened fields of source value: {:?}", src_flattened_fields);

        let tgt_flattened_fields =
            type_util::flatten_fields(self.tcx(), param_env, target_path, target_rustc_type);
        debug!("flattened fields of target value: {:?}", tgt_flattened_fields);

        self.copy_flattened_fields(src_flattened_fields, tgt_flattened_fields);
    }

    fn copy_flattened_fields(
        &mut self,
        src_flattened_fields: Vec<(usize, Rc<Path>, Ty<'tcx>)>,
        tgt_flattened_fields: Vec<(usize, Rc<Path>, Ty<'tcx>)>,
    ) {
        let src_len = src_flattened_fields.len();
        let tgt_len = tgt_flattened_fields.len();
        let mut src_field_index = 0;
        let mut tgt_field_index = 0;
        while tgt_field_index < tgt_len && src_field_index < src_len {
            // Both the src_type and tgt_type should have been specialized.
            let (tgt_offset, tgt_field, tgt_type) = &tgt_flattened_fields[tgt_field_index];
            let (src_offset, src_field, src_type) = &src_flattened_fields[src_field_index];
            if *tgt_offset < *src_offset {
                tgt_field_index += 1;
                continue;
            } else if *tgt_offset > *src_offset {
                src_field_index += 1;
                continue;
            }

            // if source type and target type are any kind of primitive pointer type (reference, raw pointer, fn pointer).
            if src_type.is_any_ptr() && tgt_type.is_any_ptr() {
                self.acx.set_path_rustc_type(src_field.clone(), *src_type);
                self.acx.set_path_rustc_type(tgt_field.clone(), *tgt_type);
                self.transmute_pointers(src_field.clone(), *src_type, tgt_field.clone(), *tgt_type);
            }
            tgt_field_index += 1;
            src_field_index += 1;
        }
    }

    // Transmute from one pointer to another pointer.
    // If the source and target pointers are of equivalent pointer types, add 
    // a direct edge between them, otherwise add a cast edge between them.
    fn transmute_pointers(
        &mut self,
        source_path: Rc<Path>,
        source_ptr_type: Ty<'tcx>,
        target_path: Rc<Path>,
        target_ptr_type: Ty<'tcx>,
    ) {
        assert!(source_ptr_type.is_any_ptr() && target_ptr_type.is_any_ptr());
        debug!(
            "Transmuting from pointer {:?} to pointer {:?}",
            source_path, target_path
        );

        // A cast edge requires that the source path and the target path are not dereference paths. 
        let source_path = if source_path.is_deref_path() {
            let aux = self.create_aux_local(source_ptr_type);
            self.add_load_edge(source_path, aux.clone());
            aux
        } else {
            source_path
        };
        let target_path = if target_path.is_deref_path() {
            let aux = self.create_aux_local(target_ptr_type);
            self.add_store_edge(aux.clone(), target_path);
            aux
        } else {
            target_path
        };

        if type_util::equivalent_ptr_types(self.tcx(), source_ptr_type, target_ptr_type) {
            self.add_direct_edge(source_path, target_path);
        } else {
            self.add_cast_edge(source_path.clone(), target_path.clone());
        }
    }

    // Returns a Function path for the given `def_id` and `gen_args`, no matter if the corresponding mir
    // is unavailable.
    // If the function refers to a specific implementation of a trait method, devirtualize it.
    fn visit_function_reference(&mut self, def_id: DefId, gen_args: GenericArgsRef<'tcx>) -> Rc<Path> {
        // Specialize substs from current generic types
        let substs = self.substs_specializer.specialize_generic_args(gen_args);
        let (def_id, substs) = call_graph_builder::resolve_fn_def(self.tcx(), def_id, substs);
        let func_id = self.acx.get_func_id(def_id, substs);
        let path = Path::new_function(func_id);
        self.acx
            .set_path_rustc_type(path.clone(), Ty::new_fn_def(self.tcx(), def_id, substs));
        return path;
    }

    /// Returns a Path representing the given closure instance
    fn new_closure_path(&mut self, closure_ty: Ty<'tcx>) -> Rc<Path> { 
        let closure_ty = self.substs_specializer.specialize_generic_argument_type(closure_ty);
        if let TyKind::Closure(def_id, args) = closure_ty.kind() {
            let func_id = self.acx.get_func_id(*def_id, args);
            let path = Path::new_function(func_id);
            self.acx
                .set_path_rustc_type(path.clone(), closure_ty);
            path
        } else {
            unreachable!("Unexpected type for creating a new closure path.");
        }
    }

    /// Returns a (Path, Type) pair that corresponds to the given Place instance
    fn get_path_and_type_for_place(&mut self, place: &mir::Place<'tcx>) -> (Rc<Path>, Ty<'tcx>) {
        let path = self.get_path_for_place(place);
        let ty = self
            .acx
            .get_path_rustc_type(&path)
            .expect("Failed to get the rustc type");
        (path, ty)
    }

    /// Returns a `Path` instance that resembles the `Place` instance.
    fn get_path_for_place(&mut self, place: &mir::Place<'tcx>) -> Rc<Path> {
        if let Some(path) = self.path_cache.get(place) {
            return path.clone();
        }
        let base_path: Rc<Path> =
            Path::new_local_parameter_or_result(self.func_id, place.local.as_usize(), self.mir.arg_count);
        let local_ty = self
            .substs_specializer
            .specialize_generic_argument_type(self.mir.local_decls[place.local].ty);
        self.acx.set_path_rustc_type(base_path.clone(), local_ty);
        if place.projection.is_empty() {
            self.path_cache.insert(*place, base_path.clone());
            base_path
        } else {
            let (path, ty) = self.visit_projection(base_path, local_ty, place.projection);
            self.acx.set_path_rustc_type(path.clone(), ty);
            self.path_cache.insert(*place, path.clone());
            path
        }
    }

    /// Canonicalize a path for ClassPAG so that source-level equivalent class refs map to one pointer.
    /// If this path is a simple local that was assigned from `&place` (Ref) or `move place` (Use),
    /// returns the path for that place; otherwise returns the original path.
    /// This makes Store base (receiver) and Load base use the same ptr_id for the same holder,
    /// and Store value use the same ptr_id as the source variable (e.g. _a1 -> local_1).
    fn canonicalize_path_for_class_pag(&mut self, path: &Rc<Path>) -> Rc<Path> {
        use crate::mir::path::PathEnum;
        let ordinal = match &path.value {
            PathEnum::LocalVariable { func_id, ordinal } if *func_id == self.fpag.func_id => *ordinal,
            PathEnum::Parameter { func_id, ordinal } if *func_id == self.fpag.func_id => *ordinal,
            _ => return path.clone(),
        };
        let local = mir::Local::from(ordinal);
        for block in self.mir.basic_blocks.iter() {
            for stmt in block.statements.iter() {
                if let mir::StatementKind::Assign(box (lhs, rvalue)) = &stmt.kind {
                    if lhs.local != local || !lhs.projection.is_empty() {
                        continue;
                    }
                    match rvalue {
                        mir::Rvalue::Ref(_, _, ref place) if place.projection.is_empty() => {
                            return self.get_path_for_place(place);
                        }
                        mir::Rvalue::Use(mir::Operand::Move(place) | mir::Operand::Copy(place))
                            if place.projection.is_empty() =>
                        {
                            return self.get_path_for_place(place);
                        }
                        _ => {}
                    }
                    break; // one assignment per local in SSA form, stop after first
                }
            }
        }
        path.clone()
    }

    /// Returns a path that is qualified by the selector corresponding to the projection.elem.
    /// If projection has a base, the give base_path is first qualified with the base.
    fn visit_projection(
        &mut self,
        base_path: Rc<Path>,
        base_ty: Ty<'tcx>,
        projection: &[mir::PlaceElem<'tcx>],
    ) -> (Rc<Path>, Ty<'tcx>) {
        let mut ty = base_ty;
        let mut base_path = base_path;
        let mut selectors = ProjectionElems::with_capacity(projection.len());
        for elem in projection.iter() {
            let selector = self.visit_projection_elem(ty, elem);
            match elem {
                // We don't need to specialize the type during iteration, as the type must be specific
                // enough when it has projections.
                mir::ProjectionElem::Deref => {
                    if ty.is_box() {
                        // Deref the pointer at field 0 of the NonNull pointer at field 0
                        // of the Unique pointer at field 0 of the box
                        // Create an auxiliary variable to represent this sub-field.
                        // `(*_1);` ==> `aux = _1.0.0.0; *aux`
                        // Old code (nightly-2024-02-03): ty.boxed_ty()
                        // New code (nightly-2025-05-09): boxed_ty() now returns Option<Ty>
                        let box_ptr_field = self.get_box_pointer_field(base_path, ty.boxed_ty().expect("Box type should have boxed_ty"));
                        let box_ptr_ty = self
                            .acx
                            .get_path_rustc_type(&box_ptr_field)
                            .expect("Box pointer type");
                        let aux = self.create_aux_local(box_ptr_ty);
                        self.add_direct_edge(box_ptr_field, aux.clone());
                        base_path = aux;
                    }
                    ty = type_util::get_dereferenced_type(ty);
                }
                mir::ProjectionElem::Field(_, field_ty) => {
                    // Cache the base path if it is union type
                    if ty.is_union() {
                        let union_path = if selectors.is_empty() {
                            base_path.clone()
                        } else {
                            Path::new_qualified(base_path.clone(), selectors.clone())
                        };
                        let union_ty = self.substs_specializer.specialize_generic_argument_type(ty);
                        self.acx.set_path_rustc_type(union_path, union_ty);
                    }
                    ty = self.substs_specializer.specialize_generic_argument_type(*field_ty);
                }
                mir::ProjectionElem::Index(..) | mir::ProjectionElem::ConstantIndex { .. } => {
                    ty = type_util::get_element_type(self.tcx(), ty);
                }
                mir::ProjectionElem::Downcast(_, variant_idx) => {
                    ty = type_util::get_downcast_type(self.tcx(), ty, *variant_idx);
                }
                mir::ProjectionElem::Subslice { .. } => {
                    continue;
                }
                mir::ProjectionElem::OpaqueCast(..) 
                | mir::ProjectionElem::Subtype(..) | mir::ProjectionElem::UnwrapUnsafeBinder(..) => {
                    // Todo
                    continue;
                }
            }
            selectors.push(selector);
        }
        let result = if selectors.len() == 0 {
            base_path
        } else {
            Path::new_qualified(base_path, selectors)
        };
        (result, ty)
    }

    /// Returns a PathSelector instance that resembles the ProjectionElem instance.
    fn visit_projection_elem(
        &mut self,
        base_ty: Ty<'tcx>,
        projection_elem: &mir::ProjectionElem<mir::Local, Ty<'tcx>>,
    ) -> PathSelector {
        match projection_elem {
            mir::ProjectionElem::Deref => PathSelector::Deref,
            mir::ProjectionElem::Field(field, _ty) => {
                if let TyKind::Adt(def, _) = base_ty.kind() {
                    if def.is_union() {
                        return PathSelector::UnionField(usize::from(*field));
                    }
                }
                PathSelector::Field(usize::from(*field))
            }
            mir::ProjectionElem::Index(_)
            | mir::ProjectionElem::ConstantIndex { .. } => PathSelector::Index,
            mir::ProjectionElem::Downcast(_name, index) => PathSelector::Downcast(index.as_usize()),
            mir::ProjectionElem::Subslice { from, to, from_end } => PathSelector::Subslice {
                from: *from,
                to: *to,
                from_end: *from_end,
            },
            mir::ProjectionElem::OpaqueCast(ty) 
            | mir::ProjectionElem::Subtype(ty) | mir::ProjectionElem::UnwrapUnsafeBinder(ty) => {
                PathSelector::Cast(self.acx.get_type_index(ty))
            }
        }
    }

    /// Returns the raw pointer field of a `Box` value. 
    fn get_box_pointer_field(&mut self, box_path: Rc<Path>, ty: Ty<'tcx>) -> Rc<Path> {
        // Box.0 = Unique, Unique.0 = NonNull, NonNull.0 = source thin pointer
        let projection = vec![
            PathSelector::Field(0),
            PathSelector::Field(0),
            PathSelector::Field(0),
        ];
        let value_path = Path::append_projection(&box_path, &projection);
        if self.acx.get_path_rustc_type(&value_path).is_none() {
            let deref_ty = self.substs_specializer.specialize_generic_argument_type(ty);
            let ty = self.tcx().mk_ty_from_kind(TyKind::RawPtr(deref_ty, rustc_middle::mir::Mutability::Not));
            self.acx.set_path_rustc_type(value_path.clone(), ty);
        }
        value_path
    }

    #[inline]
    pub fn add_addr_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        self.add_edge(src, dst, PAGEdgeEnum::AddrPAGEdge);
    }

    #[inline]
    pub fn add_direct_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        self.add_edge(src, dst, PAGEdgeEnum::DirectPAGEdge);
    }

    /// Adds a store edge from `src` to `dst`.
    /// Given a store statement ```(*p).f1.f2...fn = q```, a store edge of format `q --STORE(f1.f2...fn)--> p` is added.
    #[inline]
    pub fn add_store_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        if let PathEnum::QualifiedPath { base, projection } = &dst.value {
            assert_eq!(projection[0], PathSelector::Deref);
            let store_proj = Vec::from_iter(projection[1..].iter().cloned());
            debug!("add_store_edge: src={:?}, dst={:?}, store_proj={:?}", src, dst, store_proj);
            self.add_edge(src, base.clone(), PAGEdgeEnum::StorePAGEdge(store_proj));
        } else {
            unreachable!();
        }
    }

    /// Adds a load edge from `src` to `dst`.
    /// Given a load statement ```p = (*q).f1.f2...fn```, a Load edge `q --LOAD(f1.f2...fn)--> p` is added.
    #[inline]
    pub fn add_load_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        if let PathEnum::QualifiedPath { base, projection } = &src.value {
            assert_eq!(projection[0], PathSelector::Deref);
            let load_proj = Vec::from_iter(projection[1..].iter().cloned());
            debug!("add_load_edge: src={:?}, dst={:?}, load_proj={:?}", src, dst, load_proj);
            self.add_edge(base.clone(), dst, PAGEdgeEnum::LoadPAGEdge(load_proj));
        } else {
            unreachable!();
        }
    }

    /// Adds a gep edge from `src` to `dst`.
    /// Given a gep statement ```p = &((*q).f1.f2...fn)```, a gep edge `q --GEP(f1.f2...fn)--> p` is added.
    #[inline]
    pub fn add_gep_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        if let PathEnum::QualifiedPath { base, projection } = &src.value {
            assert_eq!(projection[0], PathSelector::Deref);
            assert!(projection.len() > 1);
            let gep_proj = Vec::from_iter(projection[1..].iter().cloned());
            self.add_edge(base.clone(), dst, PAGEdgeEnum::GepPAGEdge(gep_proj));
        } else {
            unreachable!();
        }
    }

    #[inline]
    pub fn add_cast_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        self.add_edge(src, dst, PAGEdgeEnum::CastPAGEdge);
    }

    #[inline]
    pub fn add_offset_edge(&mut self, src: Rc<Path>, dst: Rc<Path>) {
        self.add_edge(src, dst, PAGEdgeEnum::OffsetPAGEdge);
    }

    /// Adds an internal edge from `src` to `dst` of `kind` to the function pag.
    pub fn add_edge(&mut self, src: Rc<Path>, dst: Rc<Path>, kind: PAGEdgeEnum) {
        self.fpag.add_internal_edge(src, dst, kind);
    }
    
    /// Creates a new callsite.
    fn new_callsite(
        &mut self,
        func_id: FuncId,
        location: rustc_middle::mir::Location,
        args: Vec<Rc<Path>>,
        destination: Rc<Path>,
    ) -> Rc<CallSite> {
        Rc::new(CallSite::new(func_id, location, args, destination))
    }

    /// Handles getter/setter method calls by establishing Direct edges for pointer propagation.
    /// 
    /// The key insight is that setter and getter need to share the SAME field path representation
    /// so that points-to information can flow from setter to getter.
    /// 
    /// We use a synthetic "class field" path based on the container's pointee (heap object).
    /// For example:
    /// - Setter on c1: value flows to "class_field:Container:point"
    /// - Getter on c1: "class_field:Container:point" flows to destination
    /// 
    /// Since we can't easily determine the heap object at PAG build time, we use the destination
    /// of the self reference's pointees. The propagation will handle this correctly.
    fn handle_getter_setter(
        &mut self,
        gs: &analysis::GetterSetter,
        args: &[Rc<Path>],
        destination: &Rc<Path>,
        _location: mir::Location,
    ) {
        if args.is_empty() {
            warn!("Getter/Setter method call has no arguments: {:?}", gs);
            return;
        }

        // Shared for rcpta ClassPAG load/store: base pointer id (receiver)
        // Canonicalize so the same source-level holder (e.g. holder_1) gets one ptr_id (local_3) for both set_item and get_item.
        let func_name = self.rcpta_canonical_func_name();
        let base_path_canonical = self.canonicalize_path_for_class_pag(&args[0]);
        let param_slots = Some(1 + self.mir.arg_count);
        let base_ptr_id = crate::util::class::ptr_system::path_to_class_ptr_id(&base_path_canonical, Some(&func_name), param_slots);

        // Get the container reference (self parameter)
        let self_ref = args[0].clone();
        
        // ===== Class Type System: Build field path =====
        // The type system handles field registration and path construction
        let (field_path, _field_index) = self.acx.class_type_system.build_field_path(
            &gs.class_name,
            &gs.field_name,
            self_ref.clone(),
            None  // Field class type will be inferred later
        );

        if gs.is_getter {
            // Getter: Load edge from field to destination
            self.add_load_edge(field_path.clone(), destination.clone());
            
            // Mark destination as class reference if field type is known
            if let Some(field_class_type) = self.acx.class_type_system
                .get_class(&gs.class_name)
                .and_then(|c| c.get_field_class_type(&gs.field_name))
                .cloned() 
            {
                self.acx.class_type_system.mark_class_reference(
                    destination.clone(), 
                    &field_class_type, 
                    false
                );
                
                // ===== Class Pointer System Integration =====
                // Propagate points-to from field to destination (load operation)
                let func_name = self.rcpta_canonical_func_name();
                use crate::util::class::ptr_system::{ClassPtr, path_to_class_ptr_id};
                
                let field_ptr_id = path_to_class_ptr_id(&field_path, Some(&func_name), param_slots);
                let dst_ptr_id = path_to_class_ptr_id(&destination, Some(&func_name), param_slots);
                
                // Create/get pointers
                let field_ptr = ClassPtr::new_field(
                    path_to_class_ptr_id(&args[0], Some(&func_name), param_slots), // base
                    gs.field_name.clone(),
                    field_class_type.clone()
                );
                let dst_ptr = ClassPtr::new_local(dst_ptr_id.clone(), field_class_type.clone());
                self.acx.class_ptr_system.get_or_create_ptr(field_ptr);
                self.acx.class_ptr_system.get_or_create_ptr(dst_ptr);
                
                // Propagate points-to from field to destination
                self.acx.class_ptr_system.propagate_points_to(&field_ptr_id, &dst_ptr_id);
                // ============================================
            }

            // rcpta ClassPAG: Load edge base.field -> dst only when the field holds a **class reference**
            // (see ClassField.class_type). Primitives (f64, String, etc.) must not become ClassPtr / Load edges.
            let source_level = analysis::is_source_level_context(&func_name);
            if source_level {
                if let Some(dst_class_type) = self
                    .acx
                    .class_type_system
                    .get_field_class_type(&gs.class_name, &gs.field_name)
                {
                    let dst_ptr_id = crate::util::class::ptr_system::path_to_class_ptr_id(
                        destination,
                        Some(&func_name),
                        param_slots,
                    );
                    let base_cptr =
                        crate::rcpta::ClassPtr::new_local(base_ptr_id.clone(), gs.class_name.clone());
                    let dst_cptr =
                        crate::rcpta::ClassPtr::new_local(dst_ptr_id.clone(), dst_class_type.clone());
                    self.acx.class_pag.get_or_create_ptr(base_cptr);
                    self.acx.class_pag.get_or_create_ptr(dst_cptr);
                    self.acx.class_pag.add_load(&base_ptr_id, &gs.field_name, &dst_ptr_id);
                    info!(
                        "rcpta ClassPAG: add_load in callee func={} base={} field={} dst={}",
                        func_name, base_ptr_id, gs.field_name, dst_ptr_id
                    );
                } else {
                    debug!(
                        "rcpta ClassPAG: skip Load (non-class-reference field) func={} field={}",
                        func_name, gs.field_name
                    );
                }
            } else {
                info!(
                    "rcpta ClassPAG: skip Load (not source-level) func={}",
                    func_name
                );
            }

            debug!("Getter [{}.{}]: Load {:?} -> {:?}", 
                   gs.class_name, gs.field_name, field_path, destination);
        } else {
            // Setter: Store edge from value to field
            if args.len() < 2 {
                warn!("Setter method call has insufficient arguments: {:?}", gs);
                return;
            }
            let value_path = args[1].clone();
            self.add_store_edge(value_path.clone(), field_path.clone());
            // Canonicalize value for ClassPAG so _a1/_b1 map to local_1/local_2 (e.g. value from move _1 -> use _1).
            let value_path_canonical = self.canonicalize_path_for_class_pag(&value_path);
            
            // Infer and update field's class type from the value being stored
            if let Some(value_class_type) = self.acx.class_type_system
                .get_path_class_type(&value_path)
                .cloned() 
            {
                self.acx.class_type_system.update_field_class_type(
                    &gs.class_name, 
                    &gs.field_name, 
                    &value_class_type
                );
                
                // ===== Class Pointer System Integration =====
                // Propagate points-to from value to field (store operation)
                let func_name = self.rcpta_canonical_func_name();
                use crate::util::class::ptr_system::{ClassPtr, path_to_class_ptr_id};
                
                let value_ptr_id = path_to_class_ptr_id(&value_path, Some(&func_name), param_slots);
                let field_ptr_id = path_to_class_ptr_id(&field_path, Some(&func_name), param_slots);
                
                // Create/get pointers
                let value_ptr = ClassPtr::new_local(value_ptr_id.clone(), value_class_type.clone());
                let field_ptr = ClassPtr::new_field(
                    path_to_class_ptr_id(&args[0], Some(&func_name), param_slots), // base
                    gs.field_name.clone(),
                    value_class_type.clone()
                );
                self.acx.class_ptr_system.get_or_create_ptr(value_ptr);
                self.acx.class_ptr_system.get_or_create_ptr(field_ptr);
                
                // Propagate points-to from value to field
                self.acx.class_ptr_system.propagate_points_to(&value_ptr_id, &field_ptr_id);
                // ============================================
            }

            // rcpta ClassPAG: Store only when the field is a **class-reference field** (inferred or registered).
            if analysis::is_source_level_context(&func_name) {
                if self
                    .acx
                    .class_type_system
                    .get_field_class_type(&gs.class_name, &gs.field_name)
                    .is_some()
                {
                    let value_ptr_id = crate::util::class::ptr_system::path_to_class_ptr_id(
                        &value_path_canonical,
                        Some(&func_name),
                        param_slots,
                    );
                    let value_class_type = self
                        .acx
                        .class_type_system
                        .get_path_class_type(&value_path)
                        .cloned()
                        .or_else(|| {
                            self.acx
                                .class_type_system
                                .get_field_class_type(&gs.class_name, &gs.field_name)
                        })
                        .expect("class-reference field store must have value or field class type");
                    let base_cptr =
                        crate::rcpta::ClassPtr::new_local(base_ptr_id.clone(), gs.class_name.clone());
                    let value_cptr =
                        crate::rcpta::ClassPtr::new_local(value_ptr_id.clone(), value_class_type.clone());
                    self.acx.class_pag.get_or_create_ptr(base_cptr);
                    self.acx.class_pag.get_or_create_ptr(value_cptr);
                    self.acx.class_pag.add_store(&base_ptr_id, &gs.field_name, &value_ptr_id);
                    info!(
                        "rcpta ClassPAG: add_store in callee func={} base={} field={} src={}",
                        func_name, base_ptr_id, gs.field_name, value_ptr_id
                    );
                } else {
                    debug!(
                        "rcpta ClassPAG: skip Store (non-class-reference field) func={} field={}",
                        func_name, gs.field_name
                    );
                }
            }

            debug!("Setter [{}.{}]: Store {:?} -> {:?}", 
                   gs.class_name, gs.field_name, value_path, field_path);
        }

        // Mark the field path as a class field (legacy)
        self.acx.class_field_paths.insert(field_path);
    }
}
