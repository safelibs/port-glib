pub type gint64 = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type guint = ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trace_mark(
    mut begin_time_nsec: gint64,
    mut duration_nsec: gint64,
    mut group: *const gchar,
    mut name: *const gchar,
    mut message_format: *const gchar,
    mut args: ...
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trace_define_int64_counter(
    mut group: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut description: *const ::core::ffi::c_char,
) -> guint {
    return -(1 as ::core::ffi::c_int) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_trace_set_int64_counter(mut id: guint, mut val: gint64) {}
