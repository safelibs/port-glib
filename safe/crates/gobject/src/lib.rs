#![feature(c_variadic, extern_types)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_unsafe)]
#![allow(improper_ctypes)]

#[macro_use]
extern crate c2rust_bitfields;

#[path = "../../abi-support/src/ffi.rs"]
pub mod ffi;

pub mod object;
pub mod signal;
pub mod tools;
pub mod translated;
pub mod type_system;
pub mod value;

pub use object::*;
pub use signal::*;
pub use type_system::*;
pub use value::*;

pub mod abi {
    pub use crate::ffi::GType;
    pub use crate::ffi::*;
    pub use crate::object::{
        GObject, GObjectClass, GObjectConstructFunc, GObjectConstructParam, GObjectConstructedFunc,
        GObjectDispatchPropertiesChangedFunc, GObjectDisposeFunc, GObjectFinalizeFunc,
        GObjectGetPropertyFunc, GObjectNotifyFunc, GObjectSetPropertyFunc, GParamSpec,
        GParamSpecClass, GParamSpecFinalizeFunc, GParamSpecInstanceInitFunc,
        GParamSpecTypeInfo, GParamSpecValueIsValidFunc, GParamSpecValueSetDefaultFunc,
        GParamSpecValueValidateFunc, GParamSpecValuesCmpFunc,
    };
    pub use crate::signal::{
        GCClosure, GClosure, GClosureMarshal, GClosureNotify, GClosureNotifyData, GSignalFlags,
        GSignalInvocationHint, GSignalQuery,
    };
    pub use crate::type_system::{
        GBaseFinalizeFunc, GBaseInitFunc, GClassFinalizeFunc, GClassInitFunc, GInstanceInitFunc,
        GInterfaceFinalizeFunc, GInterfaceInfo, GInterfaceInitFunc, GTypeClass, GTypeFlags,
        GTypeFundamentalFlags, GTypeFundamentalInfo, GTypeInfo, GTypeInstance, GTypeInterface,
        GTypePlugin, GTypePluginClass, GTypePluginCompleteInterfaceInfo,
        GTypePluginCompleteTypeInfo, GTypePluginUnuse, GTypePluginUse, GTypeQuery,
        GTypeValueCollectFunc, GTypeValueFreeFunc, GTypeValueInitFunc, GTypeValueLCopyFunc,
        GTypeValuePeekPointerFunc, GTypeValueTable,
    };
    pub use crate::value::{GParamFlags, GTypeCValue, GValue, GValueArray, GValueTransform};
}

pub const CRATE_ID: &str = "safe-gobject";

pub fn bootstrap_marker() -> &'static str {
    "impl-gobject-rust"
}
