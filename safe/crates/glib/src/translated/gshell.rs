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
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_string_new(init: *const gchar) -> *mut GString;
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
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub const G_SHELL_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_SHELL_ERROR_EMPTY_STRING: C2RustUnnamed = 1;
pub const G_SHELL_ERROR_BAD_QUOTING: C2RustUnnamed = 0;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
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
pub unsafe extern "C" fn safe_c2rust_g_shell_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-shell-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
unsafe extern "C" fn safe_c2rust_unquote_string_inplace(
    mut str: *mut gchar,
    mut end: *mut *mut gchar,
    mut err: *mut *mut GError,
) -> gboolean {
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut quote_char: gchar = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !end.is_null() {
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
            b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if err.is_null() || (*err).is_null() {
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
            b"err == NULL || *err == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    s = str;
    dest = s;
    quote_char = *s;
    if !(*s as ::core::ffi::c_int == '"' as i32 || *s as ::core::ffi::c_int == '\'' as i32) {
        g_set_error_literal(
            err,
            safe_c2rust_g_shell_error_quark(),
            G_SHELL_ERROR_BAD_QUOTING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Quoted text doesn\xE2\x80\x99t begin with a quotation mark\0" as *const u8
                    as *const gchar,
            ),
        );
        *end = str;
        return FALSE;
    }
    s = s.offset(1);
    if quote_char as ::core::ffi::c_int == '"' as i32 {
        while *s != 0 {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if s > dest {
                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_12
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    96 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"s > dest\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            match *s as ::core::ffi::c_int {
                34 => {
                    *dest = '\0' as i32 as gchar;
                    s = s.offset(1);
                    *end = s;
                    return TRUE;
                }
                92 => {
                    s = s.offset(1);
                    match *s as ::core::ffi::c_int {
                        34 | 92 | 96 | 36 | 10 => {
                            *dest = *s;
                            s = s.offset(1);
                            dest = dest.offset(1);
                        }
                        _ => {
                            *dest = '\\' as i32 as gchar;
                            dest = dest.offset(1);
                        }
                    }
                }
                _ => {
                    *dest = *s;
                    dest = dest.offset(1);
                    s = s.offset(1);
                }
            }
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if s > dest {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"s > dest\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    } else {
        while *s != 0 {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if s > dest {
                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_14
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    146 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"s > dest\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if *s as ::core::ffi::c_int == '\'' as i32 {
                *dest = '\0' as i32 as gchar;
                s = s.offset(1);
                *end = s;
                return TRUE;
            } else {
                *dest = *s;
                dest = dest.offset(1);
                s = s.offset(1);
            }
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if s > dest {
                    _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_15
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"s > dest\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    *dest = '\0' as i32 as gchar;
    g_set_error_literal(
        err,
        safe_c2rust_g_shell_error_quark(),
        G_SHELL_ERROR_BAD_QUOTING as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Unmatched quotation mark in command line or other shell-quoted text\0" as *const u8
                as *const gchar,
        ),
    );
    *end = s;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_shell_quote(
    mut unquoted_string: *const gchar,
) -> *mut gchar {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut dest: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !unquoted_string.is_null() {
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
            b"unquoted_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    dest = g_string_new(b"'\0" as *const u8 as *const gchar);
    p = unquoted_string;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '\'' as i32 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"'\\''\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        dest,
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
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    dest,
                    b"'\\''\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            safe_c2rust_g_string_append_c_inline(dest, *p);
        }
        p = p.offset(1);
    }
    safe_c2rust_g_string_append_c_inline(dest, '\'' as i32 as gchar);
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(dest, 0 as gboolean)
        } else {
            g_string_free_and_steal(dest)
        }
    } else {
        g_string_free(dest, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_shell_unquote(
    mut quoted_string: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut unquoted: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut start: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut retval: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !quoted_string.is_null() {
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
            b"quoted_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    unquoted =
        safe_c2rust_g_strdup_inline(quoted_string as *const ::core::ffi::c_char) as *mut gchar;
    start = unquoted;
    end = unquoted;
    retval = g_string_new(::core::ptr::null::<gchar>());
    loop {
        if !(*start != 0) {
            current_block = 7056779235015430508;
            break;
        }
        while *start as ::core::ffi::c_int != 0
            && !(*start as ::core::ffi::c_int == '"' as i32
                || *start as ::core::ffi::c_int == '\'' as i32)
        {
            if *start as ::core::ffi::c_int == '\\' as i32 {
                start = start.offset(1);
                if *start != 0 {
                    if *start as ::core::ffi::c_int != '\n' as i32 {
                        safe_c2rust_g_string_append_c_inline(retval, *start);
                    }
                    start = start.offset(1);
                }
            } else {
                safe_c2rust_g_string_append_c_inline(retval, *start);
                start = start.offset(1);
            }
        }
        if !(*start != 0) {
            continue;
        }
        if safe_c2rust_unquote_string_inplace(start, &raw mut end, error) == 0 {
            current_block = 672240105093197796;
            break;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = start;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_19
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
                retval,
                start,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        start = end;
    }
    match current_block {
        672240105093197796 => {
            if ({
                let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                if error.is_null() || !(*error).is_null() {
                    _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_20
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    335 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error == NULL || *error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_free(unquoted as gpointer);
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(retval, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(retval);
                };
            } else {
                g_string_free(retval, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
            return ::core::ptr::null_mut::<gchar>();
        }
        _ => {
            g_free(unquoted as gpointer);
            return if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(retval, 0 as gboolean)
                } else {
                    g_string_free_and_steal(retval)
                }
            } else {
                g_string_free(retval, 0 as gboolean)
            };
        }
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_ensure_token(mut token: *mut *mut GString) {
    if (*token).is_null() {
        *token = g_string_new(::core::ptr::null::<gchar>());
    }
}
unsafe extern "C" fn safe_c2rust_delimit_token(
    mut token: *mut *mut GString,
    mut retval: *mut *mut GSList,
) {
    if (*token).is_null() {
        return;
    }
    *retval = g_slist_prepend(
        *retval,
        (if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(*token, 0 as gboolean)
            } else {
                g_string_free_and_steal(*token)
            }
        } else {
            g_string_free(*token, 0 as gboolean)
        }) as gpointer,
    );
    *token = ::core::ptr::null_mut::<GString>();
}
unsafe extern "C" fn safe_c2rust_tokenize_command_line(
    mut command_line: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GSList {
    let mut current_quote: gchar = 0;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut current_token: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut retval: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut quoted: gboolean = 0;
    current_quote = '\0' as i32 as gchar;
    quoted = FALSE as gboolean;
    p = command_line;
    while *p != 0 {
        if current_quote as ::core::ffi::c_int == '\\' as i32 {
            if !(*p as ::core::ffi::c_int == '\n' as i32) {
                safe_c2rust_ensure_token(&raw mut current_token);
                safe_c2rust_g_string_append_c_inline(current_token, '\\' as i32 as gchar);
                safe_c2rust_g_string_append_c_inline(current_token, *p);
            }
            current_quote = '\0' as i32 as gchar;
        } else if current_quote as ::core::ffi::c_int == '#' as i32 {
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\n' as i32 {
                p = p.offset(1);
            }
            current_quote = '\0' as i32 as gchar;
            if *p as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
        } else if current_quote != 0 {
            if *p as ::core::ffi::c_int == current_quote as ::core::ffi::c_int
                && !(current_quote as ::core::ffi::c_int == '"' as i32 && quoted != 0)
            {
                current_quote = '\0' as i32 as gchar;
            }
            safe_c2rust_ensure_token(&raw mut current_token);
            safe_c2rust_g_string_append_c_inline(current_token, *p);
        } else {
            let mut current_block_34: u64;
            match *p as ::core::ffi::c_int {
                10 => {
                    safe_c2rust_delimit_token(&raw mut current_token, &raw mut retval);
                    current_block_34 = 7427571413727699167;
                }
                32 | 9 => {
                    if !current_token.is_null() && (*current_token).len > 0 as gsize {
                        safe_c2rust_delimit_token(&raw mut current_token, &raw mut retval);
                    }
                    current_block_34 = 7427571413727699167;
                }
                39 | 34 => {
                    safe_c2rust_ensure_token(&raw mut current_token);
                    safe_c2rust_g_string_append_c_inline(current_token, *p);
                    current_block_34 = 1856533575870289582;
                }
                92 => {
                    current_block_34 = 1856533575870289582;
                }
                35 => {
                    if p == command_line {
                        current_quote = *p;
                    } else {
                        match *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int {
                            32 | 10 | 0 => {
                                current_quote = *p;
                            }
                            _ => {
                                safe_c2rust_ensure_token(&raw mut current_token);
                                safe_c2rust_g_string_append_c_inline(current_token, *p);
                            }
                        }
                    }
                    current_block_34 = 7427571413727699167;
                }
                _ => {
                    safe_c2rust_ensure_token(&raw mut current_token);
                    safe_c2rust_g_string_append_c_inline(current_token, *p);
                    current_block_34 = 7427571413727699167;
                }
            }
            match current_block_34 {
                1856533575870289582 => {
                    current_quote = *p;
                }
                _ => {}
            }
        }
        if *p as ::core::ffi::c_int != '\\' as i32 {
            quoted = FALSE as gboolean;
        } else {
            quoted = (quoted == 0) as ::core::ffi::c_int as gboolean;
        }
        p = p.offset(1);
    }
    safe_c2rust_delimit_token(&raw mut current_token, &raw mut retval);
    if current_quote != 0 {
        if current_quote as ::core::ffi::c_int == '\\' as i32 {
            g_set_error(
                error,
                safe_c2rust_g_shell_error_quark(),
                G_SHELL_ERROR_BAD_QUOTING as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Text ended just after a \xE2\x80\x9C\\\xE2\x80\x9D character. (The text was \xE2\x80\x9C%s\xE2\x80\x9D)\0"
                        as *const u8 as *const gchar,
                ),
                command_line,
            );
        } else {
            g_set_error(
                error,
                safe_c2rust_g_shell_error_quark(),
                G_SHELL_ERROR_BAD_QUOTING as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Text ended before matching quote was found for %c. (The text was \xE2\x80\x9C%s\xE2\x80\x9D)\0"
                        as *const u8 as *const gchar,
                ),
                current_quote as ::core::ffi::c_int,
                command_line,
            );
        }
    } else if retval.is_null() {
        g_set_error_literal(
            error,
            safe_c2rust_g_shell_error_quark(),
            G_SHELL_ERROR_EMPTY_STRING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Text was empty (or contained only whitespace)\0" as *const u8 as *const gchar,
            ),
        );
    } else {
        retval = g_slist_reverse(retval);
        return retval;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if error.is_null() || !(*error).is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
            610 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error == NULL || *error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_slist_free_full(retval, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    return ::core::ptr::null_mut::<GSList>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_shell_parse_argv(
    mut command_line: *const gchar,
    mut argcp: *mut gint,
    mut argvp: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut argc: gint = 0 as gint;
    let mut argv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut tokens: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut i: gint = 0;
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !command_line.is_null() {
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
            b"command_line != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    tokens = safe_c2rust_tokenize_command_line(command_line, error);
    if tokens.is_null() {
        return FALSE;
    }
    argc = g_slist_length(tokens) as gint;
    argv = ({
        let mut __n: gsize = (argc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut gchar;
    i = 0 as ::core::ffi::c_int as gint;
    tmp_list = tokens;
    loop {
        if tmp_list.is_null() {
            current_block = 8831408221741692167;
            break;
        }
        let ref mut fresh1 = *argv.offset(i as isize);
        *fresh1 = safe_c2rust_g_shell_unquote((*tmp_list).data as *const gchar, error);
        if (*argv.offset(i as isize)).is_null() {
            current_block = 12590546485753900404;
            break;
        }
        tmp_list = if !tmp_list.is_null() {
            (*tmp_list).next
        } else {
            ::core::ptr::null_mut::<GSList>()
        };
        i += 1;
    }
    match current_block {
        12590546485753900404 => {
            if ({
                let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                if error.is_null() || !(*error).is_null() {
                    _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_25
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    714 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error == NULL || *error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_strfreev(argv);
            g_slist_free_full(tokens, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            return FALSE;
        }
        _ => {
            g_slist_free_full(tokens, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
            if ({
                let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                if argc > 0 as ::core::ffi::c_int {
                    _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_23
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    699 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"argc > 0\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                if !argv.is_null() && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null() {
                    _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_24
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gshell.c\0" as *const u8 as *const ::core::ffi::c_char,
                    700 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"argv != NULL && argv[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !argcp.is_null() {
                *argcp = argc;
            }
            if !argvp.is_null() {
                *argvp = argv;
            } else {
                g_strfreev(argv);
            }
            return TRUE;
        }
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_shell_quote\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
