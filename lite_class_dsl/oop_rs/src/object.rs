#[crate::class(implements(Downcast))]
pub type EqHash = interface<
    {
        fn eq(&self, other: &EqHash) -> bool {
            core::ptr::addr_eq(self, other)
        }
        fn hash(&self, state: &mut (dyn core::hash::Hasher + '_)) {
            #[cfg(feature = "dyn-hash")]
            dyn_hash::DynHash::dyn_hash(&core::ptr::from_ref(self), state);
            #[cfg(not(feature = "dyn-hash"))]
            {
                let _ = state;
                unimplemented!("dyn-hash feature is not enabled");
            }
        }
    },
>;

impl PartialEq for EqHash {
    fn eq(&self, other: &Self) -> bool {
        IEqHash::eq(self, other)
    }
}

impl Eq for EqHash {}

impl core::hash::Hash for EqHash {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        IEqHash::hash(self, state)
    }
}

#[crate::class]
pub type Format = interface<
    {
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Write::write_fmt(f, format_args!("{:p}", self))
        }
    },
>;

impl core::fmt::Debug for Format {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        IFormat::fmt_debug(self, f)
    }
}

#[crate::class(implements(EqHash, Format))]
pub type Object = class<
    {
        pub fn new() -> Self {
            Self {}
        }
        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            #[cfg(debug_assertions)]
            core::fmt::Display::fmt(&IDowncast::ty(self).type_name(), f)?;
            #[cfg(not(debug_assertions))]
            core::fmt::Write::write_str(f, "Object")?;

            core::fmt::Write::write_fmt(f, format_args!("({:p})", self))
        }
    },
>;
