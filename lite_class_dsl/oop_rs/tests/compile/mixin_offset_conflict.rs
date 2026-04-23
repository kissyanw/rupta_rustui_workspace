use oop_rs::prelude::*;

#[class]
pub type BindingBase = class<{}>;

#[class(on(BindingBase))]
pub type SchedulerBinding = mixin<{}>;

#[class(on(BindingBase, SchedulerBinding))]
pub type ServicesBinding = mixin<{}>;
