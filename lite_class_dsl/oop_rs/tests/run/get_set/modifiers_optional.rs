use oop_rs::prelude::*;

#[class]
type Foo = class<
    {
        let foo: usize;

        pub fn new(foo: usize) -> Self {
            Self { foo }
        }
    },
>;

#[class]
#[rustfmt::skip]
type Modifier = class<{
    let copy_ty: Option<usize>;
    let class_ty: Option<CRc<Foo>>;
    let mut copy_ty_mut: Option<usize>;
    let mut class_ty_mut: Option<CRc<Foo>>;
    let ref copy_ty_ref: Option<usize>;
    let ref clone_ty_ref: Option<String>;
    let ref class_ty_ref: Option<CRc<Foo>>;
    let ref mut copy_ty_ref_mut: Option<usize>;
    let ref mut clone_ty_ref_mut: Option<String>;
    let ref mut class_ty_ref_mut: Option<CRc<Foo>>;
    #[late] let copy_ty_late: Option<usize>;
    #[late] let class_ty_late: Option<CRc<Foo>>;
    #[late] let mut copy_ty_late_mut: Option<usize>;
    // FIXME: This is not supported yet
    // #[late] let mut class_ty_late_mut: Option<CRc<Foo>> = Some(Foo::new(self.f()));
    #[late] let ref copy_ty_late_ref: Option<usize>;
    #[late] let ref clone_ty_late_ref: Option<String>;
    #[late] let ref class_ty_late_ref: Option<CRc<Foo>>;
    #[late] let ref mut copy_ty_late_ref_mut: Option<usize>;
    #[late] let ref mut clone_ty_late_ref_mut: Option<String>;
    #[late] let ref mut class_ty_late_ref_mut: Option<CRc<Foo>>;

    pub fn new() -> Self {
        Self {}
    }
}>;

#[test]
#[rustfmt::skip]
fn test_copy_ty() {
    let m = Modifier::new();
    assert!(m.get().copy_ty().is_none());
    assert!(m.raw().copy_ty().as_ref().is_none());
}

#[test]
#[rustfmt::skip]
fn test_class_ty() {
    let m = Modifier::new();
    assert!(m.get().class_ty().is_none());
    assert!(m.raw().class_ty().as_ref().is_none());
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_mut() {
    let m = Modifier::new();
    assert!(m.get().copy_ty_mut().is_none());
    assert!(m.raw().copy_ty_mut().get().is_none());
    m.set().copy_ty_mut(Some(1_usize));
    assert_eq!(m.get().copy_ty_mut().unwrap(), 1_usize);
    assert_eq!(m.raw().copy_ty_mut().get().unwrap(), 1_usize);
    let value = m.replace().copy_ty_mut(Some(2_usize));
    assert_eq!(value, Some(1_usize));
    assert_eq!(m.get().copy_ty_mut().unwrap(), 2_usize);
    assert_eq!(m.raw().copy_ty_mut().get().unwrap(), 2_usize);
    m.update().copy_ty_mut(|v| Some(v? + 1_usize));
    assert_eq!(m.get().copy_ty_mut().unwrap(), 3_usize);
    assert_eq!(m.raw().copy_ty_mut().get().unwrap(), 3_usize);
}

#[test]
#[rustfmt::skip]
fn test_class_ty_mut() {
    let m = Modifier::new();
    assert!(m.get().class_ty_mut().is_none());
    assert!(m.raw().class_ty_mut().get_cloned().is_none());
    m.set().class_ty_mut(Some(Foo::new(1_usize)));
    assert_eq!(m.get().class_ty_mut().unwrap().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_mut().get_cloned().unwrap().get().foo(), 1_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_ref() {
    let m = Modifier::new();
    assert!(m.get().copy_ty_ref().as_ref().is_none());
    assert!(m.raw().copy_ty_ref().as_ref().is_none());
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_ref() {
    let m = Modifier::new();
    assert!(m.get().clone_ty_ref().as_ref().is_none());
    assert!(m.raw().clone_ty_ref().as_ref().is_none());
}

#[test]
#[rustfmt::skip]
fn test_class_ty_ref() {
    let m = Modifier::new();
    assert!(m.get().class_ty_ref().as_ref().is_none());
    assert!(m.raw().class_ty_ref().as_ref().is_none());
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_ref_mut() {
    let m = Modifier::new();
    assert!(m.get().copy_ty_ref_mut().is_none());
    assert!(m.raw().copy_ty_ref_mut().borrow().is_none());
    m.set().copy_ty_ref_mut(Some(1_usize));
    assert_eq!(m.get().copy_ty_ref_mut().unwrap(), 1_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().unwrap(), 1_usize);
    let value = m.replace().copy_ty_ref_mut(Some(2_usize));
    assert_eq!(value, Some(1_usize));
    assert_eq!(m.get().copy_ty_ref_mut().unwrap(), 2_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().unwrap(), 2_usize);
    m.update().copy_ty_ref_mut(|v| Some(v.as_ref()? + 1_usize));
    assert_eq!(m.get().copy_ty_ref_mut().unwrap(), 3_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().unwrap(), 3_usize);
    *m.get_mut().copy_ty_ref_mut() = Some(4_usize);
    assert_eq!(m.get().copy_ty_ref_mut().unwrap(), 4_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().unwrap(), 4_usize);
    let value = m.replace_with().copy_ty_ref_mut(|v| Some(v.as_ref()? + 1_usize));
    assert_eq!(value, Some(4_usize));
    assert_eq!(m.get().copy_ty_ref_mut().unwrap(), 5_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().unwrap(), 5_usize);
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_ref_mut() {
    let m = Modifier::new();
    assert!(m.get().clone_ty_ref_mut().as_ref().is_none());
    assert!(m.raw().clone_ty_ref_mut().borrow().as_ref().is_none());
    m.set().clone_ty_ref_mut(Some(1.to_string()));
    assert_eq!(m.get().clone_ty_ref_mut().as_ref().unwrap(), "1");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().as_ref().unwrap(), "1");
    let value = m.replace().clone_ty_ref_mut(Some(2.to_string()));
    assert_eq!(value.as_deref(), Some("1"));
    assert_eq!(m.get().clone_ty_ref_mut().as_ref().unwrap(), "2");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().as_ref().unwrap(), "2");
    m.update().clone_ty_ref_mut(|v| Some(v.as_mut()?.replace("2", "3")));
    assert_eq!(m.get().clone_ty_ref_mut().as_ref().unwrap(), "3");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().as_ref().unwrap(), "3");
    *m.get_mut().clone_ty_ref_mut() = Some(4.to_string());
    assert_eq!(m.get().clone_ty_ref_mut().as_ref().unwrap(), "4");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().as_ref().unwrap(), "4");
    let value = m.replace_with().clone_ty_ref_mut(|v| Some(v.as_mut()?.replace("4", "5")));
    assert_eq!(value.as_deref(), Some("4"));
    assert_eq!(m.get().clone_ty_ref_mut().as_ref().unwrap(), "5");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().as_ref().unwrap(), "5");
}

#[test]
#[rustfmt::skip]
fn test_class_ty_ref_mut() {
    let m = Modifier::new();
    assert!(m.get().class_ty_ref_mut().as_ref().is_none());
    assert!(m.raw().class_ty_ref_mut().borrow().as_ref().is_none());
    m.set().class_ty_ref_mut(Some(Foo::new(1_usize)));
    assert_eq!(m.get().class_ty_ref_mut().as_ref().unwrap().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().as_ref().unwrap().get().foo(), 1_usize);
    let value = m.replace().class_ty_ref_mut(Some(Foo::new(2_usize)));
    assert_eq!(value.unwrap().get().foo(), 1_usize);
    assert_eq!(m.get().class_ty_ref_mut().as_ref().unwrap().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().as_ref().unwrap().get().foo(), 2_usize);
    m.update().class_ty_ref_mut(|v| Some(Foo::new(v.as_ref()?.get().foo() + 1_usize)));
    assert_eq!(m.get().class_ty_ref_mut().as_ref().unwrap().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().as_ref().unwrap().get().foo(), 3_usize);
    *m.get_mut().class_ty_ref_mut() = Some(Foo::new(4_usize));
    assert_eq!(m.get().class_ty_ref_mut().as_ref().unwrap().get().foo(), 4_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().as_ref().unwrap().get().foo(), 4_usize);
    let value = m.replace_with().class_ty_ref_mut(|v| Some(Foo::new(v.as_ref()?.get().foo() + 1_usize)));
    assert_eq!(value.unwrap().get().foo(), 4_usize);
    assert_eq!(m.get().class_ty_ref_mut().as_ref().unwrap().get().foo(), 5_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().as_ref().unwrap().get().foo(), 5_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late() {
    let m = Modifier::new();
    // ok because `copy_ty_late` is not initialized yet
    m.set().copy_ty_late(Some(0_usize));
    assert_eq!(m.get().copy_ty_late().unwrap(), 0_usize);
    assert_eq!(m.raw().copy_ty_late().get().unwrap().as_ref().unwrap(), &0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_copy_ty_late_get() {
    let m = Modifier::new();
    let _get = m.get().copy_ty_late();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_set_and_set() {
    let m = Modifier::new();
    m.set().copy_ty_late(Some(0_usize));
    m.set().copy_ty_late(Some(1_usize));
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late() {
    let m = Modifier::new();
    // ok because `class_ty_late` is not initialized yet
    m.set().class_ty_late(Some(Foo::new(0_usize)));
    assert_eq!(m.get().class_ty_late().unwrap().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late().get().unwrap().as_ref().unwrap().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_class_ty_late_get() {
    let m = Modifier::new();
    let _get = m.get().class_ty_late();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_set_and_set() {
    let m = Modifier::new();
    m.set().class_ty_late(Some(Foo::new(0_usize)));
    m.set().class_ty_late(Some(Foo::new(1_usize)));
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_mut() {
    let m = Modifier::new();
    m.set().copy_ty_late_mut(Some(0_usize));
    assert_eq!(m.get().copy_ty_late_mut().unwrap(), 0_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap().as_ref().unwrap(), &0_usize);
    m.set().copy_ty_late_mut(Some(1_usize));
    assert_eq!(m.get().copy_ty_late_mut().unwrap(), 1_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap().as_ref().unwrap(), &1_usize);
    let value = m.replace().copy_ty_late_mut(Some(2_usize));
    assert_eq!(value.unwrap(), Some(1_usize));
    assert_eq!(m.get().copy_ty_late_mut().unwrap(), 2_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap().as_ref().unwrap(), &2_usize);
    m.update().copy_ty_late_mut(|v| Some(v?? + 1_usize));
    assert_eq!(m.get().copy_ty_late_mut().unwrap(), 3_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap().as_ref().unwrap(), &3_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_copy_ty_late_mut_get() {
    let m = Modifier::new();
    let _ = m.get().copy_ty_late_mut();
}

#[test]
#[rustfmt::skip]
#[cfg(false)] // FIXME: This is not supported yet
fn test_class_ty_late_mut() {
    let m = Modifier::new();
    assert!(m.get().class_ty_late_mut().is_none());
    assert!(m.raw().class_ty_late_mut().get().is_none());
    m.set().class_ty_late_mut(Some(Foo::new(1_usize)));
    assert_eq!(m.get().class_ty_late_mut().unwrap().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_late_mut().get().unwrap().as_ref().unwrap().get().foo(), 1_usize);
    let value = m.replace().class_ty_late_mut(Some(Foo::new(2_usize)));
    assert_eq!(value.unwrap().get().foo(), Some(1_usize));
    assert_eq!(m.get().class_ty_late_mut().unwrap().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_late_mut().get().unwrap().as_ref().unwrap().get().foo(), 2_usize);
    m.update().class_ty_late_mut(|v| Some(Foo::new(v.unwrap().as_ref()?.get().foo() + 1_usize)));
    assert_eq!(m.get().class_ty_late_mut().unwrap().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_late_mut().get().unwrap().as_ref().unwrap().get().foo(), 3_usize);
}

#[test]
#[rustfmt::skip]
#[cfg(false)] // FIXME: This is not supported yet
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_class_ty_late_mut_get() {
    let m = Modifier::new();
    let _ = m.get().class_ty_late_mut();
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_ref() {
    let m = Modifier::new();
    // ok because `copy_ty_late_ref` is not initialized yet
    m.set().copy_ty_late_ref(Some(0_usize));
    assert_eq!(m.get().copy_ty_late_ref().as_ref().unwrap(), &0_usize);
    assert_eq!(m.raw().copy_ty_late_ref().get().unwrap().as_ref().unwrap(), &0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_copy_ty_late_ref_get() {
    let m = Modifier::new();
    let _get = m.get().copy_ty_late_ref();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref(Some(0_usize));
    m.set().copy_ty_late_ref(Some(1_usize));
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_late_ref() {
    let m = Modifier::new();
    // ok because `clone_ty_late_ref` is not initialized yet
    m.set().clone_ty_late_ref(Some(0.to_string()));
    assert_eq!(m.get().clone_ty_late_ref().as_ref().unwrap(), "0");
    assert_eq!(m.raw().clone_ty_late_ref().get().unwrap().as_ref().unwrap(), "0");
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_clone_ty_late_ref_get() {
    let m = Modifier::new();
    let _get = m.get().clone_ty_late_ref();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_clone_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref(Some(0.to_string()));
    m.set().clone_ty_late_ref(Some(1.to_string()));
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late_ref() {
    let m = Modifier::new();
    // ok because `class_ty_late_ref` is not initialized yet
    m.set().class_ty_late_ref(Some(Foo::new(0_usize)));
    assert_eq!(m.get().class_ty_late_ref().as_ref().unwrap().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late_ref().get().unwrap().as_ref().unwrap().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_class_ty_late_ref_get() {
    let m = Modifier::new();
    let _get = m.get().class_ty_late_ref();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().class_ty_late_ref(Some(Foo::new(0_usize)));
    m.set().class_ty_late_ref(Some(Foo::new(1_usize)));
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_ref_mut() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref_mut(Some(0_usize));
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 0_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &0_usize);
    m.set().copy_ty_late_ref_mut(Some(1_usize));
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 1_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &1_usize);
    let value = m.replace().copy_ty_late_ref_mut(Some(2_usize));
    assert_eq!(value.as_ref().unwrap(), &Some(1_usize));
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 2_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &2_usize);
    m.update().copy_ty_late_ref_mut(|v| Some(v.as_ref()?.as_ref()? + 1_usize));
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 3_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &3_usize);
    *m.get_mut().copy_ty_late_ref_mut() = Some(4_usize);
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 4_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &4_usize);
    let value = m.replace_with().copy_ty_late_ref_mut(|v| Some(v.as_ref()?.as_ref()? + 1_usize));
    assert_eq!(value.as_ref().unwrap(), &Some(4_usize));
    assert_eq!(m.get().copy_ty_late_ref_mut().unwrap(), 5_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().unwrap().as_ref().unwrap(), &5_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_copy_ty_late_ref_mut_get() {
    let m = Modifier::new();
    let _get = m.get().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_copy_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref_mut(Some(0_usize));
    let _get = m.get().copy_ty_late_ref_mut();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already mutably borrowed")]
fn test_copy_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref_mut(Some(0_usize));
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
    let _get = m.get().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_copy_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref_mut(Some(0_usize));
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_late_ref_mut() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref_mut(Some(0.to_string()));
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "0");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "0");
    m.set().clone_ty_late_ref_mut(Some(1.to_string()));
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "1");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "1");
    let value = m.replace().clone_ty_late_ref_mut(Some(2.to_string()));
    assert_eq!(value.as_ref().unwrap().as_deref(), Some("1"));
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "2");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "2");
    m.update().clone_ty_late_ref_mut(|v| Some(v.as_mut()?.as_mut()?.replace("2", "3")));
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "3");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "3");
    *m.get_mut().clone_ty_late_ref_mut() = Some(4.to_string());
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "4");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "4");
    let value = m.replace_with().clone_ty_late_ref_mut(|v| Some(v.as_mut()?.as_mut()?.replace("4", "5")));
    assert_eq!(value.as_ref().unwrap().as_deref(), Some("4"));
    assert_eq!(m.get().clone_ty_late_ref_mut().as_ref().unwrap(), "5");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap(), "5");
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_clone_ty_late_ref_mut_get() {
    let m = Modifier::new();
    let _get = m.get().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_clone_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref_mut(Some(0.to_string()));
    let _get = m.get().clone_ty_late_ref_mut();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already mutably borrowed")]
fn test_clone_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref_mut(Some(0.to_string()));
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
    let _get = m.get().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_clone_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref_mut(Some(0.to_string()));
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late_ref_mut() {
    let m = Modifier::new();
    m.set().class_ty_late_ref_mut(Some(Foo::new(0_usize)));
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 0_usize);
    m.set().class_ty_late_ref_mut(Some(Foo::new(1_usize)));
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 1_usize);
    let value = m.replace().class_ty_late_ref_mut(Some(Foo::new(2_usize)));
    assert_eq!(value.as_ref().unwrap().as_ref().unwrap().get().foo(), 1_usize);
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 2_usize);
    m.update().class_ty_late_ref_mut(|v| Some(Foo::new(v.as_ref()?.as_ref()?.get().foo() + 1_usize)));
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 3_usize);
    *m.get_mut().class_ty_late_ref_mut() = Some(Foo::new(4_usize));
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 4_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 4_usize);
    let value = m.replace_with().class_ty_late_ref_mut(|v| Some(Foo::new(v.as_ref()?.as_ref()?.get().foo() + 1_usize)));
    assert_eq!(value.as_ref().unwrap().as_ref().unwrap().get().foo(), 4_usize);
    assert_eq!(m.get().class_ty_late_ref_mut().as_ref().unwrap().get().foo(), 5_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().as_ref().unwrap().get().foo(), 5_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field not initialized")]
fn test_class_ty_late_ref_mut_get() {
    let m = Modifier::new();
    let _get = m.get().class_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_class_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    m.set().class_ty_late_ref_mut(Some(Foo::new(0_usize)));
    let _get = m.get().class_ty_late_ref_mut();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already mutably borrowed")]
fn test_class_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    m.set().class_ty_late_ref_mut(Some(Foo::new(0_usize)));
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
    let _get = m.get().class_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_class_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    m.set().class_ty_late_ref_mut(Some(Foo::new(0_usize)));
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
}
