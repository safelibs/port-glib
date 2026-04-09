#![allow(non_camel_case_types)]

use crate::ffi::GType;

pub unsafe extern "C" fn g_object_get_type_impl() -> GType {
    0x1001
}

pub unsafe extern "C" fn g_array_get_type_impl() -> GType {
    0x1002
}
