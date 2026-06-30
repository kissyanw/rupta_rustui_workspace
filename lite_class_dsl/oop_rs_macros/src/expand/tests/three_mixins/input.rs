#[class]
type Animation = class<{}>;

#[class]
pub type AnimationLazyListenerMixin = mixin<{}>;

#[class]
pub type AnimationLocalListenersMixin = mixin<{}>;

#[class]
pub type AnimationLocalStatusListenersMixin = mixin<{}>;

#[class(
    extends(Animation),
    with(
        AnimationLazyListenerMixin,
        AnimationLocalListenersMixin,
        AnimationLocalStatusListenersMixin
    )
)]
pub type ProxyAnimation = class<{}>;
