extern "C" {
    pub type _GHashTable;
    pub type _GMarkupParseContext;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_foreach(array: *mut GPtrArray, func: GFunc, user_data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_markup_parse_context_get_element_stack(context: *mut GMarkupParseContext)
        -> *const GSList;
    fn g_markup_parse_context_get_position(
        context: *mut GMarkupParseContext,
        line_number: *mut gint,
        char_number: *mut gint,
    );
    fn g_markup_printf_escaped(format: *const ::core::ffi::c_char, ...) -> *mut gchar;
    fn g_markup_collect_attributes(
        element_name: *const gchar,
        attribute_names: *mut *const gchar,
        attribute_values: *mut *const gchar,
        error: *mut *mut GError,
        first_type: GMarkupCollectType,
        first_attr: *const gchar,
        ...
    ) -> gboolean;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_MARKUP_ERROR_MISSING_ATTRIBUTE: C2RustUnnamed = 6;
pub const G_MARKUP_ERROR_INVALID_CONTENT: C2RustUnnamed = 5;
pub const G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE: C2RustUnnamed = 4;
pub const G_MARKUP_ERROR_UNKNOWN_ELEMENT: C2RustUnnamed = 3;
pub const G_MARKUP_ERROR_PARSE: C2RustUnnamed = 2;
pub const G_MARKUP_ERROR_EMPTY: C2RustUnnamed = 1;
pub const G_MARKUP_ERROR_BAD_UTF8: C2RustUnnamed = 0;
pub type GMarkupParseFlags = ::core::ffi::c_uint;
pub const G_MARKUP_IGNORE_QUALIFIED: GMarkupParseFlags = 8;
pub const G_MARKUP_PREFIX_ERROR_POSITION: GMarkupParseFlags = 4;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: GMarkupParseFlags = 2;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: GMarkupParseFlags = 1;
pub const G_MARKUP_DEFAULT_FLAGS: GMarkupParseFlags = 0;
pub type GMarkupParseContext = _GMarkupParseContext;
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
pub type GMarkupParser = _GMarkupParser;
pub type GMarkupCollectType = ::core::ffi::c_uint;
pub const G_MARKUP_COLLECT_OPTIONAL: GMarkupCollectType = 65536;
pub const G_MARKUP_COLLECT_TRISTATE: GMarkupCollectType = 4;
pub const G_MARKUP_COLLECT_BOOLEAN: GMarkupCollectType = 3;
pub const G_MARKUP_COLLECT_STRDUP: GMarkupCollectType = 2;
pub const G_MARKUP_COLLECT_STRING: GMarkupCollectType = 1;
pub const G_MARKUP_COLLECT_INVALID: GMarkupCollectType = 0;
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
pub type GType = gsize;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub methods: *mut *mut GDBusMethodInfo,
    pub signals: *mut *mut GDBusSignalInfo,
    pub properties: *mut *mut GDBusPropertyInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusNodeInfo {
    pub ref_count: gint,
    pub path: *mut gchar,
    pub interfaces: *mut *mut GDBusInterfaceInfo,
    pub nodes: *mut *mut GDBusNodeInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusNodeInfo = _GDBusNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfoCacheEntry {
    pub use_count: gint,
    pub method_name_to_data: *mut GHashTable,
    pub signal_name_to_data: *mut GHashTable,
    pub property_name_to_data: *mut GHashTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseData {
    pub args: *mut GPtrArray,
    pub out_args: *mut GPtrArray,
    pub methods: *mut GPtrArray,
    pub signals: *mut GPtrArray,
    pub properties: *mut GPtrArray,
    pub interfaces: *mut GPtrArray,
    pub nodes: *mut GPtrArray,
    pub annotations: *mut GPtrArray,
    pub annotations_stack: *mut GSList,
    pub interfaces_stack: *mut GSList,
    pub nodes_stack: *mut GSList,
    pub last_arg_was_in: gboolean,
    pub num_args: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusNodeInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDBusNodeInfo) -> *mut GDBusNodeInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusNodeInfo) -> *mut GDBusNodeInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusMethodInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDBusMethodInfo) -> *mut GDBusMethodInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusMethodInfo) -> *mut GDBusMethodInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusSignalInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDBusSignalInfo) -> *mut GDBusSignalInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusSignalInfo) -> *mut GDBusSignalInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusPropertyInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GDBusPropertyInfo) -> *mut GDBusPropertyInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusPropertyInfo) -> *mut GDBusPropertyInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> *mut GDBusArgInfo>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GDBusArgInfo) -> *mut GDBusArgInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> *mut GDBusAnnotationInfo>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GDBusAnnotationInfo) -> *mut GDBusAnnotationInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
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
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_node_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_1, C2RustUnnamed_0) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_1, C2RustUnnamed_0) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusNodeInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_1 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_node_info_ref
                    as unsafe extern "C" fn(*mut GDBusNodeInfo) -> *mut GDBusNodeInfo,
            ),
        },
        C2RustUnnamed_0 {
            do_free_type: Some(
                safe_c2rust_g_dbus_node_info_unref
                    as unsafe extern "C" fn(*mut GDBusNodeInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_node_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_3, C2RustUnnamed_2) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_3, C2RustUnnamed_2) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusInterfaceInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_3 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_interface_info_ref
                    as unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo,
            ),
        },
        C2RustUnnamed_2 {
            do_free_type: Some(
                safe_c2rust_g_dbus_interface_info_unref
                    as unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_interface_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_method_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_5, C2RustUnnamed_4) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_5, C2RustUnnamed_4) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusMethodInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_5 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_method_info_ref
                    as unsafe extern "C" fn(*mut GDBusMethodInfo) -> *mut GDBusMethodInfo,
            ),
        },
        C2RustUnnamed_4 {
            do_free_type: Some(
                safe_c2rust_g_dbus_method_info_unref
                    as unsafe extern "C" fn(*mut GDBusMethodInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_method_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_7, C2RustUnnamed_6) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_7, C2RustUnnamed_6) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusSignalInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_7 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_signal_info_ref
                    as unsafe extern "C" fn(*mut GDBusSignalInfo) -> *mut GDBusSignalInfo,
            ),
        },
        C2RustUnnamed_6 {
            do_free_type: Some(
                safe_c2rust_g_dbus_signal_info_unref
                    as unsafe extern "C" fn(*mut GDBusSignalInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_signal_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_property_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_9, C2RustUnnamed_8) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_9, C2RustUnnamed_8) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusPropertyInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_9 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_property_info_ref
                    as unsafe extern "C" fn(*mut GDBusPropertyInfo) -> *mut GDBusPropertyInfo,
            ),
        },
        C2RustUnnamed_8 {
            do_free_type: Some(
                safe_c2rust_g_dbus_property_info_unref
                    as unsafe extern "C" fn(*mut GDBusPropertyInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_property_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_property_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_11, C2RustUnnamed_10) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_11, C2RustUnnamed_10) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusArgInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_11 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_arg_info_ref
                    as unsafe extern "C" fn(*mut GDBusArgInfo) -> *mut GDBusArgInfo,
            ),
        },
        C2RustUnnamed_10 {
            do_free_type: Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_arg_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_13, C2RustUnnamed_12) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_13, C2RustUnnamed_12) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDBusAnnotationInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_13 {
            do_copy_type: Some(
                safe_c2rust_g_dbus_annotation_info_ref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> *mut GDBusAnnotationInfo,
            ),
        },
        C2RustUnnamed_12 {
            do_free_type: Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_annotation_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_ref(
    mut info: *mut GDBusNodeInfo,
) -> *mut GDBusNodeInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_ref(
    mut info: *mut GDBusInterfaceInfo,
) -> *mut GDBusInterfaceInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_info_ref(
    mut info: *mut GDBusMethodInfo,
) -> *mut GDBusMethodInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_ref(
    mut info: *mut GDBusSignalInfo,
) -> *mut GDBusSignalInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_property_info_ref(
    mut info: *mut GDBusPropertyInfo,
) -> *mut GDBusPropertyInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_ref(
    mut info: *mut GDBusArgInfo,
) -> *mut GDBusArgInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_ref(
    mut info: *mut GDBusAnnotationInfo,
) -> *mut GDBusAnnotationInfo {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return info;
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*info).ref_count;
        (*info).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*info).ref_count, 1 as ::core::ffi::c_int);
    return info;
}
unsafe extern "C" fn safe_c2rust_free_null_terminated_array(
    mut array: gpointer,
    mut unref_func: GDestroyNotify,
) {
    let mut n: guint = 0;
    let mut p: *mut gpointer = array as *mut gpointer;
    if p.is_null() {
        return;
    }
    n = 0 as guint;
    while !(*p.offset(n as isize)).is_null() {
        unref_func.expect("non-null function pointer")(*p.offset(n as isize));
        n = n.wrapping_add(1);
    }
    g_free(p as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_unref(
    mut info: *mut GDBusAnnotationInfo,
) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).key as gpointer);
        g_free((*info).value as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_unref(mut info: *mut GDBusArgInfo) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).name as gpointer);
        g_free((*info).signature as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_method_info_unref(mut info: *mut GDBusMethodInfo) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).name as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).in_args as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).out_args as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_unref(mut info: *mut GDBusSignalInfo) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).name as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).args as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_property_info_unref(mut info: *mut GDBusPropertyInfo) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).name as gpointer);
        g_free((*info).signature as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_unref(
    mut info: *mut GDBusInterfaceInfo,
) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).name as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).methods as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusMethodInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_method_info_unref
                    as unsafe extern "C" fn(*mut GDBusMethodInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).signals as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusSignalInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_signal_info_unref
                    as unsafe extern "C" fn(*mut GDBusSignalInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).properties as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusPropertyInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_property_info_unref
                    as unsafe extern "C" fn(*mut GDBusPropertyInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_unref(mut info: *mut GDBusNodeInfo) {
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(&raw mut (*info).ref_count);
        gaig_temp
    }) == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*info).ref_count;
            (*info).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*info).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free((*info).path as gpointer);
        safe_c2rust_free_null_terminated_array(
            (*info).interfaces as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_interface_info_unref
                    as unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).nodes as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusNodeInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_node_info_unref
                    as unsafe extern "C" fn(*mut GDBusNodeInfo) -> (),
            )),
        );
        safe_c2rust_free_null_terminated_array(
            (*info).annotations as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
        );
        g_free(info as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusAnnotationInfo,
    mut key: *const gchar,
    mut value: *const gchar,
    mut embedded_annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !key.is_null() {
        (*info).key = safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !value.is_null() {
        (*info).value =
            safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !embedded_annotations.is_null() {
        (*info).annotations = embedded_annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusArgInfo,
    mut name: *const gchar,
    mut signature: *const gchar,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !name.is_null() {
        (*info).name =
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !signature.is_null() {
        (*info).signature =
            safe_c2rust_g_strdup_inline(signature as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusMethodInfo,
    mut name: *const gchar,
    mut in_args: *mut *mut GDBusArgInfo,
    mut out_args: *mut *mut GDBusArgInfo,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !name.is_null() {
        (*info).name =
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !in_args.is_null() {
        (*info).in_args = in_args;
    }
    if !out_args.is_null() {
        (*info).out_args = out_args;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusSignalInfo,
    mut name: *const gchar,
    mut args: *mut *mut GDBusArgInfo,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !name.is_null() {
        (*info).name =
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !args.is_null() {
        (*info).args = args;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_property_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusPropertyInfo,
    mut name: *const gchar,
    mut signature: *const gchar,
    mut flags: GDBusPropertyInfoFlags,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !name.is_null() {
        (*info).name =
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    }
    if flags as ::core::ffi::c_uint
        != G_DBUS_PROPERTY_INFO_FLAGS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*info).flags = flags;
    }
    if !signature.is_null() {
        (*info).signature =
            safe_c2rust_g_strdup_inline(signature as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
    mut methods: *mut *mut GDBusMethodInfo,
    mut signals: *mut *mut GDBusSignalInfo,
    mut properties: *mut *mut GDBusPropertyInfo,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !name.is_null() {
        (*info).name =
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !methods.is_null() {
        (*info).methods = methods;
    }
    if !signals.is_null() {
        (*info).signals = signals;
    }
    if !properties.is_null() {
        (*info).properties = properties;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_node_info_set(
    mut data: *mut ParseData,
    mut info: *mut GDBusNodeInfo,
    mut path: *const gchar,
    mut interfaces: *mut *mut GDBusInterfaceInfo,
    mut nodes: *mut *mut GDBusNodeInfo,
    mut annotations: *mut *mut GDBusAnnotationInfo,
) {
    (*info).ref_count = 1 as ::core::ffi::c_int as gint;
    if !path.is_null() {
        (*info).path =
            safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    }
    if !interfaces.is_null() {
        (*info).interfaces = interfaces;
    }
    if !nodes.is_null() {
        (*info).nodes = nodes;
    }
    if !annotations.is_null() {
        (*info).annotations = annotations;
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_generate_xml(
    mut info: *mut GDBusAnnotationInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n: guint = 0;
    tmp = g_markup_printf_escaped(
        b"%*s<annotation name=\"%s\" value=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).key,
        (*info).value,
    );
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = tmp;
            safe_c2rust_g_string_append_len_inline(
                string_builder,
                __val,
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
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
            string_builder,
            tmp,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_free(tmp as gpointer);
    if (*info).annotations.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</annotation>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_arg_info_generate_xml(
    mut info: *mut GDBusArgInfo,
    mut indent: guint,
    mut extra_attributes: *const gchar,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    g_string_append_printf(
        string_builder,
        b"%*s<arg type=\"%s\"\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).signature,
    );
    if !(*info).name.is_null() {
        g_string_append_printf(
            string_builder,
            b" name=\"%s\"\0" as *const u8 as *const gchar,
            (*info).name,
        );
    }
    if !extra_attributes.is_null() {
        g_string_append_printf(
            string_builder,
            b" %s\0" as *const u8 as *const gchar,
            extra_attributes,
        );
    }
    if (*info).annotations.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</arg>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_method_info_generate_xml(
    mut info: *mut GDBusMethodInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    g_string_append_printf(
        string_builder,
        b"%*s<method name=\"%s\"\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).name,
    );
    if (*info).annotations.is_null() && (*info).in_args.is_null() && (*info).out_args.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).in_args.is_null() && !(*(*info).in_args.offset(n as isize)).is_null() {
            safe_c2rust_g_dbus_arg_info_generate_xml(
                *(*info).in_args.offset(n as isize),
                indent.wrapping_add(2 as guint),
                b"direction=\"in\"\0" as *const u8 as *const gchar,
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).out_args.is_null() && !(*(*info).out_args.offset(n as isize)).is_null() {
            safe_c2rust_g_dbus_arg_info_generate_xml(
                *(*info).out_args.offset(n as isize),
                indent.wrapping_add(2 as guint),
                b"direction=\"out\"\0" as *const u8 as *const gchar,
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</method>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_signal_info_generate_xml(
    mut info: *mut GDBusSignalInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    g_string_append_printf(
        string_builder,
        b"%*s<signal name=\"%s\"\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).name,
    );
    if (*info).annotations.is_null() && (*info).args.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).args.is_null() && !(*(*info).args.offset(n as isize)).is_null() {
            safe_c2rust_g_dbus_arg_info_generate_xml(
                *(*info).args.offset(n as isize),
                indent.wrapping_add(2 as guint),
                ::core::ptr::null::<gchar>(),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</signal>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_property_info_generate_xml(
    mut info: *mut GDBusPropertyInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    let mut access_string: *const gchar = ::core::ptr::null::<gchar>();
    if (*info).flags as ::core::ffi::c_uint
        & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*info).flags as ::core::ffi::c_uint
            & G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        access_string = b"readwrite\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    } else if (*info).flags as ::core::ffi::c_uint
        & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        access_string = b"read\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    } else if (*info).flags as ::core::ffi::c_uint
        & G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        access_string = b"write\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusintrospection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            736 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_string_append_printf(
        string_builder,
        b"%*s<property type=\"%s\" name=\"%s\" access=\"%s\"\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).signature,
        (*info).name,
        access_string,
    );
    if (*info).annotations.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</property>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_generate_xml(
    mut info: *mut GDBusInterfaceInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    g_string_append_printf(
        string_builder,
        b"%*s<interface name=\"%s\">\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*info).name,
    );
    n = 0 as guint;
    while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null() {
        safe_c2rust_g_dbus_annotation_info_generate_xml(
            *(*info).annotations.offset(n as isize),
            indent.wrapping_add(2 as guint),
            string_builder,
        );
        n = n.wrapping_add(1);
    }
    n = 0 as guint;
    while !(*info).methods.is_null() && !(*(*info).methods.offset(n as isize)).is_null() {
        safe_c2rust_g_dbus_method_info_generate_xml(
            *(*info).methods.offset(n as isize),
            indent.wrapping_add(2 as guint),
            string_builder,
        );
        n = n.wrapping_add(1);
    }
    n = 0 as guint;
    while !(*info).signals.is_null() && !(*(*info).signals.offset(n as isize)).is_null() {
        safe_c2rust_g_dbus_signal_info_generate_xml(
            *(*info).signals.offset(n as isize),
            indent.wrapping_add(2 as guint),
            string_builder,
        );
        n = n.wrapping_add(1);
    }
    n = 0 as guint;
    while !(*info).properties.is_null() && !(*(*info).properties.offset(n as isize)).is_null() {
        safe_c2rust_g_dbus_property_info_generate_xml(
            *(*info).properties.offset(n as isize),
            indent.wrapping_add(2 as guint),
            string_builder,
        );
        n = n.wrapping_add(1);
    }
    g_string_append_printf(
        string_builder,
        b"%*s</interface>\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_generate_xml(
    mut info: *mut GDBusNodeInfo,
    mut indent: guint,
    mut string_builder: *mut GString,
) {
    let mut n: guint = 0;
    g_string_append_printf(
        string_builder,
        b"%*s<node\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(*info).path.is_null() {
        g_string_append_printf(
            string_builder,
            b" name=\"%s\"\0" as *const u8 as *const gchar,
            (*info).path,
        );
    }
    if (*info).interfaces.is_null() && (*info).nodes.is_null() && (*info).annotations.is_null() {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/>\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                string_builder,
                b"/>\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b">\n\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string_builder,
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
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                string_builder,
                b">\n\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        n = 0 as guint;
        while !(*info).annotations.is_null() && !(*(*info).annotations.offset(n as isize)).is_null()
        {
            safe_c2rust_g_dbus_annotation_info_generate_xml(
                *(*info).annotations.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).interfaces.is_null() && !(*(*info).interfaces.offset(n as isize)).is_null() {
            safe_c2rust_g_dbus_interface_info_generate_xml(
                *(*info).interfaces.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).nodes.is_null() && !(*(*info).nodes.offset(n as isize)).is_null() {
            safe_c2rust_g_dbus_node_info_generate_xml(
                *(*info).nodes.offset(n as isize),
                indent.wrapping_add(2 as guint),
                string_builder,
            );
            n = n.wrapping_add(1);
        }
        g_string_append_printf(
            string_builder,
            b"%*s</node>\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_annotations(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusAnnotationInfo {
    let mut ret: *mut *mut GDBusAnnotationInfo =
        ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).annotations).len;
    }
    if (*data).annotations.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>();
    } else {
        g_ptr_array_add((*data).annotations, NULL_0);
        ret = g_ptr_array_free((*data).annotations, FALSE) as *mut *mut GDBusAnnotationInfo;
    }
    (*data).annotations = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_args(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusArgInfo {
    let mut ret: *mut *mut GDBusArgInfo = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).args).len;
    }
    if (*data).args.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
    } else {
        g_ptr_array_add((*data).args, NULL_0);
        ret = g_ptr_array_free((*data).args, FALSE) as *mut *mut GDBusArgInfo;
    }
    (*data).args = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_out_args(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusArgInfo {
    let mut ret: *mut *mut GDBusArgInfo = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).out_args).len;
    }
    if (*data).out_args.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
    } else {
        g_ptr_array_add((*data).out_args, NULL_0);
        ret = g_ptr_array_free((*data).out_args, FALSE) as *mut *mut GDBusArgInfo;
    }
    (*data).out_args = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_methods(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusMethodInfo {
    let mut ret: *mut *mut GDBusMethodInfo = ::core::ptr::null_mut::<*mut GDBusMethodInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).methods).len;
    }
    if (*data).methods.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusMethodInfo>();
    } else {
        g_ptr_array_add((*data).methods, NULL_0);
        ret = g_ptr_array_free((*data).methods, FALSE) as *mut *mut GDBusMethodInfo;
    }
    (*data).methods = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_signals(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusSignalInfo {
    let mut ret: *mut *mut GDBusSignalInfo = ::core::ptr::null_mut::<*mut GDBusSignalInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).signals).len;
    }
    if (*data).signals.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusSignalInfo>();
    } else {
        g_ptr_array_add((*data).signals, NULL_0);
        ret = g_ptr_array_free((*data).signals, FALSE) as *mut *mut GDBusSignalInfo;
    }
    (*data).signals = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_properties(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusPropertyInfo {
    let mut ret: *mut *mut GDBusPropertyInfo = ::core::ptr::null_mut::<*mut GDBusPropertyInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).properties).len;
    }
    if (*data).properties.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusPropertyInfo>();
    } else {
        g_ptr_array_add((*data).properties, NULL_0);
        ret = g_ptr_array_free((*data).properties, FALSE) as *mut *mut GDBusPropertyInfo;
    }
    (*data).properties = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_interfaces(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusInterfaceInfo {
    let mut ret: *mut *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).interfaces).len;
    }
    if (*data).interfaces.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
    } else {
        g_ptr_array_add((*data).interfaces, NULL_0);
        ret = g_ptr_array_free((*data).interfaces, FALSE) as *mut *mut GDBusInterfaceInfo;
    }
    (*data).interfaces = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_steal_nodes(
    mut data: *mut ParseData,
    mut out_num_elements: *mut guint,
) -> *mut *mut GDBusNodeInfo {
    let mut ret: *mut *mut GDBusNodeInfo = ::core::ptr::null_mut::<*mut GDBusNodeInfo>();
    if !out_num_elements.is_null() {
        *out_num_elements = (*(*data).nodes).len;
    }
    if (*data).nodes.is_null() {
        ret = ::core::ptr::null_mut::<*mut GDBusNodeInfo>();
    } else {
        g_ptr_array_add((*data).nodes, NULL_0);
        ret = g_ptr_array_free((*data).nodes, FALSE) as *mut *mut GDBusNodeInfo;
    }
    (*data).nodes = g_ptr_array_new();
    return ret;
}
unsafe extern "C" fn safe_c2rust_parse_data_free_annotations(mut data: *mut ParseData) {
    if (*data).annotations.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).annotations,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).annotations, TRUE);
    (*data).annotations = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_args(mut data: *mut ParseData) {
    if (*data).args.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).args,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).args, TRUE);
    (*data).args = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_out_args(mut data: *mut ParseData) {
    if (*data).out_args.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).out_args,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusArgInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_arg_info_unref as unsafe extern "C" fn(*mut GDBusArgInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).out_args, TRUE);
    (*data).out_args = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_methods(mut data: *mut ParseData) {
    if (*data).methods.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).methods,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusMethodInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_method_info_unref
                    as unsafe extern "C" fn(*mut GDBusMethodInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).methods, TRUE);
    (*data).methods = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_signals(mut data: *mut ParseData) {
    if (*data).signals.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).signals,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusSignalInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_signal_info_unref
                    as unsafe extern "C" fn(*mut GDBusSignalInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).signals, TRUE);
    (*data).signals = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_properties(mut data: *mut ParseData) {
    if (*data).properties.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).properties,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusPropertyInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_property_info_unref
                    as unsafe extern "C" fn(*mut GDBusPropertyInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).properties, TRUE);
    (*data).properties = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_interfaces(mut data: *mut ParseData) {
    if (*data).interfaces.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).interfaces,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_interface_info_unref
                    as unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).interfaces, TRUE);
    (*data).interfaces = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_free_nodes(mut data: *mut ParseData) {
    if (*data).nodes.is_null() {
        return;
    }
    g_ptr_array_foreach(
        (*data).nodes,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusNodeInfo) -> ()>, GFunc>(
            Some(
                safe_c2rust_g_dbus_node_info_unref
                    as unsafe extern "C" fn(*mut GDBusNodeInfo) -> (),
            ),
        ),
        NULL_0,
    );
    g_ptr_array_free((*data).nodes, TRUE);
    (*data).nodes = ::core::ptr::null_mut::<GPtrArray>();
}
unsafe extern "C" fn safe_c2rust_parse_data_get_annotation(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusAnnotationInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).annotations,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusAnnotationInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusAnnotationInfo as gpointer,
        );
    }
    return *(*(*data).annotations)
        .pdata
        .offset((*(*data).annotations).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusAnnotationInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_arg(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusArgInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).args,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusArgInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusArgInfo as gpointer,
        );
    }
    return *(*(*data).args)
        .pdata
        .offset((*(*data).args).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusArgInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_out_arg(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusArgInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).out_args,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusArgInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusArgInfo as gpointer,
        );
    }
    return *(*(*data).out_args)
        .pdata
        .offset((*(*data).out_args).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusArgInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_method(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusMethodInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).methods,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusMethodInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusMethodInfo as gpointer,
        );
    }
    return *(*(*data).methods)
        .pdata
        .offset((*(*data).methods).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusMethodInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_signal(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusSignalInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).signals,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusSignalInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusSignalInfo as gpointer,
        );
    }
    return *(*(*data).signals)
        .pdata
        .offset((*(*data).signals).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusSignalInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_property(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusPropertyInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).properties,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusPropertyInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusPropertyInfo as gpointer,
        );
    }
    return *(*(*data).properties)
        .pdata
        .offset((*(*data).properties).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusPropertyInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_interface(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusInterfaceInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).interfaces,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusInterfaceInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusInterfaceInfo as gpointer,
        );
    }
    return *(*(*data).interfaces)
        .pdata
        .offset((*(*data).interfaces).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusInterfaceInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_get_node(
    mut data: *mut ParseData,
    mut create_new: gboolean,
) -> *mut GDBusNodeInfo {
    if create_new != 0 {
        g_ptr_array_add(
            (*data).nodes,
            ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GDBusNodeInfo>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GDBusNodeInfo as gpointer,
        );
    }
    return *(*(*data).nodes)
        .pdata
        .offset((*(*data).nodes).len.wrapping_sub(1 as guint) as isize)
        as *mut GDBusNodeInfo;
}
unsafe extern "C" fn safe_c2rust_parse_data_new() -> *mut ParseData {
    let mut data: *mut ParseData = ::core::ptr::null_mut::<ParseData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ParseData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ParseData;
    safe_c2rust_parse_data_steal_annotations(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_args(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_out_args(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_methods(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_signals(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_properties(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_interfaces(data, ::core::ptr::null_mut::<guint>());
    safe_c2rust_parse_data_steal_nodes(data, ::core::ptr::null_mut::<guint>());
    return data;
}
unsafe extern "C" fn safe_c2rust_parse_data_free(mut data: *mut ParseData) {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = (*data).annotations_stack;
    while !l.is_null() {
        let mut annotations: *mut GPtrArray = (*l).data as *mut GPtrArray;
        g_ptr_array_foreach(
            annotations,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> ()>,
                GFunc,
            >(Some(
                safe_c2rust_g_dbus_annotation_info_unref
                    as unsafe extern "C" fn(*mut GDBusAnnotationInfo) -> (),
            )),
            NULL_0,
        );
        g_ptr_array_free(annotations, TRUE);
        l = (*l).next;
    }
    g_slist_free((*data).annotations_stack);
    l = (*data).interfaces_stack;
    while !l.is_null() {
        let mut interfaces: *mut GPtrArray = (*l).data as *mut GPtrArray;
        g_ptr_array_foreach(
            interfaces,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> ()>,
                GFunc,
            >(Some(
                safe_c2rust_g_dbus_interface_info_unref
                    as unsafe extern "C" fn(*mut GDBusInterfaceInfo) -> (),
            )),
            NULL_0,
        );
        g_ptr_array_free(interfaces, TRUE);
        l = (*l).next;
    }
    g_slist_free((*data).interfaces_stack);
    l = (*data).nodes_stack;
    while !l.is_null() {
        let mut nodes: *mut GPtrArray = (*l).data as *mut GPtrArray;
        g_ptr_array_foreach(
            nodes,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GDBusNodeInfo) -> ()>, GFunc>(
                Some(
                    safe_c2rust_g_dbus_node_info_unref
                        as unsafe extern "C" fn(*mut GDBusNodeInfo) -> (),
                ),
            ),
            NULL_0,
        );
        g_ptr_array_free(nodes, TRUE);
        l = (*l).next;
    }
    g_slist_free((*data).nodes_stack);
    safe_c2rust_parse_data_free_args(data);
    safe_c2rust_parse_data_free_out_args(data);
    safe_c2rust_parse_data_free_methods(data);
    safe_c2rust_parse_data_free_signals(data);
    safe_c2rust_parse_data_free_properties(data);
    safe_c2rust_parse_data_free_interfaces(data);
    safe_c2rust_parse_data_free_annotations(data);
    safe_c2rust_parse_data_free_nodes(data);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_parser_start_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut current_block: u64;
    let mut data: *mut ParseData = user_data as *mut ParseData;
    let mut stack: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut type_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut access: *const gchar = ::core::ptr::null::<gchar>();
    let mut direction: *const gchar = ::core::ptr::null::<gchar>();
    let mut value: *const gchar = ::core::ptr::null::<gchar>();
    name = ::core::ptr::null::<gchar>();
    type_0 = ::core::ptr::null::<gchar>();
    access = ::core::ptr::null::<gchar>();
    direction = ::core::ptr::null::<gchar>();
    value = ::core::ptr::null::<gchar>();
    stack = g_markup_parse_context_get_element_stack(context) as *mut GSList;
    if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"node\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !(g_slist_length(stack) >= 1 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"node\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<node> elements can only be top-level or embedded in other <node> elements\0"
                    as *const u8 as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            (G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int)
                as GMarkupCollectType,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int,
            b"xmlns:doc\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            safe_c2rust_g_dbus_node_info_set(
                data,
                safe_c2rust_parse_data_get_node(data, TRUE),
                name,
                ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>(),
                ::core::ptr::null_mut::<*mut GDBusNodeInfo>(),
                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            );
            (*data).interfaces_stack =
                g_slist_prepend((*data).interfaces_stack, (*data).interfaces as gpointer);
            (*data).interfaces = ::core::ptr::null_mut::<GPtrArray>();
            safe_c2rust_parse_data_steal_interfaces(data, ::core::ptr::null_mut::<guint>());
            (*data).nodes_stack = g_slist_prepend((*data).nodes_stack, (*data).nodes as gpointer);
            (*data).nodes = ::core::ptr::null_mut::<GPtrArray>();
            safe_c2rust_parse_data_steal_nodes(data, ::core::ptr::null_mut::<guint>());
            current_block = 9437375157805982253;
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"interface\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"node\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<interface> elements can only be embedded in <node> elements\0" as *const u8
                    as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            G_MARKUP_COLLECT_STRING,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            safe_c2rust_g_dbus_interface_info_set(
                data,
                safe_c2rust_parse_data_get_interface(data, TRUE),
                name,
                ::core::ptr::null_mut::<*mut GDBusMethodInfo>(),
                ::core::ptr::null_mut::<*mut GDBusSignalInfo>(),
                ::core::ptr::null_mut::<*mut GDBusPropertyInfo>(),
                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            );
            current_block = 9437375157805982253;
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"method\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"interface\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<method> elements can only be embedded in <interface> elements\0" as *const u8
                    as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            G_MARKUP_COLLECT_STRING,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            safe_c2rust_g_dbus_method_info_set(
                data,
                safe_c2rust_parse_data_get_method(data, TRUE),
                name,
                ::core::ptr::null_mut::<*mut GDBusArgInfo>(),
                ::core::ptr::null_mut::<*mut GDBusArgInfo>(),
                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            );
            (*data).num_args = 0 as guint;
            current_block = 9437375157805982253;
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"signal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"interface\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<signal> elements can only be embedded in <interface> elements\0" as *const u8
                    as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            G_MARKUP_COLLECT_STRING,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            safe_c2rust_g_dbus_signal_info_set(
                data,
                safe_c2rust_parse_data_get_signal(data, TRUE),
                name,
                ::core::ptr::null_mut::<*mut GDBusArgInfo>(),
                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            );
            (*data).num_args = 0 as guint;
            current_block = 9437375157805982253;
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut flags: GDBusPropertyInfoFlags = G_DBUS_PROPERTY_INFO_FLAGS_NONE;
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"interface\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<property> elements can only be embedded in <interface> elements\0" as *const u8
                    as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            G_MARKUP_COLLECT_STRING,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut type_0,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int,
            b"access\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut access,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            if strcmp(
                access as *const ::core::ffi::c_char,
                b"read\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                flags = G_DBUS_PROPERTY_INFO_FLAGS_READABLE;
                current_block = 7226443171521532240;
            } else if strcmp(
                access as *const ::core::ffi::c_char,
                b"write\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                flags = G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE;
                current_block = 7226443171521532240;
            } else if strcmp(
                access as *const ::core::ffi::c_char,
                b"readwrite\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                flags = (G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int
                    | G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE as ::core::ffi::c_int)
                    as GDBusPropertyInfoFlags;
                current_block = 7226443171521532240;
            } else {
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    b"Unknown value '%s' of access attribute for element <property>\0" as *const u8
                        as *const gchar,
                    access,
                );
                current_block = 10435735846551762309;
            }
            match current_block {
                10435735846551762309 => {}
                _ => {
                    safe_c2rust_g_dbus_property_info_set(
                        data,
                        safe_c2rust_parse_data_get_property(data, TRUE),
                        name,
                        type_0,
                        flags,
                        ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
                    );
                    current_block = 9437375157805982253;
                }
            }
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"arg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut is_in: gboolean = 0;
        let mut name_to_use: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"method\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"signal\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<arg> elements can only be embedded in <method> or <signal> elements\0"
                    as *const u8 as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            (G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int)
                as GMarkupCollectType,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int
                | G_MARKUP_COLLECT_OPTIONAL as ::core::ffi::c_int,
            b"direction\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut direction,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut type_0,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            if strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"method\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                is_in = TRUE as gboolean;
            } else {
                is_in = FALSE as gboolean;
            }
            if !direction.is_null() {
                if strcmp(
                    direction as *const ::core::ffi::c_char,
                    b"in\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    is_in = TRUE as gboolean;
                    current_block = 7018308795614528254;
                } else if strcmp(
                    direction as *const ::core::ffi::c_char,
                    b"out\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    is_in = FALSE as gboolean;
                    current_block = 7018308795614528254;
                } else {
                    g_set_error(
                        error,
                        g_markup_error_quark(),
                        G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                        b"Unknown value '%s' of direction attribute\0" as *const u8 as *const gchar,
                        direction,
                    );
                    current_block = 10435735846551762309;
                }
            } else {
                current_block = 7018308795614528254;
            }
            match current_block {
                10435735846551762309 => {}
                _ => {
                    if is_in != 0
                        && strcmp(
                            (*(*stack).next).data as *const ::core::ffi::c_char,
                            b"signal\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        g_set_error_literal(
                            error,
                            g_markup_error_quark(),
                            G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                            b"Only direction 'out' is allowed for <arg> elements embedded in <signal>\0"
                                as *const u8 as *const gchar,
                        );
                        current_block = 10435735846551762309;
                    } else {
                        if name.is_null() {
                            name_to_use = g_strdup_printf(
                                b"arg_%d\0" as *const u8 as *const gchar,
                                (*data).num_args,
                            );
                        } else {
                            name_to_use =
                                safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char)
                                    as *mut gchar;
                        }
                        (*data).num_args = (*data).num_args.wrapping_add(1);
                        if is_in != 0 {
                            safe_c2rust_g_dbus_arg_info_set(
                                data,
                                safe_c2rust_parse_data_get_arg(data, TRUE),
                                name_to_use,
                                type_0,
                                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
                            );
                            (*data).last_arg_was_in = TRUE as gboolean;
                        } else {
                            safe_c2rust_g_dbus_arg_info_set(
                                data,
                                safe_c2rust_parse_data_get_out_arg(data, TRUE),
                                name_to_use,
                                type_0,
                                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
                            );
                            (*data).last_arg_was_in = FALSE as gboolean;
                        }
                        g_free(name_to_use as gpointer);
                        current_block = 9437375157805982253;
                    }
                }
            }
        }
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"annotation\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if g_slist_length(stack) < 2 as guint
            || strcmp(
                (*(*stack).next).data as *const ::core::ffi::c_char,
                b"node\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"interface\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"signal\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"method\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"property\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"arg\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*(*stack).next).data as *const ::core::ffi::c_char,
                    b"annotation\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
        {
            g_set_error_literal(
                error,
                g_markup_error_quark(),
                G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                b"<annotation> elements can only be embedded in <node>, <interface>, <signal>, <method>, <property>, <arg> or <annotation> elements\0"
                    as *const u8 as *const gchar,
            );
            current_block = 10435735846551762309;
        } else if g_markup_collect_attributes(
            element_name,
            attribute_names,
            attribute_values,
            error,
            G_MARKUP_COLLECT_STRING,
            b"name\0" as *const u8 as *const gchar,
            &raw mut name,
            G_MARKUP_COLLECT_STRING as ::core::ffi::c_int,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut value,
            G_MARKUP_COLLECT_INVALID as ::core::ffi::c_int,
        ) == 0
        {
            current_block = 10435735846551762309;
        } else {
            safe_c2rust_g_dbus_annotation_info_set(
                data,
                safe_c2rust_parse_data_get_annotation(data, TRUE),
                name,
                value,
                ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>(),
            );
            current_block = 9437375157805982253;
        }
    } else {
        current_block = 9437375157805982253;
    }
    match current_block {
        9437375157805982253 => {
            (*data).annotations_stack =
                g_slist_prepend((*data).annotations_stack, (*data).annotations as gpointer);
            (*data).annotations = ::core::ptr::null_mut::<GPtrArray>();
            safe_c2rust_parse_data_steal_annotations(data, ::core::ptr::null_mut::<guint>());
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_steal_annotations(
    mut data: *mut ParseData,
) -> *mut *mut GDBusAnnotationInfo {
    return safe_c2rust_parse_data_steal_annotations(data, ::core::ptr::null_mut::<guint>());
}
unsafe extern "C" fn safe_c2rust_parser_end_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut data: *mut ParseData = user_data as *mut ParseData;
    let mut have_popped_annotations: gboolean = 0;
    have_popped_annotations = FALSE as gboolean;
    if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"node\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut num_nodes: guint = 0;
        let mut num_interfaces: guint = 0;
        let mut nodes: *mut *mut GDBusNodeInfo = ::core::ptr::null_mut::<*mut GDBusNodeInfo>();
        let mut interfaces: *mut *mut GDBusInterfaceInfo =
            ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
        nodes = safe_c2rust_parse_data_steal_nodes(data, &raw mut num_nodes);
        interfaces = safe_c2rust_parse_data_steal_interfaces(data, &raw mut num_interfaces);
        safe_c2rust_parse_data_free_interfaces(data);
        (*data).interfaces = (*(*data).interfaces_stack).data as *mut GPtrArray;
        (*data).interfaces_stack = g_slist_remove(
            (*data).interfaces_stack,
            (*(*data).interfaces_stack).data as gconstpointer,
        );
        safe_c2rust_parse_data_free_nodes(data);
        (*data).nodes = (*(*data).nodes_stack).data as *mut GPtrArray;
        (*data).nodes_stack = g_slist_remove(
            (*data).nodes_stack,
            (*(*data).nodes_stack).data as gconstpointer,
        );
        safe_c2rust_g_dbus_node_info_set(
            data,
            safe_c2rust_parse_data_get_node(data, FALSE),
            ::core::ptr::null::<gchar>(),
            interfaces,
            nodes,
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"interface\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut num_methods: guint = 0;
        let mut num_signals: guint = 0;
        let mut num_properties: guint = 0;
        let mut methods: *mut *mut GDBusMethodInfo =
            ::core::ptr::null_mut::<*mut GDBusMethodInfo>();
        let mut signals: *mut *mut GDBusSignalInfo =
            ::core::ptr::null_mut::<*mut GDBusSignalInfo>();
        let mut properties: *mut *mut GDBusPropertyInfo =
            ::core::ptr::null_mut::<*mut GDBusPropertyInfo>();
        methods = safe_c2rust_parse_data_steal_methods(data, &raw mut num_methods);
        signals = safe_c2rust_parse_data_steal_signals(data, &raw mut num_signals);
        properties = safe_c2rust_parse_data_steal_properties(data, &raw mut num_properties);
        safe_c2rust_g_dbus_interface_info_set(
            data,
            safe_c2rust_parse_data_get_interface(data, FALSE),
            ::core::ptr::null::<gchar>(),
            methods,
            signals,
            properties,
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"method\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut in_num_args: guint = 0;
        let mut out_num_args: guint = 0;
        let mut in_args: *mut *mut GDBusArgInfo = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
        let mut out_args: *mut *mut GDBusArgInfo = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
        in_args = safe_c2rust_parse_data_steal_args(data, &raw mut in_num_args);
        out_args = safe_c2rust_parse_data_steal_out_args(data, &raw mut out_num_args);
        safe_c2rust_g_dbus_method_info_set(
            data,
            safe_c2rust_parse_data_get_method(data, FALSE),
            ::core::ptr::null::<gchar>(),
            in_args,
            out_args,
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"signal\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut num_args: guint = 0;
        let mut args: *mut *mut GDBusArgInfo = ::core::ptr::null_mut::<*mut GDBusArgInfo>();
        args = safe_c2rust_parse_data_steal_out_args(data, &raw mut num_args);
        safe_c2rust_g_dbus_signal_info_set(
            data,
            safe_c2rust_parse_data_get_signal(data, FALSE),
            ::core::ptr::null::<gchar>(),
            args,
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_g_dbus_property_info_set(
            data,
            safe_c2rust_parse_data_get_property(data, FALSE),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            G_DBUS_PROPERTY_INFO_FLAGS_NONE,
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"arg\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_g_dbus_arg_info_set(
            data,
            if (*data).last_arg_was_in != 0 {
                safe_c2rust_parse_data_get_arg(data, FALSE)
            } else {
                safe_c2rust_parse_data_get_out_arg(data, FALSE)
            },
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            safe_c2rust_steal_annotations(data),
        );
    } else if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"annotation\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut embedded_annotations: *mut *mut GDBusAnnotationInfo =
            ::core::ptr::null_mut::<*mut GDBusAnnotationInfo>();
        embedded_annotations = safe_c2rust_steal_annotations(data);
        safe_c2rust_parse_data_free_annotations(data);
        (*data).annotations = (*(*data).annotations_stack).data as *mut GPtrArray;
        (*data).annotations_stack = g_slist_remove(
            (*data).annotations_stack,
            (*(*data).annotations_stack).data as gconstpointer,
        );
        have_popped_annotations = TRUE as gboolean;
        safe_c2rust_g_dbus_annotation_info_set(
            data,
            safe_c2rust_parse_data_get_annotation(data, FALSE),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            embedded_annotations,
        );
    }
    if have_popped_annotations == 0 {
        safe_c2rust_parse_data_free_annotations(data);
        (*data).annotations = (*(*data).annotations_stack).data as *mut GPtrArray;
        (*data).annotations_stack = g_slist_remove(
            (*data).annotations_stack,
            (*(*data).annotations_stack).data as gconstpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_parser_error(
    mut context: *mut GMarkupParseContext,
    mut error: *mut GError,
    mut user_data: gpointer,
) {
    let mut line_number: gint = 0;
    let mut char_number: gint = 0;
    g_markup_parse_context_get_position(context, &raw mut line_number, &raw mut char_number);
    g_prefix_error(
        &raw mut error,
        b"%d:%d: \0" as *const u8 as *const gchar,
        line_number,
        char_number,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_new_for_xml(
    mut xml_data: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GDBusNodeInfo {
    let mut ret: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
    let mut context: *mut GMarkupParseContext = ::core::ptr::null_mut::<GMarkupParseContext>();
    let mut parser: *mut GMarkupParser = ::core::ptr::null_mut::<GMarkupParser>();
    let mut num_nodes: guint = 0;
    let mut data: *mut ParseData = ::core::ptr::null_mut::<ParseData>();
    let mut ughret: *mut *mut GDBusNodeInfo = ::core::ptr::null_mut::<*mut GDBusNodeInfo>();
    ret = ::core::ptr::null_mut::<GDBusNodeInfo>();
    parser = ::core::ptr::null_mut::<GMarkupParser>();
    context = ::core::ptr::null_mut::<GMarkupParseContext>();
    parser = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GMarkupParser>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GMarkupParser;
    (*parser).start_element = Some(
        safe_c2rust_parser_start_element
            as unsafe extern "C" fn(
                *mut GMarkupParseContext,
                *const gchar,
                *mut *const gchar,
                *mut *const gchar,
                gpointer,
                *mut *mut GError,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMarkupParseContext,
                *const gchar,
                *mut *const gchar,
                *mut *const gchar,
                gpointer,
                *mut *mut GError,
            ) -> (),
        >;
    (*parser).end_element = Some(
        safe_c2rust_parser_end_element
            as unsafe extern "C" fn(
                *mut GMarkupParseContext,
                *const gchar,
                gpointer,
                *mut *mut GError,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GMarkupParseContext,
                *const gchar,
                gpointer,
                *mut *mut GError,
            ) -> (),
        >;
    (*parser).error = Some(
        safe_c2rust_parser_error
            as unsafe extern "C" fn(*mut GMarkupParseContext, *mut GError, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMarkupParseContext, *mut GError, gpointer) -> ()>;
    data = safe_c2rust_parse_data_new();
    context = g_markup_parse_context_new(
        parser,
        G_MARKUP_IGNORE_QUALIFIED,
        data as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut ParseData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_parse_data_free as unsafe extern "C" fn(*mut ParseData) -> ()),
        ),
    );
    if !(g_markup_parse_context_parse(
        context,
        xml_data,
        strlen(xml_data as *const ::core::ffi::c_char) as gssize,
        error,
    ) == 0)
    {
        if !(g_markup_parse_context_end_parse(context, error) == 0) {
            ughret = safe_c2rust_parse_data_steal_nodes(data, &raw mut num_nodes);
            if num_nodes != 1 as guint {
                let mut n: guint = 0;
                g_set_error(
                    error,
                    g_markup_error_quark(),
                    G_MARKUP_ERROR_INVALID_CONTENT as ::core::ffi::c_int as gint,
                    b"Expected a single node in introspection XML, found %d\0" as *const u8
                        as *const gchar,
                    num_nodes,
                );
                n = 0 as guint;
                while n < num_nodes {
                    safe_c2rust_g_dbus_node_info_unref(*ughret.offset(n as isize));
                    let ref mut fresh0 = *ughret.offset(n as isize);
                    *fresh0 = ::core::ptr::null_mut::<GDBusNodeInfo>();
                    n = n.wrapping_add(1);
                }
            }
            ret = *ughret.offset(0 as ::core::ffi::c_int as isize);
            g_free(ughret as gpointer);
        }
    }
    g_free(parser as gpointer);
    if !context.is_null() {
        g_markup_parse_context_free(context);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_annotation_info_lookup(
    mut annotations: *mut *mut GDBusAnnotationInfo,
    mut name: *const gchar,
) -> *const gchar {
    let mut n: guint = 0;
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null::<gchar>();
    n = 0 as guint;
    while !annotations.is_null() && !(*annotations.offset(n as isize)).is_null() {
        if g_strcmp0(
            (**annotations.offset(n as isize)).key,
            name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ret = (**annotations.offset(n as isize)).value;
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    return ret;
}
static mut safe_c2rust_g__info_cache_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_info_cache_free(mut cache: *mut InfoCacheEntry) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*cache).use_count == 0 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusintrospection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1884 as ::core::ffi::c_int,
            G_STRFUNC,
            b"cache->use_count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_unref((*cache).method_name_to_data);
    g_hash_table_unref((*cache).signal_name_to_data);
    g_hash_table_unref((*cache).property_name_to_data);
    g_slice_free1(
        ::core::mem::size_of::<InfoCacheEntry>() as gsize,
        cache as gpointer,
    );
}
static mut safe_c2rust_info_cache: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_lookup_method(
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
) -> *mut GDBusMethodInfo {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut result: *mut GDBusMethodInfo = ::core::ptr::null_mut::<GDBusMethodInfo>();
    g_mutex_lock(&raw mut safe_c2rust_g__info_cache_lock_lock);
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !safe_c2rust_info_cache.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
        let mut cache: *mut InfoCacheEntry = ::core::ptr::null_mut::<InfoCacheEntry>();
        cache = g_hash_table_lookup(safe_c2rust_info_cache, info as gconstpointer)
            as *mut InfoCacheEntry;
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if !cache.is_null() {
                _g_boolean_var_25 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_25 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_25
        }) as ::core::ffi::c_long
            != 0
        {
            result = g_hash_table_lookup((*cache).method_name_to_data, name as gconstpointer)
                as *mut GDBusMethodInfo;
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            current_block = 7777713881006634943;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            n = 0 as guint;
            loop {
                if !(!(*info).methods.is_null() && !(*(*info).methods.offset(n as isize)).is_null())
                {
                    current_block = 12209867499936983673;
                    break;
                }
                let mut i: *mut GDBusMethodInfo = *(*info).methods.offset(n as isize);
                if g_strcmp0((*i).name, name as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    result = i;
                    current_block = 7777713881006634943;
                    break;
                } else {
                    n = n.wrapping_add(1);
                }
            }
            match current_block {
                7777713881006634943 => {}
                _ => {
                    result = ::core::ptr::null_mut::<GDBusMethodInfo>();
                }
            }
        }
        _ => {}
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_lookup_signal(
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
) -> *mut GDBusSignalInfo {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut result: *mut GDBusSignalInfo = ::core::ptr::null_mut::<GDBusSignalInfo>();
    g_mutex_lock(&raw mut safe_c2rust_g__info_cache_lock_lock);
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !safe_c2rust_info_cache.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
        let mut cache: *mut InfoCacheEntry = ::core::ptr::null_mut::<InfoCacheEntry>();
        cache = g_hash_table_lookup(safe_c2rust_info_cache, info as gconstpointer)
            as *mut InfoCacheEntry;
        if ({
            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
            if !cache.is_null() {
                _g_boolean_var_27 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_27 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_27
        }) as ::core::ffi::c_long
            != 0
        {
            result = g_hash_table_lookup((*cache).signal_name_to_data, name as gconstpointer)
                as *mut GDBusSignalInfo;
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            current_block = 13732487832244106400;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            n = 0 as guint;
            loop {
                if !(!(*info).signals.is_null() && !(*(*info).signals.offset(n as isize)).is_null())
                {
                    current_block = 12209867499936983673;
                    break;
                }
                let mut i: *mut GDBusSignalInfo = *(*info).signals.offset(n as isize);
                if g_strcmp0((*i).name, name as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    result = i;
                    current_block = 13732487832244106400;
                    break;
                } else {
                    n = n.wrapping_add(1);
                }
            }
            match current_block {
                13732487832244106400 => {}
                _ => {
                    result = ::core::ptr::null_mut::<GDBusSignalInfo>();
                }
            }
        }
        _ => {}
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_lookup_property(
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
) -> *mut GDBusPropertyInfo {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut result: *mut GDBusPropertyInfo = ::core::ptr::null_mut::<GDBusPropertyInfo>();
    g_mutex_lock(&raw mut safe_c2rust_g__info_cache_lock_lock);
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !safe_c2rust_info_cache.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
        let mut cache: *mut InfoCacheEntry = ::core::ptr::null_mut::<InfoCacheEntry>();
        cache = g_hash_table_lookup(safe_c2rust_info_cache, info as gconstpointer)
            as *mut InfoCacheEntry;
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if !cache.is_null() {
                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_29
        }) as ::core::ffi::c_long
            != 0
        {
            result = g_hash_table_lookup((*cache).property_name_to_data, name as gconstpointer)
                as *mut GDBusPropertyInfo;
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            current_block = 7594129354491890485;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
            n = 0 as guint;
            loop {
                if !(!(*info).properties.is_null()
                    && !(*(*info).properties.offset(n as isize)).is_null())
                {
                    current_block = 12209867499936983673;
                    break;
                }
                let mut i: *mut GDBusPropertyInfo = *(*info).properties.offset(n as isize);
                if g_strcmp0((*i).name, name as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    result = i;
                    current_block = 7594129354491890485;
                    break;
                } else {
                    n = n.wrapping_add(1);
                }
            }
            match current_block {
                7594129354491890485 => {}
                _ => {
                    result = ::core::ptr::null_mut::<GDBusPropertyInfo>();
                }
            }
        }
        _ => {}
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_cache_build(
    mut info: *mut GDBusInterfaceInfo,
) {
    let mut cache: *mut InfoCacheEntry = ::core::ptr::null_mut::<InfoCacheEntry>();
    let mut n: guint = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__info_cache_lock_lock);
    if safe_c2rust_info_cache.is_null() {
        safe_c2rust_info_cache = g_hash_table_new_full(
            Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut InfoCacheEntry) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_info_cache_free as unsafe extern "C" fn(*mut InfoCacheEntry) -> (),
            )),
        );
    }
    cache =
        g_hash_table_lookup(safe_c2rust_info_cache, info as gconstpointer) as *mut InfoCacheEntry;
    if !cache.is_null() {
        (*cache).use_count += 1 as ::core::ffi::c_int;
    } else {
        cache = ({
            let mut __s: gsize = ::core::mem::size_of::<InfoCacheEntry>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut InfoCacheEntry;
        (*cache).use_count = 1 as ::core::ffi::c_int as gint;
        (*cache).method_name_to_data = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        (*cache).signal_name_to_data = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        (*cache).property_name_to_data = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
        n = 0 as guint;
        while !(*info).methods.is_null() && !(*(*info).methods.offset(n as isize)).is_null() {
            g_hash_table_insert(
                (*cache).method_name_to_data,
                (**(*info).methods.offset(n as isize)).name as gpointer,
                *(*info).methods.offset(n as isize) as gpointer,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).signals.is_null() && !(*(*info).signals.offset(n as isize)).is_null() {
            g_hash_table_insert(
                (*cache).signal_name_to_data,
                (**(*info).signals.offset(n as isize)).name as gpointer,
                *(*info).signals.offset(n as isize) as gpointer,
            );
            n = n.wrapping_add(1);
        }
        n = 0 as guint;
        while !(*info).properties.is_null() && !(*(*info).properties.offset(n as isize)).is_null() {
            g_hash_table_insert(
                (*cache).property_name_to_data,
                (**(*info).properties.offset(n as isize)).name as gpointer,
                *(*info).properties.offset(n as isize) as gpointer,
            );
            n = n.wrapping_add(1);
        }
        g_hash_table_insert(safe_c2rust_info_cache, info as gpointer, cache as gpointer);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_info_cache_release(
    mut info: *mut GDBusInterfaceInfo,
) {
    let mut cache: *mut InfoCacheEntry = ::core::ptr::null_mut::<InfoCacheEntry>();
    g_mutex_lock(&raw mut safe_c2rust_g__info_cache_lock_lock);
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if safe_c2rust_info_cache.is_null() {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s called for interface %s but there is no cache\0" as *const u8 as *const gchar,
            (*info).name,
            b"g_dbus_interface_info_cache_release\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        cache = g_hash_table_lookup(safe_c2rust_info_cache, info as gconstpointer)
            as *mut InfoCacheEntry;
        if ({
            let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
            if cache.is_null() {
                _g_boolean_var_31 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_31 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_31
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s called for interface %s but there is no cache entry\0" as *const u8
                    as *const gchar,
                (*info).name,
                b"g_dbus_interface_info_cache_release\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            (*cache).use_count -= 1 as ::core::ffi::c_int;
            if (*cache).use_count == 0 as ::core::ffi::c_int {
                g_hash_table_remove(safe_c2rust_info_cache, info as gconstpointer);
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__info_cache_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_node_info_lookup_interface(
    mut info: *mut GDBusNodeInfo,
    mut name: *const gchar,
) -> *mut GDBusInterfaceInfo {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut result: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    n = 0 as guint;
    loop {
        if !(!(*info).interfaces.is_null() && !(*(*info).interfaces.offset(n as isize)).is_null()) {
            current_block = 15427931788582360902;
            break;
        }
        let mut i: *mut GDBusInterfaceInfo = *(*info).interfaces.offset(n as isize);
        if g_strcmp0((*i).name, name as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            result = i;
            current_block = 3825750978674620935;
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    match current_block {
        15427931788582360902 => {
            result = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
        }
        _ => {}
    }
    return result;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
