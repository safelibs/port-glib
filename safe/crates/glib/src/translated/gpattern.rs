extern "C" {
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_strreverse(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPatternSpec {
    pub match_type: GMatchType,
    pub pattern_length: guint,
    pub min_length: guint,
    pub max_length: guint,
    pub pattern: *mut gchar,
}
pub type GMatchType = ::core::ffi::c_uint;
pub const G_MATCH_LAST: GMatchType = 5;
pub const G_MATCH_EXACT: GMatchType = 4;
pub const G_MATCH_TAIL: GMatchType = 3;
pub const G_MATCH_HEAD: GMatchType = 2;
pub const G_MATCH_ALL_TAIL: GMatchType = 1;
pub const G_MATCH_ALL: GMatchType = 0;
pub type GPatternSpec = _GPatternSpec;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_pattern_ph_match(
    mut match_pattern: *const gchar,
    mut match_string: *const gchar,
    mut wildcard_reached_p: *mut gboolean,
) -> gboolean {
    let mut pattern: *const gchar = ::core::ptr::null::<gchar>();
    let mut string: *const gchar = ::core::ptr::null::<gchar>();
    let mut ch: gchar = 0;
    pattern = match_pattern;
    string = match_string;
    ch = *pattern;
    pattern = pattern.offset(1);
    while ch != 0 {
        match ch as ::core::ffi::c_int {
            63 => {
                if *string == 0 {
                    return FALSE;
                }
                string = string.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(string as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
            42 => {
                *wildcard_reached_p = TRUE as gboolean;
                loop {
                    ch = *pattern;
                    pattern = pattern.offset(1);
                    if ch as ::core::ffi::c_int == '?' as i32 {
                        if *string == 0 {
                            return FALSE;
                        }
                        string = string.offset(
                            *safe_c2rust_g_utf8_skip.offset(*(string as *const guchar) as isize)
                                as ::core::ffi::c_int as isize,
                        ) as *mut ::core::ffi::c_char;
                    }
                    if !(ch as ::core::ffi::c_int == '*' as i32
                        || ch as ::core::ffi::c_int == '?' as i32)
                    {
                        break;
                    }
                }
                if ch == 0 {
                    return TRUE;
                }
                loop {
                    let mut next_wildcard_reached: gboolean = FALSE;
                    while ch as ::core::ffi::c_int != *string as ::core::ffi::c_int {
                        if *string == 0 {
                            return FALSE;
                        }
                        string = string.offset(
                            *safe_c2rust_g_utf8_skip.offset(*(string as *const guchar) as isize)
                                as ::core::ffi::c_int as isize,
                        ) as *mut ::core::ffi::c_char;
                    }
                    string = string.offset(1);
                    if safe_c2rust_g_pattern_ph_match(
                        pattern,
                        string,
                        &raw mut next_wildcard_reached,
                    ) != 0
                    {
                        return TRUE;
                    }
                    if next_wildcard_reached != 0 {
                        return FALSE;
                    }
                    if !(*string != 0) {
                        break;
                    }
                }
            }
            _ => {
                if ch as ::core::ffi::c_int == *string as ::core::ffi::c_int {
                    string = string.offset(1);
                } else {
                    return FALSE;
                }
            }
        }
        ch = *pattern;
        pattern = pattern.offset(1);
    }
    return (*string as ::core::ffi::c_int == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_match(
    mut pspec: *mut GPatternSpec,
    mut string_length: gsize,
    mut string: *const gchar,
    mut string_reversed: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !pspec.is_null() {
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
            b"pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if string_length < (*pspec).min_length as gsize || string_length > (*pspec).max_length as gsize
    {
        return FALSE;
    }
    let mut dummy: gboolean = 0;
    match (*pspec).match_type as ::core::ffi::c_uint {
        0 => {
            return safe_c2rust_g_pattern_ph_match((*pspec).pattern, string, &raw mut dummy);
        }
        1 => {
            if !string_reversed.is_null() {
                return safe_c2rust_g_pattern_ph_match(
                    (*pspec).pattern,
                    string_reversed,
                    &raw mut dummy,
                );
            } else {
                let mut result: gboolean = 0;
                let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
                tmp = g_utf8_strreverse(string, string_length as gssize);
                result = safe_c2rust_g_pattern_ph_match((*pspec).pattern, tmp, &raw mut dummy);
                g_free(tmp as gpointer);
                return result;
            }
        }
        2 => {
            if (*pspec).pattern_length as gsize == string_length {
                return (strcmp((*pspec).pattern, string as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            } else if (*pspec).pattern_length != 0 {
                return (strncmp(
                    (*pspec).pattern,
                    string as *const ::core::ffi::c_char,
                    (*pspec).pattern_length as size_t,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            } else {
                return TRUE;
            }
        }
        3 => {
            if (*pspec).pattern_length != 0 {
                return (strcmp(
                    (*pspec).pattern,
                    string.offset(
                        string_length.wrapping_sub((*pspec).pattern_length as gsize) as isize
                    ),
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            } else {
                return TRUE;
            }
        }
        4 => {
            if (*pspec).pattern_length as gsize != string_length {
                return FALSE;
            } else {
                return (strcmp((*pspec).pattern, string as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            }
        }
        _ => {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if ((*pspec).match_type as ::core::ffi::c_uint)
                    < G_MATCH_LAST as ::core::ffi::c_int as ::core::ffi::c_uint
                {
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
                    b"pspec->match_type < G_MATCH_LAST\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return 0 as gboolean;
            }
            return FALSE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_match(
    mut pspec: *mut GPatternSpec,
    mut string_length: guint,
    mut string: *const gchar,
    mut string_reversed: *const gchar,
) -> gboolean {
    return safe_c2rust_g_pattern_spec_match(
        pspec,
        string_length as gsize,
        string,
        string_reversed,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_new(
    mut pattern: *const gchar,
) -> *mut GPatternSpec {
    let mut pspec: *mut GPatternSpec = ::core::ptr::null_mut::<GPatternSpec>();
    let mut seen_joker: gboolean = FALSE;
    let mut seen_wildcard: gboolean = FALSE;
    let mut more_wildcards: gboolean = FALSE;
    let mut hw_pos: gint = -(1 as gint);
    let mut tw_pos: gint = -(1 as gint);
    let mut hj_pos: gint = -(1 as gint);
    let mut tj_pos: gint = -(1 as gint);
    let mut follows_wildcard: gboolean = FALSE;
    let mut pending_jokers: guint = 0 as guint;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut d: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !pattern.is_null() {
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
            b"pattern != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPatternSpec>();
    }
    pspec = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GPatternSpec>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GPatternSpec;
    (*pspec).pattern_length = strlen(pattern as *const ::core::ffi::c_char) as guint;
    (*pspec).min_length = 0 as guint;
    (*pspec).max_length = 0 as guint;
    (*pspec).pattern = ({
        let mut __n: gsize = (*pspec).pattern_length.wrapping_add(1 as guint) as gsize;
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
    d = (*pspec).pattern;
    let mut current_block_32: u64;
    i = 0 as guint;
    s = pattern;
    while *s as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        match *s as ::core::ffi::c_int {
            42 => {
                if follows_wildcard != 0 {
                    (*pspec).pattern_length = (*pspec).pattern_length.wrapping_sub(1);
                    current_block_32 = 11050875288958768710;
                } else {
                    follows_wildcard = TRUE as gboolean;
                    if hw_pos < 0 as ::core::ffi::c_int {
                        hw_pos = i as gint;
                    }
                    tw_pos = i as gint;
                    current_block_32 = 11636175345244025579;
                }
            }
            63 => {
                pending_jokers = pending_jokers.wrapping_add(1);
                (*pspec).min_length = (*pspec).min_length.wrapping_add(1);
                (*pspec).max_length = (*pspec).max_length.wrapping_add(4 as guint);
                current_block_32 = 11050875288958768710;
            }
            _ => {
                while pending_jokers != 0 {
                    let fresh0 = d;
                    d = d.offset(1);
                    *fresh0 = '?' as i32 as gchar;
                    if hj_pos < 0 as ::core::ffi::c_int {
                        hj_pos = i as gint;
                    }
                    tj_pos = i as gint;
                    pending_jokers = pending_jokers.wrapping_sub(1);
                    i = i.wrapping_add(1);
                }
                follows_wildcard = FALSE as gboolean;
                (*pspec).min_length = (*pspec).min_length.wrapping_add(1);
                (*pspec).max_length = (*pspec).max_length.wrapping_add(1);
                current_block_32 = 11636175345244025579;
            }
        }
        match current_block_32 {
            11636175345244025579 => {
                let fresh1 = d;
                d = d.offset(1);
                *fresh1 = *s;
                i = i.wrapping_add(1);
            }
            _ => {}
        }
        s = s.offset(1);
    }
    while pending_jokers != 0 {
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = '?' as i32 as gchar;
        if hj_pos < 0 as ::core::ffi::c_int {
            hj_pos = i as gint;
        }
        tj_pos = i as gint;
        pending_jokers = pending_jokers.wrapping_sub(1);
    }
    let fresh3 = d;
    d = d.offset(1);
    *fresh3 = 0 as gchar;
    seen_joker = (hj_pos >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    seen_wildcard = (hw_pos >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    more_wildcards = (seen_wildcard != 0 && hw_pos != tw_pos) as ::core::ffi::c_int as gboolean;
    if seen_wildcard != 0 {
        (*pspec).max_length = G_MAXUINT as guint;
    }
    if seen_joker == 0 && more_wildcards == 0 {
        if *(*pspec).pattern.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '*' as i32
        {
            (*pspec).match_type = G_MATCH_TAIL;
            (*pspec).pattern_length = (*pspec).pattern_length.wrapping_sub(1);
            memmove(
                (*pspec).pattern as *mut ::core::ffi::c_void,
                (*pspec).pattern.offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (*pspec).pattern_length as size_t,
            );
            *(*pspec).pattern.offset((*pspec).pattern_length as isize) = 0 as gchar;
            return pspec;
        }
        if (*pspec).pattern_length > 0 as guint
            && *(*pspec)
                .pattern
                .offset((*pspec).pattern_length.wrapping_sub(1 as guint) as isize)
                as ::core::ffi::c_int
                == '*' as i32
        {
            (*pspec).match_type = G_MATCH_HEAD;
            (*pspec).pattern_length = (*pspec).pattern_length.wrapping_sub(1);
            *(*pspec).pattern.offset((*pspec).pattern_length as isize) = 0 as gchar;
            return pspec;
        }
        if seen_wildcard == 0 {
            (*pspec).match_type = G_MATCH_EXACT;
            return pspec;
        }
    }
    tw_pos = (*pspec)
        .pattern_length
        .wrapping_sub(1 as guint)
        .wrapping_sub(tw_pos as guint) as gint;
    tj_pos = (*pspec)
        .pattern_length
        .wrapping_sub(1 as guint)
        .wrapping_sub(tj_pos as guint) as gint;
    if seen_wildcard != 0 {
        (*pspec).match_type = (if tw_pos > hw_pos {
            G_MATCH_ALL_TAIL as ::core::ffi::c_int
        } else {
            G_MATCH_ALL as ::core::ffi::c_int
        }) as GMatchType;
    } else {
        (*pspec).match_type = (if tj_pos > hj_pos {
            G_MATCH_ALL_TAIL as ::core::ffi::c_int
        } else {
            G_MATCH_ALL as ::core::ffi::c_int
        }) as GMatchType;
    }
    if (*pspec).match_type as ::core::ffi::c_uint
        == G_MATCH_ALL_TAIL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut tmp: *mut gchar = (*pspec).pattern;
        (*pspec).pattern = g_utf8_strreverse((*pspec).pattern, (*pspec).pattern_length as gssize);
        g_free(tmp as gpointer);
    }
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_copy(
    mut pspec: *mut GPatternSpec,
) -> *mut GPatternSpec {
    let mut pspec_copy: *mut GPatternSpec = ::core::ptr::null_mut::<GPatternSpec>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !pspec.is_null() {
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
            b"pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GPatternSpec>();
    }
    pspec_copy = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GPatternSpec>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GPatternSpec;
    *pspec_copy = *pspec;
    (*pspec_copy).pattern = g_strndup((*pspec).pattern, (*pspec).pattern_length as gsize);
    return pspec_copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_free(mut pspec: *mut GPatternSpec) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !pspec.is_null() {
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
            b"pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*pspec).pattern as gpointer);
    g_free(pspec as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_equal(
    mut pspec1: *mut GPatternSpec,
    mut pspec2: *mut GPatternSpec,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !pspec1.is_null() {
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
            b"pspec1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !pspec2.is_null() {
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
            b"pspec2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*pspec1).pattern_length == (*pspec2).pattern_length
        && (*pspec1).match_type as ::core::ffi::c_uint
            == (*pspec2).match_type as ::core::ffi::c_uint
        && strcmp((*pspec1).pattern, (*pspec2).pattern) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_spec_match_string(
    mut pspec: *mut GPatternSpec,
    mut string: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !pspec.is_null() {
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
            b"pspec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
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
        return 0 as gboolean;
    }
    return safe_c2rust_g_pattern_spec_match(
        pspec,
        strlen(string as *const ::core::ffi::c_char) as gsize,
        string,
        ::core::ptr::null::<gchar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_match_string(
    mut pspec: *mut GPatternSpec,
    mut string: *const gchar,
) -> gboolean {
    return safe_c2rust_g_pattern_spec_match_string(pspec, string);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pattern_match_simple(
    mut pattern: *const gchar,
    mut string: *const gchar,
) -> gboolean {
    let mut pspec: *mut GPatternSpec = ::core::ptr::null_mut::<GPatternSpec>();
    let mut ergo: gboolean = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !pattern.is_null() {
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
            b"pattern != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    pspec = safe_c2rust_g_pattern_spec_new(pattern);
    ergo = safe_c2rust_g_pattern_spec_match(
        pspec,
        strlen(string as *const ::core::ffi::c_char) as gsize,
        string,
        ::core::ptr::null::<gchar>(),
    );
    safe_c2rust_g_pattern_spec_free(pspec);
    return ergo;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_pattern_spec_new\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
