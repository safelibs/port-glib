extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_random_int() -> guint32;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUuid {
    pub bytes: [guint8; 16],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_g_uuid_to_string(mut uuid: *const GUuid) -> *mut gchar {
    let mut bytes: *const guint8 = ::core::ptr::null::<guint8>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !uuid.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uuid != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    bytes = &raw const (*uuid).bytes as *const guint8;
    return g_strdup_printf(
        b"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\0" as *const u8
            as *const gchar,
        *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *bytes.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn safe_c2rust_uuid_parse_string(
    mut str: *const gchar,
    mut uuid: *mut GUuid,
) -> gboolean {
    let mut tmp: GUuid = GUuid { bytes: [0; 16] };
    let mut bytes: *mut guint8 = &raw mut tmp.bytes as *mut guint8;
    let mut i: gint = 0;
    let mut j: gint = 0;
    let mut hi: gint = 0;
    let mut lo: gint = 0;
    let mut expected_len: guint = 36 as guint;
    if strlen(str as *const ::core::ffi::c_char) != expected_len as size_t {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int as gint;
    j = 0 as ::core::ffi::c_int as gint;
    while i < 16 as ::core::ffi::c_int {
        if j == 8 as ::core::ffi::c_int
            || j == 13 as ::core::ffi::c_int
            || j == 18 as ::core::ffi::c_int
            || j == 23 as ::core::ffi::c_int
        {
            let fresh0 = j;
            j = j + 1;
            if *str.offset(fresh0 as isize) as ::core::ffi::c_int != '-' as i32 {
                return FALSE;
            }
        } else {
            let fresh1 = j;
            j = j + 1;
            hi = g_ascii_xdigit_value(*str.offset(fresh1 as isize));
            let fresh2 = j;
            j = j + 1;
            lo = g_ascii_xdigit_value(*str.offset(fresh2 as isize));
            if hi == -(1 as ::core::ffi::c_int) || lo == -(1 as ::core::ffi::c_int) {
                return FALSE;
            }
            let fresh3 = i;
            i = i + 1;
            *bytes.offset(fresh3 as isize) = (hi << 4 as ::core::ffi::c_int | lo) as guint8;
        }
    }
    if !uuid.is_null() {
        *uuid = tmp;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uuid_string_is_valid(mut str: *const gchar) -> gboolean {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_uuid_parse_string(str, ::core::ptr::null_mut::<GUuid>());
}
unsafe extern "C" fn safe_c2rust_uuid_set_version(mut uuid: *mut GUuid, mut version: guint) {
    let mut bytes: *mut guint8 = &raw mut (*uuid).bytes as *mut guint8;
    let ref mut fresh4 = *bytes.offset(6 as ::core::ffi::c_int as isize);
    *fresh4 = (*fresh4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as guint8;
    let ref mut fresh5 = *bytes.offset(6 as ::core::ffi::c_int as isize);
    *fresh5 = (*fresh5 as guint | version << 4 as ::core::ffi::c_int) as guint8;
    let ref mut fresh6 = *bytes.offset(8 as ::core::ffi::c_int as isize);
    *fresh6 = (*fresh6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as guint8;
    let ref mut fresh7 = *bytes.offset(8 as ::core::ffi::c_int as isize);
    *fresh7 = (*fresh7 as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as guint8;
}
unsafe extern "C" fn safe_c2rust_g_uuid_generate_v4(mut uuid: *mut GUuid) {
    let mut i: ::core::ffi::c_int = 0;
    let mut bytes: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut ints: *mut guint32 = ::core::ptr::null_mut::<guint32>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !uuid.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uuid != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    bytes = &raw mut (*uuid).bytes as *mut guint8;
    ints = bytes as *mut guint32;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *ints.offset(i as isize) = g_random_int();
        i += 1;
    }
    safe_c2rust_uuid_set_version(uuid, 4 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uuid_string_random() -> *mut gchar {
    let mut uuid: GUuid = GUuid { bytes: [0; 16] };
    safe_c2rust_g_uuid_generate_v4(&raw mut uuid);
    return safe_c2rust_g_uuid_to_string(&raw mut uuid);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_uuid_string_is_valid\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
