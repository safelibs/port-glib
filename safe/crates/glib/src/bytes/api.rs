use std::ffi::c_char;

use crate::abi::GByteArray;
use crate::bridge;
use crate::ffi::{gsize, guint, guint8};

unsafe extern "C" {
    fn g_return_if_fail_warning(log_domain: *const c_char, pretty_function: *const c_char, expression: *const c_char);
}

#[unsafe(export_name = "g_byte_array_new_take")]
pub unsafe extern "C" fn byte_array_new_take(data: *mut guint8, len: gsize) -> *mut GByteArray {
    if len > guint::MAX as gsize {
        g_return_if_fail_warning(
            c"GLib".as_ptr(),
            c"g_byte_array_new_take".as_ptr(),
            c"len <= G_MAXUINT".as_ptr(),
        );
        return core::ptr::null_mut();
    }

    bridge::byte_array_new_take(data, len)
}
