use ::c2rust_bitfields;
extern "C" {
    pub type _GTimeZone;
    pub type _GDateTime;
    pub type _GHashTable;
    pub type _GMarkupParseContext;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_date_time_unref(datetime: *mut GDateTime);
    fn g_date_time_ref(datetime: *mut GDateTime) -> *mut GDateTime;
    fn g_date_time_new_now_utc() -> *mut GDateTime;
    fn g_date_time_new_from_unix_utc(t: gint64) -> *mut GDateTime;
    fn g_date_time_new_from_iso8601(
        text: *const gchar,
        default_tz: *mut GTimeZone,
    ) -> *mut GDateTime;
    fn g_date_time_to_unix(datetime: *mut GDateTime) -> gint64;
    fn g_date_time_format_iso8601(datetime: *mut GDateTime) -> *mut gchar;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_filename_from_uri(
        uri: *const gchar,
        hostname: *mut *mut gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_set_contents(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_get_prgname() -> *const gchar;
    fn g_get_application_name() -> *const gchar;
    fn g_get_user_data_dir() -> *const gchar;
    fn g_get_system_data_dirs() -> *const *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_1(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_steal(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_markup_error_quark() -> GQuark;
    fn g_markup_parse_context_new(
        parser: *const GMarkupParser,
        flags: GMarkupParseFlags,
        user_data: gpointer,
        user_data_dnotify: GDestroyNotify,
    ) -> *mut GMarkupParseContext;
    fn g_markup_parse_context_free(context: *mut GMarkupParseContext);
    fn g_markup_parse_context_parse(
        context: *mut GMarkupParseContext,
        text: *const gchar,
        text_len: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_markup_parse_context_end_parse(
        context: *mut GMarkupParseContext,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_markup_escape_text(text: *const gchar, length: gssize) -> *mut gchar;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_strchr(p: *const gchar, len: gssize, c: gunichar) -> *mut gchar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strjoin(separator: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_shell_quote(unquoted_string: *const gchar) -> *mut gchar;
    fn g_shell_unquote(quoted_string: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GTimeZone = _GTimeZone;
pub type GDateTime = _GDateTime;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND: C2RustUnnamed = 7;
pub const G_BOOKMARK_FILE_ERROR_WRITE: C2RustUnnamed = 6;
pub const G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING: C2RustUnnamed = 5;
pub const G_BOOKMARK_FILE_ERROR_READ: C2RustUnnamed = 4;
pub const G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND: C2RustUnnamed = 3;
pub const G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED: C2RustUnnamed = 2;
pub const G_BOOKMARK_FILE_ERROR_INVALID_VALUE: C2RustUnnamed = 1;
pub const G_BOOKMARK_FILE_ERROR_INVALID_URI: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBookmarkFile {
    pub title: *mut gchar,
    pub description: *mut gchar,
    pub items: *mut GList,
    pub items_by_uri: *mut GHashTable,
}
pub type GHashTable = _GHashTable;
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GBookmarkFile = _GBookmarkFile;
pub type BookmarkItem = _BookmarkItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BookmarkItem {
    pub uri: *mut gchar,
    pub title: *mut gchar,
    pub description: *mut gchar,
    pub added: *mut GDateTime,
    pub modified: *mut GDateTime,
    pub visited: *mut GDateTime,
    pub metadata: *mut BookmarkMetadata,
}
pub type BookmarkMetadata = _BookmarkMetadata;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _BookmarkMetadata {
    pub mime_type: *mut gchar,
    pub groups: *mut GList,
    pub applications: *mut GList,
    pub apps_by_name: *mut GHashTable,
    pub icon_href: *mut gchar,
    pub icon_mime: *mut gchar,
    #[bitfield(name = "is_private", ty = "guint", bits = "0..=0")]
    pub is_private: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type BookmarkAppInfo = _BookmarkAppInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BookmarkAppInfo {
    pub name: *mut gchar,
    pub exec: *mut gchar,
    pub count: guint,
    pub stamp: *mut GDateTime,
}
pub type GMarkupParseContext = _GMarkupParseContext;
pub type ParseData = _ParseData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ParseData {
    pub state: ParserState,
    pub namespaces: *mut GHashTable,
    pub bookmark_file: *mut GBookmarkFile,
    pub current_item: *mut BookmarkItem,
}
pub type ParserState = ::core::ffi::c_uint;
pub const STATE_FINISHED: ParserState = 13;
pub const STATE_ICON: ParserState = 12;
pub const STATE_MIME: ParserState = 11;
pub const STATE_GROUP: ParserState = 10;
pub const STATE_GROUPS: ParserState = 9;
pub const STATE_APPLICATION: ParserState = 8;
pub const STATE_APPLICATIONS: ParserState = 7;
pub const STATE_METADATA: ParserState = 6;
pub const STATE_INFO: ParserState = 5;
pub const STATE_DESC: ParserState = 4;
pub const STATE_TITLE: ParserState = 3;
pub const STATE_BOOKMARK: ParserState = 2;
pub const STATE_ROOT: ParserState = 1;
pub const STATE_STARTED: ParserState = 0;
pub type GMarkupParseFlags = ::core::ffi::c_uint;
pub const G_MARKUP_IGNORE_QUALIFIED: GMarkupParseFlags = 8;
pub const G_MARKUP_PREFIX_ERROR_POSITION: GMarkupParseFlags = 4;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: GMarkupParseFlags = 2;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: GMarkupParseFlags = 1;
pub const G_MARKUP_DEFAULT_FLAGS: GMarkupParseFlags = 0;
pub type GMarkupParser = _GMarkupParser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMarkupParser {
    pub start_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            *mut *const gchar,
            *mut *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub end_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub text: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub passthrough: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub error: Option<unsafe extern "C" fn(*mut GMarkupParseContext, *mut GError, gpointer) -> ()>,
}
pub type gunichar = guint32;
pub const G_MARKUP_ERROR_INVALID_CONTENT: C2RustUnnamed_0 = 5;
pub const G_MARKUP_ERROR_UNKNOWN_ELEMENT: C2RustUnnamed_0 = 3;
pub const G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 4;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_MARKUP_ERROR_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 6;
pub const G_MARKUP_ERROR_PARSE: C2RustUnnamed_0 = 2;
pub const G_MARKUP_ERROR_EMPTY: C2RustUnnamed_0 = 1;
pub const G_MARKUP_ERROR_BAD_UTF8: C2RustUnnamed_0 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL_1 as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn safe_c2rust_atol(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    return strtol(
        __nptr,
        NULL_1 as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_2 as gpointer;
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
        let fresh13 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh13 as isize) = c;
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
pub const XBEL_VERSION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"1.0\0") };
pub const XBEL_ROOT_ELEMENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"xbel\0") };
pub const XBEL_BOOKMARK_ELEMENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"bookmark\0") };
pub const XBEL_TITLE_ELEMENT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"title\0") };
pub const XBEL_DESC_ELEMENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"desc\0") };
pub const XBEL_INFO_ELEMENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"info\0") };
pub const XBEL_METADATA_ELEMENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"metadata\0") };
pub const XBEL_HREF_ATTRIBUTE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"href\0") };
pub const BOOKMARK_METADATA_OWNER: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"http://freedesktop.org\0")
};
pub const BOOKMARK_GROUPS_ELEMENT: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"groups\0") };
pub const BOOKMARK_GROUP_ELEMENT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"group\0") };
pub const BOOKMARK_APPLICATIONS_ELEMENT: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"applications\0") };
pub const BOOKMARK_APPLICATION_ELEMENT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"application\0") };
pub const BOOKMARK_ICON_ELEMENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"icon\0") };
pub const BOOKMARK_NAME_ATTRIBUTE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"name\0") };
pub const BOOKMARK_EXEC_ATTRIBUTE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"exec\0") };
pub const BOOKMARK_HREF_ATTRIBUTE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"href\0") };
pub const MIME_TYPE_ELEMENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mime-type\0") };
unsafe extern "C" fn safe_c2rust_bookmark_app_info_new(
    mut name: *const gchar,
) -> *mut BookmarkAppInfo {
    let mut retval: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
    if !(({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            220 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    retval =
        g_slice_alloc(::core::mem::size_of::<BookmarkAppInfo>() as gsize) as *mut BookmarkAppInfo;
    (*retval).name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    (*retval).exec = ::core::ptr::null_mut::<gchar>();
    (*retval).count = 0 as guint;
    (*retval).stamp = ::core::ptr::null_mut::<GDateTime>();
    return retval;
}
unsafe extern "C" fn safe_c2rust_bookmark_app_info_free(mut app_info: *mut BookmarkAppInfo) {
    if app_info.is_null() {
        return;
    }
    g_free((*app_info).name as gpointer);
    g_free((*app_info).exec as gpointer);
    let mut _pp: *mut *mut GDateTime = &raw mut (*app_info).stamp;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    g_slice_free1(
        ::core::mem::size_of::<BookmarkAppInfo>() as gsize,
        app_info as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_bookmark_app_info_copy(
    mut app_info: *mut BookmarkAppInfo,
) -> *mut BookmarkAppInfo {
    let mut copy: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
    if app_info.is_null() {
        return ::core::ptr::null_mut::<BookmarkAppInfo>();
    }
    copy = safe_c2rust_bookmark_app_info_new((*app_info).name);
    (*copy).count = (*app_info).count;
    (*copy).exec = safe_c2rust_g_strdup_inline((*app_info).exec) as *mut gchar;
    if !(*app_info).stamp.is_null() {
        (*copy).stamp = g_date_time_ref((*app_info).stamp);
    }
    return copy;
}
unsafe extern "C" fn safe_c2rust_bookmark_app_info_dump(
    mut app_info: *mut BookmarkAppInfo,
) -> *mut gchar {
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut exec: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut modified: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut count: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !(({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !app_info.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            269 as ::core::ffi::c_int,
            G_STRFUNC,
            b"app_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*app_info).count == 0 as guint {
        return ::core::ptr::null_mut::<gchar>();
    }
    name = g_markup_escape_text((*app_info).name, -(1 as ::core::ffi::c_int) as gssize);
    exec = g_markup_escape_text((*app_info).exec, -(1 as ::core::ffi::c_int) as gssize);
    count = g_strdup_printf(b"%u\0" as *const u8 as *const gchar, (*app_info).count);
    if !(*app_info).stamp.is_null() {
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = g_date_time_format_iso8601((*app_info).stamp) as *mut ::core::ffi::c_char;
        modified = g_strconcat(
            b" modified=\"\0" as *const u8 as *const gchar,
            tmp,
            b"\"\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
        g_free(tmp as gpointer);
    } else {
        modified = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    retval = g_strconcat(
        b"          <bookmark:application name=\"\0" as *const u8 as *const gchar,
        name,
        b"\" exec=\"\0" as *const u8 as *const ::core::ffi::c_char,
        exec,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        modified,
        b" count=\"\0" as *const u8 as *const ::core::ffi::c_char,
        count,
        b"\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
        NULL,
    );
    g_free(name as gpointer);
    g_free(exec as gpointer);
    g_free(modified as gpointer);
    g_free(count as gpointer);
    return retval;
}
unsafe extern "C" fn safe_c2rust_bookmark_metadata_new() -> *mut BookmarkMetadata {
    let mut retval: *mut BookmarkMetadata = ::core::ptr::null_mut::<BookmarkMetadata>();
    retval =
        g_slice_alloc(::core::mem::size_of::<BookmarkMetadata>() as gsize) as *mut BookmarkMetadata;
    (*retval).mime_type = ::core::ptr::null_mut::<gchar>();
    (*retval).groups = ::core::ptr::null_mut::<GList>();
    (*retval).applications = ::core::ptr::null_mut::<GList>();
    (*retval).apps_by_name = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        None,
    );
    (*retval).set_is_private(FALSE as guint as guint);
    (*retval).icon_href = ::core::ptr::null_mut::<gchar>();
    (*retval).icon_mime = ::core::ptr::null_mut::<gchar>();
    return retval;
}
unsafe extern "C" fn safe_c2rust_bookmark_metadata_free(mut metadata: *mut BookmarkMetadata) {
    if metadata.is_null() {
        return;
    }
    g_free((*metadata).mime_type as gpointer);
    g_list_free_full(
        (*metadata).groups,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_list_free_full(
        (*metadata).applications,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BookmarkAppInfo) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_bookmark_app_info_free as unsafe extern "C" fn(*mut BookmarkAppInfo) -> (),
        )),
    );
    g_hash_table_destroy((*metadata).apps_by_name);
    g_free((*metadata).icon_href as gpointer);
    g_free((*metadata).icon_mime as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<BookmarkMetadata>() as gsize,
        metadata as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_bookmark_metadata_copy(
    mut metadata: *mut BookmarkMetadata,
) -> *mut BookmarkMetadata {
    let mut copy: *mut BookmarkMetadata = ::core::ptr::null_mut::<BookmarkMetadata>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if metadata.is_null() {
        return ::core::ptr::null_mut::<BookmarkMetadata>();
    }
    copy = safe_c2rust_bookmark_metadata_new();
    (*copy).set_is_private((*metadata).is_private() as guint);
    (*copy).mime_type = safe_c2rust_g_strdup_inline((*metadata).mime_type) as *mut gchar;
    (*copy).icon_href = safe_c2rust_g_strdup_inline((*metadata).icon_href) as *mut gchar;
    (*copy).icon_mime = safe_c2rust_g_strdup_inline((*metadata).icon_mime) as *mut gchar;
    (*copy).groups = g_list_copy_deep(
        (*metadata).groups,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*const gchar) -> *mut gchar>, GCopyFunc>(
            Some(g_strdup as unsafe extern "C" fn(*const gchar) -> *mut gchar),
        ),
        NULL,
    );
    (*copy).applications = g_list_copy_deep(
        (*metadata).applications,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BookmarkAppInfo) -> *mut BookmarkAppInfo>,
            GCopyFunc,
        >(Some(
            safe_c2rust_bookmark_app_info_copy
                as unsafe extern "C" fn(*mut BookmarkAppInfo) -> *mut BookmarkAppInfo,
        )),
        NULL,
    );
    l = (*copy).applications;
    while !l.is_null() {
        let mut app_info: *mut BookmarkAppInfo = (*l).data as *mut BookmarkAppInfo;
        g_hash_table_insert(
            (*copy).apps_by_name,
            (*app_info).name as gpointer,
            app_info as gpointer,
        );
        l = (*l).next;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_hash_table_size((*copy).apps_by_name) == g_hash_table_size((*metadata).apps_by_name) {
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
            b"../original/glib/gbookmarkfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            384 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (copy->apps_by_name) == g_hash_table_size (metadata->apps_by_name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return copy;
}
unsafe extern "C" fn safe_c2rust_bookmark_metadata_dump(
    mut metadata: *mut BookmarkMetadata,
) -> *mut gchar {
    let mut retval: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if (*metadata).applications.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    retval = g_string_sized_new(1024 as gsize);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"      <metadata owner=\"http://freedesktop.org\">\n\0" as *const u8
                    as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_11
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
            retval,
            b"      <metadata owner=\"http://freedesktop.org\">\n\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(*metadata).mime_type.is_null() {
        buffer = g_strconcat(
            b"        <mime:mime-type type=\"\0" as *const u8 as *const gchar,
            (*metadata).mime_type,
            b"\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = buffer;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                buffer,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(buffer as gpointer);
    }
    if !(*metadata).groups.is_null() {
        let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"        <bookmark:groups>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"        <bookmark:groups>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        l = g_list_last((*metadata).groups);
        while !l.is_null() {
            let mut group_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            group_name = g_markup_escape_text(
                (*l).data as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
            buffer = g_strconcat(
                b"          <bookmark:group>\0" as *const u8 as *const gchar,
                group_name,
                b"</bookmark:group>\n\0" as *const u8 as *const ::core::ffi::c_char,
                NULL,
            );
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = buffer;
                    safe_c2rust_g_string_append_len_inline(
                        retval,
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
                    buffer,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            g_free(buffer as gpointer);
            g_free(group_name as gpointer);
            l = (*l).prev;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"        </bookmark:groups>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"        </bookmark:groups>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !(*metadata).applications.is_null() {
        let mut l_0: *mut GList = ::core::ptr::null_mut::<GList>();
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = b"        <bookmark:applications>\n\0"
                    as *const u8
                    as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"        <bookmark:applications>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        l_0 = g_list_last((*metadata).applications);
        while !l_0.is_null() {
            let mut app_info: *mut BookmarkAppInfo = (*l_0).data as *mut BookmarkAppInfo;
            let mut app_data: *mut gchar = ::core::ptr::null_mut::<gchar>();
            if !(({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if !app_info.is_null() {
                    _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_17
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gbookmarkfile.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    466 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"app_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            app_data = safe_c2rust_bookmark_app_info_dump(app_info);
            if !app_data.is_null() {
                retval = if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = app_data;
                        safe_c2rust_g_string_append_len_inline(
                            retval,
                            __val,
                            if ({
                                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_18
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        )
                    })
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        retval,
                        app_data,
                        -(1 as ::core::ffi::c_int) as gssize,
                    )
                };
                g_free(app_data as gpointer);
            }
            l_0 = (*l_0).prev;
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = b"        </bookmark:applications>\n\0"
                    as *const u8
                    as *const ::core::ffi::c_char;
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
                b"        </bookmark:applications>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !(*metadata).icon_href.is_null() {
        if (*metadata).icon_mime.is_null() {
            (*metadata).icon_mime = safe_c2rust_g_strdup_inline(
                b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        buffer = g_strconcat(
            b"       <bookmark:icon href=\"\0" as *const u8 as *const gchar,
            (*metadata).icon_href,
            b"\" type=\"\0" as *const u8 as *const ::core::ffi::c_char,
            (*metadata).icon_mime,
            b"\"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = buffer;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_20
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
                buffer,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(buffer as gpointer);
    }
    if (*metadata).is_private() != 0 {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"        <bookmark:private/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_21
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
                b"        <bookmark:private/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"      </metadata>\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
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
            retval,
            b"      </metadata>\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
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
unsafe extern "C" fn safe_c2rust_bookmark_item_new(mut uri: *const gchar) -> *mut BookmarkItem {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if !(({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            526 as ::core::ffi::c_int,
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    item = g_slice_alloc(::core::mem::size_of::<BookmarkItem>() as gsize) as *mut BookmarkItem;
    (*item).uri = safe_c2rust_g_strdup_inline(uri as *const ::core::ffi::c_char) as *mut gchar;
    (*item).title = ::core::ptr::null_mut::<gchar>();
    (*item).description = ::core::ptr::null_mut::<gchar>();
    (*item).added = ::core::ptr::null_mut::<GDateTime>();
    (*item).modified = ::core::ptr::null_mut::<GDateTime>();
    (*item).visited = ::core::ptr::null_mut::<GDateTime>();
    (*item).metadata = ::core::ptr::null_mut::<BookmarkMetadata>();
    return item;
}
unsafe extern "C" fn safe_c2rust_bookmark_item_free(mut item: *mut BookmarkItem) {
    if item.is_null() {
        return;
    }
    g_free((*item).uri as gpointer);
    g_free((*item).title as gpointer);
    g_free((*item).description as gpointer);
    if !(*item).metadata.is_null() {
        safe_c2rust_bookmark_metadata_free((*item).metadata);
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*item).added;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    let mut _pp_0: *mut *mut GDateTime = &raw mut (*item).modified;
    let mut _ptr_0: *mut GDateTime = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr_0.is_null() {
        g_date_time_unref(_ptr_0 as *mut GDateTime);
    }
    let mut _pp_1: *mut *mut GDateTime = &raw mut (*item).visited;
    let mut _ptr_1: *mut GDateTime = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr_1.is_null() {
        g_date_time_unref(_ptr_1 as *mut GDateTime);
    }
    g_slice_free1(
        ::core::mem::size_of::<BookmarkItem>() as gsize,
        item as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_bookmark_item_copy(
    mut item: *mut BookmarkItem,
) -> *mut BookmarkItem {
    let mut copy: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if item.is_null() {
        return ::core::ptr::null_mut::<BookmarkItem>();
    }
    copy = safe_c2rust_bookmark_item_new((*item).uri);
    (*copy).title = safe_c2rust_g_strdup_inline((*item).title) as *mut gchar;
    (*copy).description = safe_c2rust_g_strdup_inline((*item).description) as *mut gchar;
    (*copy).metadata = safe_c2rust_bookmark_metadata_copy((*item).metadata);
    if !(*item).added.is_null() {
        (*copy).added = g_date_time_ref((*item).added);
    }
    if !(*item).modified.is_null() {
        (*copy).modified = g_date_time_ref((*item).modified);
    }
    if !(*item).visited.is_null() {
        (*copy).visited = g_date_time_ref((*item).visited);
    }
    return copy;
}
unsafe extern "C" fn safe_c2rust_bookmark_item_touch_modified(mut item: *mut BookmarkItem) {
    let mut _pp: *mut *mut GDateTime = &raw mut (*item).modified;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    (*item).modified = g_date_time_new_now_utc();
}
unsafe extern "C" fn safe_c2rust_bookmark_item_dump(mut item: *mut BookmarkItem) -> *mut gchar {
    let mut retval: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut escaped_uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if (*item).metadata.is_null() || (*(*item).metadata).applications.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Item for URI '%s' has no registered applications: skipping.\0" as *const u8
                as *const gchar,
            (*item).uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    retval = g_string_sized_new(4096 as gsize);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"  <bookmark \0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_24
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
            retval,
            b"  <bookmark \0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    escaped_uri = g_markup_escape_text((*item).uri, -(1 as ::core::ffi::c_int) as gssize);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"href=\"\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_25
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
            retval,
            b"href=\"\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = escaped_uri;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_26
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
            retval,
            escaped_uri,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"\" \0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_27
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
            retval,
            b"\" \0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_free(escaped_uri as gpointer);
    if !(*item).added.is_null() {
        let mut added: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        added = g_date_time_format_iso8601((*item).added) as *mut ::core::ffi::c_char;
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"added=\"\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_28
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
                b"added=\"\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = added;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_29
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
                added,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\" \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"\" \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(added as gpointer);
    }
    if !(*item).modified.is_null() {
        let mut modified: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        modified = g_date_time_format_iso8601((*item).modified) as *mut ::core::ffi::c_char;
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"modified=\"\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"modified=\"\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = modified;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                modified,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\" \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"\" \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(modified as gpointer);
    }
    if !(*item).visited.is_null() {
        let mut visited: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        visited = g_date_time_format_iso8601((*item).visited) as *mut ::core::ffi::c_char;
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"visited=\"\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                b"visited=\"\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = visited;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                visited,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\" \0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"\" \0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(visited as gpointer);
    }
    if *(*retval)
        .str_0
        .offset((*retval).len.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
        == ' ' as i32
    {
        safe_c2rust_g_string_truncate_inline(retval, (*retval).len.wrapping_sub(1 as gsize));
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b">\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            retval,
            b">\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(*item).title.is_null() {
        let mut escaped_title: *mut gchar = ::core::ptr::null_mut::<gchar>();
        escaped_title = g_markup_escape_text((*item).title, -(1 as ::core::ffi::c_int) as gssize);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"    <title>\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"    <title>\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = escaped_title;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                escaped_title,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"</title>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"</title>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(escaped_title as gpointer);
    }
    if !(*item).description.is_null() {
        let mut escaped_desc: *mut gchar = ::core::ptr::null_mut::<gchar>();
        escaped_desc =
            g_markup_escape_text((*item).description, -(1 as ::core::ffi::c_int) as gssize);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"    <desc>\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"    <desc>\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = escaped_desc;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                escaped_desc,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"</desc>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
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
                retval,
                b"</desc>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(escaped_desc as gpointer);
    }
    if !(*item).metadata.is_null() {
        let mut metadata: *mut gchar = ::core::ptr::null_mut::<gchar>();
        metadata = safe_c2rust_bookmark_metadata_dump((*item).metadata);
        if !metadata.is_null() {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"    <info>\n\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        retval,
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
                    retval,
                    b"    <info>\n\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = metadata;
                    safe_c2rust_g_string_append_len_inline(
                        retval,
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
                    retval,
                    metadata,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"    </info>\n\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        retval,
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
                    retval,
                    b"    </info>\n\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            g_free(metadata as gpointer);
        }
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"  </bookmark>\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            retval,
            b"  </bookmark>\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
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
unsafe extern "C" fn safe_c2rust_bookmark_item_lookup_app_info(
    mut item: *mut BookmarkItem,
    mut app_name: *const gchar,
) -> *mut BookmarkAppInfo {
    if !(({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !item.is_null() && !app_name.is_null() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            707 as ::core::ffi::c_int,
            G_STRFUNC,
            b"item != NULL && app_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*item).metadata.is_null() {
        return ::core::ptr::null_mut::<BookmarkAppInfo>();
    }
    return g_hash_table_lookup((*(*item).metadata).apps_by_name, app_name as gconstpointer)
        as *mut BookmarkAppInfo;
}
unsafe extern "C" fn safe_c2rust_g_bookmark_file_init(mut bookmark: *mut GBookmarkFile) {
    (*bookmark).title = ::core::ptr::null_mut::<gchar>();
    (*bookmark).description = ::core::ptr::null_mut::<gchar>();
    (*bookmark).items = ::core::ptr::null_mut::<GList>();
    (*bookmark).items_by_uri = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        None,
    );
}
unsafe extern "C" fn safe_c2rust_g_bookmark_file_clear(mut bookmark: *mut GBookmarkFile) {
    g_free((*bookmark).title as gpointer);
    g_free((*bookmark).description as gpointer);
    g_list_free_full(
        (*bookmark).items,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BookmarkItem) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_bookmark_item_free as unsafe extern "C" fn(*mut BookmarkItem) -> (),
        )),
    );
    (*bookmark).items = ::core::ptr::null_mut::<GList>();
    let mut _pp: *mut *mut GHashTable = &raw mut (*bookmark).items_by_uri;
    let mut _ptr: *mut GHashTable = *_pp;
    *_pp = ::core::ptr::null_mut::<GHashTable>();
    if !_ptr.is_null() {
        g_hash_table_unref(_ptr as *mut GHashTable);
    }
}
unsafe extern "C" fn safe_c2rust_parse_data_new() -> *mut ParseData {
    let mut retval: *mut ParseData = ::core::ptr::null_mut::<ParseData>();
    retval = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ParseData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut ParseData;
    (*retval).state = STATE_STARTED;
    (*retval).namespaces = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
    (*retval).bookmark_file = ::core::ptr::null_mut::<GBookmarkFile>();
    (*retval).current_item = ::core::ptr::null_mut::<BookmarkItem>();
    return retval;
}
unsafe extern "C" fn safe_c2rust_parse_data_free(mut parse_data: *mut ParseData) {
    g_hash_table_destroy((*parse_data).namespaces);
    g_free(parse_data as gpointer);
}
unsafe extern "C" fn safe_c2rust_parse_bookmark_element(
    mut context: *mut GMarkupParseContext,
    mut parse_data: *mut ParseData,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut error: *mut *mut GError,
) {
    let mut uri: *const gchar = ::core::ptr::null::<gchar>();
    let mut added: *const gchar = ::core::ptr::null::<gchar>();
    let mut modified: *const gchar = ::core::ptr::null::<gchar>();
    let mut visited: *const gchar = ::core::ptr::null::<gchar>();
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut add_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if !(({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !parse_data.is_null()
            && (*parse_data).state as ::core::ffi::c_uint
                == STATE_BOOKMARK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            794 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(parse_data != NULL) && (parse_data->state == STATE_BOOKMARK)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    visited = ::core::ptr::null::<gchar>();
    modified = visited;
    added = modified;
    uri = added;
    attr = *attribute_names.offset(i as isize);
    while !attr.is_null() {
        if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"href\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            uri = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"added\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            added = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            modified = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"visited\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            visited = *attribute_values.offset(i as isize);
        } else {
            g_set_error(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unexpected attribute \xE2\x80\x9C%s\xE2\x80\x9D for element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                attr,
                XBEL_BOOKMARK_ELEMENT.as_ptr(),
            );
            return;
        }
        i += 1;
        attr = *attribute_names.offset(i as isize);
    }
    if uri.is_null() {
        g_set_error(
            error,
            g_markup_error_quark(),
            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Attribute \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D not found\0"
                    as *const u8 as *const gchar,
            ),
            XBEL_HREF_ATTRIBUTE.as_ptr(),
            XBEL_BOOKMARK_ELEMENT.as_ptr(),
        );
        return;
    }
    if !(({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if (*parse_data).current_item.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data->current_item == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    item = safe_c2rust_bookmark_item_new(uri);
    if !added.is_null()
        && safe_c2rust_timestamp_from_iso8601(added, &raw mut (*item).added, error) == 0
    {
        safe_c2rust_bookmark_item_free(item);
        return;
    }
    if !modified.is_null()
        && safe_c2rust_timestamp_from_iso8601(modified, &raw mut (*item).modified, error) == 0
    {
        safe_c2rust_bookmark_item_free(item);
        return;
    }
    if !visited.is_null()
        && safe_c2rust_timestamp_from_iso8601(visited, &raw mut (*item).visited, error) == 0
    {
        safe_c2rust_bookmark_item_free(item);
        return;
    }
    add_error = ::core::ptr::null_mut::<GError>();
    safe_c2rust_g_bookmark_file_add_item((*parse_data).bookmark_file, item, &raw mut add_error);
    if !add_error.is_null() {
        safe_c2rust_bookmark_item_free(item);
        g_propagate_error(error, add_error);
        return;
    }
    (*parse_data).current_item = item;
}
unsafe extern "C" fn safe_c2rust_parse_application_element(
    mut context: *mut GMarkupParseContext,
    mut parse_data: *mut ParseData,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut error: *mut *mut GError,
) {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut exec: *const gchar = ::core::ptr::null::<gchar>();
    let mut count: *const gchar = ::core::ptr::null::<gchar>();
    let mut stamp: *const gchar = ::core::ptr::null::<gchar>();
    let mut modified: *const gchar = ::core::ptr::null::<gchar>();
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut ai: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
    if !(({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !parse_data.is_null()
            && (*parse_data).state as ::core::ffi::c_uint
                == STATE_APPLICATION as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            884 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(parse_data != NULL) && (parse_data->state == STATE_APPLICATION)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    modified = ::core::ptr::null::<gchar>();
    stamp = modified;
    count = stamp;
    exec = count;
    name = exec;
    attr = *attribute_names.offset(i as isize);
    while !attr.is_null() {
        if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"name\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            name = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"exec\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            exec = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"count\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            count = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"timestamp\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            stamp = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            modified = *attribute_values.offset(i as isize);
        }
        i += 1;
        attr = *attribute_names.offset(i as isize);
    }
    if name.is_null() {
        g_set_error(
            error,
            g_markup_error_quark(),
            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Attribute \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D not found\0"
                    as *const u8 as *const gchar,
            ),
            BOOKMARK_NAME_ATTRIBUTE.as_ptr(),
            BOOKMARK_APPLICATION_ELEMENT.as_ptr(),
        );
        return;
    }
    if exec.is_null() {
        g_set_error(
            error,
            g_markup_error_quark(),
            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Attribute \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D not found\0"
                    as *const u8 as *const gchar,
            ),
            BOOKMARK_EXEC_ATTRIBUTE.as_ptr(),
            BOOKMARK_APPLICATION_ELEMENT.as_ptr(),
        );
        return;
    }
    if !(({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !(*parse_data).current_item.is_null() {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            923 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data->current_item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    item = (*parse_data).current_item;
    ai = safe_c2rust_bookmark_item_lookup_app_info(item, name);
    if ai.is_null() {
        ai = safe_c2rust_bookmark_app_info_new(name);
        if (*item).metadata.is_null() {
            (*item).metadata = safe_c2rust_bookmark_metadata_new();
        }
        (*(*item).metadata).applications =
            g_list_prepend((*(*item).metadata).applications, ai as gpointer);
        g_hash_table_replace(
            (*(*item).metadata).apps_by_name,
            (*ai).name as gpointer,
            ai as gpointer,
        );
    }
    g_free((*ai).exec as gpointer);
    (*ai).exec = safe_c2rust_g_strdup_inline(exec as *const ::core::ffi::c_char) as *mut gchar;
    if !count.is_null() {
        (*ai).count = safe_c2rust_atoi(count as *const ::core::ffi::c_char) as guint;
    } else {
        (*ai).count = 1 as guint;
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*ai).stamp;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    if !modified.is_null() {
        if safe_c2rust_timestamp_from_iso8601(modified, &raw mut (*ai).stamp, error) == 0 {
            return;
        }
    } else if !stamp.is_null() {
        (*ai).stamp = g_date_time_new_from_unix_utc(safe_c2rust_atol(
            stamp as *const ::core::ffi::c_char,
        ) as gint64);
    } else {
        (*ai).stamp = g_date_time_new_now_utc();
    };
}
unsafe extern "C" fn safe_c2rust_parse_mime_type_element(
    mut context: *mut GMarkupParseContext,
    mut parse_data: *mut ParseData,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut error: *mut *mut GError,
) {
    let mut type_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if !(({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !parse_data.is_null()
            && (*parse_data).state as ::core::ffi::c_uint
                == STATE_MIME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            976 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(parse_data != NULL) && (parse_data->state == STATE_MIME)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    type_0 = ::core::ptr::null::<gchar>();
    attr = *attribute_names.offset(i as isize);
    while !attr.is_null() {
        if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"type\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            type_0 = *attribute_values.offset(i as isize);
        }
        i += 1;
        attr = *attribute_names.offset(i as isize);
    }
    if type_0.is_null() {
        type_0 = b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char
            as *const gchar;
    }
    if !(({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !(*parse_data).current_item.is_null() {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            989 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data->current_item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    item = (*parse_data).current_item;
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    g_free((*(*item).metadata).mime_type as gpointer);
    (*(*item).metadata).mime_type =
        safe_c2rust_g_strdup_inline(type_0 as *const ::core::ffi::c_char) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_parse_icon_element(
    mut context: *mut GMarkupParseContext,
    mut parse_data: *mut ParseData,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut error: *mut *mut GError,
) {
    let mut href: *const gchar = ::core::ptr::null::<gchar>();
    let mut type_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if !(({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !parse_data.is_null()
            && (*parse_data).state as ::core::ffi::c_uint
                == STATE_ICON as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1012 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(parse_data != NULL) && (parse_data->state == STATE_ICON)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    href = ::core::ptr::null::<gchar>();
    type_0 = ::core::ptr::null::<gchar>();
    attr = *attribute_names.offset(i as isize);
    while !attr.is_null() {
        if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"href\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            href = *attribute_values.offset(i as isize);
        } else if 0 as ::core::ffi::c_int
            == strcmp(
                attr as *const ::core::ffi::c_char,
                b"type\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            type_0 = *attribute_values.offset(i as isize);
        }
        i += 1;
        attr = *attribute_names.offset(i as isize);
    }
    if href.is_null() {
        g_set_error(
            error,
            g_markup_error_quark(),
            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Attribute \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D not found\0"
                    as *const u8 as *const gchar,
            ),
            BOOKMARK_HREF_ATTRIBUTE.as_ptr(),
            BOOKMARK_ICON_ELEMENT.as_ptr(),
        );
        return;
    }
    if type_0.is_null() {
        type_0 = b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char
            as *const gchar;
    }
    if !(({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !(*parse_data).current_item.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1039 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data->current_item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    item = (*parse_data).current_item;
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    g_free((*(*item).metadata).icon_href as gpointer);
    g_free((*(*item).metadata).icon_mime as gpointer);
    (*(*item).metadata).icon_href =
        safe_c2rust_g_strdup_inline(href as *const ::core::ffi::c_char) as *mut gchar;
    (*(*item).metadata).icon_mime =
        safe_c2rust_g_strdup_inline(type_0 as *const ::core::ffi::c_char) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_map_namespace_to_name(
    mut parse_data: *mut ParseData,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
) {
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    if !(({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !parse_data.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1072 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if attribute_names.is_null()
        || (*attribute_names.offset(0 as ::core::ffi::c_int as isize)).is_null()
    {
        return;
    }
    i = 0 as ::core::ffi::c_int as gint;
    attr = *attribute_names.offset(i as isize);
    while !attr.is_null() {
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = attr as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"xmlns\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_58 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_58 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_58
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(attr, b"xmlns\0" as *const u8 as *const gchar)
        } != 0
        {
            let mut namespace_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut namespace_uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
            p = g_utf8_strchr(
                attr,
                -(1 as ::core::ffi::c_int) as gssize,
                ':' as i32 as gunichar,
            );
            if !p.is_null() {
                p = p.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char as *mut gchar;
            } else {
                p = b"default\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            }
            namespace_name = safe_c2rust_g_strdup_inline(p) as *mut gchar;
            namespace_uri = safe_c2rust_g_strdup_inline(
                *attribute_values.offset(i as isize) as *const ::core::ffi::c_char
            ) as *mut gchar;
            g_hash_table_replace(
                (*parse_data).namespaces,
                namespace_name as gpointer,
                namespace_uri as gpointer,
            );
        }
        i += 1;
        attr = *attribute_names.offset(i as isize);
    }
}
unsafe extern "C" fn safe_c2rust_is_element_full(
    mut parse_data: *mut ParseData,
    mut element_full: *const gchar,
    mut namespace: *const gchar,
    mut element: *const gchar,
    sep: gchar,
) -> gboolean {
    let mut ns_uri: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ns_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut element_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut retval: gboolean = 0;
    if !(({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !parse_data.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1118 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parse_data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !element_full.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1119 as ::core::ffi::c_int,
            G_STRFUNC,
            b"element_full != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if element.is_null() {
        return FALSE;
    }
    if namespace.is_null() {
        return (0 as ::core::ffi::c_int
            == strcmp(
                element_full as *const ::core::ffi::c_char,
                element as *const ::core::ffi::c_char,
            )) as ::core::ffi::c_int;
    }
    p = g_utf8_strchr(
        element_full,
        -(1 as ::core::ffi::c_int) as gssize,
        ':' as i32 as gunichar,
    );
    if !p.is_null() {
        ns_name = g_strndup(
            element_full,
            p.offset_from(element_full) as ::core::ffi::c_long as gsize,
        );
        element_name = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    } else {
        ns_name =
            safe_c2rust_g_strdup_inline(b"default\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut gchar;
        element_name = element_full;
    }
    ns_uri = g_hash_table_lookup((*parse_data).namespaces, ns_name as gconstpointer) as *mut gchar;
    if ns_uri.is_null() {
        g_free(ns_name as gpointer);
        return (0 as ::core::ffi::c_int
            == strcmp(
                element_full as *const ::core::ffi::c_char,
                element as *const ::core::ffi::c_char,
            )) as ::core::ffi::c_int;
    }
    retval = (0 as ::core::ffi::c_int == strcmp(ns_uri, namespace as *const ::core::ffi::c_char)
        && 0 as ::core::ffi::c_int
            == strcmp(
                element_name as *const ::core::ffi::c_char,
                element as *const ::core::ffi::c_char,
            )) as ::core::ffi::c_int as gboolean;
    g_free(ns_name as gpointer);
    return retval;
}
unsafe extern "C" fn safe_c2rust_parser_state_to_element_name(
    mut state: ParserState,
) -> *const gchar {
    match state as ::core::ffi::c_uint {
        0 | 13 => return b"(top-level)\0" as *const u8 as *const gchar,
        1 => return XBEL_ROOT_ELEMENT.as_ptr() as *const gchar,
        2 => return XBEL_BOOKMARK_ELEMENT.as_ptr() as *const gchar,
        3 => return XBEL_TITLE_ELEMENT.as_ptr() as *const gchar,
        4 => return XBEL_DESC_ELEMENT.as_ptr() as *const gchar,
        5 => return XBEL_INFO_ELEMENT.as_ptr() as *const gchar,
        6 => return XBEL_METADATA_ELEMENT.as_ptr() as *const gchar,
        7 => return BOOKMARK_APPLICATIONS_ELEMENT.as_ptr() as *const gchar,
        8 => return BOOKMARK_APPLICATION_ELEMENT.as_ptr() as *const gchar,
        9 => return BOOKMARK_GROUPS_ELEMENT.as_ptr() as *const gchar,
        10 => return BOOKMARK_GROUP_ELEMENT.as_ptr() as *const gchar,
        11 => return MIME_TYPE_ELEMENT.as_ptr() as *const gchar,
        12 => return BOOKMARK_ICON_ELEMENT.as_ptr() as *const gchar,
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                1198 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_start_element_raw_cb(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut parse_data: *mut ParseData = user_data as *mut ParseData;
    safe_c2rust_map_namespace_to_name(parse_data, attribute_names, attribute_values);
    match (*parse_data).state as ::core::ffi::c_uint {
        0 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"xbel\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                let mut attr: *const gchar = ::core::ptr::null::<gchar>();
                let mut i: gint = 0;
                i = 0 as ::core::ffi::c_int as gint;
                attr = *attribute_names.offset(i as isize);
                while !attr.is_null() {
                    if 0 as ::core::ffi::c_int
                        == strcmp(
                            attr as *const ::core::ffi::c_char,
                            b"version\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        && 0 as ::core::ffi::c_int
                            == strcmp(
                                *attribute_values.offset(i as isize) as *const ::core::ffi::c_char,
                                XBEL_VERSION.as_ptr(),
                            )
                    {
                        (*parse_data).state = STATE_ROOT;
                    }
                    i += 1;
                    attr = *attribute_names.offset(i as isize);
                }
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D, tag \xE2\x80\x9C%s\xE2\x80\x9D expected\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    XBEL_ROOT_ELEMENT.as_ptr(),
                );
            }
        }
        1 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"title\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_TITLE;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"desc\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_DESC;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"bookmark\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                let mut inner_error: *mut GError = ::core::ptr::null_mut::<GError>();
                (*parse_data).state = STATE_BOOKMARK;
                safe_c2rust_parse_bookmark_element(
                    context,
                    parse_data,
                    attribute_names,
                    attribute_values,
                    &raw mut inner_error,
                );
                if !inner_error.is_null() {
                    g_propagate_error(error, inner_error);
                }
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D inside \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    XBEL_ROOT_ELEMENT.as_ptr(),
                );
            }
        }
        2 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"title\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_TITLE;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"desc\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_DESC;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"info\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_INFO;
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D inside \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    XBEL_BOOKMARK_ELEMENT.as_ptr(),
                );
            }
        }
        5 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                ::core::ptr::null::<gchar>(),
                b"metadata\0" as *const u8 as *const gchar,
                '\0' as i32 as gchar,
            ) != 0
            {
                let mut attr_0: *const gchar = ::core::ptr::null::<gchar>();
                let mut i_0: gint = 0;
                i_0 = 0 as ::core::ffi::c_int as gint;
                attr_0 = *attribute_names.offset(i_0 as isize);
                while !attr_0.is_null() {
                    if 0 as ::core::ffi::c_int
                        == strcmp(
                            attr_0 as *const ::core::ffi::c_char,
                            b"owner\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        && 0 as ::core::ffi::c_int
                            == strcmp(
                                *attribute_values.offset(i_0 as isize)
                                    as *const ::core::ffi::c_char,
                                BOOKMARK_METADATA_OWNER.as_ptr(),
                            )
                    {
                        (*parse_data).state = STATE_METADATA;
                        if (*(*parse_data).current_item).metadata.is_null() {
                            (*(*parse_data).current_item).metadata =
                                safe_c2rust_bookmark_metadata_new();
                        }
                    }
                    i_0 += 1;
                    attr_0 = *attribute_names.offset(i_0 as isize);
                }
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D, tag \xE2\x80\x9C%s\xE2\x80\x9D expected\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    XBEL_METADATA_ELEMENT.as_ptr(),
                );
            }
        }
        6 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"applications\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_APPLICATIONS;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"groups\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_GROUPS;
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"private\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                (*(*(*parse_data).current_item).metadata).set_is_private(TRUE as guint as guint);
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"icon\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                let mut inner_error_0: *mut GError = ::core::ptr::null_mut::<GError>();
                (*parse_data).state = STATE_ICON;
                safe_c2rust_parse_icon_element(
                    context,
                    parse_data,
                    attribute_names,
                    attribute_values,
                    &raw mut inner_error_0,
                );
                if !inner_error_0.is_null() {
                    g_propagate_error(error, inner_error_0);
                }
            } else if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/shared-mime-info\0" as *const u8
                    as *const gchar,
                b"mime-type\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                let mut inner_error_1: *mut GError = ::core::ptr::null_mut::<GError>();
                (*parse_data).state = STATE_MIME;
                safe_c2rust_parse_mime_type_element(
                    context,
                    parse_data,
                    attribute_names,
                    attribute_values,
                    &raw mut inner_error_1,
                );
                if !inner_error_1.is_null() {
                    g_propagate_error(error, inner_error_1);
                }
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_UNKNOWN_ELEMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D inside \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    XBEL_METADATA_ELEMENT.as_ptr(),
                );
            }
        }
        7 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"application\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                let mut inner_error_2: *mut GError = ::core::ptr::null_mut::<GError>();
                (*parse_data).state = STATE_APPLICATION;
                safe_c2rust_parse_application_element(
                    context,
                    parse_data,
                    attribute_names,
                    attribute_values,
                    &raw mut inner_error_2,
                );
                if !inner_error_2.is_null() {
                    g_propagate_error(error, inner_error_2);
                }
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D, tag \xE2\x80\x9C%s\xE2\x80\x9D expected\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    BOOKMARK_APPLICATION_ELEMENT.as_ptr(),
                );
            }
        }
        9 => {
            if safe_c2rust_is_element_full(
                parse_data,
                element_name,
                b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                    as *const gchar,
                b"group\0" as *const u8 as *const gchar,
                '|' as i32 as gchar,
            ) != 0
            {
                (*parse_data).state = STATE_GROUP;
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D, tag \xE2\x80\x9C%s\xE2\x80\x9D expected\0"
                            as *const u8 as *const gchar,
                    ),
                    element_name,
                    BOOKMARK_GROUP_ELEMENT.as_ptr(),
                );
            }
        }
        3 | 4 | 8 | 10 | 11 | 12 | 13 => {
            g_set_error(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unexpected tag \xE2\x80\x9C%s\xE2\x80\x9D inside \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                element_name,
                safe_c2rust_parser_state_to_element_name((*parse_data).state),
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                1398 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_end_element_raw_cb(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut parse_data: *mut ParseData = user_data as *mut ParseData;
    if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        ::core::ptr::null::<gchar>(),
        b"xbel\0" as *const u8 as *const gchar,
        '\0' as i32 as gchar,
    ) != 0
    {
        (*parse_data).state = STATE_FINISHED;
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        ::core::ptr::null::<gchar>(),
        b"bookmark\0" as *const u8 as *const gchar,
        '\0' as i32 as gchar,
    ) != 0
    {
        (*parse_data).current_item = ::core::ptr::null_mut::<BookmarkItem>();
        (*parse_data).state = STATE_ROOT;
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        ::core::ptr::null::<gchar>(),
        b"info\0" as *const u8 as *const gchar,
        '\0' as i32 as gchar,
    ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            ::core::ptr::null::<gchar>(),
            b"title\0" as *const u8 as *const gchar,
            '\0' as i32 as gchar,
        ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            ::core::ptr::null::<gchar>(),
            b"desc\0" as *const u8 as *const gchar,
            '\0' as i32 as gchar,
        ) != 0
    {
        if !(*parse_data).current_item.is_null() {
            (*parse_data).state = STATE_BOOKMARK;
        } else {
            (*parse_data).state = STATE_ROOT;
        }
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        ::core::ptr::null::<gchar>(),
        b"metadata\0" as *const u8 as *const gchar,
        '\0' as i32 as gchar,
    ) != 0
    {
        (*parse_data).state = STATE_INFO;
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8 as *const gchar,
        b"application\0" as *const u8 as *const gchar,
        '|' as i32 as gchar,
    ) != 0
    {
        (*parse_data).state = STATE_APPLICATIONS;
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8 as *const gchar,
        b"group\0" as *const u8 as *const gchar,
        '|' as i32 as gchar,
    ) != 0
    {
        (*parse_data).state = STATE_GROUPS;
    } else if safe_c2rust_is_element_full(
        parse_data,
        element_name,
        b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8 as *const gchar,
        b"applications\0" as *const u8 as *const gchar,
        '|' as i32 as gchar,
    ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                as *const gchar,
            b"groups\0" as *const u8 as *const gchar,
            '|' as i32 as gchar,
        ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                as *const gchar,
            b"private\0" as *const u8 as *const gchar,
            '|' as i32 as gchar,
        ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            b"http://www.freedesktop.org/standards/desktop-bookmarks\0" as *const u8
                as *const gchar,
            b"icon\0" as *const u8 as *const gchar,
            '|' as i32 as gchar,
        ) != 0
        || safe_c2rust_is_element_full(
            parse_data,
            element_name,
            b"http://www.freedesktop.org/standards/shared-mime-info\0" as *const u8 as *const gchar,
            b"mime-type\0" as *const u8 as *const gchar,
            '|' as i32 as gchar,
        ) != 0
    {
        (*parse_data).state = STATE_METADATA;
    }
}
unsafe extern "C" fn safe_c2rust_text_raw_cb(
    mut context: *mut GMarkupParseContext,
    mut text: *const gchar,
    mut length: gsize,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut parse_data: *mut ParseData = user_data as *mut ParseData;
    let mut payload: *mut gchar = ::core::ptr::null_mut::<gchar>();
    payload = g_strndup(text, length);
    match (*parse_data).state as ::core::ffi::c_uint {
        3 => {
            if !(*parse_data).current_item.is_null() {
                g_free((*(*parse_data).current_item).title as gpointer);
                (*(*parse_data).current_item).title =
                    safe_c2rust_g_strdup_inline(payload) as *mut gchar;
            } else {
                g_free((*(*parse_data).bookmark_file).title as gpointer);
                (*(*parse_data).bookmark_file).title =
                    safe_c2rust_g_strdup_inline(payload) as *mut gchar;
            }
        }
        4 => {
            if !(*parse_data).current_item.is_null() {
                g_free((*(*parse_data).current_item).description as gpointer);
                (*(*parse_data).current_item).description =
                    safe_c2rust_g_strdup_inline(payload) as *mut gchar;
            } else {
                g_free((*(*parse_data).bookmark_file).description as gpointer);
                (*(*parse_data).bookmark_file).description =
                    safe_c2rust_g_strdup_inline(payload) as *mut gchar;
            }
        }
        10 => {
            let mut groups: *mut GList = ::core::ptr::null_mut::<GList>();
            if !(({
                let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
                if !(*parse_data).current_item.is_null() {
                    _g_boolean_var_61 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_61 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_61
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gbookmarkfile.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1488 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"parse_data->current_item != NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if (*(*parse_data).current_item).metadata.is_null() {
                (*(*parse_data).current_item).metadata = safe_c2rust_bookmark_metadata_new();
            }
            groups = (*(*(*parse_data).current_item).metadata).groups;
            (*(*(*parse_data).current_item).metadata).groups =
                g_list_prepend(groups, safe_c2rust_g_strdup_inline(payload) as gpointer);
        }
        1 | 2 | 5 | 6 | 7 | 8 | 9 | 11 | 12 => {}
        _ => {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                1508 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    g_free(payload as gpointer);
}
static mut safe_c2rust_markup_parser: GMarkupParser = unsafe {
    _GMarkupParser {
        start_element: Some(
            safe_c2rust_start_element_raw_cb
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    *mut *const gchar,
                    *mut *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        end_element: Some(
            safe_c2rust_end_element_raw_cb
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        text: Some(
            safe_c2rust_text_raw_cb
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gsize,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        passthrough: None,
        error: None,
    }
};
unsafe extern "C" fn safe_c2rust_g_bookmark_file_parse(
    mut bookmark: *mut GBookmarkFile,
    mut buffer: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut context: *mut GMarkupParseContext = ::core::ptr::null_mut::<GMarkupParseContext>();
    let mut parse_data: *mut ParseData = ::core::ptr::null_mut::<ParseData>();
    let mut parse_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut end_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut retval: gboolean = 0;
    if !(({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1535 as ::core::ffi::c_int,
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if buffer.is_null() {
        return FALSE;
    }
    parse_error = ::core::ptr::null_mut::<GError>();
    end_error = ::core::ptr::null_mut::<GError>();
    if length == -(1 as ::core::ffi::c_int) as gsize {
        length = strlen(buffer as *const ::core::ffi::c_char) as gsize;
    }
    parse_data = safe_c2rust_parse_data_new();
    (*parse_data).bookmark_file = bookmark;
    context = g_markup_parse_context_new(
        &raw const safe_c2rust_markup_parser,
        G_MARKUP_DEFAULT_FLAGS,
        parse_data as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut ParseData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_parse_data_free as unsafe extern "C" fn(*mut ParseData) -> ()),
        ),
    );
    retval = g_markup_parse_context_parse(context, buffer, length as gssize, &raw mut parse_error);
    if retval == 0 {
        g_propagate_error(error, parse_error);
    } else {
        retval = g_markup_parse_context_end_parse(context, &raw mut end_error);
        if retval == 0 {
            g_propagate_error(error, end_error);
        }
    }
    g_markup_parse_context_free(context);
    return retval;
}
unsafe extern "C" fn safe_c2rust_g_bookmark_file_dump(
    mut bookmark: *mut GBookmarkFile,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut retval: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    retval = g_string_sized_new(4096 as gsize);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<xbel version=\"1.0\"\n      xmlns:bookmark=\"http://www.freedesktop.org/standards/desktop-bookmarks\"\n      xmlns:mime=\"http://www.freedesktop.org/standards/shared-mime-info\"\n>\0"
                as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_63 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_63 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_63
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
            retval,
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<xbel version=\"1.0\"\n      xmlns:bookmark=\"http://www.freedesktop.org/standards/desktop-bookmarks\"\n      xmlns:mime=\"http://www.freedesktop.org/standards/shared-mime-info\"\n>\0"
                as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(*bookmark).title.is_null() {
        let mut escaped_title: *mut gchar = ::core::ptr::null_mut::<gchar>();
        escaped_title =
            g_markup_escape_text((*bookmark).title, -(1 as ::core::ffi::c_int) as gssize);
        buffer = g_strconcat(
            b"  <title>\0" as *const u8 as *const gchar,
            escaped_title,
            b"</title>\n\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = buffer;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_64
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
                buffer,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(buffer as gpointer);
        g_free(escaped_title as gpointer);
    }
    if !(*bookmark).description.is_null() {
        let mut escaped_desc: *mut gchar = ::core::ptr::null_mut::<gchar>();
        escaped_desc = g_markup_escape_text(
            (*bookmark).description,
            -(1 as ::core::ffi::c_int) as gssize,
        );
        buffer = g_strconcat(
            b"  <desc>\0" as *const u8 as *const gchar,
            escaped_desc,
            b"</desc>\n\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = buffer;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_65
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
                buffer,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(buffer as gpointer);
        g_free(escaped_desc as gpointer);
    }
    if !(*bookmark).items.is_null() {
        retval = if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    retval,
                    __val,
                    if ({
                        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_66
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                )
            })
        } else {
            safe_c2rust_g_string_append_len_inline(
                retval,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            )
        };
        l = g_list_last((*bookmark).items);
        while !l.is_null() {
            let mut item: *mut BookmarkItem = (*l).data as *mut BookmarkItem;
            let mut item_dump: *mut gchar = ::core::ptr::null_mut::<gchar>();
            item_dump = safe_c2rust_bookmark_item_dump(item);
            if !item_dump.is_null() {
                retval = if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char = item_dump;
                        safe_c2rust_g_string_append_len_inline(
                            retval,
                            __val,
                            if ({
                                let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_67 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_67 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_67
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        )
                    })
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        retval,
                        item_dump,
                        -(1 as ::core::ffi::c_int) as gssize,
                    )
                };
                g_free(item_dump as gpointer);
            }
            l = (*l).prev;
        }
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"</xbel>\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                retval,
                __val,
                if ({
                    let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_68 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_68 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_68
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
            retval,
            b"</xbel>\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !length.is_null() {
        *length = (*retval).len;
    }
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
unsafe extern "C" fn safe_c2rust_timestamp_from_iso8601(
    mut iso_date: *const gchar,
    mut out_date_time: *mut *mut GDateTime,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut dt: *mut GDateTime =
        g_date_time_new_from_iso8601(iso_date, ::core::ptr::null_mut::<GTimeZone>());
    if dt.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_READ as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid date/time \xE2\x80\x98%s\xE2\x80\x99 in bookmark file\0" as *const u8
                    as *const gchar,
            ),
            iso_date,
        );
        return FALSE;
    }
    *out_date_time =
        safe_c2rust_g_steal_pointer(&raw mut dt as gpointer) as *mut GDateTime as *mut GDateTime;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q = g_quark_from_static_string(
            b"g-bookmark-file-error-quark\0" as *const u8 as *const gchar,
        );
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_new() -> *mut GBookmarkFile {
    let mut bookmark: *mut GBookmarkFile = ::core::ptr::null_mut::<GBookmarkFile>();
    bookmark = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GBookmarkFile>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GBookmarkFile;
    safe_c2rust_g_bookmark_file_init(bookmark);
    return bookmark;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_copy(
    mut bookmark: *mut GBookmarkFile,
) -> *mut GBookmarkFile {
    let mut copy: *mut GBookmarkFile = ::core::ptr::null_mut::<GBookmarkFile>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBookmarkFile>();
    }
    copy = safe_c2rust_g_bookmark_file_new();
    (*copy).title = safe_c2rust_g_strdup_inline((*bookmark).title) as *mut gchar;
    (*copy).description = safe_c2rust_g_strdup_inline((*bookmark).description) as *mut gchar;
    (*copy).items = g_list_copy_deep(
        (*bookmark).items,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut BookmarkItem) -> *mut BookmarkItem>,
            GCopyFunc,
        >(Some(
            safe_c2rust_bookmark_item_copy
                as unsafe extern "C" fn(*mut BookmarkItem) -> *mut BookmarkItem,
        )),
        NULL,
    );
    l = (*copy).items;
    while !l.is_null() {
        let mut item: *mut BookmarkItem = (*l).data as *mut BookmarkItem;
        g_hash_table_insert(
            (*copy).items_by_uri,
            (*item).uri as gpointer,
            item as gpointer,
        );
        l = (*l).next;
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if g_hash_table_size((*copy).items_by_uri) == g_hash_table_size((*bookmark).items_by_uri) {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1742 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (copy->items_by_uri) == g_hash_table_size (bookmark->items_by_uri)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_free(mut bookmark: *mut GBookmarkFile) {
    if bookmark.is_null() {
        return;
    }
    safe_c2rust_g_bookmark_file_clear(bookmark);
    g_free(bookmark as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_load_from_data(
    mut bookmark: *mut GBookmarkFile,
    mut data: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut parse_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut retval: gboolean = 0;
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if length == -(1 as ::core::ffi::c_int) as gsize {
        length = strlen(data as *const ::core::ffi::c_char) as gsize;
    }
    if !(*bookmark).items.is_null() {
        safe_c2rust_g_bookmark_file_clear(bookmark);
        safe_c2rust_g_bookmark_file_init(bookmark);
    }
    parse_error = ::core::ptr::null_mut::<GError>();
    retval = safe_c2rust_g_bookmark_file_parse(bookmark, data, length, &raw mut parse_error);
    if retval == 0 {
        g_propagate_error(error, parse_error);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_load_from_file(
    mut bookmark: *mut GBookmarkFile,
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !(g_file_get_contents(filename, &raw mut buffer, &raw mut len, error) == 0) {
        if !(safe_c2rust_g_bookmark_file_load_from_data(bookmark, buffer, len, error) == 0) {
            ret = TRUE as gboolean;
        }
    }
    g_free(buffer as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_find_file_in_data_dirs(
    mut file: *const gchar,
    mut dirs: *mut *mut *mut gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut data_dirs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut data_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    path = ::core::ptr::null_mut::<gchar>();
    if dirs.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    data_dirs = *dirs;
    path = ::core::ptr::null_mut::<gchar>();
    while !data_dirs.is_null()
        && {
            data_dir = *data_dirs;
            !data_dir.is_null()
        }
        && path.is_null()
    {
        let mut candidate_file: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut sub_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
        candidate_file = file as *mut gchar;
        sub_dir = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
        while !candidate_file.is_null() && path.is_null() {
            let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
            path = g_build_filename(data_dir, sub_dir, candidate_file, NULL);
            candidate_file = strchr(candidate_file, '-' as i32) as *mut gchar;
            if candidate_file.is_null() {
                break;
            }
            candidate_file = candidate_file.offset(1);
            g_free(sub_dir as gpointer);
            sub_dir = g_strndup(
                file,
                (candidate_file.offset_from(file) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    as gsize,
            );
            p = sub_dir;
            while *p as ::core::ffi::c_int != '\0' as i32 {
                if *p as ::core::ffi::c_int == '-' as i32 {
                    *p = G_DIR_SEPARATOR as gchar;
                }
                p = p.offset(1);
            }
        }
        g_free(sub_dir as gpointer);
        data_dirs = data_dirs.offset(1);
    }
    *dirs = data_dirs;
    if path.is_null() {
        g_set_error_literal(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No valid bookmark file found in data dirs\0" as *const u8 as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_load_from_data_dirs(
    mut bookmark: *mut GBookmarkFile,
    mut file: *const gchar,
    mut full_path: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut all_data_dirs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut data_dirs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut user_data_dir: *const gchar = ::core::ptr::null::<gchar>();
    let mut system_data_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut i: gsize = 0;
    let mut j: gsize = 0;
    let mut output_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut found_file: gboolean = 0;
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if g_path_is_absolute(file) == 0 {
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
            b"!g_path_is_absolute (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    user_data_dir = g_get_user_data_dir();
    system_data_dirs = g_get_system_data_dirs();
    all_data_dirs = ({
        let mut __n: gsize =
            g_strv_length(system_data_dirs as *mut *mut gchar).wrapping_add(2 as guint) as gsize;
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
    i = 0 as gsize;
    let fresh0 = i;
    i = i.wrapping_add(1);
    let ref mut fresh1 = *all_data_dirs.offset(fresh0 as isize);
    *fresh1 =
        safe_c2rust_g_strdup_inline(user_data_dir as *const ::core::ffi::c_char) as *mut gchar;
    j = 0 as gsize;
    while !(*system_data_dirs.offset(j as isize)).is_null() {
        let fresh2 = j;
        j = j.wrapping_add(1);
        let fresh3 = i;
        i = i.wrapping_add(1);
        let ref mut fresh4 = *all_data_dirs.offset(fresh3 as isize);
        *fresh4 = safe_c2rust_g_strdup_inline(
            *system_data_dirs.offset(fresh2 as isize) as *const ::core::ffi::c_char
        ) as *mut gchar;
    }
    found_file = FALSE as gboolean;
    data_dirs = all_data_dirs;
    output_path = ::core::ptr::null_mut::<gchar>();
    while !(*data_dirs).is_null() && found_file == 0 {
        g_free(output_path as gpointer);
        output_path =
            safe_c2rust_find_file_in_data_dirs(file, &raw mut data_dirs, &raw mut file_error);
        if !file_error.is_null() {
            g_propagate_error(error, file_error);
            break;
        } else {
            found_file = safe_c2rust_g_bookmark_file_load_from_file(
                bookmark,
                output_path,
                &raw mut file_error,
            );
            if file_error.is_null() {
                continue;
            }
            g_propagate_error(error, file_error);
            break;
        }
    }
    if found_file != 0 && !full_path.is_null() {
        *full_path = output_path;
    } else {
        g_free(output_path as gpointer);
    }
    g_strfreev(all_data_dirs);
    return found_file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_to_data(
    mut bookmark: *mut GBookmarkFile,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut write_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    retval = safe_c2rust_g_bookmark_file_dump(bookmark, length, &raw mut write_error);
    if !write_error.is_null() {
        g_propagate_error(error, write_error);
        return ::core::ptr::null_mut::<gchar>();
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_to_file(
    mut bookmark: *mut GBookmarkFile,
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut data: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut data_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut write_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut len: gsize = 0;
    let mut retval: gboolean = 0;
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    data_error = ::core::ptr::null_mut::<GError>();
    data = safe_c2rust_g_bookmark_file_to_data(bookmark, &raw mut len, &raw mut data_error);
    if !data_error.is_null() {
        g_propagate_error(error, data_error);
        return FALSE;
    }
    write_error = ::core::ptr::null_mut::<GError>();
    g_file_set_contents(filename, data, len as gssize, &raw mut write_error);
    if !write_error.is_null() {
        g_propagate_error(error, write_error);
        retval = FALSE as gboolean;
    } else {
        retval = TRUE as gboolean;
    }
    g_free(data as gpointer);
    return retval;
}
unsafe extern "C" fn safe_c2rust_g_bookmark_file_lookup_item(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
) -> *mut BookmarkItem {
    if !(({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if !bookmark.is_null() && !uri.is_null() {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            2091 as ::core::ffi::c_int,
            G_STRFUNC,
            b"bookmark != NULL && uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return g_hash_table_lookup((*bookmark).items_by_uri, uri as gconstpointer)
        as *mut BookmarkItem;
}
unsafe extern "C" fn safe_c2rust_g_bookmark_file_add_item(
    mut bookmark: *mut GBookmarkFile,
    mut item: *mut BookmarkItem,
    mut error: *mut *mut GError,
) {
    if !(({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            2102 as ::core::ffi::c_int,
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if !item.is_null() {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            2103 as ::core::ffi::c_int,
            G_STRFUNC,
            b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if safe_c2rust_g_bookmark_file_has_item(bookmark, (*item).uri) != 0 {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_INVALID_URI as ::core::ffi::c_int as gint,
            glib_gettext(
                b"A bookmark for URI \xE2\x80\x9C%s\xE2\x80\x9D already exists\0" as *const u8
                    as *const gchar,
            ),
            (*item).uri,
        );
        return;
    }
    (*bookmark).items = g_list_prepend((*bookmark).items, item as gpointer);
    g_hash_table_replace(
        (*bookmark).items_by_uri,
        (*item).uri as gpointer,
        item as gpointer,
    );
    if (*item).added.is_null() {
        (*item).added = g_date_time_new_now_utc();
    }
    if (*item).modified.is_null() {
        (*item).modified = g_date_time_new_now_utc();
    }
    if (*item).visited.is_null() {
        (*item).visited = g_date_time_new_now_utc();
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_remove_item(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
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
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    (*bookmark).items = g_list_remove((*bookmark).items, item as gconstpointer);
    g_hash_table_remove((*bookmark).items_by_uri, (*item).uri as gconstpointer);
    safe_c2rust_bookmark_item_free(item);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_has_item(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (NULL != g_hash_table_lookup((*bookmark).items_by_uri, uri as gconstpointer))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_uris(
    mut bookmark: *mut GBookmarkFile,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut uris: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    let mut n_items: gsize = 0;
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    n_items = g_list_length((*bookmark).items) as gsize;
    uris = ({
        let mut __n: gsize = n_items.wrapping_add(1 as gsize);
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
    l = g_list_last((*bookmark).items);
    i = 0 as gsize;
    while !l.is_null() {
        let mut item: *mut BookmarkItem = (*l).data as *mut BookmarkItem;
        if !(({
            let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
            if !item.is_null() {
                _g_boolean_var_89 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_89 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_89
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                2227 as ::core::ffi::c_int,
                G_STRFUNC,
                b"item != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let fresh14 = i;
        i = i.wrapping_add(1);
        let ref mut fresh15 = *uris.offset(fresh14 as isize);
        *fresh15 = safe_c2rust_g_strdup_inline((*item).uri) as *mut gchar;
        l = (*l).prev;
    }
    let ref mut fresh16 = *uris.offset(i as isize);
    *fresh16 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = i;
    }
    return uris;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_title(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut title: *const gchar,
) {
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if uri.is_null() {
        g_free((*bookmark).title as gpointer);
        (*bookmark).title =
            safe_c2rust_g_strdup_inline(title as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
        item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
        if item.is_null() {
            item = safe_c2rust_bookmark_item_new(uri);
            safe_c2rust_g_bookmark_file_add_item(
                bookmark,
                item,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
        g_free((*item).title as gpointer);
        (*item).title =
            safe_c2rust_g_strdup_inline(title as *const ::core::ffi::c_char) as *mut gchar;
        safe_c2rust_bookmark_item_touch_modified(item);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_title(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if uri.is_null() {
        return safe_c2rust_g_strdup_inline((*bookmark).title) as *mut gchar;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_strdup_inline((*item).title) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_description(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut description: *const gchar,
) {
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if uri.is_null() {
        g_free((*bookmark).description as gpointer);
        (*bookmark).description =
            safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
        item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
        if item.is_null() {
            item = safe_c2rust_bookmark_item_new(uri);
            safe_c2rust_g_bookmark_file_add_item(
                bookmark,
                item,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
        g_free((*item).description as gpointer);
        (*item).description =
            safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
        safe_c2rust_bookmark_item_touch_modified(item);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_description(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if uri.is_null() {
        return safe_c2rust_g_strdup_inline((*bookmark).description) as *mut gchar;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_strdup_inline((*item).description) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_mime_type(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut mime_type: *const gchar,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
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
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !mime_type.is_null() {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"mime_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    g_free((*(*item).metadata).mime_type as gpointer);
    (*(*item).metadata).mime_type =
        safe_c2rust_g_strdup_inline(mime_type as *const ::core::ffi::c_char) as *mut gchar;
    safe_c2rust_bookmark_item_touch_modified(item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_mime_type(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if (*item).metadata.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No MIME type defined in the bookmark for URI \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_strdup_inline((*(*item).metadata).mime_type) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_is_private(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut is_private: gboolean,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
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
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    (*(*item).metadata)
        .set_is_private((is_private == TRUE) as ::core::ffi::c_int as guint as guint);
    safe_c2rust_bookmark_item_touch_modified(item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_is_private(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    if (*item).metadata.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No private flag has been defined in bookmark for URI \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    return (*(*item).metadata).is_private() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_added(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut added: time_t,
) {
    let mut added_dt: *mut GDateTime = if added != -(1 as ::core::ffi::c_int) as time_t {
        g_date_time_new_from_unix_utc(added as gint64)
    } else {
        g_date_time_new_now_utc()
    };
    safe_c2rust_g_bookmark_file_set_added_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        added_dt,
    );
    g_date_time_unref(added_dt);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_added_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut added: *mut GDateTime,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
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
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if !added.is_null() {
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"added != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri as *const gchar);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*item).added;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    (*item).added = g_date_time_ref(added);
    let mut _pp_0: *mut *mut GDateTime = &raw mut (*item).modified;
    let mut _ptr_0: *mut GDateTime = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr_0.is_null() {
        g_date_time_unref(_ptr_0 as *mut GDateTime);
    }
    (*item).modified = g_date_time_ref(added);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_added(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> time_t {
    let mut added: *mut GDateTime = safe_c2rust_g_bookmark_file_get_added_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        error,
    );
    return if !added.is_null() {
        g_date_time_to_unix(added) as time_t
    } else {
        -(1 as ::core::ffi::c_int) as time_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_added_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GDateTime {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return (*item).added;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_modified(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut modified: time_t,
) {
    let mut modified_dt: *mut GDateTime = if modified != -(1 as ::core::ffi::c_int) as time_t {
        g_date_time_new_from_unix_utc(modified as gint64)
    } else {
        g_date_time_new_now_utc()
    };
    safe_c2rust_g_bookmark_file_set_modified_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        modified_dt,
    );
    g_date_time_unref(modified_dt);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_modified_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut modified: *mut GDateTime,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
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
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if !modified.is_null() {
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"modified != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri as *const gchar);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*item).modified;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    (*item).modified = g_date_time_ref(modified);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_modified(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> time_t {
    let mut modified: *mut GDateTime = safe_c2rust_g_bookmark_file_get_modified_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        error,
    );
    return if !modified.is_null() {
        g_date_time_to_unix(modified) as time_t
    } else {
        -(1 as ::core::ffi::c_int) as time_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_modified_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GDateTime {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_112 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_112 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_112
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_113 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_113 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_113
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_114 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_114 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_114
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return (*item).modified;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_visited(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut visited: time_t,
) {
    let mut visited_dt: *mut GDateTime = if visited != -(1 as ::core::ffi::c_int) as time_t {
        g_date_time_new_from_unix_utc(visited as gint64)
    } else {
        g_date_time_new_now_utc()
    };
    safe_c2rust_g_bookmark_file_set_visited_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        visited_dt,
    );
    g_date_time_unref(visited_dt);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_visited_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut visited: *mut GDateTime,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_115 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_115 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_115
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_116 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_116 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_116
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
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if !visited.is_null() {
            _g_boolean_var_117 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_117 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_117
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"visited != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri as *const gchar);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*item).visited;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    (*item).visited = g_date_time_ref(visited);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_visited(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut error: *mut *mut GError,
) -> time_t {
    let mut visited: *mut GDateTime = safe_c2rust_g_bookmark_file_get_visited_date_time(
        bookmark,
        uri as *const ::core::ffi::c_char,
        error,
    );
    return if !visited.is_null() {
        g_date_time_to_unix(visited) as time_t
    } else {
        -(1 as ::core::ffi::c_int) as time_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_visited_date_time(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GDateTime {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_118 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_118 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_118
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_119 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_119 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_119
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    return (*item).visited;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_has_group(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut group: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_121 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_121 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_121
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_122 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_122 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_122
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    if (*item).metadata.is_null() {
        return FALSE;
    }
    l = (*(*item).metadata).groups;
    while !l.is_null() {
        if strcmp(
            (*l).data as *const ::core::ffi::c_char,
            group as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        l = (*l).next;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_add_group(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut group: *const gchar,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_123 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_123 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_123
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_124 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_124 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_124
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
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if !group.is_null()
            && *group.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_125
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group != NULL && group[0] != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    if safe_c2rust_g_bookmark_file_has_group(
        bookmark,
        uri,
        group,
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        (*(*item).metadata).groups = g_list_prepend(
            (*(*item).metadata).groups,
            safe_c2rust_g_strdup_inline(group as *const ::core::ffi::c_char) as gpointer,
        );
        safe_c2rust_bookmark_item_touch_modified(item);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_remove_group(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut group: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_126
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    if (*item).metadata.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No groups set in bookmark for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    l = (*(*item).metadata).groups;
    while !l.is_null() {
        if strcmp(
            (*l).data as *const ::core::ffi::c_char,
            group as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*(*item).metadata).groups = g_list_remove_link((*(*item).metadata).groups, l);
            g_free((*l).data);
            g_list_free_1(l);
            safe_c2rust_bookmark_item_touch_modified(item);
            return TRUE;
        }
        l = (*l).next;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_groups(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut groups: *mut *const gchar,
    mut length: gsize,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_128 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_128 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_128
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_129 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_129 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_129
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
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if !groups.is_null() {
            _g_boolean_var_130 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_130 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_130
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"groups != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    g_list_free_full(
        (*(*item).metadata).groups,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(*item).metadata).groups = ::core::ptr::null_mut::<GList>();
    if !groups.is_null() {
        i = 0 as gsize;
        while i < length && !(*groups.offset(i as isize)).is_null() {
            (*(*item).metadata).groups = g_list_append(
                (*(*item).metadata).groups,
                safe_c2rust_g_strdup_inline(*groups.offset(i as isize) as *const ::core::ffi::c_char)
                    as gpointer,
            );
            i = i.wrapping_add(1);
        }
    }
    safe_c2rust_bookmark_item_touch_modified(item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_groups(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut len: gsize = 0;
    let mut i: gsize = 0;
    let mut retval: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_131 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_131 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_131
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_132 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_132 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_132
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if (*item).metadata.is_null() {
        if !length.is_null() {
            *length = 0 as gsize;
        }
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    len = g_list_length((*(*item).metadata).groups) as gsize;
    retval = ({
        let mut __n: gsize = len.wrapping_add(1 as gsize);
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
    l = g_list_last((*(*item).metadata).groups);
    i = 0 as gsize;
    while !l.is_null() {
        let mut group_name: *mut gchar = (*l).data as *mut gchar;
        if !(({
            let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
            if !group_name.is_null() {
                _g_boolean_var_133 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_133 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_133
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                3252 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let fresh5 = i;
        i = i.wrapping_add(1);
        let ref mut fresh6 = *retval.offset(fresh5 as isize);
        *fresh6 = safe_c2rust_g_strdup_inline(group_name) as *mut gchar;
        l = (*l).prev;
    }
    let ref mut fresh7 = *retval.offset(i as isize);
    *fresh7 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = len;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_add_application(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut name: *const gchar,
    mut exec: *const gchar,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut app_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut app_exec: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut stamp: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_134 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_134 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_134
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_135 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_135 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_135
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
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if !name.is_null()
        && *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        app_name = safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        app_name =
            safe_c2rust_g_strdup_inline(g_get_application_name() as *const ::core::ffi::c_char)
                as *mut gchar;
    }
    if !exec.is_null()
        && *exec.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        app_exec = safe_c2rust_g_strdup_inline(exec as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        app_exec = g_strjoin(
            b" \0" as *const u8 as *const gchar,
            g_get_prgname(),
            b"%u\0" as *const u8 as *const ::core::ffi::c_char,
            NULL,
        );
    }
    stamp = g_date_time_new_now_utc();
    safe_c2rust_g_bookmark_file_set_application_info(
        bookmark,
        uri as *const ::core::ffi::c_char,
        app_name,
        app_exec,
        -(1 as ::core::ffi::c_int),
        stamp,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_date_time_unref(stamp);
    g_free(app_exec as gpointer);
    g_free(app_name as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_remove_application(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut set_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut retval: gboolean = 0;
    if ({
        let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_136 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_136 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_136
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    set_error = ::core::ptr::null_mut::<GError>();
    retval = safe_c2rust_g_bookmark_file_set_application_info(
        bookmark,
        uri as *const ::core::ffi::c_char,
        name as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<GDateTime>(),
        &raw mut set_error,
    );
    if !set_error.is_null() {
        g_propagate_error(error, set_error);
        return FALSE;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_has_application(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_139 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_139 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_139
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_140 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_140 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_140
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_141 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_141 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_141
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    return (NULL as *mut BookmarkAppInfo != safe_c2rust_bookmark_item_lookup_app_info(item, name))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_app_info(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut name: *const gchar,
    mut exec: *const gchar,
    mut count: gint,
    mut stamp: time_t,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut stamp_dt: *mut GDateTime = if stamp != -(1 as ::core::ffi::c_int) as time_t {
        g_date_time_new_from_unix_utc(stamp as gint64)
    } else {
        g_date_time_new_now_utc()
    };
    let mut retval: gboolean = 0;
    retval = safe_c2rust_g_bookmark_file_set_application_info(
        bookmark,
        uri as *const ::core::ffi::c_char,
        name as *const ::core::ffi::c_char,
        exec as *const ::core::ffi::c_char,
        count as ::core::ffi::c_int,
        stamp_dt,
        error,
    );
    g_date_time_unref(stamp_dt);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_application_info(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut exec: *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
    mut stamp: *mut GDateTime,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut ai: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
    if ({
        let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_142 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_142 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_142
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_143 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_143 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_143
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_144 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_144 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_144
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
        if !exec.is_null() {
            _g_boolean_var_145 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_145 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_145
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"exec != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
        if count == 0 as ::core::ffi::c_int || !stamp.is_null() {
            _g_boolean_var_146 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_146 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_146
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"count == 0 || stamp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_147 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_147 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_147
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
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        if count == 0 as ::core::ffi::c_int {
            g_set_error(
                error,
                safe_c2rust_g_bookmark_file_error_quark(),
                G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                        as *const gchar,
                ),
                uri,
            );
            return FALSE;
        } else {
            item = safe_c2rust_bookmark_item_new(uri as *const gchar);
            safe_c2rust_g_bookmark_file_add_item(
                bookmark,
                item,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    ai = safe_c2rust_bookmark_item_lookup_app_info(item, name as *const gchar);
    if ai.is_null() {
        if count == 0 as ::core::ffi::c_int {
            g_set_error(
                error,
                safe_c2rust_g_bookmark_file_error_quark(),
                G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"No application with name \xE2\x80\x9C%s\xE2\x80\x9D registered a bookmark for \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                name,
                uri,
            );
            return FALSE;
        } else {
            ai = safe_c2rust_bookmark_app_info_new(name as *const gchar);
            (*(*item).metadata).applications =
                g_list_prepend((*(*item).metadata).applications, ai as gpointer);
            g_hash_table_replace(
                (*(*item).metadata).apps_by_name,
                (*ai).name as gpointer,
                ai as gpointer,
            );
        }
    }
    if count == 0 as ::core::ffi::c_int {
        (*(*item).metadata).applications =
            g_list_remove((*(*item).metadata).applications, ai as gconstpointer);
        g_hash_table_remove(
            (*(*item).metadata).apps_by_name,
            (*ai).name as gconstpointer,
        );
        safe_c2rust_bookmark_app_info_free(ai);
        safe_c2rust_bookmark_item_touch_modified(item);
        return TRUE;
    } else if count > 0 as ::core::ffi::c_int {
        (*ai).count = count as guint;
    } else {
        (*ai).count = (*ai).count.wrapping_add(1 as guint);
    }
    let mut _pp: *mut *mut GDateTime = &raw mut (*ai).stamp;
    let mut _ptr: *mut GDateTime = *_pp;
    *_pp = ::core::ptr::null_mut::<GDateTime>();
    if !_ptr.is_null() {
        g_date_time_unref(_ptr as *mut GDateTime);
    }
    (*ai).stamp = g_date_time_ref(stamp);
    if !exec.is_null()
        && *exec.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        g_free((*ai).exec as gpointer);
        (*ai).exec = g_shell_quote(exec as *const gchar);
    }
    safe_c2rust_bookmark_item_touch_modified(item);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_expand_exec_line(
    mut exec_fmt: *const gchar,
    mut uri: *const gchar,
) -> *mut gchar {
    let mut exec: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut ch: gchar = 0;
    exec = g_string_sized_new(512 as gsize);
    loop {
        let fresh11 = exec_fmt;
        exec_fmt = exec_fmt.offset(1);
        ch = *fresh11;
        if !(ch as ::core::ffi::c_int != '\0' as i32) {
            break;
        }
        if ch as ::core::ffi::c_int != '%' as i32 {
            exec = safe_c2rust_g_string_append_c_inline(exec, ch);
        } else {
            let fresh12 = exec_fmt;
            exec_fmt = exec_fmt.offset(1);
            ch = *fresh12;
            match ch as ::core::ffi::c_int {
                0 => {
                    break;
                }
                85 | 117 => {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                uri as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                exec,
                                __val,
                                if ({
                                    let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_148 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_148 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_148
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
                            exec,
                            uri as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
                70 | 102 => {
                    let mut file: *mut gchar = g_filename_from_uri(
                        uri,
                        ::core::ptr::null_mut::<*mut gchar>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    if !file.is_null() {
                        if 0 != 0 {
                            ({
                                let __val: *const ::core::ffi::c_char = file;
                                safe_c2rust_g_string_append_len_inline(
                                    exec,
                                    __val,
                                    if ({
                                        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
                                        if !__val.is_null() {
                                            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_149
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        strlen(
                                            __val.offset(
                                                __val.is_null() as ::core::ffi::c_int as isize
                                            ),
                                        ) as gssize
                                    } else {
                                        -(1 as ::core::ffi::c_int) as gssize
                                    },
                                );
                            });
                        } else {
                            safe_c2rust_g_string_append_len_inline(
                                exec,
                                file,
                                -(1 as ::core::ffi::c_int) as gssize,
                            );
                        };
                        g_free(file as gpointer);
                    } else {
                        if 0 != 0 {
                            if 0 as ::core::ffi::c_int == 0 {
                                g_string_free(
                                    exec,
                                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                                );
                            } else {
                                g_string_free_and_steal(exec);
                            };
                        } else {
                            g_string_free(
                                exec,
                                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                            );
                        };
                        return ::core::ptr::null_mut::<gchar>();
                    }
                }
                37 | _ => {
                    exec = safe_c2rust_g_string_append_c_inline(exec, ch);
                }
            }
        }
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(exec, 0 as gboolean)
        } else {
            g_string_free_and_steal(exec)
        }
    } else {
        g_string_free(exec, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_app_info(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut name: *const gchar,
    mut exec: *mut *mut gchar,
    mut count: *mut guint,
    mut stamp: *mut time_t,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut stamp_dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut retval: gboolean = 0;
    retval = safe_c2rust_g_bookmark_file_get_application_info(
        bookmark,
        uri as *const ::core::ffi::c_char,
        name as *const ::core::ffi::c_char,
        exec as *mut *mut ::core::ffi::c_char,
        count as *mut ::core::ffi::c_uint,
        &raw mut stamp_dt,
        error,
    );
    if retval == 0 {
        return FALSE;
    }
    if !stamp.is_null() {
        *stamp = g_date_time_to_unix(stamp_dt) as time_t;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_application_info(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut exec: *mut *mut ::core::ffi::c_char,
    mut count: *mut ::core::ffi::c_uint,
    mut stamp: *mut *mut GDateTime,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut ai: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_150 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_150 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_150
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_151 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_151 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_151
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_152 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_152 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_152
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
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
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri as *const gchar);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    ai = safe_c2rust_bookmark_item_lookup_app_info(item, name as *const gchar);
    if ai.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No application with name \xE2\x80\x9C%s\xE2\x80\x9D registered a bookmark for \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            name,
            uri,
        );
        return FALSE;
    }
    if !exec.is_null() {
        let mut unquote_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut command_line: *mut gchar = ::core::ptr::null_mut::<gchar>();
        command_line = g_shell_unquote((*ai).exec, &raw mut unquote_error);
        if !unquote_error.is_null() {
            g_propagate_error(error, unquote_error);
            return FALSE;
        }
        *exec = safe_c2rust_expand_exec_line(command_line, uri as *const gchar)
            as *mut ::core::ffi::c_char;
        if (*exec).is_null() {
            g_set_error(
                error,
                safe_c2rust_g_bookmark_file_error_quark(),
                G_BOOKMARK_FILE_ERROR_INVALID_URI as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Failed to expand exec line \xE2\x80\x9C%s\xE2\x80\x9D with URI \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                (*ai).exec,
                uri,
            );
            g_free(command_line as gpointer);
            return FALSE;
        } else {
            g_free(command_line as gpointer);
        }
    }
    if !count.is_null() {
        *count = (*ai).count as ::core::ffi::c_uint;
    }
    if !stamp.is_null() {
        *stamp = (*ai).stamp;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_applications(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut apps: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    let mut n_apps: gsize = 0;
    if ({
        let mut _g_boolean_var_154: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_154 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_154 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_154
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_155: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_155 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_155 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_155
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if (*item).metadata.is_null() {
        if !length.is_null() {
            *length = 0 as gsize;
        }
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    n_apps = g_list_length((*(*item).metadata).applications) as gsize;
    apps = ({
        let mut __n: gsize = n_apps.wrapping_add(1 as gsize);
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
    l = g_list_last((*(*item).metadata).applications);
    i = 0 as gsize;
    while !l.is_null() {
        let mut ai: *mut BookmarkAppInfo = ::core::ptr::null_mut::<BookmarkAppInfo>();
        ai = (*l).data as *mut BookmarkAppInfo;
        if !(({
            let mut _g_boolean_var_156: ::core::ffi::c_int = 0;
            if !ai.is_null() {
                _g_boolean_var_156 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_156 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_156
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                3898 as ::core::ffi::c_int,
                G_STRFUNC,
                b"ai != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(({
            let mut _g_boolean_var_157: ::core::ffi::c_int = 0;
            if !(*ai).name.is_null() {
                _g_boolean_var_157 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_157 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_157
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gbookmarkfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                3899 as ::core::ffi::c_int,
                G_STRFUNC,
                b"ai->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let fresh8 = i;
        i = i.wrapping_add(1);
        let ref mut fresh9 = *apps.offset(fresh8 as isize);
        *fresh9 = safe_c2rust_g_strdup_inline((*ai).name) as *mut gchar;
        l = (*l).prev;
    }
    let ref mut fresh10 = *apps.offset(i as isize);
    *fresh10 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = i;
    }
    return apps;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_size(
    mut bookmark: *mut GBookmarkFile,
) -> gint {
    if ({
        let mut _g_boolean_var_158: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_158 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_158 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_158
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return g_list_length((*bookmark).items) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_move_item(
    mut bookmark: *mut GBookmarkFile,
    mut old_uri: *const gchar,
    mut new_uri: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_159: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_159 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_159 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_159
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_160: ::core::ffi::c_int = 0;
        if !old_uri.is_null() {
            _g_boolean_var_160 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_160 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_160
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"old_uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, old_uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            old_uri,
        );
        return FALSE;
    }
    if !new_uri.is_null()
        && *new_uri.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        if g_strcmp0(
            old_uri as *const ::core::ffi::c_char,
            new_uri as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if safe_c2rust_g_bookmark_file_has_item(bookmark, new_uri) != 0 {
            if safe_c2rust_g_bookmark_file_remove_item(bookmark, new_uri, error) == 0 {
                return FALSE;
            }
        }
        g_hash_table_steal((*bookmark).items_by_uri, (*item).uri as gconstpointer);
        g_free((*item).uri as gpointer);
        (*item).uri =
            safe_c2rust_g_strdup_inline(new_uri as *const ::core::ffi::c_char) as *mut gchar;
        safe_c2rust_bookmark_item_touch_modified(item);
        g_hash_table_replace(
            (*bookmark).items_by_uri,
            (*item).uri as gpointer,
            item as gpointer,
        );
        return TRUE;
    } else {
        if safe_c2rust_g_bookmark_file_remove_item(bookmark, old_uri, error) == 0 {
            return FALSE;
        }
        return TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_set_icon(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut href: *const gchar,
    mut mime_type: *const gchar,
) {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_161: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_161 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_161 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_161
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_162: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_162 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_162 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_162
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
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        item = safe_c2rust_bookmark_item_new(uri);
        safe_c2rust_g_bookmark_file_add_item(
            bookmark,
            item,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*item).metadata.is_null() {
        (*item).metadata = safe_c2rust_bookmark_metadata_new();
    }
    g_free((*(*item).metadata).icon_href as gpointer);
    g_free((*(*item).metadata).icon_mime as gpointer);
    (*(*item).metadata).icon_href =
        safe_c2rust_g_strdup_inline(href as *const ::core::ffi::c_char) as *mut gchar;
    if !mime_type.is_null()
        && *mime_type.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        (*(*item).metadata).icon_mime =
            safe_c2rust_g_strdup_inline(mime_type as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        (*(*item).metadata).icon_mime = safe_c2rust_g_strdup_inline(
            b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    safe_c2rust_bookmark_item_touch_modified(item);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bookmark_file_get_icon(
    mut bookmark: *mut GBookmarkFile,
    mut uri: *const gchar,
    mut href: *mut *mut gchar,
    mut mime_type: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut item: *mut BookmarkItem = ::core::ptr::null_mut::<BookmarkItem>();
    if ({
        let mut _g_boolean_var_163: ::core::ffi::c_int = 0;
        if !bookmark.is_null() {
            _g_boolean_var_163 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_163 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_163
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bookmark != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_164: ::core::ffi::c_int = 0;
        if !uri.is_null() {
            _g_boolean_var_164 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_164 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_164
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    item = safe_c2rust_g_bookmark_file_lookup_item(bookmark, uri);
    if item.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_bookmark_file_error_quark(),
            G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No bookmark found for URI \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            uri,
        );
        return FALSE;
    }
    if (*item).metadata.is_null() || (*(*item).metadata).icon_href.is_null() {
        return FALSE;
    }
    if !href.is_null() {
        *href = safe_c2rust_g_strdup_inline((*(*item).metadata).icon_href) as *mut gchar;
    }
    if !mime_type.is_null() {
        *mime_type = safe_c2rust_g_strdup_inline((*(*item).metadata).icon_mime) as *mut gchar;
    }
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_bookmark_file_copy\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
