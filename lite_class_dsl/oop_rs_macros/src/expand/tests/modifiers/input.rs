#[class]
type Modifier = class<{
    let copy_ty: Option<usize> = Some(0_usize);
    let class_ty: Option<CRc<Foo>> = Some(Foo::new(0_usize));
    let mut copy_ty_mut: Option<usize> = Some(0_usize);
    let mut class_ty_mut: Option<CRc<Foo>> = Some(Foo::new(0_usize));
    let ref copy_ty_ref: Option<usize> = Some(0_usize);
    let ref clone_ty_ref: Option<String> = Some(0.to_string());
    let ref class_ty_ref: Option<CRc<Foo>> = Some(Foo::new(0_usize));
    let ref mut copy_ty_ref_mut: Option<usize> = Some(0_usize);
    let ref mut clone_ty_ref_mut: Option<String> = Some(0.to_string());
    let ref mut class_ty_ref_mut: Option<CRc<Foo>> = Some(Foo::new(0_usize));
    #[late] let copy_ty_late: Option<usize> = Some(self.f());
    #[late] let class_ty_late: Option<CRc<Foo>> = Some(Foo::new(self.f()));
    #[late] let mut copy_ty_late_mut: Option<usize> = Some(self.f());
    // FIXME: This is not supported yet
    // #[late] let mut class_ty_late_mut: Option<CRc<Foo>> = Some(Foo::new(self.f()));
    #[late] let ref copy_ty_late_ref: Option<usize> = Some(self.f());
    #[late] let ref clone_ty_lte_ref: Option<String> = Some(self.f().to_string());
    #[late] let ref class_ty_late_ref: Option<CRc<Foo>> = Some(Foo::new(self.f()));
    #[late] let ref mut copy_ty_late_ref_mut: Option<usize> = Some(self.f());
    #[late] let ref mut clone_ty_late_ref_mut: Option<String> = Some(self.f().to_string());
    #[late] let ref mut class_ty_late_ref_mut: Option<CRc<Foo>> = Some(Foo::new(self.f()));

    pub fn new() -> Self {
        println!("Modifier::constructor");
        Self { .. }
    }

    fn f(&self) -> usize {
        1_usize
    }
}>;
