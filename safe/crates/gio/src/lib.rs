#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
mod ffi;

pub mod abi {
    use super::ffi::*;

    pub type GActionActivateFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer)>;
    pub type GActionChangeStateFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer)>;
    pub type GDBusInterfaceMethodCallFunc = GenericFn;
    pub type GDBusInterfaceGetPropertyFunc = GenericFn;
    pub type GDBusInterfaceSetPropertyFunc = GenericFn;
    pub type GDBusSubtreeEnumerateFunc = GenericFn;
    pub type GDBusSubtreeIntrospectFunc = GenericFn;
    pub type GDBusSubtreeDispatchFunc = GenericFn;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GActionEntry {
        pub name: *const gchar,
        pub activate: GActionActivateFunc,
        pub parameter_type: *const gchar,
        pub state: *const gchar,
        pub change_state: GActionChangeStateFunc,
        pub padding: [gsize; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GDBusInterfaceVTable {
        pub method_call: GDBusInterfaceMethodCallFunc,
        pub get_property: GDBusInterfaceGetPropertyFunc,
        pub set_property: GDBusInterfaceSetPropertyFunc,
        pub padding: [gpointer; 8],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GDBusSubtreeVTable {
        pub enumerate: GDBusSubtreeEnumerateFunc,
        pub introspect: GDBusSubtreeIntrospectFunc,
        pub dispatch: GDBusSubtreeDispatchFunc,
        pub padding: [gpointer; 8],
    }
}

pub const CRATE_ID: &str = "safe-gio";

pub fn bootstrap_marker() -> &'static str {
    "impl-safe-bootstrap"
}
