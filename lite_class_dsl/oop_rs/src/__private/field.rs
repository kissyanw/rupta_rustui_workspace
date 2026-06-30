//! Get and set traits for fields.

use crate::alloc::rc::{Rc, Weak};
use crate::rc::{IsRcLike, RcLike};
use core::cell::{Cell, OnceCell};
use std::cell::{Ref, RefCell, RefMut};

/// Trait for fields without any modifiers of `impl Copy` types.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not `Copy`, consider add a `ref` modifier to the field",
    note = "or use `RcLike` to wrap the field if it is `Clone` and behaves like a `Rc`"
)]
pub trait FieldCopy: Copy + Sized {
    #[inline]
    fn get(&self) -> Self {
        *self
    }
}

/// Trait for fields without any modifiers of `Rc` types.
pub trait FieldRc: Sized {
    type Get;
    type Set;

    fn new(value: Self::Set) -> Self;
    fn get(&self) -> Self::Get;
}

/// Trait for  `ref` fields.
pub trait FieldRef: Sized {
    #[inline]
    fn get(&self) -> &Self {
        self
    }
}

/// Trait for `mut` fields of `impl Copy` types.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not `Copy`, consider add a `ref` modifier to the field",
    note = "or use `RcLike` to wrap the field if it is `Clone` and behaves like a `Rc`"
)]
pub trait FieldMutCopy: FieldCopy {
    #[inline]
    fn new(value: Self) -> Cell<Self> {
        Cell::new(value)
    }

    #[inline]
    fn get(this: &Cell<Self>) -> Self {
        this.get()
    }

    #[inline]
    fn set(this: &Cell<Self>, value: Self) {
        this.set(value);
    }

    #[inline]
    fn replace(this: &Cell<Self>, value: Self) -> Self {
        this.replace(value)
    }

    #[inline]
    fn update(this: &Cell<Self>, f: impl FnOnce(Self) -> Self) {
        this.update(f)
    }
}

/// Getter and setter trait for `mut` fields of `Rc` types.
pub trait FieldMutRc: FieldRc {
    #[inline]
    fn new(value: Self::Set) -> Cell<Self> {
        Cell::new(FieldRc::new(value))
    }

    #[inline]
    fn get(this: &Cell<Self>) -> Self::Get {
        FieldRc::get(unsafe { &*this.as_ptr() })
    }

    #[inline]
    fn set(this: &Cell<Self>, value: Self::Set) {
        this.set(FieldRc::new(value));
    }

    #[inline]
    fn replace(this: &Cell<Self>, value: Self::Set) -> Self::Get {
        let old = FieldMutRc::get(this);
        FieldMutRc::set(this, value);
        old
    }

    #[inline]
    fn update(this: &Cell<Self>, f: impl FnOnce(Self::Get) -> Self::Set) {
        FieldMutRc::set(this, f(FieldMutRc::get(this)));
    }
}

/// Getter and setter trait for `ref mut` fields.
pub trait FieldRefMut: FieldRef {
    #[inline]
    #[track_caller]
    fn get(this: &RefCell<Self>) -> Ref<'_, Self> {
        this.borrow()
    }

    #[inline]
    #[track_caller]
    fn get_mut(this: &RefCell<Self>) -> RefMut<'_, Self> {
        this.borrow_mut()
    }

    #[inline]
    #[track_caller]
    fn update(this: &RefCell<Self>, f: impl FnOnce(&mut Self) -> Self) {
        this.replace_with(f);
    }

    #[inline]
    #[track_caller]
    fn replace(this: &RefCell<Self>, value: Self) -> Self {
        this.replace(value)
    }

    #[inline]
    #[track_caller]
    fn replace_with(this: &RefCell<Self>, f: impl FnOnce(&mut Self) -> Self) -> Self {
        this.replace_with(f)
    }

    #[inline]
    #[track_caller]
    fn set(this: &RefCell<Self>, value: Self) {
        this.replace(value);
    }
}

/// Rait for `#[late]` fields of `impl Copy` types.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not `Copy`, consider add a `ref` modifier to the field",
    note = "or use `RcLike` to wrap the field if it is `Clone` and behaves like a `Rc`"
)]
pub trait FieldLateCopy: FieldCopy {
    #[inline]
    fn new() -> OnceCell<Self> {
        OnceCell::new()
    }

    #[inline]
    #[track_caller]
    fn get(this: &OnceCell<Self>) -> Self {
        FieldCopy::get(this.get().unwrap_or_else(late_field_not_initialized))
    }

    #[inline]
    fn get_or_init(this: &OnceCell<Self>, f: impl FnOnce() -> Self) -> Self {
        FieldCopy::get(this.get_or_init(f))
    }

    #[inline]
    #[track_caller]
    fn set(this: &OnceCell<Self>, value: Self) {
        this.set(value)
            .unwrap_or_else(late_field_already_initialized);
    }
}

/// Getter and setter trait for `#[late]` fields of `Rc` types.
pub trait FieldLateRc: FieldRc {
    #[inline]
    #[track_caller]
    fn get(this: &OnceCell<Self>) -> Self::Get {
        FieldRc::get(this.get().unwrap_or_else(late_field_not_initialized))
    }

    #[inline]
    fn get_or_init(this: &OnceCell<Self>, f: impl FnOnce() -> Self::Set) -> Self::Get {
        FieldRc::get(this.get_or_init(|| FieldRc::new(f())))
    }
    #[inline]
    fn set(this: &OnceCell<Self>, value: Self::Set) {
        this.set(FieldRc::new(value))
            .unwrap_or_else(late_field_already_initialized);
    }
}

/// Getter and setter trait for `#[late] ref` fields.
pub trait FieldLateRef: FieldRef {
    #[inline]
    #[track_caller]
    fn get(this: &OnceCell<Self>) -> &Self {
        FieldRef::get(this.get().unwrap_or_else(late_field_not_initialized))
    }
    #[inline]
    fn get_or_init(this: &OnceCell<Self>, f: impl FnOnce() -> Self) -> &Self {
        FieldRef::get(this.get_or_init(f))
    }
    #[inline]
    fn set(this: &OnceCell<Self>, value: Self) {
        this.set(value)
            .unwrap_or_else(late_field_already_initialized);
    }
}

/// Trait for `#[late] mut` fields of `Copy` types.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not `Copy`, consider add a `ref` modifier to the field",
    note = "or use `RcLike` to wrap the field if it is `Clone` and behaves like a `Rc`"
)]
pub trait FieldLateMutCopy: FieldMutCopy {
    #[inline]
    #[track_caller]
    fn get(this: &Cell<Option<Self>>) -> Self {
        FieldMutCopy::get(this).unwrap_or_else(late_field_not_initialized)
    }

    #[inline]
    fn get_or_init(this: &Cell<Option<Self>>, f: impl FnOnce() -> Self) -> Self {
        if unsafe { &*this.as_ptr() }.is_none() {
            this.set(Some(f()));
        }
        unsafe { FieldMutCopy::get(this).unwrap_unchecked() }
    }

    #[inline]
    fn set(this: &Cell<Option<Self>>, value: Self) {
        this.set(Some(value));
    }

    #[inline]
    fn replace(this: &Cell<Option<Self>>, value: Self) -> Option<Self> {
        this.replace(Some(value))
    }

    #[inline]
    fn update(this: &Cell<Option<Self>>, f: impl FnOnce(Option<Self>) -> Self) {
        this.update(|v| Some(f(v)));
    }
}

/// Trait for `#[late] mut` fields of `Rc` types.
pub trait FieldLateMutRc: FieldLateRc
where
    Option<Self>: FieldMutRc<Get = Option<Self::Get>, Set = Option<Self::Set>>,
{
    type GetOption;

    #[inline]
    #[track_caller]
    fn get(this: &Cell<Option<Self>>) -> Self::Get {
        FieldMutRc::get(this).unwrap_or_else(late_field_not_initialized)
    }

    #[inline]
    fn get_or_init(this: &Cell<Option<Self>>, f: impl FnOnce() -> Self::Set) -> Self::Get {
        if unsafe { &*this.as_ptr() }.is_none() {
            this.set(Some(FieldRc::new(f())));
        }
        unsafe { FieldMutRc::get(this).unwrap_unchecked() }
    }

    #[inline]
    fn set(this: &Cell<Option<Self>>, value: Self::Set) {
        this.set(Some(FieldRc::new(value)));
    }

    fn replace(this: &Cell<Option<Self>>, value: Self::Set) -> Self::GetOption;
    fn update(this: &Cell<Option<Self>>, f: impl FnOnce(Self::GetOption) -> Self::Set);
}

/// Trait for `#[late] ref mut` fields.
pub trait FieldLateRefMut: FieldRefMut {
    #[inline]
    #[track_caller]
    fn get(this: &RefCell<Option<Self>>) -> Ref<'_, Self> {
        #[inline]
        #[track_caller]
        fn late_field_as_ref<T>(v: &Option<T>) -> &T {
            v.as_ref().unwrap_or_else(late_field_not_initialized)
        }
        Ref::map(FieldRefMut::get(this), late_field_as_ref)
    }

    #[inline]
    #[track_caller]
    fn get_mut(this: &RefCell<Option<Self>>) -> RefMut<'_, Self> {
        #[inline]
        #[track_caller]
        fn late_field_as_mut<T>(v: &mut Option<T>) -> &mut T {
            v.as_mut().unwrap_or_else(late_field_not_initialized)
        }
        RefMut::map(FieldRefMut::get_mut(this), late_field_as_mut)
    }

    #[inline]
    #[track_caller]
    fn get_or_init(this: &RefCell<Option<Self>>, f: impl FnOnce() -> Self) -> Ref<'_, Self> {
        if let Ok(borrow) = this.try_borrow()
            && let Ok(value) = Ref::filter_map(borrow, Option::as_ref)
        {
            return value;
        }
        FieldRefMut::set(this, Some(f()));
        unsafe {
            Ref::map(
                this.try_borrow().unwrap_unchecked(),
                #[inline]
                |v| v.as_ref().unwrap_unchecked(),
            )
        }
    }

    #[inline]
    #[track_caller]
    fn get_mut_or_init(this: &RefCell<Option<Self>>, f: impl FnOnce() -> Self) -> RefMut<'_, Self> {
        RefMut::map(FieldRefMut::get_mut(this), |v| v.get_or_insert_with(f))
    }

    #[inline]
    #[track_caller]
    fn update(this: &RefCell<Option<Self>>, f: impl FnOnce(&mut Option<Self>) -> Self) {
        FieldLateRefMut::replace_with(this, f);
    }

    #[inline]
    #[track_caller]
    fn replace(this: &RefCell<Option<Self>>, value: Self) -> Option<Self> {
        FieldRefMut::replace(this, Some(value))
    }

    #[inline]
    #[track_caller]
    fn replace_with(
        this: &RefCell<Option<Self>>,
        f: impl FnOnce(&mut Option<Self>) -> Self,
    ) -> Option<Self> {
        FieldRefMut::replace_with(this, |v| Some(f(v)))
    }

    #[inline]
    #[track_caller]
    fn set(this: &RefCell<Option<Self>>, value: Self) {
        FieldLateRefMut::replace(this, value);
    }
}

impl<T: Copy> FieldCopy for T {}

impl<T: ?Sized> FieldRc for Rc<T> {
    type Get = Rc<T>;
    type Set = Rc<T>;

    #[inline]
    fn new(value: Self::Set) -> Self {
        value
    }

    #[inline]
    fn get(&self) -> Self::Get {
        Rc::clone(self)
    }
}

impl<T: ?Sized> FieldRc for Option<Rc<T>> {
    type Get = Option<Rc<T>>;
    type Set = Option<Rc<T>>;

    #[inline]
    fn new(value: Self::Set) -> Self {
        value
    }

    #[inline]
    fn get(&self) -> Self::Get {
        self.as_ref().map(Rc::clone)
    }
}

impl<T: ?Sized> FieldRc for Weak<T> {
    type Get = Rc<T>;
    type Set = Rc<T>;

    #[inline]
    fn new(value: Self::Set) -> Self {
        Rc::downgrade(&value)
    }

    #[inline]
    #[track_caller]
    fn get(&self) -> Self::Get {
        self.upgrade().unwrap_or_else(late_field_upgrade_failed)
    }
}

impl<T: ?Sized> FieldRc for Option<Weak<T>> {
    type Get = Option<Rc<T>>;
    type Set = Option<Rc<T>>;

    #[inline]
    fn new(value: Self::Set) -> Self {
        value.as_ref().map(Rc::downgrade)
    }

    #[inline]
    fn get(&self) -> Self::Get {
        self.as_ref().and_then(Weak::upgrade)
    }
}

impl<T: IsRcLike> FieldRc for RcLike<T> {
    type Get = T;
    type Set = T;

    #[inline]
    fn new(value: Self::Set) -> Self {
        RcLike::new(value.into())
    }

    #[inline]
    fn get(&self) -> Self::Get {
        RcLike::into_inner(self.clone())
    }
}

impl<T: IsRcLike> FieldRc for Option<RcLike<T>> {
    type Get = Option<T>;
    type Set = Option<T>;

    #[inline]
    fn new(value: Self::Set) -> Self {
        value.map(RcLike::new)
    }

    #[inline]
    fn get(&self) -> Self::Get {
        self.clone().map(RcLike::into_inner)
    }
}

impl<T> FieldRef for T {}

impl<T: FieldCopy> FieldMutCopy for T {}

impl<T: FieldRc> FieldMutRc for T {}

impl<T> FieldRefMut for T {}

impl<T: FieldCopy> FieldLateCopy for T {}

impl<T: FieldRc> FieldLateRc for T {}

impl<T: FieldRef> FieldLateRef for T {}

impl<T: FieldMutCopy> FieldLateMutCopy for T {}

impl<T: ?Sized> FieldLateMutRc for Rc<T>
where
    Option<Rc<T>>: FieldMutRc<Get = Option<Self::Get>, Set = Option<Self::Set>>,
{
    type GetOption = Option<Rc<T>>;

    #[inline]
    fn replace(this: &Cell<Option<Self>>, value: Self::Set) -> Self::GetOption {
        FieldMutRc::replace(this, Some(FieldRc::new(value)))
    }
    #[inline]
    fn update(this: &Cell<Option<Self>>, f: impl FnOnce(Self::GetOption) -> Self::Set) {
        FieldMutRc::set(this, Some(f(FieldMutRc::get(this))));
    }
}

impl<T: ?Sized> FieldLateMutRc for Weak<T>
where
    Option<Weak<T>>: FieldMutRc<Get = Option<Self::Get>, Set = Option<Self::Set>>,
{
    type GetOption = Option<Rc<T>>;

    #[inline]
    #[track_caller]
    fn replace(this: &Cell<Option<Self>>, value: Self::Set) -> Self::GetOption {
        FieldMutRc::replace(this, Some(value))
    }

    #[inline]
    #[track_caller]
    fn update(this: &Cell<Option<Self>>, f: impl FnOnce(Self::GetOption) -> Self::Set) {
        FieldMutRc::set(this, Some(f(FieldMutRc::get(this))));
    }
}

impl<T: IsRcLike> FieldLateMutRc for RcLike<T>
where
    Option<RcLike<T>>: FieldMutRc<Get = Option<Self::Get>, Set = Option<Self::Set>>,
{
    type GetOption = Option<T>;

    #[inline]
    fn replace(this: &Cell<Option<Self>>, value: Self::Set) -> Self::GetOption {
        FieldMutRc::replace(this, Some(value))
    }
    #[inline]
    fn update(this: &Cell<Option<Self>>, f: impl FnOnce(Self::GetOption) -> Self::Set) {
        FieldMutRc::set(this, Some(f(FieldMutRc::get(this))));
    }
}

impl<T: FieldRefMut> FieldLateRefMut for T {}

#[cold]
#[track_caller]
#[inline(never)]
fn late_field_not_initialized<T>() -> T {
    panic!("`#[late]` field not initialized")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_field_already_initialized<T>(_: T) {
    panic!("`#[late]` field already initialized")
}

#[cold]
#[track_caller]
#[inline(never)]
fn late_field_upgrade_failed<T>() -> T {
    panic!("`#[late]` field upgrade failed")
}
