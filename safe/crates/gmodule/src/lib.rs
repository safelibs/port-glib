#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
pub mod ffi;

mod module_api;
mod runtime;

unsafe fn initialize_exports(handle: *mut core::ffi::c_void) {
    module_api::initialize(handle);
}

extern "C" fn initialize_library() {
    unsafe {
        runtime::initialize(initialize_exports);
    }
}

#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static INIT_ARRAY: extern "C" fn() = initialize_library;

pub mod abi {
    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum GModuleFlags {
        BindLazy = 1 << 0,
        BindLocal = 1 << 1,
        BindMask = 0x03,
    }
}

pub const CRATE_ID: &str = "safe-gmodule";

pub fn bootstrap_marker() -> &'static str {
    "impl-glib-core"
}
