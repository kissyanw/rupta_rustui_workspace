#[class(implements(Listenable))]
type ChangeNotifier = mixin<
    {
        let ref mut listeners: Vec<fn()>;

        pub fn has_listeners(&self) -> bool {
            println!(
                "ChangeNotifier::has_listeners, {} listeners",
                self.get().listeners().len()
            );
            !self.get().listeners().is_empty()
        }

        pub fn dispose(&self) {
            println!("ChangeNotifier::dispose");
            self.get_mut().listeners().clear();
        }

        #[method(override(Listenable))]
        pub fn add_listener(&self, f: fn()) {
            println!("ChangeNotifier::add_listener");
            self.get_mut().listeners().push(f);
        }

        #[method(override(Listenable))]
        pub fn remove_listener(&self, f: fn()) {
            println!("ChangeNotifier::remove_listener");
            self.get_mut()
                .listeners()
                .retain(|&l| !core::ptr::fn_addr_eq(l, f));
        }

        pub fn notify_listeners(&self) {
            println!("ChangeNotifier::notify_listeners");
            if self.has_listeners() {
                for listener in core::mem::take(&mut *self.get_mut().listeners()) {
                    listener();
                }
            }
        }
    },
>;

#[class(with(ChangeNotifier), implements(PipelineManifold))]
type BindingPipelineManifold = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(PipelineManifold))]
        pub fn semantics_enabled(&self) -> bool {
            println!("BindingPipelineManifold::semantics_enabled");
            true
        }

        #[method(override(PipelineManifold))]
        pub fn request_visual_update(&self) {
            if self.semantics_enabled() {
                println!("BindingPipelineManifold::request_visual_update");
                self.notify_listeners();
            }
        }
    },
>;
