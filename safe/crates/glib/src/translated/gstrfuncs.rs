extern "C" {
    pub type __locale_data;
    pub type _GHashTable;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtoll_l(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
        __loc: locale_t,
    ) -> ::core::ffi::c_longlong;
    fn strtoull_l(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
        __loc: locale_t,
    ) -> ::core::ffi::c_ulonglong;
    fn strtod_l(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __loc: locale_t,
    ) -> ::core::ffi::c_double;
    fn newlocale(
        __category_mask: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn uselocale(__dataset: locale_t) -> locale_t;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
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
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror_r(
        __errnum: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_char,
        __buflen: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strsignal(__sig: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn stpcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> size_t;
    fn strlcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_new_full(
        reserved_size: guint,
        element_free_func: GDestroyNotify,
    ) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
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
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_str_to_ascii(str: *const gchar, from_locale: *const gchar) -> *mut gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_get_console_charset(charset: *mut *const ::core::ffi::c_char) -> gboolean;
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_unichar_isalnum(c: gunichar) -> gboolean;
    fn g_unichar_ismark(c: gunichar) -> gboolean;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_casefold(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_normalize(str: *const gchar, len: gssize, mode: GNormalizeMode) -> *mut gchar;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_vasprintf(
        string: *mut *mut gchar,
        format: *const gchar,
        args: ::core::ffi::VaList,
    ) -> gint;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const ::core::ffi::c_ushort,
    pub __ctype_tolower: *const ::core::ffi::c_int,
    pub __ctype_toupper: *const ::core::ffi::c_int,
    pub __names: [*const ::core::ffi::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed_0 = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed_0 = 512;
pub const G_ASCII_SPACE: C2RustUnnamed_0 = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed_0 = 128;
pub const G_ASCII_PRINT: C2RustUnnamed_0 = 64;
pub const G_ASCII_LOWER: C2RustUnnamed_0 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_0 = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed_0 = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed_0 = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed_0 = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed_0 = 1;
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GHashTable = _GHashTable;
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
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
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
pub type gunichar = guint32;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_NUMBER_PARSER_ERROR_OUT_OF_BOUNDS: C2RustUnnamed_1 = 1;
pub const G_NUMBER_PARSER_ERROR_INVALID: C2RustUnnamed_1 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __LC_COLLATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __LC_MONETARY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_PAPER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const __LC_NAME: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const __LC_ADDRESS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const __LC_TELEPHONE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const __LC_MEASUREMENT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const __LC_IDENTIFICATION: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const LC_CTYPE_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_CTYPE;
pub const LC_NUMERIC_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_NUMERIC;
pub const LC_TIME_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_TIME;
pub const LC_COLLATE_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_COLLATE;
pub const LC_MONETARY_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_MONETARY;
pub const LC_MESSAGES_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_MESSAGES;
pub const LC_PAPER_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_PAPER;
pub const LC_NAME_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_NAME;
pub const LC_ADDRESS_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_ADDRESS;
pub const LC_TELEPHONE_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_TELEPHONE;
pub const LC_MEASUREMENT_MASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << __LC_MEASUREMENT;
pub const LC_IDENTIFICATION_MASK: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << __LC_IDENTIFICATION;
pub const LC_ALL_MASK: ::core::ffi::c_int = LC_CTYPE_MASK
    | LC_NUMERIC_MASK
    | LC_TIME_MASK
    | LC_COLLATE_MASK
    | LC_MONETARY_MASK
    | LC_MESSAGES_MASK
    | LC_PAPER_MASK
    | LC_NAME_MASK
    | LC_ADDRESS_MASK
    | LC_TELEPHONE_MASK
    | LC_MEASUREMENT_MASK
    | LC_IDENTIFICATION_MASK;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_strdelimit\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const G_STR_DELIMITERS: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"_-|> <.\0") };
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
    return safe_c2rust_g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
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
        let fresh39 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh39 as isize) = c;
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
static mut safe_c2rust_ascii_table_data: [guint16; 256] = [
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x104 as ::core::ffi::c_int as guint16,
    0x104 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x104 as ::core::ffi::c_int as guint16,
    0x104 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0x140 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0x459 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x653 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0x253 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x473 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0x73 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0xd0 as ::core::ffi::c_int as guint16,
    0x4 as ::core::ffi::c_int as guint16,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut safe_c2rust_g_ascii_table: *const guint16 =
    unsafe { &raw const safe_c2rust_ascii_table_data as *const guint16 };
unsafe extern "C" fn safe_c2rust_get_C_locale() -> locale_t {
    static mut safe_c2rust_initialized: gsize = FALSE as gsize;
    static mut safe_c2rust_C_locale: locale_t =
        ::core::ptr::null::<__locale_struct>() as *mut __locale_struct;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_C_locale = newlocale(
            LC_ALL_MASK,
            b"C\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<__locale_struct>(),
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialized = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
    return safe_c2rust_C_locale;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdup(mut str: *const gchar) -> *mut gchar {
    let mut new_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !str.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        length = strlen(str as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
        new_str = ({
            let mut __n: gsize = length;
            let mut __s: gsize = ::core::mem::size_of::<::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut ::core::ffi::c_char as *mut gchar;
        memcpy(
            new_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            length as size_t,
        );
    } else {
        new_str = ::core::ptr::null_mut::<gchar>();
    }
    return new_str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memdup(
    mut mem: gconstpointer,
    mut byte_size: guint,
) -> gpointer {
    let mut new_mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !mem.is_null() && byte_size != 0 as guint {
        new_mem = g_malloc(byte_size as gsize);
        memcpy(
            new_mem as *mut ::core::ffi::c_void,
            mem as *const ::core::ffi::c_void,
            byte_size as size_t,
        );
    } else {
        new_mem = NULL_0 as gpointer;
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memdup2(
    mut mem: gconstpointer,
    mut byte_size: gsize,
) -> gpointer {
    let mut new_mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !mem.is_null() && byte_size != 0 as gsize {
        new_mem = g_malloc(byte_size);
        memcpy(
            new_mem as *mut ::core::ffi::c_void,
            mem as *const ::core::ffi::c_void,
            byte_size as size_t,
        );
    } else {
        new_mem = NULL_0 as gpointer;
    }
    return new_mem;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strndup(mut str: *const gchar, mut n: gsize) -> *mut gchar {
    let mut new_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !str.is_null() {
        new_str = ({
            let mut __n: gsize = n.wrapping_add(1 as gsize);
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
        strncpy(
            new_str as *mut ::core::ffi::c_char,
            str as *const ::core::ffi::c_char,
            n as size_t,
        );
        *new_str.offset(n as isize) = '\0' as i32 as gchar;
    } else {
        new_str = ::core::ptr::null_mut::<gchar>();
    }
    return new_str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strnfill(
    mut length: gsize,
    mut fill_char: gchar,
) -> *mut gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    str = ({
        let mut __n: gsize = length.wrapping_add(1 as gsize);
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
    memset(
        str as *mut ::core::ffi::c_void,
        fill_char as guchar as ::core::ffi::c_int,
        length as size_t,
    );
    *str.offset(length as isize) = '\0' as i32 as gchar;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_stpcpy(
    mut dest: *mut gchar,
    mut src: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !dest.is_null() {
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
            b"dest != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !src.is_null() {
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
            b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return stpcpy(
        dest as *mut ::core::ffi::c_char,
        src as *const ::core::ffi::c_char,
    ) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdup_vprintf(
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> *mut gchar {
    let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    g_vasprintf(&raw mut string, format, args.clone());
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdup_printf(
    mut format: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    buffer = safe_c2rust_g_strdup_vprintf(format, args_0.clone());
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strconcat(
    mut string1: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut l: gsize = 0;
    let mut args_0: ::core::ffi::VaList;
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut concat: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if string1.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    l = (1 as size_t).wrapping_add(strlen(string1 as *const ::core::ffi::c_char)) as gsize;
    args_0 = args.clone();
    s = args_0.arg::<*mut gchar>();
    while !s.is_null() {
        l = (l as ::core::ffi::c_ulong).wrapping_add(strlen(s) as ::core::ffi::c_ulong) as gsize
            as gsize;
        s = args_0.arg::<*mut gchar>();
    }
    concat = ({
        let mut __n: gsize = l;
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
    ptr = concat;
    ptr = safe_c2rust_g_stpcpy(ptr, string1);
    args_0 = args.clone();
    s = args_0.arg::<*mut gchar>();
    while !s.is_null() {
        ptr = safe_c2rust_g_stpcpy(ptr, s);
        s = args_0.arg::<*mut gchar>();
    }
    return concat;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strtod(
    mut nptr: *const gchar,
    mut endptr: *mut *mut gchar,
) -> gdouble {
    let mut fail_pos_1: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut fail_pos_2: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut val_1: gdouble = 0.;
    let mut val_2: gdouble = 0 as ::core::ffi::c_int as gdouble;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !nptr.is_null() {
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
            b"nptr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    fail_pos_1 = ::core::ptr::null_mut::<gchar>();
    fail_pos_2 = ::core::ptr::null_mut::<gchar>();
    val_1 = strtod(nptr as *const ::core::ffi::c_char, &raw mut fail_pos_1) as gdouble;
    if !fail_pos_1.is_null()
        && *fail_pos_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        val_2 = safe_c2rust_g_ascii_strtod(nptr, &raw mut fail_pos_2);
    }
    if fail_pos_1.is_null()
        || *fail_pos_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        || fail_pos_1 >= fail_pos_2
    {
        if !endptr.is_null() {
            *endptr = fail_pos_1;
        }
        return val_1;
    } else {
        if !endptr.is_null() {
            *endptr = fail_pos_2;
        }
        return val_2;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strtod(
    mut nptr: *const gchar,
    mut endptr: *mut *mut gchar,
) -> gdouble {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !nptr.is_null() {
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
            b"nptr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    return strtod_l(
        nptr as *const ::core::ffi::c_char,
        endptr as *mut *mut ::core::ffi::c_char,
        safe_c2rust_get_C_locale(),
    ) as gdouble;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_dtostr(
    mut buffer: *mut gchar,
    mut buf_len: gint,
    mut d: gdouble,
) -> *mut gchar {
    return safe_c2rust_g_ascii_formatd(
        buffer,
        buf_len,
        b"%.17g\0" as *const u8 as *const gchar,
        d,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_formatd(
    mut buffer: *mut gchar,
    mut buf_len: gint,
    mut format: *const gchar,
    mut d: gdouble,
) -> *mut gchar {
    let mut old_locale: locale_t = ::core::ptr::null_mut::<__locale_struct>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !buffer.is_null() {
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
            b"buffer != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if *format.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32 {
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
            b"format[0] == '%'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if strpbrk(
            format.offset(1 as ::core::ffi::c_int as isize),
            b"'l%\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
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
            b"strpbrk (format + 1, \"'l%\") == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    old_locale = uselocale(safe_c2rust_get_C_locale());
    snprintf(
        buffer as *mut ::core::ffi::c_char,
        buf_len as size_t,
        format as *const ::core::ffi::c_char,
        d,
    );
    uselocale(old_locale);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strtoull(
    mut nptr: *const gchar,
    mut endptr: *mut *mut gchar,
    mut base: guint,
) -> guint64 {
    return strtoull_l(
        nptr as *const ::core::ffi::c_char,
        endptr as *mut *mut ::core::ffi::c_char,
        base as ::core::ffi::c_int,
        safe_c2rust_get_C_locale(),
    ) as guint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strtoll(
    mut nptr: *const gchar,
    mut endptr: *mut *mut gchar,
    mut base: guint,
) -> gint64 {
    return strtoll_l(
        nptr as *const ::core::ffi::c_char,
        endptr as *mut *mut ::core::ffi::c_char,
        base as ::core::ffi::c_int,
        safe_c2rust_get_C_locale(),
    ) as gint64;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strerror(mut errnum: gint) -> *const gchar {
    static mut safe_c2rust_errors: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    static mut safe_c2rust_g__errors_lock: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    let mut msg: *const gchar = ::core::ptr::null::<gchar>();
    let mut saved_errno: gint = *__errno_location();
    g_mutex_lock(&raw mut safe_c2rust_g__errors_lock);
    if !safe_c2rust_errors.is_null() {
        msg = g_hash_table_lookup(
            safe_c2rust_errors,
            errnum as glong as gpointer as gconstpointer,
        ) as *const gchar;
    } else {
        safe_c2rust_errors = g_hash_table_new(None, None);
        msg = ::core::ptr::null::<gchar>();
    }
    if msg.is_null() {
        let mut buf: [gchar; 1024] = [0; 1024];
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        msg = strerror_r(
            errnum as ::core::ffi::c_int,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 1024]>() as size_t,
        );
        if msg.is_null() {
            g_mutex_unlock(&raw mut safe_c2rust_g__errors_lock);
            *__errno_location() = saved_errno as ::core::ffi::c_int;
            return ::core::ptr::null::<gchar>();
        }
        if g_get_console_charset(::core::ptr::null_mut::<*const ::core::ffi::c_char>()) == 0 {
            msg = g_locale_to_utf8(
                msg,
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                &raw mut error,
            );
            if !error.is_null() {
                g_print(b"%s\n\0" as *const u8 as *const gchar, (*error).message);
                g_error_free(error);
            }
        } else if msg == &raw mut buf as *mut gchar as *const gchar {
            msg = safe_c2rust_g_strdup_inline(&raw mut buf as *mut gchar);
        }
        g_hash_table_insert(
            safe_c2rust_errors,
            errnum as glong as gpointer,
            msg as *mut ::core::ffi::c_char as gpointer,
        );
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__errors_lock);
    *__errno_location() = saved_errno as ::core::ffi::c_int;
    return msg;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strsignal(mut signum: gint) -> *const gchar {
    let mut msg: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tofree: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    tofree = ::core::ptr::null_mut::<gchar>();
    msg = tofree;
    msg = strsignal(signum as ::core::ffi::c_int) as *mut gchar;
    if g_get_console_charset(::core::ptr::null_mut::<*const ::core::ffi::c_char>()) == 0 {
        tofree = g_locale_to_utf8(
            msg,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        msg = tofree;
    }
    if msg.is_null() {
        tofree = safe_c2rust_g_strdup_printf(
            b"unknown signal (%d)\0" as *const u8 as *const gchar,
            signum,
        );
        msg = tofree;
    }
    ret = g_intern_string(msg);
    g_free(tofree as gpointer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strlcpy(
    mut dest: *mut gchar,
    mut src: *const gchar,
    mut dest_size: gsize,
) -> gsize {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !dest.is_null() {
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
            b"dest != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !src.is_null() {
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
            b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return strlcpy(
        dest as *mut ::core::ffi::c_char,
        src as *const ::core::ffi::c_char,
        dest_size as size_t,
    ) as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strlcat(
    mut dest: *mut gchar,
    mut src: *const gchar,
    mut dest_size: gsize,
) -> gsize {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !dest.is_null() {
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
            b"dest != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !src.is_null() {
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
            b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return strlcat(
        dest as *mut ::core::ffi::c_char,
        src as *const ::core::ffi::c_char,
        dest_size as size_t,
    ) as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strdown(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
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
            b"g_ascii_strdown\0" as *const u8 as *const ::core::ffi::c_char,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        result = safe_c2rust_g_strdup_inline(str as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        result = safe_c2rust_g_strndup(str, len as gsize);
    }
    s = result;
    while *s != 0 {
        *s = safe_c2rust_g_ascii_tolower(*s);
        s = s.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strup(
    mut str: *const gchar,
    mut len: gssize,
) -> *mut gchar {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"g_str_has_prefix\0" as *const u8 as *const ::core::ffi::c_char,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if len < 0 as gssize {
        result = safe_c2rust_g_strdup_inline(str as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        result = safe_c2rust_g_strndup(str, len as gsize);
    }
    s = result;
    while *s != 0 {
        *s = safe_c2rust_g_ascii_toupper(*s);
        s = s.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_is_ascii(mut str: *const gchar) -> gboolean {
    let mut i: gsize = 0;
    i = 0 as gsize;
    while *str.offset(i as isize) != 0 {
        if *str.offset(i as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            return FALSE;
        }
        i = i.wrapping_add(1);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdown(mut string: *mut gchar) -> *mut gchar {
    let mut s: *mut guchar = ::core::ptr::null_mut::<guchar>();
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
        return ::core::ptr::null_mut::<gchar>();
    }
    s = string as *mut guchar;
    while *s != 0 {
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
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strup(mut string: *mut gchar) -> *mut gchar {
    let mut s: *mut guchar = ::core::ptr::null_mut::<guchar>();
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
        return ::core::ptr::null_mut::<gchar>();
    }
    s = string as *mut guchar;
    while *s != 0 {
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
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strreverse(mut string: *mut gchar) -> *mut gchar {
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
        return ::core::ptr::null_mut::<gchar>();
    }
    if *string != 0 {
        let mut h: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut t: *mut gchar = ::core::ptr::null_mut::<gchar>();
        h = string;
        t = string
            .offset(strlen(string) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        while h < t {
            let mut c: gchar = 0;
            c = *h;
            *h = *t;
            h = h.offset(1);
            *t = c;
            t = t.offset(-1);
        }
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_tolower(mut c: gchar) -> gchar {
    return (if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_UPPER as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        c as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
    } else {
        c as ::core::ffi::c_int
    }) as gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_toupper(mut c: gchar) -> gchar {
    return (if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_LOWER as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        c as ::core::ffi::c_int - 'a' as i32 + 'A' as i32
    } else {
        c as ::core::ffi::c_int
    }) as gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_digit_value(mut c: gchar) -> gint {
    if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_DIGIT as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        return c as gint - '0' as i32;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_xdigit_value(mut c: gchar) -> gint {
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32 {
        return c as gint - 'A' as i32 + 10 as gint;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32 {
        return c as gint - 'a' as i32 + 10 as gint;
    }
    return safe_c2rust_g_ascii_digit_value(c);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strcasecmp(
    mut s1: *const gchar,
    mut s2: *const gchar,
) -> gint {
    let mut c1: gint = 0;
    let mut c2: gint = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !s1.is_null() {
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
            b"s1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !s2.is_null() {
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
            b"s2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    while *s1 as ::core::ffi::c_int != 0 && *s2 as ::core::ffi::c_int != 0 {
        c1 = (if *s1 as ::core::ffi::c_int >= 'A' as i32 && *s1 as ::core::ffi::c_int <= 'Z' as i32
        {
            *s1 as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s1 as ::core::ffi::c_int
        }) as guchar as gint;
        c2 = (if *s2 as ::core::ffi::c_int >= 'A' as i32 && *s2 as ::core::ffi::c_int <= 'Z' as i32
        {
            *s2 as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s2 as ::core::ffi::c_int
        }) as guchar as gint;
        if c1 != c2 {
            return c1 - c2;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return *s1 as guchar as gint - *s2 as guchar as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_strncasecmp(
    mut s1: *const gchar,
    mut s2: *const gchar,
    mut n: gsize,
) -> gint {
    let mut c1: gint = 0;
    let mut c2: gint = 0;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !s1.is_null() {
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
            b"s1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !s2.is_null() {
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
            b"s2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    while n != 0 && *s1 as ::core::ffi::c_int != 0 && *s2 as ::core::ffi::c_int != 0 {
        n = n.wrapping_sub(1 as gsize);
        c1 = (if *s1 as ::core::ffi::c_int >= 'A' as i32 && *s1 as ::core::ffi::c_int <= 'Z' as i32
        {
            *s1 as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s1 as ::core::ffi::c_int
        }) as guchar as gint;
        c2 = (if *s2 as ::core::ffi::c_int >= 'A' as i32 && *s2 as ::core::ffi::c_int <= 'Z' as i32
        {
            *s2 as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
        } else {
            *s2 as ::core::ffi::c_int
        }) as guchar as gint;
        if c1 != c2 {
            return c1 - c2;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    if n != 0 {
        return *s1 as guchar as gint - *s2 as guchar as gint;
    } else {
        return 0 as gint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strcasecmp(
    mut s1: *const gchar,
    mut s2: *const gchar,
) -> gint {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !s1.is_null() {
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
            b"s1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !s2.is_null() {
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
            b"s2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return strcasecmp(
        s1 as *const ::core::ffi::c_char,
        s2 as *const ::core::ffi::c_char,
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strncasecmp(
    mut s1: *const gchar,
    mut s2: *const gchar,
    mut n: guint,
) -> gint {
    return strncasecmp(
        s1 as *const ::core::ffi::c_char,
        s2 as *const ::core::ffi::c_char,
        n as size_t,
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdelimit(
    mut string: *mut gchar,
    mut delimiters: *const gchar,
    mut new_delim: gchar,
) -> *mut gchar {
    let mut c: *mut gchar = ::core::ptr::null_mut::<gchar>();
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
        return ::core::ptr::null_mut::<gchar>();
    }
    if delimiters.is_null() {
        delimiters = G_STR_DELIMITERS.as_ptr() as *const gchar;
    }
    c = string;
    while *c != 0 {
        if !strchr(
            delimiters as *const ::core::ffi::c_char,
            *c as ::core::ffi::c_int,
        )
        .is_null()
        {
            *c = new_delim;
        }
        c = c.offset(1);
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strcanon(
    mut string: *mut gchar,
    mut valid_chars: *const gchar,
    mut substitutor: gchar,
) -> *mut gchar {
    let mut c: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !valid_chars.is_null() {
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
            b"valid_chars != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    c = string;
    while *c != 0 {
        if strchr(
            valid_chars as *const ::core::ffi::c_char,
            *c as ::core::ffi::c_int,
        )
        .is_null()
        {
            *c = substitutor;
        }
        c = c.offset(1);
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strcompress(mut source: *const gchar) -> *mut gchar {
    let mut p: *const gchar = source;
    let mut octal: *const gchar = ::core::ptr::null::<gchar>();
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    dest =
        g_malloc((strlen(source as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize))
            as *mut gchar;
    q = dest;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '\\' as i32 {
            p = p.offset(1);
            match *p as ::core::ffi::c_int {
                0 => {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"g_strcompress: trailing \\\0" as *const u8 as *const gchar,
                    );
                    break;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    *q = 0 as gchar;
                    octal = p;
                    while p < octal.offset(3 as ::core::ffi::c_int as isize)
                        && *p as ::core::ffi::c_int >= '0' as i32
                        && *p as ::core::ffi::c_int <= '7' as i32
                    {
                        *q = (*q as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                            + (*p as ::core::ffi::c_int - '0' as i32))
                            as gchar;
                        p = p.offset(1);
                    }
                    q = q.offset(1);
                    p = p.offset(-1);
                }
                98 => {
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = '\u{8}' as i32 as gchar;
                }
                102 => {
                    let fresh2 = q;
                    q = q.offset(1);
                    *fresh2 = '\u{c}' as i32 as gchar;
                }
                110 => {
                    let fresh3 = q;
                    q = q.offset(1);
                    *fresh3 = '\n' as i32 as gchar;
                }
                114 => {
                    let fresh4 = q;
                    q = q.offset(1);
                    *fresh4 = '\r' as i32 as gchar;
                }
                116 => {
                    let fresh5 = q;
                    q = q.offset(1);
                    *fresh5 = '\t' as i32 as gchar;
                }
                118 => {
                    let fresh6 = q;
                    q = q.offset(1);
                    *fresh6 = '\u{b}' as i32 as gchar;
                }
                _ => {
                    let fresh7 = q;
                    q = q.offset(1);
                    *fresh7 = *p;
                }
            }
        } else {
            let fresh8 = q;
            q = q.offset(1);
            *fresh8 = *p;
        }
        p = p.offset(1);
    }
    *q = 0 as gchar;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strescape(
    mut source: *const gchar,
    mut exceptions: *const gchar,
) -> *mut gchar {
    let mut p: *const guchar = ::core::ptr::null::<guchar>();
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut excmap: [guchar; 256] = [0; 256];
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !source.is_null() {
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
            b"source != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    p = source as *mut guchar;
    dest = g_malloc(
        (strlen(source as *const ::core::ffi::c_char) as gsize)
            .wrapping_mul(4 as gsize)
            .wrapping_add(1 as gsize),
    ) as *mut gchar;
    q = dest;
    memset(
        &raw mut excmap as *mut guchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
    if !exceptions.is_null() {
        let mut e: *mut guchar = exceptions as *mut guchar;
        while *e != 0 {
            excmap[*e as usize] = 1 as guchar;
            e = e.offset(1);
        }
    }
    while *p != 0 {
        if excmap[*p as usize] != 0 {
            let fresh9 = q;
            q = q.offset(1);
            *fresh9 = *p as gchar;
        } else {
            match *p as ::core::ffi::c_int {
                8 => {
                    let fresh10 = q;
                    q = q.offset(1);
                    *fresh10 = '\\' as i32 as gchar;
                    let fresh11 = q;
                    q = q.offset(1);
                    *fresh11 = 'b' as i32 as gchar;
                }
                12 => {
                    let fresh12 = q;
                    q = q.offset(1);
                    *fresh12 = '\\' as i32 as gchar;
                    let fresh13 = q;
                    q = q.offset(1);
                    *fresh13 = 'f' as i32 as gchar;
                }
                10 => {
                    let fresh14 = q;
                    q = q.offset(1);
                    *fresh14 = '\\' as i32 as gchar;
                    let fresh15 = q;
                    q = q.offset(1);
                    *fresh15 = 'n' as i32 as gchar;
                }
                13 => {
                    let fresh16 = q;
                    q = q.offset(1);
                    *fresh16 = '\\' as i32 as gchar;
                    let fresh17 = q;
                    q = q.offset(1);
                    *fresh17 = 'r' as i32 as gchar;
                }
                9 => {
                    let fresh18 = q;
                    q = q.offset(1);
                    *fresh18 = '\\' as i32 as gchar;
                    let fresh19 = q;
                    q = q.offset(1);
                    *fresh19 = 't' as i32 as gchar;
                }
                11 => {
                    let fresh20 = q;
                    q = q.offset(1);
                    *fresh20 = '\\' as i32 as gchar;
                    let fresh21 = q;
                    q = q.offset(1);
                    *fresh21 = 'v' as i32 as gchar;
                }
                92 => {
                    let fresh22 = q;
                    q = q.offset(1);
                    *fresh22 = '\\' as i32 as gchar;
                    let fresh23 = q;
                    q = q.offset(1);
                    *fresh23 = '\\' as i32 as gchar;
                }
                34 => {
                    let fresh24 = q;
                    q = q.offset(1);
                    *fresh24 = '\\' as i32 as gchar;
                    let fresh25 = q;
                    q = q.offset(1);
                    *fresh25 = '"' as i32 as gchar;
                }
                _ => {
                    if (*p as ::core::ffi::c_int) < ' ' as i32
                        || *p as ::core::ffi::c_int >= 0o177 as ::core::ffi::c_int
                    {
                        let fresh26 = q;
                        q = q.offset(1);
                        *fresh26 = '\\' as i32 as gchar;
                        let fresh27 = q;
                        q = q.offset(1);
                        *fresh27 = ('0' as i32
                            + (*p as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                                & 0o7 as ::core::ffi::c_int))
                            as gchar;
                        let fresh28 = q;
                        q = q.offset(1);
                        *fresh28 = ('0' as i32
                            + (*p as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                                & 0o7 as ::core::ffi::c_int))
                            as gchar;
                        let fresh29 = q;
                        q = q.offset(1);
                        *fresh29 = ('0' as i32
                            + (*p as ::core::ffi::c_int & 0o7 as ::core::ffi::c_int))
                            as gchar;
                    } else {
                        let fresh30 = q;
                        q = q.offset(1);
                        *fresh30 = *p as gchar;
                    }
                }
            }
        }
        p = p.offset(1);
    }
    *q = 0 as gchar;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strchug(mut string: *mut gchar) -> *mut gchar {
    let mut start: *mut guchar = ::core::ptr::null_mut::<guchar>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    start = string as *mut guchar;
    while *start as ::core::ffi::c_int != 0
        && *safe_c2rust_g_ascii_table.offset(*start as isize) as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        start = start.offset(1);
    }
    memmove(
        string as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        strlen(start as *mut gchar).wrapping_add(1 as size_t),
    );
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strchomp(mut string: *mut gchar) -> *mut gchar {
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    len = strlen(string) as gsize;
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if !(*safe_c2rust_g_ascii_table.offset(*string.offset(len as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
        {
            break;
        }
        *string.offset(len as isize) = '\0' as i32 as gchar;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strsplit(
    mut string: *const gchar,
    mut delimiter: *const gchar,
    mut max_tokens: gint,
) -> *mut *mut gchar {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut remainder: *const gchar = ::core::ptr::null::<gchar>();
    let mut string_list: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
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
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !delimiter.is_null() {
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
            b"delimiter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if *delimiter.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
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
            b"delimiter[0] != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if max_tokens < 1 as ::core::ffi::c_int {
        max_tokens = G_MAXINT as gint;
        string_list = g_ptr_array_new();
    } else {
        string_list = g_ptr_array_new_full(
            (max_tokens as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint,
            None,
        );
    }
    remainder = string;
    s = strstr(
        remainder as *const ::core::ffi::c_char,
        delimiter as *const ::core::ffi::c_char,
    );
    if !s.is_null() {
        let mut delimiter_len: gsize = strlen(delimiter as *const ::core::ffi::c_char) as gsize;
        loop {
            max_tokens -= 1;
            if !(max_tokens != 0 && !s.is_null()) {
                break;
            }
            let mut len: gsize = 0;
            len = s.offset_from(remainder) as ::core::ffi::c_long as gsize;
            g_ptr_array_add(
                string_list,
                safe_c2rust_g_strndup(remainder, len) as gpointer,
            );
            remainder = s.offset(delimiter_len as isize);
            s = strstr(
                remainder as *const ::core::ffi::c_char,
                delimiter as *const ::core::ffi::c_char,
            );
        }
    }
    if *string != 0 {
        g_ptr_array_add(
            string_list,
            safe_c2rust_g_strdup_inline(remainder as *const ::core::ffi::c_char) as gpointer,
        );
    }
    g_ptr_array_add(string_list, NULL_0);
    return g_ptr_array_free(string_list, FALSE) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strsplit_set(
    mut string: *const gchar,
    mut delimiters: *const gchar,
    mut max_tokens: gint,
) -> *mut *mut gchar {
    let mut delim_table: [guint8; 256] = [0; 256];
    let mut tokens: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut n_tokens: gint = 0;
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut current: *const gchar = ::core::ptr::null::<gchar>();
    let mut token: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
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
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !delimiters.is_null() {
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
            b"delimiters != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if max_tokens < 1 as ::core::ffi::c_int {
        max_tokens = G_MAXINT as gint;
    }
    if *string as ::core::ffi::c_int == '\0' as i32 {
        result = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char as *mut *mut gchar;
        let ref mut fresh31 = *result.offset(0 as ::core::ffi::c_int as isize);
        *fresh31 = ::core::ptr::null_mut::<gchar>();
        return result;
    }
    memset(
        &raw mut delim_table as *mut guint8 as *mut ::core::ffi::c_void,
        FALSE,
        ::core::mem::size_of::<[guint8; 256]>() as size_t,
    );
    s = delimiters;
    while *s as ::core::ffi::c_int != '\0' as i32 {
        delim_table[*(s as *mut guchar) as usize] = TRUE as guint8;
        s = s.offset(1);
    }
    tokens = ::core::ptr::null_mut::<GSList>();
    n_tokens = 0 as ::core::ffi::c_int as gint;
    current = string;
    s = current;
    while *s as ::core::ffi::c_int != '\0' as i32 {
        if delim_table[*(s as *mut guchar) as usize] as ::core::ffi::c_int != 0
            && (n_tokens as ::core::ffi::c_int + 1 as ::core::ffi::c_int) < max_tokens
        {
            token = safe_c2rust_g_strndup(
                current,
                s.offset_from(current) as ::core::ffi::c_long as gsize,
            );
            tokens = g_slist_prepend(tokens, token as gpointer);
            n_tokens += 1;
            current = s.offset(1 as ::core::ffi::c_int as isize);
        }
        s = s.offset(1);
    }
    token = safe_c2rust_g_strndup(
        current,
        s.offset_from(current) as ::core::ffi::c_long as gsize,
    );
    tokens = g_slist_prepend(tokens, token as gpointer);
    n_tokens += 1;
    result = ({
        let mut __n: gsize = (n_tokens as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
    let ref mut fresh32 = *result.offset(n_tokens as isize);
    *fresh32 = ::core::ptr::null_mut::<gchar>();
    list = tokens;
    while !list.is_null() {
        n_tokens -= 1;
        let ref mut fresh33 = *result.offset(n_tokens as isize);
        *fresh33 = (*list).data as *mut gchar;
        list = (*list).next;
    }
    g_slist_free(tokens);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strfreev(mut str_array: *mut *mut gchar) {
    if !str_array.is_null() {
        let mut i: gsize = 0;
        i = 0 as gsize;
        while !(*str_array.offset(i as isize)).is_null() {
            g_free(*str_array.offset(i as isize) as gpointer);
            i = i.wrapping_add(1);
        }
        g_free(str_array as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strdupv(mut str_array: *mut *mut gchar) -> *mut *mut gchar {
    if !str_array.is_null() {
        let mut i: gsize = 0;
        let mut retval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        i = 0 as gsize;
        while !(*str_array.offset(i as isize)).is_null() {
            i = i.wrapping_add(1);
        }
        retval = ({
            let mut __n: gsize = i.wrapping_add(1 as gsize);
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
        i = 0 as gsize;
        while !(*str_array.offset(i as isize)).is_null() {
            let ref mut fresh34 = *retval.offset(i as isize);
            *fresh34 = safe_c2rust_g_strdup_inline(*str_array.offset(i as isize)) as *mut gchar;
            i = i.wrapping_add(1);
        }
        let ref mut fresh35 = *retval.offset(i as isize);
        *fresh35 = ::core::ptr::null_mut::<gchar>();
        return retval;
    } else {
        return ::core::ptr::null_mut::<*mut gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strjoinv(
    mut separator: *const gchar,
    mut str_array: *mut *mut gchar,
) -> *mut gchar {
    let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !str_array.is_null() {
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
            b"str_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if separator.is_null() {
        separator = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if !(*str_array).is_null() {
        let mut i: gsize = 0;
        let mut len: gsize = 0;
        let mut separator_len: gsize = 0;
        separator_len = strlen(separator as *const ::core::ffi::c_char) as gsize;
        len = (1 as size_t)
            .wrapping_add(strlen(*str_array.offset(0 as ::core::ffi::c_int as isize)))
            as gsize;
        i = 1 as gsize;
        while !(*str_array.offset(i as isize)).is_null() {
            len = (len as ::core::ffi::c_ulong)
                .wrapping_add(strlen(*str_array.offset(i as isize)) as ::core::ffi::c_ulong)
                as gsize as gsize;
            i = i.wrapping_add(1);
        }
        len = len.wrapping_add(separator_len.wrapping_mul(i.wrapping_sub(1 as gsize)));
        string = ({
            let mut __n: gsize = len;
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
        ptr = safe_c2rust_g_stpcpy(string, *str_array);
        i = 1 as gsize;
        while !(*str_array.offset(i as isize)).is_null() {
            ptr = safe_c2rust_g_stpcpy(ptr, separator);
            ptr = safe_c2rust_g_stpcpy(ptr, *str_array.offset(i as isize));
            i = i.wrapping_add(1);
        }
    } else {
        string = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strjoin(
    mut separator: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    let mut len: gsize = 0;
    let mut separator_len: gsize = 0;
    let mut ptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if separator.is_null() {
        separator = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    separator_len = strlen(separator as *const ::core::ffi::c_char) as gsize;
    args_0 = args.clone();
    s = args_0.arg::<*mut gchar>();
    if !s.is_null() {
        len = (1 as size_t).wrapping_add(strlen(s)) as gsize;
        s = args_0.arg::<*mut gchar>();
        while !s.is_null() {
            len = (len as ::core::ffi::c_ulong).wrapping_add(
                (separator_len as size_t).wrapping_add(strlen(s)) as ::core::ffi::c_ulong,
            ) as gsize as gsize;
            s = args_0.arg::<*mut gchar>();
        }
        string = ({
            let mut __n: gsize = len;
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
        args_0 = args.clone();
        s = args_0.arg::<*mut gchar>();
        ptr = safe_c2rust_g_stpcpy(string, s);
        s = args_0.arg::<*mut gchar>();
        while !s.is_null() {
            ptr = safe_c2rust_g_stpcpy(ptr, separator);
            ptr = safe_c2rust_g_stpcpy(ptr, s);
            s = args_0.arg::<*mut gchar>();
        }
    } else {
        string = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strstr_len(
    mut haystack: *const gchar,
    mut haystack_len: gssize,
    mut needle: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !haystack.is_null() {
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
            b"haystack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !needle.is_null() {
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
            b"needle != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if haystack_len < 0 as gssize {
        return strstr(
            haystack as *const ::core::ffi::c_char,
            needle as *const ::core::ffi::c_char,
        ) as *mut gchar;
    } else {
        let mut p: *const gchar = haystack;
        let mut needle_len: gsize = strlen(needle as *const ::core::ffi::c_char) as gsize;
        let mut haystack_len_unsigned: gsize = haystack_len as gsize;
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        let mut i: gsize = 0;
        if needle_len == 0 as gsize {
            return haystack as *mut gchar;
        }
        if haystack_len_unsigned < needle_len {
            return ::core::ptr::null_mut::<gchar>();
        }
        end = haystack
            .offset(haystack_len as isize)
            .offset(-(needle_len as isize));
        while p <= end && *p as ::core::ffi::c_int != 0 {
            let mut current_block_22: u64;
            i = 0 as gsize;
            loop {
                if !(i < needle_len) {
                    current_block_22 = 4495394744059808450;
                    break;
                }
                if *p.offset(i as isize) as ::core::ffi::c_int
                    != *needle.offset(i as isize) as ::core::ffi::c_int
                {
                    current_block_22 = 15168256691511893694;
                    break;
                }
                i = i.wrapping_add(1);
            }
            match current_block_22 {
                4495394744059808450 => return p as *mut gchar,
                _ => {
                    p = p.offset(1);
                }
            }
        }
        return ::core::ptr::null_mut::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strrstr(
    mut haystack: *const gchar,
    mut needle: *const gchar,
) -> *mut gchar {
    let mut i: gsize = 0;
    let mut needle_len: gsize = 0;
    let mut haystack_len: gsize = 0;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !haystack.is_null() {
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
            b"haystack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !needle.is_null() {
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
            b"needle != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    needle_len = strlen(needle as *const ::core::ffi::c_char) as gsize;
    haystack_len = strlen(haystack as *const ::core::ffi::c_char) as gsize;
    if needle_len == 0 as gsize {
        return haystack as *mut gchar;
    }
    if haystack_len < needle_len {
        return ::core::ptr::null_mut::<gchar>();
    }
    p = haystack
        .offset(haystack_len as isize)
        .offset(-(needle_len as isize));
    while p >= haystack {
        let mut current_block_23: u64;
        i = 0 as gsize;
        loop {
            if !(i < needle_len) {
                current_block_23 = 224731115979188411;
                break;
            }
            if *p.offset(i as isize) as ::core::ffi::c_int
                != *needle.offset(i as isize) as ::core::ffi::c_int
            {
                current_block_23 = 3809414565567014869;
                break;
            }
            i = i.wrapping_add(1);
        }
        match current_block_23 {
            224731115979188411 => return p as *mut gchar,
            _ => {
                p = p.offset(-1);
            }
        }
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strrstr_len(
    mut haystack: *const gchar,
    mut haystack_len: gssize,
    mut needle: *const gchar,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !haystack.is_null() {
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
            b"haystack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !needle.is_null() {
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
            b"needle != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if haystack_len < 0 as gssize {
        return safe_c2rust_g_strrstr(haystack, needle);
    } else {
        let mut needle_len: gsize = strlen(needle as *const ::core::ffi::c_char) as gsize;
        let mut haystack_max: *const gchar = haystack.offset(haystack_len as isize);
        let mut p: *const gchar = haystack;
        let mut i: gsize = 0;
        while p < haystack_max && *p as ::core::ffi::c_int != 0 {
            p = p.offset(1);
        }
        if p < haystack.offset(needle_len as isize) {
            return ::core::ptr::null_mut::<gchar>();
        }
        p = p.offset(-(needle_len as isize));
        while p >= haystack {
            let mut current_block_22: u64;
            i = 0 as gsize;
            loop {
                if !(i < needle_len) {
                    current_block_22 = 4495394744059808450;
                    break;
                }
                if *p.offset(i as isize) as ::core::ffi::c_int
                    != *needle.offset(i as isize) as ::core::ffi::c_int
                {
                    current_block_22 = 7874395875704760479;
                    break;
                }
                i = i.wrapping_add(1);
            }
            match current_block_22 {
                4495394744059808450 => return p as *mut gchar,
                _ => {
                    p = p.offset(-1);
                }
            }
        }
        return ::core::ptr::null_mut::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_has_suffix(
    mut str: *const gchar,
    mut suffix: *const gchar,
) -> gboolean {
    let mut str_len: gsize = 0;
    let mut suffix_len: gsize = 0;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !suffix.is_null() {
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
            b"suffix != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    str_len = strlen(str as *const ::core::ffi::c_char) as gsize;
    suffix_len = strlen(suffix as *const ::core::ffi::c_char) as gsize;
    if str_len < suffix_len {
        return FALSE;
    }
    return (strcmp(
        str.offset(str_len as isize).offset(-(suffix_len as isize)),
        suffix as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_has_prefix(
    mut str: *const gchar,
    mut prefix: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"g_str_has_prefix\0" as *const u8 as *const ::core::ffi::c_char,
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !prefix.is_null() {
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
            b"g_str_has_prefix\0" as *const u8 as *const ::core::ffi::c_char,
            b"prefix != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (strncmp(
        str as *const ::core::ffi::c_char,
        prefix as *const ::core::ffi::c_char,
        strlen(prefix as *const ::core::ffi::c_char),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_length(mut str_array: *mut *mut gchar) -> guint {
    let mut i: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !str_array.is_null() {
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
            b"str_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    while !(*str_array.offset(i as isize)).is_null() {
        i = i.wrapping_add(1);
    }
    return i;
}
unsafe extern "C" fn safe_c2rust_index_add_folded(
    mut array: *mut GPtrArray,
    mut start: *const gchar,
    mut end: *const gchar,
) {
    let mut normal: *mut gchar = ::core::ptr::null_mut::<gchar>();
    normal = g_utf8_normalize(
        start,
        end.offset_from(start) as gssize,
        G_NORMALIZE_ALL_COMPOSE,
    );
    if !strstr(
        normal,
        b"\xC4\xB1\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            normal,
            b"\xC4\xB0\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        let mut s: *mut gchar = normal;
        let mut tmp: *mut GString = ::core::ptr::null_mut::<GString>();
        tmp = g_string_new(::core::ptr::null::<gchar>());
        while *s != 0 {
            let mut i: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut I: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut e: *mut gchar = ::core::ptr::null_mut::<gchar>();
            i = strstr(s, b"\xC4\xB1\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
            I = strstr(s, b"\xC4\xB0\0" as *const u8 as *const ::core::ffi::c_char) as *mut gchar;
            if i.is_null() && I.is_null() {
                break;
            }
            if !i.is_null() && I.is_null() {
                e = i;
            } else if !I.is_null() && i.is_null() {
                e = I;
            } else if i < I {
                e = i;
            } else {
                e = I;
            }
            safe_c2rust_g_string_append_len_inline(tmp, s, e.offset_from(s) as gssize);
            safe_c2rust_g_string_append_c_inline(tmp, 'i' as i32 as gchar);
            s = e.offset(
                *safe_c2rust_g_utf8_skip.offset(*(e as *const guchar) as isize)
                    as ::core::ffi::c_int as isize,
            ) as *mut ::core::ffi::c_char as *mut gchar;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = s;
                safe_c2rust_g_string_append_len_inline(
                    tmp,
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
            safe_c2rust_g_string_append_len_inline(tmp, s, -(1 as ::core::ffi::c_int) as gssize);
        };
        g_free(normal as gpointer);
        normal = if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(tmp, 0 as gboolean)
            } else {
                g_string_free_and_steal(tmp)
            }
        } else {
            g_string_free(tmp, 0 as gboolean)
        };
    }
    g_ptr_array_add(
        array,
        g_utf8_casefold(normal, -(1 as ::core::ffi::c_int) as gssize) as gpointer,
    );
    g_free(normal as gpointer);
}
unsafe extern "C" fn safe_c2rust_split_words(mut value: *const gchar) -> *mut *mut gchar {
    let mut start: *const gchar = ::core::ptr::null::<gchar>();
    let mut result: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    result = g_ptr_array_new();
    s = value;
    while *s != 0 {
        let mut c: gunichar = g_utf8_get_char(s);
        if start.is_null() {
            if g_unichar_isalnum(c) != 0 || g_unichar_ismark(c) != 0 {
                start = s;
            }
        } else if g_unichar_isalnum(c) == 0 && g_unichar_ismark(c) == 0 {
            safe_c2rust_index_add_folded(result, start, s);
            start = ::core::ptr::null::<gchar>();
        }
        s = s.offset(
            *safe_c2rust_g_utf8_skip.offset(*(s as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    if !start.is_null() {
        safe_c2rust_index_add_folded(result, start, s);
    }
    g_ptr_array_add(result, NULL_0);
    return g_ptr_array_free(result, FALSE) as *mut *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_tokenize_and_fold(
    mut string: *const gchar,
    mut translit_locale: *const gchar,
    mut ascii_alternates: *mut *mut *mut gchar,
) -> *mut *mut gchar {
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if !ascii_alternates.is_null() && safe_c2rust_g_str_is_ascii(string) != 0 {
        *ascii_alternates = ({
            let mut __n: gsize = (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        ascii_alternates = ::core::ptr::null_mut::<*mut *mut gchar>();
    }
    result = safe_c2rust_split_words(string);
    if !ascii_alternates.is_null() {
        let mut i: gint = 0;
        let mut j: gint = 0;
        let mut n: gint = 0;
        n = safe_c2rust_g_strv_length(result) as gint;
        *ascii_alternates = ({
            let mut __n: gsize = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        j = 0 as ::core::ffi::c_int as gint;
        i = 0 as ::core::ffi::c_int as gint;
        while i < n {
            if safe_c2rust_g_str_is_ascii(*result.offset(i as isize)) == 0 {
                let mut composed: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut ascii: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut k: gint = 0;
                composed = g_utf8_normalize(
                    *result.offset(i as isize),
                    -(1 as ::core::ffi::c_int) as gssize,
                    G_NORMALIZE_ALL_COMPOSE,
                );
                ascii = g_str_to_ascii(composed, translit_locale);
                k = 0 as ::core::ffi::c_int as gint;
                while *ascii.offset(k as isize) != 0 {
                    if !(*safe_c2rust_g_ascii_table
                        .offset(*ascii.offset(k as isize) as guchar as isize)
                        as ::core::ffi::c_int
                        & G_ASCII_ALNUM as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int)
                    {
                        break;
                    }
                    k += 1;
                }
                if *ascii.offset(k as isize) as ::core::ffi::c_int == '\0' as i32 {
                    let fresh36 = j;
                    j = j + 1;
                    let ref mut fresh37 = *(*ascii_alternates).offset(fresh36 as isize);
                    *fresh37 = ascii;
                } else {
                    g_free(ascii as gpointer);
                }
                g_free(composed as gpointer);
            }
            i += 1;
        }
        let ref mut fresh38 = *(*ascii_alternates).offset(j as isize);
        *fresh38 = ::core::ptr::null_mut::<gchar>();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_str_match_string(
    mut search_term: *const gchar,
    mut potential_hit: *const gchar,
    mut accept_alternates: gboolean,
) -> gboolean {
    let mut current_block: u64;
    let mut alternates: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut term_tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut hit_tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut matched: gboolean = 0;
    let mut i: gint = 0;
    let mut j: gint = 0;
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !search_term.is_null() {
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
            b"search_term != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !potential_hit.is_null() {
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
            b"potential_hit != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    term_tokens = safe_c2rust_g_str_tokenize_and_fold(
        search_term,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut *mut gchar>(),
    );
    hit_tokens = safe_c2rust_g_str_tokenize_and_fold(
        potential_hit,
        ::core::ptr::null::<gchar>(),
        if accept_alternates != 0 {
            &raw mut alternates
        } else {
            ::core::ptr::null_mut::<*mut *mut gchar>()
        },
    );
    matched = TRUE as gboolean;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*term_tokens.offset(i as isize)).is_null() {
        j = 0 as ::core::ffi::c_int as gint;
        loop {
            if (*hit_tokens.offset(j as isize)).is_null() {
                current_block = 12124785117276362961;
                break;
            }
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = *hit_tokens.offset(j as isize);
                    let __prefix: *const ::core::ffi::c_char = *term_tokens.offset(i as isize);
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_59
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result = safe_c2rust_g_str_has_prefix(
                            __str as *const gchar,
                            __prefix as *const gchar,
                        );
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                safe_c2rust_g_str_has_prefix(
                    *hit_tokens.offset(j as isize),
                    *term_tokens.offset(i as isize),
                )
            } != 0
            {
                current_block = 17407779659766490442;
                break;
            }
            j += 1;
        }
        match current_block {
            12124785117276362961 => {
                if accept_alternates != 0 {
                    j = 0 as ::core::ffi::c_int as gint;
                    loop {
                        if (*alternates.offset(j as isize)).is_null() {
                            current_block = 11307063007268554308;
                            break;
                        }
                        if if 0 != 0 {
                            ({
                                let __str: *const ::core::ffi::c_char =
                                    *alternates.offset(j as isize);
                                let __prefix: *const ::core::ffi::c_char =
                                    *term_tokens.offset(i as isize);
                                let mut __result: gboolean = FALSE;
                                if ({
                                    let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
                                    if __str.is_null() || __prefix.is_null() {
                                        _g_boolean_var_60 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_60 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_60
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    __result = safe_c2rust_g_str_has_prefix(
                                        __str as *const gchar,
                                        __prefix as *const gchar,
                                    );
                                } else {
                                    let __str_len: size_t = strlen(
                                        __str
                                            .offset(__str.is_null() as ::core::ffi::c_int as isize),
                                    )
                                        as size_t;
                                    let __prefix_len: size_t =
                                        strlen(__prefix.offset(__prefix.is_null()
                                            as ::core::ffi::c_int
                                            as isize))
                                            as size_t;
                                    if __str_len >= __prefix_len {
                                        __result = (memcmp(
                                            __str.offset(
                                                __str.is_null() as ::core::ffi::c_int as isize
                                            )
                                                as *const ::core::ffi::c_void,
                                            __prefix
                                                .offset(__prefix.is_null() as ::core::ffi::c_int
                                                    as isize)
                                                as *const ::core::ffi::c_void,
                                            __prefix_len,
                                        ) == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as gboolean;
                                    }
                                }
                                __result
                            })
                        } else {
                            safe_c2rust_g_str_has_prefix(
                                *alternates.offset(j as isize),
                                *term_tokens.offset(i as isize),
                            )
                        } != 0
                        {
                            current_block = 17407779659766490442;
                            break;
                        }
                        j += 1;
                    }
                } else {
                    current_block = 11307063007268554308;
                }
                match current_block {
                    17407779659766490442 => {}
                    _ => {
                        matched = FALSE as gboolean;
                        break;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    safe_c2rust_g_strfreev(term_tokens);
    safe_c2rust_g_strfreev(hit_tokens);
    safe_c2rust_g_strfreev(alternates);
    return matched;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_contains(
    mut strv: *const *const gchar,
    mut str: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !strv.is_null() {
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
            b"strv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    while !(*strv).is_null() {
        if strcmp(
            str as *const ::core::ffi::c_char,
            *strv as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        strv = strv.offset(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strv_equal(
    mut strv1: *const *const gchar,
    mut strv2: *const *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !strv1.is_null() {
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
            b"strv1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !strv2.is_null() {
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
            b"strv2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if strv1 == strv2 {
        return TRUE;
    }
    while !(*strv1).is_null() && !(*strv2).is_null() {
        if !(strcmp(
            *strv1 as *const ::core::ffi::c_char,
            *strv2 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            return FALSE;
        }
        strv1 = strv1.offset(1);
        strv2 = strv2.offset(1);
    }
    return ((*strv1).is_null() && (*strv2).is_null()) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_str_has_sign(mut str: *const gchar) -> gboolean {
    return (*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_str_has_hex_prefix(mut str: *const gchar) -> gboolean {
    return (*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '0' as i32
        && safe_c2rust_g_ascii_tolower(*str.offset(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int
            == 'x' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_string_to_signed(
    mut str: *const gchar,
    mut base: guint,
    mut min: gint64,
    mut max: gint64,
    mut out_num: *mut gint64,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut number: gint64 = 0;
    let mut end_ptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut saved_errno: gint = 0 as gint;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if base >= 2 as guint && base <= 36 as guint {
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
            b"base >= 2 && base <= 36\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if min <= max {
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
            b"min <= max\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        g_set_error_literal(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_INVALID as ::core::ffi::c_int as gint,
            glib_gettext(b"Empty string is not a number\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    number = safe_c2rust_g_ascii_strtoll(str, &raw mut end_ptr as *mut *mut gchar, base);
    saved_errno = *__errno_location() as gint;
    if *safe_c2rust_g_ascii_table
        .offset(*str.offset(0 as ::core::ffi::c_int as isize) as guchar as isize)
        as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || base == 16 as guint
            && (if safe_c2rust_str_has_sign(str) != 0 {
                safe_c2rust_str_has_hex_prefix(str.offset(1 as ::core::ffi::c_int as isize))
            } else {
                safe_c2rust_str_has_hex_prefix(str)
            }) != 0
        || saved_errno != 0 as ::core::ffi::c_int && saved_errno != ERANGE
        || end_ptr.is_null()
        || *end_ptr as ::core::ffi::c_int != '\0' as i32
    {
        g_set_error(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_INVALID as ::core::ffi::c_int as gint,
            glib_gettext(
                b"\xE2\x80\x9C%s\xE2\x80\x9D is not a signed number\0" as *const u8 as *const gchar,
            ),
            str,
        );
        return FALSE;
    }
    if saved_errno == ERANGE || number < min || number > max {
        let mut min_str: *mut gchar =
            safe_c2rust_g_strdup_printf(b"%li\0" as *const u8 as *const gchar, min);
        let mut max_str: *mut gchar =
            safe_c2rust_g_strdup_printf(b"%li\0" as *const u8 as *const gchar, max);
        g_set_error(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_OUT_OF_BOUNDS as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Number \xE2\x80\x9C%s\xE2\x80\x9D is out of bounds [%s, %s]\0" as *const u8
                    as *const gchar,
            ),
            str,
            min_str,
            max_str,
        );
        g_free(min_str as gpointer);
        g_free(max_str as gpointer);
        return FALSE;
    }
    if !out_num.is_null() {
        *out_num = number;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ascii_string_to_unsigned(
    mut str: *const gchar,
    mut base: guint,
    mut min: guint64,
    mut max: guint64,
    mut out_num: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut number: guint64 = 0;
    let mut end_ptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut saved_errno: gint = 0 as gint;
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if base >= 2 as guint && base <= 36 as guint {
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
            b"base >= 2 && base <= 36\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if min <= max {
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
            b"min <= max\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        g_set_error_literal(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_INVALID as ::core::ffi::c_int as gint,
            glib_gettext(b"Empty string is not a number\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    number = safe_c2rust_g_ascii_strtoull(str, &raw mut end_ptr as *mut *mut gchar, base);
    saved_errno = *__errno_location() as gint;
    if *safe_c2rust_g_ascii_table
        .offset(*str.offset(0 as ::core::ffi::c_int as isize) as guchar as isize)
        as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || safe_c2rust_str_has_sign(str) != 0
        || base == 16 as guint && safe_c2rust_str_has_hex_prefix(str) != 0
        || saved_errno != 0 as ::core::ffi::c_int && saved_errno != ERANGE
        || end_ptr.is_null()
        || *end_ptr as ::core::ffi::c_int != '\0' as i32
    {
        g_set_error(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_INVALID as ::core::ffi::c_int as gint,
            glib_gettext(
                b"\xE2\x80\x9C%s\xE2\x80\x9D is not an unsigned number\0" as *const u8
                    as *const gchar,
            ),
            str,
        );
        return FALSE;
    }
    if saved_errno == ERANGE || number < min || number > max {
        let mut min_str: *mut gchar =
            safe_c2rust_g_strdup_printf(b"%lu\0" as *const u8 as *const gchar, min);
        let mut max_str: *mut gchar =
            safe_c2rust_g_strdup_printf(b"%lu\0" as *const u8 as *const gchar, max);
        g_set_error(
            error,
            safe_c2rust_g_number_parser_error_quark(),
            G_NUMBER_PARSER_ERROR_OUT_OF_BOUNDS as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Number \xE2\x80\x9C%s\xE2\x80\x9D is out of bounds [%s, %s]\0" as *const u8
                    as *const gchar,
            ),
            str,
            min_str,
            max_str,
        );
        g_free(min_str as gpointer);
        g_free(max_str as gpointer);
        return FALSE;
    }
    if !out_num.is_null() {
        *out_num = number;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_number_parser_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(
            b"g-number-parser-error-quark\0" as *const u8 as *const gchar,
        );
    }
    return safe_c2rust_q;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
