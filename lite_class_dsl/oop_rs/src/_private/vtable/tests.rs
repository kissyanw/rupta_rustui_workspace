#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::mem::offset_of;

use crate::__private::Dummy;
use crate::__private::dummy::DummyVtable;
use crate::__private::vtable::type_info::VtableOffsetEntry;
use crate::class::HasSuper;
use crate::write_vtable;

use super::*;

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct __VBuildContext<__Self> {
    __self: PhantomData<__Self>,
    widget: Option<fn(&__Self)>,
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct __VElement<__Self> {
    __self: PhantomData<Self>,
    __BuildContext: __VBuildContext<__Self>,
    rebuild: Option<fn(&__Self)>,
    perform_rebuild: Option<fn(&__Self)>,
    mount: Option<fn(&__Self)>,
    mark_needs_build: Option<fn(&__Self)>,
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct __VComponentElement<__Self> {
    __super: __VElement<__Self>,
    __self: PhantomData<__Self>,
    pub first_build: Option<fn(&__Self)>,
    pub build: Option<fn(&__Self)>,
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct __VRootElementMixin<__Self> {
    __self: PhantomData<__Self>,
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct __VMRootElementMixin<__Super, __Self> {
    __super: __Super,
    __self: __VRootElementMixin<__Self>,
}

#[derive(Default, Clone, Copy)]
pub struct __VRootElement<__Self> {
    __super: __VMRootElementMixin<__VElement<__Self>, __Self>,
    __self: PhantomData<__Self>,
}

#[test]
#[allow(non_snake_case, dead_code, non_local_definitions)]
#[cfg_attr(miri, ignore = "const `fn_addr_eq`")]
fn test_read_write_vtable() {
    impl<__Self: Copy + Default> Vtable for __VBuildContext<__Self> {
        const TYPE: VtableTypeInfo<'static> =
            VtableTypeInfo::Interface(&InterfaceVtableTypeInfo::new("BuildContext"));
    }

    impl<__Self: Copy + Default> Vtable for __VElement<__Self> {
        const TYPE: VtableTypeInfo<'static> =
            VtableTypeInfo::Class(&ClassVtableTypeInfo::new("Element").with_interfaces(&[
                VtableOffsetEntry::new(
                    __VBuildContext::<__Self>::INTERFACE_TYPE,
                    offset_of!(Self, __BuildContext),
                ),
            ]));
    }

    impl<__Self: Copy + Default> ClassVtable for __VElement<__Self> {}

    impl<__Self: Copy + Default> __VElement<__Self> {
        fn new() -> Self {
            let mut vtable = Self::default();
            vtable.__override();
            vtable
        }

        fn __override(&mut self) {
            write_vtable!( (self as BuildContext =>__VBuildContext<__Self>).widget = Self::widget);
            write_vtable!( (self as Element =>__VElement<__Self>).mark_needs_build = Self::mark_needs_build);
            write_vtable!( (self as Element =>__VElement<__Self>).mount = Self::mount);
            write_vtable!( (self as Element =>__VElement<__Self>).perform_rebuild = Self::perform_rebuild);
            write_vtable!( (self as Element =>__VElement<__Self>).rebuild = Self::rebuild);
        }
        fn widget(__self: &__Self) {}
        fn rebuild(__self: &__Self) {}
        fn perform_rebuild(__self: &__Self) {}
        fn mount(__self: &__Self) {}
        fn mark_needs_build(__self: &__Self) {}
    }

    impl<__Self: Copy + Default> Vtable for __VRootElementMixin<__Self> {
        const TYPE: VtableTypeInfo<'static> =
            VtableTypeInfo::Mixin(&MixinVtableTypeInfo::new("RootElementMixin"));
    }

    impl<__Self: Copy + Default> MixinVtable for __VRootElementMixin<__Self> {}

    impl<__Super: Vtable, __Self: Copy + Default> Vtable for __VMRootElementMixin<__Super, __Self> {
        const TYPE: VtableTypeInfo<'_> =
            VtableTypeInfo::MixinInstance(&MixinInstanceVtableTypeInfo::new(
                __Super::TYPE,
                __VRootElementMixin::<__Self>::MIXIN_TYPE,
                offset_of!(Self, __self),
            ));
    }

    impl<__Super: Vtable, __Self: Copy + Default> __VMRootElementMixin<__Super, __Self> {
        fn new(__super: __Super) -> Self {
            let mut vtable = Self {
                __super,
                __self: __VRootElementMixin {
                    __self: PhantomData,
                },
            };
            vtable.__override();
            vtable
        }

        fn __override(&mut self) {
            write_vtable! ( (self as Element =>__VElement<__Self>).mount = Self::mount);
        }
        fn mount(__self: &__Self) {}
    }

    impl<__Self: Copy + Default> Vtable for __VRootElement<__Self> {
        const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
            &ClassVtableTypeInfo::new("RootElement")
                .with_super(__VMRootElementMixin::<__VElement<__Self>, __Self>::TYPE),
        );
    }

    impl<__Self: Copy + Default> ClassVtable for __VRootElement<__Self> {}

    impl<__Self: Copy + Default> HasSuper for __VRootElement<__Self> {
        type Super = __VElement<__Self>;
    }

    impl<__Self: Copy + Default> __VRootElement<__Self> {
        fn new() -> Self {
            let mut vtable = Self {
                __super: __VMRootElementMixin::<__VElement<__Self>, __Self>::new(__VElement::<
                    __Self,
                >::new(
                )),
                __self: PhantomData,
            };
            vtable.__override();
            vtable
        }
        fn __override(&mut self) {
            write_vtable! ( (self as Element =>__VElement<__Self>).mount = Self::mount);
        }
        fn mount(__self: &__Self) {}
    }

    macro_rules! test_entries {
        ($vtable:ident { $( $Vtable:ident::$method:ident: $ty:ty ),* $(,)? }) => {
            let vtable = $vtable::<()>::new();
            $(
                assert!(std::ptr::fn_addr_eq(
                    cast_vtable::<_, $Vtable<()>>(&vtable).unwrap().$method.unwrap() as $ty,
                    $vtable::<()>::$method as $ty,
                ))
            )*
        };
    }
    test_entries!(__VRootElement { __VElement::mount: fn(_) -> _ });
}

#[test]
fn test_vtable_offset_of() {
    let build_context = InterfaceVtableTypeInfo::new("BuildContext");
    let element_interfaces = [VtableOffsetEntry::new(
        &build_context,
        offset_of!(__VElement<Dummy>, __BuildContext),
    )];
    let element = ClassVtableTypeInfo::new("Element").with_interfaces(&element_interfaces);
    let component_element =
        ClassVtableTypeInfo::new("ComponentElement").with_super(VtableTypeInfo::Class(&element));
    let root_element_mixin_interfaces = [VtableOffsetEntry::new(
        element.as_interface(),
        offset_of!(__VElement<Dummy>, __BuildContext),
    )];
    let root_element_mixin = MixinVtableTypeInfo::new("RootElementMixin")
        .with_interfaces(&root_element_mixin_interfaces);
    let root_element_mixin_instance = MixinInstanceVtableTypeInfo::new(
        VtableTypeInfo::Class(&element),
        &root_element_mixin,
        offset_of!(__VMRootElementMixin<__VElement<Dummy>, Dummy>, __self),
    );
    let root_element = ClassVtableTypeInfo::new("RootElement")
        .with_super(VtableTypeInfo::MixinInstance(&root_element_mixin_instance));

    let build_context = build_context.into_vtable();
    let element = element.into_vtable();
    let component_element = component_element.into_vtable();
    let root_element_mixin = root_element_mixin.into_vtable();
    let root_element_mixin_instance = root_element_mixin_instance.into_vtable();
    let root_element = root_element.into_vtable();

    assert_eq!(build_context.offset_of(build_context), Some(0));
    assert_eq!(element.offset_of(build_context), Some(0));
    assert_eq!(component_element.offset_of(build_context), Some(0));
    assert_eq!(component_element.offset_of(element), Some(0));
    assert_eq!(root_element_mixin.offset_of(build_context), Some(0));
    assert_eq!(root_element_mixin.offset_of(element), Some(0));
    assert_eq!(root_element_mixin.offset_of(root_element_mixin), Some(0));
    assert_eq!(
        root_element_mixin_instance.offset_of(build_context),
        Some(0)
    );
    assert_eq!(root_element_mixin_instance.offset_of(element), Some(0));
    assert_eq!(
        root_element_mixin_instance.offset_of(root_element_mixin),
        Some(40),
    );
    assert_eq!(root_element.offset_of(build_context), Some(0));
    assert_eq!(root_element.offset_of(element), Some(0));
    assert_eq!(root_element.offset_of(root_element_mixin), Some(40));
}

// Simulate the diamond inheritance structure:
// Listenable (interface)
//     ^           ^
//     |           |
// ChangeNotifier  PipelineManifold (interface)
//   (mixin)           ^
//     ^               |
//     |               |
//     +---------------+
//             |
//   BindingPipelineManifold (class)

// Define Listenable interface vtable
#[repr(C)]
#[derive(Default, Copy, Clone)]
struct __VListenable<__Self> {
    __self: PhantomData<__Self>,
    add_listener: Option<fn(&__Self, fn())>,
    remove_listener: Option<fn(&__Self, fn())>,
}

// Define PipelineManifold interface vtable (implements Listenable)
#[repr(C)]
#[derive(Default, Copy, Clone)]
struct __VPipelineManifold<__Self> {
    __Listenable: __VListenable<__Self>,
    __self: PhantomData<__Self>,
    semantics_enabled: Option<fn(&__Self) -> bool>,
    request_visual_update: Option<fn(&__Self)>,
}

// Define ChangeNotifier mixin vtable (implements Listenable)
#[repr(C)]
#[derive(Default, Copy, Clone)]
struct __VChangeNotifier<__Self> {
    __Listenable: __VListenable<__Self>,
    __self: PhantomData<__Self>,
    has_listeners: Option<fn(&__Self) -> bool>,
    dispose: Option<fn(&__Self)>,
    notify_listeners: Option<fn(&__Self)>,
}

// Define MixinInstance vtable for ChangeNotifier
#[repr(C)]
#[derive(Default, Copy, Clone)]
struct __VMixinInstanceChangeNotifier<__Super, __Self> {
    __super: __Super,
    __self: __VChangeNotifier<__Self>,
}

// Define BindingPipelineManifold class vtable
#[repr(C)]
#[derive(Copy, Clone)]
struct __VBindingPipelineManifold<__Self> {
    __super: __VMixinInstanceChangeNotifier<DummyVtable, __Self>,
    __PipelineManifold: __VPipelineManifold<__Self>,
    __self: PhantomData<__Self>,
}

#[test]
fn test_vtable_offset_of_mixin_only() {
    // Test case: class with only mixin, no extends (like BindingPipelineManifold)
    // Structure: BindingPipelineManifold with(ChangeNotifier)
    // The vtable structure should be:
    // - __super: MixinInstance<Dummy, ChangeNotifier>
    // - __self: PhantomData

    // Define Dummy class (base class when no extends)
    let dummy = ClassVtableTypeInfo::new("Dummy");

    // Define ChangeNotifier mixin vtable (implements Listenable)
    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    struct __VChangeNotifier<__Self> {
        __self: PhantomData<__Self>,
        has_listeners: Option<fn(&__Self) -> bool>,
        dispose: Option<fn(&__Self)>,
        notify_listeners: Option<fn(&__Self)>,
    }

    // Define MixinInstance vtable for ChangeNotifier
    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    struct __VMixinInstanceChangeNotifier<__Super, __Self> {
        __super: __Super,
        __self: __VChangeNotifier<__Self>,
    }

    // Define BindingPipelineManifold class vtable
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct __VBindingPipelineManifold<__Self> {
        __super: __VMixinInstanceChangeNotifier<DummyVtable, __Self>,
    }

    // Define ChangeNotifier mixin
    let change_notifier = MixinVtableTypeInfo::new("ChangeNotifier");

    // Define MixinInstance vtable: wraps Dummy with ChangeNotifier
    // This is what __super field points to
    let mixin_instance = MixinInstanceVtableTypeInfo::new(
        dummy.into_vtable(),
        &change_notifier,
        offset_of!(__VMixinInstanceChangeNotifier<DummyVtable, DummyVtable>, __self), // offset of mixin within the mixin instance vtable
    );

    // Define BindingPipelineManifold class
    // Its __super field is the mixin instance, but TYPE doesn't record this
    // because with_super only accepts ClassVtableTypeInfo
    let binding_pipeline_manifold = ClassVtableTypeInfo::new("BindingPipelineManifold")
        .with_super(VtableTypeInfo::MixinInstance(&mixin_instance));

    let change_notifier = change_notifier.into_vtable();
    let mixin_instance = mixin_instance.into_vtable();
    let binding_pipeline_manifold = binding_pipeline_manifold.into_vtable();

    // Test: Can we find ChangeNotifier from BindingPipelineManifold?
    // This should work through the __super field at runtime, but TYPE doesn't know about it
    assert_eq!(
        binding_pipeline_manifold.offset_of(change_notifier),
        Some(0)
    );

    // Test: Can we find ChangeNotifier from the mixin instance?
    assert_eq!(mixin_instance.offset_of(change_notifier), Some(0));
}

#[test]
fn test_vtable_offset_of_diamond_inheritance() {
    // Test case: Diamond inheritance with mixin
    // Structure:
    //   BindingPipelineManifold
    //     -> ChangeNotifier (mixin) -> Listenable
    //     -> PipelineManifold (interface) -> Listenable
    //
    // This creates a diamond: BindingPipelineManifold has two paths to Listenable

    // Define Listenable interface
    let listenable = InterfaceVtableTypeInfo::new("Listenable");

    // Define ChangeNotifier mixin that implements Listenable
    let change_notifier_interfaces = [VtableOffsetEntry::new(
        &listenable,
        offset_of!(__VChangeNotifier<DummyVtable>, __Listenable),
    )];
    let change_notifier =
        MixinVtableTypeInfo::new("ChangeNotifier").with_interfaces(&change_notifier_interfaces);

    // Define PipelineManifold interface that extends Listenable
    let pipeline_manifold_interfaces = [VtableOffsetEntry::new(
        &listenable,
        offset_of!(__VPipelineManifold<DummyVtable>, __Listenable),
    )];
    let pipeline_manifold = InterfaceVtableTypeInfo::new("PipelineManifold")
        .with_interfaces(&pipeline_manifold_interfaces);

    // Define Dummy class
    let dummy = ClassVtableTypeInfo::new("Dummy");
    let dummy = dummy.into_vtable();

    // Define MixinInstance: Dummy + ChangeNotifier
    let mixin_instance = MixinInstanceVtableTypeInfo::new(
        dummy,
        &change_notifier,
        offset_of!(__VMixinInstanceChangeNotifier<DummyVtable, DummyVtable>, __self),
    );

    let pipeline_manifold_entries = [VtableOffsetEntry::new(
        &pipeline_manifold,
        offset_of!(__VBindingPipelineManifold<DummyVtable>, __PipelineManifold),
    )];

    let binding_pipeline_manifold = ClassVtableTypeInfo::new("BindingPipelineManifold")
        .with_super(VtableTypeInfo::MixinInstance(&mixin_instance))
        .with_interfaces(&pipeline_manifold_entries);

    let listenable = listenable.into_vtable();
    let change_notifier = change_notifier.into_vtable();
    let _mixin_instance = mixin_instance.into_vtable();
    let pipeline_manifold = pipeline_manifold.into_vtable();
    let binding_pipeline_manifold = binding_pipeline_manifold.into_vtable();

    // Test 1: Can we find ChangeNotifier?
    assert_eq!(
        binding_pipeline_manifold.offset_of(change_notifier),
        Some(0)
    );

    // Test 2: Can we find PipelineManifold?
    assert_eq!(
        binding_pipeline_manifold.offset_of(pipeline_manifold),
        Some(40)
    );

    // Test 3: Can we find Listenable? (Diamond inheritance)
    // There are two paths to Listenable:
    // 1. Through ChangeNotifier (offset 0) - in mixins
    // 2. Through PipelineManifold (offset 8) - in interfaces
    // After fix: offset_of should return the minimum offset (0)
    assert_eq!(binding_pipeline_manifold.offset_of(listenable), Some(0));

    // Test 4: Can we find the next Listenable using next_offset_of?
    // After finding the one at offset 0, we should find the one at offset 8
    assert_eq!(
        binding_pipeline_manifold.next_offset_of(listenable, 0),
        Some(40)
    );

    // Test 5: Verify no more Listenable after offset 8
    assert_eq!(
        binding_pipeline_manifold.next_offset_of(listenable, 8),
        Some(40)
    );

    // Test 5: Verify no more Listenable after offset 40
    assert_eq!(
        binding_pipeline_manifold.next_offset_of(listenable, 40),
        None,
    );

    // Test 6: Can we find Listenable through ChangeNotifier?
    // First find ChangeNotifier, then find Listenable within it
    assert_eq!(
        binding_pipeline_manifold.offset_of(change_notifier),
        Some(0)
    );

    // Now find Listenable within ChangeNotifier
    assert_eq!(change_notifier.offset_of(listenable), Some(0));
}

#[test]
#[allow(non_snake_case, dead_code, non_local_definitions)]
#[cfg_attr(miri, ignore = "const `fn_addr_eq`")]
fn test_diamond_inheritance_vtable_override() {
    impl<__Self: Copy + Default> Vtable for __VListenable<__Self> {
        const TYPE: VtableTypeInfo<'static> =
            VtableTypeInfo::Interface(&InterfaceVtableTypeInfo::new("Listenable"));
    }

    impl<__Self: Copy + Default> Vtable for __VChangeNotifier<__Self> {
        const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Mixin(
            &MixinVtableTypeInfo::new("ChangeNotifier").with_interfaces(&[VtableOffsetEntry::new(
                __VListenable::<__Self>::INTERFACE_TYPE,
                offset_of!(Self, __Listenable),
            )]),
        );
    }

    impl<__Self: Copy + Default> MixinVtable for __VChangeNotifier<__Self> {}

    impl<__Self: Copy + Default> __VChangeNotifier<__Self> {
        const MIXIN_TYPE: &'static MixinVtableTypeInfo<'static> = match Self::TYPE {
            VtableTypeInfo::Mixin(m) => m,
            _ => panic!("expected mixin"),
        };

        fn new() -> Self {
            let mut vtable = Self::default();
            vtable.__override();
            vtable
        }

        fn __override(&mut self) {
            // Override Listenable methods
            write_vtable!((self as Listenable => __VListenable<__Self>).add_listener = Self::add_listener);
            write_vtable!((self as Listenable => __VListenable<__Self>).remove_listener = Self::remove_listener);
            // Own methods
            write_vtable!(self.has_listeners = Self::has_listeners);
            write_vtable!(self.dispose = Self::dispose);
            write_vtable!(self.notify_listeners = Self::notify_listeners);
        }

        fn add_listener(_self: &__Self, _f: fn()) {}
        fn remove_listener(_self: &__Self, _f: fn()) {}
        fn has_listeners(_self: &__Self) -> bool {
            true
        }
        fn dispose(_self: &__Self) {}
        fn notify_listeners(_self: &__Self) {}
    }

    impl<__Super: Copy, __Self: Copy + Default> Vtable
        for __VMixinInstanceChangeNotifier<__Super, __Self>
    {
        const TYPE: VtableTypeInfo<'static> =
            VtableTypeInfo::MixinInstance(&MixinInstanceVtableTypeInfo::new(
                VtableTypeInfo::Interface(&InterfaceVtableTypeInfo::new("Object")),
                __VChangeNotifier::<__Self>::MIXIN_TYPE,
                offset_of!(Self, __self),
            ));
    }

    impl<__Self: Copy + Default> Vtable for __VPipelineManifold<__Self> {
        const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Interface(
            &InterfaceVtableTypeInfo::new("PipelineManifold").with_interfaces(&[
                VtableOffsetEntry::new(
                    __VListenable::<__Self>::INTERFACE_TYPE,
                    offset_of!(Self, __Listenable),
                ),
            ]),
        );
    }

    impl<__Self: Copy + Default> __VPipelineManifold<__Self> {
        const INTERFACE_TYPE: &'static InterfaceVtableTypeInfo<'static> = match Self::TYPE {
            VtableTypeInfo::Interface(i) => i,
            _ => panic!("expected interface"),
        };
    }

    impl<__Self: Copy + Default> Default for __VBindingPipelineManifold<__Self> {
        fn default() -> Self {
            Self {
                __super: __VMixinInstanceChangeNotifier {
                    __super: DummyVtable,
                    __self: __VChangeNotifier::new(),
                },
                __PipelineManifold: __VPipelineManifold::default(),
                __self: PhantomData,
            }
        }
    }

    impl<__Self: Copy + Default> Vtable for __VBindingPipelineManifold<__Self> {
        const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
            &ClassVtableTypeInfo::new("BindingPipelineManifold")
                .with_super(__VMixinInstanceChangeNotifier::<DummyVtable, __Self>::TYPE)
                .with_interfaces(&[VtableOffsetEntry::new(
                    __VPipelineManifold::<__Self>::INTERFACE_TYPE,
                    offset_of!(Self, __PipelineManifold),
                )]),
        );
    }

    impl<__Self: Copy + Default> ClassVtable for __VBindingPipelineManifold<__Self> {}

    impl<__Self: Copy + Default> HasSuper for __VBindingPipelineManifold<__Self> {
        type Super = __VMixinInstanceChangeNotifier<DummyVtable, __Self>;
    }

    impl<__Self: Copy + Default> __VBindingPipelineManifold<__Self> {
        const CLASS_TYPE: &'static ClassVtableTypeInfo<'static> = match Self::TYPE {
            VtableTypeInfo::Class(c) => c,
            _ => panic!("expected class"),
        };

        fn new() -> Self {
            let mut vtable = Self::default();
            vtable.__override();
            vtable
        }

        fn __override(&mut self) {
            // Override PipelineManifold methods
            write_vtable!((self as PipelineManifold => __VPipelineManifold<__Self>).semantics_enabled = Self::semantics_enabled);
            write_vtable!((self as PipelineManifold => __VPipelineManifold<__Self>).request_visual_update = Self::request_visual_update);
        }

        fn semantics_enabled(_self: &__Self) -> bool {
            true
        }
        fn request_visual_update(_self: &__Self) {}
    }

    // Create the vtable
    let vtable = __VBindingPipelineManifold::<()>::new();

    // Test 1: Verify ChangeNotifier's Listenable methods are set
    let change_notifier_vtable = &vtable.__super.__self;
    assert!(
        change_notifier_vtable.__Listenable.add_listener.is_some(),
        "ChangeNotifier should have add_listener"
    );
    assert!(
        change_notifier_vtable
            .__Listenable
            .remove_listener
            .is_some(),
        "ChangeNotifier should have remove_listener"
    );

    // Test 2: Verify we can access Listenable through ChangeNotifier path
    let listenable_offset = __VBindingPipelineManifold::<()>::TYPE
        .offset_of(__VListenable::<()>::TYPE)
        .expect("Should find Listenable");
    println!(
        "Listenable offset in BindingPipelineManifold: {}",
        listenable_offset
    );

    // Cast to Listenable and verify methods are accessible
    let listenable_vtable: &__VListenable<()> = unsafe {
        &*core::ptr::from_ref(&vtable)
            .byte_add(listenable_offset)
            .cast()
    };
    assert!(
        listenable_vtable.add_listener.is_some(),
        "Listenable (through ChangeNotifier) should have add_listener"
    );
    assert!(
        listenable_vtable.remove_listener.is_some(),
        "Listenable (through ChangeNotifier) should have remove_listener"
    );

    // Test 3: Verify PipelineManifold's Listenable is also accessible
    // Note: PipelineManifold's Listenable methods should be None because
    // BindingPipelineManifold doesn't override them directly
    assert!(
        vtable
            .__PipelineManifold
            .__Listenable
            .add_listener
            .is_none(),
        "PipelineManifold's Listenable methods should be None (not overridden by BindingPipelineManifold)"
    );

    // Test 4: Verify PipelineManifold's own methods are set
    assert!(
        vtable.__PipelineManifold.semantics_enabled.is_some(),
        "PipelineManifold should have semantics_enabled"
    );
    assert!(
        vtable.__PipelineManifold.request_visual_update.is_some(),
        "PipelineManifold should have request_visual_update"
    );

    println!("Diamond inheritance vtable override test passed!");
}
