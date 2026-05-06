use ::c2rust_bitfields;
extern "C" {
    pub type _GTimeZone;
    pub type _GDateTime;
    pub type _GData;
    pub type _GHashTable;
    pub type _GIcon;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_array_insert_vals(
        array: *mut GArray,
        index_: guint,
        data: gconstpointer,
        len: guint,
    ) -> *mut GArray;
    fn g_array_set_size(array: *mut GArray, length: guint) -> *mut GArray;
    fn g_array_remove_index(array: *mut GArray, index_: guint) -> *mut GArray;
    fn g_array_sort(array: *mut GArray, compare_func: GCompareFunc);
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_time_zone_new_local() -> *mut GTimeZone;
    fn g_time_zone_unref(tz: *mut GTimeZone);
    fn g_date_time_unref(datetime: *mut GDateTime);
    fn g_date_time_new_from_unix_utc(t: gint64) -> *mut GDateTime;
    fn g_date_time_new_from_iso8601(
        text: *const gchar,
        default_tz: *mut GTimeZone,
    ) -> *mut GDateTime;
    fn g_date_time_add(datetime: *mut GDateTime, timespan: GTimeSpan) -> *mut GDateTime;
    fn g_date_time_get_microsecond(datetime: *mut GDateTime) -> gint;
    fn g_date_time_to_unix(datetime: *mut GDateTime) -> gint64;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
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
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn _g_file_attribute_value_clear(attr: *mut GFileAttributeValue);
    fn _g_file_attribute_value_set(
        attr: *mut GFileAttributeValue,
        new_value: *const GFileAttributeValue,
    );
    fn _g_file_attribute_value_peek_as_pointer(attr: *mut GFileAttributeValue) -> gpointer;
    fn _g_file_attribute_value_as_string(
        attr: *const GFileAttributeValue,
    ) -> *mut ::core::ffi::c_char;
    fn _g_file_attribute_value_get_string(
        attr: *const GFileAttributeValue,
    ) -> *const ::core::ffi::c_char;
    fn _g_file_attribute_value_get_byte_string(
        attr: *const GFileAttributeValue,
    ) -> *const ::core::ffi::c_char;
    fn _g_file_attribute_value_get_boolean(attr: *const GFileAttributeValue) -> gboolean;
    fn _g_file_attribute_value_get_uint32(attr: *const GFileAttributeValue) -> guint32;
    fn _g_file_attribute_value_get_int32(attr: *const GFileAttributeValue) -> gint32;
    fn _g_file_attribute_value_get_uint64(attr: *const GFileAttributeValue) -> guint64;
    fn _g_file_attribute_value_get_int64(attr: *const GFileAttributeValue) -> gint64;
    fn _g_file_attribute_value_get_object(attr: *const GFileAttributeValue) -> *mut GObject;
    fn _g_file_attribute_value_get_stringv(
        attr: *const GFileAttributeValue,
    ) -> *mut *mut ::core::ffi::c_char;
    fn _g_file_attribute_value_set_from_pointer(
        attr: *mut GFileAttributeValue,
        type_0: GFileAttributeType,
        value_p: gpointer,
        dup: gboolean,
    );
    fn _g_file_attribute_value_set_string(
        attr: *mut GFileAttributeValue,
        string: *const ::core::ffi::c_char,
    );
    fn _g_file_attribute_value_set_byte_string(
        attr: *mut GFileAttributeValue,
        string: *const ::core::ffi::c_char,
    );
    fn _g_file_attribute_value_set_boolean(attr: *mut GFileAttributeValue, value: gboolean);
    fn _g_file_attribute_value_set_uint32(attr: *mut GFileAttributeValue, value: guint32);
    fn _g_file_attribute_value_set_int32(attr: *mut GFileAttributeValue, value: gint32);
    fn _g_file_attribute_value_set_uint64(attr: *mut GFileAttributeValue, value: guint64);
    fn _g_file_attribute_value_set_int64(attr: *mut GFileAttributeValue, value: gint64);
    fn _g_file_attribute_value_set_object(attr: *mut GFileAttributeValue, obj: *mut GObject);
    fn _g_file_attribute_value_set_stringv(
        attr: *mut GFileAttributeValue,
        value: *mut *mut ::core::ffi::c_char,
    );
    fn g_icon_get_type() -> GType;
}
pub type size_t = usize;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTimeVal {
    pub tv_sec: glong,
    pub tv_usec: glong,
}
pub type GTimeVal = _GTimeVal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GTimeZone = _GTimeZone;
pub type GTimeSpan = gint64;
pub type GDateTime = _GDateTime;
pub type GData = _GData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GParamFlags = ::core::ffi::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor:
        Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>,
    pub set_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>,
    pub get_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> ()>,
    pub notify: Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeStatus = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING: GFileAttributeStatus = 2;
pub const G_FILE_ATTRIBUTE_STATUS_SET: GFileAttributeStatus = 1;
pub const G_FILE_ATTRIBUTE_STATUS_UNSET: GFileAttributeStatus = 0;
pub type GFileType = ::core::ffi::c_uint;
pub const G_FILE_TYPE_MOUNTABLE: GFileType = 6;
pub const G_FILE_TYPE_SHORTCUT: GFileType = 5;
pub const G_FILE_TYPE_SPECIAL: GFileType = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: GFileType = 3;
pub const G_FILE_TYPE_DIRECTORY: GFileType = 2;
pub const G_FILE_TYPE_REGULAR: GFileType = 1;
pub const G_FILE_TYPE_UNKNOWN: GFileType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInfo {
    pub parent_instance: GObject,
    pub attributes: *mut GArray,
    pub mask: *mut GFileAttributeMatcher,
}
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeMatcher {
    pub all: gboolean,
    pub ref_0: gint,
    pub sub_matchers: *mut GArray,
    pub iterator_ns: guint32,
    pub iterator_pos: gint,
}
pub type GFileInfo = _GFileInfo;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInfoClass {
    pub parent_class: GObjectClass,
}
pub type GFileInfoClass = _GFileInfoClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GFileAttribute {
    pub attribute: guint32,
    pub value: GFileAttributeValue,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GFileAttributeValue {
    #[bitfield(name = "type_0", ty = "guint", bits = "0..=7")]
    #[bitfield(name = "status", ty = "guint", bits = "8..=15")]
    pub type_0_status: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: gboolean,
    pub int32: gint32,
    pub uint32: guint32,
    pub int64: gint64,
    pub uint64: guint64,
    pub string: *mut ::core::ffi::c_char,
    pub obj: *mut GObject,
    pub stringv: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NSInfo {
    pub id: guint32,
    pub attribute_id_counter: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubMatcher {
    pub id: guint32,
    pub mask: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GFileAttributeMatcher) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GFileAttributeMatcher) -> *mut GFileAttributeMatcher>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GFileAttributeMatcher) -> *mut GFileAttributeMatcher>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const G_FILE_ATTRIBUTE_STANDARD_TYPE: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::type\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_IS_HIDDEN: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"standard::is-hidden\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_IS_BACKUP: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"standard::is-backup\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_IS_SYMLINK: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"standard::is-symlink\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_IS_VIRTUAL: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"standard::is-virtual\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_IS_VOLATILE: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"standard::is-volatile\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_NAME: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::name\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"standard::display-name\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"standard::edit-name\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_COPY_NAME: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"standard::copy-name\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_DESCRIPTION: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"standard::description\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_ICON: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::icon\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"standard::symbolic-icon\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"standard::content-type\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"standard::fast-content-type\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_SIZE: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::size\0") };
pub const G_FILE_ATTRIBUTE_STANDARD_ALLOCATED_SIZE: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"standard::allocated-size\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"standard::symlink-target\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_TARGET_URI: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"standard::target-uri\0")
};
pub const G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"standard::sort-order\0")
};
pub const G_FILE_ATTRIBUTE_ETAG_VALUE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"etag::value\0") };
pub const G_FILE_ATTRIBUTE_ID_FILE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"id::file\0") };
pub const G_FILE_ATTRIBUTE_ID_FILESYSTEM: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"id::filesystem\0") };
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_READ: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"access::can-read\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_WRITE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"access::can-write\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"access::can-execute\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_DELETE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"access::can-delete\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_TRASH: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"access::can-trash\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_RENAME: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"access::can-rename\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_MOUNT: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"mountable::can-mount\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_UNMOUNT: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"mountable::can-unmount\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_EJECT: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"mountable::can-eject\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"mountable::unix-device\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE_FILE: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"mountable::unix-device-file\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_HAL_UDI: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"mountable::hal-udi\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_START: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"mountable::can-start\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_START_DEGRADED: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"mountable::can-start-degraded\0",
    )
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_STOP: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"mountable::can-stop\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_START_STOP_TYPE: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"mountable::start-stop-type\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_CAN_POLL: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"mountable::can-poll\0")
};
pub const G_FILE_ATTRIBUTE_MOUNTABLE_IS_MEDIA_CHECK_AUTOMATIC: [::core::ffi::c_char; 36] = unsafe {
    ::core::mem::transmute::<[u8; 36], [::core::ffi::c_char; 36]>(
        *b"mountable::is-media-check-automatic\0",
    )
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"time::modified\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-nsec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_ACCESS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"time::access\0") };
pub const G_FILE_ATTRIBUTE_TIME_ACCESS_USEC: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"time::access-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"time::access-nsec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_CHANGED: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"time::changed\0") };
pub const G_FILE_ATTRIBUTE_TIME_CHANGED_USEC: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"time::changed-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_CHANGED_NSEC: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"time::changed-nsec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_CREATED: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"time::created\0") };
pub const G_FILE_ATTRIBUTE_TIME_CREATED_USEC: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"time::created-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_CREATED_NSEC: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"time::created-nsec\0")
};
pub const G_FILE_ATTRIBUTE_UNIX_DEVICE: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"unix::device\0") };
pub const G_FILE_ATTRIBUTE_UNIX_INODE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"unix::inode\0") };
pub const G_FILE_ATTRIBUTE_UNIX_MODE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"unix::mode\0") };
pub const G_FILE_ATTRIBUTE_UNIX_NLINK: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"unix::nlink\0") };
pub const G_FILE_ATTRIBUTE_UNIX_UID: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"unix::uid\0") };
pub const G_FILE_ATTRIBUTE_UNIX_GID: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"unix::gid\0") };
pub const G_FILE_ATTRIBUTE_UNIX_RDEV: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"unix::rdev\0") };
pub const G_FILE_ATTRIBUTE_UNIX_BLOCK_SIZE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"unix::block-size\0")
};
pub const G_FILE_ATTRIBUTE_UNIX_BLOCKS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"unix::blocks\0") };
pub const G_FILE_ATTRIBUTE_UNIX_IS_MOUNTPOINT: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"unix::is-mountpoint\0")
};
pub const G_FILE_ATTRIBUTE_DOS_IS_ARCHIVE: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"dos::is-archive\0") };
pub const G_FILE_ATTRIBUTE_DOS_IS_SYSTEM: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"dos::is-system\0") };
pub const G_FILE_ATTRIBUTE_DOS_IS_MOUNTPOINT: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"dos::is-mountpoint\0")
};
pub const G_FILE_ATTRIBUTE_DOS_REPARSE_POINT_TAG: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"dos::reparse-point-tag\0")
};
pub const G_FILE_ATTRIBUTE_OWNER_USER: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"owner::user\0") };
pub const G_FILE_ATTRIBUTE_OWNER_USER_REAL: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"owner::user-real\0")
};
pub const G_FILE_ATTRIBUTE_OWNER_GROUP: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"owner::group\0") };
pub const G_FILE_ATTRIBUTE_THUMBNAIL_PATH: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"thumbnail::path\0") };
pub const G_FILE_ATTRIBUTE_THUMBNAILING_FAILED: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"thumbnail::failed\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"thumbnail::is-valid\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_PATH_NORMAL: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"thumbnail::path-normal\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_NORMAL: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"thumbnail::failed-normal\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_NORMAL: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"thumbnail::is-valid-normal\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_PATH_LARGE: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"thumbnail::path-large\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_LARGE: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"thumbnail::failed-large\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_LARGE: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"thumbnail::is-valid-large\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_PATH_XLARGE: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"thumbnail::path-xlarge\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_XLARGE: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"thumbnail::failed-xlarge\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_XLARGE: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"thumbnail::is-valid-xlarge\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_PATH_XXLARGE: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"thumbnail::path-xxlarge\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_XXLARGE: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"thumbnail::failed-xxlarge\0")
};
pub const G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_XXLARGE: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"thumbnail::is-valid-xxlarge\0")
};
pub const G_FILE_ATTRIBUTE_PREVIEW_ICON: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"preview::icon\0") };
pub const G_FILE_ATTRIBUTE_FILESYSTEM_SIZE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::size\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_FREE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::free\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_TYPE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::type\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_READONLY: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"filesystem::readonly\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_USE_PREVIEW: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"filesystem::use-preview\0")
};
pub const G_FILE_ATTRIBUTE_GVFS_BACKEND: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"gvfs::backend\0") };
pub const G_FILE_ATTRIBUTE_SELINUX_CONTEXT: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"selinux::context\0")
};
pub const G_FILE_ATTRIBUTE_TRASH_ITEM_COUNT: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"trash::item-count\0")
};
pub const G_FILE_ATTRIBUTE_TRASH_ORIG_PATH: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"trash::orig-path\0")
};
pub const G_FILE_ATTRIBUTE_TRASH_DELETION_DATE: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"trash::deletion-date\0")
};
pub const NO_ATTRIBUTE_MASK: *mut GFileAttributeMatcher =
    1 as ::core::ffi::c_int as *mut GFileAttributeMatcher;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_file_info_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GFileInfo\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GFileInfoClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_info_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GFileInfo>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GFileInfo) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_file_info_init as unsafe extern "C" fn(*mut GFileInfo) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GFileInfo_private_offset: gint = 0;
static mut safe_c2rust_g_file_info_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_file_info_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_file_info_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GFileInfo_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GFileInfo_private_offset);
    }
    safe_c2rust_g_file_info_class_init(klass as *mut GFileInfoClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_file_info_get_type_once();
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
static mut safe_c2rust_g__attribute_hash_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_namespace_id_counter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut safe_c2rust_ns_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_attribute_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_global_attributes: *mut *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut *mut ::core::ffi::c_char>() as *mut *mut *mut ::core::ffi::c_char;
pub const NS_POS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const NS_MASK: guint32 =
    (((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as guint32;
pub const ID_POS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ID_MASK: guint32 =
    (((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as guint32;
unsafe extern "C" fn safe_c2rust__lookup_namespace(
    mut namespace: *const ::core::ffi::c_char,
) -> *mut NSInfo {
    let mut ns_info: *mut NSInfo = ::core::ptr::null_mut::<NSInfo>();
    ns_info = g_hash_table_lookup(safe_c2rust_ns_hash, namespace as gconstpointer) as *mut NSInfo;
    if ns_info.is_null() {
        ns_info = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<NSInfo>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut NSInfo;
        safe_c2rust_namespace_id_counter += 1;
        (*ns_info).id = safe_c2rust_namespace_id_counter as guint32;
        g_hash_table_insert(
            safe_c2rust_ns_hash,
            safe_c2rust_g_strdup_inline(namespace) as gpointer,
            ns_info as gpointer,
        );
        safe_c2rust_global_attributes = g_realloc(
            safe_c2rust_global_attributes as gpointer,
            ((*ns_info).id.wrapping_add(1 as guint32) as gsize)
                .wrapping_mul(::core::mem::size_of::<*mut *mut ::core::ffi::c_char>() as gsize),
        ) as *mut *mut *mut ::core::ffi::c_char;
        let ref mut fresh4 = *safe_c2rust_global_attributes.offset((*ns_info).id as isize);
        *fresh4 = ({
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
        }) as *mut *mut ::core::ffi::c_char;
        let ref mut fresh5 = *(*safe_c2rust_global_attributes.offset((*ns_info).id as isize))
            .offset(0 as ::core::ffi::c_int as isize);
        *fresh5 = g_strconcat(
            namespace as *const gchar,
            b"::*\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
    }
    return ns_info;
}
unsafe extern "C" fn safe_c2rust__lookup_attribute(
    mut attribute: *const ::core::ffi::c_char,
) -> guint32 {
    let mut attr_id: guint32 = 0;
    let mut id: guint32 = 0;
    let mut ns: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut colon: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ns_info: *mut NSInfo = ::core::ptr::null_mut::<NSInfo>();
    attr_id = g_hash_table_lookup(safe_c2rust_attribute_hash, attribute as gconstpointer) as gulong
        as guint as guint32;
    if attr_id != 0 as guint32 {
        return attr_id;
    }
    colon = strstr(
        attribute,
        b"::\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !colon.is_null() {
        ns = g_strndup(
            attribute as *const gchar,
            colon.offset_from(attribute) as ::core::ffi::c_long as gsize,
        ) as *mut ::core::ffi::c_char;
    } else {
        ns = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    ns_info = safe_c2rust__lookup_namespace(ns);
    g_free(ns as gpointer);
    (*ns_info).attribute_id_counter = (*ns_info).attribute_id_counter.wrapping_add(1);
    id = (*ns_info).attribute_id_counter;
    let ref mut fresh2 = *safe_c2rust_global_attributes.offset((*ns_info).id as isize);
    *fresh2 = g_realloc(
        *safe_c2rust_global_attributes.offset((*ns_info).id as isize) as gpointer,
        (id.wrapping_add(1 as guint32) as gsize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize),
    ) as *mut *mut ::core::ffi::c_char;
    let ref mut fresh3 =
        *(*safe_c2rust_global_attributes.offset((*ns_info).id as isize)).offset(id as isize);
    *fresh3 = safe_c2rust_g_strdup_inline(attribute);
    attr_id = ((*ns_info).id & NS_MASK) << NS_POS | (id & ID_MASK) << ID_POS;
    g_hash_table_insert(
        safe_c2rust_attribute_hash,
        *(*safe_c2rust_global_attributes.offset((*ns_info).id as isize)).offset(id as isize)
            as gpointer,
        attr_id as gulong as gpointer,
    );
    return attr_id;
}
unsafe extern "C" fn safe_c2rust_ensure_attribute_hash() {
    if !safe_c2rust_attribute_hash.is_null() {
        return;
    }
    safe_c2rust_ns_hash = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_attribute_hash = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    let mut _u: guint = 0;
    _u = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if _u == (1048576 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            199 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_TYPE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_0: guint = 0;
    _u_0 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_HIDDEN.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if _u_0 == (1048576 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            200 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_IS_HIDDEN\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_1: guint = 0;
    _u_1 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_BACKUP.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if _u_1 == (1048576 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            201 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_IS_BACKUP\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_2: guint = 0;
    _u_2 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_SYMLINK.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if _u_2 == (1048576 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            202 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_IS_SYMLINK\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_3: guint = 0;
    _u_3 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_VIRTUAL.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if _u_3 == (1048576 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            203 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_IS_VIRTUAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_4: guint = 0;
    _u_4 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if _u_4 == (1048576 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            204 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_NAME\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_5: guint = 0;
    _u_5 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if _u_5 == (1048576 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as guint {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            205 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_DISPLAY_NAME\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_6: guint = 0;
    _u_6 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if _u_6 == (1048576 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as guint {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            206 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_EDIT_NAME\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_7: guint = 0;
    _u_7 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_COPY_NAME.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if _u_7 == (1048576 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as guint {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            207 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_COPY_NAME\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_8: guint = 0;
    _u_8 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_DESCRIPTION.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if _u_8 == (1048576 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as guint {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            208 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_DESCRIPTION\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_9: guint = 0;
    _u_9 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_ICON.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if _u_9 == (1048576 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            209 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_ICON\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_10: guint = 0;
    _u_10 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if _u_10 == (1048576 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            210 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_11: guint = 0;
    _u_11 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if _u_11 == (1048576 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as guint {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            211 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_12: guint = 0;
    _u_12 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if _u_12 == (1048576 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            212 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_SIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_13: guint = 0;
    _u_13 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_ALLOCATED_SIZE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if _u_13 == (1048576 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            213 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_ALLOCATED_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_14: guint = 0;
    _u_14 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if _u_14 == (1048576 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            214 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_SYMLINK_TARGET\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_15: guint = 0;
    _u_15 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_TARGET_URI.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if _u_15 == (1048576 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            215 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_TARGET_URI\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_16: guint = 0;
    _u_16 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if _u_16 == (1048576 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            216 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_SORT_ORDER\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_17: guint = 0;
    _u_17 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if _u_17 == (1048576 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            217 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_SYMBOLIC_ICON\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_18: guint = 0;
    _u_18 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_VOLATILE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if _u_18 == (1048576 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as guint {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            218 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_STANDARD_IS_VOLATILE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_19: guint = 0;
    _u_19 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ETAG_VALUE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if _u_19 == (2097152 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            219 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ETAG_VALUE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_20: guint = 0;
    _u_20 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ID_FILE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if _u_20 == (3145728 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            220 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ID_FILE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_21: guint = 0;
    _u_21 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ID_FILESYSTEM.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if _u_21 == (3145728 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            221 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ID_FILESYSTEM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_22: guint = 0;
    _u_22 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_READ.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if _u_22 == (4194304 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            222 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_23: guint = 0;
    _u_23 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_WRITE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if _u_23 == (4194304 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            223 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_24: guint = 0;
    _u_24 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if _u_24 == (4194304 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            224 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_25: guint = 0;
    _u_25 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_DELETE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if _u_25 == (4194304 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            225 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_26: guint = 0;
    _u_26 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_TRASH.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if _u_26 == (4194304 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            226 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_27: guint = 0;
    _u_27 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_ACCESS_CAN_RENAME.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if _u_27 == (4194304 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            227 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_28: guint = 0;
    _u_28 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_MOUNT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if _u_28 == (5242880 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            228 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_MOUNT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_29: guint = 0;
    _u_29 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_UNMOUNT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if _u_29 == (5242880 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            229 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_UNMOUNT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_30: guint = 0;
    _u_30 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_EJECT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if _u_30 == (5242880 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_EJECT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_31: guint = 0;
    _u_31 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if _u_31 == (5242880 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            231 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_UNIX_DEVICE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_32: guint = 0;
    _u_32 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE_FILE.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if _u_32 == (5242880 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            232 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_UNIX_DEVICE_FILE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_33: guint = 0;
    _u_33 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_HAL_UDI.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if _u_33 == (5242880 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            233 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_HAL_UDI\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_34: guint = 0;
    _u_34 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_START.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if _u_34 == (5242880 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as guint {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            234 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_START\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_35: guint = 0;
    _u_35 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_START_DEGRADED.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if _u_35 == (5242880 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as guint {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            235 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_START_DEGRADED\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_36: guint = 0;
    _u_36 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_STOP.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if _u_36 == (5242880 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as guint {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            236 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_STOP\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_37: guint = 0;
    _u_37 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_START_STOP_TYPE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if _u_37 == (5242880 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as guint {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            237 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_START_STOP_TYPE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_38: guint = 0;
    _u_38 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_CAN_POLL.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if _u_38 == (5242880 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as guint {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            238 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_CAN_POLL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_39: guint = 0;
    _u_39 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_MOUNTABLE_IS_MEDIA_CHECK_AUTOMATIC.as_ptr())
            as guint;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if _u_39 == (5242880 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            239 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_MOUNTABLE_IS_MEDIA_CHECK_AUTOMATIC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_40: guint = 0;
    _u_40 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if _u_40 == (6291456 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            240 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_MODIFIED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_41: guint = 0;
    _u_41 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if _u_41 == (6291456 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            241 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_USEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_42: guint = 0;
    _u_42 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if _u_42 == (6291456 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            242 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_ACCESS\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_43: guint = 0;
    _u_43 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if _u_43 == (6291456 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            243 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_ACCESS_USEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_44: guint = 0;
    _u_44 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CHANGED.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if _u_44 == (6291456 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            244 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CHANGED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_45: guint = 0;
    _u_45 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CHANGED_USEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if _u_45 == (6291456 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            245 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CHANGED_USEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_46: guint = 0;
    _u_46 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if _u_46 == (6291456 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as guint {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            246 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CREATED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_47: guint = 0;
    _u_47 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED_USEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if _u_47 == (6291456 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as guint {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            247 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CREATED_USEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_48: guint = 0;
    _u_48 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if _u_48 == (6291456 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as guint {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_NSEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_49: guint = 0;
    _u_49 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if _u_49 == (6291456 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as guint {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            249 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_ACCESS_NSEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_50: guint = 0;
    _u_50 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED_NSEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if _u_50 == (6291456 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as guint {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            250 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CREATED_NSEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_51: guint = 0;
    _u_51 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TIME_CHANGED_NSEC.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if _u_51 == (6291456 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as guint {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            251 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TIME_CHANGED_NSEC\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_52: guint = 0;
    _u_52 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_DEVICE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if _u_52 == (7340032 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            252 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_DEVICE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_53: guint = 0;
    _u_53 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_INODE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if _u_53 == (7340032 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            253 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_INODE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_54: guint = 0;
    _u_54 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_MODE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if _u_54 == (7340032 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            254 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_MODE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_55: guint = 0;
    _u_55 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_NLINK.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if _u_55 == (7340032 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_NLINK\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_56: guint = 0;
    _u_56 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_UID.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if _u_56 == (7340032 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            256 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_UID\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_57: guint = 0;
    _u_57 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_GID.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if _u_57 == (7340032 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            257 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_GID\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_58: guint = 0;
    _u_58 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_RDEV.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if _u_58 == (7340032 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as guint {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            258 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_RDEV\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_59: guint = 0;
    _u_59 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_BLOCK_SIZE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if _u_59 == (7340032 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as guint {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            259 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_BLOCK_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_60: guint = 0;
    _u_60 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_BLOCKS.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if _u_60 == (7340032 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            260 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_BLOCKS\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_61: guint = 0;
    _u_61 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_UNIX_IS_MOUNTPOINT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if _u_61 == (7340032 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as guint {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_UNIX_IS_MOUNTPOINT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_62: guint = 0;
    _u_62 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_DOS_IS_ARCHIVE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if _u_62 == (8388608 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            262 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_DOS_IS_ARCHIVE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_63: guint = 0;
    _u_63 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_DOS_IS_SYSTEM.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if _u_63 == (8388608 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            263 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_DOS_IS_SYSTEM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_64: guint = 0;
    _u_64 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_DOS_IS_MOUNTPOINT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if _u_64 == (8388608 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            264 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_DOS_IS_MOUNTPOINT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_65: guint = 0;
    _u_65 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_DOS_REPARSE_POINT_TAG.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if _u_65 == (8388608 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            265 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_DOS_REPARSE_POINT_TAG\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_66: guint = 0;
    _u_66 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_OWNER_USER.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if _u_66 == (9437184 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            266 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_OWNER_USER\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_67: guint = 0;
    _u_67 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_OWNER_USER_REAL.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if _u_67 == (9437184 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            267 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_OWNER_USER_REAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_68: guint = 0;
    _u_68 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_OWNER_GROUP.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if _u_68 == (9437184 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            268 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_OWNER_GROUP\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_69: guint = 0;
    _u_69 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_PATH.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if _u_69 == (10485760 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            269 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_70: guint = 0;
    _u_70 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAILING_FAILED.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if _u_70 == (10485760 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            270 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_71: guint = 0;
    _u_71 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if _u_71 == (10485760 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            271 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_72: guint = 0;
    _u_72 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_PATH_NORMAL.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if _u_72 == (10485760 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            272 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_NORMAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_73: guint = 0;
    _u_73 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_NORMAL.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if _u_73 == (10485760 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            273 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_NORMAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_74: guint = 0;
    _u_74 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_NORMAL.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if _u_74 == (10485760 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as guint {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            274 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_NORMAL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_75: guint = 0;
    _u_75 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_PATH_LARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if _u_75 == (10485760 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as guint {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            275 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_LARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_76: guint = 0;
    _u_76 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_LARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if _u_76 == (10485760 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as guint {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            276 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_LARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_77: guint = 0;
    _u_77 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_LARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if _u_77 == (10485760 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as guint {
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            277 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_LARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_78: guint = 0;
    _u_78 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_PATH_XLARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if _u_78 == (10485760 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as guint {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            278 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_79: guint = 0;
    _u_79 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_XLARGE.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if _u_79 == (10485760 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as guint {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            279 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_80: guint = 0;
    _u_80 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_XLARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if _u_80 == (10485760 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as guint {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            280 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_81: guint = 0;
    _u_81 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_PATH_XXLARGE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if _u_81 == (10485760 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as guint {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            281 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XXLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_82: guint = 0;
    _u_82 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAILING_FAILED_XXLARGE.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if _u_82 == (10485760 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as guint {
            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_93
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            282 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XXLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_83: guint = 0;
    _u_83 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_THUMBNAIL_IS_VALID_XXLARGE.as_ptr())
        as guint;
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if _u_83 == (10485760 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as guint {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            283 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XXLARGE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_84: guint = 0;
    _u_84 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_PREVIEW_ICON.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if _u_84 == (11534336 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            284 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_PREVIEW_ICON\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_85: guint = 0;
    _u_85 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_FILESYSTEM_SIZE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if _u_85 == (12582912 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            285 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_FILESYSTEM_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_86: guint = 0;
    _u_86 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_FILESYSTEM_FREE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if _u_86 == (12582912 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            286 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_FILESYSTEM_FREE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_87: guint = 0;
    _u_87 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if _u_87 == (12582912 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_FILESYSTEM_TYPE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_88: guint = 0;
    _u_88 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if _u_88 == (12582912 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            288 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_FILESYSTEM_READONLY\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_89: guint = 0;
    _u_89 =
        safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_FILESYSTEM_USE_PREVIEW.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if _u_89 == (12582912 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as guint {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_FILESYSTEM_USE_PREVIEW\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_90: guint = 0;
    _u_90 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_GVFS_BACKEND.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if _u_90 == (13631488 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            290 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_GVFS_BACKEND\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _u_91: guint = 0;
    _u_91 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_SELINUX_CONTEXT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if _u_91 == (14680064 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            291 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_92: guint = 0;
    _u_92 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TRASH_ITEM_COUNT.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if _u_92 == (15728640 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as guint {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            292 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TRASH_ITEM_COUNT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_93: guint = 0;
    _u_93 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TRASH_ORIG_PATH.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if _u_93 == (15728640 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as guint {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            293 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TRASH_ORIG_PATH\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    let mut _u_94: guint = 0;
    _u_94 = safe_c2rust__lookup_attribute(G_FILE_ATTRIBUTE_TRASH_DELETION_DATE.as_ptr()) as guint;
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if _u_94 == (15728640 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as guint {
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
            G_STRFUNC,
            b"_u == G_FILE_ATTRIBUTE_ID_TRASH_DELETION_DATE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_lookup_namespace(
    mut namespace: *const ::core::ffi::c_char,
) -> guint32 {
    let mut ns_info: *mut NSInfo = ::core::ptr::null_mut::<NSInfo>();
    let mut id: guint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__attribute_hash_lock);
    safe_c2rust_ensure_attribute_hash();
    ns_info = safe_c2rust__lookup_namespace(namespace);
    id = 0 as guint32;
    if !ns_info.is_null() {
        id = (*ns_info).id;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__attribute_hash_lock);
    return id;
}
unsafe extern "C" fn safe_c2rust_get_attribute_for_id(
    mut attribute: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_mutex_lock(&raw mut safe_c2rust_g__attribute_hash_lock);
    s = *(*safe_c2rust_global_attributes
        .offset((attribute as guint32 >> NS_POS & NS_MASK) as isize))
    .offset((attribute as guint32 >> ID_POS & ID_MASK) as isize);
    g_mutex_unlock(&raw mut safe_c2rust_g__attribute_hash_lock);
    return s;
}
unsafe extern "C" fn safe_c2rust_lookup_attribute(
    mut attribute: *const ::core::ffi::c_char,
) -> guint32 {
    let mut attr_id: guint32 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__attribute_hash_lock);
    safe_c2rust_ensure_attribute_hash();
    attr_id = safe_c2rust__lookup_attribute(attribute);
    g_mutex_unlock(&raw mut safe_c2rust_g__attribute_hash_lock);
    return attr_id;
}
unsafe extern "C" fn safe_c2rust_g_file_info_finalize(mut object: *mut GObject) {
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut i: guint = 0;
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    info = object as *mut ::core::ffi::c_void as *mut GFileInfo;
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*info).attributes).len {
        _g_file_attribute_value_clear(&raw mut (*attrs.offset(i as isize)).value);
        i = i.wrapping_add(1);
    }
    g_array_free((*info).attributes, TRUE);
    if (*info).mask != NO_ATTRIBUTE_MASK {
        safe_c2rust_g_file_attribute_matcher_unref((*info).mask);
    }
    (*(safe_c2rust_g_file_info_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_file_info_class_init(mut klass: *mut GFileInfoClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_file_info_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_file_info_init(mut info: *mut GFileInfo) {
    (*info).mask = NO_ATTRIBUTE_MASK;
    (*info).attributes = g_array_new(
        FALSE,
        FALSE,
        ::core::mem::size_of::<GFileAttribute>() as guint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_new() -> *mut GFileInfo {
    return g_object_new(
        safe_c2rust_g_file_info_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GFileInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_copy_into(
    mut src_info: *mut GFileInfo,
    mut dest_info: *mut GFileInfo,
) {
    let mut source: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut dest: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = src_info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (src_info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = dest_info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (dest_info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    dest = (*(*dest_info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*dest_info).attributes).len {
        _g_file_attribute_value_clear(&raw mut (*dest.offset(i as isize)).value);
        i = i.wrapping_add(1);
    }
    g_array_set_size((*dest_info).attributes, (*(*src_info).attributes).len);
    source = (*(*src_info).attributes).data as *mut GFileAttribute;
    dest = (*(*dest_info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*src_info).attributes).len {
        (*dest.offset(i as isize)).attribute = (*source.offset(i as isize)).attribute;
        let ref mut fresh1 = (*dest.offset(i as isize)).value;
        (*fresh1).set_type_0(G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int as guint as guint);
        _g_file_attribute_value_set(
            &raw mut (*dest.offset(i as isize)).value,
            &raw mut (*source.offset(i as isize)).value,
        );
        i = i.wrapping_add(1);
    }
    if (*dest_info).mask != NO_ATTRIBUTE_MASK {
        safe_c2rust_g_file_attribute_matcher_unref((*dest_info).mask);
    }
    if (*src_info).mask == NO_ATTRIBUTE_MASK {
        (*dest_info).mask = NO_ATTRIBUTE_MASK;
    } else {
        (*dest_info).mask = safe_c2rust_g_file_attribute_matcher_ref((*src_info).mask);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_dup(mut other: *mut GFileInfo) -> *mut GFileInfo {
    let mut new: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = other as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (other)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    new = safe_c2rust_g_file_info_new();
    safe_c2rust_g_file_info_copy_into(other, new);
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_mask(
    mut info: *mut GFileInfo,
    mut mask: *mut GFileAttributeMatcher,
) {
    let mut attr: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if mask != (*info).mask {
        if (*info).mask != NO_ATTRIBUTE_MASK {
            safe_c2rust_g_file_attribute_matcher_unref((*info).mask);
        }
        (*info).mask = safe_c2rust_g_file_attribute_matcher_ref(mask);
        i = 0 as guint;
        while i < (*(*info).attributes).len {
            attr = ((*(*info).attributes).data as *mut ::core::ffi::c_void as *mut GFileAttribute)
                .offset(i as isize) as *mut GFileAttribute;
            if safe_c2rust__g_file_attribute_matcher_matches_id(mask, (*attr).attribute) == 0 {
                _g_file_attribute_value_clear(&raw mut (*attr).value);
                g_array_remove_index((*info).attributes, i);
                i = i.wrapping_sub(1);
            }
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_unset_attribute_mask(mut info: *mut GFileInfo) {
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*info).mask != NO_ATTRIBUTE_MASK {
        safe_c2rust_g_file_attribute_matcher_unref((*info).mask);
    }
    (*info).mask = NO_ATTRIBUTE_MASK;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_clear_status(mut info: *mut GFileInfo) {
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*info).attributes).len {
        let ref mut fresh6 = (*attrs.offset(i as isize)).value;
        (*fresh6).set_status(G_FILE_ATTRIBUTE_STATUS_UNSET as ::core::ffi::c_int as guint as guint);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_g_file_info_find_place(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
) -> ::core::ffi::c_int {
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut med: ::core::ffi::c_int = 0;
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    min = 0 as ::core::ffi::c_int;
    max = (*(*info).attributes).len as ::core::ffi::c_int;
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    while min < max {
        med = min + (max - min) / 2 as ::core::ffi::c_int;
        if (*attrs.offset(med as isize)).attribute == attribute {
            min = med;
            break;
        } else if (*attrs.offset(med as isize)).attribute < attribute {
            min = med + 1 as ::core::ffi::c_int;
        } else {
            max = med;
        }
    }
    return min;
}
unsafe extern "C" fn safe_c2rust_g_file_info_find_value(
    mut info: *mut GFileInfo,
    mut attr_id: guint32,
) -> *mut GFileAttributeValue {
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    i = safe_c2rust_g_file_info_find_place(info, attr_id) as guint;
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    if i < (*(*info).attributes).len && (*attrs.offset(i as isize)).attribute == attr_id {
        return &raw mut (*attrs.offset(i as isize)).value;
    }
    return ::core::ptr::null_mut::<GFileAttributeValue>();
}
unsafe extern "C" fn safe_c2rust_g_file_info_find_value_by_name(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *mut GFileAttributeValue {
    let mut attr_id: guint32 = 0;
    attr_id = safe_c2rust_lookup_attribute(attribute);
    return safe_c2rust_g_file_info_find_value(info, attr_id);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_has_attribute(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return (value != NULL_0 as *mut GFileAttributeValue) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_has_namespace(
    mut info: *mut GFileInfo,
    mut name_space: *const ::core::ffi::c_char,
) -> gboolean {
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut ns_id: guint32 = 0;
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if !name_space.is_null() {
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
            b"name_space != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ns_id = safe_c2rust_lookup_namespace(name_space);
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*info).attributes).len {
        if (*attrs.offset(i as isize)).attribute >> NS_POS & NS_MASK == ns_id {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_list_attributes(
    mut info: *mut GFileInfo,
    mut name_space: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut names: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut attribute: guint32 = 0;
    let mut ns_id: guint32 = if !name_space.is_null() {
        safe_c2rust_lookup_namespace(name_space)
    } else {
        0 as guint32
    };
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    names = g_ptr_array_new();
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    i = 0 as guint;
    while i < (*(*info).attributes).len {
        attribute = (*attrs.offset(i as isize)).attribute;
        if ns_id == 0 as guint32 || attribute >> NS_POS & NS_MASK == ns_id {
            g_ptr_array_add(
                names,
                safe_c2rust_g_strdup_inline(safe_c2rust_get_attribute_for_id(
                    attribute as ::core::ffi::c_int,
                )) as gpointer,
            );
        }
        i = i.wrapping_add(1);
    }
    g_ptr_array_add(names, NULL_0);
    return g_ptr_array_free(names, FALSE) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_type(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> GFileAttributeType {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_FILE_ATTRIBUTE_TYPE_INVALID;
    }
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_FILE_ATTRIBUTE_TYPE_INVALID;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    if !value.is_null() {
        return (*value).type_0() as GFileAttributeType;
    } else {
        return G_FILE_ATTRIBUTE_TYPE_INVALID;
    };
}
unsafe extern "C" fn safe_c2rust_g_file_info_remove_value(
    mut info: *mut GFileInfo,
    mut attr_id: guint32,
) {
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    if (*info).mask != NO_ATTRIBUTE_MASK
        && safe_c2rust__g_file_attribute_matcher_matches_id((*info).mask, attr_id) == 0
    {
        return;
    }
    i = safe_c2rust_g_file_info_find_place(info, attr_id) as guint;
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    if i < (*(*info).attributes).len && (*attrs.offset(i as isize)).attribute == attr_id {
        _g_file_attribute_value_clear(&raw mut (*attrs.offset(i as isize)).value);
        g_array_remove_index((*info).attributes, i);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_remove_attribute(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) {
    let mut attr_id: guint32 = 0;
    if ({
        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    attr_id = safe_c2rust_lookup_attribute(attribute);
    safe_c2rust_g_file_info_remove_value(info, attr_id);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_data(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut type_0: *mut GFileAttributeType,
    mut value_pp: *mut gpointer,
    mut status: *mut GFileAttributeStatus,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    if value.is_null() {
        return FALSE;
    }
    if !status.is_null() {
        *status = (*value).status() as GFileAttributeStatus;
    }
    if !type_0.is_null() {
        *type_0 = (*value).type_0() as GFileAttributeType;
    }
    if !value_pp.is_null() {
        *value_pp = _g_file_attribute_value_peek_as_pointer(value);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_status(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> GFileAttributeStatus {
    let mut val: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_FILE_ATTRIBUTE_STATUS_UNSET;
    }
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_FILE_ATTRIBUTE_STATUS_UNSET;
    }
    val = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    if !val.is_null() {
        return (*val).status() as GFileAttributeStatus;
    }
    return G_FILE_ATTRIBUTE_STATUS_UNSET;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_status(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut status: GFileAttributeStatus,
) -> gboolean {
    let mut val: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    val = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    if !val.is_null() {
        (*val).set_status(status as guint as guint);
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_get_attribute_value(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *mut GFileAttributeValue {
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeValue>();
    }
    if ({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFileAttributeValue>();
    }
    return safe_c2rust_g_file_info_find_value_by_name(info, attribute);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_as_string(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut val: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    val = safe_c2rust__g_file_info_get_attribute_value(info, attribute);
    if !val.is_null() {
        return _g_file_attribute_value_as_string(val);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_object(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *mut GObject {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_object(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_string(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_byte_string(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_133 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_133 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_133
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_byte_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_file_path(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return safe_c2rust_g_file_info_get_attribute_byte_string(info, attribute);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_stringv(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_stringv(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_boolean(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_boolean(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_uint32(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> guint32 {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if ({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_uint32(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_int32(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> gint32 {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    if ({
        let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_int32(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_uint64(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> guint64 {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    if ({
        let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_uint64(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_attribute_int64(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
) -> gint64 {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    if ({
        let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    value = safe_c2rust_g_file_info_find_value_by_name(info, attribute);
    return _g_file_attribute_value_get_int64(value);
}
unsafe extern "C" fn safe_c2rust_g_file_info_create_value(
    mut info: *mut GFileInfo,
    mut attr_id: guint32,
) -> *mut GFileAttributeValue {
    let mut attrs: *mut GFileAttribute = ::core::ptr::null_mut::<GFileAttribute>();
    let mut i: guint = 0;
    if (*info).mask != NO_ATTRIBUTE_MASK
        && safe_c2rust__g_file_attribute_matcher_matches_id((*info).mask, attr_id) == 0
    {
        return ::core::ptr::null_mut::<GFileAttributeValue>();
    }
    i = safe_c2rust_g_file_info_find_place(info, attr_id) as guint;
    attrs = (*(*info).attributes).data as *mut GFileAttribute;
    if i < (*(*info).attributes).len && (*attrs.offset(i as isize)).attribute == attr_id {
        return &raw mut (*attrs.offset(i as isize)).value;
    } else {
        let mut attr: GFileAttribute = GFileAttribute {
            attribute: 0 as guint32,
            value: GFileAttributeValue {
                type_0_status: [0; 2],
                c2rust_padding: [0; 6],
                u: C2RustUnnamed_0 { boolean: 0 },
            },
        };
        attr.attribute = attr_id;
        g_array_insert_vals(
            (*info).attributes,
            i,
            &raw mut attr as gconstpointer,
            1 as guint,
        );
        attrs = (*(*info).attributes).data as *mut GFileAttribute;
        return &raw mut (*attrs.offset(i as isize)).value;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_from_pointer(value, type_0, value_p, TRUE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
) {
    if ({
        let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_148 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_148 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_148
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        type_0,
        value_p,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_object_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: *mut GObject,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_object(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_object(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: *mut GObject,
) {
    if ({
        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_149
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if g_type_check_instance_is_fundamentally_a(
            attr_value as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
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
            b"G_IS_OBJECT (attr_value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_object_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_stringv_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: *mut *mut ::core::ffi::c_char,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_stringv(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_stringv(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: *mut *mut ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_154: ::core::ffi::c_int = 0;
        if !attr_value.is_null() {
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
            b"attr_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_stringv_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_string_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: *const ::core::ffi::c_char,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_string(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_string(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_155: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_156: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_156 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_156 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_156
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_157: ::core::ffi::c_int = 0;
        if !attr_value.is_null() {
            _g_boolean_var_157 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_157 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_157
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attr_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_string_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_byte_string_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: *const ::core::ffi::c_char,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_byte_string(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_byte_string(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_158: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_159: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_160: ::core::ffi::c_int = 0;
        if !attr_value.is_null() {
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
            b"attr_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_byte_string_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_file_path(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_file_info_set_attribute_byte_string(info, attribute, attr_value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_boolean_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: gboolean,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_boolean(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_boolean(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: gboolean,
) {
    if ({
        let mut _g_boolean_var_161: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_162: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_boolean_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_uint32_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: guint32,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_uint32(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: guint32,
) {
    if ({
        let mut _g_boolean_var_163: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
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
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_164: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
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
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_uint32_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_int32_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: gint32,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_int32(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_int32(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: gint32,
) {
    if ({
        let mut _g_boolean_var_165: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_165 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_165 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_165
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_166: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_166 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_166 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_166
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_int32_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_uint64_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: guint64,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_uint64(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: guint64,
) {
    if ({
        let mut _g_boolean_var_167: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_167 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_167 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_167
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_168: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_168 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_168 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_168
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_uint64_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_info_set_attribute_int64_by_id(
    mut info: *mut GFileInfo,
    mut attribute: guint32,
    mut attr_value: gint64,
) {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    value = safe_c2rust_g_file_info_create_value(info, attribute);
    if !value.is_null() {
        _g_file_attribute_value_set_int64(value, attr_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_attribute_int64(
    mut info: *mut GFileInfo,
    mut attribute: *const ::core::ffi::c_char,
    mut attr_value: gint64,
) {
    if ({
        let mut _g_boolean_var_169: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_169 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_169 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_169
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_170: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_170 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_170 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_170
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust__g_file_info_set_attribute_int64_by_id(
        info,
        safe_c2rust_lookup_attribute(attribute),
        attr_value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_deletion_date(
    mut info: *mut GFileInfo,
) -> *mut GDateTime {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut date_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut local_tz: *mut GTimeZone = ::core::ptr::null_mut::<GTimeZone>();
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_171: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_171 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_171 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_171
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TRASH_DELETION_DATE.as_ptr());
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    date_str = _g_file_attribute_value_get_string(value);
    if date_str.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    local_tz = g_time_zone_new_local();
    dt = g_date_time_new_from_iso8601(date_str as *const gchar, local_tz);
    g_time_zone_unref(local_tz);
    return safe_c2rust_g_steal_pointer(&raw mut dt as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_file_type(
    mut info: *mut GFileInfo,
) -> GFileType {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_172: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_172 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_172 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_172
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_FILE_TYPE_UNKNOWN;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::type\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_173: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_173 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_173 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_173
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::type\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1610 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return G_FILE_TYPE_UNKNOWN;
    }
    return _g_file_attribute_value_get_uint32(value) as GFileType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_is_hidden(
    mut info: *mut GFileInfo,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_174: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_174 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_174 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_174
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::is-hidden\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_175: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_175 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_175 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_175
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::is-hidden\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1632 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gboolean;
    }
    return _g_file_attribute_value_get_boolean(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_is_backup(
    mut info: *mut GFileInfo,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_176: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_176 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_176 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_176
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::is-backup\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_177: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_177 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_177 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_177
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::is-backup\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gboolean;
    }
    return _g_file_attribute_value_get_boolean(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_is_symlink(
    mut info: *mut GFileInfo,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_178: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_178 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_178 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_178
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::is-symlink\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_179: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_179 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_179 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_179
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::is-symlink\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1676 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gboolean;
    }
    return _g_file_attribute_value_get_boolean(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_name(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_180: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_180 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_180 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_180
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_181: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_181 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_181 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_181
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::name\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1698 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_byte_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_display_name(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_182: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_182 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_182 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_182
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::display-name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_183: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_183 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_183 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_183
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::display-name\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1720 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_edit_name(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_184: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_184 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_184 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_184
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::edit-name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_185: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_185 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_185 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_185
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::edit-name\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1742 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_icon(mut info: *mut GFileInfo) -> *mut GIcon {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut obj: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut _g_boolean_var_186: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_186 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_186 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_186
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::icon\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_187: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_187 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_187 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_187
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::icon\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1765 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    obj = _g_file_attribute_value_get_object(value);
    if ({
        let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
        let mut __t: GType = g_icon_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        return obj as *mut ::core::ffi::c_void as *mut GIcon;
    }
    return ::core::ptr::null_mut::<GIcon>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_symbolic_icon(
    mut info: *mut GFileInfo,
) -> *mut GIcon {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut obj: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut _g_boolean_var_188: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_188 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_188 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_188
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::symbolic-icon\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_189: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_189 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_189 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_189
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::symbolic-icon\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1794 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    obj = _g_file_attribute_value_get_object(value);
    if ({
        let mut __inst: *mut GTypeInstance = obj as *mut GTypeInstance;
        let mut __t: GType = g_icon_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        return obj as *mut ::core::ffi::c_void as *mut GIcon;
    }
    return ::core::ptr::null_mut::<GIcon>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_content_type(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_190: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_190 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_190 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_190
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::content-type\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_191: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_191 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_191 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_191
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::content-type\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1821 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_size(mut info: *mut GFileInfo) -> goffset {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_192: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_192 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_192 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_192
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as goffset;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_193: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_193 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_193 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_193
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::size\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1845 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as ::core::ffi::c_int as goffset;
    }
    return _g_file_attribute_value_get_uint64(value) as goffset;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_modification_time(
    mut info: *mut GFileInfo,
    mut result: *mut GTimeVal,
) {
    static mut safe_c2rust_attr_mtime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_usec: guint32 = 0;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_194: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_194 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_194 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_194
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_195: ::core::ffi::c_int = 0;
        if !result.is_null() {
            _g_boolean_var_195 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_195 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_195
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"result != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr_mtime == 0 as guint32 {
        safe_c2rust_attr_mtime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr());
        safe_c2rust_attr_mtime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_mtime);
    if ({
        let mut _g_boolean_var_196: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_196 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_196 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_196
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without time::modified\0" as *const u8 as *const gchar,
        );
        (*result).tv_usec = 0 as glong;
        (*result).tv_sec = (*result).tv_usec;
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1887 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return;
    }
    (*result).tv_sec = _g_file_attribute_value_get_uint64(value) as glong;
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_mtime_usec);
    (*result).tv_usec = _g_file_attribute_value_get_uint32(value) as glong;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_modification_date_time(
    mut info: *mut GFileInfo,
) -> *mut GDateTime {
    static mut safe_c2rust_attr_mtime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_usec: guint32 = 0;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut value_usec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut dt2: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_197: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_197 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_197 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_197
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if safe_c2rust_attr_mtime == 0 as guint32 {
        safe_c2rust_attr_mtime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr());
        safe_c2rust_attr_mtime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_mtime);
    if value.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    dt = g_date_time_new_from_unix_utc(_g_file_attribute_value_get_uint64(value) as gint64);
    value_usec = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_mtime_usec);
    if value_usec.is_null() {
        return safe_c2rust_g_steal_pointer(&raw mut dt as gpointer) as *mut GDateTime;
    }
    dt2 = g_date_time_add(
        dt,
        _g_file_attribute_value_get_uint32(value_usec) as GTimeSpan,
    );
    g_date_time_unref(dt);
    return safe_c2rust_g_steal_pointer(&raw mut dt2 as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_access_date_time(
    mut info: *mut GFileInfo,
) -> *mut GDateTime {
    static mut safe_c2rust_attr_atime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_atime_usec: guint32 = 0;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut value_usec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut dt2: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_198: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_198 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_198 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_198
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if safe_c2rust_attr_atime == 0 as guint32 {
        safe_c2rust_attr_atime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr());
        safe_c2rust_attr_atime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_atime);
    if value.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    dt = g_date_time_new_from_unix_utc(_g_file_attribute_value_get_uint64(value) as gint64);
    value_usec = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_atime_usec);
    if value_usec.is_null() {
        return safe_c2rust_g_steal_pointer(&raw mut dt as gpointer) as *mut GDateTime;
    }
    dt2 = g_date_time_add(
        dt,
        _g_file_attribute_value_get_uint32(value_usec) as GTimeSpan,
    );
    g_date_time_unref(dt);
    return safe_c2rust_g_steal_pointer(&raw mut dt2 as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_creation_date_time(
    mut info: *mut GFileInfo,
) -> *mut GDateTime {
    static mut safe_c2rust_attr_ctime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_ctime_usec: guint32 = 0;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut value_usec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut dt: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    let mut dt2: *mut GDateTime = ::core::ptr::null_mut::<GDateTime>();
    if ({
        let mut _g_boolean_var_199: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_199 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_199 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_199
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDateTime>();
    }
    if safe_c2rust_attr_ctime == 0 as guint32 {
        safe_c2rust_attr_ctime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED.as_ptr());
        safe_c2rust_attr_ctime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED_USEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_ctime);
    if value.is_null() {
        return ::core::ptr::null_mut::<GDateTime>();
    }
    dt = g_date_time_new_from_unix_utc(_g_file_attribute_value_get_uint64(value) as gint64);
    value_usec = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr_ctime_usec);
    if value_usec.is_null() {
        return safe_c2rust_g_steal_pointer(&raw mut dt as gpointer) as *mut GDateTime;
    }
    dt2 = g_date_time_add(
        dt,
        _g_file_attribute_value_get_uint32(value_usec) as GTimeSpan,
    );
    g_date_time_unref(dt);
    return safe_c2rust_g_steal_pointer(&raw mut dt2 as gpointer) as *mut GDateTime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_symlink_target(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_200: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_200 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_200 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_200
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::symlink-target\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_201: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_201 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_201 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_201
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::symlink-target\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2061 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_byte_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_etag(
    mut info: *mut GFileInfo,
) -> *const ::core::ffi::c_char {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_202: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_202 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_202 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_202
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"etag::value\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_203: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_203 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_203 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_203
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without etag::value\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2084 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return _g_file_attribute_value_get_string(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_get_sort_order(
    mut info: *mut GFileInfo,
) -> gint32 {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_204: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_204 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_204 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_204
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(
            b"standard::sort-order\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value = safe_c2rust_g_file_info_find_value(info, safe_c2rust_attr);
    if ({
        let mut _g_boolean_var_205: ::core::ffi::c_int = 0;
        if value.is_null() {
            _g_boolean_var_205 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_205 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_205
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"GFileInfo created without standard::sort-order\0" as *const u8 as *const gchar,
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2107 as ::core::ffi::c_int,
            G_STRFUNC,
        );
        return 0 as gint32;
    }
    return _g_file_attribute_value_get_int32(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_file_type(
    mut info: *mut GFileInfo,
    mut type_0: GFileType,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_206: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_206 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_206 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_206
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_TYPE.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(value, type_0 as guint32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_is_hidden(
    mut info: *mut GFileInfo,
    mut is_hidden: gboolean,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_207: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_207 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_207 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_207
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_HIDDEN.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_boolean(value, is_hidden);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_is_symlink(
    mut info: *mut GFileInfo,
    mut is_symlink: gboolean,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_208: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_208 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_208 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_208
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_IS_SYMLINK.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_boolean(value, is_symlink);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_name(
    mut info: *mut GFileInfo,
    mut name: *const ::core::ffi::c_char,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_209: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_209 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_209 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_209
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_210: ::core::ffi::c_int = 0;
        if !name.is_null() {
            _g_boolean_var_210 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_210 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_210
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_byte_string(value, name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_display_name(
    mut info: *mut GFileInfo,
    mut display_name: *const ::core::ffi::c_char,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_211: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_211 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_211 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_211
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_212: ::core::ffi::c_int = 0;
        if !display_name.is_null() {
            _g_boolean_var_212 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_212 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_212
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"display_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_string(value, display_name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_edit_name(
    mut info: *mut GFileInfo,
    mut edit_name: *const ::core::ffi::c_char,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_213: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_213 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_213 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_213
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_214: ::core::ffi::c_int = 0;
        if !edit_name.is_null() {
            _g_boolean_var_214 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_214 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_214
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"edit_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_string(value, edit_name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_icon(
    mut info: *mut GFileInfo,
    mut icon: *mut GIcon,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_215: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_215 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_215 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_215
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_216: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_icon_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_216 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_216 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_216
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_ICON.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_object(value, icon as *mut ::core::ffi::c_void as *mut GObject);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_symbolic_icon(
    mut info: *mut GFileInfo,
    mut icon: *mut GIcon,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_217: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_217 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_217 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_217
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_218: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_icon_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_218 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_218 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_218
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_object(value, icon as *mut ::core::ffi::c_void as *mut GObject);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_content_type(
    mut info: *mut GFileInfo,
    mut content_type: *const ::core::ffi::c_char,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_219: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_219 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_219 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_219
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_220: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
            _g_boolean_var_220 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_220 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_220
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_string(value, content_type);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_size(
    mut info: *mut GFileInfo,
    mut size: goffset,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_221: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_221 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_221 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_221
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr = safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SIZE.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, size as guint64);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_modification_time(
    mut info: *mut GFileInfo,
    mut mtime: *mut GTimeVal,
) {
    static mut safe_c2rust_attr_mtime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_usec: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_nsec: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_222: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_222 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_222 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_222
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_223: ::core::ffi::c_int = 0;
        if !mtime.is_null() {
            _g_boolean_var_223 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_223 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_223
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"mtime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr_mtime == 0 as guint32 {
        safe_c2rust_attr_mtime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr());
        safe_c2rust_attr_mtime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr());
        safe_c2rust_attr_mtime_nsec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_mtime);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, (*mtime).tv_sec as guint64);
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_mtime_usec);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(value, (*mtime).tv_usec as guint32);
    }
    safe_c2rust_g_file_info_remove_value(info, safe_c2rust_attr_mtime_nsec);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_modification_date_time(
    mut info: *mut GFileInfo,
    mut mtime: *mut GDateTime,
) {
    static mut safe_c2rust_attr_mtime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_usec: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_mtime_nsec: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_224: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_224 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_224 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_224
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_225: ::core::ffi::c_int = 0;
        if !mtime.is_null() {
            _g_boolean_var_225 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_225 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_225
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"mtime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr_mtime == 0 as guint32 {
        safe_c2rust_attr_mtime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr());
        safe_c2rust_attr_mtime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr());
        safe_c2rust_attr_mtime_nsec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_mtime);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, g_date_time_to_unix(mtime) as guint64);
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_mtime_usec);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(value, g_date_time_get_microsecond(mtime) as guint32);
    }
    safe_c2rust_g_file_info_remove_value(info, safe_c2rust_attr_mtime_nsec);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_access_date_time(
    mut info: *mut GFileInfo,
    mut atime: *mut GDateTime,
) {
    static mut safe_c2rust_attr_atime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_atime_usec: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_atime_nsec: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_226: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_226 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_226 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_226
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_227: ::core::ffi::c_int = 0;
        if !atime.is_null() {
            _g_boolean_var_227 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_227 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_227
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"atime != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr_atime == 0 as guint32 {
        safe_c2rust_attr_atime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr());
        safe_c2rust_attr_atime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr());
        safe_c2rust_attr_atime_nsec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_atime);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, g_date_time_to_unix(atime) as guint64);
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_atime_usec);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(value, g_date_time_get_microsecond(atime) as guint32);
    }
    safe_c2rust_g_file_info_remove_value(info, safe_c2rust_attr_atime_nsec);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_creation_date_time(
    mut info: *mut GFileInfo,
    mut creation_time: *mut GDateTime,
) {
    static mut safe_c2rust_attr_ctime: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_ctime_usec: guint32 = 0 as guint32;
    static mut safe_c2rust_attr_ctime_nsec: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_228: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_228 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_228 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_228
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_229: ::core::ffi::c_int = 0;
        if !creation_time.is_null() {
            _g_boolean_var_229 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_229 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_229
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"creation_time != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr_ctime == 0 as guint32 {
        safe_c2rust_attr_ctime =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED.as_ptr());
        safe_c2rust_attr_ctime_usec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED_USEC.as_ptr());
        safe_c2rust_attr_ctime_nsec =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_TIME_CREATED_NSEC.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_ctime);
    if !value.is_null() {
        _g_file_attribute_value_set_uint64(value, g_date_time_to_unix(creation_time) as guint64);
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr_ctime_usec);
    if !value.is_null() {
        _g_file_attribute_value_set_uint32(
            value,
            g_date_time_get_microsecond(creation_time) as guint32,
        );
    }
    safe_c2rust_g_file_info_remove_value(info, safe_c2rust_attr_ctime_nsec);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_symlink_target(
    mut info: *mut GFileInfo,
    mut symlink_target: *const ::core::ffi::c_char,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_230: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_230 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_230 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_230
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_231: ::core::ffi::c_int = 0;
        if !symlink_target.is_null() {
            _g_boolean_var_231 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_231 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_231
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"symlink_target != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_byte_string(value, symlink_target);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_info_set_sort_order(
    mut info: *mut GFileInfo,
    mut sort_order: gint32,
) {
    static mut safe_c2rust_attr: guint32 = 0 as guint32;
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    if ({
        let mut _g_boolean_var_232: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_file_info_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_232 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_232 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_232
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_FILE_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_attr == 0 as guint32 {
        safe_c2rust_attr =
            safe_c2rust_lookup_attribute(G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER.as_ptr());
    }
    value = safe_c2rust_g_file_info_create_value(info, safe_c2rust_attr);
    if !value.is_null() {
        _g_file_attribute_value_set_int32(value, sort_order);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_file_attribute_matcher_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_2, C2RustUnnamed_1) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GFileAttributeMatcher\0" as *const u8 as *const gchar),
        C2RustUnnamed_2 {
            do_copy_type: Some(
                safe_c2rust_g_file_attribute_matcher_ref
                    as unsafe extern "C" fn(
                        *mut GFileAttributeMatcher,
                    ) -> *mut GFileAttributeMatcher,
            ),
        },
        C2RustUnnamed_1 {
            do_free_type: Some(
                safe_c2rust_g_file_attribute_matcher_unref
                    as unsafe extern "C" fn(*mut GFileAttributeMatcher) -> (),
            ),
        },
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_compare_sub_matchers(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut suba: *const SubMatcher = a as *const SubMatcher;
    let mut subb: *const SubMatcher = b as *const SubMatcher;
    let mut diff: ::core::ffi::c_int = 0;
    diff = (*suba).id.wrapping_sub((*subb).id) as ::core::ffi::c_int;
    if diff != 0 {
        return diff as gint;
    }
    return (*suba).mask.wrapping_sub((*subb).mask) as gint;
}
unsafe extern "C" fn safe_c2rust_sub_matcher_matches(
    mut matcher: *mut SubMatcher,
    mut submatcher: *mut SubMatcher,
) -> gboolean {
    if (*matcher).mask & (*submatcher).mask != (*matcher).mask {
        return FALSE;
    }
    return ((*matcher).id == (*submatcher).id & (*matcher).mask) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_matcher_optimize(
    mut matcher: *mut GFileAttributeMatcher,
) -> *mut GFileAttributeMatcher {
    let mut submatcher: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut compare: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    if (*matcher).all != 0 {
        if !(*matcher).sub_matchers.is_null() {
            g_array_free((*matcher).sub_matchers, TRUE);
            (*matcher).sub_matchers = ::core::ptr::null_mut::<GArray>();
        }
        return matcher;
    }
    if (*(*matcher).sub_matchers).len == 0 as guint {
        safe_c2rust_g_file_attribute_matcher_unref(matcher);
        return ::core::ptr::null_mut::<GFileAttributeMatcher>();
    }
    g_array_sort(
        (*matcher).sub_matchers,
        Some(
            safe_c2rust_compare_sub_matchers
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
    );
    j = 0 as guint;
    compare = ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
        .offset(j as isize) as *mut SubMatcher;
    i = 1 as guint;
    while i < (*(*matcher).sub_matchers).len {
        submatcher = ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void
            as *mut SubMatcher)
            .offset(i as isize) as *mut SubMatcher;
        if !(safe_c2rust_sub_matcher_matches(compare, submatcher) != 0) {
            j = j.wrapping_add(1);
            compare = compare.offset(1);
            if j < i {
                *compare = *submatcher;
            }
        }
        i = i.wrapping_add(1);
    }
    g_array_set_size((*matcher).sub_matchers, j.wrapping_add(1 as guint));
    return matcher;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_new(
    mut attributes: *const ::core::ffi::c_char,
) -> *mut GFileAttributeMatcher {
    let mut split: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut colon: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut matcher: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    if attributes.is_null() || *attributes as ::core::ffi::c_int == '\0' as i32 {
        return ::core::ptr::null_mut::<GFileAttributeMatcher>();
    }
    matcher = g_malloc0(::core::mem::size_of::<GFileAttributeMatcher>() as gsize)
        as *mut GFileAttributeMatcher;
    (*matcher).ref_0 = 1 as ::core::ffi::c_int as gint;
    (*matcher).sub_matchers =
        g_array_new(FALSE, FALSE, ::core::mem::size_of::<SubMatcher>() as guint);
    split = g_strsplit(
        attributes as *const gchar,
        b",\0" as *const u8 as *const gchar,
        -(1 as gint),
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while !(*split.offset(i as isize)).is_null() {
        if strcmp(
            *split.offset(i as isize),
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*matcher).all = TRUE as gboolean;
        } else {
            let mut s: SubMatcher = SubMatcher { id: 0, mask: 0 };
            colon = strstr(
                *split.offset(i as isize),
                b"::\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !colon.is_null()
                && !(*colon.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || *colon.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '*' as i32
                        && *colon.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int)
            {
                s.id = safe_c2rust_lookup_attribute(*split.offset(i as isize));
                s.mask = 0xffffffff as ::core::ffi::c_uint as guint32;
            } else {
                if !colon.is_null() {
                    *colon = 0 as ::core::ffi::c_char;
                }
                s.id = safe_c2rust_lookup_namespace(*split.offset(i as isize)) << NS_POS;
                s.mask = NS_MASK << NS_POS;
            }
            g_array_append_vals(
                (*matcher).sub_matchers,
                &raw mut s as gconstpointer,
                1 as guint,
            );
        }
        i += 1;
    }
    g_strfreev(split as *mut *mut gchar);
    matcher = safe_c2rust_matcher_optimize(matcher);
    return matcher;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_subtract(
    mut matcher: *mut GFileAttributeMatcher,
    mut subtract: *mut GFileAttributeMatcher,
) -> *mut GFileAttributeMatcher {
    let mut result: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    let mut mi: guint = 0;
    let mut si: guint = 0;
    let mut msub: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut ssub: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    if matcher.is_null() {
        return ::core::ptr::null_mut::<GFileAttributeMatcher>();
    }
    if subtract.is_null() {
        return safe_c2rust_g_file_attribute_matcher_ref(matcher);
    }
    if (*subtract).all != 0 {
        return ::core::ptr::null_mut::<GFileAttributeMatcher>();
    }
    if (*matcher).all != 0 {
        return safe_c2rust_g_file_attribute_matcher_ref(matcher);
    }
    result = g_malloc0(::core::mem::size_of::<GFileAttributeMatcher>() as gsize)
        as *mut GFileAttributeMatcher;
    (*result).ref_0 = 1 as ::core::ffi::c_int as gint;
    (*result).sub_matchers =
        g_array_new(FALSE, FALSE, ::core::mem::size_of::<SubMatcher>() as guint);
    si = 0 as guint;
    if ({
        let mut _g_boolean_var_233: ::core::ffi::c_int = 0;
        if (*(*subtract).sub_matchers).len > 0 as guint {
            _g_boolean_var_233 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_233 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_233
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfileinfo.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2806 as ::core::ffi::c_int,
            G_STRFUNC,
            b"subtract->sub_matchers->len > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    ssub = ((*(*subtract).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
        .offset(si as isize) as *mut SubMatcher;
    mi = 0 as guint;
    's_72: while mi < (*(*matcher).sub_matchers).len {
        msub = ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
            .offset(mi as isize) as *mut SubMatcher;
        while !(safe_c2rust_sub_matcher_matches(ssub, msub) != 0) {
            si = si.wrapping_add(1);
            if si >= (*(*subtract).sub_matchers).len {
                break 's_72;
            }
            ssub = ((*(*subtract).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
                .offset(si as isize) as *mut SubMatcher;
            if (*ssub).id <= (*msub).id {
                continue;
            }
            g_array_append_vals((*result).sub_matchers, msub as gconstpointer, 1 as guint);
            break;
        }
        mi = mi.wrapping_add(1);
    }
    if mi < (*(*matcher).sub_matchers).len {
        g_array_append_vals(
            (*result).sub_matchers,
            ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
                .offset(mi as isize) as *mut SubMatcher as gconstpointer,
            (*(*matcher).sub_matchers).len.wrapping_sub(mi),
        );
    }
    result = safe_c2rust_matcher_optimize(result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_ref(
    mut matcher: *mut GFileAttributeMatcher,
) -> *mut GFileAttributeMatcher {
    if !matcher.is_null() {
        if ({
            let mut _g_boolean_var_234: ::core::ffi::c_int = 0;
            if (*matcher).ref_0 > 0 as ::core::ffi::c_int {
                _g_boolean_var_234 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_234 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_234
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"matcher->ref > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GFileAttributeMatcher>();
        }
        if 0 as ::core::ffi::c_int != 0 {
            (*matcher).ref_0;
            (*matcher).ref_0;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(&raw mut (*matcher).ref_0, 1 as ::core::ffi::c_int);
    }
    return matcher;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_unref(
    mut matcher: *mut GFileAttributeMatcher,
) {
    if !matcher.is_null() {
        if ({
            let mut _g_boolean_var_235: ::core::ffi::c_int = 0;
            if (*matcher).ref_0 > 0 as ::core::ffi::c_int {
                _g_boolean_var_235 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_235 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_235
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"matcher->ref > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if ({
            if 0 as ::core::ffi::c_int != 0 {
                (*matcher).ref_0;
                (*matcher).ref_0;
            } else {
            };
            (crate::translated::compat::atomic_xsub_seqcst(
                &raw mut (*matcher).ref_0,
                1 as ::core::ffi::c_int,
            ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        }) != 0
        {
            if !(*matcher).sub_matchers.is_null() {
                g_array_free((*matcher).sub_matchers, TRUE);
            }
            g_free(matcher as gpointer);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_matches_only(
    mut matcher: *mut GFileAttributeMatcher,
    mut attribute: *const ::core::ffi::c_char,
) -> gboolean {
    let mut sub_matcher: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut id: guint32 = 0;
    if ({
        let mut _g_boolean_var_236: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_236 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_236 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_236
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if matcher.is_null() || (*matcher).all != 0 {
        return FALSE;
    }
    if (*(*matcher).sub_matchers).len != 1 as guint {
        return FALSE;
    }
    id = safe_c2rust_lookup_attribute(attribute);
    sub_matcher = ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
        .offset(0 as ::core::ffi::c_int as isize) as *mut SubMatcher;
    return ((*sub_matcher).id == id && (*sub_matcher).mask == 0xffffffff as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_matcher_matches_id(
    mut matcher: *mut GFileAttributeMatcher,
    mut id: guint32,
) -> gboolean {
    let mut sub_matchers: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut i: guint = 0;
    if !(*matcher).sub_matchers.is_null() {
        sub_matchers = (*(*matcher).sub_matchers).data as *mut SubMatcher;
        i = 0 as guint;
        while i < (*(*matcher).sub_matchers).len {
            if (*sub_matchers.offset(i as isize)).id == id & (*sub_matchers.offset(i as isize)).mask
            {
                return TRUE;
            }
            i = i.wrapping_add(1);
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_file_attribute_matcher_matches_id(
    mut matcher: *mut GFileAttributeMatcher,
    mut id: guint32,
) -> gboolean {
    if matcher.is_null() {
        return FALSE;
    }
    if (*matcher).all != 0 {
        return TRUE;
    }
    return safe_c2rust_matcher_matches_id(matcher, id);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_matches(
    mut matcher: *mut GFileAttributeMatcher,
    mut attribute: *const ::core::ffi::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_237: ::core::ffi::c_int = 0;
        if !attribute.is_null() && *attribute as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_237 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_237 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_237
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"attribute != NULL && *attribute != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if matcher.is_null() {
        return FALSE;
    }
    if (*matcher).all != 0 {
        return TRUE;
    }
    return safe_c2rust_matcher_matches_id(matcher, safe_c2rust_lookup_attribute(attribute));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_enumerate_namespace(
    mut matcher: *mut GFileAttributeMatcher,
    mut ns: *const ::core::ffi::c_char,
) -> gboolean {
    let mut sub_matchers: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    let mut ns_id: guint = 0;
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_238: ::core::ffi::c_int = 0;
        if !ns.is_null() && *ns as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_238 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_238 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_238
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"ns != NULL && *ns != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if matcher.is_null() {
        return FALSE;
    }
    if (*matcher).all != 0 {
        return TRUE;
    }
    ns_id = (safe_c2rust_lookup_namespace(ns) << NS_POS) as guint;
    if !(*matcher).sub_matchers.is_null() {
        sub_matchers = (*(*matcher).sub_matchers).data as *mut SubMatcher;
        i = 0 as guint;
        while i < (*(*matcher).sub_matchers).len {
            if (*sub_matchers.offset(i as isize)).id == ns_id {
                return TRUE;
            }
            i = i.wrapping_add(1);
        }
    }
    (*matcher).iterator_ns = ns_id as guint32;
    (*matcher).iterator_pos = 0 as ::core::ffi::c_int as gint;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_enumerate_next(
    mut matcher: *mut GFileAttributeMatcher,
) -> *const ::core::ffi::c_char {
    let mut i: guint = 0;
    let mut sub_matcher: *mut SubMatcher = ::core::ptr::null_mut::<SubMatcher>();
    if matcher.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    loop {
        let fresh7 = (*matcher).iterator_pos;
        (*matcher).iterator_pos = (*matcher).iterator_pos + 1;
        i = fresh7 as guint;
        if (*matcher).sub_matchers.is_null() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        if i < (*(*matcher).sub_matchers).len {
            sub_matcher = ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void
                as *mut SubMatcher)
                .offset(i as isize) as *mut SubMatcher;
        } else {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        if (*sub_matcher).mask == 0xffffffff as ::core::ffi::c_uint
            && (*sub_matcher).id & NS_MASK << NS_POS == (*matcher).iterator_ns
        {
            return safe_c2rust_get_attribute_for_id((*sub_matcher).id as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_to_string(
    mut matcher: *mut GFileAttributeMatcher,
) -> *mut ::core::ffi::c_char {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: guint = 0;
    if matcher.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*matcher).all != 0 {
        return safe_c2rust_g_strdup_inline(b"*\0" as *const u8 as *const ::core::ffi::c_char);
    }
    string = g_string_new(b"\0" as *const u8 as *const gchar);
    i = 0 as guint;
    while i < (*(*matcher).sub_matchers).len {
        let mut submatcher: *mut SubMatcher =
            ((*(*matcher).sub_matchers).data as *mut ::core::ffi::c_void as *mut SubMatcher)
                .offset(i as isize) as *mut SubMatcher;
        if i > 0 as guint {
            safe_c2rust_g_string_append_c_inline(string, ',' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    safe_c2rust_get_attribute_for_id((*submatcher).id as ::core::ffi::c_int);
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_239: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_239 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_239 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_239
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
                safe_c2rust_get_attribute_for_id((*submatcher).id as ::core::ffi::c_int),
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        i = i.wrapping_add(1);
    }
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
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
