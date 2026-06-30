// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Provides special handling for a set of functions.

use lazy_static::lazy_static;
use log::*;
use std::collections::HashSet;
use std::rc::Rc;

use rustc_hir::def_id::DefId;
use rustc_hir::lang_items::LangItem;
use rustc_middle::mir;
use rustc_middle::ty::{GenericArgsRef, List, Ty, TyCtxt, TyKind};
use rustc_span::Pos;

use crate::builder::fpag_builder::FuncPAGBuilder;
use crate::graph::pag::PAGPath;
use crate::mir::analysis_context::AnalysisContext;
use crate::mir::known_names::KnownNames;
use crate::mir::path::{Path, PathEnum, PathSelector};
use crate::util::type_util;
use crate::util::class::analysis as class_analysis;

lazy_static! {
    static ref SPECIALLY_HANDLED_FUNCTIONS: HashSet<KnownNames> = {
        let mut set = HashSet::new();
        set.insert(KnownNames::StdIntrinsicsTransmute);
        set.insert(KnownNames::StdIntrinsicsOffset);
        set.insert(KnownNames::StdIntrinsicsArithOffset);
        set.insert(KnownNames::StdPtrConstPtrCast);
        set.insert(KnownNames::StdPtrConstPtrAdd);
        set.insert(KnownNames::StdPtrConstPtrSub);
        set.insert(KnownNames::StdPtrConstPtrOffset);
        set.insert(KnownNames::StdPtrConstPtrByteAdd);
        set.insert(KnownNames::StdPtrConstPtrByteSub);
        set.insert(KnownNames::StdPtrConstPtrByteOffset);
        set.insert(KnownNames::StdPtrConstPtrWrappingAdd);
        set.insert(KnownNames::StdPtrConstPtrWrappingSub);
        set.insert(KnownNames::StdPtrConstPtrWrappingOffset);
        set.insert(KnownNames::StdPtrConstPtrWrappingByteAdd);
        set.insert(KnownNames::StdPtrConstPtrWrappingByteSub);
        set.insert(KnownNames::StdPtrConstPtrWrappingByteOffset);
        set.insert(KnownNames::StdPtrMutPtrCast);
        set.insert(KnownNames::StdPtrMutPtrAdd);
        set.insert(KnownNames::StdPtrMutPtrSub);
        set.insert(KnownNames::StdPtrMutPtrOffset);
        set.insert(KnownNames::StdPtrMutPtrByteAdd);
        set.insert(KnownNames::StdPtrMutPtrByteSub);
        set.insert(KnownNames::StdPtrMutPtrByteOffset);
        set.insert(KnownNames::StdPtrMutPtrWrappingAdd);
        set.insert(KnownNames::StdPtrMutPtrWrappingSub);
        set.insert(KnownNames::StdPtrMutPtrWrappingOffset);
        set.insert(KnownNames::StdPtrMutPtrWrappingByteAdd);
        set.insert(KnownNames::StdPtrMutPtrWrappingByteSub);
        set.insert(KnownNames::StdPtrMutPtrWrappingByteOffset);
        set.insert(KnownNames::AllocRawVecAllocateIn);
        set.insert(KnownNames::StdThreadBuilderSpawnUnchecked);
        set.insert(KnownNames::StdPtrNonNullAsPtr);
        set.insert(KnownNames::StdPtrUniqueNewUnchecked);
        set.insert(KnownNames::StdResultMapErr);
        set.insert(KnownNames::RustAlloc);
        set.insert(KnownNames::RustAllocZeroed);
        set.insert(KnownNames::StdAllocAlloc);
        set.insert(KnownNames::StdAllocAllocZeroed);
        set.insert(KnownNames::StdAllocExchangeMalloc);
        set.insert(KnownNames::StdAllocAllocatorAllocate);
        set.insert(KnownNames::StdAllocAllocatorAllocateZeroed);
        set.insert(KnownNames::RustRealloc);
        set.insert(KnownNames::StdAllocRealloc);
        set.insert(KnownNames::StdAllocAllocatorGrow);
        set.insert(KnownNames::StdAllocAllocatorGrowZeroed);
        set.insert(KnownNames::StdAllocAllocatorShrink);
        set.insert(KnownNames::RustDealloc);
        set.insert(KnownNames::RustAllocErrorHandler);
        set.insert(KnownNames::StdAllocDealloc);
        set.insert(KnownNames::StdAllocBoxFree);
        set.insert(KnownNames::StdAllocHandleAllocError);
        set.insert(KnownNames::StdAllocAllocatorDeallocate);
        set
    };
}

/// Returns true if the function with `def_id` is specially handled.
pub fn is_specially_handled_function(acx: &mut AnalysisContext, def_id: DefId) -> bool {
    let known_name = acx.get_known_name_for(def_id);
    SPECIALLY_HANDLED_FUNCTIONS.contains(&known_name)
}

/// Handling calls to special functions.
///
/// Returns true if this callee function is handled as a special function.
/// If the return result is false, we need to continue with the normal logic.
pub fn handled_as_special_function_call<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    callee_def_id: &DefId,
    gen_args: &GenericArgsRef<'tcx>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
    location: mir::Location,
) -> bool {
    let callee_known_name = fpb.acx.get_known_name_for(*callee_def_id);
    match callee_known_name {
        KnownNames::StdIntrinsicsTransmute => {
            handle_transmute(fpb, gen_args, args, destination);
            return true;
        }
        KnownNames::StdIntrinsicsOffset
        | KnownNames::StdIntrinsicsArithOffset
        | KnownNames::StdPtrConstPtrAdd
        | KnownNames::StdPtrConstPtrSub
        | KnownNames::StdPtrConstPtrOffset
        | KnownNames::StdPtrConstPtrByteAdd
        | KnownNames::StdPtrConstPtrByteSub
        | KnownNames::StdPtrConstPtrByteOffset
        | KnownNames::StdPtrConstPtrWrappingAdd
        | KnownNames::StdPtrConstPtrWrappingSub
        | KnownNames::StdPtrConstPtrWrappingOffset
        | KnownNames::StdPtrConstPtrWrappingByteAdd
        | KnownNames::StdPtrConstPtrWrappingByteSub
        | KnownNames::StdPtrConstPtrWrappingByteOffset
        | KnownNames::StdPtrMutPtrAdd
        | KnownNames::StdPtrMutPtrSub
        | KnownNames::StdPtrMutPtrOffset
        | KnownNames::StdPtrMutPtrByteAdd
        | KnownNames::StdPtrMutPtrByteSub
        | KnownNames::StdPtrMutPtrByteOffset
        | KnownNames::StdPtrMutPtrWrappingAdd
        | KnownNames::StdPtrMutPtrWrappingSub
        | KnownNames::StdPtrMutPtrWrappingOffset
        | KnownNames::StdPtrMutPtrWrappingByteAdd
        | KnownNames::StdPtrMutPtrWrappingByteSub
        | KnownNames::StdPtrMutPtrWrappingByteOffset => {
            handle_offset(fpb, args, destination);
            return true;
        }
        KnownNames::StdPtrConstPtrCast | KnownNames::StdPtrMutPtrCast => {
            handle_ptr_cast(fpb, args, destination);
            return true;
        }
        KnownNames::AllocRawVecAllocateIn => {
            handle_raw_vec_allocate_in(fpb, gen_args, args, destination, location);
            return true;
        }
        KnownNames::StdThreadBuilderSpawnUnchecked => {
            handle_thread_builder_spawn_unchecked(fpb, gen_args, args, destination, location);
            return true;
        }
        KnownNames::StdPtrNonNullAsPtr => {
            handle_non_null_as_ptr(fpb, args, destination);
            return true;
        }
        KnownNames::StdPtrUniqueNewUnchecked => {
            handle_unique_new_unchecked(fpb, args, destination);
            return true;
        }
        KnownNames::StdResultMapErr => {
            handle_result_map_err(fpb, gen_args, args, destination);
            return true;
        }
        KnownNames::StdConvertInto => {
            let tcx = fpb.acx.tcx;
            let generic_types = gen_args.into_type_list(tcx);
            assert!(generic_types.len() >= 2);
            if is_std_ptr_unique(tcx, generic_types[0]) && is_std_ptr_nonnull(tcx, generic_types[1]) {
                handle_unique_into_nonnull(fpb, args, destination);
                return true;
            }
            return false;
        }
        _ => {
            // Check if this is a class constructor before handling as alloc
            // For class constructors, we create HeapObj and pointer relationships,
            // but we still need to build the call graph to analyze the constructor's body
            if is_class_constructor(&mut *fpb.acx, *callee_def_id, gen_args) {
                handle_class_constructor(fpb, callee_def_id, gen_args, destination, location);
                // Return false to continue with normal call graph building
                // This allows analysis of the constructor's internal implementation
                return false;
            }
            return handle_alloc(fpb, callee_known_name, args, destination, location);
        }
    }
}

/// Handles the call to the intrinsics `Transmute` function.
fn handle_transmute<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    gen_args: &GenericArgsRef<'tcx>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) {
    let source_path = args[0].clone();
    let source_rustc_type = gen_args.get(0).expect("rustc type error").expect_ty();
    let target_path = destination.clone();
    let target_rustc_type = fpb
        .acx
        .get_path_rustc_type(&target_path)
        .expect("rustc type error");
    fpb.copy_and_transmute(source_path, source_rustc_type, target_path, target_rustc_type);
}

/// Handles the call to the `offset` function, such as `std::ptr::mut_ptr::offset(_1: *mut T, _2: isize)`.
/// The offset function returns the address computed from the based address and the offset, and is commonly
/// used in vector's read/write operations.
fn handle_offset<'tcx>(fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>, args: &Vec<Rc<Path>>, destination: &Rc<Path>) {
    // Adds an offset edge from the source path to the destination path.
    let source_path = args[0].clone();
    fpb.add_offset_edge(source_path, destination.clone());
}

/// `core::ptr::const_ptr::cast()` and `core::ptr::mut_ptr::cast()`.
///
/// The cast functions significantly impacts the analysis precision and efficiency
/// when analyzed context-insensitively.
fn handle_ptr_cast<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) {
    // Adds a cast edge from the source path to the destination path.
    let source_path = args[0].clone();
    fpb.add_cast_edge(source_path, destination.clone());
}

/// ```fn allocate_in(capacity: usize, init: AllocInit, alloc: A) -> Self```.
/// ```RawVec<T, A: Allocator = Global> { ptr: Unique<T>, cap: usize, alloc: A, }```
fn handle_raw_vec_allocate_in<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    gen_args: &GenericArgsRef<'tcx>,
    _args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
    location: mir::Location,
) {
    let tcx = fpb.acx.tcx;
    let heap_object_path = Path::new_heap_obj(fpb.fpag.func_id, location);
    fpb.acx
        .set_path_rustc_type(heap_object_path.clone(), tcx.types.u8);

    let generic_type = gen_args.get(0).expect("rustc type error").expect_ty();
    fpb.acx
        .concretized_heap_objs
        .insert(heap_object_path.clone(), generic_type);
    let cast_heap_object_path = fpb
        .acx
        .cast_to(&heap_object_path, generic_type)
        .expect("Cast Error");

    // dst.0 = Unique, Unique.0 = NonNull, NonNull.0 = source thin pointer
    let projection = vec![
        PathSelector::Field(0),
        PathSelector::Field(0),
        PathSelector::Field(0),
    ];
    let dst_ptr_path = Path::new_qualified(destination.clone(), projection);
    let const_ptr_type = const_rawptr_type(tcx, generic_type);
    fpb.acx.set_path_rustc_type(dst_ptr_path.clone(), const_ptr_type);
    // Instead of inserting an address_of address from heap_object to dst_ptr_path,
    // we create a auxiliary path as an intermediary
    // ```let aux: *const T = &heap_object;  dst.0.0.0 = aux;```
    let aux = fpb.acx.create_aux_local(fpb.fpag.func_id, const_ptr_type);
    fpb.add_addr_edge(cast_heap_object_path, aux.clone());
    fpb.add_direct_edge(aux, dst_ptr_path);
}

/// ```fn spawn_unchecked<'a, F, T>(self, f: F) -> io::Result<JoinHandle<T>>```.
/// This function starts a new thread by calling external C function.
/// Instead of calling this function, we indirect the call to the thread closure f.
/// We can call `inline_indirectly_called_function` in fpb directly to resolve this call.
fn handle_thread_builder_spawn_unchecked<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    gen_args: &GenericArgsRef<'tcx>,
    args: &Vec<Rc<Path>>,
    _destination: &Rc<Path>,
    location: mir::Location,
) {
    let fn_once_defid = fpb.acx.tcx.require_lang_item(LangItem::FnOnce, None);
    let dst_ty = gen_args.get(1).expect("rustc type error").expect_ty();
    // FnOnce call requires two arguments, the first argument is the fn item that implements FnOnce trait,
    // and the second argument is the actual arguments list, an empty tuple in this case.
    let aux_arg = fpb.create_aux_local(fpb.acx.tcx.mk_ty_from_kind(TyKind::Tuple(List::empty())));
    let new_args = vec![args[1].clone(), aux_arg];
    let aux_dst = fpb.create_aux_local(dst_ty);
    let mut new_location = location;
    new_location.statement_index += 1;
    fpb.inline_indirectly_called_function(&fn_once_defid, gen_args, new_args, aux_dst, new_location);

    // Todo: Add edges from `aux_dst` to `destination`, to do so, we need to allocate a heap memory for the packet field.
    // Destination type: io::Result<JoinHandle<T>>, where struct JoinHandle<T>(JoinInner<'static, T>);
    // struct JoinInner<'scope, T> {
    //     native: imp::Thread,
    //     thread: Thread,
    //     packet: Arc<Packet<'scope, T>>,
    // }
    // struct Packet<'scope, T> {
    //     scope: Option<Arc<scoped::ScopeData>>,
    //     result: UnsafeCell<Option<Result<T>>>,
    //     _marker: PhantomData<Option<&'scope scoped::ScopeData>>,
    // }
}

fn handle_non_null_as_ptr<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) {
    // Adds an direct edge from the source path's first field to the destination path .
    let source_path = args[0].clone();
    let field_path = Path::new_field(source_path, 0);
    let ty = fpb.acx.get_path_rustc_type(destination).unwrap();
    fpb.acx.set_path_rustc_type(field_path.clone(), ty);
    fpb.add_direct_edge(field_path, destination.clone());
}

/// ```fn std::ptr::Unique::<T>::new_unchecked(_1: *mut T) -> std::ptr::Unique<T>```
fn handle_unique_new_unchecked<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) {
    // Adds an direct edge from args[0] to dst.0.0
    let dst_field_path = Path::new_qualified(
        destination.clone(),
        vec![PathSelector::Field(0), PathSelector::Field(0)],
    );
    fpb.add_direct_edge(args[0].clone(), dst_field_path);
}

/// ```fn std::result::Result::<T, E>::map_err(_1: std::result::Result<T, E>, _2: O)
///    -> std::result::Result<T, F>
/// ```
/// Handles as an assignment from `param_1.as_variant#0.0` to `ret.as_variant#0.0`.
fn handle_result_map_err<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    gen_args: &GenericArgsRef<'tcx>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) {
    assert!(!matches!(args[0].value, PathEnum::QualifiedPath { .. }));
    let source_path = Path::new_qualified(
        args[0].clone(),
        vec![PathSelector::Downcast(0), PathSelector::Field(0)],
    );
    let source_rustc_type = gen_args.get(0).expect("rustc type error").expect_ty();
    let target_path = Path::new_qualified(
        destination.clone(),
        vec![PathSelector::Downcast(0), PathSelector::Field(0)],
    );
    let target_rustc_type = source_rustc_type;
    fpb.add_internal_edges(source_path, source_rustc_type, target_path, target_rustc_type);
}

#[allow(unused)]
fn handle_slice_index_index<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
) -> bool {
    let slice_path = args[1].clone();
    let slice_ty = fpb
        .acx
        .get_path_rustc_type(&slice_path)
        .expect("rustc type error");
    let dst_ty = fpb
        .acx
        .get_path_rustc_type(destination)
        .expect("rustc type error");

    if slice_ty == dst_ty {
        fpb.add_internal_edges(slice_path, slice_ty, destination.clone(), dst_ty);
        return true;
    }
    return false;
}

fn handle_unique_into_nonnull(fpb: &mut FuncPAGBuilder, args: &Vec<Rc<Path>>, destination: &Rc<Path>) {
    assert!(!matches!(args[0].value, PathEnum::QualifiedPath { .. }));
    let source_path = Path::new_field(args[0].clone(), 0);
    let source_rustc_type = type_util::try_eval_path_type(fpb.acx, &source_path).unwrap();
    let target_rustc_type = fpb
        .acx
        .get_path_rustc_type(destination)
        .expect("rustc type error");
    info!(
        "Add edge from {:?}({:?}) to {:?}({:?})",
        source_path, source_rustc_type, destination, target_rustc_type
    );
    fpb.add_internal_edges(
        source_path,
        source_rustc_type,
        destination.clone(),
        target_rustc_type,
    );
}

fn handle_alloc<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    callee_known_name: KnownNames,
    args: &Vec<Rc<Path>>,
    destination: &Rc<Path>,
    location: mir::Location,
) -> bool {
    let tcx = fpb.acx.tcx;
    match callee_known_name {
        // Allocates memory on the heap and returns the address as `*mut u8`
        KnownNames::RustAlloc
        | KnownNames::RustAllocZeroed
        | KnownNames::StdAllocAlloc
        | KnownNames::StdAllocAllocZeroed
        | KnownNames::StdAllocExchangeMalloc => {
            let heap_object_path = Path::new_heap_obj(fpb.fpag.func_id, location);
            fpb.acx
                .set_path_rustc_type(heap_object_path.clone(), tcx.types.u8);
            fpb.add_addr_edge(heap_object_path, destination.clone());
            true
        }
        // Allocates memory on the heap and returns a result of Result<NonNull<[u8]>, AllocError> type.
        // If the allocation is successful, the result would be Result::Ok<NonNull<[u8]>>, Result::Err<AllocError> otherwise.
        KnownNames::StdAllocAllocatorAllocate | KnownNames::StdAllocAllocatorAllocateZeroed => {
            let heap_object_path = Path::new_heap_obj(fpb.fpag.func_id, location);
            fpb.acx
                .set_path_rustc_type(heap_object_path.clone(), tcx.types.u8);
            let cast_heap_object_path = fpb
                .acx
                .cast_to(&heap_object_path, Ty::new_slice(fpb.acx.tcx, tcx.types.u8))
                .expect("Cast Error");
            // (dst as Ok).0: NonNull<[u8]>, ((dst as Ok).0).0: *const [u8]
            let projection = vec![
                PathSelector::Downcast(0),
                PathSelector::Field(0),
                PathSelector::Field(0),
            ];
            let qualified_path = Path::new_qualified(destination.clone(), projection);
            fpb.acx
                .set_path_rustc_type(qualified_path.clone(), const_u8_rawptr_type(tcx));
            // Instead of inserting an address_of address from heap_object to ((dst as Ok).0).0,
            // we create a auxiliary path as an intermediary
            // ```let aux: *const u8 = &heap_object;  ((dst as Ok).0).0 = aux;```
            let aux = fpb
                .acx
                .create_aux_local(fpb.fpag.func_id, const_u8_rawptr_type(tcx));
            fpb.add_addr_edge(cast_heap_object_path, aux.clone());
            fpb.add_direct_edge(aux, qualified_path);
            true
        }
        // Reallocate memory on the heap and returns the address as `*mut u8`
        KnownNames::RustRealloc | KnownNames::StdAllocRealloc => {
            // Instead of creating a new heap object path, we return the original heap object directly.
            // Therefore we add an direct edge from the source heap object to the target heap object.
            fpb.add_direct_edge(args[0].clone(), destination.clone());
            true
        }
        // Reallocates memory on the heap and returns a result of `Result<NonNull<[u8]>, AllocError>` type.
        KnownNames::StdAllocAllocatorGrow
        | KnownNames::StdAllocAllocatorGrowZeroed
        | KnownNames::StdAllocAllocatorShrink => {
            // Similar to RustRealloc, we add an direct edge from the source pointer to the destination pointer
            // Note: source arg type: NonNull<u8>, destination type: Result<NonNull<[u8]>, AllocError>
            // we need to cast from type *const u8 (arg[1].0) to type *const [u8] (ret.downcast(0).0.0)
            let src_ptr_path = Path::new_qualified(args[1].clone(), vec![PathSelector::Field(0)]);

            // (dst as Ok).0: NonNull<[u8]>, ((dst as Ok).0).0: *const [u8]
            let projection = vec![
                PathSelector::Downcast(0),
                PathSelector::Field(0),
                PathSelector::Field(0),
            ];
            let dst_ptr_path = Path::new_qualified(destination.clone(), projection);
            fpb.acx
                .set_path_rustc_type(dst_ptr_path.clone(), const_u8_rawptr_type(tcx));
            fpb.add_cast_edge(src_ptr_path, dst_ptr_path);
            true
        }
        KnownNames::RustDealloc
        | KnownNames::RustAllocErrorHandler
        | KnownNames::StdAllocDealloc
        | KnownNames::StdAllocBoxFree
        | KnownNames::StdAllocHandleAllocError
        | KnownNames::StdAllocAllocatorDeallocate => true,
        _ => false,
    }
}

fn is_std_ptr_unique<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    match ty.kind() {
        TyKind::Adt(def, _) => {
            let def_path_str = tcx.def_path_str(def.did());
            def_path_str == "std::ptr::Unique"
        }
        _ => false,
    }
}

fn is_std_ptr_nonnull<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    match ty.kind() {
        TyKind::Adt(def, _) => {
            let def_path_str = tcx.def_path_str(def.did());
            def_path_str == "std::ptr::NonNull"
        }
        _ => false,
    }
}

fn const_rawptr_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    tcx.mk_ty_from_kind(TyKind::RawPtr(ty, rustc_middle::mir::Mutability::Not))
}

fn const_u8_rawptr_type(tcx: TyCtxt) -> Ty {
    tcx.mk_ty_from_kind(TyKind::RawPtr(
        tcx.mk_ty_from_kind(TyKind::Slice(tcx.types.u8)),
        rustc_middle::mir::Mutability::Not,
    ))
}

/// Checks if a function is a class constructor (wrapper constructor, not data constructor)
/// We only handle wrapper constructors here, as they are the entry point for class instantiation.
fn is_class_constructor<'tcx>(
    acx: &mut AnalysisContext<'tcx, '_>,
    callee_def_id: DefId,
    gen_args: &GenericArgsRef<'tcx>,
) -> bool {
    let func_id = acx.get_func_id(callee_def_id, *gen_args);
    let func_ref = acx.get_function_reference(func_id);
    
    if let Some(constructor) = class_analysis::identify_class_constructor(&func_ref) {
        // Only handle wrapper constructors (the public API)
        // Data constructors are internal implementation details
        return constructor.is_wrapper;
    }
    
    false
}

/// Handles class constructor calls by creating a heap object abstraction
/// and establishing pointer relationships.
/// 
/// For `let p = Point::new(10, 20);`:
/// - Creates a HeapObj node representing the Point instance
/// - Establishes that `p` (or the pointer inside `p`) points to this HeapObj
/// - Registers the class type and marks instance/reference in the type system
fn handle_class_constructor<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    callee_def_id: &DefId,
    gen_args: &GenericArgsRef<'tcx>,
    destination: &Rc<Path>,
    location: mir::Location,
) {
    // Get class name from the constructor
    let func_id = fpb.acx.get_func_id(*callee_def_id, *gen_args);
    let func_ref = fpb.acx.get_function_reference(func_id);
    let class_name = class_analysis::identify_class_constructor(&func_ref)
        .map(|c| c.class_name)
        .unwrap_or_else(|| "Unknown".to_string());
    
    // Get the return type of the constructor (e.g., Point<RcDyn<Point>>)
    let return_ty = fpb.acx.get_path_rustc_type(destination)
        .expect("Failed to get return type for class constructor");
    let class_name = if class_name == "Unknown" {
        class_analysis::extract_dsl_class_from_wrapper(fpb.acx.tcx, return_ty, None)
            .and_then(|ty| class_analysis::class_name_of_dsl_type(fpb.acx.tcx, ty))
            .unwrap_or(class_name)
    } else {
        class_name
    };
    
    // Create a heap object to represent the class instance
    // This represents the actual class data allocated on the heap
    let heap_object_path = Path::new_heap_obj(fpb.fpag.func_id, location);
    
    // The heap object represents the concrete class data type
    // For now, we use the return type, but ideally we'd extract the inner data type
    // For Point, the heap object would be of type data::Point
    fpb.acx.set_path_rustc_type(heap_object_path.clone(), return_ty);
    
    // Store the concretized type for the heap object
    // This allows the analysis to know what type this heap object represents
    fpb.acx
        .concretized_heap_objs
        .insert(heap_object_path.clone(), return_ty);
    
    // ===== Class Type System Integration =====
    // Register the class type in our simplified type system
    fpb.acx.class_type_system.register_class(&class_name);
    
    // Mark the heap object as an instance of this class
    fpb.acx.class_type_system.mark_class_instance(heap_object_path.clone(), &class_name);
    
    // Mark the destination as a reference to this class
    fpb.acx.class_type_system.mark_class_reference(destination.clone(), &class_name, true);
    // ==========================================
    
    // Use canonical name so ptr ids match visit_assign/cast (no duplicate e.g. get_and_wrap::local_2).
    let func_ref = fpb.acx.get_function_reference(fpb.fpag.func_id);
    let func_name_raw = func_ref.to_string();
    let func_name = class_analysis::canonical_class_method_name(&func_name_raw);

    use crate::util::class::ptr_system::path_to_class_ptr_id;
    let ptr_id = path_to_class_ptr_id(&destination, Some(&func_name), None);

    // ===== rcpta ClassPAG Alloc (source-level only). Author: Yan Wang, Date: 2026-02-02 =====
    // Only create ClassObj when the *caller* is source-level (user code), not inside core/std/alloc/classes
    // or class ctor/DSL internal. Use is_source_level_context so we don't create ptrs/objs in DSL runtime.
    if class_analysis::is_source_level_context(&func_name_raw) {
        use crate::rcpta::{AllocSite, ClassPtr};
        let alloc_site = AllocSite::new(&func_name_raw, format!("{:?}", location));
        let obj_id_rcpta = fpb.acx.class_pag.create_obj(class_name.clone(), alloc_site);
        let cptr = ClassPtr::new_local(ptr_id.clone(), class_name.clone());
        fpb.acx.class_pag.get_or_create_ptr(cptr);
        fpb.acx.class_pag.add_alloc(&ptr_id, &obj_id_rcpta);
    }
    // ============================================
    
    // Establish that the destination points to the heap object
    // Use add_addr_edge because the constructor returns a pointer/reference to the heap object
    // This creates: destination -> (address of) -> HeapObj
    fpb.add_addr_edge(heap_object_path.clone(), destination.clone());
    
    debug!(
        "Class constructor [{}]: created HeapObj {:?} for type {:?}, destination: {:?}",
        class_name, heap_object_path, return_ty, destination
    );
}

/// Handles class cast calls (into_superclass, try_into_subtype, cast_mixin): same object, different type view.
/// rcpta: create ClassPtr for receiver and destination, add Assign edge receiver → destination. Author: Yan Wang, Date: 2026-02-02
pub fn handle_class_cast_call<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    callee_def_id: &DefId,
    gen_args: &GenericArgsRef<'tcx>,
    args: &[Rc<Path>],
    destination: &Rc<Path>,
    location: mir::Location,
) {
    let callee_path = fpb.acx.tcx.def_path_str(*callee_def_id);
    if class_analysis::is_lite_downcast_callee_path(&callee_path) {
        handle_lite_downcast_call(fpb, callee_def_id, gen_args, args, destination, location);
        return;
    }

    if args.is_empty() {
        return;
    }
    let tcx = fpb.acx.tcx;
    let receiver_path = &args[0];

    // Types from callee signature (substituted)
    let return_ty = match type_util::function_return_type(tcx, *callee_def_id, *gen_args) {
        ty if !ty.is_unit() => ty,
        _ => return,
    };
    let receiver_ty = match type_util::function_first_arg_type(tcx, *callee_def_id, *gen_args) {
        Some(ty) => ty,
        None => return,
    };

    let receiver_class_ty = match class_analysis::extract_dsl_class_from_wrapper(tcx, receiver_ty, None) {
        Some(ty) => ty,
        None => return,
    };
    let dest_class_ty = match class_analysis::extract_dsl_class_from_wrapper(tcx, return_ty, None) {
        Some(ty) => ty,
        None => return,
    };
    let receiver_class = match class_analysis::class_name_of_dsl_type(tcx, receiver_class_ty) {
        Some(s) => s,
        None => return,
    };
    let dest_class = match class_analysis::class_name_of_dsl_type(tcx, dest_class_ty) {
        Some(s) => s,
        None => return,
    };

    // rcpta graph feeding: ensure entry-related interface/mixin nodes are present in
    // `class_type_system` so later dumps (e.g. inheritance graph) can filter by entry.
    fpb.acx.class_type_system.register_class(&receiver_class);
    fpb.acx.class_type_system.register_class(&dest_class);

    // Use canonical name so ptr ids match (no duplicate get_and_wrap::local_2/local_3 from impl vs data).
    let func_ref = fpb.acx.get_function_reference(fpb.fpag.func_id);
    let func_name = class_analysis::canonical_class_method_name(&func_ref.to_string());
    use crate::util::class::ptr_system::path_to_class_ptr_id;

    // When return type is Option<CRc<T>> (e.g. try_into_subtype), the cast result is stored
    // in Option.Some.0; use that path as cast dest so later unwrap() can Assign(option_inner -> lhs).
    let is_option_return = if let TyKind::Adt(def, _) = return_ty.kind() {
        let path = tcx.def_path_str(def.did());
        path.contains("option::Option") || path.contains("core::option") || path.contains("std::option")
    } else {
        false
    };
    let (effective_dest, dest_ptr_id) = if is_option_return {
        let option_some_inner = Path::new_qualified(
            destination.clone(),
            vec![PathSelector::Downcast(1), PathSelector::Field(0)],
        );
        fpb.acx.set_path_rustc_type(option_some_inner.clone(), dest_class_ty);
        let ptr_id = path_to_class_ptr_id(&option_some_inner, Some(&func_name), None);
        (option_some_inner, ptr_id)
    } else {
        let ptr_id = path_to_class_ptr_id(destination, Some(&func_name), None);
        (destination.clone(), ptr_id)
    };

    let receiver_ptr_id = path_to_class_ptr_id(receiver_path, Some(&func_name), None);

    // rcpta: ClassPAG — cast source = canonical(receiver).
    if class_analysis::is_source_level_context(&func_name) {
        use crate::rcpta::ClassPtr;
        let canonical_receiver = fpb.acx.get_canonical_rcpta_ptr(&receiver_ptr_id);
        let receiver_cptr = ClassPtr::new_local(canonical_receiver.clone(), receiver_class);
        let dest_cptr = ClassPtr::new_local(dest_ptr_id.clone(), dest_class);
        fpb.acx.class_pag.get_or_create_ptr(receiver_cptr);
        fpb.acx.class_pag.get_or_create_ptr(dest_cptr);
        fpb.acx.class_pag.add_cast(&canonical_receiver, &dest_ptr_id);

        // Record cast site span so we can later output `line:col cast is safe/unsafe`.
        let span = fpb.mir.source_info(location).span;
        let sm = tcx.sess.source_map();
        let lo = sm.lookup_char_pos(span.lo());
        let file = lo.file.name.prefer_local().to_string();
        // rustc line/col are 1-based line, 0-based column; normalize to 1-based col for readability.
        let line = lo.line;
        let col = lo.col.to_usize() + 1;
        let src_loc = format!("{}:{}:{}", file, line, col);
        fpb.acx
            .class_pag
            .add_cast_site(canonical_receiver, dest_ptr_id.clone(), src_loc);
    }

    debug!(
        "Class cast: {} -> {} (receiver: {:?}, dest: {:?})",
        receiver_ptr_id, dest_ptr_id, receiver_path, effective_dest
    );
}

fn handle_lite_downcast_call<'tcx>(
    fpb: &mut FuncPAGBuilder<'_, 'tcx, '_>,
    callee_def_id: &DefId,
    gen_args: &GenericArgsRef<'tcx>,
    args: &[Rc<Path>],
    destination: &Rc<Path>,
    location: mir::Location,
) {
    let trace = std::env::var_os("RCPTA_TRACE_LITE_DOWNCAST").is_some();
    if args.is_empty() {
        if trace {
            eprintln!("[rcpta][lite-downcast] skip: no args");
        }
        return;
    }

    let tcx = fpb.acx.tcx;
    let return_ty = match type_util::function_return_type(tcx, *callee_def_id, *gen_args) {
        ty if !ty.is_unit() => ty,
        ty => {
            if trace {
                eprintln!("[rcpta][lite-downcast] skip: unit return ty={:?}", ty);
            }
            return;
        }
    };
    if trace {
        eprintln!(
            "[rcpta][lite-downcast] callee={} return_ty={:?} dst={:?} arg0={:?}",
            tcx.def_path_str(*callee_def_id),
            return_ty,
            destination,
            args[0]
        );
    }
    let Some(dest_class_ty) = class_analysis::extract_dsl_class_from_wrapper(tcx, return_ty, None) else {
        if trace {
            eprintln!("[rcpta][lite-downcast] skip: no dest class in return_ty={:?}", return_ty);
        }
        return;
    };
    let Some(dest_class) = class_analysis::class_name_of_dsl_type(tcx, dest_class_ty) else {
        if trace {
            eprintln!("[rcpta][lite-downcast] skip: no dest class name for ty={:?}", dest_class_ty);
        }
        return;
    };

    let func_ref = fpb.acx.get_function_reference(fpb.fpag.func_id);
    let func_name = class_analysis::canonical_class_method_name(&func_ref.to_string());
    if !class_analysis::is_source_level_context(&func_name) {
        if trace {
            eprintln!("[rcpta][lite-downcast] skip: non source context {}", func_name);
        }
        return;
    }

    use crate::mir::path::{Path, PathSelector};
    use crate::rcpta::ClassPtr;
    use crate::util::class::ptr_system::path_to_class_ptr_id;

    let result_ok_inner = Path::new_qualified(
        destination.clone(),
        vec![PathSelector::Downcast(0), PathSelector::Field(0)],
    );
    fpb.acx.set_path_rustc_type(result_ok_inner.clone(), dest_class_ty);
    let dest_ptr_id = path_to_class_ptr_id(&result_ok_inner, Some(&func_name), None);

    let raw_receiver_ptr_id = path_to_class_ptr_id(&args[0], Some(&func_name), None);
    let mut src_ptr_id = if fpb.acx.class_pag.get_ptr(&raw_receiver_ptr_id).is_some() {
        raw_receiver_ptr_id.clone()
    } else {
        fpb.acx.get_canonical_rcpta_ptr(&raw_receiver_ptr_id)
    };
    if fpb.acx.class_pag.get_ptr(&src_ptr_id).is_none() {
        if let Some(base_path) = fpb.acx.rcpta_ref_ptr_to_base_path.get(&raw_receiver_ptr_id) {
            let base_ptr_id = path_to_class_ptr_id(base_path, Some(&func_name), None);
            src_ptr_id = if fpb.acx.class_pag.get_ptr(&base_ptr_id).is_some() {
                base_ptr_id
            } else {
                fpb.acx.get_canonical_rcpta_ptr(&base_ptr_id)
            };
        }
    }
    if fpb.acx.class_pag.get_ptr(&src_ptr_id).is_none() {
        let func_prefix = format!("{}::param_", func_name);
        let existing_param_ptr_id = fpb
            .acx
            .class_pag
            .ptr_ids()
            .find(|ptr_id| ptr_id.starts_with(&func_prefix))
            .cloned();
        if let Some(param_ptr_id) = existing_param_ptr_id {
            src_ptr_id = param_ptr_id;
        } else {
            let param_1 = Path::new_local_parameter_or_result(fpb.fpag.func_id, 1, fpb.mir.arg_count);
            if let Some(param_ty) = fpb.acx.get_path_rustc_type(&param_1) {
                if let Some(src_class_ty) = class_analysis::extract_dsl_class_from_wrapper(tcx, param_ty, None) {
                    if let Some(src_class) = class_analysis::class_name_of_dsl_type(tcx, src_class_ty) {
                        let param_ptr_id = path_to_class_ptr_id(&param_1, Some(&func_name), None);
                        fpb.acx
                            .class_pag
                            .get_or_create_ptr(ClassPtr::new_local(param_ptr_id.clone(), src_class));
                        src_ptr_id = param_ptr_id;
                    }
                }
            }
        }
    }

    let src_class = fpb
        .acx
        .class_pag
        .get_ptr(&src_ptr_id)
        .map(|p| p.class_type.clone())
        .unwrap_or_else(|| {
            class_analysis::extract_dsl_class_from_wrapper(tcx, args[0].try_eval_path_type(fpb.acx), None)
                .and_then(|ty| class_analysis::class_name_of_dsl_type(tcx, ty))
                .unwrap_or_else(|| dest_class.clone())
        });

    fpb.acx.class_type_system.register_class(&src_class);
    fpb.acx.class_type_system.register_class(&dest_class);
    fpb.acx
        .class_pag
        .get_or_create_ptr(ClassPtr::new_local(src_ptr_id.clone(), src_class));
    fpb.acx
        .class_pag
        .get_or_create_ptr(ClassPtr::new_local(dest_ptr_id.clone(), dest_class));
    fpb.acx.class_pag.add_cast(&src_ptr_id, &dest_ptr_id);

    let span = fpb.mir.source_info(location).span;
    let sm = tcx.sess.source_map();
    let lo = sm.lookup_char_pos(span.lo());
    let file = lo.file.name.prefer_local().to_string();
    let line = lo.line;
    let col = lo.col.to_usize() + 1;
    let src_loc = format!("{}:{}:{}", file, line, col);
    fpb.acx.class_pag.add_cast_site(src_ptr_id, dest_ptr_id, src_loc);
    if trace {
        eprintln!("[rcpta][lite-downcast] added cast site");
    }
}
