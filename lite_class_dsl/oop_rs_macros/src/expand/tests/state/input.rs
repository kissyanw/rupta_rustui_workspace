#[class(abstract)]
type State = class<
    {
        fn new() -> Self {
            Self {}
        }
        pub fn widget(&self) -> CRc<Widget>;
        pub fn init_state(&self) {
            println!("State::init_state");
        }
        pub fn build(&self, cx: &BuildContext) -> CRc<Widget>;
        pub fn set_state(&self, f: &mut dyn FnMut()) {
            println!("State::set_state");
            f();
        }
    },
>;
