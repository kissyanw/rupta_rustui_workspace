use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Listenable = interface<
    {
        pub fn add_listener(&self, f: fn());
        pub fn remove_listener(&self, f: fn());
    },
>;

#[class(implements(Listenable))]
type PipelineManifold = interface<
    {
        pub fn semantics_enabled(&self) -> bool;
        pub fn request_visual_update(&self);
    },
>;

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

        #[allow(dead_code)]
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

static EXPECTED: &[&str] = &[
    // manifold.add_listener(listener1);
    "ChangeNotifier::add_listener",
    // manifold.add_listener(listener2);
    "ChangeNotifier::add_listener",
    // manifold.add_listener(listener3);
    "ChangeNotifier::add_listener",
    // manifold.remove_listener(listener2);
    "ChangeNotifier::remove_listener",
    // manifold.request_visual_update();
    "BindingPipelineManifold::semantics_enabled",
    "BindingPipelineManifold::request_visual_update",
    "ChangeNotifier::notify_listeners",
    #[cfg(miri)]
    "ChangeNotifier::has_listeners, 3 listeners",
    #[cfg(not(miri))]
    "ChangeNotifier::has_listeners, 2 listeners",
    "listener1",
    #[cfg(miri)]
    "listener2",
    "listener3",
];

#[test]
fn diamond_inheritance() {
    BUF.take();
    let manifold: CRc<PipelineManifold> = BindingPipelineManifold::new();
    macro_rules! listener {
        ($name:ident) => {
            fn $name() {
                println!(stringify!($name));
            }
        };
    }
    listener!(listener1);
    listener!(listener2);
    listener!(listener3);
    manifold.add_listener(listener1);
    manifold.add_listener(listener2);
    manifold.add_listener(listener3);
    manifold.remove_listener(listener2);
    manifold.request_visual_update();
    assert_eq!(BUF.take(), EXPECTED);
}
