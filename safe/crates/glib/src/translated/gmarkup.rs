use ::c2rust_bitfields;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
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
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_alloc() -> *mut GSList;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_concat(list1: *mut GSList, list2: *mut GSList) -> *mut GSList;
    fn g_slist_remove_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_unichar_isalpha(c: gunichar) -> gboolean;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_validate_len(str: *const gchar, max_len: gsize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_make_valid(str: *const gchar, len: gssize) -> *mut gchar;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMarkupError = ::core::ffi::c_uint;
pub const G_MARKUP_ERROR_MISSING_ATTRIBUTE: GMarkupError = 6;
pub const G_MARKUP_ERROR_INVALID_CONTENT: GMarkupError = 5;
pub const G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE: GMarkupError = 4;
pub const G_MARKUP_ERROR_UNKNOWN_ELEMENT: GMarkupError = 3;
pub const G_MARKUP_ERROR_PARSE: GMarkupError = 2;
pub const G_MARKUP_ERROR_EMPTY: GMarkupError = 1;
pub const G_MARKUP_ERROR_BAD_UTF8: GMarkupError = 0;
pub type GMarkupParseFlags = ::core::ffi::c_uint;
pub const G_MARKUP_IGNORE_QUALIFIED: GMarkupParseFlags = 8;
pub const G_MARKUP_PREFIX_ERROR_POSITION: GMarkupParseFlags = 4;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: GMarkupParseFlags = 2;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: GMarkupParseFlags = 1;
pub const G_MARKUP_DEFAULT_FLAGS: GMarkupParseFlags = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GMarkupParseContext {
    pub parser: *const GMarkupParser,
    pub ref_count: gint,
    pub flags: GMarkupParseFlags,
    pub line_number: gint,
    pub char_number: gint,
    pub state: GMarkupParseState,
    pub user_data: gpointer,
    pub dnotify: GDestroyNotify,
    pub partial_chunk: *mut GString,
    pub spare_chunks: *mut GSList,
    pub tag_stack: *mut GSList,
    pub tag_stack_gstr: *mut GSList,
    pub spare_list_nodes: *mut GSList,
    pub attr_names: *mut *mut GString,
    pub attr_values: *mut *mut GString,
    pub cur_attr: gint,
    pub alloc_attrs: gint,
    pub current_text: *const gchar,
    pub current_text_len: gssize,
    pub current_text_end: *const gchar,
    pub start: *const gchar,
    pub iter: *const gchar,
    #[bitfield(name = "document_empty", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "parsing", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "awaiting_pop", ty = "guint", bits = "2..=2")]
    pub document_empty_parsing_awaiting_pop: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub balance: gint,
    pub subparser_stack: *mut GSList,
    pub subparser_element: *const ::core::ffi::c_char,
    pub held_user_data: gpointer,
}
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GMarkupParseState = ::core::ffi::c_uint;
pub const STATE_ERROR: GMarkupParseState = 16;
pub const STATE_INSIDE_PASSTHROUGH: GMarkupParseState = 15;
pub const STATE_AFTER_CLOSE_TAG_NAME: GMarkupParseState = 14;
pub const STATE_INSIDE_CLOSE_TAG_NAME: GMarkupParseState = 13;
pub const STATE_AFTER_CLOSE_TAG_SLASH: GMarkupParseState = 12;
pub const STATE_INSIDE_TEXT: GMarkupParseState = 11;
pub const STATE_INSIDE_ATTRIBUTE_VALUE_DQ: GMarkupParseState = 10;
pub const STATE_INSIDE_ATTRIBUTE_VALUE_SQ: GMarkupParseState = 9;
pub const STATE_AFTER_ATTRIBUTE_EQUALS_SIGN: GMarkupParseState = 8;
pub const STATE_BETWEEN_ATTRIBUTES: GMarkupParseState = 7;
pub const STATE_AFTER_ATTRIBUTE_NAME: GMarkupParseState = 6;
pub const STATE_INSIDE_ATTRIBUTE_NAME: GMarkupParseState = 5;
pub const STATE_INSIDE_OPEN_TAG_NAME: GMarkupParseState = 4;
pub const STATE_AFTER_ELISION_SLASH: GMarkupParseState = 3;
pub const STATE_AFTER_CLOSE_ANGLE: GMarkupParseState = 2;
pub const STATE_AFTER_OPEN_ANGLE: GMarkupParseState = 1;
pub const STATE_START: GMarkupParseState = 0;
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
pub type GMarkupParseContext = _GMarkupParseContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMarkupRecursionTracker {
    pub prev_element: *const ::core::ffi::c_char,
    pub prev_parser: *const GMarkupParser,
    pub prev_user_data: gpointer,
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
pub type gunichar = guint32;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub type GMarkupCollectType = ::core::ffi::c_uint;
pub const G_MARKUP_COLLECT_OPTIONAL: GMarkupCollectType = 65536;
pub const G_MARKUP_COLLECT_TRISTATE: GMarkupCollectType = 4;
pub const G_MARKUP_COLLECT_BOOLEAN: GMarkupCollectType = 3;
pub const G_MARKUP_COLLECT_STRDUP: GMarkupCollectType = 2;
pub const G_MARKUP_COLLECT_STRING: GMarkupCollectType = 1;
pub const G_MARKUP_COLLECT_INVALID: GMarkupCollectType = 0;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_markup_parse_context_new\0" as *const u8 as *const ::core::ffi::c_char;
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
        let fresh12 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh12 as isize) = c;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_error_quark() -> GQuark {
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
            g_quark_from_static_string(b"g-markup-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
unsafe extern "C" fn safe_c2rust_get_list_node(
    mut context: *mut GMarkupParseContext,
    mut data: gpointer,
) -> *mut GSList {
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if !(*context).spare_list_nodes.is_null() {
        node = (*context).spare_list_nodes;
        (*context).spare_list_nodes = g_slist_remove_link((*context).spare_list_nodes, node);
    } else {
        node = g_slist_alloc();
    }
    (*node).data = data;
    return node;
}
unsafe extern "C" fn safe_c2rust_free_list_node(
    mut context: *mut GMarkupParseContext,
    mut node: *mut GSList,
) {
    (*node).data = NULL_0 as gpointer;
    (*context).spare_list_nodes = g_slist_concat(node, (*context).spare_list_nodes);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_new(
    mut parser: *const GMarkupParser,
    mut flags: GMarkupParseFlags,
    mut user_data: gpointer,
    mut user_data_dnotify: GDestroyNotify,
) -> *mut GMarkupParseContext {
    let mut context: *mut GMarkupParseContext = ::core::ptr::null_mut::<GMarkupParseContext>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !parser.is_null() {
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
            b"parser != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMarkupParseContext>();
    }
    context = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GMarkupParseContext>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GMarkupParseContext;
    (*context).ref_count = 1 as ::core::ffi::c_int as gint;
    (*context).parser = parser;
    (*context).flags = flags;
    (*context).user_data = user_data;
    (*context).dnotify = user_data_dnotify;
    (*context).line_number = 1 as ::core::ffi::c_int as gint;
    (*context).char_number = 1 as ::core::ffi::c_int as gint;
    (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
    (*context).spare_chunks = ::core::ptr::null_mut::<GSList>();
    (*context).spare_list_nodes = ::core::ptr::null_mut::<GSList>();
    (*context).state = STATE_START;
    (*context).tag_stack = ::core::ptr::null_mut::<GSList>();
    (*context).tag_stack_gstr = ::core::ptr::null_mut::<GSList>();
    (*context).attr_names = ::core::ptr::null_mut::<*mut GString>();
    (*context).attr_values = ::core::ptr::null_mut::<*mut GString>();
    (*context).cur_attr = -(1 as ::core::ffi::c_int) as gint;
    (*context).alloc_attrs = 0 as ::core::ffi::c_int as gint;
    (*context).current_text = ::core::ptr::null::<gchar>();
    (*context).current_text_len = -(1 as ::core::ffi::c_int) as gssize;
    (*context).current_text_end = ::core::ptr::null::<gchar>();
    (*context).start = ::core::ptr::null::<gchar>();
    (*context).iter = ::core::ptr::null::<gchar>();
    (*context).set_document_empty(TRUE as guint as guint);
    (*context).set_parsing(FALSE as guint as guint);
    (*context).set_awaiting_pop(FALSE as guint as guint);
    (*context).subparser_stack = ::core::ptr::null_mut::<GSList>();
    (*context).subparser_element = ::core::ptr::null::<::core::ffi::c_char>();
    (*context).held_user_data = NULL_0 as gpointer;
    (*context).balance = 0 as ::core::ffi::c_int as gint;
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_ref(
    mut context: *mut GMarkupParseContext,
) -> *mut GMarkupParseContext {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMarkupParseContext>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*context).ref_count > 0 as ::core::ffi::c_int {
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
            b"context->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMarkupParseContext>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*context).ref_count;
        (*context).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*context).ref_count, 1 as ::core::ffi::c_int);
    return context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_unref(
    mut context: *mut GMarkupParseContext,
) {
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
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*context).ref_count > 0 as ::core::ffi::c_int {
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
            b"context->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*context).ref_count;
            (*context).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*context).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_markup_parse_context_free(context);
    }
}
unsafe extern "C" fn safe_c2rust_string_full_free(mut ptr: gpointer) {
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                ptr as *mut GString,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(ptr as *mut GString);
        };
    } else {
        g_string_free(
            ptr as *mut GString,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_free(
    mut context: *mut GMarkupParseContext,
) {
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
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*context).parsing() == 0 {
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
            b"!context->parsing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*context).subparser_stack.is_null() {
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
            b"!context->subparser_stack\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if (*context).awaiting_pop() == 0 {
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
            b"!context->awaiting_pop\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*context).dnotify.is_some() {
        Some((*context).dnotify.expect("non-null function pointer"))
            .expect("non-null function pointer")((*context).user_data);
    }
    safe_c2rust_clear_attributes(context);
    g_free((*context).attr_names as gpointer);
    g_free((*context).attr_values as gpointer);
    g_slist_free_full(
        (*context).tag_stack_gstr,
        Some(safe_c2rust_string_full_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_slist_free((*context).tag_stack);
    g_slist_free_full(
        (*context).spare_chunks,
        Some(safe_c2rust_string_full_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_slist_free((*context).spare_list_nodes);
    if !(*context).partial_chunk.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    (*context).partial_chunk,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal((*context).partial_chunk);
            };
        } else {
            g_string_free(
                (*context).partial_chunk,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
    }
    g_free(context as gpointer);
}
unsafe extern "C" fn safe_c2rust_mark_error(
    mut context: *mut GMarkupParseContext,
    mut error: *mut GError,
) {
    (*context).state = STATE_ERROR;
    if (*(*context).parser).error.is_some() {
        Some(
            (*(*context).parser)
                .error
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(context, error, (*context).user_data);
    }
    while !(*context).subparser_stack.is_null() {
        safe_c2rust_pop_subparser_stack(context);
        (*context).set_awaiting_pop(FALSE as guint as guint);
        if (*(*context).parser).error.is_some() {
            Some(
                (*(*context).parser)
                    .error
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(context, error, (*context).user_data);
        }
    }
}
unsafe extern "C" fn safe_c2rust_set_error_literal(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
    mut code: GMarkupError,
    mut message: *const gchar,
) {
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    tmp_error = g_error_new_literal(safe_c2rust_g_markup_error_quark(), code as gint, message);
    g_prefix_error(
        &raw mut tmp_error,
        glib_gettext(b"Error on line %d char %d: \0" as *const u8 as *const gchar),
        (*context).line_number,
        (*context).char_number,
    );
    safe_c2rust_mark_error(context, tmp_error);
    g_propagate_error(error, tmp_error);
}
unsafe extern "C" fn safe_c2rust_set_error(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
    mut code: GMarkupError,
    mut format: *const gchar,
    mut args: ...
) {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s_valid: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    s = g_strdup_vprintf(format, args_0.clone());
    s_valid = g_utf8_make_valid(s, -(1 as ::core::ffi::c_int) as gssize);
    safe_c2rust_set_error_literal(context, error, code, s);
    g_free(s as gpointer);
    g_free(s_valid as gpointer);
}
unsafe extern "C" fn safe_c2rust_propagate_error(
    mut context: *mut GMarkupParseContext,
    mut dest: *mut *mut GError,
    mut src: *mut GError,
) {
    if (*context).flags as ::core::ffi::c_uint
        & G_MARKUP_PREFIX_ERROR_POSITION as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_prefix_error(
            &raw mut src,
            glib_gettext(b"Error on line %d char %d: \0" as *const u8 as *const gchar),
            (*context).line_number,
            (*context).char_number,
        );
    }
    safe_c2rust_mark_error(context, src);
    g_propagate_error(dest, src);
}
unsafe extern "C" fn safe_c2rust_slow_name_validate(
    mut context: *mut GMarkupParseContext,
    mut name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut p: *const gchar = name;
    if g_utf8_validate(
        name,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        safe_c2rust_set_error(
            context,
            error,
            G_MARKUP_ERROR_BAD_UTF8,
            glib_gettext(
                b"Invalid UTF-8 encoded text in name \xE2\x80\x94 not valid \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            name,
        );
        return FALSE;
    }
    if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALPHA as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || !(*p as ::core::ffi::c_int == '=' as i32
            || *p as ::core::ffi::c_int == '/' as i32
            || *p as ::core::ffi::c_int == '>' as i32
            || *p as ::core::ffi::c_int == ' ' as i32)
            && (*p as ::core::ffi::c_int == '_' as i32
                || *p as ::core::ffi::c_int == ':' as i32
                || g_unichar_isalpha(g_utf8_get_char(p)) != 0))
    {
        safe_c2rust_set_error(
            context,
            error,
            G_MARKUP_ERROR_PARSE,
            glib_gettext(
                b"\xE2\x80\x9C%s\xE2\x80\x9D is not a valid name\0" as *const u8 as *const gchar,
            ),
            name,
        );
        return FALSE;
    }
    p = name.offset(
        *safe_c2rust_g_utf8_skip.offset(*(name as *const guchar) as isize) as ::core::ffi::c_int
            as isize,
    ) as *mut ::core::ffi::c_char;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || !(*p as ::core::ffi::c_int == '=' as i32
                || *p as ::core::ffi::c_int == '/' as i32
                || *p as ::core::ffi::c_int == '>' as i32
                || *p as ::core::ffi::c_int == ' ' as i32)
                && (*p as ::core::ffi::c_int == '.' as i32
                    || *p as ::core::ffi::c_int == '-' as i32
                    || *p as ::core::ffi::c_int == '_' as i32
                    || *p as ::core::ffi::c_int == ':' as i32
                    || g_unichar_isalpha(g_utf8_get_char(p)) != 0))
        {
            safe_c2rust_set_error(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"\xE2\x80\x9C%s\xE2\x80\x9D is not a valid name: \xE2\x80\x9C%c\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                name,
                *p as ::core::ffi::c_int,
            );
            return FALSE;
        }
        p = p.offset(
            *safe_c2rust_g_utf8_skip.offset(*(p as *const guchar) as isize) as ::core::ffi::c_int
                as isize,
        ) as *mut ::core::ffi::c_char;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_name_validate(
    mut context: *mut GMarkupParseContext,
    mut name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut mask: ::core::ffi::c_char = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = name as *const ::core::ffi::c_char;
    if !(({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if *p as ::core::ffi::c_int == '=' as i32
            || *p as ::core::ffi::c_int == '/' as i32
            || *p as ::core::ffi::c_int == '>' as i32
            || *p as ::core::ffi::c_int == ' ' as i32
            || !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_ALPHA as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '_' as i32
                || *p as ::core::ffi::c_int == ':' as i32)
        {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0)
    {
        let fresh7 = p;
        p = p.offset(1);
        mask = *fresh7;
        loop {
            if !(*p as ::core::ffi::c_int != '\0' as i32) {
                current_block = 6483416627284290920;
                break;
            }
            mask = (mask as ::core::ffi::c_int | *p as ::core::ffi::c_int) as ::core::ffi::c_char;
            if ({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                    & G_ASCII_ALNUM as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    || !(*p as ::core::ffi::c_int == '=' as i32
                        || *p as ::core::ffi::c_int == '/' as i32
                        || *p as ::core::ffi::c_int == '>' as i32
                        || *p as ::core::ffi::c_int == ' ' as i32)
                        && (*p as ::core::ffi::c_int == '.' as i32
                            || *p as ::core::ffi::c_int == '-' as i32
                            || *p as ::core::ffi::c_int == '_' as i32
                            || *p as ::core::ffi::c_int == ':' as i32))
                {
                    _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_19
            }) as ::core::ffi::c_long
                != 0
            {
                current_block = 4638945012334404071;
                break;
            }
            p = p.offset(1);
        }
        match current_block {
            4638945012334404071 => {}
            _ => {
                if !(mask as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0) {
                    return TRUE;
                }
            }
        }
    }
    return safe_c2rust_slow_name_validate(context, name, error);
}
unsafe extern "C" fn safe_c2rust_text_validate(
    mut context: *mut GMarkupParseContext,
    mut p: *const gchar,
    mut len: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    if g_utf8_validate_len(p, len as gsize, ::core::ptr::null_mut::<*const gchar>()) == 0 {
        safe_c2rust_set_error(
            context,
            error,
            G_MARKUP_ERROR_BAD_UTF8,
            glib_gettext(
                b"Invalid UTF-8 encoded text in name \xE2\x80\x94 not valid \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            p,
        );
        return FALSE;
    } else {
        return TRUE;
    };
}
unsafe extern "C" fn safe_c2rust_char_str(mut c: gunichar, mut buf: *mut gchar) -> *mut gchar {
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        8 as size_t,
    );
    g_unichar_to_utf8(c, buf);
    return buf;
}
unsafe extern "C" fn safe_c2rust_utf8_str(
    mut utf8: *const gchar,
    mut max_len: gsize,
    mut buf: *mut gchar,
) -> *mut gchar {
    let mut c: gunichar = g_utf8_get_char_validated(utf8, max_len as gssize);
    if c == -(1 as ::core::ffi::c_int) as gunichar || c == -(2 as ::core::ffi::c_int) as gunichar {
        let mut ch: guchar = (if max_len > 0 as gsize {
            *utf8 as guchar as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as guchar;
        let mut temp: *mut gchar =
            g_strdup_printf(b"\\x%02x\0" as *const u8 as *const gchar, ch as guint);
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            8 as size_t,
        );
        memcpy(
            buf as *mut ::core::ffi::c_void,
            temp as *const ::core::ffi::c_void,
            strlen(temp),
        );
        g_free(temp as gpointer);
    } else {
        safe_c2rust_char_str(c, buf);
    }
    return buf;
}
unsafe extern "C" fn safe_c2rust_set_unescape_error(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
    mut remaining_text: *const gchar,
    mut code: GMarkupError,
    mut format: *const gchar,
    mut args: ...
) {
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    let mut remaining_newlines: gint = 0;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    remaining_newlines = 0 as ::core::ffi::c_int as gint;
    p = remaining_text;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        if *p as ::core::ffi::c_int == '\n' as i32 {
            remaining_newlines += 1;
        }
        p = p.offset(1);
    }
    args_0 = args.clone();
    s = g_strdup_vprintf(format, args_0.clone());
    tmp_error = g_error_new(
        safe_c2rust_g_markup_error_quark(),
        code as gint,
        glib_gettext(b"Error on line %d: %s\0" as *const u8 as *const gchar),
        (*context).line_number - remaining_newlines,
        s,
    );
    g_free(s as gpointer);
    safe_c2rust_mark_error(context, tmp_error);
    g_propagate_error(error, tmp_error);
}
unsafe extern "C" fn safe_c2rust_unescape_gstring_inplace(
    mut context: *mut GMarkupParseContext,
    mut string: *mut GString,
    mut is_ascii: *mut gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut mask: ::core::ffi::c_char = 0;
    let mut to: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut from: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut normalize_attribute: gboolean = 0;
    *is_ascii = FALSE as gboolean;
    if (*context).state as ::core::ffi::c_uint
        == STATE_INSIDE_ATTRIBUTE_VALUE_SQ as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*context).state as ::core::ffi::c_uint
            == STATE_INSIDE_ATTRIBUTE_VALUE_DQ as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        normalize_attribute = TRUE as gboolean;
    } else {
        normalize_attribute = FALSE as gboolean;
    }
    mask = 0 as ::core::ffi::c_char;
    to = (*string).str_0 as *mut ::core::ffi::c_char;
    from = to;
    while *from as ::core::ffi::c_int != '\0' as i32 {
        *to = *from;
        mask = (mask as ::core::ffi::c_int | *to as ::core::ffi::c_int) as ::core::ffi::c_char;
        if normalize_attribute != 0
            && (*to as ::core::ffi::c_int == '\t' as i32
                || *to as ::core::ffi::c_int == '\n' as i32)
        {
            *to = ' ' as i32 as ::core::ffi::c_char;
        }
        if *to as ::core::ffi::c_int == '\r' as i32 {
            *to = (if normalize_attribute != 0 {
                ' ' as i32
            } else {
                '\n' as i32
            }) as ::core::ffi::c_char;
            if *from.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32 {
                from = from.offset(1);
            }
        }
        if *from as ::core::ffi::c_int == '&' as i32 {
            from = from.offset(1);
            if *from as ::core::ffi::c_int == '#' as i32 {
                let mut base: gint = 10 as gint;
                let mut l: gulong = 0;
                let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
                from = from.offset(1);
                if *from as ::core::ffi::c_int == 'x' as i32 {
                    base = 16 as ::core::ffi::c_int as gint;
                    from = from.offset(1);
                }
                *__errno_location() = 0 as ::core::ffi::c_int;
                l = strtoul(from, &raw mut end, base as ::core::ffi::c_int) as gulong;
                if end == from as *mut gchar || *__errno_location() != 0 as ::core::ffi::c_int {
                    safe_c2rust_set_unescape_error(
                        context,
                        error,
                        from as *const gchar,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Failed to parse \xE2\x80\x9C%-.*s\xE2\x80\x9D, which should have been a digit inside a character reference (&#234; for example) \xE2\x80\x94 perhaps the digit is too large\0"
                                as *const u8 as *const gchar,
                        ),
                        end.offset_from(from) as ::core::ffi::c_long
                            as ::core::ffi::c_int,
                        from,
                    );
                    return FALSE;
                } else if *end as ::core::ffi::c_int != ';' as i32 {
                    safe_c2rust_set_unescape_error(
                        context,
                        error,
                        from as *const gchar,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Character reference did not end with a semicolon; most likely you used an ampersand character without intending to start an entity \xE2\x80\x94 escape ampersand as &amp;\0"
                                as *const u8 as *const gchar,
                        ),
                    );
                    return FALSE;
                } else if (0 as gulong) < l && l <= 0xd7ff as gulong
                    || 0xe000 as gulong <= l && l <= 0xfffd as gulong
                    || 0x10000 as gulong <= l && l <= 0x10ffff as gulong
                {
                    let mut buf: [gchar; 8] = [0; 8];
                    safe_c2rust_char_str(l as gunichar, &raw mut buf as *mut gchar);
                    strcpy(to, &raw mut buf as *mut gchar);
                    to = to.offset(
                        strlen(&raw mut buf as *mut gchar).wrapping_sub(1 as size_t) as isize
                    );
                    from = end;
                    if l >= 0x80 as gulong {
                        mask = (mask as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                } else {
                    safe_c2rust_set_unescape_error(
                        context,
                        error,
                        from as *const gchar,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Character reference \xE2\x80\x9C%-.*s\xE2\x80\x9D does not encode a permitted character\0"
                                as *const u8 as *const gchar,
                        ),
                        end.offset_from(from) as ::core::ffi::c_long
                            as ::core::ffi::c_int,
                        from,
                    );
                    return FALSE;
                }
            } else if strncmp(
                from,
                b"lt;\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                *to = '<' as i32 as ::core::ffi::c_char;
                from = from.offset(2 as ::core::ffi::c_int as isize);
            } else if strncmp(
                from,
                b"gt;\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                *to = '>' as i32 as ::core::ffi::c_char;
                from = from.offset(2 as ::core::ffi::c_int as isize);
            } else if strncmp(
                from,
                b"amp;\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                *to = '&' as i32 as ::core::ffi::c_char;
                from = from.offset(3 as ::core::ffi::c_int as isize);
            } else if strncmp(
                from,
                b"quot;\0" as *const u8 as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                *to = '"' as i32 as ::core::ffi::c_char;
                from = from.offset(4 as ::core::ffi::c_int as isize);
            } else if strncmp(
                from,
                b"apos;\0" as *const u8 as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                *to = '\'' as i32 as ::core::ffi::c_char;
                from = from.offset(4 as ::core::ffi::c_int as isize);
            } else {
                if *from as ::core::ffi::c_int == ';' as i32 {
                    safe_c2rust_set_unescape_error(
                        context,
                        error,
                        from as *const gchar,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Empty entity \xE2\x80\x9C&;\xE2\x80\x9D seen; valid entities are: &amp; &quot; &lt; &gt; &apos;\0"
                                as *const u8 as *const gchar,
                        ),
                    );
                } else {
                    let mut end_0: *const ::core::ffi::c_char = strchr(from, ';' as i32);
                    if !end_0.is_null() {
                        safe_c2rust_set_unescape_error(
                            context,
                            error,
                            from as *const gchar,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Entity name \xE2\x80\x9C%-.*s\xE2\x80\x9D is not known\0"
                                    as *const u8 as *const gchar,
                            ),
                            end_0.offset_from(from) as ::core::ffi::c_long as ::core::ffi::c_int,
                            from,
                        );
                    } else {
                        safe_c2rust_set_unescape_error(
                            context,
                            error,
                            from as *const gchar,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Entity did not end with a semicolon; most likely you used an ampersand character without intending to start an entity \xE2\x80\x94 escape ampersand as &amp;\0"
                                    as *const u8 as *const gchar,
                            ),
                        );
                    }
                }
                return FALSE;
            }
        }
        from = from.offset(1);
        to = to.offset(1);
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if to.offset_from((*string).str_0) as ::core::ffi::c_long <= (*string).len as gssize {
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
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            735 as ::core::ffi::c_int,
            G_STRFUNC,
            b"to - string->str <= (gssize) string->len\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if to.offset_from((*string).str_0) as ::core::ffi::c_long != (*string).len as gssize {
        safe_c2rust_g_string_truncate_inline(
            string,
            to.offset_from((*string).str_0) as ::core::ffi::c_long as gsize,
        );
    }
    *is_ascii = (mask as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        as gboolean;
    return TRUE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_advance_char(mut context: *mut GMarkupParseContext) -> gboolean {
    (*context).iter = (*context).iter.offset(1);
    (*context).char_number += 1;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*context).iter == (*context).current_text_end {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
        return FALSE;
    } else if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if *(*context).iter as ::core::ffi::c_int == '\n' as i32 {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
        (*context).line_number += 1;
        (*context).char_number = 1 as ::core::ffi::c_int as gint;
    }
    return TRUE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_xml_isspace(mut c: ::core::ffi::c_char) -> gboolean {
    return (c as ::core::ffi::c_int == ' ' as i32
        || c as ::core::ffi::c_int == '\t' as i32
        || c as ::core::ffi::c_int == '\n' as i32
        || c as ::core::ffi::c_int == '\r' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_skip_spaces(mut context: *mut GMarkupParseContext) {
    loop {
        if safe_c2rust_xml_isspace(*(*context).iter) == 0 {
            return;
        }
        if !(safe_c2rust_advance_char(context) != 0) {
            break;
        }
    }
}
unsafe extern "C" fn safe_c2rust_advance_to_name_end(mut context: *mut GMarkupParseContext) {
    loop {
        if *(*context).iter as ::core::ffi::c_int == '=' as i32
            || *(*context).iter as ::core::ffi::c_int == '/' as i32
            || *(*context).iter as ::core::ffi::c_int == '>' as i32
            || *(*context).iter as ::core::ffi::c_int == ' ' as i32
        {
            return;
        }
        if safe_c2rust_xml_isspace(*(*context).iter) != 0 {
            return;
        }
        if !(safe_c2rust_advance_char(context) != 0) {
            break;
        }
    }
}
unsafe extern "C" fn safe_c2rust_release_chunk(
    mut context: *mut GMarkupParseContext,
    mut str: *mut GString,
) {
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if str.is_null() {
        return;
    }
    if (*str).allocated_len > 256 as gsize {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(str);
            };
        } else {
            g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return;
    }
    safe_c2rust_g_string_truncate_inline(str, 0 as gsize);
    node = safe_c2rust_get_list_node(context, str as gpointer);
    (*context).spare_chunks = g_slist_concat(node, (*context).spare_chunks);
}
unsafe extern "C" fn safe_c2rust_add_to_partial(
    mut context: *mut GMarkupParseContext,
    mut text_start: *const gchar,
    mut text_end: *const gchar,
) {
    if (*context).partial_chunk.is_null() {
        if !(*context).spare_chunks.is_null() {
            let mut node: *mut GSList = (*context).spare_chunks;
            (*context).spare_chunks = g_slist_remove_link((*context).spare_chunks, node);
            (*context).partial_chunk = (*node).data as *mut GString;
            safe_c2rust_free_list_node(context, node);
        } else {
            (*context).partial_chunk = g_string_sized_new(
                (if 28 as ::core::ffi::c_long
                    > text_end.offset_from(text_start) as ::core::ffi::c_long
                {
                    28 as ::core::ffi::c_long
                } else {
                    text_end.offset_from(text_start) as ::core::ffi::c_long
                }) as gsize,
            );
        }
    }
    if text_start != text_end {
        safe_c2rust_g_string_append_len_inline(
            (*context).partial_chunk,
            text_start as *const ::core::ffi::c_char,
            text_end.offset_from(text_start) as gssize,
        );
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_truncate_partial(mut context: *mut GMarkupParseContext) {
    if !(*context).partial_chunk.is_null() {
        safe_c2rust_g_string_truncate_inline((*context).partial_chunk, 0 as gsize);
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_current_element(
    mut context: *mut GMarkupParseContext,
) -> *const gchar {
    return (*(*context).tag_stack).data as *const gchar;
}
unsafe extern "C" fn safe_c2rust_pop_subparser_stack(mut context: *mut GMarkupParseContext) {
    let mut tracker: *mut GMarkupRecursionTracker =
        ::core::ptr::null_mut::<GMarkupRecursionTracker>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !(*context).subparser_stack.is_null() {
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
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            850 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->subparser_stack\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    tracker = (*(*context).subparser_stack).data as *mut GMarkupRecursionTracker;
    (*context).set_awaiting_pop(TRUE as guint as guint);
    (*context).held_user_data = (*context).user_data;
    (*context).user_data = (*tracker).prev_user_data;
    (*context).parser = (*tracker).prev_parser;
    (*context).subparser_element = (*tracker).prev_element;
    g_slice_free1(
        ::core::mem::size_of::<GMarkupRecursionTracker>() as gsize,
        tracker as gpointer,
    );
    (*context).subparser_stack =
        g_slist_delete_link((*context).subparser_stack, (*context).subparser_stack);
}
unsafe extern "C" fn safe_c2rust_push_partial_as_tag(mut context: *mut GMarkupParseContext) {
    let mut str: *mut GString = (*context).partial_chunk;
    (*context).tag_stack = g_slist_concat(
        safe_c2rust_get_list_node(context, (*str).str_0 as gpointer),
        (*context).tag_stack,
    );
    (*context).tag_stack_gstr = g_slist_concat(
        safe_c2rust_get_list_node(context, str as gpointer),
        (*context).tag_stack_gstr,
    );
    (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
}
unsafe extern "C" fn safe_c2rust_pop_tag(mut context: *mut GMarkupParseContext) {
    let mut nodea: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut nodeb: *mut GSList = ::core::ptr::null_mut::<GSList>();
    nodea = (*context).tag_stack;
    nodeb = (*context).tag_stack_gstr;
    safe_c2rust_release_chunk(context, (*nodeb).data as *mut GString);
    (*context).tag_stack = g_slist_remove_link((*context).tag_stack, nodea);
    (*context).tag_stack_gstr = g_slist_remove_link((*context).tag_stack_gstr, nodeb);
    safe_c2rust_free_list_node(context, nodea);
    safe_c2rust_free_list_node(context, nodeb);
}
unsafe extern "C" fn safe_c2rust_possibly_finish_subparser(mut context: *mut GMarkupParseContext) {
    if safe_c2rust_current_element(context) == (*context).subparser_element {
        safe_c2rust_pop_subparser_stack(context);
    }
}
unsafe extern "C" fn safe_c2rust_ensure_no_outstanding_subparser(
    mut context: *mut GMarkupParseContext,
) {
    if (*context).awaiting_pop() != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"During the first end_element call after invoking a subparser you must pop the subparser stack and handle the freeing of the subparser user_data.  This can be done by calling the end function of the subparser.  Very probably, your program just leaked memory.\0"
                as *const u8 as *const gchar,
        );
    }
    (*context).held_user_data = NULL_0 as gpointer;
    (*context).set_awaiting_pop(FALSE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_current_attribute(
    mut context: *mut GMarkupParseContext,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*context).cur_attr >= 0 as ::core::ffi::c_int {
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
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            915 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->cur_attr >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return (**(*context).attr_names.offset((*context).cur_attr as isize)).str_0;
}
unsafe extern "C" fn safe_c2rust_add_attribute(
    mut context: *mut GMarkupParseContext,
    mut str: *mut GString,
) -> gboolean {
    if (*context).cur_attr >= 1000 as ::core::ffi::c_int {
        return FALSE;
    }
    if (*context).cur_attr as ::core::ffi::c_int + 2 as ::core::ffi::c_int >= (*context).alloc_attrs
    {
        (*context).alloc_attrs += 5 as ::core::ffi::c_int;
        (*context).attr_names = g_realloc(
            (*context).attr_names as gpointer,
            (::core::mem::size_of::<*mut GString>() as gsize)
                .wrapping_mul((*context).alloc_attrs as gsize),
        ) as *mut *mut GString;
        (*context).attr_values = g_realloc(
            (*context).attr_values as gpointer,
            (::core::mem::size_of::<*mut GString>() as gsize)
                .wrapping_mul((*context).alloc_attrs as gsize),
        ) as *mut *mut GString;
    }
    (*context).cur_attr += 1;
    let ref mut fresh8 = *(*context).attr_names.offset((*context).cur_attr as isize);
    *fresh8 = str;
    let ref mut fresh9 = *(*context).attr_values.offset((*context).cur_attr as isize);
    *fresh9 = ::core::ptr::null_mut::<GString>();
    let ref mut fresh10 = *(*context)
        .attr_names
        .offset(((*context).cur_attr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    *fresh10 = ::core::ptr::null_mut::<GString>();
    let ref mut fresh11 = *(*context)
        .attr_values
        .offset(((*context).cur_attr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    *fresh11 = ::core::ptr::null_mut::<GString>();
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_clear_attributes(mut context: *mut GMarkupParseContext) {
    while (*context).cur_attr >= 0 as ::core::ffi::c_int {
        let mut pos: ::core::ffi::c_int = (*context).cur_attr as ::core::ffi::c_int;
        safe_c2rust_release_chunk(context, *(*context).attr_names.offset(pos as isize));
        safe_c2rust_release_chunk(context, *(*context).attr_values.offset(pos as isize));
        let ref mut fresh0 = *(*context).attr_values.offset(pos as isize);
        *fresh0 = ::core::ptr::null_mut::<GString>();
        let ref mut fresh1 = *(*context).attr_names.offset(pos as isize);
        *fresh1 = *fresh0;
        (*context).cur_attr -= 1;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if (*context).cur_attr == -(1 as ::core::ffi::c_int) {
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
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            952 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->cur_attr == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if (*context).attr_names.is_null()
            || (*(*context)
                .attr_names
                .offset(0 as ::core::ffi::c_int as isize))
            .is_null()
        {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            954 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->attr_names == NULL || context->attr_names[0] == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*context).attr_values.is_null()
            || (*(*context)
                .attr_values
                .offset(0 as ::core::ffi::c_int as isize))
            .is_null()
        {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->attr_values == NULL || context->attr_values[0] == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust_emit_start_element(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut attr_names: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut attr_values: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*context).flags as ::core::ffi::c_uint
        & G_MARKUP_IGNORE_QUALIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !strchr(
            safe_c2rust_current_element(context) as *const ::core::ffi::c_char,
            ':' as i32,
        )
        .is_null()
    {
        static mut safe_c2rust_ignore_parser: GMarkupParser = _GMarkupParser {
            start_element: None,
            end_element: None,
            text: None,
            passthrough: None,
            error: None,
        };
        safe_c2rust_g_markup_parse_context_push(
            context,
            &raw const safe_c2rust_ignore_parser,
            NULL_0,
        );
        safe_c2rust_clear_attributes(context);
        return;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*const gchar>() as usize).wrapping_mul(
            ((*context).cur_attr as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize,
        ) as usize,
    ));
    attr_names = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut *const gchar;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*const gchar>() as usize).wrapping_mul(
            ((*context).cur_attr as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize,
        ) as usize,
    ));
    attr_values = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut *const gchar;
    i = 0 as ::core::ffi::c_int;
    while i < (*context).cur_attr as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        if !((*context).flags as ::core::ffi::c_uint
            & G_MARKUP_IGNORE_QUALIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && !strchr(
                (**(*context).attr_names.offset(i as isize)).str_0,
                ':' as i32,
            )
            .is_null())
        {
            let ref mut fresh3 = *attr_names.offset(j as isize);
            *fresh3 = (**(*context).attr_names.offset(i as isize)).str_0;
            let ref mut fresh4 = *attr_values.offset(j as isize);
            *fresh4 = (**(*context).attr_values.offset(i as isize)).str_0;
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh5 = *attr_names.offset(j as isize);
    *fresh5 = ::core::ptr::null::<gchar>();
    let ref mut fresh6 = *attr_values.offset(j as isize);
    *fresh6 = ::core::ptr::null::<gchar>();
    tmp_error = ::core::ptr::null_mut::<GError>();
    start_name = safe_c2rust_current_element(context);
    if safe_c2rust_name_validate(context, start_name, error) == 0 {
        return;
    }
    if (*(*context).parser).start_element.is_some() {
        Some(
            (*(*context).parser)
                .start_element
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            context,
            start_name,
            attr_names,
            attr_values,
            (*context).user_data,
            &raw mut tmp_error,
        );
    }
    safe_c2rust_clear_attributes(context);
    if !tmp_error.is_null() {
        safe_c2rust_propagate_error(context, error, tmp_error);
    }
}
unsafe extern "C" fn safe_c2rust_emit_end_element(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
) {
    let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !(*context).tag_stack.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            1031 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->tag_stack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_possibly_finish_subparser(context);
    if (*context).flags as ::core::ffi::c_uint
        & G_MARKUP_IGNORE_QUALIFIED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !strchr(
            safe_c2rust_current_element(context) as *const ::core::ffi::c_char,
            ':' as i32,
        )
        .is_null()
    {
        safe_c2rust_g_markup_parse_context_pop(context);
        safe_c2rust_pop_tag(context);
        return;
    }
    tmp_error = ::core::ptr::null_mut::<GError>();
    if (*(*context).parser).end_element.is_some() {
        Some(
            (*(*context).parser)
                .end_element
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            context,
            safe_c2rust_current_element(context),
            (*context).user_data,
            &raw mut tmp_error,
        );
    }
    safe_c2rust_ensure_no_outstanding_subparser(context);
    if !tmp_error.is_null() {
        safe_c2rust_mark_error(context, tmp_error);
        g_propagate_error(error, tmp_error);
    }
    safe_c2rust_pop_tag(context);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_parse(
    mut context: *mut GMarkupParseContext,
    mut text: *const gchar,
    mut text_len: gssize,
    mut error: *mut *mut GError,
) -> gboolean {
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
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !text.is_null() {
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
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*context).state as ::core::ffi::c_uint
            != STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"context->state != STATE_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*context).parsing() == 0 {
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
            b"!context->parsing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if text_len < 0 as gssize {
        text_len = strlen(text as *const ::core::ffi::c_char) as gssize;
    }
    if text_len == 0 as gssize {
        return TRUE;
    }
    (*context).set_parsing(TRUE as guint as guint);
    (*context).current_text = text;
    (*context).current_text_len = text_len;
    (*context).current_text_end = (*context).current_text.offset(text_len as isize);
    (*context).iter = (*context).current_text;
    (*context).start = (*context).iter;
    while (*context).iter != (*context).current_text_end {
        match (*context).state as ::core::ffi::c_uint {
            0 => {
                if ({
                    let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                    if (*context).tag_stack.is_null() {
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
                        b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1114 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"context->tag_stack == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_skip_spaces(context);
                if (*context).iter != (*context).current_text_end {
                    if *(*context).iter as ::core::ffi::c_int == '<' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_AFTER_OPEN_ANGLE;
                        (*context).start = (*context).iter;
                        (*context).set_document_empty(FALSE as guint as guint);
                    } else {
                        safe_c2rust_set_error_literal(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Document must begin with an element (e.g. <book>)\0" as *const u8
                                    as *const gchar,
                            ),
                        );
                    }
                }
            }
            1 => {
                if *(*context).iter as ::core::ffi::c_int == '?' as i32
                    || *(*context).iter as ::core::ffi::c_int == '!' as i32
                {
                    let mut openangle: *const gchar = b"<\0" as *const u8 as *const gchar;
                    safe_c2rust_add_to_partial(
                        context,
                        openangle,
                        openangle.offset(1 as ::core::ffi::c_int as isize),
                    );
                    (*context).start = (*context).iter;
                    (*context).balance = 1 as ::core::ffi::c_int as gint;
                    (*context).state = STATE_INSIDE_PASSTHROUGH;
                } else if *(*context).iter as ::core::ffi::c_int == '/' as i32 {
                    safe_c2rust_advance_char(context);
                    (*context).state = STATE_AFTER_CLOSE_TAG_SLASH;
                } else if !(*(*context).iter as ::core::ffi::c_int == '=' as i32
                    || *(*context).iter as ::core::ffi::c_int == '/' as i32
                    || *(*context).iter as ::core::ffi::c_int == '>' as i32
                    || *(*context).iter as ::core::ffi::c_int == ' ' as i32)
                {
                    (*context).state = STATE_INSIDE_OPEN_TAG_NAME;
                    (*context).start = (*context).iter;
                } else {
                    let mut buf: [gchar; 8] = [0; 8];
                    safe_c2rust_set_error(
                        context,
                        error,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"\xE2\x80\x9C%s\xE2\x80\x9D is not a valid character following a \xE2\x80\x9C<\xE2\x80\x9D character; it may not begin an element name\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_utf8_str(
                            (*context).iter,
                            (*context).current_text_end.offset_from((*context).iter)
                                as ::core::ffi::c_long as gsize,
                            &raw mut buf as *mut gchar,
                        ),
                    );
                }
            }
            2 => {
                if (*context).tag_stack.is_null() {
                    (*context).start = ::core::ptr::null::<gchar>();
                    (*context).state = STATE_START;
                } else {
                    (*context).start = (*context).iter;
                    (*context).state = STATE_INSIDE_TEXT;
                }
            }
            3 => {
                if *(*context).iter as ::core::ffi::c_int == '>' as i32 {
                    safe_c2rust_advance_char(context);
                    (*context).state = STATE_AFTER_CLOSE_ANGLE;
                    safe_c2rust_emit_end_element(context, error);
                } else {
                    let mut buf_0: [gchar; 8] = [0; 8];
                    safe_c2rust_set_error(
                        context,
                        error,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Odd character \xE2\x80\x9C%s\xE2\x80\x9D, expected a \xE2\x80\x9C>\xE2\x80\x9D character to end the empty-element tag \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_utf8_str(
                            (*context).iter,
                            (*context).current_text_end.offset_from((*context).iter)
                                as ::core::ffi::c_long as gsize,
                            &raw mut buf_0 as *mut gchar,
                        ),
                        safe_c2rust_current_element(context),
                    );
                }
            }
            4 => {
                safe_c2rust_advance_to_name_end(context);
                if (*context).iter == (*context).current_text_end {
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                } else {
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                    safe_c2rust_push_partial_as_tag(context);
                    (*context).state = STATE_BETWEEN_ATTRIBUTES;
                    (*context).start = ::core::ptr::null::<gchar>();
                }
            }
            5 => {
                safe_c2rust_advance_to_name_end(context);
                safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                if (*context).iter != (*context).current_text_end {
                    (*context).state = STATE_AFTER_ATTRIBUTE_NAME;
                }
            }
            6 => {
                safe_c2rust_skip_spaces(context);
                if !((*context).iter != (*context).current_text_end) {
                    continue;
                }
                if safe_c2rust_name_validate(context, (*(*context).partial_chunk).str_0, error) == 0
                {
                    continue;
                }
                if safe_c2rust_add_attribute(context, (*context).partial_chunk) == 0 {
                    safe_c2rust_set_error(
                        context,
                        error,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"Too many attributes in element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_current_element(context),
                    );
                } else {
                    (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
                    (*context).start = ::core::ptr::null::<gchar>();
                    if *(*context).iter as ::core::ffi::c_int == '=' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_AFTER_ATTRIBUTE_EQUALS_SIGN;
                    } else {
                        let mut buf_1: [gchar; 8] = [0; 8];
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Odd character \xE2\x80\x9C%s\xE2\x80\x9D, expected a \xE2\x80\x9C=\xE2\x80\x9D after attribute name \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                    as *const u8 as *const gchar,
                            ),
                            safe_c2rust_utf8_str(
                                (*context).iter,
                                (*context).current_text_end.offset_from((*context).iter)
                                    as ::core::ffi::c_long as gsize,
                                &raw mut buf_1 as *mut gchar,
                            ),
                            safe_c2rust_current_attribute(context),
                            safe_c2rust_current_element(context),
                        );
                    }
                }
            }
            7 => {
                safe_c2rust_skip_spaces(context);
                if (*context).iter != (*context).current_text_end {
                    if *(*context).iter as ::core::ffi::c_int == '/' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_AFTER_ELISION_SLASH;
                    } else if *(*context).iter as ::core::ffi::c_int == '>' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_AFTER_CLOSE_ANGLE;
                    } else if !(*(*context).iter as ::core::ffi::c_int == '=' as i32
                        || *(*context).iter as ::core::ffi::c_int == '/' as i32
                        || *(*context).iter as ::core::ffi::c_int == '>' as i32
                        || *(*context).iter as ::core::ffi::c_int == ' ' as i32)
                    {
                        (*context).state = STATE_INSIDE_ATTRIBUTE_NAME;
                        (*context).start = (*context).iter;
                    } else {
                        let mut buf_2: [gchar; 8] = [0; 8];
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Odd character \xE2\x80\x9C%s\xE2\x80\x9D, expected a \xE2\x80\x9C>\xE2\x80\x9D or \xE2\x80\x9C/\xE2\x80\x9D character to end the start tag of element \xE2\x80\x9C%s\xE2\x80\x9D, or optionally an attribute; perhaps you used an invalid character in an attribute name\0"
                                    as *const u8 as *const gchar,
                            ),
                            safe_c2rust_utf8_str(
                                (*context).iter,
                                (*context).current_text_end.offset_from((*context).iter)
                                    as ::core::ffi::c_long as gsize,
                                &raw mut buf_2 as *mut gchar,
                            ),
                            safe_c2rust_current_element(context),
                        );
                    }
                    if (*context).state as ::core::ffi::c_uint
                        == STATE_AFTER_ELISION_SLASH as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*context).state as ::core::ffi::c_uint
                            == STATE_AFTER_CLOSE_ANGLE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        safe_c2rust_emit_start_element(context, error);
                    }
                }
            }
            8 => {
                safe_c2rust_skip_spaces(context);
                if (*context).iter != (*context).current_text_end {
                    if *(*context).iter as ::core::ffi::c_int == '"' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_INSIDE_ATTRIBUTE_VALUE_DQ;
                        (*context).start = (*context).iter;
                    } else if *(*context).iter as ::core::ffi::c_int == '\'' as i32 {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_INSIDE_ATTRIBUTE_VALUE_SQ;
                        (*context).start = (*context).iter;
                    } else {
                        let mut buf_3: [gchar; 8] = [0; 8];
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Odd character \xE2\x80\x9C%s\xE2\x80\x9D, expected an open quote mark after the equals sign when giving value for attribute \xE2\x80\x9C%s\xE2\x80\x9D of element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                    as *const u8 as *const gchar,
                            ),
                            safe_c2rust_utf8_str(
                                (*context).iter,
                                (*context).current_text_end.offset_from((*context).iter)
                                    as ::core::ffi::c_long as gsize,
                                &raw mut buf_3 as *mut gchar,
                            ),
                            safe_c2rust_current_attribute(context),
                            safe_c2rust_current_element(context),
                        );
                    }
                }
            }
            9 | 10 => {
                let mut delim: gchar = 0;
                if (*context).state as ::core::ffi::c_uint
                    == STATE_INSIDE_ATTRIBUTE_VALUE_SQ as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    delim = '\'' as i32 as gchar;
                } else {
                    delim = '"' as i32 as gchar;
                }
                while !(*(*context).iter as ::core::ffi::c_int == delim as ::core::ffi::c_int) {
                    if !(safe_c2rust_advance_char(context) != 0) {
                        break;
                    }
                }
                if (*context).iter == (*context).current_text_end {
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                } else {
                    let mut is_ascii: gboolean = 0;
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                    if ({
                        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
                        if (*context).cur_attr >= 0 as ::core::ffi::c_int {
                            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_34
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gmarkup.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1448 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"context->cur_attr >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if safe_c2rust_unescape_gstring_inplace(
                        context,
                        (*context).partial_chunk,
                        &raw mut is_ascii,
                        error,
                    ) != 0
                        && (is_ascii != 0
                            || safe_c2rust_text_validate(
                                context,
                                (*(*context).partial_chunk).str_0,
                                (*(*context).partial_chunk).len as gint,
                                error,
                            ) != 0)
                    {
                        let ref mut fresh2 =
                            *(*context).attr_values.offset((*context).cur_attr as isize);
                        *fresh2 = (*context).partial_chunk;
                        (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_BETWEEN_ATTRIBUTES;
                        (*context).start = ::core::ptr::null::<gchar>();
                    }
                    safe_c2rust_truncate_partial(context);
                }
            }
            11 => {
                while !(*(*context).iter as ::core::ffi::c_int == '<' as i32) {
                    if !(safe_c2rust_advance_char(context) != 0) {
                        break;
                    }
                }
                safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                if (*context).iter != (*context).current_text_end {
                    let mut is_ascii_0: gboolean = 0;
                    if safe_c2rust_unescape_gstring_inplace(
                        context,
                        (*context).partial_chunk,
                        &raw mut is_ascii_0,
                        error,
                    ) != 0
                        && (is_ascii_0 != 0
                            || safe_c2rust_text_validate(
                                context,
                                (*(*context).partial_chunk).str_0,
                                (*(*context).partial_chunk).len as gint,
                                error,
                            ) != 0)
                    {
                        let mut tmp_error: *mut GError = ::core::ptr::null_mut::<GError>();
                        if (*(*context).parser).text.is_some() {
                            Some(
                                (*(*context).parser)
                                    .text
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                context,
                                (*(*context).partial_chunk).str_0,
                                (*(*context).partial_chunk).len,
                                (*context).user_data,
                                &raw mut tmp_error,
                            );
                        }
                        if tmp_error.is_null() {
                            safe_c2rust_advance_char(context);
                            (*context).state = STATE_AFTER_OPEN_ANGLE;
                            (*context).start = (*context).iter;
                        } else {
                            safe_c2rust_propagate_error(context, error, tmp_error);
                        }
                    }
                    safe_c2rust_truncate_partial(context);
                }
            }
            12 => {
                if !(*(*context).iter as ::core::ffi::c_int == '=' as i32
                    || *(*context).iter as ::core::ffi::c_int == '/' as i32
                    || *(*context).iter as ::core::ffi::c_int == '>' as i32
                    || *(*context).iter as ::core::ffi::c_int == ' ' as i32)
                {
                    (*context).state = STATE_INSIDE_CLOSE_TAG_NAME;
                    (*context).start = (*context).iter;
                } else {
                    let mut buf_4: [gchar; 8] = [0; 8];
                    safe_c2rust_set_error(
                        context,
                        error,
                        G_MARKUP_ERROR_PARSE,
                        glib_gettext(
                            b"\xE2\x80\x9C%s\xE2\x80\x9D is not a valid character following the characters \xE2\x80\x9C</\xE2\x80\x9D; \xE2\x80\x9C%s\xE2\x80\x9D may not begin an element name\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_utf8_str(
                            (*context).iter,
                            (*context).current_text_end.offset_from((*context).iter)
                                as ::core::ffi::c_long as gsize,
                            &raw mut buf_4 as *mut gchar,
                        ),
                        safe_c2rust_utf8_str(
                            (*context).iter,
                            (*context).current_text_end.offset_from((*context).iter)
                                as ::core::ffi::c_long as gsize,
                            &raw mut buf_4 as *mut gchar,
                        ),
                    );
                }
            }
            13 => {
                safe_c2rust_advance_to_name_end(context);
                safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                if (*context).iter != (*context).current_text_end {
                    (*context).state = STATE_AFTER_CLOSE_TAG_NAME;
                }
            }
            14 => {
                safe_c2rust_skip_spaces(context);
                if (*context).iter != (*context).current_text_end {
                    let mut close_name: *mut GString = ::core::ptr::null_mut::<GString>();
                    close_name = (*context).partial_chunk;
                    (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
                    if *(*context).iter as ::core::ffi::c_int != '>' as i32 {
                        let mut buf_5: [gchar; 8] = [0; 8];
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"\xE2\x80\x9C%s\xE2\x80\x9D is not a valid character following the close element name \xE2\x80\x9C%s\xE2\x80\x9D; the allowed character is \xE2\x80\x9C>\xE2\x80\x9D\0"
                                    as *const u8 as *const gchar,
                            ),
                            safe_c2rust_utf8_str(
                                (*context).iter,
                                (*context).current_text_end.offset_from((*context).iter)
                                    as ::core::ffi::c_long as gsize,
                                &raw mut buf_5 as *mut gchar,
                            ),
                            (*close_name).str_0,
                        );
                    } else if (*context).tag_stack.is_null() {
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Element \xE2\x80\x9C%s\xE2\x80\x9D was closed, no element is currently open\0"
                                    as *const u8 as *const gchar,
                            ),
                            (*close_name).str_0,
                        );
                    } else if strcmp(
                        (*close_name).str_0,
                        safe_c2rust_current_element(context) as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        safe_c2rust_set_error(
                            context,
                            error,
                            G_MARKUP_ERROR_PARSE,
                            glib_gettext(
                                b"Element \xE2\x80\x9C%s\xE2\x80\x9D was closed, but the currently open element is \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                    as *const u8 as *const gchar,
                            ),
                            (*close_name).str_0,
                            safe_c2rust_current_element(context),
                        );
                    } else {
                        safe_c2rust_advance_char(context);
                        (*context).state = STATE_AFTER_CLOSE_ANGLE;
                        (*context).start = ::core::ptr::null::<gchar>();
                        safe_c2rust_emit_end_element(context, error);
                    }
                    (*context).partial_chunk = close_name;
                    safe_c2rust_truncate_partial(context);
                }
            }
            15 => {
                loop {
                    if *(*context).iter as ::core::ffi::c_int == '<' as i32 {
                        (*context).balance += 1;
                    }
                    if *(*context).iter as ::core::ffi::c_int == '>' as i32 {
                        let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
                        let mut len: gsize = 0;
                        (*context).balance -= 1;
                        safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                        (*context).start = (*context).iter;
                        str = (*(*context).partial_chunk).str_0;
                        len = (*(*context).partial_chunk).len;
                        if *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '?' as i32
                            && *str.offset(len.wrapping_sub(1 as gsize) as isize)
                                as ::core::ffi::c_int
                                == '?' as i32
                        {
                            break;
                        }
                        if strncmp(
                            str,
                            b"<!--\0" as *const u8 as *const ::core::ffi::c_char,
                            4 as size_t,
                        ) == 0 as ::core::ffi::c_int
                            && strcmp(
                                str.offset(len as isize)
                                    .offset(-(2 as ::core::ffi::c_int as isize)),
                                b"--\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        if strncmp(
                            str,
                            b"<![CDATA[\0" as *const u8 as *const ::core::ffi::c_char,
                            9 as size_t,
                        ) == 0 as ::core::ffi::c_int
                            && strcmp(
                                str.offset(len as isize)
                                    .offset(-(2 as ::core::ffi::c_int as isize)),
                                b"]]\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        if strncmp(
                            str,
                            b"<!DOCTYPE\0" as *const u8 as *const ::core::ffi::c_char,
                            9 as size_t,
                        ) == 0 as ::core::ffi::c_int
                            && (*context).balance == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(safe_c2rust_advance_char(context) != 0) {
                        break;
                    }
                }
                if (*context).iter == (*context).current_text_end {
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                } else {
                    let mut tmp_error_0: *mut GError = ::core::ptr::null_mut::<GError>();
                    safe_c2rust_advance_char(context);
                    safe_c2rust_add_to_partial(context, (*context).start, (*context).iter);
                    if (*context).flags as ::core::ffi::c_uint
                        & G_MARKUP_TREAT_CDATA_AS_TEXT as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                        && strncmp(
                            (*(*context).partial_chunk).str_0,
                            b"<![CDATA[\0" as *const u8 as *const ::core::ffi::c_char,
                            9 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        if (*(*context).parser).text.is_some()
                            && safe_c2rust_text_validate(
                                context,
                                (*(*context).partial_chunk)
                                    .str_0
                                    .offset(9 as ::core::ffi::c_int as isize),
                                (*(*context).partial_chunk).len.wrapping_sub(12 as gsize) as gint,
                                error,
                            ) != 0
                        {
                            Some(
                                (*(*context).parser)
                                    .text
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                context,
                                (*(*context).partial_chunk)
                                    .str_0
                                    .offset(9 as ::core::ffi::c_int as isize),
                                (*(*context).partial_chunk).len.wrapping_sub(12 as gsize),
                                (*context).user_data,
                                &raw mut tmp_error_0,
                            );
                        }
                    } else if (*(*context).parser).passthrough.is_some()
                        && safe_c2rust_text_validate(
                            context,
                            (*(*context).partial_chunk).str_0,
                            (*(*context).partial_chunk).len as gint,
                            error,
                        ) != 0
                    {
                        Some(
                            (*(*context).parser)
                                .passthrough
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            context,
                            (*(*context).partial_chunk).str_0,
                            (*(*context).partial_chunk).len,
                            (*context).user_data,
                            &raw mut tmp_error_0,
                        );
                    }
                    safe_c2rust_truncate_partial(context);
                    if tmp_error_0.is_null() {
                        (*context).state = STATE_AFTER_CLOSE_ANGLE;
                        (*context).start = (*context).iter;
                    } else {
                        safe_c2rust_propagate_error(context, error, tmp_error_0);
                    }
                }
            }
            16 => {
                break;
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    (*context).set_parsing(FALSE as guint as guint);
    return ((*context).state as ::core::ffi::c_uint
        != STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_end_parse(
    mut context: *mut GMarkupParseContext,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*context).parsing() == 0 {
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
            b"!context->parsing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if (*context).state as ::core::ffi::c_uint
            != STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"context->state != STATE_ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !(*context).partial_chunk.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    (*context).partial_chunk,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal((*context).partial_chunk);
            };
        } else {
            g_string_free(
                (*context).partial_chunk,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        (*context).partial_chunk = ::core::ptr::null_mut::<GString>();
    }
    if (*context).document_empty() != 0 {
        safe_c2rust_set_error_literal(
            context,
            error,
            G_MARKUP_ERROR_EMPTY,
            glib_gettext(
                b"Document was empty or contained only whitespace\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    (*context).set_parsing(TRUE as guint as guint);
    match (*context).state as ::core::ffi::c_uint {
        0 => {}
        1 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly just after an open angle bracket \xE2\x80\x9C<\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
            );
        }
        2 => {
            if !(*context).tag_stack.is_null() {
                safe_c2rust_set_error(
                    context,
                    error,
                    G_MARKUP_ERROR_PARSE,
                    glib_gettext(
                        b"Document ended unexpectedly with elements still open \xE2\x80\x94 \xE2\x80\x9C%s\xE2\x80\x9D was the last element opened\0"
                            as *const u8 as *const gchar,
                    ),
                    safe_c2rust_current_element(context),
                );
            }
        }
        3 => {
            safe_c2rust_set_error(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly, expected to see a close angle bracket ending the tag <%s/>\0"
                        as *const u8 as *const gchar,
                ),
                safe_c2rust_current_element(context),
            );
        }
        4 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly inside an element name\0" as *const u8
                        as *const gchar,
                ),
            );
        }
        5 | 6 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly inside an attribute name\0" as *const u8
                        as *const gchar,
                ),
            );
        }
        7 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly inside an element-opening tag.\0" as *const u8
                        as *const gchar,
                ),
            );
        }
        8 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly after the equals sign following an attribute name; no attribute value\0"
                        as *const u8 as *const gchar,
                ),
            );
        }
        9 | 10 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly while inside an attribute value\0" as *const u8
                        as *const gchar,
                ),
            );
        }
        11 => {
            if ({
                let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
                if !(*context).tag_stack.is_null() {
                    _g_boolean_var_38 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_38 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_38
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1810 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"context->tag_stack != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_set_error(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly with elements still open \xE2\x80\x94 \xE2\x80\x9C%s\xE2\x80\x9D was the last element opened\0"
                        as *const u8 as *const gchar,
                ),
                safe_c2rust_current_element(context),
            );
        }
        12 | 13 | 14 => {
            if !(*context).tag_stack.is_null() {
                safe_c2rust_set_error(
                    context,
                    error,
                    G_MARKUP_ERROR_PARSE,
                    glib_gettext(
                        b"Document ended unexpectedly inside the close tag for element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    safe_c2rust_current_element(context),
                );
            } else {
                safe_c2rust_set_error(
                    context,
                    error,
                    G_MARKUP_ERROR_PARSE,
                    glib_gettext(
                        b"Document ended unexpectedly inside the close tag for an unopened element\0"
                            as *const u8 as *const gchar,
                    ),
                );
            }
        }
        15 => {
            safe_c2rust_set_error_literal(
                context,
                error,
                G_MARKUP_ERROR_PARSE,
                glib_gettext(
                    b"Document ended unexpectedly inside a comment or processing instruction\0"
                        as *const u8 as *const gchar,
                ),
            );
        }
        16 | _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
                1838 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    (*context).set_parsing(FALSE as guint as guint);
    return ((*context).state as ::core::ffi::c_uint
        != STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_get_element(
    mut context: *mut GMarkupParseContext,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if (*context).tag_stack.is_null() {
        return ::core::ptr::null::<gchar>();
    } else {
        return safe_c2rust_current_element(context);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_get_element_stack(
    mut context: *mut GMarkupParseContext,
) -> *const GSList {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GSList>();
    }
    return (*context).tag_stack;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_get_position(
    mut context: *mut GMarkupParseContext,
    mut line_number: *mut gint,
    mut char_number: *mut gint,
) {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !context.is_null() {
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
            b"context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !line_number.is_null() {
        *line_number = (*context).line_number;
    }
    if !char_number.is_null() {
        *char_number = (*context).char_number;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_get_user_data(
    mut context: *mut GMarkupParseContext,
) -> gpointer {
    return (*context).user_data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_push(
    mut context: *mut GMarkupParseContext,
    mut parser: *const GMarkupParser,
    mut user_data: gpointer,
) {
    let mut tracker: *mut GMarkupRecursionTracker =
        ::core::ptr::null_mut::<GMarkupRecursionTracker>();
    tracker = g_slice_alloc(::core::mem::size_of::<GMarkupRecursionTracker>() as gsize)
        as *mut GMarkupRecursionTracker;
    (*tracker).prev_element = (*context).subparser_element;
    (*tracker).prev_parser = (*context).parser;
    (*tracker).prev_user_data = (*context).user_data;
    (*context).subparser_element =
        safe_c2rust_current_element(context) as *const ::core::ffi::c_char;
    (*context).parser = parser;
    (*context).user_data = user_data;
    (*context).subparser_stack = g_slist_prepend((*context).subparser_stack, tracker as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_parse_context_pop(
    mut context: *mut GMarkupParseContext,
) -> gpointer {
    let mut user_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*context).awaiting_pop() == 0 {
        safe_c2rust_possibly_finish_subparser(context);
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if (*context).awaiting_pop() != 0 {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
            2119 as ::core::ffi::c_int,
            G_STRFUNC,
            b"context->awaiting_pop\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*context).set_awaiting_pop(FALSE as guint as guint);
    user_data = (*context).held_user_data;
    (*context).held_user_data = NULL_0 as gpointer;
    return user_data;
}
unsafe extern "C" fn safe_c2rust_append_escaped_text(
    mut str: *mut GString,
    mut text: *const gchar,
    mut length: gssize,
) {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut pending: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    pending = text;
    p = pending;
    end = text.offset(length as isize);
    while p < end && pending < end {
        let mut c: guchar = *pending as guchar;
        match c as ::core::ffi::c_int {
            38 => {
                if pending > p {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        p as *const ::core::ffi::c_char,
                        pending.offset_from(p) as gssize,
                    );
                }
                pending = pending.offset(1);
                p = pending;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"&amp;\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
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
                        str,
                        b"&amp;\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            60 => {
                if pending > p {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        p as *const ::core::ffi::c_char,
                        pending.offset_from(p) as gssize,
                    );
                }
                pending = pending.offset(1);
                p = pending;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"&lt;\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
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
                        str,
                        b"&lt;\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            62 => {
                if pending > p {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        p as *const ::core::ffi::c_char,
                        pending.offset_from(p) as gssize,
                    );
                }
                pending = pending.offset(1);
                p = pending;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"&gt;\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
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
                        str,
                        b"&gt;\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            39 => {
                if pending > p {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        p as *const ::core::ffi::c_char,
                        pending.offset_from(p) as gssize,
                    );
                }
                pending = pending.offset(1);
                p = pending;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"&apos;\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
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
                        str,
                        b"&apos;\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            34 => {
                if pending > p {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        p as *const ::core::ffi::c_char,
                        pending.offset_from(p) as gssize,
                    );
                }
                pending = pending.offset(1);
                p = pending;
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"&quot;\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
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
                        str,
                        b"&quot;\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            _ => {
                if 0x1 as ::core::ffi::c_int <= c as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= 0x8 as ::core::ffi::c_int
                    || 0xb as ::core::ffi::c_int <= c as ::core::ffi::c_int
                        && c as ::core::ffi::c_int <= 0xc as ::core::ffi::c_int
                    || 0xe as ::core::ffi::c_int <= c as ::core::ffi::c_int
                        && c as ::core::ffi::c_int <= 0x1f as ::core::ffi::c_int
                    || c as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
                {
                    if pending > p {
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            p as *const ::core::ffi::c_char,
                            pending.offset_from(p) as gssize,
                        );
                    }
                    pending = pending.offset(1);
                    p = pending;
                    g_string_append_printf(
                        str,
                        b"&#x%x;\0" as *const u8 as *const gchar,
                        c as ::core::ffi::c_int,
                    );
                } else if c as ::core::ffi::c_int == 0xc2 as ::core::ffi::c_int {
                    let mut u: gunichar = g_utf8_get_char(pending);
                    if (0x7f as gunichar) < u && u <= 0x84 as gunichar
                        || 0x86 as gunichar <= u && u <= 0x9f as gunichar
                    {
                        if pending > p {
                            safe_c2rust_g_string_append_len_inline(
                                str,
                                p as *const ::core::ffi::c_char,
                                pending.offset_from(p) as gssize,
                            );
                        }
                        pending = pending.offset(1);
                        p = pending;
                        g_string_append_printf(str, b"&#x%x;\0" as *const u8 as *const gchar, u);
                        p = p.offset(1);
                    } else {
                        pending = pending.offset(1);
                    }
                } else {
                    pending = pending.offset(1);
                }
            }
        }
    }
    if pending > p {
        safe_c2rust_g_string_append_len_inline(
            str,
            p as *const ::core::ffi::c_char,
            pending.offset_from(p) as gssize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_escape_text(
    mut text: *const gchar,
    mut length: gssize,
) -> *mut gchar {
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !text.is_null() {
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
            b"text != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if length < 0 as gssize {
        length = strlen(text as *const ::core::ffi::c_char) as gssize;
    }
    str = g_string_sized_new(length as gsize);
    safe_c2rust_append_escaped_text(str, text, length);
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
unsafe extern "C" fn safe_c2rust_find_conversion(
    mut format: *const ::core::ffi::c_char,
    mut after: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut start: *const ::core::ffi::c_char = format;
    let mut cp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    while *start as ::core::ffi::c_int != '\0' as i32 && *start as ::core::ffi::c_int != '%' as i32
    {
        start = start.offset(1);
    }
    if *start as ::core::ffi::c_int == '\0' as i32 {
        *after = start;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    cp = start.offset(1 as ::core::ffi::c_int as isize);
    if *cp as ::core::ffi::c_int == '\0' as i32 {
        *after = cp;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if *cp as ::core::ffi::c_int >= '0' as i32 && *cp as ::core::ffi::c_int <= '9' as i32 {
        let mut np: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        np = cp;
        while *np as ::core::ffi::c_int >= '0' as i32 && *np as ::core::ffi::c_int <= '9' as i32 {
            np = np.offset(1);
        }
        if *np as ::core::ffi::c_int == '$' as i32 {
            cp = np.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    while *cp as ::core::ffi::c_int == '\'' as i32
        || *cp as ::core::ffi::c_int == '-' as i32
        || *cp as ::core::ffi::c_int == '+' as i32
        || *cp as ::core::ffi::c_int == ' ' as i32
        || *cp as ::core::ffi::c_int == '#' as i32
        || *cp as ::core::ffi::c_int == '0' as i32
    {
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '*' as i32 {
        cp = cp.offset(1);
        if *cp as ::core::ffi::c_int >= '0' as i32 && *cp as ::core::ffi::c_int <= '9' as i32 {
            let mut np_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            np_0 = cp;
            while *np_0 as ::core::ffi::c_int >= '0' as i32
                && *np_0 as ::core::ffi::c_int <= '9' as i32
            {
                np_0 = np_0.offset(1);
            }
            if *np_0 as ::core::ffi::c_int == '$' as i32 {
                cp = np_0.offset(1 as ::core::ffi::c_int as isize);
            }
        }
    } else {
        while *cp as ::core::ffi::c_int >= '0' as i32 && *cp as ::core::ffi::c_int <= '9' as i32 {
            cp = cp.offset(1);
        }
    }
    if *cp as ::core::ffi::c_int == '.' as i32 {
        cp = cp.offset(1);
        if *cp as ::core::ffi::c_int == '*' as i32 {
            if *cp as ::core::ffi::c_int >= '0' as i32 && *cp as ::core::ffi::c_int <= '9' as i32 {
                let mut np_1: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                np_1 = cp;
                while *np_1 as ::core::ffi::c_int >= '0' as i32
                    && *np_1 as ::core::ffi::c_int <= '9' as i32
                {
                    np_1 = np_1.offset(1);
                }
                if *np_1 as ::core::ffi::c_int == '$' as i32 {
                    cp = np_1.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        } else {
            while *cp as ::core::ffi::c_int >= '0' as i32 && *cp as ::core::ffi::c_int <= '9' as i32
            {
                cp = cp.offset(1);
            }
        }
    }
    while *cp as ::core::ffi::c_int == 'h' as i32
        || *cp as ::core::ffi::c_int == 'L' as i32
        || *cp as ::core::ffi::c_int == 'l' as i32
        || *cp as ::core::ffi::c_int == 'j' as i32
        || *cp as ::core::ffi::c_int == 'z' as i32
        || *cp as ::core::ffi::c_int == 'Z' as i32
        || *cp as ::core::ffi::c_int == 't' as i32
    {
        cp = cp.offset(1);
    }
    cp = cp.offset(1);
    *after = cp;
    return start;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_vprintf_escaped(
    mut format: *const gchar,
    mut args: ::core::ffi::VaList,
) -> *mut gchar {
    let mut format1: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut format2: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut result: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut output1: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut output2: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut op1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut op2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut args2: ::core::ffi::VaList;
    format1 = g_string_new(::core::ptr::null::<gchar>());
    format2 = g_string_new(::core::ptr::null::<gchar>());
    p = format as *const ::core::ffi::c_char;
    while FALSE == 0 {
        let mut after: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut conv: *const ::core::ffi::c_char = safe_c2rust_find_conversion(p, &raw mut after);
        if conv.is_null() {
            break;
        }
        safe_c2rust_g_string_append_len_inline(format1, conv, after.offset_from(conv) as gssize);
        safe_c2rust_g_string_append_c_inline(format1, 'X' as i32 as gchar);
        safe_c2rust_g_string_append_len_inline(format2, conv, after.offset_from(conv) as gssize);
        safe_c2rust_g_string_append_c_inline(format2, 'Y' as i32 as gchar);
        p = after;
    }
    args2 = args.clone();
    output1 = g_strdup_vprintf((*format1).str_0, args.clone());
    if !output1.is_null() {
        output2 = g_strdup_vprintf((*format2).str_0, args2.clone());
        if !output2.is_null() {
            result = g_string_new(::core::ptr::null::<gchar>());
            op1 = output1;
            op2 = output2;
            p = format as *const ::core::ffi::c_char;
            while FALSE == 0 {
                let mut after_0: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut output_start: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut conv_0: *const ::core::ffi::c_char =
                    safe_c2rust_find_conversion(p, &raw mut after_0);
                let mut escaped: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if conv_0.is_null() {
                    safe_c2rust_g_string_append_len_inline(
                        result,
                        p,
                        after_0.offset_from(p) as gssize,
                    );
                    break;
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        result,
                        p,
                        conv_0.offset_from(p) as gssize,
                    );
                    output_start = op1;
                    while *op1 as ::core::ffi::c_int == *op2 as ::core::ffi::c_int {
                        op1 = op1.offset(1);
                        op2 = op2.offset(1);
                    }
                    escaped = safe_c2rust_g_markup_escape_text(
                        output_start as *const gchar,
                        op1.offset_from(output_start) as gssize,
                    ) as *mut ::core::ffi::c_char;
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char = escaped;
                            safe_c2rust_g_string_append_len_inline(
                                result,
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
                            escaped,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                    g_free(escaped as gpointer);
                    p = after_0;
                    op1 = op1.offset(1);
                    op2 = op2.offset(1);
                }
            }
        }
    }
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                format1,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(format1);
        };
    } else {
        g_string_free(
            format1,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                format2,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(format2);
        };
    } else {
        g_string_free(
            format2,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
    g_free(output1 as gpointer);
    g_free(output2 as gpointer);
    if !result.is_null() {
        return if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(result, 0 as gboolean)
            } else {
                g_string_free_and_steal(result)
            }
        } else {
            g_string_free(result, 0 as gboolean)
        };
    } else {
        return ::core::ptr::null_mut::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_printf_escaped(
    mut format: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    result = safe_c2rust_g_markup_vprintf_escaped(format, args_0.clone())
        as *mut ::core::ffi::c_char;
    return result as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_markup_parse_boolean(
    mut string: *const ::core::ffi::c_char,
    mut value: *mut gboolean,
) -> gboolean {
    let falses: [*const ::core::ffi::c_char; 5] = [
        b"false\0" as *const u8 as *const ::core::ffi::c_char,
        b"f\0" as *const u8 as *const ::core::ffi::c_char,
        b"no\0" as *const u8 as *const ::core::ffi::c_char,
        b"n\0" as *const u8 as *const ::core::ffi::c_char,
        b"0\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let trues: [*const ::core::ffi::c_char; 5] = [
        b"true\0" as *const u8 as *const ::core::ffi::c_char,
        b"t\0" as *const u8 as *const ::core::ffi::c_char,
        b"yes\0" as *const u8 as *const ::core::ffi::c_char,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        b"1\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: gsize = 0;
    i = 0 as gsize;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if g_ascii_strcasecmp(string as *const gchar, falses[i as usize]) == 0 as ::core::ffi::c_int
        {
            if !value.is_null() {
                *value = FALSE as gboolean;
            }
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as gsize;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if g_ascii_strcasecmp(string as *const gchar, trues[i as usize]) == 0 as ::core::ffi::c_int
        {
            if !value.is_null() {
                *value = TRUE as gboolean;
            }
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_markup_collect_attributes(
    mut element_name: *const gchar,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut error: *mut *mut GError,
    mut first_type: GMarkupCollectType,
    mut first_attr: *const gchar,
    mut args: ...
) -> gboolean {
    let mut current_block: u64;
    let mut type_0: GMarkupCollectType = G_MARKUP_COLLECT_INVALID;
    let mut attr: *const gchar = ::core::ptr::null::<gchar>();
    let mut collected: guint64 = 0;
    let mut written: ::core::ffi::c_int = 0;
    let mut ap: ::core::ffi::VaList;
    let mut i: ::core::ffi::c_int = 0;
    type_0 = first_type;
    attr = first_attr;
    collected = 0 as guint64;
    written = 0 as ::core::ffi::c_int;
    ap = args.clone();
    loop {
        if !(type_0 as ::core::ffi::c_uint
            != G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            current_block = 12381812505308290051;
            break;
        }
        let mut mandatory: gboolean = 0;
        let mut value: *const gchar = ::core::ptr::null::<gchar>();
        mandatory = (type_0 as ::core::ffi::c_uint
            & G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0) as ::core::ffi::c_int as gboolean;
        type_0 = ::core::mem::transmute::<::core::ffi::c_uint, GMarkupCollectType>(
            type_0 as ::core::ffi::c_uint
                & (G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
        );
        if type_0 as ::core::ffi::c_uint
            == G_MARKUP_COLLECT_TRISTATE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            mandatory = FALSE as gboolean;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*attribute_names.offset(i as isize)).is_null() {
            if i >= 40 as ::core::ffi::c_int
                || collected as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << i == 0
            {
                if strcmp(
                    *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
                    attr as *const ::core::ffi::c_char,
                ) == 0
                {
                    break;
                }
            }
            i += 1;
        }
        if i < 40 as ::core::ffi::c_int {
            collected |= (1 as ::core::ffi::c_ulong) << i;
        }
        value = *attribute_values.offset(i as isize);
        if value.is_null() && mandatory != 0 {
            g_set_error(
                error,
                safe_c2rust_g_markup_error_quark(),
                G_MARKUP_ERROR_MISSING_ATTRIBUTE as ::core::ffi::c_int as gint,
                b"element '%s' requires attribute '%s'\0" as *const u8 as *const gchar,
                element_name,
                attr,
            );
            current_block = 8163710838839710173;
            break;
        } else {
            match type_0 as ::core::ffi::c_uint {
                1 => {
                    let mut str_ptr: *mut *const ::core::ffi::c_char =
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
                    str_ptr = ap.arg::<*mut *const ::core::ffi::c_char>();
                    if !str_ptr.is_null() {
                        *str_ptr = value as *const ::core::ffi::c_char;
                    }
                }
                2 => {
                    let mut str_ptr_0: *mut *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                    str_ptr_0 = ap.arg::<*mut *mut ::core::ffi::c_char>();
                    if !str_ptr_0.is_null() {
                        *str_ptr_0 =
                            safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char);
                    }
                }
                3 | 4 => {
                    if value.is_null() {
                        let mut bool_ptr: *mut gboolean = ::core::ptr::null_mut::<gboolean>();
                        bool_ptr = ap.arg::<*mut gboolean>();
                        if !bool_ptr.is_null() {
                            if type_0 as ::core::ffi::c_uint
                                == G_MARKUP_COLLECT_TRISTATE as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                *bool_ptr = -(1 as ::core::ffi::c_int) as gboolean;
                            } else {
                                *bool_ptr = FALSE as gboolean;
                            }
                        }
                    } else if safe_c2rust_g_markup_parse_boolean(
                        value as *const ::core::ffi::c_char,
                        ap.arg::<*mut gboolean>(),
                    ) == 0
                    {
                        g_set_error(
                            error,
                            safe_c2rust_g_markup_error_quark(),
                            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                            b"element '%s', attribute '%s', value '%s' cannot be parsed as a boolean value\0"
                                as *const u8 as *const gchar,
                            element_name,
                            attr,
                            value,
                        );
                        current_block = 8163710838839710173;
                        break;
                    }
                }
                _ => {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gmarkup.c\0" as *const u8 as *const ::core::ffi::c_char,
                        2832 as ::core::ffi::c_int,
                        G_STRFUNC,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
            written += 1;
            type_0 = ap.arg::<GMarkupCollectType>();
            if type_0 as ::core::ffi::c_uint
                != G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                attr = ap.arg::<*const ::core::ffi::c_char>() as *const gchar;
            }
        }
    }
    match current_block {
        12381812505308290051 => {
            i = 0 as ::core::ffi::c_int;
            loop {
                if (*attribute_names.offset(i as isize)).is_null() {
                    current_block = 17233182392562552756;
                    break;
                }
                if collected as ::core::ffi::c_ulong & (1 as ::core::ffi::c_ulong) << i
                    == 0 as ::core::ffi::c_ulong
                {
                    let mut j: ::core::ffi::c_int = 0;
                    j = 0 as ::core::ffi::c_int;
                    while j < i {
                        if strcmp(
                            *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
                            *attribute_names.offset(j as isize) as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        j += 1;
                    }
                    if i == j {
                        g_set_error(
                            error,
                            safe_c2rust_g_markup_error_quark(),
                            G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE as ::core::ffi::c_int as gint,
                            b"attribute '%s' invalid for element '%s'\0" as *const u8
                                as *const gchar,
                            *attribute_names.offset(i as isize),
                            element_name,
                        );
                    } else {
                        g_set_error(
                            error,
                            safe_c2rust_g_markup_error_quark(),
                            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                            b"attribute '%s' given multiple times for element '%s'\0" as *const u8
                                as *const gchar,
                            *attribute_names.offset(i as isize),
                            element_name,
                        );
                    }
                    current_block = 8163710838839710173;
                    break;
                } else {
                    i += 1;
                }
            }
            match current_block {
                8163710838839710173 => {}
                _ => return TRUE,
            }
        }
        _ => {}
    }
    type_0 = first_type;
    ap = args.clone();
    while type_0 as ::core::ffi::c_uint
        != G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ptr = ap.arg::<gpointer>();
        if !ptr.is_null() {
            match type_0 as ::core::ffi::c_uint
                & (G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
            {
                2 => {
                    if written != 0 {
                        g_free(*(ptr as *mut *mut ::core::ffi::c_char) as gpointer);
                    }
                    let ref mut fresh13 = *(ptr as *mut *mut ::core::ffi::c_char);
                    *fresh13 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                1 => {
                    let ref mut fresh14 = *(ptr as *mut *mut ::core::ffi::c_char);
                    *fresh14 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                3 => {
                    *(ptr as *mut gboolean) = FALSE as gboolean;
                }
                4 => {
                    *(ptr as *mut gboolean) = -(1 as ::core::ffi::c_int) as gboolean;
                }
                _ => {}
            }
        }
        type_0 = ap.arg::<GMarkupCollectType>();
        if type_0 as ::core::ffi::c_uint
            != G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            attr = ap.arg::<*const ::core::ffi::c_char>() as *const gchar;
        }
    }
    return FALSE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
