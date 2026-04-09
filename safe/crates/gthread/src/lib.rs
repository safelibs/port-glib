#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
pub mod ffi;

mod compat;
mod runtime;

unsafe fn initialize_exports(handle: *mut core::ffi::c_void) {
    compat::initialize(handle);
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
    use super::ffi::*;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union GMutex {
        pub p: gpointer,
        pub i: [guint; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GRecMutex {
        pub p: gpointer,
        pub i: [guint; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GRWLock {
        pub p: gpointer,
        pub i: [guint; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GCond {
        pub p: gpointer,
        pub i: [guint; 2],
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum GOnceStatus {
        NotCalled = 0,
        Progress = 1,
        Ready = 2,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GOnce {
        pub status: GOnceStatus,
        pub retval: gpointer,
    }
}

pub const CRATE_ID: &str = "safe-gthread";

pub fn bootstrap_marker() -> &'static str {
    "impl-glib-core"
}
