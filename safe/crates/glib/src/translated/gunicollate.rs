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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> ::core::ffi::c_int;
    fn wcsxfrm(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn _g_utf8_normalize_wc(
        str: *const gchar,
        max_len: gssize,
        mode: GNormalizeMode,
    ) -> *mut gunichar;
    static safe_c2rust_g_ascii_table: *const guint16;
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
}
pub type size_t = usize;
pub type wchar_t = ::libc::wchar_t;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gunichar = guint32;
pub type GNormalizeMode = ::core::ffi::c_uint;
pub const G_NORMALIZE_NFKC: GNormalizeMode = 3;
pub const G_NORMALIZE_ALL_COMPOSE: GNormalizeMode = 3;
pub const G_NORMALIZE_NFKD: GNormalizeMode = 2;
pub const G_NORMALIZE_ALL: GNormalizeMode = 2;
pub const G_NORMALIZE_NFC: GNormalizeMode = 1;
pub const G_NORMALIZE_DEFAULT_COMPOSE: GNormalizeMode = 1;
pub const G_NORMALIZE_NFD: GNormalizeMode = 0;
pub const G_NORMALIZE_DEFAULT: GNormalizeMode = 0;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_utf8_collate\0" as *const u8 as *const ::core::ffi::c_char;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
        let fresh1 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh1 as isize) = c;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_collate(
    mut str1: *const gchar,
    mut str2: *const gchar,
) -> gint {
    let mut result: gint = 0;
    let mut str1_norm: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    let mut str2_norm: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !str1.is_null() {
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
            b"str1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !str2.is_null() {
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
            b"str2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    str1_norm = _g_utf8_normalize_wc(
        str1,
        -(1 as ::core::ffi::c_int) as gssize,
        G_NORMALIZE_ALL_COMPOSE,
    );
    str2_norm = _g_utf8_normalize_wc(
        str2,
        -(1 as ::core::ffi::c_int) as gssize,
        G_NORMALIZE_ALL_COMPOSE,
    );
    result = wcscoll(str1_norm as *mut wchar_t, str2_norm as *mut wchar_t) as gint;
    g_free(str1_norm as gpointer);
    g_free(str2_norm as gpointer);
    return result;
}
#[inline]
unsafe extern "C" fn safe_c2rust_utf8_encode(
    mut buf: *mut ::core::ffi::c_char,
    mut val: wchar_t,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    if val < 0x80 as wchar_t {
        if !buf.is_null() {
            let fresh0 = buf;
            buf = buf.offset(1);
            *fresh0 = val as ::core::ffi::c_char;
        }
        retval = 1 as ::core::ffi::c_int;
    } else {
        let mut step: ::core::ffi::c_int = 0;
        step = 2 as ::core::ffi::c_int;
        while step < 6 as ::core::ffi::c_int {
            if val as guint32
                & !(0 as ::core::ffi::c_int as guint32)
                    << 5 as ::core::ffi::c_int * step + 1 as ::core::ffi::c_int
                == 0 as guint32
            {
                break;
            }
            step += 1;
        }
        retval = step;
        if !buf.is_null() {
            *buf = (!(0xff as ::core::ffi::c_int) >> step) as ::core::ffi::c_uchar
                as ::core::ffi::c_char;
            step -= 1;
            loop {
                *buf.offset(step as isize) =
                    (0x80 as wchar_t | val & 0x3f as wchar_t) as ::core::ffi::c_char;
                val >>= 6 as ::core::ffi::c_int;
                step -= 1;
                if !(step > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            *buf = (*buf as ::core::ffi::c_int | val as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_collate_key(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut xfrm_len: gsize = 0;
    let mut str_norm: *mut gunichar = ::core::ptr::null_mut::<gunichar>();
    let mut result_wc: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut i: gsize = 0;
    let mut result_len: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    str_norm = _g_utf8_normalize_wc(str, len, G_NORMALIZE_ALL_COMPOSE);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !str_norm.is_null() {
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
            b"str_norm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    xfrm_len = wcsxfrm(
        ::core::ptr::null_mut::<wchar_t>(),
        str_norm as *mut wchar_t,
        0 as size_t,
    ) as gsize;
    result_wc = ({
        let mut __n: gsize = xfrm_len.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<wchar_t>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut wchar_t;
    wcsxfrm(
        result_wc,
        str_norm as *mut wchar_t,
        (xfrm_len as size_t).wrapping_add(1 as size_t),
    );
    i = 0 as gsize;
    while i < xfrm_len {
        result_len = result_len.wrapping_add(safe_c2rust_utf8_encode(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            *result_wc.offset(i as isize),
        ) as gsize);
        i = i.wrapping_add(1);
    }
    result = g_malloc(result_len.wrapping_add(1 as gsize)) as *mut gchar;
    result_len = 0 as gsize;
    i = 0 as gsize;
    while i < xfrm_len {
        result_len = result_len.wrapping_add(safe_c2rust_utf8_encode(
            result.offset(result_len as isize),
            *result_wc.offset(i as isize),
        ) as gsize);
        i = i.wrapping_add(1);
    }
    *result.offset(result_len as isize) = '\0' as i32 as gchar;
    g_free(result_wc as gpointer);
    g_free(str_norm as gpointer);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utf8_collate_key_for_filename(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut result: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut append: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut prev: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    let mut collate_key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut digits: gint = 0;
    let mut leading_zeros: gint = 0;
    if len < 0 as gssize {
        len = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    result = g_string_sized_new((len * 2 as gssize) as gsize);
    append = g_string_sized_new(0 as gsize);
    end = str.offset(len as isize);
    p = str;
    prev = p;
    while p < end {
        match *p as ::core::ffi::c_int {
            46 => {
                if prev != p {
                    collate_key =
                        safe_c2rust_g_utf8_collate_key(prev, p.offset_from(prev) as gssize);
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = collate_key;
                            safe_c2rust_g_string_append_len_inline(
                                result,
                                __val,
                                if ({
                                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_12
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            result,
                            collate_key,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(collate_key as gpointer);
                }
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\x01\x01\x01\x01\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            result,
                            __val,
                            if ({
                                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_13
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
                        result,
                        b"\x01\x01\x01\x01\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                prev = p.offset(1 as ::core::ffi::c_int as isize);
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if prev != p {
                    collate_key =
                        safe_c2rust_g_utf8_collate_key(prev, p.offset_from(prev) as gssize);
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = collate_key;
                            safe_c2rust_g_string_append_len_inline(
                                result,
                                __val,
                                if ({
                                    let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_14
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            result,
                            collate_key,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(collate_key as gpointer);
                }
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\x01\x01\x01\x02\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            result,
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
                        result,
                        b"\x01\x01\x01\x02\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                prev = p;
                if *p as ::core::ffi::c_int == '0' as i32 {
                    leading_zeros = 1 as ::core::ffi::c_int as gint;
                    digits = 0 as ::core::ffi::c_int as gint;
                } else {
                    leading_zeros = 0 as ::core::ffi::c_int as gint;
                    digits = 1 as ::core::ffi::c_int as gint;
                }
                loop {
                    p = p.offset(1);
                    if !(p < end) {
                        break;
                    }
                    if *p as ::core::ffi::c_int == '0' as i32 && digits == 0 {
                        leading_zeros += 1;
                    } else if *safe_c2rust_g_ascii_table.offset(*p as guchar as isize)
                        as ::core::ffi::c_int
                        & G_ASCII_DIGIT as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        digits += 1;
                    } else {
                        if digits == 0 {
                            digits += 1;
                            leading_zeros -= 1;
                        }
                        break;
                    }
                }
                while digits > 1 as ::core::ffi::c_int {
                    safe_c2rust_g_string_append_c_inline(result, ':' as i32 as gchar);
                    digits -= 1;
                }
                if leading_zeros > 0 as ::core::ffi::c_int {
                    safe_c2rust_g_string_append_c_inline(append, leading_zeros as gchar);
                    prev = prev.offset(leading_zeros as isize);
                }
                safe_c2rust_g_string_append_len_inline(
                    result,
                    prev as *const ::core::ffi::c_char,
                    p.offset_from(prev) as gssize,
                );
                prev = p;
                p = p.offset(-1);
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if prev != p {
        collate_key = safe_c2rust_g_utf8_collate_key(prev, p.offset_from(prev) as gssize);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = collate_key;
                safe_c2rust_g_string_append_len_inline(
                    result,
                    __val,
                    if ({
                        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_16
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
                result,
                collate_key,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(collate_key as gpointer);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = (*append).str_0;
            safe_c2rust_g_string_append_len_inline(
                result,
                __val,
                if ({
                    let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_17
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            result,
            (*append).str_0,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(append, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(append);
        };
    } else {
        g_string_free(append, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(result, 0 as gboolean)
        } else {
            g_string_free_and_steal(result)
        }
    } else {
        g_string_free(result, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
