pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type guint = ::core::ffi::c_uint;
pub const GLIB_BINARY_AGE: ::core::ffi::c_int = 8000 as ::core::ffi::c_int;
pub const GLIB_INTERFACE_AGE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GLIB_MAJOR_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GLIB_MINOR_VERSION: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const GLIB_MICRO_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut safe_c2rust_glib_major_version: guint = GLIB_MAJOR_VERSION as guint;
#[no_mangle]
pub static mut safe_c2rust_glib_minor_version: guint = GLIB_MINOR_VERSION as guint;
#[no_mangle]
pub static mut safe_c2rust_glib_micro_version: guint = GLIB_MICRO_VERSION as guint;
#[no_mangle]
pub static mut safe_c2rust_glib_interface_age: guint = GLIB_INTERFACE_AGE as guint;
#[no_mangle]
pub static mut safe_c2rust_glib_binary_age: guint = GLIB_BINARY_AGE as guint;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_check_version(
    mut required_major: guint,
    mut required_minor: guint,
    mut required_micro: guint,
) -> *const gchar {
    let mut glib_effective_micro: gint = 100 as gint * GLIB_MINOR_VERSION + GLIB_MICRO_VERSION;
    let mut required_effective_micro: gint = (100 as guint)
        .wrapping_mul(required_minor)
        .wrapping_add(required_micro) as gint;
    if required_major > GLIB_MAJOR_VERSION as guint {
        return b"GLib version too old (major mismatch)\0" as *const u8 as *const gchar;
    }
    if required_major < GLIB_MAJOR_VERSION as guint {
        return b"GLib version too new (major mismatch)\0" as *const u8 as *const gchar;
    }
    if required_effective_micro < glib_effective_micro as ::core::ffi::c_int - GLIB_BINARY_AGE {
        return b"GLib version too new (micro mismatch)\0" as *const u8 as *const gchar;
    }
    if required_effective_micro > glib_effective_micro {
        return b"GLib version too old (micro mismatch)\0" as *const u8 as *const gchar;
    }
    return ::core::ptr::null::<gchar>();
}
