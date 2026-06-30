#[class(implements(EqHash, Format))]
pub type Object = class<
    {
        pub fn new() -> Self {
            Self {}
        }
        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            #[cfg(debug_assertions)]
            core::fmt::Display::fmt(&IDowncast::__type_name(self), f)?;
            #[cfg(not(debug_assertions))]
            core::fmt::Write::write_str(f, "Object")?;

            core::fmt::Write::write_fmt(f, format_args!("({:p})", self))
        }
    },
>;
