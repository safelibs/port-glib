// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_datalist_clear(datalist: *mut *mut GData);
    fn g_datalist_id_get_data(datalist: *mut *mut GData, key_id: GQuark) -> gpointer;
    fn g_datalist_id_set_data_full(
        datalist: *mut *mut GData,
        key_id: GQuark,
        data: gpointer,
        destroy_func: GDestroyNotify,
    );
    fn g_datalist_id_remove_no_notify(datalist: *mut *mut GData, key_id: GQuark) -> gpointer;
    fn g_datalist_set_flags(datalist: *mut *mut GData, flags: guint);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_foreach(hash_table: *mut GHashTable, func: GHFunc, user_data: gpointer);
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_sort(list: *mut GSList, compare_func: GCompareFunc) -> *mut GSList;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_from_name(name: *const gchar) -> GType;
    fn g_type_parent(type_0: GType) -> GType;
    fn g_type_depth(type_0: GType) -> guint;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_register_static(
        parent_type: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_fundamental(
        type_id: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        finfo: *const GTypeFundamentalInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_create_instance(type_0: GType) -> *mut GTypeInstance;
    fn g_type_free_instance(instance: *mut GTypeInstance);
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_type_check_value(value: *const GValue) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_reset(value: *mut GValue) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_type_compatible(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_transform(src_value: *const GValue, dest_value: *mut GValue) -> gboolean;
    fn g_value_register_transform_func(
        src_type: GType,
        dest_type: GType,
        transform_func: GValueTransform,
    );
    static mut g_param_spec_types: *mut GType;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub union _GTypeCValue {
    pub v_int: gint,
    pub v_long: glong,
    pub v_int64: gint64,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GTypeCValue = _GTypeCValue;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInfo {
    pub class_size: guint16,
    pub base_init: GBaseInitFunc,
    pub base_finalize: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: GInstanceInitFunc,
    pub value_table: *const GTypeValueTable,
}
pub type GTypeValueTable = _GTypeValueTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeValueTable {
    pub value_init: GTypeValueInitFunc,
    pub value_free: GTypeValueFreeFunc,
    pub value_copy: GTypeValueCopyFunc,
    pub value_peek_pointer: GTypeValuePeekPointerFunc,
    pub collect_format: *const gchar,
    pub collect_value: GTypeValueCollectFunc,
    pub lcopy_format: *const gchar,
    pub lcopy_value: GTypeValueLCopyFunc,
}
pub type GTypeValueLCopyFunc =
    Option<unsafe extern "C" fn(*const GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValueCollectFunc =
    Option<unsafe extern "C" fn(*mut GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValuePeekPointerFunc = Option<unsafe extern "C" fn(*const GValue) -> gpointer>;
pub type GTypeValueCopyFunc = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
pub type GTypeValueFreeFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GTypeValueInitFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GBaseFinalizeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBaseInitFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GTypeInfo = _GTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeFundamentalInfo {
    pub type_flags: GTypeFundamentalFlags,
}
pub type GTypeFundamentalFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEEP_DERIVABLE: GTypeFundamentalFlags = 8;
pub const G_TYPE_FLAG_DERIVABLE: GTypeFundamentalFlags = 4;
pub const G_TYPE_FLAG_INSTANTIATABLE: GTypeFundamentalFlags = 2;
pub const G_TYPE_FLAG_CLASSED: GTypeFundamentalFlags = 1;
pub type GTypeFundamentalInfo = _GTypeFundamentalInfo;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GValueTransform = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecClass {
    pub g_type_class: GTypeClass,
    pub value_type: GType,
    pub finalize: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_set_default: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>,
    pub value_validate: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>,
    pub values_cmp:
        Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>,
    pub value_is_valid: Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>,
    pub dummy: [gpointer; 3],
}
pub type GParamSpecClass = _GParamSpecClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecPool {
    pub mutex: GMutex,
    pub type_prefixing: gboolean,
    pub hash_table: *mut GHashTable,
}
pub type GParamSpecPool = _GParamSpecPool;
pub type GParamSpecOverride = _GParamSpecOverride;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecOverride {
    pub parent_instance: GParamSpec,
    pub overridden: *mut GParamSpec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GParamSpecPrivate {
    pub default_value: GValue,
    pub name_quark: GQuark,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecTypeInfo {
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_type: GType,
    pub finalize: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_set_default: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>,
    pub value_validate: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>,
    pub values_cmp:
        Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>,
}
pub type GParamSpecTypeInfo = _GParamSpecTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamSpecClassInfo {
    pub value_type: GType,
    pub finalize: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_set_default: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>,
    pub value_validate: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>,
    pub values_cmp:
        Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>,
}
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
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
static mut g_param_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn g_param_spec_get_private(
    mut pspec: *mut GParamSpec,
) -> *mut GParamSpecPrivate {
    return (pspec as *mut guint8).offset(g_param_private_offset as glong as isize) as gpointer
        as *mut GParamSpecPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn _g_param_type_init() {
    static mut finfo: GTypeFundamentalInfo = _GTypeFundamentalInfo {
        type_flags: (G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
            | G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DEEP_DERIVABLE as ::core::ffi::c_int)
            as GTypeFundamentalFlags,
    };
    static mut param_value_table: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(value_param_init as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: Some(value_param_free_value as unsafe extern "C" fn(*mut GValue) -> ()),
            value_copy: Some(
                value_param_copy_value as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                value_param_peek_pointer as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                value_param_collect_value
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                value_param_lcopy_value
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    let param_spec_info: GTypeInfo = _GTypeInfo {
        class_size: ::core::mem::size_of::<GParamSpecClass>() as guint16,
        base_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GParamSpecClass) -> ()>,
            GBaseInitFunc,
        >(Some(
            g_param_spec_class_base_init as unsafe extern "C" fn(*mut GParamSpecClass) -> (),
        )),
        base_finalize: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GParamSpecClass) -> ()>,
            GBaseFinalizeFunc,
        >(Some(
            g_param_spec_class_base_finalize as unsafe extern "C" fn(*mut GParamSpecClass) -> (),
        )),
        class_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GParamSpecClass, gpointer) -> ()>,
            GClassInitFunc,
        >(Some(
            g_param_spec_class_init as unsafe extern "C" fn(*mut GParamSpecClass, gpointer) -> (),
        )),
        class_finalize: ::core::mem::transmute::<*mut ::core::ffi::c_void, GClassFinalizeFunc>(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ),
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: ::core::mem::size_of::<GParamSpec>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GParamSpec, *mut GParamSpecClass) -> ()>,
            GInstanceInitFunc,
        >(Some(
            g_param_spec_init as unsafe extern "C" fn(*mut GParamSpec, *mut GParamSpecClass) -> (),
        )),
        value_table: &raw const param_value_table,
    };
    let mut type_0: GType = 0;
    type_0 = g_type_register_fundamental(
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GParam\0" as *const u8 as *const gchar),
        &raw const param_spec_info,
        &raw const finfo,
        G_TYPE_FLAG_ABSTRACT,
    );
    if type_0 == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparam.c\0" as *const u8
                as *const ::core::ffi::c_char,
            141 as ::core::ffi::c_int,
            b"_g_param_type_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_param_private_offset =
        g_type_add_instance_private(type_0, ::core::mem::size_of::<GParamSpecPrivate>() as gsize);
    g_value_register_transform_func(
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(value_param_transform_value as unsafe extern "C" fn(*const GValue, *mut GValue) -> ()),
    );
}
unsafe extern "C" fn g_param_spec_class_base_init(mut class: *mut GParamSpecClass) {}
unsafe extern "C" fn g_param_spec_class_base_finalize(mut class: *mut GParamSpecClass) {}
unsafe extern "C" fn g_param_spec_class_init(
    mut class: *mut GParamSpecClass,
    mut class_data: gpointer,
) {
    (*class).value_type = ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    (*class).finalize = Some(g_param_spec_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ())
        as Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>;
    (*class).value_set_default = None;
    (*class).value_validate = None;
    (*class).values_cmp = None;
    g_type_class_adjust_private_offset(class as gpointer, &raw mut g_param_private_offset);
}
unsafe extern "C" fn g_param_spec_init(
    mut pspec: *mut GParamSpec,
    mut class: *mut GParamSpecClass,
) {
    (*pspec).name = ::core::ptr::null::<gchar>();
    (*pspec)._nick = ::core::ptr::null_mut::<gchar>();
    (*pspec)._blurb = ::core::ptr::null_mut::<gchar>();
    (*pspec).flags = 0 as GParamFlags;
    (*pspec).value_type = (*class).value_type;
    (*pspec).owner_type = 0 as GType;
    (*pspec).qdata = ::core::ptr::null_mut::<GData>();
    g_datalist_set_flags(&raw mut (*pspec).qdata, 0x2 as guint);
    (*pspec).ref_count = 1 as guint;
    (*pspec).param_id = 0 as guint;
}
unsafe extern "C" fn g_param_spec_finalize(mut pspec: *mut GParamSpec) {
    let mut priv_0: *mut GParamSpecPrivate = g_param_spec_get_private(pspec);
    if (*priv_0).default_value.g_type != 0 {
        g_value_reset(&raw mut (*priv_0).default_value);
    }
    g_datalist_clear(&raw mut (*pspec).qdata);
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_STATIC_NICK as ::core::ffi::c_int == 0 {
        g_free((*pspec)._nick as gpointer);
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_STATIC_BLURB as ::core::ffi::c_int == 0 {
        g_free((*pspec)._blurb as gpointer);
    }
    g_type_free_instance(pspec as *mut GTypeInstance);
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_ref(mut pspec: *mut GParamSpec) -> *mut GParamSpec {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        *(&raw mut (*pspec).ref_count as *mut ::core::ffi::c_int);
        *(&raw mut (*pspec).ref_count as *mut ::core::ffi::c_int);
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut (*pspec).ref_count as *mut ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_unref(mut pspec: *mut GParamSpec) {
    let mut is_zero: gboolean = 0;
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    is_zero = ({
        if 0 as ::core::ffi::c_int != 0 {
            *(&raw mut (*pspec).ref_count as *mut ::core::ffi::c_int);
            *(&raw mut (*pspec).ref_count as *mut ::core::ffi::c_int);
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*pspec).ref_count as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) as gboolean;
    if is_zero != 0 {
        (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
            .finalize
            .expect("non-null function pointer")(pspec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_sink(mut pspec: *mut GParamSpec) {
    let mut oldvalue: guintptr = 0;
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    oldvalue = ({
        let mut gapa_atomic: *mut guintptr = &raw mut (*pspec).qdata as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            (*pspec).qdata;
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
            !(0x2 as ::core::ffi::c_int as gsize);
            !(0x2 as ::core::ffi::c_int as gsize);
        } else {
        };
        crate::translated::compat::atomic_and_seqcst(gapa_atomic, !(0x2 as ::core::ffi::c_int as gsize))
    });
    if oldvalue & 0x2 as guintptr != 0 {
        g_param_spec_unref(pspec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_ref_sink(mut pspec: *mut GParamSpec) -> *mut GParamSpec {
    let mut oldvalue: guintptr = 0;
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_ref_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    oldvalue = ({
        let mut gapa_atomic: *mut guintptr = &raw mut (*pspec).qdata as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            (*pspec).qdata;
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
            !(0x2 as ::core::ffi::c_int as gsize);
            !(0x2 as ::core::ffi::c_int as gsize);
        } else {
        };
        crate::translated::compat::atomic_and_seqcst(gapa_atomic, !(0x2 as ::core::ffi::c_int as gsize))
    });
    if oldvalue & 0x2 as guintptr == 0 {
        g_param_spec_ref(pspec);
    }
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_name(mut pspec: *mut GParamSpec) -> *const gchar {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_get_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*pspec).name;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_nick(mut pspec: *mut GParamSpec) -> *const gchar {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_get_nick\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*pspec)._nick.is_null() {
        return (*pspec)._nick;
    } else {
        let mut redirect_target: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        redirect_target = g_param_spec_get_redirect_target(pspec);
        if !redirect_target.is_null() && !(*redirect_target)._nick.is_null() {
            return (*redirect_target)._nick;
        }
    }
    return (*pspec).name;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_blurb(mut pspec: *mut GParamSpec) -> *const gchar {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_get_blurb\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*pspec)._blurb.is_null() {
        return (*pspec)._blurb;
    } else {
        let mut redirect_target: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        redirect_target = g_param_spec_get_redirect_target(pspec);
        if !redirect_target.is_null() && !(*redirect_target)._blurb.is_null() {
            return (*redirect_target)._blurb;
        }
    }
    return ::core::ptr::null::<gchar>();
}
unsafe extern "C" fn canonicalize_key(mut key: *mut gchar) {
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    p = key;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut c: gchar = *p;
        if c as ::core::ffi::c_int == '_' as i32 {
            *p = '-' as i32 as gchar;
        }
        p = p.offset(1);
    }
}
unsafe extern "C" fn is_canonical(mut key: *const gchar) -> gboolean {
    return (strchr(key as *const ::core::ffi::c_char, '_' as i32)
        == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_is_valid_name(mut name: *const gchar) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ((*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) < 'A' as i32
        || *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int > 'Z' as i32)
        && ((*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) < 'a' as i32
            || *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int > 'z' as i32)
    {
        return 0 as gboolean;
    }
    p = name;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let c: gchar = *p;
        if c as ::core::ffi::c_int != '-' as i32
            && c as ::core::ffi::c_int != '_' as i32
            && ((c as ::core::ffi::c_int) < '0' as i32 || c as ::core::ffi::c_int > '9' as i32)
            && ((c as ::core::ffi::c_int) < 'A' as i32 || c as ::core::ffi::c_int > 'Z' as i32)
            && ((c as ::core::ffi::c_int) < 'a' as i32 || c as ::core::ffi::c_int > 'z' as i32)
        {
            return 0 as gboolean;
        }
        p = p.offset(1);
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_internal(
    mut param_type: GType,
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut flags: GParamFlags,
) -> gpointer {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut priv_0: *mut GParamSpecPrivate = ::core::ptr::null_mut::<GParamSpecPrivate>();
    if g_type_fundamental(param_type)
        == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && param_type != ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_internal\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_PARAM (param_type) && param_type != G_TYPE_PARAM\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_internal\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if g_param_spec_is_valid_name(name) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_internal\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_is_valid_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if flags as ::core::ffi::c_int & G_PARAM_STATIC_NAME as ::core::ffi::c_int == 0
        || is_canonical(name) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_internal\0" as *const u8 as *const ::core::ffi::c_char,
            b"!(flags & G_PARAM_STATIC_NAME) || is_canonical (name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    pspec = g_type_create_instance(param_type) as gpointer as *mut GParamSpec;
    if flags as ::core::ffi::c_int & G_PARAM_STATIC_NAME as ::core::ffi::c_int != 0 {
        (*pspec).name = g_intern_static_string(name) as *mut gchar;
        if is_canonical((*pspec).name) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"G_PARAM_STATIC_NAME used with non-canonical pspec name: %s\0" as *const u8
                    as *const gchar,
                (*pspec).name,
            );
        }
    } else if is_canonical(name) != 0 {
        (*pspec).name = g_intern_string(name) as *mut gchar;
    } else {
        let mut tmp: *mut gchar = g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
        canonicalize_key(tmp);
        (*pspec).name = g_intern_string(tmp) as *mut gchar;
        g_free(tmp as gpointer);
    }
    priv_0 = g_param_spec_get_private(pspec);
    (*priv_0).name_quark = g_quark_from_string((*pspec).name);
    if flags as ::core::ffi::c_int & G_PARAM_STATIC_NICK as ::core::ffi::c_int != 0 {
        (*pspec)._nick = nick as *mut gchar;
    } else {
        (*pspec)._nick = g_strdup_inline(nick as *const ::core::ffi::c_char) as *mut gchar;
    }
    if flags as ::core::ffi::c_int & G_PARAM_STATIC_BLURB as ::core::ffi::c_int != 0 {
        (*pspec)._blurb = blurb as *mut gchar;
    } else {
        (*pspec)._blurb = g_strdup_inline(blurb as *const ::core::ffi::c_char) as *mut gchar;
    }
    (*pspec).flags = (flags as ::core::ffi::c_uint
        & !(0 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
        | (flags as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as GParamFlags;
    return pspec as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_qdata(
    mut pspec: *mut GParamSpec,
    mut quark: GQuark,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_get_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if quark != 0 {
        g_datalist_id_get_data(&raw mut (*pspec).qdata, quark)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_set_qdata(
    mut pspec: *mut GParamSpec,
    mut quark: GQuark,
    mut data: gpointer,
) {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(&raw mut (*pspec).qdata, quark, data, None);
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_set_qdata_full(
    mut pspec: *mut GParamSpec,
    mut quark: GQuark,
    mut data: gpointer,
    mut destroy: GDestroyNotify,
) {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_set_qdata_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_set_qdata_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(
        &raw mut (*pspec).qdata,
        quark,
        data,
        if !data.is_null() {
            destroy
        } else {
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GDestroyNotify>(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            )
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_steal_qdata(
    mut pspec: *mut GParamSpec,
    mut quark: GQuark,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_steal_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_steal_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_datalist_id_remove_no_notify(&raw mut (*pspec).qdata, quark);
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_redirect_target(
    mut pspec: *mut GParamSpec,
) -> *mut GParamSpec {
    let mut inst: *mut GTypeInstance = pspec as *mut GTypeInstance;
    if !inst.is_null()
        && !(*inst).g_class.is_null()
        && (*(*inst).g_class).g_type
            == *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize)
    {
        return (*(pspec as *mut GParamSpecOverride)).overridden;
    } else {
        return ::core::ptr::null_mut::<GParamSpec>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_value_set_default(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_set_default\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value).g_type == ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
        g_value_init(
            value,
            (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
        );
    } else {
        if g_type_check_value(value) != 0 {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_value_set_default\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_VALUE (value)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if ({
            let mut __val: *const GValue = value as *const GValue;
            let mut __t: GType =
                (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
            let mut __r: gboolean = 0;
            if __val.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if (*__val).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_value_holds(__val, __t);
            }
            __r
        }) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_value_set_default\0" as *const u8 as *const ::core::ffi::c_char,
                b"PSPEC_APPLIES_TO_VALUE (pspec, value)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        g_value_reset(value);
    }
    (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
        .value_set_default
        .expect("non-null function pointer")(pspec, value);
}
#[no_mangle]
pub unsafe extern "C" fn g_param_value_defaults(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut dflt_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut defaults: gboolean = 0;
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_defaults\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_type_check_value(value as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_defaults\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_defaults\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_value_init(
        &raw mut dflt_value,
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
    (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
        .value_set_default
        .expect("non-null function pointer")(pspec, &raw mut dflt_value);
    defaults = ((*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
        .values_cmp
        .expect("non-null function pointer")(pspec, value, &raw mut dflt_value)
        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    g_value_unset(&raw mut dflt_value);
    return defaults;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_value_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_validate\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_type_check_value(value) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_validate\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_validate\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
        .value_validate
        .is_some()
    {
        let mut oval: GValue = *value;
        if (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
            .value_validate
            .expect("non-null function pointer")(pspec, value)
            != 0
            || memcmp(
                &raw mut oval.data as *const ::core::ffi::c_void,
                &raw mut (*value).data as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[C2RustUnnamed; 2]>() as size_t,
            ) != 0
        {
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
    }
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_value_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut class: *mut GParamSpecClass = ::core::ptr::null_mut::<GParamSpecClass>();
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_is_valid\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if g_type_check_value(value as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_is_valid\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_is_valid\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    class = (*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass;
    if (*class).value_is_valid.is_some() {
        return (*class).value_is_valid.expect("non-null function pointer")(pspec, value);
    } else if (*class).value_validate.is_some() {
        let mut val: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        let mut changed: gboolean = 0;
        g_value_init(&raw mut val, (*(value as *mut GValue)).g_type);
        g_value_copy(value, &raw mut val);
        changed = (*class).value_validate.expect("non-null function pointer")(pspec, &raw mut val);
        g_value_unset(&raw mut val);
        return (changed == 0) as ::core::ffi::c_int;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_value_convert(
    mut pspec: *mut GParamSpec,
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
    mut strict_validation: gboolean,
) -> gboolean {
    let mut tmp_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_convert\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_type_check_value(src_value as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_convert\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (src_value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_type_check_value(dest_value) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_convert\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (dest_value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut __val: *const GValue = dest_value as *const GValue;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_value_convert\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, dest_value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_value_init(&raw mut tmp_value, (*dest_value).g_type);
    if g_value_transform(src_value, &raw mut tmp_value) != 0
        && (g_param_value_validate(pspec, &raw mut tmp_value) == 0 || strict_validation == 0)
    {
        g_value_unset(dest_value);
        memcpy(
            dest_value as *mut ::core::ffi::c_void,
            &raw mut tmp_value as *const ::core::ffi::c_void,
            ::core::mem::size_of::<GValue>() as size_t,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    } else {
        g_value_unset(&raw mut tmp_value);
        return 0 as gboolean;
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut cmp: gint = 0;
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_values_cmp\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if g_type_check_value(value1 as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_values_cmp\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if g_type_check_value(value2 as *mut GValue) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_values_cmp\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_VALUE (value2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut __val: *const GValue = value1;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_values_cmp\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, value1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut __val: *const GValue = value2;
        let mut __t: GType = (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_values_cmp\0" as *const u8 as *const ::core::ffi::c_char,
            b"PSPEC_APPLIES_TO_VALUE (pspec, value2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    cmp = (*((*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass))
        .values_cmp
        .expect("non-null function pointer")(pspec, value1, value2);
    return if cmp > 1 as ::core::ffi::c_int {
        1 as gint
    } else if cmp < -(1 as ::core::ffi::c_int) {
        -(1 as gint)
    } else {
        cmp
    };
}
unsafe extern "C" fn value_param_init(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn value_param_free_value(mut value: *mut GValue) {
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_param_spec_unref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        );
    }
}
unsafe extern "C" fn value_param_copy_value(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    if !(*src_value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_param_spec_ref(
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        ) as gpointer;
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    };
}
unsafe extern "C" fn value_param_transform_value(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    if !(*src_value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && ((*(*((*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer
            as *mut GTypeInstance))
            .g_class)
            .g_type
            == (*dest_value).g_type
            || g_type_is_a(
                (*(*((*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer
                    as *mut GTypeInstance))
                    .g_class)
                    .g_type,
                (*dest_value).g_type,
            ) != 0)
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_param_spec_ref(
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        ) as gpointer;
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    };
}
unsafe extern "C" fn value_param_peek_pointer(mut value: *const GValue) -> gpointer {
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
unsafe extern "C" fn value_param_collect_value(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if !(*collect_values.offset(0 as ::core::ffi::c_int as isize))
        .v_pointer
        .is_null()
    {
        let mut param: *mut GParamSpec =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut GParamSpec;
        if (*param).g_type_instance.g_class.is_null() {
            return g_strconcat(
                b"invalid unclassed param spec pointer for value type '\0" as *const u8
                    as *const gchar,
                g_type_name((*value).g_type),
                b"'\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else if g_value_type_compatible(
            (*(*(param as *mut GTypeInstance)).g_class).g_type,
            (*value).g_type,
        ) == 0
        {
            return g_strconcat(
                b"invalid param spec type '\0" as *const u8 as *const gchar,
                g_type_name((*(*(param as *mut GTypeInstance)).g_class).g_type),
                b"' for value type '\0" as *const u8 as *const ::core::ffi::c_char,
                g_type_name((*value).g_type),
                b"'\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_param_spec_ref(param) as gpointer;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn value_param_lcopy_value(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut param_p: *mut *mut GParamSpec = (*collect_values
        .offset(0 as ::core::ffi::c_int as isize))
    .v_pointer as *mut *mut GParamSpec;
    if !param_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_param_lcopy_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"param_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        *param_p = ::core::ptr::null_mut::<GParamSpec>();
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        *param_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec;
    } else {
        *param_p = g_param_spec_ref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        );
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn param_spec_pool_hash(mut key_spec: gconstpointer) -> guint {
    let mut key: *const GParamSpec = key_spec as *const GParamSpec;
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut h: guint = (*key).owner_type as guint;
    p = (*key).name;
    while *p != 0 {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_sub(h)
            .wrapping_add(*p as guint);
        p = p.offset(1);
    }
    return h;
}
unsafe extern "C" fn param_spec_pool_equals(
    mut key_spec_1: gconstpointer,
    mut key_spec_2: gconstpointer,
) -> gboolean {
    let mut key1: *const GParamSpec = key_spec_1 as *const GParamSpec;
    let mut key2: *const GParamSpec = key_spec_2 as *const GParamSpec;
    return ((*key1).owner_type == (*key2).owner_type
        && ((*key1).name == (*key2).name
            || strcmp(
                (*key1).name as *const ::core::ffi::c_char,
                (*key2).name as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_new(
    mut type_prefixing: gboolean,
) -> *mut GParamSpecPool {
    static mut init_mutex: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    let mut pool: *mut GParamSpecPool = g_malloc_n(
        1 as gsize,
        ::core::mem::size_of::<GParamSpecPool>() as gsize,
    ) as *mut GParamSpecPool;
    memcpy(
        &raw mut (*pool).mutex as *mut ::core::ffi::c_void,
        &raw mut init_mutex as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GMutex>() as size_t,
    );
    (*pool).type_prefixing =
        (type_prefixing != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    (*pool).hash_table = g_hash_table_new_full(
        Some(param_spec_pool_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(
            param_spec_pool_equals
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
        ),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>, GDestroyNotify>(
            Some(g_param_spec_unref as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        ),
        None,
    );
    return pool;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_free(mut pool: *mut GParamSpecPool) {
    g_mutex_lock(&raw mut (*pool).mutex);
    g_hash_table_unref((*pool).hash_table);
    g_mutex_unlock(&raw mut (*pool).mutex);
    g_mutex_clear(&raw mut (*pool).mutex);
    g_free(pool as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_insert(
    mut pool: *mut GParamSpecPool,
    mut pspec: *mut GParamSpec,
    mut owner_type: GType,
) {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if !pool.is_null()
        && !pspec.is_null()
        && owner_type > 0 as GType
        && (*pspec).owner_type == 0 as GType
    {
        p = (*pspec).name;
        while *p != 0 {
            if strchr(
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_\0" as *const u8
                    as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_int,
            )
            .is_null()
            {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparam.c:1044: pspec name \"%s\" contains invalid characters\0"
                        as *const u8 as *const gchar,
                    (*pspec).name,
                );
                return;
            }
            p = p.offset(1);
        }
        g_mutex_lock(&raw mut (*pool).mutex);
        (*pspec).owner_type = owner_type;
        g_param_spec_ref(pspec);
        g_hash_table_add((*pool).hash_table, pspec as gpointer);
        g_mutex_unlock(&raw mut (*pool).mutex);
    } else {
        if !pool.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_insert\0" as *const u8 as *const ::core::ffi::c_char,
                b"pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if !pspec.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_insert\0" as *const u8 as *const ::core::ffi::c_char,
                b"pspec\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if owner_type > 0 as GType {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_insert\0" as *const u8 as *const ::core::ffi::c_char,
                b"owner_type > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if (*pspec).owner_type == 0 as GType {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_insert\0" as *const u8 as *const ::core::ffi::c_char,
                b"pspec->owner_type == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_remove(
    mut pool: *mut GParamSpecPool,
    mut pspec: *mut GParamSpec,
) {
    if !pool.is_null() && !pspec.is_null() {
        g_mutex_lock(&raw mut (*pool).mutex);
        if g_hash_table_remove((*pool).hash_table, pspec as gconstpointer) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparam.c:1078: attempt to remove unknown pspec '%s' from pool\0"
                    as *const u8 as *const gchar,
                (*pspec).name,
            );
        }
        g_mutex_unlock(&raw mut (*pool).mutex);
    } else {
        if !pool.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_remove\0" as *const u8 as *const ::core::ffi::c_char,
                b"pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if !pspec.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_param_spec_pool_remove\0" as *const u8 as *const ::core::ffi::c_char,
                b"pspec\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    };
}
#[inline]
unsafe extern "C" fn param_spec_ht_lookup(
    mut hash_table: *mut GHashTable,
    mut param_name: *const gchar,
    mut owner_type: GType,
    mut walk_ancestors: gboolean,
) -> *mut GParamSpec {
    let mut key: GParamSpec = _GParamSpec {
        g_type_instance: _GTypeInstance {
            g_class: ::core::ptr::null_mut::<GTypeClass>(),
        },
        name: ::core::ptr::null::<gchar>(),
        flags: 0 as GParamFlags,
        value_type: 0,
        owner_type: 0,
        _nick: ::core::ptr::null_mut::<gchar>(),
        _blurb: ::core::ptr::null_mut::<gchar>(),
        qdata: ::core::ptr::null_mut::<GData>(),
        ref_count: 0,
        param_id: 0,
    };
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    key.owner_type = owner_type;
    key.name = param_name as *mut gchar;
    if walk_ancestors != 0 {
        loop {
            pspec =
                g_hash_table_lookup(hash_table, &raw mut key as gconstpointer) as *mut GParamSpec;
            if !pspec.is_null() {
                return pspec;
            }
            key.owner_type = g_type_parent(key.owner_type);
            if !(key.owner_type != 0) {
                break;
            }
        }
    } else {
        pspec = g_hash_table_lookup(hash_table, &raw mut key as gconstpointer) as *mut GParamSpec;
    }
    if pspec.is_null() && is_canonical(param_name) == 0 {
        let mut canonical: *mut gchar = ::core::ptr::null_mut::<gchar>();
        canonical = g_strdup_inline(key.name as *const ::core::ffi::c_char) as *mut gchar;
        canonicalize_key(canonical);
        key.name = canonical;
        key.owner_type = owner_type;
        if walk_ancestors != 0 {
            loop {
                pspec = g_hash_table_lookup(hash_table, &raw mut key as gconstpointer)
                    as *mut GParamSpec;
                if !pspec.is_null() {
                    g_free(canonical as gpointer);
                    return pspec;
                }
                key.owner_type = g_type_parent(key.owner_type);
                if !(key.owner_type != 0) {
                    break;
                }
            }
        } else {
            pspec =
                g_hash_table_lookup(hash_table, &raw mut key as gconstpointer) as *mut GParamSpec;
        }
        g_free(canonical as gpointer);
    }
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_lookup(
    mut pool: *mut GParamSpecPool,
    mut param_name: *const gchar,
    mut owner_type: GType,
    mut walk_ancestors: gboolean,
) -> *mut GParamSpec {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if !pool.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_lookup\0" as *const u8 as *const ::core::ffi::c_char,
            b"pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if !param_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_lookup\0" as *const u8 as *const ::core::ffi::c_char,
            b"param_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    g_mutex_lock(&raw mut (*pool).mutex);
    pspec = param_spec_ht_lookup((*pool).hash_table, param_name, owner_type, walk_ancestors);
    if !pspec.is_null() {
        g_mutex_unlock(&raw mut (*pool).mutex);
        return pspec;
    }
    if (*pool).type_prefixing != 0 {
        let mut delim: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        delim = strchr(param_name as *const ::core::ffi::c_char, ':' as i32);
        if !delim.is_null()
            && *delim.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
        {
            let mut l: guint = delim.offset_from(param_name) as ::core::ffi::c_long as guint;
            let mut stack_buffer: [gchar; 32] = [0; 32];
            let mut buffer: *mut gchar = if l < 32 as guint {
                &raw mut stack_buffer as *mut gchar
            } else {
                g_malloc_n(
                    l.wrapping_add(1 as guint) as gsize,
                    ::core::mem::size_of::<gchar>() as gsize,
                ) as *mut gchar
            };
            let mut type_0: GType = 0;
            strncpy(
                buffer as *mut ::core::ffi::c_char,
                param_name as *const ::core::ffi::c_char,
                delim.offset_from(param_name) as ::core::ffi::c_long as size_t,
            );
            *buffer.offset(l as isize) = 0 as gchar;
            type_0 = g_type_from_name(buffer);
            if l >= 32 as guint {
                g_free(buffer as gpointer);
            }
            if type_0 != 0 {
                if walk_ancestors == 0 && type_0 != owner_type
                    || !(owner_type == type_0 || g_type_is_a(owner_type, type_0) != 0)
                {
                    g_mutex_unlock(&raw mut (*pool).mutex);
                    return ::core::ptr::null_mut::<GParamSpec>();
                }
                owner_type = type_0;
                param_name = param_name.offset(l.wrapping_add(2 as guint) as isize);
                pspec = param_spec_ht_lookup(
                    (*pool).hash_table,
                    param_name,
                    owner_type,
                    walk_ancestors,
                );
                g_mutex_unlock(&raw mut (*pool).mutex);
                return pspec;
            }
        }
    }
    g_mutex_unlock(&raw mut (*pool).mutex);
    return ::core::ptr::null_mut::<GParamSpec>();
}
unsafe extern "C" fn pool_list(mut key: gpointer, mut value: gpointer, mut user_data: gpointer) {
    let mut pspec: *mut GParamSpec = value as *mut GParamSpec;
    let mut data: *mut gpointer = user_data as *mut gpointer;
    let mut owner_type: GType = *data.offset(1 as ::core::ffi::c_int as isize) as GType;
    if owner_type == (*pspec).owner_type {
        let ref mut fresh0 = *data.offset(0 as ::core::ffi::c_int as isize);
        *fresh0 = g_list_prepend(
            *data.offset(0 as ::core::ffi::c_int as isize) as *mut GList,
            pspec as gpointer,
        ) as gpointer;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_list_owned(
    mut pool: *mut GParamSpecPool,
    mut owner_type: GType,
) -> *mut GList {
    let mut data: [gpointer; 2] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 2];
    if !pool.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_list_owned\0" as *const u8 as *const ::core::ffi::c_char,
            b"pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if owner_type > 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_list_owned\0" as *const u8 as *const ::core::ffi::c_char,
            b"owner_type > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    g_mutex_lock(&raw mut (*pool).mutex);
    data[0 as ::core::ffi::c_int as usize] =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    data[1 as ::core::ffi::c_int as usize] = owner_type as gpointer;
    g_hash_table_foreach(
        (*pool).hash_table,
        Some(pool_list as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()),
        &raw mut data as gpointer,
    );
    g_mutex_unlock(&raw mut (*pool).mutex);
    return data[0 as ::core::ffi::c_int as usize] as *mut GList;
}
unsafe extern "C" fn pspec_compare_id(mut a: gconstpointer, mut b: gconstpointer) -> gint {
    let mut pspec1: *const GParamSpec = a as *const GParamSpec;
    let mut pspec2: *const GParamSpec = b as *const GParamSpec;
    if (*pspec1).param_id < (*pspec2).param_id {
        return -(1 as gint);
    }
    if (*pspec1).param_id > (*pspec2).param_id {
        return 1 as gint;
    }
    return strcmp(
        (*pspec1).name as *const ::core::ffi::c_char,
        (*pspec2).name as *const ::core::ffi::c_char,
    ) as gint;
}
#[inline]
unsafe extern "C" fn should_list_pspec(
    mut pspec: *mut GParamSpec,
    mut owner_type: GType,
    mut ht: *mut GHashTable,
) -> gboolean {
    let mut found: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if !g_param_spec_get_redirect_target(pspec).is_null() {
        return 0 as gboolean;
    }
    found = param_spec_ht_lookup(
        ht,
        (*pspec).name,
        owner_type,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if found != pspec {
        let mut redirect: *mut GParamSpec = g_param_spec_get_redirect_target(found);
        if redirect != pspec {
            return 0 as gboolean;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn pool_depth_list(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) {
    let mut pspec: *mut GParamSpec = value as *mut GParamSpec;
    let mut data: *mut gpointer = user_data as *mut gpointer;
    let mut slists: *mut *mut GSList =
        *data.offset(0 as ::core::ffi::c_int as isize) as *mut *mut GSList;
    let mut owner_type: GType = *data.offset(1 as ::core::ffi::c_int as isize) as GType;
    let mut ht: *mut GHashTable = *data.offset(2 as ::core::ffi::c_int as isize) as *mut GHashTable;
    let mut count: *mut ::core::ffi::c_int =
        *data.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    if (owner_type == (*pspec).owner_type || g_type_is_a(owner_type, (*pspec).owner_type) != 0)
        && should_list_pspec(pspec, owner_type, ht) != 0
    {
        if g_type_fundamental((*pspec).owner_type)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            let ref mut fresh4 = *slists.offset(0 as ::core::ffi::c_int as isize);
            *fresh4 = g_slist_prepend(
                *slists.offset(0 as ::core::ffi::c_int as isize),
                pspec as gpointer,
            );
            *count = *count + 1 as ::core::ffi::c_int;
        } else {
            let mut d: guint = g_type_depth((*pspec).owner_type);
            let ref mut fresh5 = *slists.offset(d.wrapping_sub(1 as guint) as isize);
            *fresh5 = g_slist_prepend(
                *slists.offset(d.wrapping_sub(1 as guint) as isize),
                pspec as gpointer,
            );
            *count = *count + 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn pool_depth_list_for_interface(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) {
    let mut pspec: *mut GParamSpec = value as *mut GParamSpec;
    let mut data: *mut gpointer = user_data as *mut gpointer;
    let mut slists: *mut *mut GSList =
        *data.offset(0 as ::core::ffi::c_int as isize) as *mut *mut GSList;
    let mut owner_type: GType = *data.offset(1 as ::core::ffi::c_int as isize) as GType;
    let mut ht: *mut GHashTable = *data.offset(2 as ::core::ffi::c_int as isize) as *mut GHashTable;
    let mut count: *mut ::core::ffi::c_int =
        *data.offset(3 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    if (*pspec).owner_type == owner_type && should_list_pspec(pspec, owner_type, ht) != 0 {
        let ref mut fresh6 = *slists.offset(0 as ::core::ffi::c_int as isize);
        *fresh6 = g_slist_prepend(
            *slists.offset(0 as ::core::ffi::c_int as isize),
            pspec as gpointer,
        );
        *count = *count + 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pool_list(
    mut pool: *mut GParamSpecPool,
    mut owner_type: GType,
    mut n_pspecs_p: *mut guint,
) -> *mut *mut GParamSpec {
    let mut pspecs: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut p: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut slists: *mut *mut GSList = ::core::ptr::null_mut::<*mut GSList>();
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut data: [gpointer; 4] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 4];
    let mut d: guint = 0;
    let mut i: guint = 0;
    let mut n_pspecs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !pool.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_list\0" as *const u8 as *const ::core::ffi::c_char,
            b"pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut GParamSpec>();
    }
    if owner_type > 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_list\0" as *const u8 as *const ::core::ffi::c_char,
            b"owner_type > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut GParamSpec>();
    }
    if !n_pspecs_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_pool_list\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_pspecs_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut GParamSpec>();
    }
    g_mutex_lock(&raw mut (*pool).mutex);
    d = g_type_depth(owner_type);
    slists =
        g_malloc0_n(d as gsize, ::core::mem::size_of::<*mut GSList>() as gsize) as *mut *mut GSList;
    data[0 as ::core::ffi::c_int as usize] = slists as gpointer;
    data[1 as ::core::ffi::c_int as usize] = owner_type as gpointer;
    data[2 as ::core::ffi::c_int as usize] = (*pool).hash_table as gpointer;
    data[3 as ::core::ffi::c_int as usize] = &raw mut n_pspecs as gpointer;
    g_hash_table_foreach(
        (*pool).hash_table,
        if g_type_fundamental(owner_type)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            Some(
                pool_depth_list_for_interface
                    as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
            )
        } else {
            Some(pool_depth_list as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ())
        },
        &raw mut data as gpointer,
    );
    pspecs = g_malloc_n(
        (n_pspecs + 1 as ::core::ffi::c_int) as gsize,
        ::core::mem::size_of::<*mut GParamSpec>() as gsize,
    ) as *mut *mut GParamSpec;
    p = pspecs;
    i = 0 as guint;
    while i < d {
        let ref mut fresh1 = *slists.offset(i as isize);
        *fresh1 = g_slist_sort(
            *slists.offset(i as isize),
            Some(pspec_compare_id as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint),
        );
        node = *slists.offset(i as isize);
        while !node.is_null() {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = (*node).data as *mut GParamSpec;
            node = (*node).next;
        }
        g_slist_free(*slists.offset(i as isize));
        i = i.wrapping_add(1);
    }
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = ::core::ptr::null_mut::<GParamSpec>();
    g_free(slists as gpointer);
    g_mutex_unlock(&raw mut (*pool).mutex);
    *n_pspecs_p = n_pspecs as guint;
    return pspecs;
}
unsafe extern "C" fn param_spec_generic_class_init(
    mut g_class: gpointer,
    mut class_data: gpointer,
) {
    let mut class: *mut GParamSpecClass = g_class as *mut GParamSpecClass;
    let mut info: *mut ParamSpecClassInfo = class_data as *mut ParamSpecClassInfo;
    (*class).value_type = (*info).value_type;
    if (*info).finalize.is_some() {
        (*class).finalize = (*info).finalize;
    }
    (*class).value_set_default = (*info).value_set_default;
    if (*info).value_validate.is_some() {
        (*class).value_validate = (*info).value_validate;
    }
    (*class).values_cmp = (*info).values_cmp;
    g_free(class_data);
}
unsafe extern "C" fn default_value_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
}
unsafe extern "C" fn default_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    return memcmp(
        &raw const (*value1).data as *const ::core::ffi::c_void,
        &raw const (*value2).data as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[C2RustUnnamed; 2]>() as size_t,
    ) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_type_register_static(
    mut name: *const gchar,
    mut pspec_info: *const GParamSpecTypeInfo,
) -> GType {
    let mut info: GTypeInfo = _GTypeInfo {
        class_size: ::core::mem::size_of::<GParamSpecClass>() as guint16,
        base_init: None,
        base_finalize: None,
        class_init: Some(
            param_spec_generic_class_init as unsafe extern "C" fn(gpointer, gpointer) -> (),
        ),
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 16 as guint16,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    let mut cinfo: *mut ParamSpecClassInfo = ::core::ptr::null_mut::<ParamSpecClassInfo>();
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !pspec_info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"pspec_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if g_type_from_name(name) == 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_from_name (name) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if (*pspec_info).instance_size as usize >= ::core::mem::size_of::<GParamSpec>() as usize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"pspec_info->instance_size >= sizeof (GParamSpec)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !g_type_name((*pspec_info).value_type).is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_name (pspec_info->value_type) != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    info.instance_size = (*pspec_info).instance_size;
    info.n_preallocs = (*pspec_info).n_preallocs;
    info.instance_init = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
        GInstanceInitFunc,
    >((*pspec_info).instance_init);
    cinfo = g_malloc_n(
        1 as gsize,
        ::core::mem::size_of::<ParamSpecClassInfo>() as gsize,
    ) as *mut ParamSpecClassInfo;
    (*cinfo).value_type = (*pspec_info).value_type;
    (*cinfo).finalize = (*pspec_info).finalize;
    (*cinfo).value_set_default = (if (*pspec_info).value_set_default.is_some() {
        (*pspec_info).value_set_default
            as Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>
    } else {
        Some(default_value_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ())
    })
        as Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>;
    (*cinfo).value_validate = (*pspec_info).value_validate;
    (*cinfo).values_cmp = (if (*pspec_info).values_cmp.is_some() {
        (*pspec_info).values_cmp
            as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>
    } else {
        Some(
            default_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        )
    })
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>;
    info.class_data = cinfo as gconstpointer;
    return g_type_register_static(
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut info,
        G_TYPE_FLAG_NONE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_param(mut value: *mut GValue, mut param: *mut GParamSpec) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_param\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_PARAM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !param.is_null() {
        if g_type_check_instance_is_fundamentally_a(
            param as *mut GTypeInstance,
            ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_set_param\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_PARAM_SPEC (param)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_param_spec_unref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        );
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = param as gpointer;
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_param_spec_ref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_param_take_ownership(
    mut value: *mut GValue,
    mut param: *mut GParamSpec,
) {
    g_value_take_param(value, param);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_take_param(mut value: *mut GValue, mut param: *mut GParamSpec) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_take_param\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_PARAM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !param.is_null() {
        if g_type_check_instance_is_fundamentally_a(
            param as *mut GTypeInstance,
            ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_take_param\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_PARAM_SPEC (param)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_param_spec_unref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        );
    }
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = param as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_param(mut value: *const GValue) -> *mut GParamSpec {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_get_param\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_PARAM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_dup_param(mut value: *const GValue) -> *mut GParamSpec {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_dup_param\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_PARAM (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    return if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_param_spec_ref(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec,
        )
    } else {
        ::core::ptr::null_mut::<GParamSpec>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_default_value(
    mut pspec: *mut GParamSpec,
) -> *const GValue {
    let mut priv_0: *mut GParamSpecPrivate = g_param_spec_get_private(pspec);
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).default_value.g_type;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut (*priv_0).default_value.g_type;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut (*priv_0).default_value.g_type as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut default_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        g_value_init(&raw mut default_value, (*pspec).value_type);
        g_param_value_set_default(pspec, &raw mut default_value);
        memcpy(
            &raw mut (*priv_0).default_value.data as *mut C2RustUnnamed as *mut ::core::ffi::c_void,
            &raw mut default_value.data as *mut C2RustUnnamed as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[C2RustUnnamed; 2]>() as size_t,
        );
        if 0 as ::core::ffi::c_int != 0 {
            (*priv_0).default_value.g_type = (*pspec).value_type;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut (*priv_0).default_value.g_type as *mut ::core::ffi::c_void,
            (*pspec).value_type as gpointer,
        );
    }
    return &raw mut (*priv_0).default_value;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_get_name_quark(mut pspec: *mut GParamSpec) -> GQuark {
    let mut priv_0: *mut GParamSpecPrivate = g_param_spec_get_private(pspec);
    return (*priv_0).name_quark;
}
