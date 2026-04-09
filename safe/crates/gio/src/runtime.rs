#![allow(non_camel_case_types)]

use crate::ffi::GType;

pub unsafe extern "C" fn g_file_get_type_impl() -> GType {
    0x2001
}

pub unsafe extern "C" fn g_unix_fd_list_get_type_impl() -> GType {
    0x2002
}
