#[class]
type A = class<{
    pub fn new() -> Self {
        Self {}
    }
    pub fn f(&self) {
        println!("A::f");
    }
}>;

#[class]
type I = interface<{
    pub fn i(&self);
}>;

#[class(implements(I))]
type M1 = mixin<{
    pub fn g(&self) {
        println!("M1::g");
    }
    #[method(override(I))]
    pub fn i(&self) {
        println!("M1::i");
    }
}>;

#[class(on(#[mixin] M1))]
type M2 = mixin<{
    #[method(override(M1))]
    pub fn g(&self) {
        super.g();
        println!("M2::g");
    }
    pub fn h(&self) {
        println!("M2::h");
    }
    #[method(override(I))]
    pub fn i(&self) {
        println!("M2::i");
    }
}>;

#[class(extends(A), with(M1, M2))]
type B = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
    #[method(override(A))]
    pub fn f(&self) {
        super.f();
        println!("B::f");
    }
    #[method(override(M1))]
    pub fn g(&self) {
        super.g();
        println!("B::g");
    }
    #[method(override(M2))]
    pub fn h(&self) {
        super.h();
        println!("B::h");
    }
}>;
