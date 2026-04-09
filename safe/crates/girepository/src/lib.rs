#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
pub mod ffi;

mod runtime;
pub mod exports;

pub mod abi {
    use super::ffi::*;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union GIArgument {
        pub v_boolean: gboolean,
        pub v_int8: i8,
        pub v_uint8: u8,
        pub v_int16: i16,
        pub v_uint16: u16,
        pub v_int32: i32,
        pub v_uint32: u32,
        pub v_int64: i64,
        pub v_uint64: u64,
        pub v_float: f32,
        pub v_double: f64,
        pub v_short: gshort,
        pub v_ushort: gushort,
        pub v_int: gint,
        pub v_uint: guint,
        pub v_long: glong,
        pub v_ulong: gulong,
        pub v_ssize: gssize,
        pub v_size: usize,
        pub v_string: *mut i8,
        pub v_pointer: gpointer,
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum GITypeTag {
        Void = 0,
        Boolean = 1,
        Int8 = 2,
        Uint8 = 3,
        Int16 = 4,
        Uint16 = 5,
        Int32 = 6,
        Uint32 = 7,
        Int64 = 8,
        Uint64 = 9,
        Float = 10,
        Double = 11,
        GType = 12,
        Utf8 = 13,
        Filename = 14,
        Array = 15,
        Interface = 16,
        GList = 17,
        GSList = 18,
        GHash = 19,
        Error = 20,
        Unichar = 21,
    }

    #[repr(i32)]
    #[derive(Copy, Clone)]
    pub enum GIArrayType {
        C = 0,
        Array = 1,
        PtrArray = 2,
        ByteArray = 3,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GIAttributeIter {
        pub data: *mut core::ffi::c_void,
        pub _dummy: [*mut core::ffi::c_void; 4],
    }
}

pub const CRATE_ID: &str = "safe-girepository";

pub fn bootstrap_marker() -> &'static str {
    "impl-safe-bootstrap"
}
