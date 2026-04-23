use oop_rs::prelude::*;
use std::ops::Deref;

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
    let copy_ty: usize = 0_usize;
    let class_ty: CRc<Foo> = Foo::new(0_usize);
    let mut copy_ty_mut: usize = 0_usize;
    let mut class_ty_mut: CRc<Foo> = Foo::new(0_usize);
    let ref copy_ty_ref: usize = 0_usize;
    let ref clone_ty_ref: String = 0.to_string();
    let ref class_ty_ref: CRc<Foo> = Foo::new(0_usize);
    let ref mut copy_ty_ref_mut: usize = 0_usize;
    let ref mut clone_ty_ref_mut: String = 0.to_string();
    let ref mut class_ty_ref_mut: CRc<Foo> = Foo::new(0_usize);
    #[late] let copy_ty_late: usize = self.f();
    #[late] let class_ty_late: CRc<Foo> = Foo::new(self.f());
    #[late] let mut copy_ty_late_mut: usize = self.f();
    #[late] let mut class_ty_late_mut: CRc<Foo> = Foo::new(self.f());
    #[late] let ref copy_ty_late_ref: usize = self.f();
    #[late] let ref clone_ty_late_ref: String = self.f().to_string();
    #[late] let ref class_ty_late_ref: CRc<Foo> = Foo::new(self.f());
    #[late] let ref mut copy_ty_late_ref_mut: usize = self.f();
    #[late] let ref mut clone_ty_late_ref_mut: String = self.f().to_string();
    #[late] let ref mut class_ty_late_ref_mut: CRc<Foo> = Foo::new(self.f());

    pub fn new() -> Self {
        Self {}
    }

    fn f(&self) -> usize {
        0_usize
    }
}>;

#[test]
#[rustfmt::skip]
fn test_copy_ty() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty(), 0_usize);
    assert_eq!(m.raw().copy_ty(), &0_usize);
}

#[test]
#[rustfmt::skip]
fn test_class_ty() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_mut(), 0_usize);
    assert_eq!(m.raw().copy_ty_mut().get(), 0_usize);
    m.set().copy_ty_mut(1_usize);
    assert_eq!(m.get().copy_ty_mut(), 1_usize);
    assert_eq!(m.raw().copy_ty_mut().get(), 1_usize);
    let value = m.replace().copy_ty_mut(2_usize);
    assert_eq!(value, 1_usize);
    assert_eq!(m.get().copy_ty_mut(), 2_usize);
    assert_eq!(m.raw().copy_ty_mut().get(), 2_usize);
    m.update().copy_ty_mut(|v| v + 1_usize);
    assert_eq!(m.get().copy_ty_mut(), 3_usize);
    assert_eq!(m.raw().copy_ty_mut().get(), 3_usize);
}

#[test]
#[rustfmt::skip]
fn test_class_ty_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_mut().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_mut().get_cloned().get().foo(), 0_usize);
    m.set().class_ty_mut(Foo::new(1_usize));
    assert_eq!(m.get().class_ty_mut().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_mut().get_cloned().get().foo(), 1_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_ref() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_ref(), &0_usize);
    assert_eq!(m.raw().copy_ty_ref(), &0_usize);
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_ref() {
    let m = Modifier::new();
    assert_eq!(m.get().clone_ty_ref(), "0");
    assert_eq!(m.raw().clone_ty_ref(), "0");
}

#[test]
#[rustfmt::skip]
fn test_class_ty_ref() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_ref().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_ref().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &0_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &0_usize);
    m.set().copy_ty_ref_mut(1_usize);
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &1_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &1_usize);
    let value = m.replace().copy_ty_ref_mut(2_usize);
    assert_eq!(value, 1_usize);
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &2_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &2_usize);
    m.update().copy_ty_ref_mut(|v| v.deref() + 1_usize);
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &3_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &3_usize);
    *m.get_mut().copy_ty_ref_mut() = 4_usize;
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &4_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &4_usize);
    let value = m.replace_with().copy_ty_ref_mut(|v| v.deref() + 1_usize);
    assert_eq!(value, 4_usize);
    assert_eq!(m.get().copy_ty_ref_mut().deref(), &5_usize);
    assert_eq!(m.raw().copy_ty_ref_mut().borrow().deref(), &5_usize);
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"0");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"0");
    m.set().clone_ty_ref_mut(1.to_string());
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"1");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"1");
    let value = m.replace().clone_ty_ref_mut(2.to_string());
    assert_eq!(value, "1");
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"2");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"2");
    m.update().clone_ty_ref_mut(|v| v.deref().replace("2", "3"));
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"3");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"3");
    *m.get_mut().clone_ty_ref_mut() = 4.to_string();
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"4");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"4");
    let value = m.replace_with().clone_ty_ref_mut(|v| v.deref().replace("4", "5"));
    assert_eq!(value, "4");
    assert_eq!(m.get().clone_ty_ref_mut().deref(), &"5");
    assert_eq!(m.raw().clone_ty_ref_mut().borrow().deref(), &"5");
}

#[test]
#[rustfmt::skip]
fn test_class_ty_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 0_usize);
    m.set().class_ty_ref_mut(Foo::new(1_usize));
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 1_usize);
    let value = m.replace().class_ty_ref_mut(Foo::new(2_usize));
    assert_eq!(value.get().foo(), 1_usize);
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 2_usize);
    m.update().class_ty_ref_mut(|v| Foo::new(v.get().foo() + 1_usize));
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 3_usize);
    *m.get_mut().class_ty_ref_mut() = Foo::new(4_usize);
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 4_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 4_usize);
    let value = m.replace_with().class_ty_ref_mut(|v| Foo::new(v.get().foo() + 1_usize));
    assert_eq!(value.get().foo(), 4_usize);
    assert_eq!(m.get().class_ty_ref_mut().get().foo(), 5_usize);
    assert_eq!(m.raw().class_ty_ref_mut().borrow().get().foo(), 5_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late() {
    let m = Modifier::new();
    // ok because `copy_ty_late` is not initialized yet
    m.set().copy_ty_late(0_usize);
    assert_eq!(m.get().copy_ty_late(), 0_usize);
    assert_eq!(m.raw().copy_ty_late().get().unwrap(), &0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_get_and_set() {
    let m = Modifier::new();
    let _get = m.get().copy_ty_late();
    m.set().copy_ty_late(1_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_set_and_set() {
    let m = Modifier::new();
    m.set().copy_ty_late(0_usize);
    m.set().copy_ty_late(1_usize);
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late() {
    let m = Modifier::new();
    // ok because `class_ty_late` is not initialized yet
    m.set().class_ty_late(Foo::new(0_usize));
    assert_eq!(m.get().class_ty_late().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late().get().unwrap().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_get_and_set() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_late().get().foo(), 0_usize);
    m.set().class_ty_late(Foo::new(1_usize));
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_set_and_set() {
    let m = Modifier::new();
    m.set().class_ty_late(Foo::new(0_usize));
    m.set().class_ty_late(Foo::new(1_usize));
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_late_mut(), 0_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap(), 0_usize);
    m.set().copy_ty_late_mut(1_usize);
    assert_eq!(m.get().copy_ty_late_mut(), 1_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap(), 1_usize);
    let value = m.replace().copy_ty_late_mut(2_usize);
    assert_eq!(value.unwrap(), 1_usize);
    assert_eq!(m.get().copy_ty_late_mut(), 2_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap(), 2_usize);
    m.update().copy_ty_late_mut(|v| v.unwrap() + 1_usize);
    assert_eq!(m.get().copy_ty_late_mut(), 3_usize);
    assert_eq!(m.raw().copy_ty_late_mut().get().unwrap(), 3_usize);
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_late_mut().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late_mut().get_cloned().unwrap().get().foo(), 0_usize);
    m.set().class_ty_late_mut(Foo::new(1_usize));
    assert_eq!(m.get().class_ty_late_mut().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_late_mut().get_cloned().unwrap().get().foo(), 1_usize);
    let value = m.replace().class_ty_late_mut(Foo::new(2_usize));
    assert_eq!(value.unwrap().get().foo(), 1_usize);
    assert_eq!(m.get().class_ty_late_mut().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_late_mut().get_cloned().unwrap().get().foo(), 2_usize);
    m.update().class_ty_late_mut(|v| Foo::new(v.unwrap().get().foo() + 1_usize));
    assert_eq!(m.get().class_ty_late_mut().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_late_mut().get_cloned().unwrap().get().foo(), 3_usize);
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_ref() {
    let m = Modifier::new();
    // ok because `copy_ty_late_ref` is not initialized yet
    m.set().copy_ty_late_ref(0_usize);
    assert_eq!(m.get().copy_ty_late_ref(), &0_usize);
    assert_eq!(m.raw().copy_ty_late_ref().get().unwrap(), &0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_ref_get_and_set() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_late_ref(), &0_usize);
    m.set().copy_ty_late_ref(1_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_copy_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().copy_ty_late_ref(0_usize);
    m.set().copy_ty_late_ref(1_usize);
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_late_ref() {
    let m = Modifier::new();
    // ok because `clone_ty_late_ref` is not initialized yet
    m.set().clone_ty_late_ref(0.to_string());
    assert_eq!(m.get().clone_ty_late_ref(), "0");
    assert_eq!(m.raw().clone_ty_late_ref().get().unwrap(), "0");
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_clone_ty_late_ref_get_and_set() {
    let m = Modifier::new();
    assert_eq!(m.get().clone_ty_late_ref(), "0");
    m.set().clone_ty_late_ref(1.to_string());
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_clone_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().clone_ty_late_ref(0.to_string());
    m.set().clone_ty_late_ref(1.to_string());
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late_ref() {
    let m = Modifier::new();
    // ok because `class_ty_late_ref` is not initialized yet
    m.set().class_ty_late_ref(Foo::new(0_usize));
    assert_eq!(m.get().class_ty_late_ref().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late_ref().get().unwrap().get().foo(), 0_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_ref_get_and_set() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_late_ref().get().foo(), 0_usize);
    m.set().class_ty_late_ref(Foo::new(1_usize));
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "`#[late]` field already initialized")]
fn test_class_ty_late_ref_set_and_set() {
    let m = Modifier::new();
    m.set().class_ty_late_ref(Foo::new(0_usize));
    m.set().class_ty_late_ref(Foo::new(1_usize));
}

#[test]
#[rustfmt::skip]
fn test_copy_ty_late_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &0_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &0_usize);
    m.set().copy_ty_late_ref_mut(1_usize);
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &1_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &1_usize);
    let value = m.replace().copy_ty_late_ref_mut(2_usize);
    assert_eq!(value.as_ref().unwrap(), &1_usize);
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &2_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &2_usize);
    m.update().copy_ty_late_ref_mut(|v| v.unwrap() + 1_usize);
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &3_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &3_usize);
    *m.get_mut().copy_ty_late_ref_mut() = 4_usize;
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &4_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &4_usize);
    let value = m.replace_with().copy_ty_late_ref_mut(|v| v.unwrap() + 1_usize);
    assert_eq!(value.as_ref().unwrap(), &4_usize);
    assert_eq!(m.get().copy_ty_late_ref_mut().deref(), &5_usize);
    assert_eq!(m.raw().copy_ty_late_ref_mut().borrow().as_ref().unwrap(), &5_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_copy_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    let _get = m.get().copy_ty_late_ref_mut();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_copy_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
    let _get = m.get().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_copy_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
    let _get_mut = m.get_mut().copy_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
fn test_clone_ty_late_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "0");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "0");
    m.set().clone_ty_late_ref_mut(1.to_string());
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "1");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "1");
    let value = m.replace().clone_ty_late_ref_mut(2.to_string());
    assert_eq!(value.as_ref().unwrap(), "1");
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "2");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "2");
    m.update().clone_ty_late_ref_mut(|v| v.as_ref().unwrap().replace("2", "3"));
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "3");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "3");
    *m.get_mut().clone_ty_late_ref_mut() = 4.to_string();
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "4");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "4");
    let value = m.replace_with().clone_ty_late_ref_mut(|v| v.as_ref().unwrap().replace("4", "5"));
    assert_eq!(value.as_ref().unwrap(), "4");
    assert_eq!(m.get().clone_ty_late_ref_mut().deref(), "5");
    assert_eq!(m.raw().clone_ty_late_ref_mut().borrow().as_ref().unwrap(), "5");
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_clone_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    let _get = m.get().clone_ty_late_ref_mut();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_clone_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
    let _get = m.get().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_clone_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
    let _get_mut = m.get_mut().clone_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
fn test_class_ty_late_ref_mut() {
    let m = Modifier::new();
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 0_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 0_usize);
    m.set().class_ty_late_ref_mut(Foo::new(1_usize));
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 1_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 1_usize);
    let value = m.replace().class_ty_late_ref_mut(Foo::new(2_usize));
    assert_eq!(value.as_ref().unwrap().get().foo(), 1_usize);
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 2_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 2_usize);
    m.update().class_ty_late_ref_mut(|v| Foo::new(v.as_ref().unwrap().get().foo() + 1_usize));
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 3_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 3_usize);
    *m.get_mut().class_ty_late_ref_mut() = Foo::new(4_usize);
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 4_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 4_usize);
    let value = m.replace_with().class_ty_late_ref_mut(|v| Foo::new(v.as_ref().unwrap().get().foo() + 1_usize));
    assert_eq!(value.as_ref().unwrap().get().foo(), 4_usize);
    assert_eq!(m.get().class_ty_late_ref_mut().deref().get().foo(), 5_usize);
    assert_eq!(m.raw().class_ty_late_ref_mut().borrow().as_ref().unwrap().get().foo(), 5_usize);
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_class_ty_late_ref_mut_get_and_get_mut() {
    let m = Modifier::new();
    let _get = m.get().class_ty_late_ref_mut();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_class_ty_late_ref_mut_get_mut_and_get() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
    let _get = m.get().class_ty_late_ref_mut();
}

#[test]
#[rustfmt::skip]
#[should_panic(expected = "already borrowed")]
fn test_class_ty_late_ref_mut_get_mut_and_get_mut() {
    let m = Modifier::new();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
    let _get_mut = m.get_mut().class_ty_late_ref_mut();
}
