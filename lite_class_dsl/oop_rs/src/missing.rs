/// Like `Default`, but implemented for types that has an empty value
/// (e.g., `Option`, `Vec`, `HashMap`, etc.)
#[diagnostic::on_unimplemented(message = "`{Self}` must has an initializer")]
pub trait Missing: Default {
    fn missing() -> Self;
}

impl<T> Missing for Option<T> {
    fn missing() -> Self {
        Default::default()
    }
}

impl<T: Missing + crate::rc::IsRcLike> Missing for crate::rc::RcLike<T> {
    fn missing() -> Self {
        crate::rc::RcLike::new(T::missing())
    }
}

impl<T: Missing> Missing for core::cell::Cell<T> {
    fn missing() -> Self {
        core::cell::Cell::new(T::missing())
    }
}

impl<T: Missing> Missing for core::cell::RefCell<T> {
    fn missing() -> Self {
        core::cell::RefCell::new(T::missing())
    }
}

impl<T> Missing for crate::alloc::vec::Vec<T> {
    fn missing() -> Self {
        Default::default()
    }
}

impl Missing for crate::alloc::string::String {
    fn missing() -> Self {
        Default::default()
    }
}

impl<T> Missing for crate::alloc::collections::VecDeque<T> {
    fn missing() -> Self {
        Default::default()
    }
}

impl<T> Missing for crate::alloc::collections::LinkedList<T> {
    fn missing() -> Self {
        Default::default()
    }
}

impl<K, V> Missing for crate::alloc::collections::BTreeMap<K, V> {
    fn missing() -> Self {
        Default::default()
    }
}

impl<T> Missing for crate::alloc::collections::BTreeSet<T> {
    fn missing() -> Self {
        Default::default()
    }
}

impl<T: Ord> Missing for crate::alloc::collections::BinaryHeap<T> {
    fn missing() -> Self {
        Default::default()
    }
}

#[cfg(feature = "hashbrown")]
impl<K, V> Missing for hashbrown::HashMap<K, V> {
    fn missing() -> Self {
        Default::default()
    }
}

#[cfg(feature = "hashbrown")]
impl<T> Missing for hashbrown::HashSet<T> {
    fn missing() -> Self {
        Default::default()
    }
}

#[cfg(feature = "std")]
impl<K, V> Missing for std::collections::HashMap<K, V> {
    fn missing() -> Self {
        Default::default()
    }
}

#[cfg(feature = "std")]
impl<T> Missing for std::collections::HashSet<T> {
    fn missing() -> Self {
        Default::default()
    }
}
