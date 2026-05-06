extern "C" {
    pub type _GVariantType;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
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
}
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GVariantType = _GVariantType;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
        let fresh6 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh6 as isize) = c;
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
pub const G_VARIANT_MAX_RECURSION_DEPTH: gsize = 128 as ::core::ffi::c_int as gsize;
unsafe extern "C" fn safe_c2rust_g_variant_type_check(mut type_0: *const GVariantType) -> gboolean {
    if type_0.is_null() {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_variant_type_string_scan_internal(
    mut string: *const gchar,
    mut limit: *const gchar,
    mut endptr: *mut *const gchar,
    mut depth: *mut gsize,
    mut depth_limit: gsize,
) -> gboolean {
    let mut max_depth: gsize = 0 as gsize;
    let mut child_depth: gsize = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if string == limit || *string as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    let fresh0 = string;
    string = string.offset(1);
    match *fresh0 as ::core::ffi::c_int {
        40 => {
            while string == limit || *string as ::core::ffi::c_int != ')' as i32 {
                if depth_limit == 0 as gsize
                    || safe_c2rust_variant_type_string_scan_internal(
                        string,
                        limit,
                        &raw mut string,
                        &raw mut child_depth,
                        depth_limit.wrapping_sub(1 as gsize),
                    ) == 0
                {
                    return FALSE;
                }
                max_depth = if max_depth > child_depth.wrapping_add(1 as gsize) {
                    max_depth
                } else {
                    child_depth.wrapping_add(1 as gsize)
                };
            }
            string = string.offset(1);
        }
        123 => {
            if depth_limit == 0 as gsize
                || string == limit
                || *string as ::core::ffi::c_int == '\0' as i32
                || {
                    let fresh1 = string;
                    string = string.offset(1);
                    strchr(
                        b"bynqihuxtdsog?\0" as *const u8 as *const ::core::ffi::c_char,
                        *fresh1 as ::core::ffi::c_int,
                    )
                    .is_null()
                }
                || safe_c2rust_variant_type_string_scan_internal(
                    string,
                    limit,
                    &raw mut string,
                    &raw mut child_depth,
                    depth_limit.wrapping_sub(1 as gsize),
                ) == 0
                || string == limit
                || {
                    let fresh2 = string;
                    string = string.offset(1);
                    *fresh2 as ::core::ffi::c_int != '}' as i32
                }
            {
                return FALSE;
            }
            max_depth = if max_depth > child_depth.wrapping_add(1 as gsize) {
                max_depth
            } else {
                child_depth.wrapping_add(1 as gsize)
            };
        }
        109 | 97 => {
            if depth_limit == 0 as gsize
                || safe_c2rust_variant_type_string_scan_internal(
                    string,
                    limit,
                    &raw mut string,
                    &raw mut child_depth,
                    depth_limit.wrapping_sub(1 as gsize),
                ) == 0
            {
                return FALSE;
            }
            max_depth = if max_depth > child_depth.wrapping_add(1 as gsize) {
                max_depth
            } else {
                child_depth.wrapping_add(1 as gsize)
            };
        }
        98 | 121 | 110 | 113 | 105 | 117 | 120 | 116 | 100 | 115 | 111 | 103 | 118 | 114 | 42
        | 63 | 104 => {
            max_depth = if max_depth > 1 as gsize {
                max_depth
            } else {
                1 as gsize
            };
        }
        _ => return FALSE,
    }
    if !endptr.is_null() {
        *endptr = string;
    }
    if !depth.is_null() {
        *depth = max_depth;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_string_scan(
    mut string: *const gchar,
    mut limit: *const gchar,
    mut endptr: *mut *const gchar,
) -> gboolean {
    return safe_c2rust_variant_type_string_scan_internal(
        string,
        limit,
        endptr,
        ::core::ptr::null_mut::<gsize>(),
        G_VARIANT_MAX_RECURSION_DEPTH,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_string_get_depth_(
    mut type_string: *const gchar,
) -> gsize {
    let mut endptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut depth: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !type_string.is_null() {
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
            b"type_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if safe_c2rust_variant_type_string_scan_internal(
        type_string,
        ::core::ptr::null::<gchar>(),
        &raw mut endptr,
        &raw mut depth,
        G_VARIANT_MAX_RECURSION_DEPTH,
    ) == 0
        || *endptr as ::core::ffi::c_int != '\0' as i32
    {
        return 0 as gsize;
    }
    return depth;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_string_is_valid(
    mut type_string: *const gchar,
) -> gboolean {
    let mut endptr: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !type_string.is_null() {
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
            b"type_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_variant_type_string_scan(
        type_string,
        ::core::ptr::null::<gchar>(),
        &raw mut endptr,
    ) == 0
    {
        return FALSE;
    }
    return (*endptr as ::core::ffi::c_int == '\0' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_free(mut type_0: *mut GVariantType) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if type_0.is_null() || safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"type == NULL || g_variant_type_check (type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free(type_0 as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_copy(
    mut type_0: *const GVariantType,
) -> *mut GVariantType {
    let mut length: gsize = 0;
    let mut new: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    length = safe_c2rust_g_variant_type_get_string_length(type_0);
    new = g_malloc(length.wrapping_add(1 as gsize)) as *mut gchar;
    memcpy(
        new as *mut ::core::ffi::c_void,
        type_0 as *const ::core::ffi::c_void,
        length as size_t,
    );
    *new.offset(length as isize) = '\0' as i32 as gchar;
    return new as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_new(
    mut type_string: *const gchar,
) -> *mut GVariantType {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !type_string.is_null() {
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
            b"type_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    return safe_c2rust_g_variant_type_copy(safe_c2rust_g_variant_type_checked_(type_string));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_get_string_length(
    mut type_0: *const GVariantType,
) -> gsize {
    let mut type_string: *const gchar = type_0 as *const gchar;
    let mut brackets: gint = 0 as gint;
    let mut index: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    loop {
        while *type_string.offset(index as isize) as ::core::ffi::c_int == 'a' as i32
            || *type_string.offset(index as isize) as ::core::ffi::c_int == 'm' as i32
        {
            index = index.wrapping_add(1);
        }
        if *type_string.offset(index as isize) as ::core::ffi::c_int == '(' as i32
            || *type_string.offset(index as isize) as ::core::ffi::c_int == '{' as i32
        {
            brackets += 1;
        } else if *type_string.offset(index as isize) as ::core::ffi::c_int == ')' as i32
            || *type_string.offset(index as isize) as ::core::ffi::c_int == '}' as i32
        {
            brackets -= 1;
        }
        index = index.wrapping_add(1);
        if !(brackets != 0) {
            break;
        }
    }
    return index;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_peek_string(
    mut type_0: *const GVariantType,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return type_0 as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_dup_string(
    mut type_0: *const GVariantType,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return g_strndup(
        safe_c2rust_g_variant_type_peek_string(type_0),
        safe_c2rust_g_variant_type_get_string_length(type_0),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_definite(
    mut type_0: *const GVariantType,
) -> gboolean {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    let mut type_length: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    type_length = safe_c2rust_g_variant_type_get_string_length(type_0);
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    i = 0 as gsize;
    while i < type_length {
        if *type_string.offset(i as isize) as ::core::ffi::c_int == '*' as i32
            || *type_string.offset(i as isize) as ::core::ffi::c_int == '?' as i32
            || *type_string.offset(i as isize) as ::core::ffi::c_int == 'r' as i32
        {
            return FALSE;
        }
        i = i.wrapping_add(1);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_container(
    mut type_0: *const GVariantType,
) -> gboolean {
    let mut first_char: gchar = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    first_char =
        *safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize);
    match first_char as ::core::ffi::c_int {
        97 | 109 | 114 | 40 | 123 | 118 => return TRUE,
        _ => return FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_basic(
    mut type_0: *const GVariantType,
) -> gboolean {
    let mut first_char: gchar = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    first_char =
        *safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize);
    match first_char as ::core::ffi::c_int {
        98 | 121 | 110 | 113 | 105 | 104 | 117 | 116 | 120 | 100 | 115 | 111 | 103 | 63 => {
            return TRUE
        }
        _ => return FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_maybe(
    mut type_0: *const GVariantType,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'm' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_array(
    mut type_0: *const GVariantType,
) -> gboolean {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'a' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_tuple(
    mut type_0: *const GVariantType,
) -> gboolean {
    let mut type_char: gchar = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    type_char =
        *safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize);
    return (type_char as ::core::ffi::c_int == 'r' as i32
        || type_char as ::core::ffi::c_int == '(' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_dict_entry(
    mut type_0: *const GVariantType,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '{' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_variant(
    mut type_0: *const GVariantType,
) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*safe_c2rust_g_variant_type_peek_string(type_0).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'v' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_hash(mut type_0: gconstpointer) -> guint {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    let mut value: guint = 0 as guint;
    let mut length: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0 as *const GVariantType) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0 as *const GVariantType);
    length = safe_c2rust_g_variant_type_get_string_length(type_0 as *const GVariantType);
    i = 0 as gsize;
    while i < length {
        value = (value << 5 as ::core::ffi::c_int)
            .wrapping_sub(value)
            .wrapping_add(*type_string.offset(i as isize) as guint);
        i = i.wrapping_add(1);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_equal(
    mut type1: gconstpointer,
    mut type2: gconstpointer,
) -> gboolean {
    let mut string1: *const gchar = ::core::ptr::null::<gchar>();
    let mut string2: *const gchar = ::core::ptr::null::<gchar>();
    let mut size1: gsize = 0;
    let mut size2: gsize = 0;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type1 as *const GVariantType) != 0 {
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
            b"g_variant_type_check (type1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type2 as *const GVariantType) != 0 {
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
            b"g_variant_type_check (type2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if type1 == type2 {
        return TRUE;
    }
    size1 = safe_c2rust_g_variant_type_get_string_length(type1 as *const GVariantType);
    size2 = safe_c2rust_g_variant_type_get_string_length(type2 as *const GVariantType);
    if size1 != size2 {
        return FALSE;
    }
    string1 = safe_c2rust_g_variant_type_peek_string(type1 as *const GVariantType);
    string2 = safe_c2rust_g_variant_type_peek_string(type2 as *const GVariantType);
    return (memcmp(
        string1 as *const ::core::ffi::c_void,
        string2 as *const ::core::ffi::c_void,
        size1 as size_t,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_is_subtype_of(
    mut type_0: *const GVariantType,
    mut supertype: *const GVariantType,
) -> gboolean {
    let mut supertype_string: *const gchar = ::core::ptr::null::<gchar>();
    let mut supertype_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(supertype) != 0 {
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
            b"g_variant_type_check (supertype)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    supertype_string = safe_c2rust_g_variant_type_peek_string(supertype);
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    supertype_end =
        supertype_string.offset(safe_c2rust_g_variant_type_get_string_length(supertype) as isize);
    while supertype_string < supertype_end {
        let fresh3 = supertype_string;
        supertype_string = supertype_string.offset(1);
        let mut supertype_char: ::core::ffi::c_char = *fresh3;
        if supertype_char as ::core::ffi::c_int == *type_string as ::core::ffi::c_int {
            type_string = type_string.offset(1);
        } else if *type_string as ::core::ffi::c_int == ')' as i32 {
            return FALSE;
        } else {
            let mut target_type: *const GVariantType = type_string as *mut GVariantType;
            match supertype_char as ::core::ffi::c_int {
                114 => {
                    if safe_c2rust_g_variant_type_is_tuple(target_type) == 0 {
                        return FALSE;
                    }
                }
                42 => {}
                63 => {
                    if safe_c2rust_g_variant_type_is_basic(target_type) == 0 {
                        return FALSE;
                    }
                }
                _ => return FALSE,
            }
            type_string = type_string
                .offset(safe_c2rust_g_variant_type_get_string_length(target_type) as isize);
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_element(
    mut type_0: *const GVariantType,
) -> *const GVariantType {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'a' as i32
            || *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'm' as i32
        {
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
            b"../original/glib/gvarianttype.c\0" as *const u8 as *const ::core::ffi::c_char,
            937 as ::core::ffi::c_int,
            G_STRFUNC,
            b"type_string[0] == 'a' || type_string[0] == 'm'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return type_string.offset(1 as ::core::ffi::c_int as isize) as *const gchar
        as *const GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_first(
    mut type_0: *const GVariantType,
) -> *const GVariantType {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '(' as i32
            || *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '{' as i32
        {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttype.c\0" as *const u8 as *const ::core::ffi::c_char,
            973 as ::core::ffi::c_int,
            G_STRFUNC,
            b"type_string[0] == '(' || type_string[0] == '{'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if *type_string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ')' as i32 {
        return ::core::ptr::null::<GVariantType>();
    }
    return type_string.offset(1 as ::core::ffi::c_int as isize) as *const gchar
        as *const GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_next(
    mut type_0: *const GVariantType,
) -> *const GVariantType {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    type_string = type_string.offset(safe_c2rust_g_variant_type_get_string_length(type_0) as isize);
    if *type_string as ::core::ffi::c_int == ')' as i32
        || *type_string as ::core::ffi::c_int == '}' as i32
    {
        return ::core::ptr::null::<GVariantType>();
    }
    return type_string as *const GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_n_items(
    mut type_0: *const GVariantType,
) -> gsize {
    let mut count: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    type_0 = safe_c2rust_g_variant_type_first(type_0);
    while !type_0.is_null() {
        count = count.wrapping_add(1);
        type_0 = safe_c2rust_g_variant_type_next(type_0);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_key(
    mut type_0: *const GVariantType,
) -> *const GVariantType {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '{' as i32
        {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttype.c\0" as *const u8 as *const ::core::ffi::c_char,
            1072 as ::core::ffi::c_int,
            G_STRFUNC,
            b"type_string[0] == '{'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return type_string.offset(1 as ::core::ffi::c_int as isize) as *const gchar
        as *const GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_value(
    mut type_0: *const GVariantType,
) -> *const GVariantType {
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(type_0) != 0 {
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
            b"g_variant_type_check (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_string = safe_c2rust_g_variant_type_peek_string(type_0);
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '{' as i32
        {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttype.c\0" as *const u8 as *const ::core::ffi::c_char,
            1100 as ::core::ffi::c_int,
            G_STRFUNC,
            b"type_string[0] == '{'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_g_variant_type_next(safe_c2rust_g_variant_type_key(type_0));
}
unsafe extern "C" fn safe_c2rust_g_variant_type_new_tuple_slow(
    mut items: *const *const GVariantType,
    mut length: gint,
) -> *mut GVariantType {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gint = 0;
    string = g_string_new(b"(\0" as *const u8 as *const gchar);
    i = 0 as ::core::ffi::c_int as gint;
    while i < length {
        let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut size: gsize = 0;
        if ({
            let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
            if safe_c2rust_g_variant_type_check(*items.offset(i as isize)) != 0 {
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
                b"g_variant_type_check (items[i])\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GVariantType>();
        }
        type_0 = *items.offset(i as isize);
        size = safe_c2rust_g_variant_type_get_string_length(type_0);
        safe_c2rust_g_string_append_len_inline(
            string,
            type_0 as *const ::core::ffi::c_char,
            size as gssize,
        );
        i += 1;
    }
    safe_c2rust_g_string_append_c_inline(string, ')' as i32 as gchar);
    return (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean)
        } else {
            g_string_free_and_steal(string)
        }
    } else {
        g_string_free(string, 0 as gboolean)
    }) as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_new_tuple(
    mut items: *const *const GVariantType,
    mut length: gint,
) -> *mut GVariantType {
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    let mut length_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if length == 0 as ::core::ffi::c_int || !items.is_null() {
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
            b"length == 0 || items != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    if length < 0 as ::core::ffi::c_int {
        length_unsigned = 0 as gsize;
        while !(*items.offset(length_unsigned as isize)).is_null() {
            length_unsigned = length_unsigned.wrapping_add(1);
        }
    } else {
        length_unsigned = length as gsize;
    }
    offset = 0 as gsize;
    let fresh4 = offset;
    offset = offset.wrapping_add(1);
    buffer[fresh4 as usize] = '(' as i32 as ::core::ffi::c_char;
    i = 0 as gsize;
    while i < length_unsigned {
        let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut size: gsize = 0;
        if ({
            let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
            if safe_c2rust_g_variant_type_check(*items.offset(i as isize)) != 0 {
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
                b"g_variant_type_check (items[i])\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GVariantType>();
        }
        type_0 = *items.offset(i as isize);
        size = safe_c2rust_g_variant_type_get_string_length(type_0);
        if (offset as usize).wrapping_add(size as usize)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize
        {
            return safe_c2rust_g_variant_type_new_tuple_slow(items, length_unsigned as gint);
        }
        memcpy(
            (&raw mut buffer as *mut ::core::ffi::c_char).offset(offset as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            type_0 as *const ::core::ffi::c_void,
            size as size_t,
        );
        offset = offset.wrapping_add(size);
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if (offset as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvarianttype.c\0" as *const u8 as *const ::core::ffi::c_char,
            1186 as ::core::ffi::c_int,
            G_STRFUNC,
            b"offset < sizeof buffer\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let fresh5 = offset;
    offset = offset.wrapping_add(1);
    buffer[fresh5 as usize] = ')' as i32 as ::core::ffi::c_char;
    return g_memdup2(
        &raw mut buffer as *mut ::core::ffi::c_char as gconstpointer,
        offset,
    ) as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_new_array(
    mut element: *const GVariantType,
) -> *mut GVariantType {
    let mut size: gsize = 0;
    let mut new: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(element) != 0 {
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
            b"g_variant_type_check (element)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    size = safe_c2rust_g_variant_type_get_string_length(element);
    new = g_malloc(size.wrapping_add(1 as gsize)) as *mut gchar;
    *new.offset(0 as ::core::ffi::c_int as isize) = 'a' as i32 as gchar;
    memcpy(
        new.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        element as *const ::core::ffi::c_void,
        size as size_t,
    );
    return new as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_new_maybe(
    mut element: *const GVariantType,
) -> *mut GVariantType {
    let mut size: gsize = 0;
    let mut new: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(element) != 0 {
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
            b"g_variant_type_check (element)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    size = safe_c2rust_g_variant_type_get_string_length(element);
    new = g_malloc(size.wrapping_add(1 as gsize)) as *mut gchar;
    *new.offset(0 as ::core::ffi::c_int as isize) = 'm' as i32 as gchar;
    memcpy(
        new.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        element as *const ::core::ffi::c_void,
        size as size_t,
    );
    return new as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_new_dict_entry(
    mut key: *const GVariantType,
    mut value: *const GVariantType,
) -> *mut GVariantType {
    let mut keysize: gsize = 0;
    let mut valsize: gsize = 0;
    let mut new: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(key) != 0 {
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
            b"g_variant_type_check (key)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_check(value) != 0 {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_check (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantType>();
    }
    keysize = safe_c2rust_g_variant_type_get_string_length(key);
    valsize = safe_c2rust_g_variant_type_get_string_length(value);
    new = g_malloc(
        (1 as gsize)
            .wrapping_add(keysize)
            .wrapping_add(valsize)
            .wrapping_add(1 as gsize),
    ) as *mut gchar;
    *new.offset(0 as ::core::ffi::c_int as isize) = '{' as i32 as gchar;
    memcpy(
        new.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        key as *const ::core::ffi::c_void,
        keysize as size_t,
    );
    memcpy(
        new.offset(1 as ::core::ffi::c_int as isize)
            .offset(keysize as isize) as *mut ::core::ffi::c_void,
        value as *const ::core::ffi::c_void,
        valsize as size_t,
    );
    *new.offset((1 as gsize).wrapping_add(keysize).wrapping_add(valsize) as isize) =
        '}' as i32 as gchar;
    return new as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_type_checked_(
    mut type_string: *const gchar,
) -> *const GVariantType {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_type_string_is_valid(type_string) != 0 {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_string_is_valid (type_string)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    return type_string as *const GVariantType;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"variant_type_string_scan_internal\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
