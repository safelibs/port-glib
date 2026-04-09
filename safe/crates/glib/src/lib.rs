#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
mod ffi;

pub mod abi {
    use super::ffi::*;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GList {
        pub data: gpointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GQueue {
        pub head: *mut GList,
        pub tail: *mut GList,
        pub length: guint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GArray {
        pub data: *mut gchar,
        pub len: guint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GByteArray {
        pub data: *mut guint8,
        pub len: guint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GString {
        pub str: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GOptionEntry {
        pub long_name: *const gchar,
        pub short_name: gchar,
        pub flags: gint,
        pub arg: gint,
        pub arg_data: gpointer,
        pub description: *const gchar,
        pub arg_description: *const gchar,
    }
}

pub const CRATE_ID: &str = "safe-glib";

pub fn bootstrap_marker() -> &'static str {
    "impl-safe-bootstrap"
}
