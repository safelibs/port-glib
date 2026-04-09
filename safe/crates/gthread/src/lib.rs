#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
mod ffi;

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
    "impl-safe-bootstrap"
}
