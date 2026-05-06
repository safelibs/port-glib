extern "C" {
    pub type _GBytes;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_free(mem: gpointer);
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_ascii_tolower(c: gchar) -> gchar;
    fn g_ascii_toupper(c: gchar) -> gchar;
    fn _uri_encoder(
        out: *mut GString,
        start: *const guchar,
        length: gsize,
        reserved_chars_allowed: *const gchar,
        allow_utf8: gboolean,
    );
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_vasprintf(
        string: *mut *mut gchar,
        format: *const gchar,
        args: ::core::ffi::VaList,
    ) -> gint;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __int32_t = i32;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type gunichar = guint32;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GLogLevelFlags = ::core::ffi::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_nearest_pow\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
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
        return safe_c2rust_g_string_append_len(gstring, val as *const gchar, len);
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
            safe_c2rust_g_string_append_len(gstring, val as *const gchar, len)
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
        return safe_c2rust_g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_truncate_inline(
    mut gstring: *mut GString,
    mut len: gsize,
) -> *mut GString {
    (*gstring).len = if len < (*gstring).len {
        len
    } else {
        (*gstring).len
    };
    *(*gstring).str_0.offset((*gstring).len as isize) = '\0' as i32 as gchar;
    return gstring;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_nearest_pow(mut num: gsize) -> gsize {
    let mut n: gsize = num.wrapping_sub(1 as gsize);
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if num > 0 as gsize
            && num
                <= (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
        {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gutilsprivate.h\0" as *const u8 as *const ::core::ffi::c_char,
            44 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num > 0 && num <= G_MAXSIZE / 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as gsize);
}
unsafe extern "C" fn safe_c2rust_g_string_expand(mut string: *mut GString, mut len: gsize) {
    (*string).allocated_len =
        safe_c2rust_g_nearest_pow((*string).len.wrapping_add(len).wrapping_add(1 as gsize));
    if (*string).allocated_len == 0 as gsize {
        (*string).allocated_len = (*string).len.wrapping_add(len).wrapping_add(1 as gsize);
    }
    (*string).str_0 = g_realloc((*string).str_0 as gpointer, (*string).allocated_len) as *mut gchar;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_string_maybe_expand(mut string: *mut GString, mut len: gsize) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
            .wrapping_mul(2 as ::core::ffi::c_ulong)
            .wrapping_add(1 as ::core::ffi::c_ulong)
            .wrapping_sub((*string).len as ::core::ffi::c_ulong)
            .wrapping_sub(1 as ::core::ffi::c_ulong)
            < len
        {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"adding %lu to string would overflow\0" as *const u8 as *const gchar,
            len,
        );
        loop {}
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*string).len.wrapping_add(len) >= (*string).allocated_len {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_string_expand(string, len);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_sized_new(mut dfl_size: gsize) -> *mut GString {
    let mut string: *mut GString =
        g_slice_alloc(::core::mem::size_of::<GString>() as gsize) as *mut GString;
    (*string).allocated_len = 0 as gsize;
    (*string).len = 0 as gsize;
    (*string).str_0 = ::core::ptr::null_mut::<gchar>();
    safe_c2rust_g_string_expand(
        string,
        if dfl_size > 64 as gsize {
            dfl_size
        } else {
            64 as gsize
        },
    );
    *(*string).str_0.offset(0 as ::core::ffi::c_int as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_new(mut init: *const gchar) -> *mut GString {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    if init.is_null() || *init as ::core::ffi::c_int == '\0' as i32 {
        string = safe_c2rust_g_string_sized_new(2 as gsize);
    } else {
        let mut len: gint = 0;
        len = strlen(init as *const ::core::ffi::c_char) as gint;
        string = safe_c2rust_g_string_sized_new(
            (len as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as gsize,
        );
        safe_c2rust_g_string_append_len_inline(
            string,
            init as *const ::core::ffi::c_char,
            len as gssize,
        );
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_new_take(mut init: *mut gchar) -> *mut GString {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    if init.is_null() {
        return safe_c2rust_g_string_new(::core::ptr::null::<gchar>());
    }
    string = g_slice_alloc(::core::mem::size_of::<GString>() as gsize) as *mut GString;
    (*string).str_0 = init;
    (*string).len = strlen((*string).str_0) as gsize;
    (*string).allocated_len = (*string).len.wrapping_add(1 as gsize);
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_new_len(
    mut init: *const gchar,
    mut len: gssize,
) -> *mut GString {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    if len < 0 as gssize {
        return safe_c2rust_g_string_new(init);
    } else {
        string = safe_c2rust_g_string_sized_new(len as gsize);
        if !init.is_null() {
            safe_c2rust_g_string_append_len_inline(string, init as *const ::core::ffi::c_char, len);
        }
        return string;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_free(
    mut string: *mut GString,
    mut free_segment: gboolean,
) -> *mut gchar {
    let mut segment: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
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
    if free_segment != 0 {
        g_free((*string).str_0 as gpointer);
        segment = ::core::ptr::null_mut::<gchar>();
    } else {
        segment = (*string).str_0;
    }
    g_slice_free1(
        ::core::mem::size_of::<GString>() as gsize,
        string as gpointer,
    );
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_free_and_steal(
    mut string: *mut GString,
) -> *mut gchar {
    return safe_c2rust_g_string_free(string, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_free_to_bytes(
    mut string: *mut GString,
) -> *mut GBytes {
    let mut len: gsize = 0;
    let mut buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    len = (*string).len;
    buf = if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_string_free(string, 0 as gboolean)
        } else {
            safe_c2rust_g_string_free_and_steal(string)
        }
    } else {
        safe_c2rust_g_string_free(string, 0 as gboolean)
    };
    return g_bytes_new_take(buf as gpointer, len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_equal(
    mut v: *const GString,
    mut v2: *const GString,
) -> gboolean {
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut string1: *mut GString = v as *mut GString;
    let mut string2: *mut GString = v2 as *mut GString;
    let mut i: gsize = (*string1).len;
    if i != (*string2).len {
        return FALSE;
    }
    p = (*string1).str_0;
    q = (*string2).str_0;
    while i != 0 {
        if *p as ::core::ffi::c_int != *q as ::core::ffi::c_int {
            return FALSE;
        }
        p = p.offset(1);
        q = q.offset(1);
        i = i.wrapping_sub(1);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_hash(mut str: *const GString) -> guint {
    let mut p: *const gchar = (*str).str_0;
    let mut n: gsize = (*str).len;
    let mut h: guint = 0 as guint;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_sub(h)
            .wrapping_add(*p as guint);
        p = p.offset(1);
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_assign(
    mut string: *mut GString,
    mut rval: *const gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !rval.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"rval != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    if (*string).str_0 != rval as *mut gchar {
        safe_c2rust_g_string_truncate_inline(string, 0 as gsize);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = rval as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_15
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
                rval as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_truncate(
    mut string: *mut GString,
    mut len: gsize,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    (*string).len = if len < (*string).len {
        len
    } else {
        (*string).len
    };
    *(*string).str_0.offset((*string).len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_set_size(
    mut string: *mut GString,
    mut len: gsize,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if len >= (*string).allocated_len {
        safe_c2rust_g_string_maybe_expand(string, len.wrapping_sub((*string).len));
    }
    (*string).len = len;
    *(*string).str_0.offset(len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_insert_len(
    mut string: *mut GString,
    mut pos: gssize,
    mut val: *const gchar,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    let mut pos_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if len == 0 as gssize || !val.is_null() {
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
            b"len == 0 || val != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    if len == 0 as gssize {
        return string;
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val as *const ::core::ffi::c_char) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if pos < 0 as gssize {
        pos_unsigned = (*string).len;
    } else {
        pos_unsigned = pos as gsize;
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if pos_unsigned <= (*string).len {
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
                b"pos_unsigned <= string->len\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return string;
        }
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if val >= (*string).str_0 as *const gchar
            && val <= (*string).str_0.offset((*string).len as isize) as *const gchar
        {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
        let mut offset: gsize = val.offset_from((*string).str_0) as ::core::ffi::c_long as gsize;
        let mut precount: gsize = 0 as gsize;
        safe_c2rust_g_string_maybe_expand(string, len_unsigned);
        val = (*string).str_0.offset(offset as isize);
        if pos_unsigned < (*string).len {
            memmove(
                (*string)
                    .str_0
                    .offset(pos_unsigned as isize)
                    .offset(len_unsigned as isize) as *mut ::core::ffi::c_void,
                (*string).str_0.offset(pos_unsigned as isize) as *const ::core::ffi::c_void,
                ((*string).len as size_t).wrapping_sub(pos_unsigned as size_t),
            );
        }
        if offset < pos_unsigned {
            precount = if len_unsigned < pos_unsigned.wrapping_sub(offset) {
                len_unsigned
            } else {
                pos_unsigned.wrapping_sub(offset)
            };
            memcpy(
                (*string).str_0.offset(pos_unsigned as isize) as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                precount as size_t,
            );
        }
        if len_unsigned > precount {
            memcpy(
                (*string)
                    .str_0
                    .offset(pos_unsigned as isize)
                    .offset(precount as isize) as *mut ::core::ffi::c_void,
                val.offset(precount as isize).offset(len_unsigned as isize)
                    as *const ::core::ffi::c_void,
                (len_unsigned as size_t).wrapping_sub(precount as size_t),
            );
        }
    } else {
        safe_c2rust_g_string_maybe_expand(string, len_unsigned);
        if pos_unsigned < (*string).len {
            memmove(
                (*string)
                    .str_0
                    .offset(pos_unsigned as isize)
                    .offset(len_unsigned as isize) as *mut ::core::ffi::c_void,
                (*string).str_0.offset(pos_unsigned as isize) as *const ::core::ffi::c_void,
                ((*string).len as size_t).wrapping_sub(pos_unsigned as size_t),
            );
        }
        if len_unsigned == 1 as gsize {
            *(*string).str_0.offset(pos_unsigned as isize) = *val;
        } else {
            memcpy(
                (*string).str_0.offset(pos_unsigned as isize) as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
    }
    (*string).len = (*string).len.wrapping_add(len_unsigned);
    *(*string).str_0.offset((*string).len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_uri_escaped(
    mut string: *mut GString,
    mut unescaped: *const gchar,
    mut reserved_chars_allowed: *const gchar,
    mut allow_utf8: gboolean,
) -> *mut GString {
    _uri_encoder(
        string,
        unescaped as *const guchar,
        strlen(unescaped as *const ::core::ffi::c_char) as gsize,
        reserved_chars_allowed,
        allow_utf8,
    );
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append(
    mut string: *mut GString,
    mut val: *const gchar,
) -> *mut GString {
    return safe_c2rust_g_string_insert_len(
        string,
        -(1 as ::core::ffi::c_int) as gssize,
        val,
        -(1 as ::core::ffi::c_int) as gssize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_len(
    mut string: *mut GString,
    mut val: *const gchar,
    mut len: gssize,
) -> *mut GString {
    return safe_c2rust_g_string_insert_len(string, -(1 as ::core::ffi::c_int) as gssize, val, len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_c(
    mut string: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    return safe_c2rust_g_string_insert_c(string, -(1 as ::core::ffi::c_int) as gssize, c);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_unichar(
    mut string: *mut GString,
    mut wc: gunichar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    return safe_c2rust_g_string_insert_unichar(string, -(1 as ::core::ffi::c_int) as gssize, wc);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_prepend(
    mut string: *mut GString,
    mut val: *const gchar,
) -> *mut GString {
    return safe_c2rust_g_string_insert_len(
        string,
        0 as gssize,
        val,
        -(1 as ::core::ffi::c_int) as gssize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_prepend_len(
    mut string: *mut GString,
    mut val: *const gchar,
    mut len: gssize,
) -> *mut GString {
    return safe_c2rust_g_string_insert_len(string, 0 as gssize, val, len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_prepend_c(
    mut string: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    return safe_c2rust_g_string_insert_c(string, 0 as gssize, c);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_prepend_unichar(
    mut string: *mut GString,
    mut wc: gunichar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    return safe_c2rust_g_string_insert_unichar(string, 0 as gssize, wc);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_insert(
    mut string: *mut GString,
    mut pos: gssize,
    mut val: *const gchar,
) -> *mut GString {
    return safe_c2rust_g_string_insert_len(string, pos, val, -(1 as ::core::ffi::c_int) as gssize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_insert_c(
    mut string: *mut GString,
    mut pos: gssize,
    mut c: gchar,
) -> *mut GString {
    let mut pos_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    safe_c2rust_g_string_maybe_expand(string, 1 as gsize);
    if pos < 0 as gssize {
        pos_unsigned = (*string).len;
    } else {
        pos_unsigned = pos as gsize;
        if ({
            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
            if pos_unsigned <= (*string).len {
                _g_boolean_var_27 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_27 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_27
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"pos_unsigned <= string->len\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return string;
        }
    }
    if pos_unsigned < (*string).len {
        memmove(
            (*string)
                .str_0
                .offset(pos_unsigned as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*string).str_0.offset(pos_unsigned as isize) as *const ::core::ffi::c_void,
            ((*string).len as size_t).wrapping_sub(pos_unsigned as size_t),
        );
    }
    *(*string).str_0.offset(pos_unsigned as isize) = c;
    (*string).len = (*string).len.wrapping_add(1 as gsize);
    *(*string).str_0.offset((*string).len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_insert_unichar(
    mut string: *mut GString,
    mut pos: gssize,
    mut wc: gunichar,
) -> *mut GString {
    let mut pos_unsigned: gsize = 0;
    let mut charlen: gint = 0;
    let mut first: gint = 0;
    let mut i: gint = 0;
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if wc < 0x80 as gunichar {
        first = 0 as ::core::ffi::c_int as gint;
        charlen = 1 as ::core::ffi::c_int as gint;
    } else if wc < 0x800 as gunichar {
        first = 0xc0 as ::core::ffi::c_int as gint;
        charlen = 2 as ::core::ffi::c_int as gint;
    } else if wc < 0x10000 as ::core::ffi::c_int as gunichar {
        first = 0xe0 as ::core::ffi::c_int as gint;
        charlen = 3 as ::core::ffi::c_int as gint;
    } else if wc < 0x200000 as ::core::ffi::c_int as gunichar {
        first = 0xf0 as ::core::ffi::c_int as gint;
        charlen = 4 as ::core::ffi::c_int as gint;
    } else if wc < 0x4000000 as ::core::ffi::c_int as gunichar {
        first = 0xf8 as ::core::ffi::c_int as gint;
        charlen = 5 as ::core::ffi::c_int as gint;
    } else {
        first = 0xfc as ::core::ffi::c_int as gint;
        charlen = 6 as ::core::ffi::c_int as gint;
    }
    safe_c2rust_g_string_maybe_expand(string, charlen as gsize);
    if pos < 0 as gssize {
        pos_unsigned = (*string).len;
    } else {
        pos_unsigned = pos as gsize;
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if pos_unsigned <= (*string).len {
                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_29
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"pos_unsigned <= string->len\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return string;
        }
    }
    if pos_unsigned < (*string).len {
        memmove(
            (*string)
                .str_0
                .offset(pos_unsigned as isize)
                .offset(charlen as isize) as *mut ::core::ffi::c_void,
            (*string).str_0.offset(pos_unsigned as isize) as *const ::core::ffi::c_void,
            ((*string).len as size_t).wrapping_sub(pos_unsigned as size_t),
        );
    }
    dest = (*string).str_0.offset(pos_unsigned as isize);
    i = (charlen as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint;
    while i > 0 as ::core::ffi::c_int {
        *dest.offset(i as isize) = (wc & 0x3f as gunichar | 0x80 as gunichar) as gchar;
        wc >>= 6 as ::core::ffi::c_int;
        i -= 1;
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = (wc | first as gunichar) as gchar;
    (*string).len = (*string).len.wrapping_add(charlen as gsize);
    *(*string).str_0.offset((*string).len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_overwrite(
    mut string: *mut GString,
    mut pos: gsize,
    mut val: *const gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"val != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    return safe_c2rust_g_string_overwrite_len(
        string,
        pos,
        val,
        strlen(val as *const ::core::ffi::c_char) as gssize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_overwrite_len(
    mut string: *mut GString,
    mut pos: gsize,
    mut val: *const gchar,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    let mut end: gsize = 0;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if len == 0 {
        return string;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !val.is_null() {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"val != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if pos <= (*string).len {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pos <= string->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val as *const ::core::ffi::c_char) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    end = pos.wrapping_add(len_unsigned);
    if end > (*string).len {
        safe_c2rust_g_string_maybe_expand(string, end.wrapping_sub((*string).len));
    }
    memcpy(
        (*string).str_0.offset(pos as isize) as *mut ::core::ffi::c_void,
        val as *const ::core::ffi::c_void,
        len_unsigned as size_t,
    );
    if end > (*string).len {
        *(*string).str_0.offset(end as isize) = '\0' as i32 as gchar;
        (*string).len = end;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_erase(
    mut string: *mut GString,
    mut pos: gssize,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    let mut pos_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if pos >= 0 as gssize {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pos >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    pos_unsigned = pos as gsize;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if pos_unsigned <= (*string).len {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pos_unsigned <= string->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return string;
    }
    if len < 0 as gssize {
        len_unsigned = (*string).len.wrapping_sub(pos_unsigned);
    } else {
        len_unsigned = len as gsize;
        if ({
            let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
            if pos_unsigned.wrapping_add(len_unsigned) <= (*string).len {
                _g_boolean_var_37 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_37 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_37
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"pos_unsigned + len_unsigned <= string->len\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return string;
        }
        if pos_unsigned.wrapping_add(len_unsigned) < (*string).len {
            memmove(
                (*string).str_0.offset(pos_unsigned as isize) as *mut ::core::ffi::c_void,
                (*string)
                    .str_0
                    .offset(pos_unsigned as isize)
                    .offset(len_unsigned as isize) as *const ::core::ffi::c_void,
                ((*string).len as size_t)
                    .wrapping_sub((pos_unsigned as size_t).wrapping_add(len_unsigned as size_t)),
            );
        }
    }
    (*string).len = (*string).len.wrapping_sub(len_unsigned);
    *(*string).str_0.offset((*string).len as isize) = 0 as gchar;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_replace(
    mut string: *mut GString,
    mut find: *const gchar,
    mut replace: *const gchar,
    mut limit: guint,
) -> guint {
    let mut f_len: gsize = 0;
    let mut r_len: gsize = 0;
    let mut pos: gsize = 0;
    let mut cur: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut next: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !find.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"find != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !replace.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"replace != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    f_len = strlen(find as *const ::core::ffi::c_char) as gsize;
    r_len = strlen(replace as *const ::core::ffi::c_char) as gsize;
    cur = (*string).str_0;
    loop {
        next = strstr(cur, find as *const ::core::ffi::c_char) as *mut gchar;
        if next.is_null() {
            break;
        }
        pos = next.offset_from((*string).str_0) as ::core::ffi::c_long as gsize;
        safe_c2rust_g_string_erase(string, pos as gssize, f_len as gssize);
        safe_c2rust_g_string_insert(string, pos as gssize, replace);
        cur = (*string).str_0.offset(pos as isize).offset(r_len as isize);
        n = n.wrapping_add(1);
        if f_len == 0 as gsize {
            if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
            cur = cur.offset(1);
        }
        if n == limit {
            break;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_ascii_down(mut string: *mut GString) -> *mut GString {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n: gint = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    n = (*string).len as gint;
    s = (*string).str_0;
    while n != 0 {
        *s = g_ascii_tolower(*s);
        s = s.offset(1);
        n -= 1;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_ascii_up(mut string: *mut GString) -> *mut GString {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n: gint = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    n = (*string).len as gint;
    s = (*string).str_0;
    while n != 0 {
        *s = g_ascii_toupper(*s);
        s = s.offset(1);
        n -= 1;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_down(mut string: *mut GString) -> *mut GString {
    let mut s: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut n: glong = 0;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    n = (*string).len as glong;
    s = (*string).str_0 as *mut guchar;
    while n != 0 {
        if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            *s = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<guchar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *s as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = safe_c2rust_tolower(*s as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*s as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as guchar;
        }
        s = s.offset(1);
        n -= 1;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_up(mut string: *mut GString) -> *mut GString {
    let mut s: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut n: glong = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GString>();
    }
    n = (*string).len as glong;
    s = (*string).str_0 as *mut guchar;
    while n != 0 {
        if *(*__ctype_b_loc()).offset(*s as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISlower as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            *s = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<guchar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *s as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = safe_c2rust_toupper(*s as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(*s as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as guchar;
        }
        s = s.offset(1);
        n -= 1;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_vprintf(
    mut string: *mut GString,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) {
    let mut buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gint = 0;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    len = g_vasprintf(&raw mut buf, format, args.clone());
    if len >= 0 as ::core::ffi::c_int {
        safe_c2rust_g_string_maybe_expand(string, len as gsize);
        memcpy(
            (*string).str_0.offset((*string).len as isize) as *mut ::core::ffi::c_void,
            buf as *const ::core::ffi::c_void,
            (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        (*string).len = (*string).len.wrapping_add(len as gsize);
        g_free(buf as gpointer);
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Failed to append to string: invalid format/args passed to g_vasprintf()\0"
                as *const u8 as *const gchar,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_vprintf(
    mut string: *mut GString,
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) {
    safe_c2rust_g_string_truncate_inline(string, 0 as gsize);
    safe_c2rust_g_string_append_vprintf(string, format, args.clone());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_printf(
    mut string: *mut GString,
    mut format: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    safe_c2rust_g_string_truncate_inline(string, 0 as gsize);
    args_0 = args.clone();
    safe_c2rust_g_string_append_vprintf(string, format, args_0.clone());
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_string_append_printf(
    mut string: *mut GString,
    mut format: *const gchar,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    safe_c2rust_g_string_append_vprintf(string, format, args_0.clone());
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
