#[class(implements(EqHash))]
pub type MyEqClass = class<
    {
        let x: u32;

        pub fn new(x: u32) -> Self {
            Self { x }
        }

        #[method(override(EqHash))]
        pub fn eq(&self, other: &EqHash) -> bool {
            other
                .downcast_ref::<MyEqClass>()
                .is_ok_and(|other| self.get().x() == other.get().x())
        }

        #[method(override(EqHash))]
        pub fn hash(&self, state: &mut (dyn core::hash::Hasher + '_)) {
            state.write_u32(self.get().x());
        }
    },
>;
