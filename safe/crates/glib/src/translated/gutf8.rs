extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_convert_error_quark() -> GQuark;
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_try_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_strrstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_CONVERT_ERROR_EMBEDDED_NUL: C2RustUnnamed = 7;
pub const G_CONVERT_ERROR_NO_MEMORY: C2RustUnnamed = 6;
pub const G_CONVERT_ERROR_NOT_ABSOLUTE_PATH: C2RustUnnamed = 5;
pub const G_CONVERT_ERROR_BAD_URI: C2RustUnnamed = 4;
pub const G_CONVERT_ERROR_PARTIAL_INPUT: C2RustUnnamed = 3;
pub const G_CONVERT_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_CONVERT_ERROR_ILLEGAL_SEQUENCE: C2RustUnnamed = 1;
pub const G_CONVERT_ERROR_NO_CONVERSION: C2RustUnnamed = 0;
pub type gunichar = guint32;
pub type gunichar2 = guint16;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_utf8_strlen\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh19 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh19 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
static mut safe_c2rust_utf8_skip_data: [gchar; 256] = [
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    2 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    3 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    4 as ::core::ffi::c_int as gchar,
    5 as ::core::ffi::c_int as gchar,
    5 as ::core::ffi::c_int as gchar,
    5 as ::core::ffi::c_int as gchar,
    5 as ::core::ffi::c_int as gchar,
    6 as ::core::ffi::c_int as gchar,
    6 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
    1 as ::core::ffi::c_int as gchar,
];
#[no_mangle]
pub static mut safe_c2rust_g_utf8_skip: *const gchar =
    unsafe { &raw const safe_c2rust_utf8_skip_data as *const gchar };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_find_prev_char(
    mut str: *const gchar,
    mut p: *const gchar,
) -> *mut gchar {
    while p > str {
        p = p.offset(-1);
        if *p as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int {
            return p as *mut gchar;
        }
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_find_next_char(
    mut p: *const gchar,
    mut end: *const gchar,
) -> *mut gchar {
    if !end.is_null() {
        p = p.offset(1);
        while p < end
            && *p as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        return if p >= end {
            ::core::ptr::null_mut::<gchar>()
        } else {
            p as *mut gchar
        };
    } else {
        p = p.offset(1);
        while *p as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
            p = p.offset(1);
        }
        return p as *mut gchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_prev_char(mut p: *const gchar) -> *mut gchar {
    while FALSE == 0 {
        p = p.offset(-1);
        if *p as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int {
            return p as *mut gchar;
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_strlen(mut p: *const gchar, mut max: gssize) -> glong {
    let mut len: glong = 0 as glong;
    let mut start: *const gchar = p;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !p.is_null() || max == 0 as gssize {
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
            b"p != NULL || max == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as glong;
    }
    if max < 0 as gssize {
        while *p != 0 {
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            len += 1;
        }
    } else {
        if max == 0 as gssize || *p == 0 {
            return 0 as glong;
        }
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
        while (p.offset_from(start) as ::core::ffi::c_long) < max && *p as ::core::ffi::c_int != 0 {
            len += 1;
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
        if p.offset_from(start) as ::core::ffi::c_long <= max {
            len += 1;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_substring(
    mut str: *const gchar,
    mut start_pos: glong,
    mut end_pos: glong,
) -> *mut gchar {
    let mut start: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if end_pos >= start_pos || end_pos == -(1 as ::core::ffi::c_int) as glong {
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
            b"end_pos >= start_pos || end_pos == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    start = safe_c2rust_g_utf8_offset_to_pointer(str, start_pos);
    if end_pos == -(1 as ::core::ffi::c_int) as glong {
        let mut length: glong =
            safe_c2rust_g_utf8_strlen(start, -(1 as ::core::ffi::c_int) as gssize);
        end = safe_c2rust_g_utf8_offset_to_pointer(start, length);
    } else {
        end = safe_c2rust_g_utf8_offset_to_pointer(start, end_pos - start_pos);
    }
    out = g_malloc(
        (end.offset_from(start) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as gsize,
    ) as *mut gchar;
    memcpy(
        out as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        end.offset_from(start) as ::core::ffi::c_long as size_t,
    );
    *out.offset(end.offset_from(start) as ::core::ffi::c_long as isize) = 0 as gchar;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_get_char(mut p: *const gchar) -> gunichar {
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    let mut result: gunichar = 0;
    let mut c: ::core::ffi::c_uchar = *p as ::core::ffi::c_uchar;
    if (c as ::core::ffi::c_int) < 128 as ::core::ffi::c_int {
        len = 1 as ::core::ffi::c_int;
        mask = 0x7f as ::core::ffi::c_int;
    } else if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
        len = 2 as ::core::ffi::c_int;
        mask = 0x1f as ::core::ffi::c_int;
    } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int {
        len = 3 as ::core::ffi::c_int;
        mask = 0xf as ::core::ffi::c_int;
    } else if c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int {
        len = 4 as ::core::ffi::c_int;
        mask = 0x7 as ::core::ffi::c_int;
    } else if c as ::core::ffi::c_int & 0xfc as ::core::ffi::c_int == 0xf8 as ::core::ffi::c_int {
        len = 5 as ::core::ffi::c_int;
        mask = 0x3 as ::core::ffi::c_int;
    } else if c as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int == 0xfc as ::core::ffi::c_int {
        len = 6 as ::core::ffi::c_int;
        mask = 0x1 as ::core::ffi::c_int;
    } else {
        len = -(1 as ::core::ffi::c_int);
    }
    if len == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int) as gunichar;
    }
    result = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int & mask) as gunichar;
    i = 1 as ::core::ffi::c_int;
    while i < len {
        if *p.offset(i as isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
        {
            result = -(1 as ::core::ffi::c_int) as gunichar;
            break;
        } else {
            result <<= 6 as ::core::ffi::c_int;
            result |= (*p.offset(i as isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                as gunichar;
            i += 1;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_offset_to_pointer(
    mut str: *const gchar,
    mut offset: glong,
) -> *mut gchar {
    let mut s: *const gchar = str;
    if offset > 0 as glong {
        loop {
            let fresh0 = offset;
            offset = offset - 1;
            if !(fresh0 != 0) {
                break;
            }
            s = s.offset(
                *safe_c2rust_g_utf8_skip.offset(*(s as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
    } else {
        let mut s1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        while offset != 0 {
            s1 = s as *const ::core::ffi::c_char;
            s = s.offset(offset as isize);
            while *s as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
            {
                s = s.offset(-1);
            }
            offset += safe_c2rust_g_utf8_pointer_to_offset(s, s1 as *const gchar);
        }
    }
    return s as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_pointer_to_offset(
    mut str: *const gchar,
    mut pos: *const gchar,
) -> glong {
    let mut s: *const gchar = str;
    let mut offset: glong = 0 as glong;
    if pos < str {
        offset = -safe_c2rust_g_utf8_pointer_to_offset(pos, str);
    } else {
        while s < pos {
            s = s.offset(
                *safe_c2rust_g_utf8_skip.offset(*(s as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            offset += 1;
        }
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_strncpy(
    mut dest: *mut gchar,
    mut src: *const gchar,
    mut n: gsize,
) -> *mut gchar {
    let mut s: *const gchar = src;
    while n != 0 && *s as ::core::ffi::c_int != 0 {
        s = s.offset(
            *safe_c2rust_g_utf8_skip.offset(*(s as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
        n = n.wrapping_sub(1);
    }
    strncpy(
        dest as *mut ::core::ffi::c_char,
        src as *const ::core::ffi::c_char,
        s.offset_from(src) as ::core::ffi::c_long as size_t,
    );
    *dest.offset(s.offset_from(src) as ::core::ffi::c_long as isize) = 0 as gchar;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_truncate_middle(
    mut string: *const gchar,
    mut truncate_length: gsize,
) -> *mut gchar {
    let mut ellipsis: *const gchar = b"\xE2\x80\xA6\0" as *const u8 as *const gchar;
    let ellipsis_bytes: gsize = strlen(ellipsis as *const ::core::ffi::c_char) as gsize;
    let mut length: gsize = 0;
    let mut left_substring_length: gsize = 0;
    let mut left_substring_end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut right_substring_begin: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut right_substring_end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut left_bytes: gsize = 0;
    let mut right_bytes: gsize = 0;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    length = safe_c2rust_g_utf8_strlen(string, -(1 as ::core::ffi::c_int) as gssize) as gsize;
    if length <= truncate_length {
        return safe_c2rust_g_strdup_inline(string as *const ::core::ffi::c_char) as *mut gchar;
    }
    if truncate_length == 0 as gsize {
        return safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    truncate_length = truncate_length.wrapping_sub(1 as gsize);
    left_substring_length = truncate_length.wrapping_div(2 as gsize);
    left_substring_end =
        safe_c2rust_g_utf8_offset_to_pointer(string, left_substring_length as glong);
    right_substring_begin = safe_c2rust_g_utf8_offset_to_pointer(
        left_substring_end,
        length.wrapping_sub(truncate_length) as glong,
    );
    right_substring_end = safe_c2rust_g_utf8_offset_to_pointer(
        right_substring_begin,
        truncate_length.wrapping_sub(left_substring_length) as glong,
    );
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if *right_substring_end as ::core::ffi::c_int == '\0' as i32 {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gutf8.c\0" as *const u8 as *const ::core::ffi::c_char,
            514 as ::core::ffi::c_int,
            G_STRFUNC,
            b"*right_substring_end == '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    left_bytes = left_substring_end.offset_from(string) as ::core::ffi::c_long as gsize;
    right_bytes =
        right_substring_end.offset_from(right_substring_begin) as ::core::ffi::c_long as gsize;
    result = g_malloc(
        left_bytes
            .wrapping_add(ellipsis_bytes)
            .wrapping_add(right_bytes)
            .wrapping_add(1 as gsize),
    ) as *mut gchar;
    strncpy(
        result as *mut ::core::ffi::c_char,
        string as *const ::core::ffi::c_char,
        left_bytes as size_t,
    );
    memcpy(
        result.offset(left_bytes as isize) as *mut ::core::ffi::c_void,
        ellipsis as *const ::core::ffi::c_void,
        ellipsis_bytes as size_t,
    );
    strncpy(
        result
            .offset(left_bytes as isize)
            .offset(ellipsis_bytes as isize),
        right_substring_begin,
        right_bytes as size_t,
    );
    *result.offset(
        left_bytes
            .wrapping_add(ellipsis_bytes)
            .wrapping_add(right_bytes) as isize,
    ) = '\0' as i32 as gchar;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unichar_to_utf8(
    mut c: gunichar,
    mut outbuf: *mut gchar,
) -> gint {
    let mut len: guint = 0 as guint;
    let mut first: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if c < 0x80 as gunichar {
        first = 0 as ::core::ffi::c_int;
        len = 1 as guint;
    } else if c < 0x800 as gunichar {
        first = 0xc0 as ::core::ffi::c_int;
        len = 2 as guint;
    } else if c < 0x10000 as ::core::ffi::c_int as gunichar {
        first = 0xe0 as ::core::ffi::c_int;
        len = 3 as guint;
    } else if c < 0x200000 as ::core::ffi::c_int as gunichar {
        first = 0xf0 as ::core::ffi::c_int;
        len = 4 as guint;
    } else if c < 0x4000000 as ::core::ffi::c_int as gunichar {
        first = 0xf8 as ::core::ffi::c_int;
        len = 5 as guint;
    } else {
        first = 0xfc as ::core::ffi::c_int;
        len = 6 as guint;
    }
    if !outbuf.is_null() {
        i = len.wrapping_sub(1 as guint) as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            *outbuf.offset(i as isize) = (c & 0x3f as gunichar | 0x80 as gunichar) as gchar;
            c >>= 6 as ::core::ffi::c_int;
            i -= 1;
        }
        *outbuf.offset(0 as ::core::ffi::c_int as isize) = (c | first as gunichar) as gchar;
    }
    return len as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_strchr(
    mut p: *const ::core::ffi::c_char,
    mut len: gssize,
    mut c: gunichar,
) -> *mut gchar {
    let mut ch: [gchar; 10] = [0; 10];
    let mut charlen: gint = safe_c2rust_g_unichar_to_utf8(c, &raw mut ch as *mut gchar);
    ch[charlen as usize] = '\0' as i32 as gchar;
    return g_strstr_len(p as *const gchar, len, &raw mut ch as *mut gchar);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_strrchr(
    mut p: *const ::core::ffi::c_char,
    mut len: gssize,
    mut c: gunichar,
) -> *mut gchar {
    let mut ch: [gchar; 10] = [0; 10];
    let mut charlen: gint = safe_c2rust_g_unichar_to_utf8(c, &raw mut ch as *mut gchar);
    ch[charlen as usize] = '\0' as i32 as gchar;
    return g_strrstr_len(p as *const gchar, len, &raw mut ch as *mut gchar);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_utf8_get_char_extended(
    mut p: *const gchar,
    mut max_len: gssize,
) -> gunichar {
    let mut i: gsize = 0;
    let mut len: gsize = 0;
    let mut min_code: gunichar = 0;
    let mut wc: gunichar = *p as guchar as gunichar;
    let partial_sequence: gunichar = -(2 as ::core::ffi::c_int) as gunichar;
    let malformed_sequence: gunichar = -(1 as ::core::ffi::c_int) as gunichar;
    if wc < 0x80 as gunichar {
        return wc;
    } else if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if wc < 0xc0 as gunichar {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        return malformed_sequence;
    } else if wc < 0xe0 as gunichar {
        len = 2 as gsize;
        wc &= 0x1f as gunichar;
        min_code = ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as gunichar;
    } else if wc < 0xf0 as gunichar {
        len = 3 as gsize;
        wc &= 0xf as gunichar;
        min_code = ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as gunichar;
    } else if wc < 0xf8 as gunichar {
        len = 4 as gsize;
        wc &= 0x7 as gunichar;
        min_code = ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as gunichar;
    } else if wc < 0xfc as gunichar {
        len = 5 as gsize;
        wc &= 0x3 as gunichar;
        min_code = ((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int) as gunichar;
    } else if wc < 0xfe as gunichar {
        len = 6 as gsize;
        wc &= 0x1 as gunichar;
        min_code = ((1 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as gunichar;
    } else {
        return malformed_sequence;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if max_len >= 0 as gssize && len > max_len as gsize {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        i = 1 as gsize;
        while i < max_len as gsize {
            if *(p as *mut guchar).offset(i as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return malformed_sequence;
            }
            i = i.wrapping_add(1);
        }
        return partial_sequence;
    }
    i = 1 as gsize;
    while i < len {
        let mut ch: gunichar = *(p as *mut guchar).offset(i as isize) as gunichar;
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if ch & 0xc0 as gunichar != 0x80 as gunichar {
                _g_boolean_var_14 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_14 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_14
        }) as ::core::ffi::c_long
            != 0
        {
            if ch != 0 {
                return malformed_sequence;
            } else {
                return partial_sequence;
            }
        }
        wc <<= 6 as ::core::ffi::c_int;
        wc |= ch & 0x3f as gunichar;
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if wc < min_code {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        return malformed_sequence;
    }
    return wc;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_get_char_validated(
    mut p: *const gchar,
    mut max_len: gssize,
) -> gunichar {
    let mut result: gunichar = 0;
    if max_len == 0 as gssize {
        return -(2 as ::core::ffi::c_int) as gunichar;
    }
    result = safe_c2rust_g_utf8_get_char_extended(p, max_len);
    if result == 0 as gunichar && max_len > 0 as gssize {
        return -(2 as ::core::ffi::c_int) as gunichar;
    }
    if result as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint != 0 {
        return result;
    } else if !(result < 0x110000 as ::core::ffi::c_int as gunichar
        && result as ::core::ffi::c_uint & 0xfffff800 as ::core::ffi::c_uint
            != 0xd800 as ::core::ffi::c_uint)
    {
        return -(1 as ::core::ffi::c_int) as gunichar;
    } else {
        return result;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_to_ucs4_fast(
    mut str: *const gchar,
    mut len: glong,
    mut items_written: *mut glong,
) -> *mut gunichar {
    let mut result: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    let mut n_chars: gint = 0;
    let mut i: gint = 0;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gunichar>();
    }
    p = str;
    n_chars = 0 as ::core::ffi::c_int as gint;
    if len < 0 as glong {
        while *p != 0 {
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            n_chars += 1;
        }
    } else {
        while p < str.offset(len as isize) && *p as ::core::ffi::c_int != 0 {
            p = p.offset(
                *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
            n_chars += 1;
        }
    }
    result = ({
        let mut __n: gsize = (n_chars as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gunichar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gunichar;
    p = str;
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_chars {
        let fresh6 = p;
        p = p.offset(1);
        let mut first: guchar = *fresh6 as guchar;
        let mut wc: gunichar = 0;
        if (first as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
            wc = first as gunichar;
        } else {
            let fresh7 = p;
            p = p.offset(1);
            let mut c1: gunichar =
                (*fresh7 as guchar as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as gunichar;
            if (first as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                wc = ((first as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as gunichar
                    | c1;
            } else {
                let fresh8 = p;
                p = p.offset(1);
                let mut c2: gunichar = (*fresh8 as guchar as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    as gunichar;
                if (first as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                    wc = ((first as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                        << 12 as ::core::ffi::c_int) as gunichar
                        | c1 << 6 as ::core::ffi::c_int
                        | c2;
                } else {
                    let fresh9 = p;
                    p = p.offset(1);
                    let mut c3: gunichar = (*fresh9 as guchar as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int)
                        as gunichar;
                    wc = ((first as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                        << 18 as ::core::ffi::c_int) as gunichar
                        | c1 << 12 as ::core::ffi::c_int
                        | c2 << 6 as ::core::ffi::c_int
                        | c3;
                    if ({
                        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                        if first as ::core::ffi::c_int >= 0xf8 as ::core::ffi::c_int {
                            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_17
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        let mut mask: gunichar =
                            ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as gunichar;
                        while wc & mask != 0 as gunichar {
                            wc <<= 6 as ::core::ffi::c_int;
                            let fresh10 = p;
                            p = p.offset(1);
                            wc |= (*fresh10 as guchar as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_int)
                                as gunichar;
                            mask <<= 5 as ::core::ffi::c_int;
                        }
                        wc &= mask.wrapping_sub(1 as gunichar);
                    }
                }
            }
        }
        *result.offset(i as isize) = wc;
        i += 1;
    }
    *result.offset(i as isize) = 0 as gunichar;
    if !items_written.is_null() {
        *items_written = i as glong;
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_try_malloc_n(
    mut n_blocks: gsize,
    mut n_block_bytes: gsize,
    mut error: *mut *mut GError,
) -> gpointer {
    let mut ptr: gpointer = g_try_malloc_n(n_blocks, n_block_bytes);
    if ptr.is_null() {
        g_set_error_literal(
            error,
            g_convert_error_quark(),
            G_CONVERT_ERROR_NO_MEMORY as ::core::ffi::c_int as gint,
            glib_gettext(b"Failed to allocate memory\0" as *const u8 as *const gchar),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_to_ucs4(
    mut str: *const gchar,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gunichar {
    let mut current_block: u64;
    let mut result: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    let mut n_chars: gint = 0;
    let mut i: gint = 0;
    let mut in_0: *const gchar = ::core::ptr::null::<gchar>();
    in_0 = str;
    n_chars = 0 as ::core::ffi::c_int as gint;
    loop {
        if !((len < 0 as glong
            || str.offset(len as isize).offset_from(in_0) as ::core::ffi::c_long
                > 0 as ::core::ffi::c_long)
            && *in_0 as ::core::ffi::c_int != 0)
        {
            current_block = 11650488183268122163;
            break;
        }
        let mut wc: gunichar = safe_c2rust_g_utf8_get_char_extended(
            in_0,
            if len < 0 as glong {
                6 as gssize
            } else {
                str.offset(len as isize).offset_from(in_0) as gssize
            },
        );
        if wc as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint != 0 {
            if wc == -(2 as ::core::ffi::c_int) as gunichar {
                if !items_read.is_null() {
                    current_block = 11650488183268122163;
                    break;
                }
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Partial character sequence at end of input\0" as *const u8
                            as *const gchar,
                    ),
                );
                current_block = 10326713798369316726;
                break;
            } else {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid byte sequence in conversion input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 10326713798369316726;
                break;
            }
        } else {
            n_chars += 1;
            in_0 = in_0.offset(
                *safe_c2rust_g_utf8_skip.offset(*(in_0 as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
    }
    match current_block {
        11650488183268122163 => {
            result = safe_c2rust_try_malloc_n(
                (n_chars as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
                ::core::mem::size_of::<gunichar>() as gsize,
                error,
            ) as *mut gunichar;
            if !result.is_null() {
                in_0 = str;
                i = 0 as ::core::ffi::c_int as gint;
                while i < n_chars {
                    *result.offset(i as isize) = safe_c2rust_g_utf8_get_char(in_0);
                    in_0 = in_0.offset(
                        *safe_c2rust_g_utf8_skip.offset(*(in_0 as *const guchar) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as *mut ::core::ffi::c_char;
                    i += 1;
                }
                *result.offset(i as isize) = 0 as gunichar;
                if !items_written.is_null() {
                    *items_written = n_chars as glong;
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = in_0.offset_from(str) as ::core::ffi::c_long as glong;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ucs4_to_utf8(
    mut str: *const gunichar,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut result_length: gint = 0;
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gint = 0;
    result_length = 0 as ::core::ffi::c_int as gint;
    i = 0 as ::core::ffi::c_int as gint;
    loop {
        if !(len < 0 as glong || (i as glong) < len) {
            current_block = 14523784380283086299;
            break;
        }
        if *str.offset(i as isize) == 0 {
            current_block = 14523784380283086299;
            break;
        }
        if *str.offset(i as isize) >= 0x80000000 as ::core::ffi::c_uint {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(b"Character out of range for UTF-8\0" as *const u8 as *const gchar),
            );
            current_block = 7010495578377576394;
            break;
        } else {
            result_length += if *str.offset(i as isize) < 0x80 as gunichar {
                1 as ::core::ffi::c_int
            } else if *str.offset(i as isize) < 0x800 as gunichar {
                2 as ::core::ffi::c_int
            } else if *str.offset(i as isize) < 0x10000 as ::core::ffi::c_int as gunichar {
                3 as ::core::ffi::c_int
            } else if *str.offset(i as isize) < 0x200000 as ::core::ffi::c_int as gunichar {
                4 as ::core::ffi::c_int
            } else if *str.offset(i as isize) < 0x4000000 as ::core::ffi::c_int as gunichar {
                5 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            };
            i += 1;
        }
    }
    match current_block {
        14523784380283086299 => {
            result = safe_c2rust_try_malloc_n(
                (result_length as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
                1 as gsize,
                error,
            ) as *mut gchar;
            if !result.is_null() {
                p = result;
                i = 0 as ::core::ffi::c_int as gint;
                while p < result.offset(result_length as isize) {
                    let fresh14 = i;
                    i = i + 1;
                    p = p.offset(
                        safe_c2rust_g_unichar_to_utf8(*str.offset(fresh14 as isize), p) as isize,
                    );
                }
                *p = '\0' as i32 as gchar;
                if !items_written.is_null() {
                    *items_written = p.offset_from(result) as ::core::ffi::c_long as glong;
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = i as glong;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf16_to_utf8(
    mut str: *const gunichar2,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut in_0: *const gunichar2 = ::core::ptr::null::<gunichar2>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n_bytes: gint = 0;
    let mut high_surrogate: gunichar = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    n_bytes = 0 as ::core::ffi::c_int as gint;
    in_0 = str;
    high_surrogate = 0 as gunichar;
    loop {
        if !((len < 0 as glong || (in_0.offset_from(str) as ::core::ffi::c_long) < len)
            && *in_0 as ::core::ffi::c_int != 0)
        {
            current_block = 13472856163611868459;
            break;
        }
        let mut c: gunichar2 = *in_0;
        let mut wc: gunichar = 0;
        if c as ::core::ffi::c_int >= 0xdc00 as ::core::ffi::c_int
            && (c as ::core::ffi::c_int) < 0xe000 as ::core::ffi::c_int
        {
            if high_surrogate != 0 {
                wc = high_surrogate
                    .wrapping_sub(0xd800 as gunichar)
                    .wrapping_mul(0x400 as gunichar)
                    .wrapping_add(c as gunichar)
                    .wrapping_sub(0xdc00 as gunichar)
                    .wrapping_add(0x10000 as ::core::ffi::c_int as gunichar);
                high_surrogate = 0 as gunichar;
            } else {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 11212771201286138179;
                break;
            }
            current_block = 2668756484064249700;
        } else if high_surrogate != 0 {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                ),
            );
            current_block = 11212771201286138179;
            break;
        } else if c as ::core::ffi::c_int >= 0xd800 as ::core::ffi::c_int
            && (c as ::core::ffi::c_int) < 0xdc00 as ::core::ffi::c_int
        {
            high_surrogate = c as gunichar;
            current_block = 3310230692648646822;
        } else {
            wc = c as gunichar;
            current_block = 2668756484064249700;
        }
        match current_block {
            2668756484064249700 => {
                n_bytes += if wc < 0x80 as gunichar {
                    1 as ::core::ffi::c_int
                } else if wc < 0x800 as gunichar {
                    2 as ::core::ffi::c_int
                } else if wc < 0x10000 as ::core::ffi::c_int as gunichar {
                    3 as ::core::ffi::c_int
                } else if wc < 0x200000 as ::core::ffi::c_int as gunichar {
                    4 as ::core::ffi::c_int
                } else if wc < 0x4000000 as ::core::ffi::c_int as gunichar {
                    5 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                };
            }
            _ => {}
        }
        in_0 = in_0.offset(1);
    }
    match current_block {
        13472856163611868459 => {
            if high_surrogate != 0 && items_read.is_null() {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Partial character sequence at end of input\0" as *const u8
                            as *const gchar,
                    ),
                );
            } else {
                result = safe_c2rust_try_malloc_n(
                    (n_bytes as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
                    1 as gsize,
                    error,
                ) as *mut gchar;
                if !result.is_null() {
                    high_surrogate = 0 as gunichar;
                    out = result;
                    in_0 = str;
                    while out < result.offset(n_bytes as isize) {
                        let mut current_block_30: u64;
                        let mut c_0: gunichar2 = *in_0;
                        let mut wc_0: gunichar = 0;
                        if c_0 as ::core::ffi::c_int >= 0xdc00 as ::core::ffi::c_int
                            && (c_0 as ::core::ffi::c_int) < 0xe000 as ::core::ffi::c_int
                        {
                            wc_0 = high_surrogate
                                .wrapping_sub(0xd800 as gunichar)
                                .wrapping_mul(0x400 as gunichar)
                                .wrapping_add(c_0 as gunichar)
                                .wrapping_sub(0xdc00 as gunichar)
                                .wrapping_add(0x10000 as ::core::ffi::c_int as gunichar);
                            high_surrogate = 0 as gunichar;
                            current_block_30 = 6417057564578538666;
                        } else if c_0 as ::core::ffi::c_int >= 0xd800 as ::core::ffi::c_int
                            && (c_0 as ::core::ffi::c_int) < 0xdc00 as ::core::ffi::c_int
                        {
                            high_surrogate = c_0 as gunichar;
                            current_block_30 = 9929780071722701723;
                        } else {
                            wc_0 = c_0 as gunichar;
                            current_block_30 = 6417057564578538666;
                        }
                        match current_block_30 {
                            6417057564578538666 => {
                                out = out.offset(safe_c2rust_g_unichar_to_utf8(wc_0, out) as isize);
                            }
                            _ => {}
                        }
                        in_0 = in_0.offset(1);
                    }
                    *out = '\0' as i32 as gchar;
                    if !items_written.is_null() {
                        *items_written = out.offset_from(result) as ::core::ffi::c_long as glong;
                    }
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = in_0.offset_from(str) as ::core::ffi::c_long as glong;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf16_to_ucs4(
    mut str: *const gunichar2,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gunichar {
    let mut current_block: u64;
    let mut in_0: *const gunichar2 = ::core::ptr::null::<gunichar2>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n_bytes: gint = 0;
    let mut high_surrogate: gunichar = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gunichar>();
    }
    n_bytes = 0 as ::core::ffi::c_int as gint;
    in_0 = str;
    high_surrogate = 0 as gunichar;
    loop {
        if !((len < 0 as glong || (in_0.offset_from(str) as ::core::ffi::c_long) < len)
            && *in_0 as ::core::ffi::c_int != 0)
        {
            current_block = 14576567515993809846;
            break;
        }
        let mut c: gunichar2 = *in_0;
        if c as ::core::ffi::c_int >= 0xdc00 as ::core::ffi::c_int
            && (c as ::core::ffi::c_int) < 0xe000 as ::core::ffi::c_int
        {
            if high_surrogate != 0 {
                high_surrogate = 0 as gunichar;
            } else {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 11279305633851793203;
                break;
            }
            current_block = 4808432441040389987;
        } else if high_surrogate != 0 {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                ),
            );
            current_block = 11279305633851793203;
            break;
        } else if c as ::core::ffi::c_int >= 0xd800 as ::core::ffi::c_int
            && (c as ::core::ffi::c_int) < 0xdc00 as ::core::ffi::c_int
        {
            high_surrogate = c as gunichar;
            current_block = 9581801748415692716;
        } else {
            current_block = 4808432441040389987;
        }
        match current_block {
            4808432441040389987 => {
                n_bytes =
                    (n_bytes as ::core::ffi::c_ulong).wrapping_add(
                        ::core::mem::size_of::<gunichar>() as usize as ::core::ffi::c_ulong,
                    ) as gint as gint;
            }
            _ => {}
        }
        in_0 = in_0.offset(1);
    }
    match current_block {
        14576567515993809846 => {
            if high_surrogate != 0 && items_read.is_null() {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Partial character sequence at end of input\0" as *const u8
                            as *const gchar,
                    ),
                );
            } else {
                result = safe_c2rust_try_malloc_n(
                    (n_bytes as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as gsize,
                    1 as gsize,
                    error,
                ) as *mut gchar;
                if !result.is_null() {
                    high_surrogate = 0 as gunichar;
                    out = result;
                    in_0 = str;
                    while out < result.offset(n_bytes as isize) {
                        let mut current_block_29: u64;
                        let mut c_0: gunichar2 = *in_0;
                        let mut wc: gunichar = 0;
                        if c_0 as ::core::ffi::c_int >= 0xdc00 as ::core::ffi::c_int
                            && (c_0 as ::core::ffi::c_int) < 0xe000 as ::core::ffi::c_int
                        {
                            wc = high_surrogate
                                .wrapping_sub(0xd800 as gunichar)
                                .wrapping_mul(0x400 as gunichar)
                                .wrapping_add(c_0 as gunichar)
                                .wrapping_sub(0xdc00 as gunichar)
                                .wrapping_add(0x10000 as ::core::ffi::c_int as gunichar);
                            high_surrogate = 0 as gunichar;
                            current_block_29 = 7245201122033322888;
                        } else if c_0 as ::core::ffi::c_int >= 0xd800 as ::core::ffi::c_int
                            && (c_0 as ::core::ffi::c_int) < 0xdc00 as ::core::ffi::c_int
                        {
                            high_surrogate = c_0 as gunichar;
                            current_block_29 = 12837098503634163823;
                        } else {
                            wc = c_0 as gunichar;
                            current_block_29 = 7245201122033322888;
                        }
                        match current_block_29 {
                            7245201122033322888 => {
                                *(out as *mut gunichar) = wc;
                                out = out
                                    .offset(::core::mem::size_of::<gunichar>() as usize as isize);
                            }
                            _ => {}
                        }
                        in_0 = in_0.offset(1);
                    }
                    *(out as *mut gunichar) = 0 as gunichar;
                    if !items_written.is_null() {
                        *items_written = (out.offset_from(result) as ::core::ffi::c_long as usize)
                            .wrapping_div(::core::mem::size_of::<gunichar>() as usize)
                            as glong;
                    }
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = in_0.offset_from(str) as ::core::ffi::c_long as glong;
    }
    return result as *mut gunichar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_to_utf16(
    mut str: *const gchar,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gunichar2 {
    let mut current_block: u64;
    let mut result: *mut gunichar2 = ::core::ptr::null_mut::<gunichar2>();
    let mut n16: gint = 0;
    let mut in_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gunichar2>();
    }
    in_0 = str;
    n16 = 0 as ::core::ffi::c_int as gint;
    loop {
        if !((len < 0 as glong
            || str.offset(len as isize).offset_from(in_0) as ::core::ffi::c_long
                > 0 as ::core::ffi::c_long)
            && *in_0 as ::core::ffi::c_int != 0)
        {
            current_block = 13472856163611868459;
            break;
        }
        let mut wc: gunichar = safe_c2rust_g_utf8_get_char_extended(
            in_0,
            if len < 0 as glong {
                6 as gssize
            } else {
                str.offset(len as isize).offset_from(in_0) as gssize
            },
        );
        if wc as ::core::ffi::c_uint & 0x80000000 as ::core::ffi::c_uint != 0 {
            if wc == -(2 as ::core::ffi::c_int) as gunichar {
                if !items_read.is_null() {
                    current_block = 13472856163611868459;
                    break;
                }
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_PARTIAL_INPUT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Partial character sequence at end of input\0" as *const u8
                            as *const gchar,
                    ),
                );
                current_block = 9656796771906786635;
                break;
            } else {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid byte sequence in conversion input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 9656796771906786635;
                break;
            }
        } else {
            if wc < 0xd800 as gunichar {
                n16 += 1 as ::core::ffi::c_int;
            } else if wc < 0xe000 as gunichar {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 9656796771906786635;
                break;
            } else if wc < 0x10000 as ::core::ffi::c_int as gunichar {
                n16 += 1 as ::core::ffi::c_int;
            } else if wc < 0x110000 as ::core::ffi::c_int as gunichar {
                n16 += 2 as ::core::ffi::c_int;
            } else {
                g_set_error_literal(
                    error,
                    g_convert_error_quark(),
                    G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Character out of range for UTF-16\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 9656796771906786635;
                break;
            }
            in_0 = in_0.offset(
                *safe_c2rust_g_utf8_skip.offset(*(in_0 as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char;
        }
    }
    match current_block {
        13472856163611868459 => {
            result = safe_c2rust_try_malloc_n(
                (n16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
                ::core::mem::size_of::<gunichar2>() as gsize,
                error,
            ) as *mut gunichar2;
            if !result.is_null() {
                in_0 = str;
                i = 0 as ::core::ffi::c_int as gint;
                while i < n16 {
                    let mut wc_0: gunichar = safe_c2rust_g_utf8_get_char(in_0);
                    if wc_0 < 0x10000 as ::core::ffi::c_int as gunichar {
                        let fresh3 = i;
                        i = i + 1;
                        *result.offset(fresh3 as isize) = wc_0 as gunichar2;
                    } else {
                        let fresh4 = i;
                        i = i + 1;
                        *result.offset(fresh4 as isize) = wc_0
                            .wrapping_sub(0x10000 as ::core::ffi::c_int as gunichar)
                            .wrapping_div(0x400 as gunichar)
                            .wrapping_add(0xd800 as gunichar)
                            as gunichar2;
                        let fresh5 = i;
                        i = i + 1;
                        *result.offset(fresh5 as isize) = wc_0
                            .wrapping_sub(0x10000 as ::core::ffi::c_int as gunichar)
                            .wrapping_rem(0x400 as gunichar)
                            .wrapping_add(0xdc00 as gunichar)
                            as gunichar2;
                    }
                    in_0 = in_0.offset(
                        *safe_c2rust_g_utf8_skip.offset(*(in_0 as *const guchar) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as *mut ::core::ffi::c_char;
                }
                *result.offset(i as isize) = 0 as gunichar2;
                if !items_written.is_null() {
                    *items_written = n16 as glong;
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = in_0.offset_from(str) as ::core::ffi::c_long as glong;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ucs4_to_utf16(
    mut str: *const gunichar,
    mut len: glong,
    mut items_read: *mut glong,
    mut items_written: *mut glong,
    mut error: *mut *mut GError,
) -> *mut gunichar2 {
    let mut current_block: u64;
    let mut result: *mut gunichar2 = ::core::ptr::null_mut::<gunichar2>();
    let mut n16: gint = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    n16 = 0 as ::core::ffi::c_int as gint;
    i = 0 as ::core::ffi::c_int as gint;
    loop {
        if !((len < 0 as glong || (i as glong) < len) && *str.offset(i as isize) != 0) {
            current_block = 11050875288958768710;
            break;
        }
        let mut wc: gunichar = *str.offset(i as isize);
        if wc < 0xd800 as gunichar {
            n16 += 1 as ::core::ffi::c_int;
        } else if wc < 0xe000 as gunichar {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Invalid sequence in conversion input\0" as *const u8 as *const gchar,
                ),
            );
            current_block = 5693595906878768094;
            break;
        } else if wc < 0x10000 as ::core::ffi::c_int as gunichar {
            n16 += 1 as ::core::ffi::c_int;
        } else if wc < 0x110000 as ::core::ffi::c_int as gunichar {
            n16 += 2 as ::core::ffi::c_int;
        } else {
            g_set_error_literal(
                error,
                g_convert_error_quark(),
                G_CONVERT_ERROR_ILLEGAL_SEQUENCE as ::core::ffi::c_int as gint,
                glib_gettext(b"Character out of range for UTF-16\0" as *const u8 as *const gchar),
            );
            current_block = 5693595906878768094;
            break;
        }
        i += 1;
    }
    match current_block {
        11050875288958768710 => {
            result = safe_c2rust_try_malloc_n(
                (n16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
                ::core::mem::size_of::<gunichar2>() as gsize,
                error,
            ) as *mut gunichar2;
            if !result.is_null() {
                i = 0 as ::core::ffi::c_int as gint;
                j = 0 as ::core::ffi::c_int as gint;
                while j < n16 {
                    let mut wc_0: gunichar = *str.offset(i as isize);
                    if wc_0 < 0x10000 as ::core::ffi::c_int as gunichar {
                        let fresh11 = j;
                        j = j + 1;
                        *result.offset(fresh11 as isize) = wc_0 as gunichar2;
                    } else {
                        let fresh12 = j;
                        j = j + 1;
                        *result.offset(fresh12 as isize) = wc_0
                            .wrapping_sub(0x10000 as ::core::ffi::c_int as gunichar)
                            .wrapping_div(0x400 as gunichar)
                            .wrapping_add(0xd800 as gunichar)
                            as gunichar2;
                        let fresh13 = j;
                        j = j + 1;
                        *result.offset(fresh13 as isize) = wc_0
                            .wrapping_sub(0x10000 as ::core::ffi::c_int as gunichar)
                            .wrapping_rem(0x400 as gunichar)
                            .wrapping_add(0xdc00 as gunichar)
                            as gunichar2;
                    }
                    i += 1;
                }
                *result.offset(j as isize) = 0 as gunichar2;
                if !items_written.is_null() {
                    *items_written = n16 as glong;
                }
            }
        }
        _ => {}
    }
    if !items_read.is_null() {
        *items_read = i as glong;
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_fast_validate(
    mut str: *const ::core::ffi::c_char,
) -> *const gchar {
    let mut current_block: u64;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = str as *const gchar;
    while *p != 0 {
        if !((*(p as *mut guchar) as ::core::ffi::c_int) < 128 as ::core::ffi::c_int) {
            let mut last: *const gchar = ::core::ptr::null::<gchar>();
            last = p;
            if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                if ({
                    let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                    if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xc2 as ::core::ffi::c_int {
                        _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_21
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 7944236900507219060;
                } else {
                    current_block = 15597372965620363352;
                }
            } else if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                let fresh15 = p;
                p = p.offset(1);
                match *(fresh15 as *mut guchar) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int {
                    0 => {
                        if ({
                            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                != 0xa0 as ::core::ffi::c_int
                            {
                                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_22
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    }
                    13 => {
                        if ({
                            let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_23
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    }
                    _ => {
                        if ({
                            let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_24
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    }
                }
            } else if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xf5 as ::core::ffi::c_int {
                let fresh16 = p;
                p = p.offset(1);
                match *(fresh16 as *mut guchar) as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int {
                    0 => {
                        if ({
                            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_25
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else if ({
                            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0x30 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_26
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 652864300344834934;
                        }
                    }
                    4 => {
                        if ({
                            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_27
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 652864300344834934;
                        }
                    }
                    _ => {
                        if ({
                            let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_28
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 652864300344834934;
                        }
                    }
                }
                match current_block {
                    7944236900507219060 => {}
                    _ => {
                        p = p.offset(1);
                        if ({
                            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                            if *(p as *mut guchar) as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_29
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            current_block = 7944236900507219060;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    }
                }
            } else {
                current_block = 7944236900507219060;
            }
            match current_block {
                15597372965620363352 => {
                    p = p.offset(1);
                    if ({
                        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
                        if *(p as *mut guchar) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                        {
                            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_30
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        current_block = 7944236900507219060;
                    } else {
                        current_block = 11174649648027449784;
                    }
                }
                _ => {}
            }
            match current_block {
                11174649648027449784 => {}
                _ => return last,
            }
        }
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn safe_c2rust_fast_validate_len(
    mut str: *const ::core::ffi::c_char,
    mut max_len: gssize,
) -> *const gchar {
    let mut current_block: u64;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if max_len >= 0 as gssize {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gutf8.c\0" as *const u8 as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            G_STRFUNC,
            b"max_len >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    p = str as *const gchar;
    while (p.offset_from(str) as ::core::ffi::c_long) < max_len && *p as ::core::ffi::c_int != 0 {
        if !((*(p as *mut guchar) as ::core::ffi::c_int) < 128 as ::core::ffi::c_int) {
            let mut last: *const gchar = ::core::ptr::null::<gchar>();
            last = p;
            if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                if ({
                    let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                    if (max_len as ::core::ffi::c_long - p.offset_from(str) as ::core::ffi::c_long)
                        < 2 as ::core::ffi::c_long
                    {
                        _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_32
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 17381804087586844724;
                } else if ({
                    let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                    if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xc2 as ::core::ffi::c_int {
                        _g_boolean_var_33 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_33 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_33
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 17381804087586844724;
                } else {
                    current_block = 15004371738079956865;
                }
            } else if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                if ({
                    let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
                    if (max_len as ::core::ffi::c_long - p.offset_from(str) as ::core::ffi::c_long)
                        < 3 as ::core::ffi::c_long
                    {
                        _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_34
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 17381804087586844724;
                } else {
                    let fresh17 = p;
                    p = p.offset(1);
                    match *(fresh17 as *mut guchar) as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int
                    {
                        0 => {
                            if ({
                                let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    != 0xa0 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_35 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_35 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_35
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 15004371738079956865;
                            }
                        }
                        13 => {
                            if ({
                                let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_36 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_36 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_36
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 15004371738079956865;
                            }
                        }
                        _ => {
                            if ({
                                let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xc0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_37 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_37 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_37
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 15004371738079956865;
                            }
                        }
                    }
                }
            } else if (*(p as *mut guchar) as ::core::ffi::c_int) < 0xf5 as ::core::ffi::c_int {
                if ({
                    let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
                    if (max_len as ::core::ffi::c_long - p.offset_from(str) as ::core::ffi::c_long)
                        < 4 as ::core::ffi::c_long
                    {
                        _g_boolean_var_38 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_38 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_38
                }) as ::core::ffi::c_long
                    != 0
                {
                    current_block = 17381804087586844724;
                } else {
                    let fresh18 = p;
                    p = p.offset(1);
                    match *(fresh18 as *mut guchar) as ::core::ffi::c_int
                        & 0x7 as ::core::ffi::c_int
                    {
                        0 => {
                            if ({
                                let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xc0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_39 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_39 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_39
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else if ({
                                let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0x30 as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_40 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_40 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_40
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                        4 => {
                            if ({
                                let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_41
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                        _ => {
                            if ({
                                let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xc0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_42 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_42 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_42
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        17381804087586844724 => {}
                        _ => {
                            p = p.offset(1);
                            if ({
                                let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
                                if *(p as *mut guchar) as ::core::ffi::c_int
                                    & 0xc0 as ::core::ffi::c_int
                                    != 0x80 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_43 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_43 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_43
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                current_block = 17381804087586844724;
                            } else {
                                current_block = 15004371738079956865;
                            }
                        }
                    }
                }
            } else {
                current_block = 17381804087586844724;
            }
            match current_block {
                15004371738079956865 => {
                    p = p.offset(1);
                    if ({
                        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
                        if *(p as *mut guchar) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                        {
                            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_44
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        current_block = 17381804087586844724;
                    } else {
                        current_block = 7095457783677275021;
                    }
                }
                _ => {}
            }
            match current_block {
                7095457783677275021 => {}
                _ => return last,
            }
        }
        p = p.offset(1);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_validate(
    mut str: *const ::core::ffi::c_char,
    mut max_len: gssize,
    mut end: *mut *const gchar,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if max_len >= 0 as gssize {
        return safe_c2rust_g_utf8_validate_len(str, max_len as gsize, end);
    }
    p = safe_c2rust_fast_validate(str);
    if !end.is_null() {
        *end = p;
    }
    if *p as ::core::ffi::c_int != '\0' as i32 {
        return FALSE;
    } else {
        return TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_validate_len(
    mut str: *const ::core::ffi::c_char,
    mut max_len: gsize,
    mut end: *mut *const gchar,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = safe_c2rust_fast_validate_len(str, max_len as gssize);
    if !end.is_null() {
        *end = p;
    }
    if p != str.offset(max_len as isize) {
        return FALSE;
    } else {
        return TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unichar_validate(mut ch: gunichar) -> gboolean {
    return (ch < 0x110000 as ::core::ffi::c_int as gunichar
        && ch as ::core::ffi::c_uint & 0xfffff800 as ::core::ffi::c_uint
            != 0xd800 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_strreverse(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut r: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if len < 0 as gssize {
        len = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    result = ({
        let mut __n: gsize = (len + 1 as gssize) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gchar;
    r = result.offset(len as isize);
    p = str;
    while r > result {
        let mut m: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut skip: gchar = *safe_c2rust_g_utf8_skip.offset(*(p as *mut guchar) as isize);
        r = r.offset(-(skip as ::core::ffi::c_int as isize));
        if ({
            let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
            if r >= result {
                _g_boolean_var_45 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_45 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_45
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gutf8.c\0" as *const u8 as *const ::core::ffi::c_char,
                1866 as ::core::ffi::c_int,
                G_STRFUNC,
                b"r >= result\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        m = r;
        while skip != 0 {
            let fresh1 = p;
            p = p.offset(1);
            let fresh2 = m;
            m = m.offset(1);
            *fresh2 = *fresh1;
            skip -= 1;
        }
    }
    *result.offset(len as isize) = 0 as gchar;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_make_valid(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut remainder: *const gchar = ::core::ptr::null::<gchar>();
    let mut invalid: *const gchar = ::core::ptr::null::<gchar>();
    let mut remaining_bytes: gsize = 0;
    let mut valid_bytes: gsize = 0;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        len = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    string = ::core::ptr::null_mut::<GString>();
    remainder = str;
    remaining_bytes = len as gsize;
    while remaining_bytes != 0 as gsize {
        if safe_c2rust_g_utf8_validate(
            remainder as *const ::core::ffi::c_char,
            remaining_bytes as gssize,
            &raw mut invalid,
        ) != 0
        {
            break;
        }
        valid_bytes = invalid.offset_from(remainder) as ::core::ffi::c_long as gsize;
        if string.is_null() {
            string = g_string_sized_new(remaining_bytes);
        }
        safe_c2rust_g_string_append_len_inline(
            string,
            remainder as *const ::core::ffi::c_char,
            valid_bytes as gssize,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\xEF\xBF\xBD\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_47
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                string,
                b"\xEF\xBF\xBD\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        remaining_bytes = remaining_bytes.wrapping_sub(valid_bytes.wrapping_add(1 as gsize));
        remainder = invalid.offset(1 as ::core::ffi::c_int as isize);
    }
    if string.is_null() {
        return g_strndup(str, len as gsize);
    }
    safe_c2rust_g_string_append_len_inline(
        string,
        remainder as *const ::core::ffi::c_char,
        remaining_bytes as gssize,
    );
    safe_c2rust_g_string_append_c_inline(string, '\0' as i32 as gchar);
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if safe_c2rust_g_utf8_validate(
            (*string).str_0,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gutf8.c\0" as *const u8 as *const ::core::ffi::c_char,
            1935 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_utf8_validate (string->str, -1, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean)
        } else {
            g_string_free_and_steal(string)
        }
    } else {
        g_string_free(string, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
