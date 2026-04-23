use crate::BUF;

use oop_rs::class;

#[class(abstract)]
type BuildContext = class<
    {
        fn new() -> Self {
            Self {}
        }
        pub fn widget(&self) -> CRc<Widget>;
    },
>;

#[class(abstract, extends(BuildContext))]
type Element = class<
    {
        let mut parent: Option<CWeak<Element>>;
        let mut widget: Option<CRc<Widget>>;
        let mut dirty: bool;

        pub fn new(widget: CRc<Widget>) -> Self {
            let self = Self {
                parent: None,
                widget: Some(widget),
                dirty: true,
                ..Super::new()
            };
            println!("Element");
            self
        }

        #[method(override(BuildContext))]
        pub fn widget(&self) -> CRc<Widget> {
            self.get().widget().unwrap()
        }
        fn rebuild(&self, force: bool) {
            println!("Element::rebuild");
            if self.get().dirty() || force {
                self.perform_rebuild();
            }
        }
        fn perform_rebuild(&self) {
            println!("Element::perform_rebuild");
            self.set().dirty(false);
        }
        fn mount(&self, parent: Option<CRc<Element>>) {
            println!("Element::mount");
            self.set().parent(parent);
        }
        #[allow(dead_code)]
        fn mark_needs_build(&self) {
            println!("Element::mark_needs_build");
            self.set().dirty(true);
            self.rebuild(false);
        }
    },
>;

#[class(abstract, extends(Element))]
type ComponentElement = class<
    {
        let mut parent: Option<CWeak<Element>> = None;
        let mut widget: Option<CRc<Widget>> = None;
        let mut dirty: bool = true;

        pub fn new(widget: CRc<Widget>) -> Self {
            let self = Self {
                ..Super::new(widget)
            };
            println!("ComponentElement");
            self
        }

        fn first_build(&self) {
            println!("ComponentElement::first_build");
            self.rebuild(false);
        }
        fn build(&self);
        #[method(override(Element))]
        fn perform_rebuild(&self) {
            println!("ComponentElement::perform_rebuild");
            IComponentElement::build(self);
            super.perform_rebuild();
        }
        #[method(override(Element))]
        fn mount(&self, parent: Option<CRc<Element>>) {
            println!("ComponentElement::mount");
            super.mount(parent);
            self.first_build();
        }
    },
>;

#[class(extends(ComponentElement))]
type StatefulElement = class<
    {
        let mut state: Option<CRc<State>>;

        fn new(widget: CRc<StatefulWidget>) -> Self {
            let state = widget.create_state();
            let self = Self {
                state: Some(state),
                ..Super::new(widget.clone())
            };
            println!("StatefulElement");
            self
        }
        fn state(&self) -> CRc<State> {
            self.get().state().unwrap()
        }
        #[method(override(ComponentElement))]
        fn build(&self) {
            self.state().build(self);
        }
        #[method(override(ComponentElement))]
        fn first_build(&self) {
            println!("StatefulElement::first_build");
            self.state().init_state();
            super.first_build();
        }
        #[method(override(Element))]
        fn perform_rebuild(&self) {
            println!("StatefulElement::perform_rebuild");
            super.perform_rebuild();
        }
        #[method(override(Element))]
        fn mount(&self, parent: Option<CRc<Element>>) {
            println!("StatefulElement::mount");
            super.mount(parent);
        }
    },
>;

#[class(abstract, extends(Object))]
type Widget = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        pub fn create_element(&self) -> CRc<Element>;
    },
>;

#[class(abstract, extends(Widget))]
type StatefulWidget = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        pub fn create_state(&self) -> CRc<State>;
        #[method(override(Widget))]
        pub fn create_element(&self) -> CRc<Element> {
            StatefulElement::new(self.to_rc())
        }
    },
>;

#[class(abstract)]
type State = class<
    {
        let mut widget: Option<CRc<StatefulWidget>>;
        let mut element: Option<CRc<Element>>;

        fn new() -> Self {
            Self {}
        }
        #[allow(dead_code)]
        pub fn widget(&self) -> CRc<Widget> {
            self.get().widget().unwrap()
        }
        pub fn init_state(&self) {
            println!("State::init_state");
        }
        pub fn build(&self, cx: &BuildContext) -> CRc<Widget>;
        #[allow(dead_code)]
        pub fn set_state(&self, f: &mut dyn FnMut()) {
            println!("State::set_state");
            f();
        }
    },
>;

#[class(on(Element))]
type RootElementMixin = mixin<
    {
        #[method(override(Element))]
        fn mount(&self, parent: Option<CRc<Element>>) {
            println!("RootElementMixin::mount");
            super.mount(parent);
        }
    },
>;

#[class(extends(Element), with(RootElementMixin))]
type RootElement = class<
    {
        fn new(widget: CRc<Widget>) -> Self {
            let self = Self {
                ..Super::new(widget)
            };
            println!("RootElement");
            self
        }

        #[method(override(Element))]
        fn mount(&self, parent: Option<CRc<Element>>) {
            println!("RootElement::mount");
            super.mount(parent);
            self.perform_rebuild();
        }

        #[method(override(Element))]
        fn perform_rebuild(&self) {
            println!("RootElement::perform_rebuild");
            super.perform_rebuild();
        }
    },
>;

#[class(extends(StatefulWidget))]
type GalleryPage = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        fn on_create(&self) {
            println!("GalleryPage::on_create");
        }
        #[method(override(StatefulWidget))]
        pub fn create_state(&self) -> CRc<State> {
            println!("GalleryPage::create_state");
            GalleryPageState::new()
        }
    },
>;

#[class(extends(State))]
type GalleryPageState = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(State))]
        pub fn init_state(&self) {
            println!("GalleryPageState::init_state");
            super.init_state();
        }
        #[method(override(State))]
        pub fn build(&self, cx: &BuildContext) -> CRc<Widget> {
            println!("GalleryPageState::build");
            cx.widget()
                .downcast_ref::<GalleryPage>()
                .unwrap()
                .on_create();
            MyWidget::new()
        }
    },
>;

#[class(extends(Widget))]
type MyWidget = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(Widget))]
        pub fn create_element(&self) -> CRc<Element> {
            MyElement::new(self.to_rc())
        }
    },
>;

#[class(extends(Element))]
type MyElement = class<
    {
        fn new(widget: CRc<MyWidget>) -> Self {
            Self {
                ..Super::new(widget)
            }
        }
    },
>;

#[class(extends(Widget))]
type RootWidget = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(Widget))]
        pub fn create_element(&self) -> CRc<Element> {
            RootElement::new(self.to_rc())
        }
    },
>;

#[test]
#[cfg_attr(miri, ignore = "known bug of `RcRef`")]
fn test() {
    BUF.take();
    let root = RootWidget::new().create_element();
    root.mount(None);
    GalleryPage::new().create_element().mount(Some(root));
    assert_eq!(BUF.take(), EXPECTED_OUTPUT);
}

const EXPECTED_OUTPUT: &[&str] = &[
    "Element",
    "RootElement",
    "RootElement::mount",
    "RootElementMixin::mount",
    "Element::mount",
    "RootElement::perform_rebuild",
    "Element::perform_rebuild",
    "GalleryPage::create_state",
    "Element",
    "ComponentElement",
    "StatefulElement",
    "StatefulElement::mount",
    "ComponentElement::mount",
    "Element::mount",
    "StatefulElement::first_build",
    "GalleryPageState::init_state",
    "State::init_state",
    "ComponentElement::first_build",
    "Element::rebuild",
    "StatefulElement::perform_rebuild",
    "ComponentElement::perform_rebuild",
    "GalleryPageState::build",
    "GalleryPage::on_create",
    "Element::perform_rebuild",
];
