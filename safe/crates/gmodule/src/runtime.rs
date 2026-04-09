#![allow(non_camel_case_types)]

use crate::ffi::{gboolean, gchar, gint};

pub struct GModuleHandle {
    _opaque: usize,
}

pub unsafe extern "C" fn g_module_supported_impl() -> gboolean {
    1
}

pub unsafe extern "C" fn g_module_open_impl(_file_name: *const gchar, _flags: gint) -> *mut GModuleHandle {
    Box::into_raw(Box::new(GModuleHandle { _opaque: 1 }))
}

pub unsafe extern "C" fn g_module_close_impl(module: *mut GModuleHandle) -> gboolean {
    if module.is_null() {
        return 0;
    }
    drop(Box::from_raw(module));
    1
}
