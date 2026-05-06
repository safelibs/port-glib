extern "C" {
    pub type _GBytes;
    pub type _GHashTable;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_hostname_is_non_ascii(hostname: *const gchar) -> gboolean;
    fn g_hostname_is_ip_address(hostname: *const gchar) -> gboolean;
    fn g_hostname_to_ascii(hostname: *const gchar) -> *mut gchar;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_toupper(c: gchar) -> gchar;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_ascii_strdown(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
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
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_string_append_uri_escaped(
        string: *mut GString,
        unescaped: *const gchar,
        reserved_chars_allowed: *const gchar,
        allow_utf8: gboolean,
    ) -> *mut GString;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_atomic_rc_box_alloc0(block_size: gsize) -> gpointer;
    fn g_atomic_rc_box_acquire(mem_block: gpointer) -> gpointer;
    fn g_atomic_rc_box_release_full(mem_block: gpointer, clear_func: GDestroyNotify);
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
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GBytes = _GBytes;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GHashTable = _GHashTable;
pub type gunichar = guint32;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUri {
    pub scheme: *mut gchar,
    pub userinfo: *mut gchar,
    pub host: *mut gchar,
    pub port: gint,
    pub path: *mut gchar,
    pub query: *mut gchar,
    pub fragment: *mut gchar,
    pub user: *mut gchar,
    pub password: *mut gchar,
    pub auth_params: *mut gchar,
    pub flags: GUriFlags,
}
pub type GUriFlags = ::core::ffi::c_uint;
pub const G_URI_FLAGS_SCHEME_NORMALIZE: GUriFlags = 256;
pub const G_URI_FLAGS_ENCODED_FRAGMENT: GUriFlags = 128;
pub const G_URI_FLAGS_ENCODED_PATH: GUriFlags = 64;
pub const G_URI_FLAGS_ENCODED_QUERY: GUriFlags = 32;
pub const G_URI_FLAGS_NON_DNS: GUriFlags = 16;
pub const G_URI_FLAGS_ENCODED: GUriFlags = 8;
pub const G_URI_FLAGS_HAS_AUTH_PARAMS: GUriFlags = 4;
pub const G_URI_FLAGS_HAS_PASSWORD: GUriFlags = 2;
pub const G_URI_FLAGS_PARSE_RELAXED: GUriFlags = 1;
pub const G_URI_FLAGS_NONE: GUriFlags = 0;
pub type GUri = _GUri;
pub type GUriError = ::core::ffi::c_uint;
pub const G_URI_ERROR_BAD_FRAGMENT: GUriError = 9;
pub const G_URI_ERROR_BAD_QUERY: GUriError = 8;
pub const G_URI_ERROR_BAD_PATH: GUriError = 7;
pub const G_URI_ERROR_BAD_PORT: GUriError = 6;
pub const G_URI_ERROR_BAD_HOST: GUriError = 5;
pub const G_URI_ERROR_BAD_AUTH_PARAMS: GUriError = 4;
pub const G_URI_ERROR_BAD_PASSWORD: GUriError = 3;
pub const G_URI_ERROR_BAD_USER: GUriError = 2;
pub const G_URI_ERROR_BAD_SCHEME: GUriError = 1;
pub const G_URI_ERROR_FAILED: GUriError = 0;
pub type GUriHideFlags = ::core::ffi::c_uint;
pub const G_URI_HIDE_FRAGMENT: GUriHideFlags = 16;
pub const G_URI_HIDE_QUERY: GUriHideFlags = 8;
pub const G_URI_HIDE_AUTH_PARAMS: GUriHideFlags = 4;
pub const G_URI_HIDE_PASSWORD: GUriHideFlags = 2;
pub const G_URI_HIDE_USERINFO: GUriHideFlags = 1;
pub const G_URI_HIDE_NONE: GUriHideFlags = 0;
pub type GUriParamsFlags = ::core::ffi::c_uint;
pub const G_URI_PARAMS_PARSE_RELAXED: GUriParamsFlags = 4;
pub const G_URI_PARAMS_WWW_FORM: GUriParamsFlags = 2;
pub const G_URI_PARAMS_CASE_INSENSITIVE: GUriParamsFlags = 1;
pub const G_URI_PARAMS_NONE: GUriParamsFlags = 0;
pub type GUriParamsIter = _GUriParamsIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUriParamsIter {
    pub dummy0: gint,
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: [guint8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RealIter {
    pub flags: GUriParamsFlags,
    pub attr: *const gchar,
    pub end: *const gchar,
    pub sep_table: [guint8; 256],
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_uri_ref\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
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
pub const G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"!$&'()*+,;=\0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_ref(mut uri: *mut GUri) -> *mut GUri {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    return g_atomic_rc_box_acquire(uri as gpointer) as *mut GUri;
}
unsafe extern "C" fn safe_c2rust_g_uri_clear(mut uri: *mut GUri) {
    g_free((*uri).scheme as gpointer);
    g_free((*uri).userinfo as gpointer);
    g_free((*uri).host as gpointer);
    g_free((*uri).path as gpointer);
    g_free((*uri).query as gpointer);
    g_free((*uri).fragment as gpointer);
    g_free((*uri).user as gpointer);
    g_free((*uri).password as gpointer);
    g_free((*uri).auth_params as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_unref(mut uri: *mut GUri) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_atomic_rc_box_release_full(
        uri as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GUri) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_g_uri_clear as unsafe extern "C" fn(*mut GUri) -> ()),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_uri_char_is_unreserved(mut ch: gchar) -> gboolean {
    if *safe_c2rust_g_ascii_table.offset(ch as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALNUM as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        return TRUE;
    }
    return (ch as ::core::ffi::c_int == '-' as i32
        || ch as ::core::ffi::c_int == '.' as i32
        || ch as ::core::ffi::c_int == '_' as i32
        || ch as ::core::ffi::c_int == '~' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_uri_decoder(
    mut out: *mut *mut gchar,
    mut illegal_chars: *const gchar,
    mut start: *const gchar,
    mut length: gsize,
    mut just_normalize: gboolean,
    mut www_form: gboolean,
    mut flags: GUriFlags,
    mut parse_error: GUriError,
    mut error: *mut *mut GError,
) -> gssize {
    let mut c: gchar = 0;
    let mut decoded: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut invalid: *const gchar = ::core::ptr::null::<gchar>();
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    let mut len: gssize = 0;
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_ENCODED as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        just_normalize = FALSE as gboolean;
    }
    decoded = g_string_sized_new(length.wrapping_add(1 as gsize));
    s = start;
    end = s.offset(length as isize);
    while s < end {
        if *s as ::core::ffi::c_int == '%' as i32 {
            if s.offset(2 as ::core::ffi::c_int as isize) >= end
                || !(*safe_c2rust_g_ascii_table
                    .offset(*s.offset(1 as ::core::ffi::c_int as isize) as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_XDIGIT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
                || !(*safe_c2rust_g_ascii_table
                    .offset(*s.offset(2 as ::core::ffi::c_int as isize) as guchar as isize)
                    as ::core::ffi::c_int
                    & G_ASCII_XDIGIT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
            {
                if flags as ::core::ffi::c_uint
                    & G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
                    == 0
                {
                    g_set_error_literal(
                        error,
                        safe_c2rust_g_uri_error_quark(),
                        parse_error as gint,
                        glib_gettext(b"Invalid %-encoding in URI\0" as *const u8 as *const gchar),
                    );
                    if 0 != 0 {
                        if 0 as ::core::ffi::c_int == 0 {
                            g_string_free(
                                decoded,
                                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                            );
                        } else {
                            g_string_free_and_steal(decoded);
                        };
                    } else {
                        g_string_free(
                            decoded,
                            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                        );
                    };
                    return -(1 as ::core::ffi::c_int) as gssize;
                }
                safe_c2rust_g_string_append_c_inline(decoded, *s);
            } else {
                c = (((if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= '9' as i32
                {
                    *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int - '0' as i32
                } else {
                    (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        & 0x4f as ::core::ffi::c_int)
                        - 'A' as i32
                        + 10 as ::core::ffi::c_int
                }) << 4 as ::core::ffi::c_int)
                    + (if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32
                    {
                        *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            - '0' as i32
                    } else {
                        (*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            & 0x4f as ::core::ffi::c_int)
                            - 'A' as i32
                            + 10 as ::core::ffi::c_int
                    })) as gchar;
                if !illegal_chars.is_null()
                    && !strchr(
                        illegal_chars as *const ::core::ffi::c_char,
                        c as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    g_set_error_literal(
                        error,
                        safe_c2rust_g_uri_error_quark(),
                        parse_error as gint,
                        glib_gettext(b"Illegal character in URI\0" as *const u8 as *const gchar),
                    );
                    if 0 != 0 {
                        if 0 as ::core::ffi::c_int == 0 {
                            g_string_free(
                                decoded,
                                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                            );
                        } else {
                            g_string_free_and_steal(decoded);
                        };
                    } else {
                        g_string_free(
                            decoded,
                            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                        );
                    };
                    return -(1 as ::core::ffi::c_int) as gssize;
                }
                if just_normalize != 0 && safe_c2rust_g_uri_char_is_unreserved(c) == 0 {
                    safe_c2rust_g_string_append_c_inline(decoded, *s);
                    safe_c2rust_g_string_append_c_inline(
                        decoded,
                        g_ascii_toupper(*s.offset(1 as ::core::ffi::c_int as isize)),
                    );
                    safe_c2rust_g_string_append_c_inline(
                        decoded,
                        g_ascii_toupper(*s.offset(2 as ::core::ffi::c_int as isize)),
                    );
                    s = s.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    safe_c2rust_g_string_append_c_inline(decoded, c);
                    s = s.offset(2 as ::core::ffi::c_int as isize);
                }
            }
        } else if www_form != 0 && *s as ::core::ffi::c_int == '+' as i32 {
            safe_c2rust_g_string_append_c_inline(decoded, ' ' as i32 as gchar);
        } else if just_normalize != 0
            && !(*safe_c2rust_g_ascii_table.offset(*s as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_GRAPH as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            g_string_append_printf(
                decoded,
                b"%%%02X\0" as *const u8 as *const gchar,
                *s as guchar as ::core::ffi::c_int,
            );
        } else {
            safe_c2rust_g_string_append_c_inline(decoded, *s);
        }
        s = s.offset(1);
    }
    len = (*decoded).len as gssize;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if len >= 0 as gssize {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/guri.c\0" as *const u8 as *const ::core::ffi::c_char,
            354 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_ENCODED as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
        && g_utf8_validate((*decoded).str_0, len, &raw mut invalid) == 0
    {
        g_set_error_literal(
            error,
            safe_c2rust_g_uri_error_quark(),
            parse_error as gint,
            glib_gettext(b"Non-UTF-8 characters in URI\0" as *const u8 as *const gchar),
        );
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    decoded,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(decoded);
            };
        } else {
            g_string_free(
                decoded,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if !out.is_null() {
        *out = if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(decoded, 0 as gboolean)
            } else {
                g_string_free_and_steal(decoded)
            }
        } else {
            g_string_free(decoded, 0 as gboolean)
        };
    } else {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    decoded,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(decoded);
            };
        } else {
            g_string_free(
                decoded,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
    }
    return len;
}
unsafe extern "C" fn safe_c2rust_uri_decode(
    mut out: *mut *mut gchar,
    mut illegal_chars: *const gchar,
    mut start: *const gchar,
    mut length: gsize,
    mut www_form: gboolean,
    mut flags: GUriFlags,
    mut parse_error: GUriError,
    mut error: *mut *mut GError,
) -> gboolean {
    return (safe_c2rust_uri_decoder(
        out,
        illegal_chars,
        start,
        length,
        FALSE,
        www_form,
        flags,
        parse_error,
        error,
    ) != -(1 as ::core::ffi::c_int) as gssize) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_uri_normalize(
    mut out: *mut *mut gchar,
    mut start: *const gchar,
    mut length: gsize,
    mut flags: GUriFlags,
    mut parse_error: GUriError,
    mut error: *mut *mut GError,
) -> gboolean {
    return (safe_c2rust_uri_decoder(
        out,
        ::core::ptr::null::<gchar>(),
        start,
        length,
        TRUE,
        FALSE,
        flags,
        parse_error,
        error,
    ) != -(1 as ::core::ffi::c_int) as gssize) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_is_valid(
    mut c: guchar,
    mut reserved_chars_allowed: *const gchar,
) -> gboolean {
    if safe_c2rust_g_uri_char_is_unreserved(c as gchar) != 0 {
        return TRUE;
    }
    if !reserved_chars_allowed.is_null()
        && !strchr(
            reserved_chars_allowed as *const ::core::ffi::c_char,
            c as ::core::ffi::c_int,
        )
        .is_null()
    {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__uri_encoder(
    mut out: *mut GString,
    mut start: *const guchar,
    mut length: gsize,
    mut reserved_chars_allowed: *const gchar,
    mut allow_utf8: gboolean,
) {
    static mut safe_c2rust_hex: [gchar; 17] =
        unsafe { ::core::mem::transmute::<[u8; 17], [gchar; 17]>(*b"0123456789ABCDEF\0") };
    let mut p: *const guchar = start;
    let mut end: *const guchar = p.offset(length as isize);
    while p < end {
        let mut multibyte_utf8_char: gunichar = 0 as gunichar;
        if allow_utf8 != 0 && *p as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
            multibyte_utf8_char =
                g_utf8_get_char_validated(p as *mut gchar, end.offset_from(p) as gssize);
        }
        if multibyte_utf8_char > 0 as gunichar
            && multibyte_utf8_char != -(1 as ::core::ffi::c_int) as gunichar
            && multibyte_utf8_char != -(2 as ::core::ffi::c_int) as gunichar
        {
            let mut len: gint = *safe_c2rust_g_utf8_skip.offset(*p as isize) as gint;
            safe_c2rust_g_string_append_len_inline(out, p as *mut gchar, len as gssize);
            p = p.offset(len as isize);
        } else if safe_c2rust_is_valid(*p, reserved_chars_allowed) != 0 {
            safe_c2rust_g_string_append_c_inline(out, *p as gchar);
            p = p.offset(1);
        } else {
            safe_c2rust_g_string_append_c_inline(out, '%' as i32 as gchar);
            safe_c2rust_g_string_append_c_inline(
                out,
                safe_c2rust_hex[(*p as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize],
            );
            safe_c2rust_g_string_append_c_inline(
                out,
                safe_c2rust_hex[(*p as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize],
            );
            p = p.offset(1);
        }
    }
}
unsafe extern "C" fn safe_c2rust_parse_ip_literal(
    mut start: *const gchar,
    mut length: gsize,
    mut flags: GUriFlags,
    mut out: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut pct: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut zone_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut addr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut addr_length: gsize = 0 as gsize;
    let mut zone_id_length: gsize = 0 as gsize;
    let mut decoded_zone_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !(*start.offset(length.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
        != ']' as i32)
    {
        addr = g_strndup(
            start.offset(1 as ::core::ffi::c_int as isize),
            length.wrapping_sub(2 as gsize),
        );
        addr_length = length.wrapping_sub(2 as gsize);
        pct = strchr(addr, '%' as i32) as *mut gchar;
        if !pct.is_null() {
            *pct = '\0' as i32 as gchar;
            if addr_length.wrapping_sub(pct.offset_from(addr) as ::core::ffi::c_long as gsize)
                >= 4 as gsize
                && *pct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '2' as i32
                && *pct.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '5' as i32
            {
                zone_id = pct.offset(3 as ::core::ffi::c_int as isize);
                zone_id_length = addr_length
                    .wrapping_sub(zone_id.offset_from(addr) as ::core::ffi::c_long as gsize);
                current_block = 7651349459974463963;
            } else if flags as ::core::ffi::c_uint
                & G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && addr_length.wrapping_sub(pct.offset_from(addr) as ::core::ffi::c_long as gsize)
                    >= 2 as gsize
            {
                zone_id = pct.offset(1 as ::core::ffi::c_int as isize);
                zone_id_length = addr_length
                    .wrapping_sub(zone_id.offset_from(addr) as ::core::ffi::c_long as gsize);
                current_block = 7651349459974463963;
            } else {
                current_block = 7689290862149178220;
            }
            match current_block {
                7689290862149178220 => {}
                _ => {
                    if ({
                        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                        if zone_id_length >= 1 as gsize {
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
                            b"../original/glib/guri.c\0" as *const u8 as *const ::core::ffi::c_char,
                            512 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"zone_id_length >= 1\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    current_block = 15976848397966268834;
                }
            }
        } else {
            current_block = 15976848397966268834;
        }
        match current_block {
            7689290862149178220 => {}
            _ => {
                if !(g_hostname_is_ip_address(addr) == 0 || strchr(addr, ':' as i32).is_null()) {
                    if !(!zone_id.is_null()
                        && safe_c2rust_uri_decode(
                            &raw mut decoded_zone_id,
                            ::core::ptr::null::<gchar>(),
                            zone_id,
                            zone_id_length,
                            FALSE,
                            flags,
                            G_URI_ERROR_BAD_HOST,
                            ::core::ptr::null_mut::<*mut GError>(),
                        ) == 0)
                    {
                        if !out.is_null() && !decoded_zone_id.is_null() {
                            *out = g_strconcat(
                                addr,
                                b"%\0" as *const u8 as *const ::core::ffi::c_char,
                                decoded_zone_id,
                                NULL_0,
                            );
                        } else if !out.is_null() {
                            *out = safe_c2rust_g_steal_pointer(&raw mut addr as gpointer)
                                as *mut gchar as *mut gchar;
                        }
                        g_free(addr as gpointer);
                        g_free(decoded_zone_id as gpointer);
                        return TRUE;
                    }
                }
            }
        }
    }
    g_free(addr as gpointer);
    g_free(decoded_zone_id as gpointer);
    g_set_error(
        error,
        safe_c2rust_g_uri_error_quark(),
        G_URI_ERROR_BAD_HOST as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Invalid IPv6 address \xE2\x80\x98%.*s\xE2\x80\x99 in URI\0" as *const u8
                as *const gchar,
        ),
        length as gint,
        start,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_parse_host(
    mut start: *const gchar,
    mut length: gsize,
    mut flags: GUriFlags,
    mut out: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut decoded: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut host: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut addr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if *start as ::core::ffi::c_int == '[' as i32 {
        if safe_c2rust_parse_ip_literal(start, length, flags, &raw mut host, error) == 0 {
            return FALSE;
        }
    } else {
        if *safe_c2rust_g_ascii_table.offset(*start as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_DIGIT as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            addr = g_strndup(start, length);
            if g_hostname_is_ip_address(addr) != 0 {
                host = addr;
                current_block = 4457489249055356782;
            } else {
                g_free(addr as gpointer);
                current_block = 10886091980245723256;
            }
        } else {
            current_block = 10886091980245723256;
        }
        match current_block {
            4457489249055356782 => {}
            _ => {
                if flags as ::core::ffi::c_uint
                    & G_URI_FLAGS_NON_DNS as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    if safe_c2rust_uri_normalize(
                        &raw mut decoded,
                        start,
                        length,
                        flags,
                        G_URI_ERROR_BAD_HOST,
                        error,
                    ) == 0
                    {
                        return FALSE;
                    }
                    host = safe_c2rust_g_steal_pointer(&raw mut decoded as gpointer) as *mut gchar
                        as *mut gchar;
                } else {
                    flags = ::core::mem::transmute::<::core::ffi::c_uint, GUriFlags>(
                        flags as ::core::ffi::c_uint
                            & !(G_URI_FLAGS_ENCODED as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    );
                    if safe_c2rust_uri_decode(
                        &raw mut decoded,
                        ::core::ptr::null::<gchar>(),
                        start,
                        length,
                        FALSE,
                        flags,
                        G_URI_ERROR_BAD_HOST,
                        error,
                    ) == 0
                    {
                        return FALSE;
                    }
                    if g_hostname_is_ip_address(decoded) != 0 {
                        g_free(decoded as gpointer);
                        g_set_error(
                            error,
                            safe_c2rust_g_uri_error_quark(),
                            G_URI_ERROR_BAD_HOST as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Illegal encoded IP address \xE2\x80\x98%.*s\xE2\x80\x99 in URI\0"
                                    as *const u8 as *const gchar,
                            ),
                            length as gint,
                            start,
                        );
                        return FALSE;
                    }
                    if g_hostname_is_non_ascii(decoded) != 0 {
                        host = g_hostname_to_ascii(decoded);
                        if host.is_null() {
                            g_free(decoded as gpointer);
                            g_set_error(
                                error,
                                safe_c2rust_g_uri_error_quark(),
                                G_URI_ERROR_BAD_HOST as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Illegal internationalized hostname \xE2\x80\x98%.*s\xE2\x80\x99 in URI\0"
                                        as *const u8 as *const gchar,
                                ),
                                length as gint,
                                start,
                            );
                            return FALSE;
                        }
                    } else {
                        host = safe_c2rust_g_steal_pointer(&raw mut decoded as gpointer)
                            as *mut gchar as *mut gchar;
                    }
                }
            }
        }
    }
    if !out.is_null() {
        *out = safe_c2rust_g_steal_pointer(&raw mut host as gpointer) as *mut gchar as *mut gchar;
    }
    g_free(host as gpointer);
    g_free(decoded as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_port(
    mut start: *const gchar,
    mut length: gsize,
    mut out: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut parsed_port: gulong = 0;
    if !(*safe_c2rust_g_ascii_table.offset(*start as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_DIGIT as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        g_set_error(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_BAD_PORT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Could not parse port \xE2\x80\x98%.*s\xE2\x80\x99 in URI\0" as *const u8
                    as *const gchar,
            ),
            length as gint,
            start,
        );
        return FALSE;
    }
    parsed_port = strtoul(
        start as *const ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    ) as gulong;
    if end != start.offset(length as isize) as *mut gchar {
        g_set_error(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_BAD_PORT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Could not parse port \xE2\x80\x98%.*s\xE2\x80\x99 in URI\0" as *const u8
                    as *const gchar,
            ),
            length as gint,
            start,
        );
        return FALSE;
    } else if parsed_port > 65535 as gulong {
        g_set_error(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_BAD_PORT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Port \xE2\x80\x98%.*s\xE2\x80\x99 in URI is out of range\0" as *const u8
                    as *const gchar,
            ),
            length as gint,
            start,
        );
        return FALSE;
    }
    if !out.is_null() {
        *out = parsed_port as gint;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_userinfo(
    mut start: *const gchar,
    mut length: gsize,
    mut flags: GUriFlags,
    mut user: *mut *mut gchar,
    mut password: *mut *mut gchar,
    mut auth_params: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut user_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut password_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut auth_params_end: *const gchar = ::core::ptr::null::<gchar>();
    auth_params_end = start.offset(length as isize);
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_HAS_AUTH_PARAMS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        password_end = memchr(
            start as *const ::core::ffi::c_void,
            ';' as i32,
            auth_params_end.offset_from(start) as ::core::ffi::c_long as size_t,
        ) as *const gchar;
    }
    if password_end.is_null() {
        password_end = auth_params_end;
    }
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_HAS_PASSWORD as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        user_end = memchr(
            start as *const ::core::ffi::c_void,
            ':' as i32,
            password_end.offset_from(start) as ::core::ffi::c_long as size_t,
        ) as *const gchar;
    }
    if user_end.is_null() {
        user_end = password_end;
    }
    if safe_c2rust_uri_normalize(
        user,
        start,
        user_end.offset_from(start) as ::core::ffi::c_long as gsize,
        flags,
        G_URI_ERROR_BAD_USER,
        error,
    ) == 0
    {
        return FALSE;
    }
    if *user_end as ::core::ffi::c_int == ':' as i32 {
        start = user_end.offset(1 as ::core::ffi::c_int as isize);
        if safe_c2rust_uri_normalize(
            password,
            start,
            password_end.offset_from(start) as ::core::ffi::c_long as gsize,
            flags,
            G_URI_ERROR_BAD_PASSWORD,
            error,
        ) == 0
        {
            if !user.is_null() {
                let mut _pp: *mut *mut gchar = user as *mut *mut gchar;
                let mut _ptr: *mut gchar = *_pp;
                *_pp = ::core::ptr::null_mut::<gchar>();
                if !_ptr.is_null() {
                    g_free(_ptr as gpointer);
                }
            }
            return FALSE;
        }
    } else if !password.is_null() {
        *password = ::core::ptr::null_mut::<gchar>();
    }
    if *password_end as ::core::ffi::c_int == ';' as i32 {
        start = password_end.offset(1 as ::core::ffi::c_int as isize);
        if safe_c2rust_uri_normalize(
            auth_params,
            start,
            auth_params_end.offset_from(start) as ::core::ffi::c_long as gsize,
            flags,
            G_URI_ERROR_BAD_AUTH_PARAMS,
            error,
        ) == 0
        {
            if !user.is_null() {
                let mut _pp_0: *mut *mut gchar = user as *mut *mut gchar;
                let mut _ptr_0: *mut gchar = *_pp_0;
                *_pp_0 = ::core::ptr::null_mut::<gchar>();
                if !_ptr_0.is_null() {
                    g_free(_ptr_0 as gpointer);
                }
            }
            if !password.is_null() {
                let mut _pp_1: *mut *mut gchar = password as *mut *mut gchar;
                let mut _ptr_1: *mut gchar = *_pp_1;
                *_pp_1 = ::core::ptr::null_mut::<gchar>();
                if !_ptr_1.is_null() {
                    g_free(_ptr_1 as gpointer);
                }
            }
            return FALSE;
        }
    } else if !auth_params.is_null() {
        *auth_params = ::core::ptr::null_mut::<gchar>();
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_uri_cleanup(mut uri_string: *const gchar) -> *mut gchar {
    let mut copy: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    while *safe_c2rust_g_ascii_table.offset(*uri_string as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        uri_string = uri_string.offset(1);
    }
    end = uri_string.offset(strlen(uri_string as *const ::core::ffi::c_char) as isize);
    while end > uri_string
        && *safe_c2rust_g_ascii_table
            .offset(*end.offset(-(1 as ::core::ffi::c_int as isize)) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        end = end.offset(-1);
    }
    copy = g_string_sized_new(end.offset_from(uri_string) as ::core::ffi::c_long as gsize);
    while uri_string < end {
        if *uri_string as ::core::ffi::c_int == ' ' as i32 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"%20\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        copy,
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
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    copy,
                    b"%20\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else if !(*safe_c2rust_g_ascii_table.offset(*uri_string as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
        {
            safe_c2rust_g_string_append_c_inline(copy, *uri_string);
        }
        uri_string = uri_string.offset(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(copy, 0 as gboolean)
        } else {
            g_string_free_and_steal(copy)
        }
    } else {
        g_string_free(copy, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_should_normalize_empty_path(
    mut scheme: *const ::core::ffi::c_char,
) -> gboolean {
    let schemes: [*const ::core::ffi::c_char; 4] = [
        b"https\0" as *const u8 as *const ::core::ffi::c_char,
        b"http\0" as *const u8 as *const ::core::ffi::c_char,
        b"wss\0" as *const u8 as *const ::core::ffi::c_char,
        b"ws\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: gsize = 0;
    i = 0 as gsize;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if strcmp(schemes[i as usize], scheme) == 0 {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_normalize_port(
    mut scheme: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut default_schemes: [*const ::core::ffi::c_char; 3] = [
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0;
    match port {
        21 => {
            default_schemes[0 as ::core::ffi::c_int as usize] =
                b"ftp\0" as *const u8 as *const ::core::ffi::c_char;
        }
        80 => {
            default_schemes[0 as ::core::ffi::c_int as usize] =
                b"http\0" as *const u8 as *const ::core::ffi::c_char;
            default_schemes[1 as ::core::ffi::c_int as usize] =
                b"ws\0" as *const u8 as *const ::core::ffi::c_char;
        }
        443 => {
            default_schemes[0 as ::core::ffi::c_int as usize] =
                b"https\0" as *const u8 as *const ::core::ffi::c_char;
            default_schemes[1 as ::core::ffi::c_int as usize] =
                b"wss\0" as *const u8 as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    i = 0 as ::core::ffi::c_int;
    while !default_schemes[i as usize].is_null() {
        if strcmp(scheme, default_schemes[i as usize]) == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_default_scheme_port(
    mut scheme: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if strcmp(scheme, b"http\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(scheme, b"ws\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return 80 as ::core::ffi::c_int;
    }
    if strcmp(
        scheme,
        b"https\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(scheme, b"wss\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return 443 as ::core::ffi::c_int;
    }
    if strcmp(scheme, b"ftp\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 21 as ::core::ffi::c_int;
    }
    if strstr(
        scheme,
        b"socks\0" as *const u8 as *const ::core::ffi::c_char,
    ) == scheme as *mut ::core::ffi::c_char
    {
        return 1080 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn safe_c2rust_g_uri_split_internal(
    mut uri_string: *const gchar,
    mut flags: GUriFlags,
    mut scheme: *mut *mut gchar,
    mut userinfo: *mut *mut gchar,
    mut user: *mut *mut gchar,
    mut password: *mut *mut gchar,
    mut auth_params: *mut *mut gchar,
    mut host: *mut *mut gchar,
    mut port: *mut gint,
    mut path: *mut *mut gchar,
    mut query: *mut *mut gchar,
    mut fragment: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    let mut colon: *const gchar = ::core::ptr::null::<gchar>();
    let mut at: *const gchar = ::core::ptr::null::<gchar>();
    let mut path_start: *const gchar = ::core::ptr::null::<gchar>();
    let mut semi: *const gchar = ::core::ptr::null::<gchar>();
    let mut question: *const gchar = ::core::ptr::null::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut bracket: *const gchar = ::core::ptr::null::<gchar>();
    let mut hostend: *const gchar = ::core::ptr::null::<gchar>();
    let mut cleaned_uri_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut normalized_scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !scheme.is_null() {
        *scheme = ::core::ptr::null_mut::<gchar>();
    }
    if !userinfo.is_null() {
        *userinfo = ::core::ptr::null_mut::<gchar>();
    }
    if !user.is_null() {
        *user = ::core::ptr::null_mut::<gchar>();
    }
    if !password.is_null() {
        *password = ::core::ptr::null_mut::<gchar>();
    }
    if !auth_params.is_null() {
        *auth_params = ::core::ptr::null_mut::<gchar>();
    }
    if !host.is_null() {
        *host = ::core::ptr::null_mut::<gchar>();
    }
    if !port.is_null() {
        *port = -(1 as ::core::ffi::c_int) as gint;
    }
    if !path.is_null() {
        *path = ::core::ptr::null_mut::<gchar>();
    }
    if !query.is_null() {
        *query = ::core::ptr::null_mut::<gchar>();
    }
    if !fragment.is_null() {
        *fragment = ::core::ptr::null_mut::<gchar>();
    }
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !strpbrk(
            uri_string as *const ::core::ffi::c_char,
            b" \t\n\r\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        cleaned_uri_string = safe_c2rust_uri_cleanup(uri_string);
        uri_string = cleaned_uri_string;
    }
    p = uri_string;
    while *p as ::core::ffi::c_int != 0
        && (*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALPHA as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || p > uri_string
                && (*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                    & G_ASCII_DIGIT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    || *p as ::core::ffi::c_int == '.' as i32
                    || *p as ::core::ffi::c_int == '+' as i32
                    || *p as ::core::ffi::c_int == '-' as i32))
    {
        p = p.offset(1);
    }
    if p > uri_string && *p as ::core::ffi::c_int == ':' as i32 {
        normalized_scheme = g_ascii_strdown(uri_string, p.offset_from(uri_string) as gssize);
        if !scheme.is_null() {
            *scheme = safe_c2rust_g_steal_pointer(&raw mut normalized_scheme as gpointer)
                as *mut gchar as *mut gchar;
        }
        p = p.offset(1);
    } else {
        if !scheme.is_null() {
            *scheme = ::core::ptr::null_mut::<gchar>();
        }
        p = uri_string;
    }
    if strncmp(
        p as *const ::core::ffi::c_char,
        b"//\0" as *const u8 as *const ::core::ffi::c_char,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        p = p.offset(2 as ::core::ffi::c_int as isize);
        path_start = p.offset(strcspn(
            p as *const ::core::ffi::c_char,
            b"/?#\0" as *const u8 as *const ::core::ffi::c_char,
        ) as isize);
        at = memchr(
            p as *const ::core::ffi::c_void,
            '@' as i32,
            path_start.offset_from(p) as ::core::ffi::c_long as size_t,
        ) as *const gchar;
        if !at.is_null() {
            if flags as ::core::ffi::c_uint
                & G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                let mut next_at: *mut gchar = ::core::ptr::null_mut::<gchar>();
                loop {
                    next_at = memchr(
                        at.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        '@' as i32,
                        path_start.offset_from(at.offset(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_long as size_t,
                    ) as *mut gchar;
                    if !next_at.is_null() {
                        at = next_at;
                    }
                    if next_at.is_null() {
                        break;
                    }
                }
            }
            if !user.is_null()
                || !password.is_null()
                || !auth_params.is_null()
                || flags as ::core::ffi::c_uint
                    & (G_URI_FLAGS_HAS_PASSWORD as ::core::ffi::c_int
                        | G_URI_FLAGS_HAS_AUTH_PARAMS as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    != 0
            {
                if safe_c2rust_parse_userinfo(
                    p,
                    at.offset_from(p) as ::core::ffi::c_long as gsize,
                    flags,
                    user,
                    password,
                    auth_params,
                    error,
                ) == 0
                {
                    current_block = 2640329927079296558;
                } else {
                    current_block = 3222590281903869779;
                }
            } else {
                current_block = 3222590281903869779;
            }
            match current_block {
                2640329927079296558 => {}
                _ => {
                    if safe_c2rust_uri_normalize(
                        userinfo,
                        p,
                        at.offset_from(p) as ::core::ffi::c_long as gsize,
                        flags,
                        G_URI_ERROR_BAD_USER,
                        error,
                    ) == 0
                    {
                        current_block = 2640329927079296558;
                    } else {
                        p = at.offset(1 as ::core::ffi::c_int as isize);
                        current_block = 15004371738079956865;
                    }
                }
            }
        } else {
            current_block = 15004371738079956865;
        }
        match current_block {
            2640329927079296558 => {}
            _ => {
                if flags as ::core::ffi::c_uint
                    & G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    semi = strchr(p as *const ::core::ffi::c_char, ';' as i32);
                    if !semi.is_null() && semi < path_start {
                        path_start = semi;
                    }
                }
                if *p as ::core::ffi::c_int == '[' as i32 {
                    bracket = memchr(
                        p as *const ::core::ffi::c_void,
                        ']' as i32,
                        path_start.offset_from(p) as ::core::ffi::c_long as size_t,
                    ) as *const gchar;
                    if !bracket.is_null()
                        && *bracket.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == ':' as i32
                    {
                        colon = bracket.offset(1 as ::core::ffi::c_int as isize);
                    } else {
                        colon = ::core::ptr::null::<gchar>();
                    }
                } else {
                    colon = memchr(
                        p as *const ::core::ffi::c_void,
                        ':' as i32,
                        path_start.offset_from(p) as ::core::ffi::c_long as size_t,
                    ) as *const gchar;
                }
                hostend = if !colon.is_null() { colon } else { path_start };
                if safe_c2rust_parse_host(
                    p,
                    hostend.offset_from(p) as ::core::ffi::c_long as gsize,
                    flags,
                    host,
                    error,
                ) == 0
                {
                    current_block = 2640329927079296558;
                } else {
                    if !colon.is_null()
                        && colon != path_start.offset(-(1 as ::core::ffi::c_int as isize))
                    {
                        p = colon.offset(1 as ::core::ffi::c_int as isize);
                        if safe_c2rust_parse_port(
                            p,
                            path_start.offset_from(p) as ::core::ffi::c_long as gsize,
                            port,
                            error,
                        ) == 0
                        {
                            current_block = 2640329927079296558;
                        } else {
                            current_block = 11441799814184323368;
                        }
                    } else {
                        current_block = 11441799814184323368;
                    }
                    match current_block {
                        2640329927079296558 => {}
                        _ => {
                            p = path_start;
                            current_block = 3546145585875536353;
                        }
                    }
                }
            }
        }
    } else {
        current_block = 3546145585875536353;
    }
    match current_block {
        3546145585875536353 => {
            end = p.offset(strcspn(
                p as *const ::core::ffi::c_char,
                b"#\0" as *const u8 as *const ::core::ffi::c_char,
            ) as isize);
            if *end as ::core::ffi::c_int == '#' as i32 {
                if safe_c2rust_uri_normalize(
                    fragment,
                    end.offset(1 as ::core::ffi::c_int as isize),
                    strlen(end.offset(1 as ::core::ffi::c_int as isize)) as gsize,
                    (flags as ::core::ffi::c_uint
                        | (if flags as ::core::ffi::c_uint
                            & G_URI_FLAGS_ENCODED_FRAGMENT as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            != 0
                        {
                            G_URI_FLAGS_ENCODED as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint) as GUriFlags,
                    G_URI_ERROR_BAD_FRAGMENT,
                    error,
                ) == 0
                {
                    current_block = 2640329927079296558;
                } else {
                    current_block = 13125627826496529465;
                }
            } else {
                current_block = 13125627826496529465;
            }
            match current_block {
                2640329927079296558 => {}
                _ => {
                    question = memchr(
                        p as *const ::core::ffi::c_void,
                        '?' as i32,
                        end.offset_from(p) as ::core::ffi::c_long as size_t,
                    ) as *const gchar;
                    if !question.is_null() {
                        if safe_c2rust_uri_normalize(
                            query,
                            question.offset(1 as ::core::ffi::c_int as isize),
                            end.offset_from(question.offset(1 as ::core::ffi::c_int as isize))
                                as ::core::ffi::c_long as gsize,
                            (flags as ::core::ffi::c_uint
                                | (if flags as ::core::ffi::c_uint
                                    & G_URI_FLAGS_ENCODED_QUERY as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                {
                                    G_URI_FLAGS_ENCODED as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as ::core::ffi::c_uint) as GUriFlags,
                            G_URI_ERROR_BAD_QUERY,
                            error,
                        ) == 0
                        {
                            current_block = 2640329927079296558;
                        } else {
                            end = question;
                            current_block = 5028470053297453708;
                        }
                    } else {
                        current_block = 5028470053297453708;
                    }
                    match current_block {
                        2640329927079296558 => {}
                        _ => {
                            if !(safe_c2rust_uri_normalize(
                                path,
                                p,
                                end.offset_from(p) as ::core::ffi::c_long as gsize,
                                (flags as ::core::ffi::c_uint
                                    | (if flags as ::core::ffi::c_uint
                                        & G_URI_FLAGS_ENCODED_PATH as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        != 0
                                    {
                                        G_URI_FLAGS_ENCODED as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    }) as ::core::ffi::c_uint)
                                    as GUriFlags,
                                G_URI_ERROR_BAD_PATH,
                                error,
                            ) == 0)
                            {
                                if flags as ::core::ffi::c_uint
                                    & G_URI_FLAGS_SCHEME_NORMALIZE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                    && (!scheme.is_null() && !(*scheme).is_null()
                                        || !normalized_scheme.is_null())
                                {
                                    let mut scheme_str: *const ::core::ffi::c_char =
                                        if !scheme.is_null() && !(*scheme).is_null() {
                                            *scheme
                                        } else {
                                            normalized_scheme
                                        };
                                    if safe_c2rust_should_normalize_empty_path(scheme_str) != 0
                                        && !path.is_null()
                                        && **path == 0
                                    {
                                        g_free(*path as gpointer);
                                        *path = safe_c2rust_g_strdup_inline(
                                            b"/\0" as *const u8 as *const ::core::ffi::c_char,
                                        )
                                            as *mut gchar;
                                    }
                                    if !port.is_null() && *port == -(1 as ::core::ffi::c_int) {
                                        *port =
                                            safe_c2rust_g_uri_get_default_scheme_port(scheme_str)
                                                as gint;
                                    }
                                }
                                g_free(normalized_scheme as gpointer);
                                g_free(cleaned_uri_string as gpointer);
                                return TRUE;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !scheme.is_null() {
        let mut _pp: *mut *mut gchar = scheme as *mut *mut gchar;
        let mut _ptr: *mut gchar = *_pp;
        *_pp = ::core::ptr::null_mut::<gchar>();
        if !_ptr.is_null() {
            g_free(_ptr as gpointer);
        }
    }
    if !userinfo.is_null() {
        let mut _pp_0: *mut *mut gchar = userinfo as *mut *mut gchar;
        let mut _ptr_0: *mut gchar = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_0.is_null() {
            g_free(_ptr_0 as gpointer);
        }
    }
    if !host.is_null() {
        let mut _pp_1: *mut *mut gchar = host as *mut *mut gchar;
        let mut _ptr_1: *mut gchar = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_1.is_null() {
            g_free(_ptr_1 as gpointer);
        }
    }
    if !port.is_null() {
        *port = -(1 as ::core::ffi::c_int) as gint;
    }
    if !path.is_null() {
        let mut _pp_2: *mut *mut gchar = path as *mut *mut gchar;
        let mut _ptr_2: *mut gchar = *_pp_2;
        *_pp_2 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_2.is_null() {
            g_free(_ptr_2 as gpointer);
        }
    }
    if !query.is_null() {
        let mut _pp_3: *mut *mut gchar = query as *mut *mut gchar;
        let mut _ptr_3: *mut gchar = *_pp_3;
        *_pp_3 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_3.is_null() {
            g_free(_ptr_3 as gpointer);
        }
    }
    if !fragment.is_null() {
        let mut _pp_4: *mut *mut gchar = fragment as *mut *mut gchar;
        let mut _ptr_4: *mut gchar = *_pp_4;
        *_pp_4 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_4.is_null() {
            g_free(_ptr_4 as gpointer);
        }
    }
    g_free(normalized_scheme as gpointer);
    g_free(cleaned_uri_string as gpointer);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_split(
    mut uri_ref: *const gchar,
    mut flags: GUriFlags,
    mut scheme: *mut *mut gchar,
    mut userinfo: *mut *mut gchar,
    mut host: *mut *mut gchar,
    mut port: *mut gint,
    mut path: *mut *mut gchar,
    mut query: *mut *mut gchar,
    mut fragment: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !uri_ref.is_null() {
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
            b"uri_ref != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_g_uri_split_internal(
        uri_ref,
        flags,
        scheme,
        userinfo,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        host,
        port,
        path,
        query,
        fragment,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_split_with_user(
    mut uri_ref: *const gchar,
    mut flags: GUriFlags,
    mut scheme: *mut *mut gchar,
    mut user: *mut *mut gchar,
    mut password: *mut *mut gchar,
    mut auth_params: *mut *mut gchar,
    mut host: *mut *mut gchar,
    mut port: *mut gint,
    mut path: *mut *mut gchar,
    mut query: *mut *mut gchar,
    mut fragment: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !uri_ref.is_null() {
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
            b"uri_ref != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_g_uri_split_internal(
        uri_ref,
        flags,
        scheme,
        ::core::ptr::null_mut::<*mut gchar>(),
        user,
        password,
        auth_params,
        host,
        port,
        path,
        query,
        fragment,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_split_network(
    mut uri_string: *const gchar,
    mut flags: GUriFlags,
    mut scheme: *mut *mut gchar,
    mut host: *mut *mut gchar,
    mut port: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut my_scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut my_host: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !uri_string.is_null() {
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
            b"uri_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_uri_split_internal(
        uri_string,
        flags,
        &raw mut my_scheme,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        &raw mut my_host,
        port,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        error,
    ) == 0
    {
        return FALSE;
    }
    if my_scheme.is_null() || my_host.is_null() {
        if my_scheme.is_null() {
            g_set_error(
                error,
                safe_c2rust_g_uri_error_quark(),
                G_URI_ERROR_BAD_SCHEME as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"URI \xE2\x80\x98%s\xE2\x80\x99 is not an absolute URI\0" as *const u8
                        as *const gchar,
                ),
                uri_string,
            );
        } else {
            g_set_error(
                error,
                safe_c2rust_g_uri_error_quark(),
                G_URI_ERROR_BAD_HOST as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"URI \xE2\x80\x98%s\xE2\x80\x99 has no host component\0" as *const u8
                        as *const gchar,
                ),
                uri_string,
            );
        }
        g_free(my_scheme as gpointer);
        g_free(my_host as gpointer);
        return FALSE;
    }
    if !scheme.is_null() {
        *scheme =
            safe_c2rust_g_steal_pointer(&raw mut my_scheme as gpointer) as *mut gchar as *mut gchar;
    }
    if !host.is_null() {
        *host =
            safe_c2rust_g_steal_pointer(&raw mut my_host as gpointer) as *mut gchar as *mut gchar;
    }
    g_free(my_scheme as gpointer);
    g_free(my_host as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_is_valid(
    mut uri_string: *const gchar,
    mut flags: GUriFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut my_scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !uri_string.is_null() {
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
            b"uri_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_uri_split_internal(
        uri_string,
        flags,
        &raw mut my_scheme,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<gint>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
        error,
    ) == 0
    {
        return FALSE;
    }
    if my_scheme.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_BAD_SCHEME as ::core::ffi::c_int as gint,
            glib_gettext(
                b"URI \xE2\x80\x98%s\xE2\x80\x99 is not an absolute URI\0" as *const u8
                    as *const gchar,
            ),
            uri_string,
        );
        return FALSE;
    }
    g_free(my_scheme as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_remove_dot_segments(mut path: *mut gchar) {
    let mut input: *mut gchar = path;
    let mut output: *mut gchar = path;
    if *path == 0 {
        return;
    }
    while *input != 0 {
        if strncmp(
            input,
            b"../\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            input = input.offset(3 as ::core::ffi::c_int as isize);
        } else if strncmp(
            input,
            b"./\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            input = input.offset(2 as ::core::ffi::c_int as isize);
        } else if strncmp(
            input,
            b"/./\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            input = input.offset(2 as ::core::ffi::c_int as isize);
        } else if strcmp(input, b"/.\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            *input.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
        } else if strncmp(
            input,
            b"/../\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            input = input.offset(3 as ::core::ffi::c_int as isize);
            if output > path {
                loop {
                    output = output.offset(-1);
                    if !(*output as ::core::ffi::c_int != '/' as i32 && output > path) {
                        break;
                    }
                }
            }
        } else if strcmp(input, b"/..\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            *input.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
            if output > path {
                loop {
                    output = output.offset(-1);
                    if !(*output as ::core::ffi::c_int != '/' as i32 && output > path) {
                        break;
                    }
                }
            }
        } else if strcmp(input, b"..\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(input, b".\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            *input.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as gchar;
        } else {
            let fresh1 = input;
            input = input.offset(1);
            let fresh2 = output;
            output = output.offset(1);
            *fresh2 = *fresh1;
            while *input as ::core::ffi::c_int != 0 && *input as ::core::ffi::c_int != '/' as i32 {
                let fresh3 = input;
                input = input.offset(1);
                let fresh4 = output;
                output = output.offset(1);
                *fresh4 = *fresh3;
            }
        }
    }
    *output = '\0' as i32 as gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_parse(
    mut uri_string: *const gchar,
    mut flags: GUriFlags,
    mut error: *mut *mut GError,
) -> *mut GUri {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !uri_string.is_null() {
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
            b"uri_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    return safe_c2rust_g_uri_parse_relative(
        ::core::ptr::null_mut::<GUri>(),
        uri_string,
        flags,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_parse_relative(
    mut base_uri: *mut GUri,
    mut uri_ref: *const gchar,
    mut flags: GUriFlags,
    mut error: *mut *mut GError,
) -> *mut GUri {
    let mut uri: *mut GUri = ::core::ptr::null_mut::<GUri>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !uri_ref.is_null() {
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
            b"uri_ref != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if base_uri.is_null() || !(*base_uri).scheme.is_null() {
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
            b"base_uri == NULL || base_uri->scheme != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    uri = g_atomic_rc_box_alloc0(::core::mem::size_of::<GUri>() as gsize) as *mut GUri;
    (*uri).flags = flags;
    if safe_c2rust_g_uri_split_internal(
        uri_ref,
        flags,
        &raw mut (*uri).scheme,
        &raw mut (*uri).userinfo,
        &raw mut (*uri).user,
        &raw mut (*uri).password,
        &raw mut (*uri).auth_params,
        &raw mut (*uri).host,
        &raw mut (*uri).port,
        &raw mut (*uri).path,
        &raw mut (*uri).query,
        &raw mut (*uri).fragment,
        error,
    ) == 0
    {
        safe_c2rust_g_uri_unref(uri);
        return ::core::ptr::null_mut::<GUri>();
    }
    if (*uri).scheme.is_null() && base_uri.is_null() {
        g_set_error_literal(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"URI is not absolute, and no base URI was provided\0" as *const u8 as *const gchar,
            ),
        );
        safe_c2rust_g_uri_unref(uri);
        return ::core::ptr::null_mut::<GUri>();
    }
    if !base_uri.is_null() {
        if !(*uri).scheme.is_null() {
            safe_c2rust_remove_dot_segments((*uri).path);
        } else {
            (*uri).scheme = safe_c2rust_g_strdup_inline((*base_uri).scheme) as *mut gchar;
            if !(*uri).host.is_null() {
                safe_c2rust_remove_dot_segments((*uri).path);
            } else {
                if *(*uri).path == 0 {
                    g_free((*uri).path as gpointer);
                    (*uri).path = safe_c2rust_g_strdup_inline((*base_uri).path) as *mut gchar;
                    if (*uri).query.is_null() {
                        (*uri).query = safe_c2rust_g_strdup_inline((*base_uri).query) as *mut gchar;
                    }
                } else if *(*uri).path as ::core::ffi::c_int == '/' as i32 {
                    safe_c2rust_remove_dot_segments((*uri).path);
                } else {
                    let mut newpath: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    let mut last: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    last = strrchr((*base_uri).path, '/' as i32) as *mut gchar;
                    if !last.is_null() {
                        newpath = g_strdup_printf(
                            b"%.*s/%s\0" as *const u8 as *const gchar,
                            last.offset_from((*base_uri).path) as ::core::ffi::c_long as gint,
                            (*base_uri).path,
                            (*uri).path,
                        );
                    } else {
                        newpath =
                            g_strdup_printf(b"/%s\0" as *const u8 as *const gchar, (*uri).path);
                    }
                    g_free((*uri).path as gpointer);
                    (*uri).path = safe_c2rust_g_steal_pointer(&raw mut newpath as gpointer)
                        as *mut gchar as *mut gchar;
                    safe_c2rust_remove_dot_segments((*uri).path);
                }
                (*uri).userinfo = safe_c2rust_g_strdup_inline((*base_uri).userinfo) as *mut gchar;
                (*uri).user = safe_c2rust_g_strdup_inline((*base_uri).user) as *mut gchar;
                (*uri).password = safe_c2rust_g_strdup_inline((*base_uri).password) as *mut gchar;
                (*uri).auth_params =
                    safe_c2rust_g_strdup_inline((*base_uri).auth_params) as *mut gchar;
                (*uri).host = safe_c2rust_g_strdup_inline((*base_uri).host) as *mut gchar;
                (*uri).port = (*base_uri).port;
            }
        }
        if flags as ::core::ffi::c_uint
            & G_URI_FLAGS_SCHEME_NORMALIZE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            if safe_c2rust_should_normalize_empty_path((*uri).scheme) != 0 && *(*uri).path == 0 {
                g_free((*uri).path as gpointer);
                (*uri).path =
                    safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char)
                        as *mut gchar;
            }
            (*uri).port =
                safe_c2rust_normalize_port((*uri).scheme, (*uri).port as ::core::ffi::c_int)
                    as gint;
        }
    } else {
        safe_c2rust_remove_dot_segments((*uri).path);
    }
    return safe_c2rust_g_steal_pointer(&raw mut uri as gpointer) as *mut GUri;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_resolve_relative(
    mut base_uri_string: *const gchar,
    mut uri_ref: *const gchar,
    mut flags: GUriFlags,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut base_uri: *mut GUri = ::core::ptr::null_mut::<GUri>();
    let mut resolved_uri: *mut GUri = ::core::ptr::null_mut::<GUri>();
    let mut resolved_uri_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !uri_ref.is_null() {
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
            b"uri_ref != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    flags = ::core::mem::transmute::<::core::ffi::c_uint, GUriFlags>(
        flags as ::core::ffi::c_uint
            | G_URI_FLAGS_ENCODED as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if !base_uri_string.is_null() {
        base_uri = safe_c2rust_g_uri_parse(base_uri_string, flags, error);
        if base_uri.is_null() {
            return ::core::ptr::null_mut::<gchar>();
        }
    } else {
        base_uri = ::core::ptr::null_mut::<GUri>();
    }
    resolved_uri = safe_c2rust_g_uri_parse_relative(base_uri, uri_ref, flags, error);
    if !base_uri.is_null() {
        safe_c2rust_g_uri_unref(base_uri);
    }
    if resolved_uri.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    resolved_uri_string = safe_c2rust_g_uri_to_string(resolved_uri) as *mut gchar;
    safe_c2rust_g_uri_unref(resolved_uri);
    return safe_c2rust_g_steal_pointer(&raw mut resolved_uri_string as gpointer) as *mut gchar;
}
pub const USER_ALLOWED_CHARS: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"!$&'()*+,=\0") };
pub const PASSWORD_ALLOWED_CHARS: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"!$&'()*+,=:\0") };
pub const IP_ADDR_ALLOWED_CHARS: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b":\0") };
pub const HOST_ALLOWED_CHARS: [::core::ffi::c_char; 12] =
    G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS;
unsafe extern "C" fn safe_c2rust_g_uri_join_internal(
    mut flags: GUriFlags,
    mut scheme: *const gchar,
    mut userinfo: gboolean,
    mut user: *const gchar,
    mut password: *const gchar,
    mut auth_params: *const gchar,
    mut host: *const gchar,
    mut port: gint,
    mut path: *const gchar,
    mut query: *const gchar,
    mut fragment: *const gchar,
) -> *mut gchar {
    let mut encoded: gboolean = (flags as ::core::ffi::c_uint
        & G_URI_FLAGS_ENCODED as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut normalized_scheme: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if host.is_null()
            || (*path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
                || *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32)
        {
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
            b"host == NULL || (path[0] == '\\0' || path[0] == '/')\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !host.is_null()
            || (*path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32
                || *path.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '/' as i32)
        {
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
            b"host != NULL || (path[0] != '/' || path[1] != '/')\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    str = g_string_sized_new(127 as gsize);
    if !scheme.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = scheme as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_31
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
                str,
                scheme as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(str, ':' as i32 as gchar);
    }
    if flags as ::core::ffi::c_uint
        & G_URI_FLAGS_SCHEME_NORMALIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !scheme.is_null()
        && (!host.is_null() && port != -(1 as ::core::ffi::c_int)
            || *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32)
    {
        normalized_scheme = g_ascii_strdown(scheme, -(1 as ::core::ffi::c_int) as gssize)
            as *mut ::core::ffi::c_char;
    }
    if !host.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"//\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_32
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
                str,
                b"//\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if !user.is_null() {
            if encoded != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = user as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            __val,
                            if ({
                                let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_33 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_33 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_33
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
                        str,
                        user as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else if userinfo != 0 {
                g_string_append_uri_escaped(
                    str,
                    user,
                    b"!$&'()*+,;=:\0" as *const u8 as *const gchar,
                    TRUE,
                );
            } else {
                g_string_append_uri_escaped(
                    str,
                    user,
                    USER_ALLOWED_CHARS.as_ptr() as *const gchar,
                    TRUE,
                );
            }
            if !password.is_null() {
                safe_c2rust_g_string_append_c_inline(str, ':' as i32 as gchar);
                if encoded != 0 {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                password as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                str,
                                __val,
                                if ({
                                    let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_34
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
                            str,
                            password as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                } else {
                    g_string_append_uri_escaped(
                        str,
                        password,
                        PASSWORD_ALLOWED_CHARS.as_ptr() as *const gchar,
                        TRUE,
                    );
                }
            }
            if !auth_params.is_null() {
                safe_c2rust_g_string_append_c_inline(str, ';' as i32 as gchar);
                if encoded != 0 {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                auth_params as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                str,
                                __val,
                                if ({
                                    let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_35 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_35 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_35
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
                            str,
                            auth_params as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                } else {
                    g_string_append_uri_escaped(
                        str,
                        auth_params,
                        b"!$&'()*+,;=:\0" as *const u8 as *const gchar,
                        TRUE,
                    );
                }
            }
            safe_c2rust_g_string_append_c_inline(str, '@' as i32 as gchar);
        }
        if !strchr(host as *const ::core::ffi::c_char, ':' as i32).is_null()
            && g_hostname_is_ip_address(host) != 0
        {
            safe_c2rust_g_string_append_c_inline(str, '[' as i32 as gchar);
            if encoded != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = host as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            __val,
                            if ({
                                let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_36 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_36 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_36
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
                        str,
                        host as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else {
                g_string_append_uri_escaped(
                    str,
                    host,
                    IP_ADDR_ALLOWED_CHARS.as_ptr() as *const gchar,
                    TRUE,
                );
            }
            safe_c2rust_g_string_append_c_inline(str, ']' as i32 as gchar);
        } else if encoded != 0 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = host as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_37 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_37 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_37
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
                    str,
                    host as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            g_string_append_uri_escaped(
                str,
                host,
                HOST_ALLOWED_CHARS.as_ptr() as *const gchar,
                TRUE,
            );
        }
        if port != -(1 as ::core::ffi::c_int)
            && (normalized_scheme.is_null()
                || safe_c2rust_normalize_port(normalized_scheme, port as ::core::ffi::c_int)
                    != -(1 as ::core::ffi::c_int))
        {
            g_string_append_printf(str, b":%d\0" as *const u8 as *const gchar, port);
        }
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        && !normalized_scheme.is_null()
        && safe_c2rust_should_normalize_empty_path(normalized_scheme) != 0
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_38
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
                str,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else if encoded != 0
        || flags as ::core::ffi::c_uint
            & G_URI_FLAGS_ENCODED_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = path as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_39
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
                str,
                path as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        g_string_append_uri_escaped(
            str,
            path,
            b"!$&'()*+,;=:@/\0" as *const u8 as *const gchar,
            TRUE,
        );
    }
    g_free(normalized_scheme as gpointer);
    if !query.is_null() {
        safe_c2rust_g_string_append_c_inline(str, '?' as i32 as gchar);
        if encoded != 0
            || flags as ::core::ffi::c_uint
                & G_URI_FLAGS_ENCODED_QUERY as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = query as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_40 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_40 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_40
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
                    str,
                    query as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            g_string_append_uri_escaped(
                str,
                query,
                b"!$&'()*+,;=:@/?\0" as *const u8 as *const gchar,
                TRUE,
            );
        }
    }
    if !fragment.is_null() {
        safe_c2rust_g_string_append_c_inline(str, '#' as i32 as gchar);
        if encoded != 0
            || flags as ::core::ffi::c_uint
                & G_URI_FLAGS_ENCODED_FRAGMENT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = fragment as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_41
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
                    str,
                    fragment as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            g_string_append_uri_escaped(
                str,
                fragment,
                b"!$&'()*+,;=:@/?\0" as *const u8 as *const gchar,
                TRUE,
            );
        }
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(str, 0 as gboolean)
        } else {
            g_string_free_and_steal(str)
        }
    } else {
        g_string_free(str, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_join(
    mut flags: GUriFlags,
    mut scheme: *const gchar,
    mut userinfo: *const gchar,
    mut host: *const gchar,
    mut port: gint,
    mut path: *const gchar,
    mut query: *const gchar,
    mut fragment: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if port >= -(1 as ::core::ffi::c_int) && port <= 65535 as ::core::ffi::c_int {
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
            b"port >= -1 && port <= 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_uri_join_internal(
        flags,
        scheme,
        TRUE,
        userinfo,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        host,
        port,
        path,
        query,
        fragment,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_join_with_user(
    mut flags: GUriFlags,
    mut scheme: *const gchar,
    mut user: *const gchar,
    mut password: *const gchar,
    mut auth_params: *const gchar,
    mut host: *const gchar,
    mut port: gint,
    mut path: *const gchar,
    mut query: *const gchar,
    mut fragment: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if port >= -(1 as ::core::ffi::c_int) && port <= 65535 as ::core::ffi::c_int {
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
            b"port >= -1 && port <= 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_uri_join_internal(
        flags,
        scheme,
        FALSE,
        user,
        password,
        auth_params,
        host,
        port,
        path,
        query,
        fragment,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_build(
    mut flags: GUriFlags,
    mut scheme: *const gchar,
    mut userinfo: *const gchar,
    mut host: *const gchar,
    mut port: gint,
    mut path: *const gchar,
    mut query: *const gchar,
    mut fragment: *const gchar,
) -> *mut GUri {
    let mut uri: *mut GUri = ::core::ptr::null_mut::<GUri>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !scheme.is_null() {
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
            b"scheme != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if port >= -(1 as ::core::ffi::c_int) && port <= 65535 as ::core::ffi::c_int {
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
            b"port >= -1 && port <= 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    uri = g_atomic_rc_box_alloc0(::core::mem::size_of::<GUri>() as gsize) as *mut GUri;
    (*uri).flags = flags;
    (*uri).scheme = g_ascii_strdown(scheme, -(1 as ::core::ffi::c_int) as gssize);
    (*uri).userinfo =
        safe_c2rust_g_strdup_inline(userinfo as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).host = safe_c2rust_g_strdup_inline(host as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).port = port;
    (*uri).path = safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).query = safe_c2rust_g_strdup_inline(query as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).fragment =
        safe_c2rust_g_strdup_inline(fragment as *const ::core::ffi::c_char) as *mut gchar;
    return safe_c2rust_g_steal_pointer(&raw mut uri as gpointer) as *mut GUri;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_build_with_user(
    mut flags: GUriFlags,
    mut scheme: *const gchar,
    mut user: *const gchar,
    mut password: *const gchar,
    mut auth_params: *const gchar,
    mut host: *const gchar,
    mut port: gint,
    mut path: *const gchar,
    mut query: *const gchar,
    mut fragment: *const gchar,
) -> *mut GUri {
    let mut uri: *mut GUri = ::core::ptr::null_mut::<GUri>();
    let mut userinfo: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !scheme.is_null() {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"scheme != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if password.is_null() || !user.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"password == NULL || user != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if auth_params.is_null() || !user.is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"auth_params == NULL || user != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if port >= -(1 as ::core::ffi::c_int) && port <= 65535 as ::core::ffi::c_int {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"port >= -1 && port <= 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !path.is_null() {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUri>();
    }
    uri = g_atomic_rc_box_alloc0(::core::mem::size_of::<GUri>() as gsize) as *mut GUri;
    (*uri).flags = (flags as ::core::ffi::c_uint
        | G_URI_FLAGS_HAS_PASSWORD as ::core::ffi::c_int as ::core::ffi::c_uint)
        as GUriFlags;
    (*uri).scheme = g_ascii_strdown(scheme, -(1 as ::core::ffi::c_int) as gssize);
    (*uri).user = safe_c2rust_g_strdup_inline(user as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).password =
        safe_c2rust_g_strdup_inline(password as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).auth_params =
        safe_c2rust_g_strdup_inline(auth_params as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).host = safe_c2rust_g_strdup_inline(host as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).port = port;
    (*uri).path = safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).query = safe_c2rust_g_strdup_inline(query as *const ::core::ffi::c_char) as *mut gchar;
    (*uri).fragment =
        safe_c2rust_g_strdup_inline(fragment as *const ::core::ffi::c_char) as *mut gchar;
    if !user.is_null() {
        userinfo = g_string_new(user);
        if !password.is_null() {
            safe_c2rust_g_string_append_c_inline(userinfo, ':' as i32 as gchar);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = (*uri).password;
                    safe_c2rust_g_string_append_len_inline(
                        userinfo,
                        __val,
                        if ({
                            let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_54 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_54 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_54
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
                    userinfo,
                    (*uri).password,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        if !auth_params.is_null() {
            safe_c2rust_g_string_append_c_inline(userinfo, ';' as i32 as gchar);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = (*uri).auth_params;
                    safe_c2rust_g_string_append_len_inline(
                        userinfo,
                        __val,
                        if ({
                            let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_55 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_55 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_55
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
                    userinfo,
                    (*uri).auth_params,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        (*uri).userinfo = if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(userinfo, 0 as gboolean)
            } else {
                g_string_free_and_steal(userinfo)
            }
        } else {
            g_string_free(userinfo, 0 as gboolean)
        };
    }
    return safe_c2rust_g_steal_pointer(&raw mut uri as gpointer) as *mut GUri;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_to_string(
    mut uri: *mut GUri,
) -> *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return safe_c2rust_g_uri_to_string_partial(uri, G_URI_HIDE_NONE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_to_string_partial(
    mut uri: *mut GUri,
    mut flags: GUriHideFlags,
) -> *mut ::core::ffi::c_char {
    let mut hide_user: gboolean = (flags as ::core::ffi::c_uint
        & G_URI_HIDE_USERINFO as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    let mut hide_password: gboolean = (flags as ::core::ffi::c_uint
        & (G_URI_HIDE_USERINFO as ::core::ffi::c_int | G_URI_HIDE_PASSWORD as ::core::ffi::c_int)
            as ::core::ffi::c_uint) as gboolean;
    let mut hide_auth_params: gboolean = (flags as ::core::ffi::c_uint
        & (G_URI_HIDE_USERINFO as ::core::ffi::c_int | G_URI_HIDE_AUTH_PARAMS as ::core::ffi::c_int)
            as ::core::ffi::c_uint) as gboolean;
    let mut hide_query: gboolean = (flags as ::core::ffi::c_uint
        & G_URI_HIDE_QUERY as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    let mut hide_fragment: gboolean = (flags as ::core::ffi::c_uint
        & G_URI_HIDE_FRAGMENT as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*uri).flags as ::core::ffi::c_uint
        & (G_URI_FLAGS_HAS_PASSWORD as ::core::ffi::c_int
            | G_URI_FLAGS_HAS_AUTH_PARAMS as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        return safe_c2rust_g_uri_join_with_user(
            (*uri).flags,
            (*uri).scheme,
            if hide_user != 0 {
                ::core::ptr::null_mut::<gchar>()
            } else {
                (*uri).user
            },
            if hide_password != 0 {
                ::core::ptr::null_mut::<gchar>()
            } else {
                (*uri).password
            },
            if hide_auth_params != 0 {
                ::core::ptr::null_mut::<gchar>()
            } else {
                (*uri).auth_params
            },
            (*uri).host,
            (*uri).port,
            (*uri).path,
            if hide_query != 0 {
                ::core::ptr::null_mut::<gchar>()
            } else {
                (*uri).query
            },
            if hide_fragment != 0 {
                ::core::ptr::null_mut::<gchar>()
            } else {
                (*uri).fragment
            },
        ) as *mut ::core::ffi::c_char;
    }
    return safe_c2rust_g_uri_join(
        (*uri).flags,
        (*uri).scheme,
        if hide_user != 0 {
            ::core::ptr::null_mut::<gchar>()
        } else {
            (*uri).userinfo
        },
        (*uri).host,
        (*uri).port,
        (*uri).path,
        if hide_query != 0 {
            ::core::ptr::null_mut::<gchar>()
        } else {
            (*uri).query
        },
        if hide_fragment != 0 {
            ::core::ptr::null_mut::<gchar>()
        } else {
            (*uri).fragment
        },
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_str_ascii_case_hash(mut v: gconstpointer) -> guint {
    let mut p: *const ::core::ffi::c_schar = ::core::ptr::null::<::core::ffi::c_schar>();
    let mut h: guint32 = 5381 as guint32;
    p = v as *const ::core::ffi::c_schar;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_add(h)
            .wrapping_add(g_ascii_toupper(*p as gchar) as guint32);
        p = p.offset(1);
    }
    return h as guint;
}
unsafe extern "C" fn safe_c2rust_str_ascii_case_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    let mut string1: *const gchar = v1 as *const gchar;
    let mut string2: *const gchar = v2 as *const gchar;
    return (g_ascii_strcasecmp(string1, string2) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_params_iter_init(
    mut iter: *mut GUriParamsIter,
    mut params: *const gchar,
    mut length: gssize,
    mut separators: *const gchar,
    mut flags: GUriParamsFlags,
) {
    let mut ri: *mut RealIter = iter as *mut RealIter;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !iter.is_null() {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !params.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if length >= -(1 as ::core::ffi::c_int) as gssize {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !separators.is_null() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"separators != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*ri).flags = flags;
    if length == -(1 as ::core::ffi::c_int) as gssize {
        (*ri).end = params.offset(strlen(params as *const ::core::ffi::c_char) as isize);
    } else {
        (*ri).end = params.offset(length as isize);
    }
    memset(
        &raw mut (*ri).sep_table as *mut guint8 as *mut ::core::ffi::c_void,
        FALSE,
        ::core::mem::size_of::<[guint8; 256]>() as size_t,
    );
    s = separators;
    while *s as ::core::ffi::c_int != '\0' as i32 {
        (*ri).sep_table[*(s as *mut guchar) as usize] = TRUE as guint8;
        s = s.offset(1);
    }
    (*ri).attr = params;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_params_iter_next(
    mut iter: *mut GUriParamsIter,
    mut attribute: *mut *mut gchar,
    mut value: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ri: *mut RealIter = iter as *mut RealIter;
    let mut attr_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut val: *const gchar = ::core::ptr::null::<gchar>();
    let mut val_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut decoded_attr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut decoded_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut www_form: gboolean = ((*ri).flags as ::core::ffi::c_uint
        & G_URI_PARAMS_WWW_FORM as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    let mut decode_flags: GUriFlags = G_URI_FLAGS_NONE;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !iter.is_null() {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !attribute.is_null() {
        *attribute = ::core::ptr::null_mut::<gchar>();
    }
    if !value.is_null() {
        *value = ::core::ptr::null_mut::<gchar>();
    }
    if (*ri).attr >= (*ri).end {
        return FALSE;
    }
    if (*ri).flags as ::core::ffi::c_uint
        & G_URI_PARAMS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        decode_flags = ::core::mem::transmute::<::core::ffi::c_uint, GUriFlags>(
            decode_flags as ::core::ffi::c_uint
                | G_URI_FLAGS_PARSE_RELAXED as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    val_end = (*ri).attr;
    while val_end < (*ri).end {
        if (*ri).sep_table[*(val_end as *mut guchar) as usize] != 0 {
            break;
        }
        val_end = val_end.offset(1);
    }
    attr_end = memchr(
        (*ri).attr as *const ::core::ffi::c_void,
        '=' as i32,
        val_end.offset_from((*ri).attr) as ::core::ffi::c_long as size_t,
    ) as *const gchar;
    if attr_end.is_null() {
        g_set_error_literal(
            error,
            safe_c2rust_g_uri_error_quark(),
            G_URI_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Missing \xE2\x80\x98=\xE2\x80\x99 and parameter value\0" as *const u8
                    as *const gchar,
            ),
        );
        return FALSE;
    }
    if safe_c2rust_uri_decode(
        &raw mut decoded_attr,
        ::core::ptr::null::<gchar>(),
        (*ri).attr,
        attr_end.offset_from((*ri).attr) as ::core::ffi::c_long as gsize,
        www_form,
        decode_flags,
        G_URI_ERROR_FAILED,
        error,
    ) == 0
    {
        return FALSE;
    }
    val = attr_end.offset(1 as ::core::ffi::c_int as isize);
    if safe_c2rust_uri_decode(
        &raw mut decoded_value,
        ::core::ptr::null::<gchar>(),
        val,
        val_end.offset_from(val) as ::core::ffi::c_long as gsize,
        www_form,
        decode_flags,
        G_URI_ERROR_FAILED,
        error,
    ) == 0
    {
        g_free(decoded_attr as gpointer);
        return FALSE;
    }
    if !attribute.is_null() {
        *attribute = safe_c2rust_g_steal_pointer(&raw mut decoded_attr as gpointer) as *mut gchar
            as *mut gchar;
    }
    if !value.is_null() {
        *value = safe_c2rust_g_steal_pointer(&raw mut decoded_value as gpointer) as *mut gchar
            as *mut gchar;
    }
    g_free(decoded_attr as gpointer);
    g_free(decoded_value as gpointer);
    (*ri).attr = val_end.offset(1 as ::core::ffi::c_int as isize);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_parse_params(
    mut params: *const gchar,
    mut length: gssize,
    mut separators: *const gchar,
    mut flags: GUriParamsFlags,
    mut error: *mut *mut GError,
) -> *mut GHashTable {
    let mut hash: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut iter: GUriParamsIter = _GUriParamsIter {
        dummy0: 0,
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: [0; 256],
    };
    let mut attribute: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut err: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !params.is_null() {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || params != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if length >= -(1 as ::core::ffi::c_int) as gssize {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !separators.is_null() {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"separators != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHashTable>();
    }
    if flags as ::core::ffi::c_uint
        & G_URI_PARAMS_CASE_INSENSITIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        hash = g_hash_table_new_full(
            Some(safe_c2rust_str_ascii_case_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                safe_c2rust_str_ascii_case_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        hash = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    safe_c2rust_g_uri_params_iter_init(&raw mut iter, params, length, separators, flags);
    while safe_c2rust_g_uri_params_iter_next(
        &raw mut iter,
        &raw mut attribute,
        &raw mut value,
        &raw mut err,
    ) != 0
    {
        g_hash_table_insert(hash, attribute as gpointer, value as gpointer);
    }
    if !err.is_null() {
        g_propagate_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut err as gpointer) as *mut GError,
        );
        g_hash_table_destroy(hash);
        return ::core::ptr::null_mut::<GHashTable>();
    }
    return safe_c2rust_g_steal_pointer(&raw mut hash as gpointer) as *mut GHashTable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_scheme(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).scheme;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_userinfo(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).userinfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_user(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).user;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_password(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).password;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_auth_params(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).auth_params;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_host(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).host;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_port(mut uri: *mut GUri) -> gint {
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if (*uri).port == -(1 as ::core::ffi::c_int)
        && (*uri).flags as ::core::ffi::c_uint
            & G_URI_FLAGS_SCHEME_NORMALIZE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        return safe_c2rust_g_uri_get_default_scheme_port((*uri).scheme) as gint;
    }
    return (*uri).port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_path(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_query(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).query;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_fragment(mut uri: *mut GUri) -> *const gchar {
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*uri).fragment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_get_flags(mut uri: *mut GUri) -> GUriFlags {
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_URI_FLAGS_NONE;
    }
    return (*uri).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_unescape_segment(
    mut escaped_string: *const gchar,
    mut escaped_string_end: *const gchar,
    mut illegal_characters: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut unescaped: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    let mut decoded_len: gssize = 0;
    if escaped_string.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !escaped_string_end.is_null() {
        length = escaped_string_end.offset_from(escaped_string) as ::core::ffi::c_long as gsize;
    } else {
        length = strlen(escaped_string as *const ::core::ffi::c_char) as gsize;
    }
    decoded_len = safe_c2rust_uri_decoder(
        &raw mut unescaped,
        illegal_characters,
        escaped_string,
        length,
        FALSE,
        FALSE,
        G_URI_FLAGS_ENCODED,
        G_URI_ERROR_FAILED,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if decoded_len < 0 as gssize {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !memchr(
        unescaped as *const ::core::ffi::c_void,
        '\0' as i32,
        decoded_len as size_t,
    )
    .is_null()
    {
        g_free(unescaped as gpointer);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return unescaped as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_unescape_string(
    mut escaped_string: *const gchar,
    mut illegal_characters: *const gchar,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_uri_unescape_segment(
        escaped_string,
        ::core::ptr::null::<gchar>(),
        illegal_characters,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_escape_string(
    mut unescaped: *const gchar,
    mut reserved_chars_allowed: *const gchar,
    mut allow_utf8: gboolean,
) -> *mut ::core::ffi::c_char {
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if !unescaped.is_null() {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"unescaped != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = g_string_sized_new(
        (strlen(unescaped as *const ::core::ffi::c_char) as ::core::ffi::c_double * 1.25f64)
            as gsize,
    );
    g_string_append_uri_escaped(s, unescaped, reserved_chars_allowed, allow_utf8);
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(s, 0 as gboolean) as *mut ::core::ffi::c_char
        } else {
            g_string_free_and_steal(s) as *mut ::core::ffi::c_char
        }
    } else {
        g_string_free(s, 0 as gboolean) as *mut ::core::ffi::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_unescape_bytes(
    mut escaped_string: *const gchar,
    mut length: gssize,
    mut illegal_characters: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut unescaped_length: gssize = 0;
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if !escaped_string.is_null() {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"escaped_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    if length == -(1 as ::core::ffi::c_int) as gssize {
        length = strlen(escaped_string as *const ::core::ffi::c_char) as gssize;
    }
    unescaped_length = safe_c2rust_uri_decoder(
        &raw mut buf,
        illegal_characters as *const gchar,
        escaped_string,
        length as gsize,
        FALSE,
        FALSE,
        G_URI_FLAGS_ENCODED,
        G_URI_ERROR_FAILED,
        error,
    );
    if unescaped_length == -(1 as ::core::ffi::c_int) as gssize {
        return ::core::ptr::null_mut::<GBytes>();
    }
    return g_bytes_new_take(buf as gpointer, unescaped_length as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_escape_bytes(
    mut unescaped: *const guint8,
    mut length: gsize,
    mut reserved_chars_allowed: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if !unescaped.is_null() {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"unescaped != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    string = g_string_sized_new((length as ::core::ffi::c_double * 1.25f64) as gsize);
    safe_c2rust__uri_encoder(
        string,
        unescaped as *const guchar,
        length,
        reserved_chars_allowed,
        FALSE,
    );
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean) as *mut ::core::ffi::c_char
        } else {
            g_string_free_and_steal(string) as *mut ::core::ffi::c_char
        }
    } else {
        g_string_free(string, 0 as gboolean) as *mut ::core::ffi::c_char
    };
}
unsafe extern "C" fn safe_c2rust_g_uri_scheme_length(mut uri: *const gchar) -> gssize {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = uri;
    if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALPHA as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    p = p.offset(1);
    while *safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALNUM as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == '.' as i32
        || *p as ::core::ffi::c_int == '+' as i32
        || *p as ::core::ffi::c_int == '-' as i32
    {
        p = p.offset(1);
    }
    if p > uri && *p as ::core::ffi::c_int == ':' as i32 {
        return p.offset_from(uri) as gssize;
    }
    return -(1 as ::core::ffi::c_int) as gssize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_parse_scheme(
    mut uri: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut len: gssize = 0;
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    len = safe_c2rust_g_uri_scheme_length(uri);
    return if len == -(1 as ::core::ffi::c_int) as gssize {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        g_strndup(uri, len as gsize) as *mut ::core::ffi::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_peek_scheme(
    mut uri: *const gchar,
) -> *const ::core::ffi::c_char {
    let mut len: gssize = 0;
    let mut lower_scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut scheme: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    len = safe_c2rust_g_uri_scheme_length(uri);
    if len == -(1 as ::core::ffi::c_int) as gssize {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    lower_scheme = g_ascii_strdown(uri, len);
    scheme = g_intern_string(lower_scheme);
    g_free(lower_scheme as gpointer);
    return scheme as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_uri_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(b"g-uri-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
