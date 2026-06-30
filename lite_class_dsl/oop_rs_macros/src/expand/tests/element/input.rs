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
        fn mount(&self, parent: Option<Weak<Element>>) {
            println!("Element::mount");
            self.set().parent(parent);
        }
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
            self.build();
            super.perform_rebuild();
        }
        #[method(override(Element))]
        fn mount(&self, parent: Option<Weak<Element>>) {
            println!("ComponentElement::mount");
            super.mount(parent);
            self.first_build();
        }
    },
>;

#[class(on(Element))]
type RootElementMixin = mixin<
    {
        #[method(override(Element))]
        fn mount(&self, parent: Option<Weak<Element>>) {
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
        fn mount(&self, parent: Option<Weak<Element>>) {
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
