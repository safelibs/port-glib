use ::c2rust_bitfields;
extern "C" {
    pub type _GHashTable;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_get_prgname() -> *const gchar;
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_dgettext(domain: *const gchar, msgid: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_unichar_iswide(c: gunichar) -> gboolean;
    fn g_unichar_iszerowidth(c: gunichar) -> gboolean;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
    fn g_ascii_strtoll(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
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
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_set_prgname_once(prgname: *const gchar) -> gboolean;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GTranslateFunc = Option<unsafe extern "C" fn(*const gchar, gpointer) -> *const gchar>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GOptionContext {
    pub groups: *mut GList,
    pub parameter_string: *mut gchar,
    pub summary: *mut gchar,
    pub description: *mut gchar,
    pub translate_func: GTranslateFunc,
    pub translate_notify: GDestroyNotify,
    pub translate_data: gpointer,
    #[bitfield(name = "help_enabled", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "ignore_unknown", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "strv_mode", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "strict_posix", ty = "guint", bits = "3..=3")]
    pub help_enabled_ignore_unknown_strv_mode_strict_posix: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub main_group: *mut GOptionGroup,
    pub changes: *mut GList,
    pub pending_nulls: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GOptionGroup = _GOptionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOptionGroup {
    pub name: *mut gchar,
    pub description: *mut gchar,
    pub help_description: *mut gchar,
    pub ref_count: gint,
    pub destroy_notify: GDestroyNotify,
    pub user_data: gpointer,
    pub translate_func: GTranslateFunc,
    pub translate_notify: GDestroyNotify,
    pub translate_data: gpointer,
    pub entries: *mut GOptionEntry,
    pub n_entries: gsize,
    pub pre_parse_func: GOptionParseFunc,
    pub post_parse_func: GOptionParseFunc,
    pub error_func: GOptionErrorFunc,
}
pub type GOptionErrorFunc = Option<
    unsafe extern "C" fn(*mut GOptionContext, *mut GOptionGroup, gpointer, *mut *mut GError) -> (),
>;
pub type GOptionContext = _GOptionContext;
pub type GOptionParseFunc = Option<
    unsafe extern "C" fn(
        *mut GOptionContext,
        *mut GOptionGroup,
        gpointer,
        *mut *mut GError,
    ) -> gboolean,
>;
pub type GOptionEntry = _GOptionEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOptionEntry {
    pub long_name: *const gchar,
    pub short_name: gchar,
    pub flags: gint,
    pub arg: GOptionArg,
    pub arg_data: gpointer,
    pub description: *const gchar,
    pub arg_description: *const gchar,
}
pub type GOptionArg = ::core::ffi::c_uint;
pub const G_OPTION_ARG_INT64: GOptionArg = 8;
pub const G_OPTION_ARG_DOUBLE: GOptionArg = 7;
pub const G_OPTION_ARG_FILENAME_ARRAY: GOptionArg = 6;
pub const G_OPTION_ARG_STRING_ARRAY: GOptionArg = 5;
pub const G_OPTION_ARG_FILENAME: GOptionArg = 4;
pub const G_OPTION_ARG_CALLBACK: GOptionArg = 3;
pub const G_OPTION_ARG_INT: GOptionArg = 2;
pub const G_OPTION_ARG_STRING: GOptionArg = 1;
pub const G_OPTION_ARG_NONE: GOptionArg = 0;
pub type GOptionFlags = ::core::ffi::c_uint;
pub const G_OPTION_FLAG_NOALIAS: GOptionFlags = 64;
pub const G_OPTION_FLAG_OPTIONAL_ARG: GOptionFlags = 32;
pub const G_OPTION_FLAG_FILENAME: GOptionFlags = 16;
pub const G_OPTION_FLAG_NO_ARG: GOptionFlags = 8;
pub const G_OPTION_FLAG_REVERSE: GOptionFlags = 4;
pub const G_OPTION_FLAG_IN_MAIN: GOptionFlags = 2;
pub const G_OPTION_FLAG_HIDDEN: GOptionFlags = 1;
pub const G_OPTION_FLAG_NONE: GOptionFlags = 0;
pub type GOptionArgFunc = Option<
    unsafe extern "C" fn(*const gchar, *const gchar, gpointer, *mut *mut GError) -> gboolean,
>;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_OPTION_ERROR_FAILED: C2RustUnnamed = 2;
pub const G_OPTION_ERROR_BAD_VALUE: C2RustUnnamed = 1;
pub const G_OPTION_ERROR_UNKNOWN_OPTION: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PendingNull {
    pub ptr: *mut *mut gchar,
    pub value: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Change {
    pub arg_type: GOptionArg,
    pub arg_data: gpointer,
    pub prev: C2RustUnnamed_2,
    pub allocated: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub str_0: *mut gchar,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub len: gint,
    pub data: *mut *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub bool_0: gboolean,
    pub integer: gint,
    pub str_0: *mut gchar,
    pub array: *mut *mut gchar,
    pub dbl: gdouble,
    pub int64: gint64,
}
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
pub const G_ASCII_PRINT: C2RustUnnamed_3 = 64;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GHashTable = _GHashTable;
pub type gunichar = guint32;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed_3 = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed_3 = 512;
pub const G_ASCII_SPACE: C2RustUnnamed_3 = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed_3 = 128;
pub const G_ASCII_LOWER: C2RustUnnamed_3 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_3 = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed_3 = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed_3 = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed_3 = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed_3 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_option_context_set_summary\0" as *const u8 as *const ::core::ffi::c_char;
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
unsafe extern "C" fn safe_c2rust__g_unichar_get_width(mut c: gunichar) -> ::core::ffi::c_int {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if g_unichar_iszerowidth(c) != 0 {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if g_unichar_iswide(c) != 0 {
        return 2 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust__g_utf8_strwidth(mut p: *const gchar) -> glong {
    let mut len: glong = 0 as glong;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !p.is_null() {
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
            b"p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as glong;
    }
    while *p != 0 {
        len += safe_c2rust__g_unichar_get_width(g_utf8_get_char(p)) as glong;
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(
            b"g-option-context-error-quark\0" as *const u8 as *const gchar,
        );
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_new(
    mut parameter_string: *const gchar,
) -> *mut GOptionContext {
    let mut context: *mut GOptionContext = ::core::ptr::null_mut::<GOptionContext>();
    context = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GOptionContext>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GOptionContext;
    if !parameter_string.is_null() && *parameter_string as ::core::ffi::c_int == '\0' as i32 {
        parameter_string = ::core::ptr::null::<gchar>();
    }
    (*context).parameter_string =
        safe_c2rust_g_strdup_inline(parameter_string as *const ::core::ffi::c_char) as *mut gchar;
    (*context).set_strict_posix(FALSE as guint as guint);
    (*context).set_help_enabled(TRUE as guint as guint);
    (*context).set_ignore_unknown(FALSE as guint as guint);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_free(mut context: *mut GOptionContext) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_list_free_full(
        (*context).groups,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GOptionGroup) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_option_group_unref as unsafe extern "C" fn(*mut GOptionGroup) -> (),
        )),
    );
    if !(*context).main_group.is_null() {
        safe_c2rust_g_option_group_unref((*context).main_group);
    }
    safe_c2rust_free_changes_list(context, FALSE);
    safe_c2rust_free_pending_nulls(context, FALSE);
    g_free((*context).parameter_string as gpointer);
    g_free((*context).summary as gpointer);
    g_free((*context).description as gpointer);
    if (*context).translate_notify.is_some() {
        Some(
            (*context)
                .translate_notify
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*context).translate_data);
    }
    g_free(context as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_help_enabled(
    mut context: *mut GOptionContext,
    mut help_enabled: gboolean,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*context).set_help_enabled(help_enabled as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_help_enabled(
    mut context: *mut GOptionContext,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*context).help_enabled() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_ignore_unknown_options(
    mut context: *mut GOptionContext,
    mut ignore_unknown: gboolean,
) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*context).set_ignore_unknown(ignore_unknown as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_ignore_unknown_options(
    mut context: *mut GOptionContext,
) -> gboolean {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*context).ignore_unknown() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_strict_posix(
    mut context: *mut GOptionContext,
    mut strict_posix: gboolean,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*context).set_strict_posix(strict_posix as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_strict_posix(
    mut context: *mut GOptionContext,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*context).strict_posix() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_add_group(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !(*group).name.is_null() {
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
            b"group->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !(*group).description.is_null() {
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
            b"group->description != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !(*group).help_description.is_null() {
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
            b"group->help_description != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    list = (*context).groups;
    while !list.is_null() {
        let mut g: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
        if (*group).name.is_null() && (*g).name.is_null()
            || !(*group).name.is_null()
                && !(*g).name.is_null()
                && strcmp((*group).name, (*g).name) == 0 as ::core::ffi::c_int
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"A group named \"%s\" is already part of this GOptionContext\0" as *const u8
                    as *const gchar,
                (*group).name,
            );
        }
        list = (*list).next;
    }
    (*context).groups = g_list_append((*context).groups, group as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_main_group(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*context).main_group.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"This GOptionContext already has a main group\0" as *const u8 as *const gchar,
        );
        return;
    }
    (*context).main_group = group;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_main_group(
    mut context: *mut GOptionContext,
) -> *mut GOptionGroup {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOptionGroup>();
    }
    return (*context).main_group;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_add_main_entries(
    mut context: *mut GOptionContext,
    mut entries: *const GOptionEntry,
    mut translation_domain: *const gchar,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !entries.is_null() {
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
            b"entries != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*context).main_group.is_null() {
        (*context).main_group = safe_c2rust_g_option_group_new(
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            NULL_0,
            None,
        );
    }
    safe_c2rust_g_option_group_add_entries((*context).main_group, entries);
    safe_c2rust_g_option_group_set_translation_domain((*context).main_group, translation_domain);
}
unsafe extern "C" fn safe_c2rust_calculate_max_length(
    mut group: *mut GOptionGroup,
    mut aliases: *mut GHashTable,
) -> gint {
    let mut entry: *mut GOptionEntry = ::core::ptr::null_mut::<GOptionEntry>();
    let mut i: gsize = 0;
    let mut len: gsize = 0;
    let mut max_length: gsize = 0;
    let mut long_name: *const gchar = ::core::ptr::null::<gchar>();
    max_length = 0 as gsize;
    i = 0 as gsize;
    while i < (*group).n_entries {
        entry = (*group).entries.offset(i as isize) as *mut GOptionEntry;
        if !((*entry).flags as ::core::ffi::c_int & G_OPTION_FLAG_HIDDEN as ::core::ffi::c_int != 0)
        {
            long_name = g_hash_table_lookup(aliases, &raw mut (*entry).long_name as gconstpointer)
                as *const gchar;
            if long_name.is_null() {
                long_name = (*entry).long_name;
            }
            len = safe_c2rust__g_utf8_strwidth(long_name) as gsize;
            if (*entry).short_name != 0 {
                len = len.wrapping_add(4 as gsize);
            }
            if !((*entry).arg as ::core::ffi::c_uint
                == G_OPTION_ARG_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*entry).arg as ::core::ffi::c_uint
                    == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*entry).flags as ::core::ffi::c_int
                        & G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                        != 0)
                && !(*entry).arg_description.is_null()
            {
                len = len.wrapping_add(
                    (1 as glong
                        + safe_c2rust__g_utf8_strwidth(
                            (if (*group).translate_func.is_some() {
                                Some((*group).translate_func.expect("non-null function pointer"))
                                    .expect("non-null function pointer")(
                                    (*entry).arg_description,
                                    (*group).translate_data,
                                )
                            } else {
                                (*entry).arg_description
                            }),
                        )) as gsize,
                );
            }
            max_length = if max_length > len { max_length } else { len };
        }
        i = i.wrapping_add(1);
    }
    return max_length as gint;
}
unsafe extern "C" fn safe_c2rust_print_entry(
    mut group: *mut GOptionGroup,
    mut max_length: gint,
    mut entry: *const GOptionEntry,
    mut string: *mut GString,
    mut aliases: *mut GHashTable,
) {
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut long_name: *const gchar = ::core::ptr::null::<gchar>();
    if (*entry).flags as ::core::ffi::c_int & G_OPTION_FLAG_HIDDEN as ::core::ffi::c_int != 0 {
        return;
    }
    if *(*entry).long_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    long_name = g_hash_table_lookup(aliases, &raw const (*entry).long_name as gconstpointer)
        as *const gchar;
    if long_name.is_null() {
        long_name = (*entry).long_name;
    }
    str = g_string_new(::core::ptr::null::<gchar>());
    if (*entry).short_name != 0 {
        g_string_append_printf(
            str,
            b"  -%c, --%s\0" as *const u8 as *const gchar,
            (*entry).short_name as ::core::ffi::c_int,
            long_name,
        );
    } else {
        g_string_append_printf(str, b"  --%s\0" as *const u8 as *const gchar, long_name);
    }
    if !(*entry).arg_description.is_null() {
        g_string_append_printf(
            str,
            b"=%s\0" as *const u8 as *const gchar,
            if (*group).translate_func.is_some() {
                Some((*group).translate_func.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    (*entry).arg_description,
                    (*group).translate_data,
                )
            } else {
                (*entry).arg_description
            },
        );
    }
    g_string_append_printf(
        string,
        b"%s%*s %s\n\0" as *const u8 as *const gchar,
        (*str).str_0,
        ((max_length as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as glong
            - safe_c2rust__g_utf8_strwidth((*str).str_0)) as ::core::ffi::c_int,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        if !(*entry).description.is_null() {
            if (*group).translate_func.is_some() {
                Some((*group).translate_func.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    (*entry).description,
                    (*group).translate_data,
                ) as *const ::core::ffi::c_char
            } else {
                (*entry).description as *const ::core::ffi::c_char
            }
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(str);
        };
    } else {
        g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn safe_c2rust_group_has_visible_entries(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
    mut main_entries: gboolean,
) -> gboolean {
    let mut reject_filter: GOptionFlags = G_OPTION_FLAG_HIDDEN;
    let mut entry: *mut GOptionEntry = ::core::ptr::null_mut::<GOptionEntry>();
    let mut i: gint = 0;
    let mut l: gint = 0;
    let mut main_group: gboolean = (group == (*context).main_group) as ::core::ffi::c_int;
    if main_entries == 0 {
        reject_filter = ::core::mem::transmute::<::core::ffi::c_uint, GOptionFlags>(
            reject_filter as ::core::ffi::c_uint
                | G_OPTION_FLAG_IN_MAIN as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    l = (if !group.is_null() {
        (*group).n_entries
    } else {
        0 as gsize
    }) as gint;
    while i < l {
        entry = (*group).entries.offset(i as isize) as *mut GOptionEntry;
        if !(main_entries != 0
            && main_group == 0
            && (*entry).flags as ::core::ffi::c_int & G_OPTION_FLAG_IN_MAIN as ::core::ffi::c_int
                == 0)
        {
            if !(*(*entry).long_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int)
            {
                if (*entry).flags as ::core::ffi::c_uint & reject_filter as ::core::ffi::c_uint == 0
                {
                    return TRUE;
                }
            }
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_group_list_has_visible_entries(
    mut context: *mut GOptionContext,
    mut group_list: *mut GList,
    mut main_entries: gboolean,
) -> gboolean {
    while !group_list.is_null() {
        if safe_c2rust_group_has_visible_entries(
            context,
            (*group_list).data as *mut GOptionGroup,
            main_entries,
        ) != 0
        {
            return TRUE;
        }
        group_list = (*group_list).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_context_has_h_entry(mut context: *mut GOptionContext) -> gboolean {
    let mut i: gsize = 0;
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if !(*context).main_group.is_null() {
        i = 0 as gsize;
        while i < (*(*context).main_group).n_entries {
            if (*(*(*context).main_group).entries.offset(i as isize)).short_name
                as ::core::ffi::c_int
                == 'h' as i32
            {
                return TRUE;
            }
            i = i.wrapping_add(1);
        }
    }
    list = (*context).groups;
    while !list.is_null() {
        let mut group: *mut GOptionGroup = ::core::ptr::null_mut::<GOptionGroup>();
        group = (*list).data as *mut GOptionGroup;
        i = 0 as gsize;
        while i < (*group).n_entries {
            if (*(*group).entries.offset(i as isize)).short_name as ::core::ffi::c_int == 'h' as i32
            {
                return TRUE;
            }
            i = i.wrapping_add(1);
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_help(
    mut context: *mut GOptionContext,
    mut main_help: gboolean,
    mut group: *mut GOptionGroup,
) -> *mut gchar {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut max_length: gint = 0 as gint;
    let mut len: gint = 0;
    let mut i: gsize = 0;
    let mut entry: *mut GOptionEntry = ::core::ptr::null_mut::<GOptionEntry>();
    let mut shadow_map: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut aliases: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut seen: [gboolean; 256] = [0; 256];
    let mut rest_description: *const gchar = ::core::ptr::null::<gchar>();
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut token: guchar = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    string = g_string_sized_new(1024 as gsize);
    rest_description = ::core::ptr::null::<gchar>();
    if !(*context).main_group.is_null() {
        i = 0 as gsize;
        while i < (*(*context).main_group).n_entries {
            entry = (*(*context).main_group).entries.offset(i as isize) as *mut GOptionEntry;
            if *(*entry).long_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                rest_description = if (*(*context).main_group).translate_func.is_some() {
                    Some(
                        (*(*context).main_group)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*entry).arg_description,
                        (*(*context).main_group).translate_data,
                    )
                } else {
                    (*entry).arg_description
                };
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    g_string_append_printf(
        string,
        b"%s\n  %s\0" as *const u8 as *const gchar,
        glib_gettext(b"Usage:\0" as *const u8 as *const gchar),
        g_get_prgname(),
    );
    if (*context).help_enabled() as ::core::ffi::c_int != 0
        || !(*context).main_group.is_null() && (*(*context).main_group).n_entries > 0 as gsize
        || !(*context).groups.is_null()
    {
        g_string_append_printf(
            string,
            b" %s\0" as *const u8 as *const gchar,
            glib_gettext(b"[OPTION\xE2\x80\xA6]\0" as *const u8 as *const gchar),
        );
    }
    if !rest_description.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b" \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_30
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
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    rest_description as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                string,
                rest_description as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !(*context).parameter_string.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b" \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                string,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).parameter_string,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).parameter_string as *const ::core::ffi::c_char
                };
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                string,
                if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).parameter_string,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).parameter_string as *const ::core::ffi::c_char
                },
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"\n\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                string,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            string,
            b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(*context).summary.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).summary,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).summary as *const ::core::ffi::c_char
                };
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).summary,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).summary as *const ::core::ffi::c_char
                },
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                string,
                b"\n\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    memset(
        &raw mut seen as *mut gboolean as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<gboolean>() as size_t).wrapping_mul(256 as size_t),
    );
    shadow_map = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    aliases = g_hash_table_new_full(
        None,
        None,
        None,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    if !(*context).main_group.is_null() {
        i = 0 as gsize;
        while i < (*(*context).main_group).n_entries {
            entry = (*(*context).main_group).entries.offset(i as isize) as *mut GOptionEntry;
            g_hash_table_insert(
                shadow_map,
                (*entry).long_name as gpointer,
                entry as gpointer,
            );
            if seen[(*entry).short_name as guchar as usize] != 0 {
                (*entry).short_name = 0 as gchar;
            } else {
                seen[(*entry).short_name as guchar as usize] = TRUE as gboolean;
            }
            i = i.wrapping_add(1);
        }
    }
    list = (*context).groups;
    while !list.is_null() {
        let mut g: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
        i = 0 as gsize;
        while i < (*g).n_entries {
            entry = (*g).entries.offset(i as isize) as *mut GOptionEntry;
            if !g_hash_table_lookup(shadow_map, (*entry).long_name as gconstpointer).is_null()
                && (*entry).flags as ::core::ffi::c_int
                    & G_OPTION_FLAG_NOALIAS as ::core::ffi::c_int
                    == 0
            {
                g_hash_table_insert(
                    aliases,
                    &raw mut (*entry).long_name as gpointer,
                    g_strdup_printf(
                        b"%s-%s\0" as *const u8 as *const gchar,
                        (*g).name,
                        (*entry).long_name,
                    ) as gpointer,
                );
            } else {
                g_hash_table_insert(
                    shadow_map,
                    (*entry).long_name as gpointer,
                    entry as gpointer,
                );
            }
            if seen[(*entry).short_name as guchar as usize] != 0
                && (*entry).flags as ::core::ffi::c_int
                    & G_OPTION_FLAG_NOALIAS as ::core::ffi::c_int
                    == 0
            {
                (*entry).short_name = 0 as gchar;
            } else {
                seen[(*entry).short_name as guchar as usize] = TRUE as gboolean;
            }
            i = i.wrapping_add(1);
        }
        list = (*list).next;
    }
    g_hash_table_destroy(shadow_map);
    list = (*context).groups;
    if (*context).help_enabled() != 0 {
        max_length =
            safe_c2rust__g_utf8_strwidth(b"-?, --help\0" as *const u8 as *const gchar) as gint;
        if !list.is_null() {
            len =
                safe_c2rust__g_utf8_strwidth(b"--help-all\0" as *const u8 as *const gchar) as gint;
            max_length = if max_length > len { max_length } else { len };
        }
    }
    if !(*context).main_group.is_null() {
        len = safe_c2rust_calculate_max_length((*context).main_group, aliases);
        max_length = if max_length > len { max_length } else { len };
    }
    while !list.is_null() {
        let mut g_0: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
        if (*context).help_enabled() != 0 {
            len = (safe_c2rust__g_utf8_strwidth(b"--help-\0" as *const u8 as *const gchar)
                + safe_c2rust__g_utf8_strwidth((*g_0).name)) as gint;
            max_length = if max_length > len { max_length } else { len };
        }
        len = safe_c2rust_calculate_max_length(g_0, aliases);
        max_length = if max_length > len { max_length } else { len };
        list = (*list).next;
    }
    max_length += 4 as ::core::ffi::c_int;
    if group.is_null() && (*context).help_enabled() as ::core::ffi::c_int != 0 {
        list = (*context).groups;
        token = (if safe_c2rust_context_has_h_entry(context) != 0 {
            '?' as i32
        } else {
            'h' as i32
        }) as guchar;
        g_string_append_printf(
            string,
            b"%s\n  -%c, --%-*s %s\n\0" as *const u8 as *const gchar,
            glib_gettext(b"Help Options:\0" as *const u8 as *const gchar),
            token as ::core::ffi::c_int,
            max_length as ::core::ffi::c_int - 4 as ::core::ffi::c_int,
            b"help\0" as *const u8 as *const ::core::ffi::c_char,
            glib_gettext(b"Show help options\0" as *const u8 as *const gchar),
        );
        if !list.is_null() {
            g_string_append_printf(
                string,
                b"  --%-*s %s\n\0" as *const u8 as *const gchar,
                max_length,
                b"help-all\0" as *const u8 as *const ::core::ffi::c_char,
                glib_gettext(b"Show all help options\0" as *const u8 as *const gchar),
            );
        }
        while !list.is_null() {
            let mut g_1: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
            if safe_c2rust_group_has_visible_entries(context, g_1, FALSE) != 0 {
                g_string_append_printf(
                    string,
                    b"  --help-%-*s %s\n\0" as *const u8 as *const gchar,
                    max_length as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
                    (*g_1).name,
                    if (*g_1).translate_func.is_some() {
                        Some((*g_1).translate_func.expect("non-null function pointer"))
                            .expect("non-null function pointer")(
                            (*g_1).help_description,
                            (*g_1).translate_data,
                        )
                    } else {
                        (*g_1).help_description as *const gchar
                    },
                );
            }
            list = (*list).next;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
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
                string,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !group.is_null() {
        if safe_c2rust_group_has_visible_entries(context, group, FALSE) != 0 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = if (*group).translate_func.is_some() {
                        Some((*group).translate_func.expect("non-null function pointer"))
                            .expect("non-null function pointer")(
                            (*group).description,
                            (*group).translate_data,
                        ) as *const ::core::ffi::c_char
                    } else {
                        (*group).description as *const ::core::ffi::c_char
                    };
                    safe_c2rust_g_string_append_len_inline(
                        string,
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
                    string,
                    if (*group).translate_func.is_some() {
                        Some((*group).translate_func.expect("non-null function pointer"))
                            .expect("non-null function pointer")(
                            (*group).description,
                            (*group).translate_data,
                        ) as *const ::core::ffi::c_char
                    } else {
                        (*group).description as *const ::core::ffi::c_char
                    },
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
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
                    string,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            i = 0 as gsize;
            while i < (*group).n_entries {
                safe_c2rust_print_entry(
                    group,
                    max_length,
                    (*group).entries.offset(i as isize) as *mut GOptionEntry,
                    string,
                    aliases,
                );
                i = i.wrapping_add(1);
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
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
                    string,
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
    } else if main_help == 0 {
        list = (*context).groups;
        while !list.is_null() {
            let mut g_2: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
            if safe_c2rust_group_has_visible_entries(context, g_2, FALSE) != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = (*g_2).description;
                        safe_c2rust_g_string_append_len_inline(
                            string,
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
                        string,
                        (*g_2).description,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_42 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_42 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_42
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
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                i = 0 as gsize;
                while i < (*g_2).n_entries {
                    if (*(*g_2).entries.offset(i as isize)).flags as ::core::ffi::c_int
                        & G_OPTION_FLAG_IN_MAIN as ::core::ffi::c_int
                        == 0
                    {
                        safe_c2rust_print_entry(
                            g_2,
                            max_length,
                            (*g_2).entries.offset(i as isize) as *mut GOptionEntry,
                            string,
                            aliases,
                        );
                    }
                    i = i.wrapping_add(1);
                }
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_43 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_43 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_43
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
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            list = (*list).next;
        }
    }
    if (main_help != 0 || group.is_null())
        && (safe_c2rust_group_has_visible_entries(context, (*context).main_group, TRUE) != 0
            || safe_c2rust_group_list_has_visible_entries(context, (*context).groups, TRUE) != 0)
    {
        list = (*context).groups;
        if (*context).help_enabled() as ::core::ffi::c_int != 0 || !list.is_null() {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        glib_gettext(b"Application Options:\0" as *const u8 as *const gchar)
                            as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_44 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_44 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_44
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
                    glib_gettext(b"Application Options:\0" as *const u8 as *const gchar)
                        as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        } else {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        glib_gettext(b"Options:\0" as *const u8 as *const gchar)
                            as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_45 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_45 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_45
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
                    glib_gettext(b"Options:\0" as *const u8 as *const gchar)
                        as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_46
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
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if !(*context).main_group.is_null() {
            i = 0 as gsize;
            while i < (*(*context).main_group).n_entries {
                safe_c2rust_print_entry(
                    (*context).main_group,
                    max_length,
                    (*(*context).main_group).entries.offset(i as isize) as *mut GOptionEntry,
                    string,
                    aliases,
                );
                i = i.wrapping_add(1);
            }
        }
        while !list.is_null() {
            let mut g_3: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
            i = 0 as gsize;
            while i < (*g_3).n_entries {
                if (*(*g_3).entries.offset(i as isize)).flags as ::core::ffi::c_int
                    & G_OPTION_FLAG_IN_MAIN as ::core::ffi::c_int
                    != 0
                {
                    safe_c2rust_print_entry(
                        g_3,
                        max_length,
                        (*g_3).entries.offset(i as isize) as *mut GOptionEntry,
                        string,
                        aliases,
                    );
                }
                i = i.wrapping_add(1);
            }
            list = (*list).next;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
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
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !(*context).description.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).description,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).description as *const ::core::ffi::c_char
                };
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_48
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
                if (*context).translate_func.is_some() {
                    Some(
                        (*context)
                            .translate_func
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        (*context).description,
                        (*context).translate_data,
                    ) as *const ::core::ffi::c_char
                } else {
                    (*context).description as *const ::core::ffi::c_char
                },
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_49
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
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    g_hash_table_destroy(aliases);
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
unsafe extern "C" fn safe_c2rust_print_help(
    mut context: *mut GOptionContext,
    mut main_help: gboolean,
    mut group: *mut GOptionGroup,
) -> ! {
    let mut help: *mut gchar = ::core::ptr::null_mut::<gchar>();
    help = safe_c2rust_g_option_context_get_help(context, main_help, group);
    g_print(b"%s\0" as *const u8 as *const gchar, help);
    g_free(help as gpointer);
    exit(0 as ::core::ffi::c_int);
}
unsafe extern "C" fn safe_c2rust_parse_int(
    mut arg_name: *const gchar,
    mut arg: *const gchar,
    mut result: *mut gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: glong = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    tmp = strtol(
        arg as *const ::core::ffi::c_char,
        &raw mut end,
        0 as ::core::ffi::c_int,
    ) as glong;
    if *arg as ::core::ffi::c_int == '\0' as i32 || *end as ::core::ffi::c_int != '\0' as i32 {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot parse integer value \xE2\x80\x9C%s\xE2\x80\x9D for %s\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    *result = tmp as gint;
    if *result as glong != tmp || *__errno_location() == ERANGE {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Integer value \xE2\x80\x9C%s\xE2\x80\x9D for %s out of range\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_double(
    mut arg_name: *const gchar,
    mut arg: *const gchar,
    mut result: *mut gdouble,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: gdouble = 0.;
    *__errno_location() = 0 as ::core::ffi::c_int;
    tmp = g_strtod(arg, &raw mut end);
    if *arg as ::core::ffi::c_int == '\0' as i32 || *end as ::core::ffi::c_int != '\0' as i32 {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot parse double value \xE2\x80\x9C%s\xE2\x80\x9D for %s\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    if *__errno_location() == ERANGE {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Double value \xE2\x80\x9C%s\xE2\x80\x9D for %s out of range\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    *result = tmp;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_int64(
    mut arg_name: *const gchar,
    mut arg: *const gchar,
    mut result: *mut gint64,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: gint64 = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    tmp = g_ascii_strtoll(arg, &raw mut end, 0 as guint);
    if *arg as ::core::ffi::c_int == '\0' as i32 || *end as ::core::ffi::c_int != '\0' as i32 {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot parse integer value \xE2\x80\x9C%s\xE2\x80\x9D for %s\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    if *__errno_location() == ERANGE {
        g_set_error(
            error,
            safe_c2rust_g_option_error_quark(),
            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Integer value \xE2\x80\x9C%s\xE2\x80\x9D for %s out of range\0" as *const u8
                    as *const gchar,
            ),
            arg,
            arg_name,
        );
        return FALSE;
    }
    *result = tmp;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_change(
    mut context: *mut GOptionContext,
    mut arg_type: GOptionArg,
    mut arg_data: gpointer,
) -> *mut Change {
    let mut current_block: u64;
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut change: *mut Change = ::core::ptr::null_mut::<Change>();
    list = (*context).changes;
    loop {
        if list.is_null() {
            current_block = 7502529970979898288;
            break;
        }
        change = (*list).data as *mut Change;
        if (*change).arg_data == arg_data {
            current_block = 14210603680981652394;
            break;
        }
        list = (*list).next;
    }
    match current_block {
        7502529970979898288 => {
            change = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<Change>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut Change;
            (*change).arg_type = arg_type;
            (*change).arg_data = arg_data;
            (*context).changes = g_list_prepend((*context).changes, change as gpointer);
        }
        _ => {}
    }
    return change;
}
unsafe extern "C" fn safe_c2rust_add_pending_null(
    mut context: *mut GOptionContext,
    mut ptr: *mut *mut gchar,
    mut value: *mut gchar,
) {
    let mut n: *mut PendingNull = ::core::ptr::null_mut::<PendingNull>();
    n = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<PendingNull>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut PendingNull;
    (*n).ptr = ptr;
    (*n).value = value;
    (*context).pending_nulls = g_list_prepend((*context).pending_nulls, n as gpointer);
}
unsafe extern "C" fn safe_c2rust_parse_arg(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
    mut entry: *mut GOptionEntry,
    mut value: *const gchar,
    mut option_name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut change: *mut Change = ::core::ptr::null_mut::<Change>();
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !value.is_null()
            || (*entry).arg as ::core::ffi::c_uint
                == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*entry).flags as ::core::ffi::c_int
                    & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                    != 0
            || ((*entry).arg as ::core::ffi::c_uint
                == G_OPTION_ARG_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*entry).arg as ::core::ffi::c_uint
                    == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*entry).flags as ::core::ffi::c_int
                        & G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                        != 0)
        {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/goption.c\0" as *const u8 as *const ::core::ffi::c_char,
            1109 as ::core::ffi::c_int,
            G_STRFUNC,
            b"value || OPTIONAL_ARG (entry) || NO_ARG (entry)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    match (*entry).arg as ::core::ffi::c_uint {
        0 => {
            safe_c2rust_get_change(context, G_OPTION_ARG_NONE, (*entry).arg_data);
            *((*entry).arg_data as *mut gboolean) =
                ((*entry).flags as ::core::ffi::c_int & G_OPTION_FLAG_REVERSE as ::core::ffi::c_int
                    == 0) as ::core::ffi::c_int as gboolean;
        }
        1 => {
            let mut data: *mut gchar = ::core::ptr::null_mut::<gchar>();
            data = g_locale_to_utf8(
                value,
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                error,
            );
            if data.is_null() {
                return FALSE;
            }
            change = safe_c2rust_get_change(context, G_OPTION_ARG_STRING, (*entry).arg_data);
            if (*change).allocated.str_0.is_null() {
                (*change).prev.str_0 = *((*entry).arg_data as *mut *mut gchar);
            } else {
                g_free((*change).allocated.str_0 as gpointer);
            }
            (*change).allocated.str_0 = data;
            let ref mut fresh7 = *((*entry).arg_data as *mut *mut gchar);
            *fresh7 = data;
        }
        5 => {
            let mut data_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
            data_0 = g_locale_to_utf8(
                value,
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                error,
            );
            if data_0.is_null() {
                return FALSE;
            }
            change = safe_c2rust_get_change(context, G_OPTION_ARG_STRING_ARRAY, (*entry).arg_data);
            if (*change).allocated.array.len == 0 as ::core::ffi::c_int {
                (*change).prev.array = *((*entry).arg_data as *mut *mut *mut gchar);
                (*change).allocated.array.data = ({
                    let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
                    let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                    let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    if __s == 1 as gsize {
                        __p = g_malloc(__n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_malloc(__n.wrapping_mul(__s));
                    } else {
                        __p = g_malloc_n(__n, __s);
                    }
                    __p
                }) as *mut *mut gchar;
            } else {
                (*change).allocated.array.data = ({
                    let mut __n: gsize = ((*change).allocated.array.len as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as gsize;
                    let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                    let mut __p: gpointer = (*change).allocated.array.data as gpointer;
                    if __s == 1 as gsize {
                        __p = g_realloc(__p, __n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_realloc(__p, __n.wrapping_mul(__s));
                    } else {
                        __p = g_realloc_n(__p, __n, __s);
                    }
                    __p
                }) as *mut *mut gchar;
            }
            let ref mut fresh8 = *(*change)
                .allocated
                .array
                .data
                .offset((*change).allocated.array.len as isize);
            *fresh8 = data_0;
            let ref mut fresh9 = *(*change).allocated.array.data.offset(
                ((*change).allocated.array.len as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            );
            *fresh9 = ::core::ptr::null_mut::<gchar>();
            (*change).allocated.array.len += 1;
            let ref mut fresh10 = *((*entry).arg_data as *mut *mut *mut gchar);
            *fresh10 = (*change).allocated.array.data;
        }
        4 => {
            let mut data_1: *mut gchar = ::core::ptr::null_mut::<gchar>();
            data_1 = safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
            change = safe_c2rust_get_change(context, G_OPTION_ARG_FILENAME, (*entry).arg_data);
            if (*change).allocated.str_0.is_null() {
                (*change).prev.str_0 = *((*entry).arg_data as *mut *mut gchar);
            } else {
                g_free((*change).allocated.str_0 as gpointer);
            }
            (*change).allocated.str_0 = data_1;
            let ref mut fresh11 = *((*entry).arg_data as *mut *mut gchar);
            *fresh11 = data_1;
        }
        6 => {
            let mut data_2: *mut gchar = ::core::ptr::null_mut::<gchar>();
            data_2 = safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
            change = safe_c2rust_get_change(context, G_OPTION_ARG_STRING_ARRAY, (*entry).arg_data);
            if (*change).allocated.array.len == 0 as ::core::ffi::c_int {
                (*change).prev.array = *((*entry).arg_data as *mut *mut *mut gchar);
                (*change).allocated.array.data = ({
                    let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
                    let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                    let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    if __s == 1 as gsize {
                        __p = g_malloc(__n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_malloc(__n.wrapping_mul(__s));
                    } else {
                        __p = g_malloc_n(__n, __s);
                    }
                    __p
                }) as *mut *mut gchar;
            } else {
                (*change).allocated.array.data = ({
                    let mut __n: gsize = ((*change).allocated.array.len as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as gsize;
                    let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
                    let mut __p: gpointer = (*change).allocated.array.data as gpointer;
                    if __s == 1 as gsize {
                        __p = g_realloc(__p, __n);
                    } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                        __p = g_realloc(__p, __n.wrapping_mul(__s));
                    } else {
                        __p = g_realloc_n(__p, __n, __s);
                    }
                    __p
                }) as *mut *mut gchar;
            }
            let ref mut fresh12 = *(*change)
                .allocated
                .array
                .data
                .offset((*change).allocated.array.len as isize);
            *fresh12 = data_2;
            let ref mut fresh13 = *(*change).allocated.array.data.offset(
                ((*change).allocated.array.len as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            );
            *fresh13 = ::core::ptr::null_mut::<gchar>();
            (*change).allocated.array.len += 1;
            let ref mut fresh14 = *((*entry).arg_data as *mut *mut *mut gchar);
            *fresh14 = (*change).allocated.array.data;
        }
        2 => {
            let mut data_3: gint = 0;
            if safe_c2rust_parse_int(option_name, value, &raw mut data_3, error) == 0 {
                return FALSE;
            }
            change = safe_c2rust_get_change(context, G_OPTION_ARG_INT, (*entry).arg_data);
            (*change).prev.integer = *((*entry).arg_data as *mut gint);
            *((*entry).arg_data as *mut gint) = data_3;
        }
        3 => {
            let mut data_4: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut retval: gboolean = 0;
            if value.is_null()
                && (*entry).flags as ::core::ffi::c_int
                    & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                    != 0
            {
                data_4 = ::core::ptr::null_mut::<gchar>();
            } else if (*entry).flags as ::core::ffi::c_int
                & G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                != 0
            {
                data_4 = ::core::ptr::null_mut::<gchar>();
            } else if (*entry).flags as ::core::ffi::c_int
                & G_OPTION_FLAG_FILENAME as ::core::ffi::c_int
                != 0
            {
                data_4 =
                    safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
            } else {
                data_4 = g_locale_to_utf8(
                    value,
                    -(1 as ::core::ffi::c_int) as gssize,
                    ::core::ptr::null_mut::<gsize>(),
                    ::core::ptr::null_mut::<gsize>(),
                    error,
                );
            }
            if (*entry).flags as ::core::ffi::c_int
                & (G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                    | G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int)
                == 0
                && data_4.is_null()
            {
                return FALSE;
            }
            retval = Some(
                ::core::mem::transmute::<gpointer, GOptionArgFunc>((*entry).arg_data)
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                option_name, data_4, (*group).user_data, error
            );
            if retval == 0 && !error.is_null() && (*error).is_null() {
                g_set_error(
                    error,
                    safe_c2rust_g_option_error_quark(),
                    G_OPTION_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(b"Error parsing option %s\0" as *const u8 as *const gchar),
                    option_name,
                );
            }
            g_free(data_4 as gpointer);
            return retval;
        }
        7 => {
            let mut data_5: gdouble = 0.;
            if safe_c2rust_parse_double(option_name, value, &raw mut data_5, error) == 0 {
                return FALSE;
            }
            change = safe_c2rust_get_change(context, G_OPTION_ARG_DOUBLE, (*entry).arg_data);
            (*change).prev.dbl = *((*entry).arg_data as *mut gdouble);
            *((*entry).arg_data as *mut gdouble) = data_5;
        }
        8 => {
            let mut data_6: gint64 = 0;
            if safe_c2rust_parse_int64(option_name, value, &raw mut data_6, error) == 0 {
                return FALSE;
            }
            change = safe_c2rust_get_change(context, G_OPTION_ARG_INT64, (*entry).arg_data);
            (*change).prev.int64 = *((*entry).arg_data as *mut gint64);
            *((*entry).arg_data as *mut gint64) = data_6;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/goption.c\0" as *const u8 as *const ::core::ffi::c_char,
                1346 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_short_option(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
    mut idx: gint,
    mut new_idx: *mut gint,
    mut arg: gchar,
    mut argc: *mut gint,
    mut argv: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
    mut parsed: *mut gboolean,
) -> gboolean {
    let mut j: gsize = 0;
    j = 0 as gsize;
    while j < (*group).n_entries {
        if arg as ::core::ffi::c_int
            == (*(*group).entries.offset(j as isize)).short_name as ::core::ffi::c_int
        {
            let mut option_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
            option_name = g_strdup_printf(
                b"-%c\0" as *const u8 as *const gchar,
                (*(*group).entries.offset(j as isize)).short_name as ::core::ffi::c_int,
            );
            if (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                == G_OPTION_ARG_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                    == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                        & G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                        != 0
            {
                value = ::core::ptr::null_mut::<gchar>();
            } else {
                if *new_idx > idx {
                    g_set_error(
                        error,
                        safe_c2rust_g_option_error_quark(),
                        G_OPTION_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(b"Error parsing option %s\0" as *const u8 as *const gchar),
                        option_name,
                    );
                    g_free(option_name as gpointer);
                    return FALSE;
                }
                if idx < *argc - 1 as ::core::ffi::c_int {
                    if (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                        == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                            & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                            != 0
                        && *(*(*argv)
                            .offset((idx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                    {
                        value = ::core::ptr::null_mut::<gchar>();
                    } else {
                        value = *(*argv)
                            .offset((idx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                        safe_c2rust_add_pending_null(
                            context,
                            (*argv).offset(
                                (idx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ) as *mut *mut gchar,
                            ::core::ptr::null_mut::<gchar>(),
                        );
                        *new_idx = (idx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
                    }
                } else if idx >= *argc - 1 as ::core::ffi::c_int
                    && ((*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                        == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                            & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                            != 0)
                {
                    value = ::core::ptr::null_mut::<gchar>();
                } else {
                    g_set_error(
                        error,
                        safe_c2rust_g_option_error_quark(),
                        G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
                        glib_gettext(b"Missing argument for %s\0" as *const u8 as *const gchar),
                        option_name,
                    );
                    g_free(option_name as gpointer);
                    return FALSE;
                }
            }
            if safe_c2rust_parse_arg(
                context,
                group,
                (*group).entries.offset(j as isize) as *mut GOptionEntry,
                value,
                option_name,
                error,
            ) == 0
            {
                g_free(option_name as gpointer);
                return FALSE;
            }
            g_free(option_name as gpointer);
            *parsed = TRUE as gboolean;
        }
        j = j.wrapping_add(1);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_long_option(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
    mut idx: *mut gint,
    mut arg: *mut gchar,
    mut aliased: gboolean,
    mut argc: *mut gint,
    mut argv: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
    mut parsed: *mut gboolean,
) -> gboolean {
    let mut j: gsize = 0;
    j = 0 as gsize;
    while j < (*group).n_entries {
        if *idx >= *argc {
            return TRUE;
        }
        if !(aliased != 0
            && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                & G_OPTION_FLAG_NOALIAS as ::core::ffi::c_int
                != 0)
        {
            if ((*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                == G_OPTION_ARG_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                    == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                        & G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                        != 0)
                && strcmp(
                    arg,
                    (*(*group).entries.offset(j as isize)).long_name as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut option_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut retval: gboolean = 0;
                option_name = g_strconcat(
                    b"--\0" as *const u8 as *const gchar,
                    (*(*group).entries.offset(j as isize)).long_name,
                    NULL_0,
                );
                retval = safe_c2rust_parse_arg(
                    context,
                    group,
                    (*group).entries.offset(j as isize) as *mut GOptionEntry,
                    ::core::ptr::null::<gchar>(),
                    option_name,
                    error,
                );
                g_free(option_name as gpointer);
                safe_c2rust_add_pending_null(
                    context,
                    (*argv).offset(*idx as isize) as *mut *mut gchar,
                    ::core::ptr::null_mut::<gchar>(),
                );
                *parsed = TRUE as gboolean;
                return retval;
            } else {
                let mut len: gint = strlen(
                    (*(*group).entries.offset(j as isize)).long_name as *const ::core::ffi::c_char,
                ) as gint;
                if strncmp(
                    arg,
                    (*(*group).entries.offset(j as isize)).long_name as *const ::core::ffi::c_char,
                    len as size_t,
                ) == 0 as ::core::ffi::c_int
                    && (*arg.offset(len as isize) as ::core::ffi::c_int == '=' as i32
                        || *arg.offset(len as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int)
                {
                    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    let mut option_name_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    safe_c2rust_add_pending_null(
                        context,
                        (*argv).offset(*idx as isize) as *mut *mut gchar,
                        ::core::ptr::null_mut::<gchar>(),
                    );
                    option_name_0 = g_strconcat(
                        b"--\0" as *const u8 as *const gchar,
                        (*(*group).entries.offset(j as isize)).long_name,
                        NULL_0,
                    );
                    if *arg.offset(len as isize) as ::core::ffi::c_int == '=' as i32 {
                        value = arg
                            .offset(len as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                    } else if *idx < *argc - 1 as ::core::ffi::c_int {
                        if !((*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                            == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                                & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                                != 0)
                        {
                            value = *(*argv).offset((*idx + 1 as ::core::ffi::c_int) as isize);
                            safe_c2rust_add_pending_null(
                                context,
                                (*argv).offset((*idx + 1 as ::core::ffi::c_int) as isize)
                                    as *mut *mut gchar,
                                ::core::ptr::null_mut::<gchar>(),
                            );
                            *idx += 1;
                        } else if *(*(*argv).offset((*idx + 1 as ::core::ffi::c_int) as isize))
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                        {
                            let mut retval_0: gboolean = 0;
                            retval_0 = safe_c2rust_parse_arg(
                                context,
                                group,
                                (*group).entries.offset(j as isize) as *mut GOptionEntry,
                                ::core::ptr::null::<gchar>(),
                                option_name_0,
                                error,
                            );
                            *parsed = TRUE as gboolean;
                            g_free(option_name_0 as gpointer);
                            return retval_0;
                        } else {
                            value = *(*argv).offset((*idx + 1 as ::core::ffi::c_int) as isize);
                            safe_c2rust_add_pending_null(
                                context,
                                (*argv).offset((*idx + 1 as ::core::ffi::c_int) as isize)
                                    as *mut *mut gchar,
                                ::core::ptr::null_mut::<gchar>(),
                            );
                            *idx += 1;
                        }
                    } else if *idx >= *argc - 1 as ::core::ffi::c_int
                        && ((*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                            == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*(*group).entries.offset(j as isize)).flags as ::core::ffi::c_int
                                & G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                                != 0)
                    {
                        let mut retval_1: gboolean = 0;
                        retval_1 = safe_c2rust_parse_arg(
                            context,
                            group,
                            (*group).entries.offset(j as isize) as *mut GOptionEntry,
                            ::core::ptr::null::<gchar>(),
                            option_name_0,
                            error,
                        );
                        *parsed = TRUE as gboolean;
                        g_free(option_name_0 as gpointer);
                        return retval_1;
                    } else {
                        g_set_error(
                            error,
                            safe_c2rust_g_option_error_quark(),
                            G_OPTION_ERROR_BAD_VALUE as ::core::ffi::c_int as gint,
                            glib_gettext(b"Missing argument for %s\0" as *const u8 as *const gchar),
                            option_name_0,
                        );
                        g_free(option_name_0 as gpointer);
                        return FALSE;
                    }
                    if safe_c2rust_parse_arg(
                        context,
                        group,
                        (*group).entries.offset(j as isize) as *mut GOptionEntry,
                        value,
                        option_name_0,
                        error,
                    ) == 0
                    {
                        g_free(option_name_0 as gpointer);
                        return FALSE;
                    }
                    g_free(option_name_0 as gpointer);
                    *parsed = TRUE as gboolean;
                }
            }
        }
        j = j.wrapping_add(1);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_remaining_arg(
    mut context: *mut GOptionContext,
    mut group: *mut GOptionGroup,
    mut idx: *mut gint,
    mut argc: *mut gint,
    mut argv: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
    mut parsed: *mut gboolean,
) -> gboolean {
    let mut j: gsize = 0;
    j = 0 as gsize;
    while j < (*group).n_entries {
        if *idx >= *argc {
            return TRUE;
        }
        if *(*(*group).entries.offset(j as isize))
            .long_name
            .offset(0 as ::core::ffi::c_int as isize)
            != 0
        {
            j = j.wrapping_add(1);
        } else {
            if ({
                let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
                if (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                    == G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                        == G_OPTION_ARG_STRING_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*(*group).entries.offset(j as isize)).arg as ::core::ffi::c_uint
                        == G_OPTION_ARG_FILENAME_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint
                {
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
                    b"group->entries[j].arg == G_OPTION_ARG_CALLBACK || group->entries[j].arg == G_OPTION_ARG_STRING_ARRAY || group->entries[j].arg == G_OPTION_ARG_FILENAME_ARRAY\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as gboolean;
            }
            safe_c2rust_add_pending_null(
                context,
                (*argv).offset(*idx as isize) as *mut *mut gchar,
                ::core::ptr::null_mut::<gchar>(),
            );
            if safe_c2rust_parse_arg(
                context,
                group,
                (*group).entries.offset(j as isize) as *mut GOptionEntry,
                *(*argv).offset(*idx as isize),
                b"\0" as *const u8 as *const gchar,
                error,
            ) == 0
            {
                return FALSE;
            }
            *parsed = TRUE as gboolean;
            return TRUE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_free_changes_list(
    mut context: *mut GOptionContext,
    mut revert: gboolean,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    list = (*context).changes;
    while !list.is_null() {
        let mut change: *mut Change = (*list).data as *mut Change;
        if revert != 0 {
            match (*change).arg_type as ::core::ffi::c_uint {
                0 => {
                    *((*change).arg_data as *mut gboolean) = (*change).prev.bool_0;
                }
                2 => {
                    *((*change).arg_data as *mut gint) = (*change).prev.integer;
                }
                1 | 4 => {
                    g_free((*change).allocated.str_0 as gpointer);
                    let ref mut fresh0 = *((*change).arg_data as *mut *mut gchar);
                    *fresh0 = (*change).prev.str_0;
                }
                5 | 6 => {
                    g_strfreev((*change).allocated.array.data);
                    let ref mut fresh1 = *((*change).arg_data as *mut *mut *mut gchar);
                    *fresh1 = (*change).prev.array;
                }
                7 => {
                    *((*change).arg_data as *mut gdouble) = (*change).prev.dbl;
                }
                8 => {
                    *((*change).arg_data as *mut gint64) = (*change).prev.int64;
                }
                _ => {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/goption.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1610 as ::core::ffi::c_int,
                        G_STRFUNC,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        g_free(change as gpointer);
        list = (*list).next;
    }
    g_list_free((*context).changes);
    (*context).changes = ::core::ptr::null_mut::<GList>();
}
unsafe extern "C" fn safe_c2rust_free_pending_nulls(
    mut context: *mut GOptionContext,
    mut perform_nulls: gboolean,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    list = (*context).pending_nulls;
    while !list.is_null() {
        let mut n: *mut PendingNull = (*list).data as *mut PendingNull;
        if perform_nulls != 0 {
            if !(*n).value.is_null() {
                **(*n).ptr.offset(0 as ::core::ffi::c_int as isize) = '-' as i32 as gchar;
                strcpy(
                    (*(*n).ptr).offset(1 as ::core::ffi::c_int as isize),
                    (*n).value,
                );
            } else {
                if (*context).strv_mode() != 0 {
                    g_free(*(*n).ptr as gpointer);
                }
                *(*n).ptr = ::core::ptr::null_mut::<gchar>();
            }
        }
        g_free((*n).value as gpointer);
        g_free(n as gpointer);
        list = (*list).next;
    }
    g_list_free((*context).pending_nulls);
    (*context).pending_nulls = ::core::ptr::null_mut::<GList>();
}
unsafe extern "C" fn safe_c2rust_platform_get_argv0() -> *mut ::core::ffi::c_char {
    let mut cmdline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut base_arg0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: gsize = 0;
    if g_file_get_contents(
        b"/proc/self/cmdline\0" as *const u8 as *const gchar,
        &raw mut cmdline,
        &raw mut len,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !memchr(
            cmdline as *const ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (len as size_t).wrapping_add(1 as size_t),
        )
        .is_null()
        {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/goption.c\0" as *const u8 as *const ::core::ffi::c_char,
            1678 as ::core::ffi::c_int,
            G_STRFUNC,
            b"memchr (cmdline, 0, len + 1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    base_arg0 = g_path_get_basename(cmdline) as *mut ::core::ffi::c_char;
    g_free(cmdline as gpointer);
    return base_arg0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_parse(
    mut context: *mut GOptionContext,
    mut argc: *mut gint,
    mut argv: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut current_block: u64;
    let mut i: gint = 0;
    let mut j: gint = 0;
    let mut k: gint = 0;
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_get_prgname().is_null() {
        let mut prgname: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if !argc.is_null() && !argv.is_null() && *argc != 0 {
            prgname = g_path_get_basename(*(*argv).offset(0 as ::core::ffi::c_int as isize));
        } else {
            prgname = safe_c2rust_platform_get_argv0() as *mut gchar;
        }
        g_set_prgname_once(if !prgname.is_null() {
            prgname as *const gchar
        } else {
            b"<unknown>\0" as *const u8 as *const gchar
        });
        g_free(prgname as gpointer);
    }
    list = (*context).groups;
    loop {
        if list.is_null() {
            current_block = 17833034027772472439;
            break;
        }
        let mut group: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
        if (*group).pre_parse_func.is_some() {
            if Some((*group).pre_parse_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                context, group, (*group).user_data, error
            ) == 0
            {
                current_block = 11825551604755636462;
                break;
            }
        }
        list = (*list).next;
    }
    match current_block {
        17833034027772472439 => {
            if !(*context).main_group.is_null() && (*(*context).main_group).pre_parse_func.is_some()
            {
                if Some(
                    (*(*context).main_group)
                        .pre_parse_func
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    context,
                    (*context).main_group,
                    (*(*context).main_group).user_data,
                    error,
                ) == 0
                {
                    current_block = 11825551604755636462;
                } else {
                    current_block = 13797916685926291137;
                }
            } else {
                current_block = 13797916685926291137;
            }
            match current_block {
                11825551604755636462 => {}
                _ => {
                    if !argc.is_null() && !argv.is_null() {
                        let mut stop_parsing: gboolean = FALSE;
                        let mut has_unknown: gboolean = FALSE;
                        let mut separator_pos: gint = 0 as gint;
                        i = 1 as ::core::ffi::c_int as gint;
                        's_112: loop {
                            if !(i < *argc) {
                                current_block = 5265702136860997526;
                                break;
                            }
                            let mut arg: *mut gchar = ::core::ptr::null_mut::<gchar>();
                            let mut dash: *mut gchar = ::core::ptr::null_mut::<gchar>();
                            let mut parsed: gboolean = FALSE;
                            if *(*(*argv).offset(i as isize))
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '-' as i32
                                && *(*(*argv).offset(i as isize))
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    != '\0' as i32
                                && stop_parsing == 0
                            {
                                if *(*(*argv).offset(i as isize))
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32
                                {
                                    arg = (*(*argv).offset(i as isize))
                                        .offset(2 as ::core::ffi::c_int as isize);
                                    if *arg as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                        separator_pos = i;
                                        stop_parsing = TRUE as gboolean;
                                        current_block = 14576567515993809846;
                                    } else {
                                        if (*context).help_enabled() != 0 {
                                            if strcmp(
                                                arg,
                                                b"help\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                safe_c2rust_print_help(
                                                    context,
                                                    TRUE,
                                                    ::core::ptr::null_mut::<GOptionGroup>(),
                                                );
                                            } else if strcmp(
                                                arg,
                                                b"help-all\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                safe_c2rust_print_help(
                                                    context,
                                                    FALSE,
                                                    ::core::ptr::null_mut::<GOptionGroup>(),
                                                );
                                            } else if strncmp(
                                                arg,
                                                b"help-\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                5 as size_t,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                list = (*context).groups;
                                                while !list.is_null() {
                                                    let mut group_0: *mut GOptionGroup =
                                                        (*list).data as *mut GOptionGroup;
                                                    if strcmp(
                                                        arg.offset(
                                                            5 as ::core::ffi::c_int as isize,
                                                        ),
                                                        (*group_0).name,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        safe_c2rust_print_help(
                                                            context, FALSE, group_0,
                                                        );
                                                    }
                                                    list = (*list).next;
                                                }
                                            }
                                        }
                                        if !(*context).main_group.is_null()
                                            && safe_c2rust_parse_long_option(
                                                context,
                                                (*context).main_group,
                                                &raw mut i,
                                                arg,
                                                FALSE,
                                                argc,
                                                argv,
                                                error,
                                                &raw mut parsed,
                                            ) == 0
                                        {
                                            current_block = 11825551604755636462;
                                            break;
                                        }
                                        if parsed != 0 {
                                            current_block = 14576567515993809846;
                                        } else {
                                            list = (*context).groups;
                                            while !list.is_null() {
                                                let mut group_1: *mut GOptionGroup =
                                                    (*list).data as *mut GOptionGroup;
                                                if safe_c2rust_parse_long_option(
                                                    context,
                                                    group_1,
                                                    &raw mut i,
                                                    arg,
                                                    FALSE,
                                                    argc,
                                                    argv,
                                                    error,
                                                    &raw mut parsed,
                                                ) == 0
                                                {
                                                    current_block = 11825551604755636462;
                                                    break 's_112;
                                                }
                                                if parsed != 0 {
                                                    break;
                                                }
                                                list = (*list).next;
                                            }
                                            if parsed != 0 {
                                                current_block = 14576567515993809846;
                                            } else {
                                                dash = strchr(arg, '-' as i32) as *mut gchar;
                                                if !dash.is_null() && arg < dash {
                                                    list = (*context).groups;
                                                    while !list.is_null() {
                                                        let mut group_2: *mut GOptionGroup =
                                                            (*list).data as *mut GOptionGroup;
                                                        if strncmp(
                                                            (*group_2).name,
                                                            arg,
                                                            dash.offset_from(arg)
                                                                as ::core::ffi::c_long
                                                                as size_t,
                                                        ) == 0 as ::core::ffi::c_int
                                                        {
                                                            if safe_c2rust_parse_long_option(
                                                                context,
                                                                group_2,
                                                                &raw mut i,
                                                                dash.offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                ),
                                                                TRUE,
                                                                argc,
                                                                argv,
                                                                error,
                                                                &raw mut parsed,
                                                            ) == 0
                                                            {
                                                                current_block =
                                                                    11825551604755636462;
                                                                break 's_112;
                                                            }
                                                            if parsed != 0 {
                                                                break;
                                                            }
                                                        }
                                                        list = (*list).next;
                                                    }
                                                }
                                                if (*context).ignore_unknown() != 0 {
                                                    current_block = 14576567515993809846;
                                                } else {
                                                    current_block = 6014157347423944569;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let mut new_i: gint = i;
                                    let mut arg_length: gint = 0;
                                    let mut nulled_out: *mut gboolean =
                                        ::core::ptr::null_mut::<gboolean>();
                                    let mut has_h_entry: gboolean =
                                        safe_c2rust_context_has_h_entry(context);
                                    arg = (*(*argv).offset(i as isize))
                                        .offset(1 as ::core::ffi::c_int as isize);
                                    arg_length = strlen(arg) as gint;
                                    nulled_out = (if (::core::mem::size_of::<gboolean>() as usize)
                                        .wrapping_mul(arg_length as usize)
                                        == 0 as usize
                                    {
                                        NULL_0
                                    } else {
                                        alloca_allocations.push(::std::vec::from_elem(
                                            0,
                                            (::core::mem::size_of::<gboolean>() as usize)
                                                .wrapping_mul(arg_length as usize)
                                                as usize,
                                        ));
                                        memset(
                                            alloca_allocations.last_mut().unwrap().as_mut_ptr().cast(),
                                            0 as ::core::ffi::c_int,
                                            (::core::mem::size_of::<gboolean>() as size_t)
                                                .wrapping_mul(arg_length as size_t),
                                        )
                                    })
                                        as *mut gboolean;
                                    j = 0 as ::core::ffi::c_int as gint;
                                    while j < arg_length {
                                        if (*context).help_enabled() as ::core::ffi::c_int != 0
                                            && (*arg.offset(j as isize) as ::core::ffi::c_int
                                                == '?' as i32
                                                || *arg.offset(j as isize) as ::core::ffi::c_int
                                                    == 'h' as i32
                                                    && has_h_entry == 0)
                                        {
                                            safe_c2rust_print_help(
                                                context,
                                                TRUE,
                                                ::core::ptr::null_mut::<GOptionGroup>(),
                                            );
                                        }
                                        parsed = FALSE as gboolean;
                                        if !(*context).main_group.is_null()
                                            && safe_c2rust_parse_short_option(
                                                context,
                                                (*context).main_group,
                                                i,
                                                &raw mut new_i,
                                                *arg.offset(j as isize),
                                                argc,
                                                argv,
                                                error,
                                                &raw mut parsed,
                                            ) == 0
                                        {
                                            current_block = 11825551604755636462;
                                            break 's_112;
                                        }
                                        if parsed == 0 {
                                            list = (*context).groups;
                                            while !list.is_null() {
                                                let mut group_3: *mut GOptionGroup =
                                                    (*list).data as *mut GOptionGroup;
                                                if safe_c2rust_parse_short_option(
                                                    context,
                                                    group_3,
                                                    i,
                                                    &raw mut new_i,
                                                    *arg.offset(j as isize),
                                                    argc,
                                                    argv,
                                                    error,
                                                    &raw mut parsed,
                                                ) == 0
                                                {
                                                    current_block = 11825551604755636462;
                                                    break 's_112;
                                                }
                                                if parsed != 0 {
                                                    break;
                                                }
                                                list = (*list).next;
                                            }
                                        }
                                        if (*context).ignore_unknown() as ::core::ffi::c_int != 0
                                            && parsed != 0
                                        {
                                            *nulled_out.offset(j as isize) = TRUE as gboolean;
                                        } else if !((*context).ignore_unknown() != 0) {
                                            if parsed == 0 {
                                                break;
                                            }
                                        }
                                        j += 1;
                                    }
                                    if (*context).ignore_unknown() != 0 {
                                        let mut new_arg: *mut gchar =
                                            ::core::ptr::null_mut::<gchar>();
                                        let mut arg_index: gint = 0 as gint;
                                        j = 0 as ::core::ffi::c_int as gint;
                                        while j < arg_length {
                                            if *nulled_out.offset(j as isize) == 0 {
                                                if new_arg.is_null() {
                                                    new_arg = g_malloc(
                                                        (arg_length as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as gsize,
                                                    )
                                                        as *mut gchar;
                                                }
                                                let fresh4 = arg_index;
                                                arg_index = arg_index + 1;
                                                *new_arg.offset(fresh4 as isize) =
                                                    *arg.offset(j as isize);
                                            }
                                            j += 1;
                                        }
                                        if !new_arg.is_null() {
                                            *new_arg.offset(arg_index as isize) =
                                                '\0' as i32 as gchar;
                                        }
                                        safe_c2rust_add_pending_null(
                                            context,
                                            (*argv).offset(i as isize) as *mut *mut gchar,
                                            new_arg,
                                        );
                                        i = new_i;
                                    } else if parsed != 0 {
                                        safe_c2rust_add_pending_null(
                                            context,
                                            (*argv).offset(i as isize) as *mut *mut gchar,
                                            ::core::ptr::null_mut::<gchar>(),
                                        );
                                        i = new_i;
                                    }
                                    current_block = 6014157347423944569;
                                }
                                match current_block {
                                    14576567515993809846 => {}
                                    _ => {
                                        if parsed == 0 {
                                            has_unknown = TRUE as gboolean;
                                        }
                                        if parsed == 0 && (*context).ignore_unknown() == 0 {
                                            g_set_error(
                                                error,
                                                safe_c2rust_g_option_error_quark(),
                                                G_OPTION_ERROR_UNKNOWN_OPTION as ::core::ffi::c_int
                                                    as gint,
                                                glib_gettext(
                                                    b"Unknown option %s\0" as *const u8
                                                        as *const gchar,
                                                ),
                                                *(*argv).offset(i as isize),
                                            );
                                            current_block = 11825551604755636462;
                                            break;
                                        }
                                    }
                                }
                            } else {
                                if (*context).strict_posix() != 0 {
                                    stop_parsing = TRUE as gboolean;
                                }
                                if !(*context).main_group.is_null()
                                    && safe_c2rust_parse_remaining_arg(
                                        context,
                                        (*context).main_group,
                                        &raw mut i,
                                        argc,
                                        argv,
                                        error,
                                        &raw mut parsed,
                                    ) == 0
                                {
                                    current_block = 11825551604755636462;
                                    break;
                                }
                                if parsed == 0
                                    && (has_unknown != 0
                                        || *(*(*argv).offset(i as isize))
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '-' as i32)
                                {
                                    separator_pos = 0 as ::core::ffi::c_int as gint;
                                }
                            }
                            i += 1;
                        }
                        match current_block {
                            11825551604755636462 => {}
                            _ => {
                                if separator_pos > 0 as ::core::ffi::c_int {
                                    safe_c2rust_add_pending_null(
                                        context,
                                        (*argv).offset(separator_pos as isize) as *mut *mut gchar,
                                        ::core::ptr::null_mut::<gchar>(),
                                    );
                                }
                                current_block = 5846959088466685742;
                            }
                        }
                    } else {
                        current_block = 5846959088466685742;
                    }
                    match current_block {
                        11825551604755636462 => {}
                        _ => {
                            list = (*context).groups;
                            loop {
                                if list.is_null() {
                                    current_block = 1677945370889843322;
                                    break;
                                }
                                let mut group_4: *mut GOptionGroup =
                                    (*list).data as *mut GOptionGroup;
                                if (*group_4).post_parse_func.is_some() {
                                    if Some(
                                        (*group_4)
                                            .post_parse_func
                                            .expect("non-null function pointer"),
                                    )
                                    .expect("non-null function pointer")(
                                        context,
                                        group_4,
                                        (*group_4).user_data,
                                        error,
                                    ) == 0
                                    {
                                        current_block = 11825551604755636462;
                                        break;
                                    }
                                }
                                list = (*list).next;
                            }
                            match current_block {
                                11825551604755636462 => {}
                                _ => {
                                    if !(*context).main_group.is_null()
                                        && (*(*context).main_group).post_parse_func.is_some()
                                    {
                                        if Some(
                                            (*(*context).main_group)
                                                .post_parse_func
                                                .expect("non-null function pointer"),
                                        )
                                        .expect("non-null function pointer")(
                                            context,
                                            (*context).main_group,
                                            (*(*context).main_group).user_data,
                                            error,
                                        ) == 0
                                        {
                                            current_block = 11825551604755636462;
                                        } else {
                                            current_block = 18104233774012731761;
                                        }
                                    } else {
                                        current_block = 18104233774012731761;
                                    }
                                    match current_block {
                                        11825551604755636462 => {}
                                        _ => {
                                            if !argc.is_null() && !argv.is_null() {
                                                safe_c2rust_free_pending_nulls(context, TRUE);
                                                i = 1 as ::core::ffi::c_int as gint;
                                                while i < *argc {
                                                    k = i;
                                                    while k < *argc {
                                                        if !(*(*argv).offset(k as isize)).is_null()
                                                        {
                                                            break;
                                                        }
                                                        k += 1;
                                                    }
                                                    if k > i {
                                                        k -= i;
                                                        j = i + k;
                                                        while j < *argc {
                                                            let ref mut fresh5 =
                                                                *(*argv).offset((j - k) as isize);
                                                            *fresh5 = *(*argv).offset(j as isize);
                                                            let ref mut fresh6 =
                                                                *(*argv).offset(j as isize);
                                                            *fresh6 =
                                                                ::core::ptr::null_mut::<gchar>();
                                                            j += 1;
                                                        }
                                                        *argc -= k;
                                                    }
                                                    i += 1;
                                                }
                                            }
                                            return TRUE;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    list = (*context).groups;
    while !list.is_null() {
        let mut group_5: *mut GOptionGroup = (*list).data as *mut GOptionGroup;
        if (*group_5).error_func.is_some() {
            Some((*group_5).error_func.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                context, group_5, (*group_5).user_data, error
            );
        }
        list = (*list).next;
    }
    if !(*context).main_group.is_null() && (*(*context).main_group).error_func.is_some() {
        Some(
            (*(*context).main_group)
                .error_func
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            context,
            (*context).main_group,
            (*(*context).main_group).user_data,
            error,
        );
    }
    safe_c2rust_free_changes_list(context, TRUE);
    safe_c2rust_free_pending_nulls(context, FALSE);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_new(
    mut name: *const gchar,
    mut description: *const gchar,
    mut help_description: *const gchar,
    mut user_data: gpointer,
    mut destroy: GDestroyNotify,
) -> *mut GOptionGroup {
    let mut group: *mut GOptionGroup = ::core::ptr::null_mut::<GOptionGroup>();
    group = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GOptionGroup>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GOptionGroup;
    (*group).ref_count = 1 as ::core::ffi::c_int as gint;
    (*group).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*group).description =
        safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
    (*group).help_description =
        safe_c2rust_g_strdup_inline(help_description as *const ::core::ffi::c_char) as *mut gchar;
    (*group).user_data = user_data;
    (*group).destroy_notify = destroy;
    return group;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_free(mut group: *mut GOptionGroup) {
    safe_c2rust_g_option_group_unref(group);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_ref(
    mut group: *mut GOptionGroup,
) -> *mut GOptionGroup {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !group.is_null() {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GOptionGroup>();
    }
    (*group).ref_count += 1;
    return group;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_unref(mut group: *mut GOptionGroup) {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !group.is_null() {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*group).ref_count -= 1;
    if (*group).ref_count == 0 as ::core::ffi::c_int {
        g_free((*group).name as gpointer);
        g_free((*group).description as gpointer);
        g_free((*group).help_description as gpointer);
        g_free((*group).entries as gpointer);
        if (*group).destroy_notify.is_some() {
            Some((*group).destroy_notify.expect("non-null function pointer"))
                .expect("non-null function pointer")((*group).user_data);
        }
        if (*group).translate_notify.is_some() {
            Some(
                (*group)
                    .translate_notify
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")((*group).translate_data);
        }
        g_free(group as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_add_entries(
    mut group: *mut GOptionGroup,
    mut entries: *const GOptionEntry,
) {
    let mut i: gsize = 0;
    let mut n_entries: gsize = 0;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !entries.is_null() {
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
            b"entries != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    n_entries = 0 as gsize;
    while !(*entries.offset(n_entries as isize)).long_name.is_null() {
        n_entries = n_entries.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if n_entries
            <= (9223372036854775807 as ::core::ffi::c_long as gsize)
                .wrapping_mul(2 as gsize)
                .wrapping_add(1 as gsize)
                .wrapping_sub((*group).n_entries)
        {
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
            b"n_entries <= G_MAXSIZE - group->n_entries\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*group).entries = ({
        let mut __n: gsize = (*group).n_entries.wrapping_add(n_entries);
        let mut __s: gsize = ::core::mem::size_of::<GOptionEntry>() as gsize;
        let mut __p: gpointer = (*group).entries as gpointer;
        if __s == 1 as gsize {
            __p = g_realloc(__p, __n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_realloc(__p, __n.wrapping_mul(__s));
        } else {
            __p = g_realloc_n(__p, __n, __s);
        }
        __p
    }) as *mut GOptionEntry;
    if n_entries != 0 as gsize {
        memcpy(
            (*group).entries.offset((*group).n_entries as isize) as *mut ::core::ffi::c_void,
            entries as *const ::core::ffi::c_void,
            (::core::mem::size_of::<GOptionEntry>() as size_t).wrapping_mul(n_entries as size_t),
        );
    }
    i = (*group).n_entries;
    while i < (*group).n_entries.wrapping_add(n_entries) {
        let mut c: gchar = (*(*group).entries.offset(i as isize)).short_name;
        if c as ::core::ffi::c_int == '-' as i32
            || c as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && !(*safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
                    & G_ASCII_PRINT as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/goption.c:2275: ignoring invalid short option '%c' (%d) in entry %s:%s\0"
                    as *const u8 as *const gchar,
                c as ::core::ffi::c_int,
                c as ::core::ffi::c_int,
                (*group).name,
                (*(*group).entries.offset(i as isize)).long_name,
            );
            (*(*group).entries.offset(i as isize)).short_name = '\0' as i32 as gchar;
        }
        if (*(*group).entries.offset(i as isize)).arg as ::core::ffi::c_uint
            != G_OPTION_ARG_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(*group).entries.offset(i as isize)).flags as ::core::ffi::c_int
                & G_OPTION_FLAG_REVERSE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/goption.c:2283: ignoring reverse flag on option of arg-type %d in entry %s:%s\0"
                    as *const u8 as *const gchar,
                (*(*group).entries.offset(i as isize)).arg as ::core::ffi::c_uint,
                (*group).name,
                (*(*group).entries.offset(i as isize)).long_name,
            );
            let ref mut fresh2 = (*(*group).entries.offset(i as isize)).flags;
            *fresh2 &= !(G_OPTION_FLAG_REVERSE as ::core::ffi::c_int);
        }
        if (*(*group).entries.offset(i as isize)).arg as ::core::ffi::c_uint
            != G_OPTION_ARG_CALLBACK as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(*group).entries.offset(i as isize)).flags as ::core::ffi::c_int
                & (G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                    | G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                    | G_OPTION_FLAG_FILENAME as ::core::ffi::c_int)
                != 0 as ::core::ffi::c_int
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"../original/glib/goption.c:2292: ignoring no-arg, optional-arg or filename flags (%d) on option of arg-type %d in entry %s:%s\0"
                    as *const u8 as *const gchar,
                (*(*group).entries.offset(i as isize)).flags,
                (*(*group).entries.offset(i as isize)).arg as ::core::ffi::c_uint,
                (*group).name,
                (*(*group).entries.offset(i as isize)).long_name,
            );
            let ref mut fresh3 = (*(*group).entries.offset(i as isize)).flags;
            *fresh3 &= !(G_OPTION_FLAG_NO_ARG as ::core::ffi::c_int
                | G_OPTION_FLAG_OPTIONAL_ARG as ::core::ffi::c_int
                | G_OPTION_FLAG_FILENAME as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    (*group).n_entries = (*group).n_entries.wrapping_add(n_entries);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_set_parse_hooks(
    mut group: *mut GOptionGroup,
    mut pre_parse_func: GOptionParseFunc,
    mut post_parse_func: GOptionParseFunc,
) {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*group).pre_parse_func = pre_parse_func;
    (*group).post_parse_func = post_parse_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_set_error_hook(
    mut group: *mut GOptionGroup,
    mut error_func: GOptionErrorFunc,
) {
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*group).error_func = error_func;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_set_translate_func(
    mut group: *mut GOptionGroup,
    mut func: GTranslateFunc,
    mut data: gpointer,
    mut destroy_notify: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*group).translate_notify.is_some() {
        (*group)
            .translate_notify
            .expect("non-null function pointer")((*group).translate_data);
    }
    (*group).translate_func = func;
    (*group).translate_data = data;
    (*group).translate_notify = destroy_notify;
}
unsafe extern "C" fn safe_c2rust_dgettext_swapped(
    mut msgid: *const gchar,
    mut domainname: *const gchar,
) -> *const gchar {
    return g_dgettext(domainname, msgid);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_group_set_translation_domain(
    mut group: *mut GOptionGroup,
    mut domain: *const gchar,
) {
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_option_group_set_translate_func(
        group,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const gchar, *const gchar) -> *const gchar>,
            GTranslateFunc,
        >(Some(
            safe_c2rust_dgettext_swapped
                as unsafe extern "C" fn(*const gchar, *const gchar) -> *const gchar,
        )),
        safe_c2rust_g_strdup_inline(domain as *const ::core::ffi::c_char) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_translate_func(
    mut context: *mut GOptionContext,
    mut func: GTranslateFunc,
    mut data: gpointer,
    mut destroy_notify: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*context).translate_notify.is_some() {
        (*context)
            .translate_notify
            .expect("non-null function pointer")((*context).translate_data);
    }
    (*context).translate_func = func;
    (*context).translate_data = data;
    (*context).translate_notify = destroy_notify;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_translation_domain(
    mut context: *mut GOptionContext,
    mut domain: *const gchar,
) {
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_option_context_set_translate_func(
        context,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const gchar, *const gchar) -> *const gchar>,
            GTranslateFunc,
        >(Some(
            safe_c2rust_dgettext_swapped
                as unsafe extern "C" fn(*const gchar, *const gchar) -> *const gchar,
        )),
        safe_c2rust_g_strdup_inline(domain as *const ::core::ffi::c_char) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_summary(
    mut context: *mut GOptionContext,
    mut summary: *const gchar,
) {
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*context).summary as gpointer);
    (*context).summary =
        safe_c2rust_g_strdup_inline(summary as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_summary(
    mut context: *mut GOptionContext,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*context).summary;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_set_description(
    mut context: *mut GOptionContext,
    mut description: *const gchar,
) {
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*context).description as gpointer);
    (*context).description =
        safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_get_description(
    mut context: *mut GOptionContext,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*context).description;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_option_context_parse_strv(
    mut context: *mut GOptionContext,
    mut arguments: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut success: gboolean = 0;
    let mut argc: gint = 0;
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    (*context).set_strv_mode(TRUE as guint as guint);
    argc = (if !arguments.is_null() && !(*arguments).is_null() {
        g_strv_length(*arguments)
    } else {
        0 as guint
    }) as gint;
    success = safe_c2rust_g_option_context_parse(context, &raw mut argc, arguments, error);
    (*context).set_strv_mode(FALSE as guint as guint);
    return success;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
