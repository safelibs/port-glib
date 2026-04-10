// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GWakeup;
    pub type _GParamSpecPool;
    fn memcpy(
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_nullify_pointer(nullify_location: *mut gpointer);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bit_lock(address: *mut gint, lock_bit: gint);
    fn g_bit_unlock(address: *mut gint, lock_bit: gint);
    fn g_pointer_bit_lock(address: *mut ::core::ffi::c_void, lock_bit: gint);
    fn g_pointer_bit_lock_and_get(address: gpointer, lock_bit: guint, out_ptr: *mut guintptr);
    fn g_pointer_bit_unlock(address: *mut ::core::ffi::c_void, lock_bit: gint);
    fn g_pointer_bit_lock_mask_ptr(
        ptr: gpointer,
        lock_bit: guint,
        set: gboolean,
        preserve_mask: guintptr,
        preserve_ptr: gpointer,
    ) -> gpointer;
    fn g_pointer_bit_unlock_and_set(
        address: *mut ::core::ffi::c_void,
        lock_bit: guint,
        ptr: gpointer,
        preserve_mask: guintptr,
    );
    fn g_datalist_clear(datalist: *mut *mut GData);
    fn g_datalist_id_get_data(datalist: *mut *mut GData, key_id: GQuark) -> gpointer;
    fn g_datalist_id_set_data_full(
        datalist: *mut *mut GData,
        key_id: GQuark,
        data: gpointer,
        destroy_func: GDestroyNotify,
    );
    fn g_datalist_id_dup_data(
        datalist: *mut *mut GData,
        key_id: GQuark,
        dup_func: GDuplicateFunc,
        user_data: gpointer,
    ) -> gpointer;
    fn g_datalist_id_replace_data(
        datalist: *mut *mut GData,
        key_id: GQuark,
        oldval: gpointer,
        newval: gpointer,
        destroy: GDestroyNotify,
        old_destroy: *mut GDestroyNotify,
    ) -> gboolean;
    fn g_datalist_id_remove_no_notify(datalist: *mut *mut GData, key_id: GQuark) -> gpointer;
    fn g_datalist_set_flags(datalist: *mut *mut GData, flags: guint);
    fn g_datalist_unset_flags(datalist: *mut *mut GData, flags: guint);
    fn g_datalist_get_flags(datalist: *mut *mut GData) -> guint;
    fn g_datalist_get_data(datalist: *mut *mut GData, key: *const gchar) -> gpointer;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_free_sized(mem: gpointer, size: size_t);
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_copy(list: *mut GSList) -> *mut GSList;
    fn g_slist_find(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
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
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_parent(type_0: GType) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_peek(type_0: GType) -> gpointer;
    fn g_type_class_peek_static(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_interfaces(type_0: GType, n_interfaces: *mut guint) -> *mut GType;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_fundamental(
        type_id: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        finfo: *const GTypeFundamentalInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_create_instance(type_0: GType) -> *mut GTypeInstance;
    fn g_type_free_instance(instance: *mut GTypeInstance);
    fn g_type_add_interface_check(check_data: gpointer, check_func: GTypeInterfaceCheckFunc);
    fn g_type_value_table_peek(type_0: GType) -> *mut GTypeValueTable;
    fn g_type_check_instance(instance: *mut GTypeInstance) -> gboolean;
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_type_check_class_is_a(g_class: *mut GTypeClass, is_a_type: GType) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_reset(value: *mut GValue) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_value_type_compatible(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_type_transformable(src_type: GType, dest_type: GType) -> gboolean;
    fn g_value_transform(src_value: *const GValue, dest_value: *mut GValue) -> gboolean;
    fn g_value_register_transform_func(
        src_type: GType,
        dest_type: GType,
        transform_func: GValueTransform,
    );
    fn g_param_spec_unref(pspec: *mut GParamSpec);
    fn g_param_spec_ref_sink(pspec: *mut GParamSpec) -> *mut GParamSpec;
    fn g_param_value_set_default(pspec: *mut GParamSpec, value: *mut GValue);
    fn g_param_value_validate(pspec: *mut GParamSpec, value: *mut GValue) -> gboolean;
    fn g_param_spec_get_default_value(pspec: *mut GParamSpec) -> *const GValue;
    fn g_param_spec_get_name_quark(pspec: *mut GParamSpec) -> GQuark;
    fn g_param_spec_pool_new(type_prefixing: gboolean) -> *mut GParamSpecPool;
    fn g_param_spec_pool_insert(
        pool: *mut GParamSpecPool,
        pspec: *mut GParamSpec,
        owner_type: GType,
    );
    fn g_param_spec_pool_remove(pool: *mut GParamSpecPool, pspec: *mut GParamSpec);
    fn g_param_spec_pool_lookup(
        pool: *mut GParamSpecPool,
        param_name: *const gchar,
        owner_type: GType,
        walk_ancestors: gboolean,
    ) -> *mut GParamSpec;
    fn g_param_spec_pool_list_owned(pool: *mut GParamSpecPool, owner_type: GType) -> *mut GList;
    fn g_param_spec_pool_list(
        pool: *mut GParamSpecPool,
        owner_type: GType,
        n_pspecs_p: *mut guint,
    ) -> *mut *mut GParamSpec;
    fn g_param_spec_pool_free(pool: *mut GParamSpecPool);
    fn g_cclosure_new(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
    fn g_cclosure_new_swap(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
    fn g_closure_new_simple(sizeof_closure: guint, data: gpointer) -> *mut GClosure;
    fn g_closure_add_invalidate_notifier(
        closure: *mut GClosure,
        notify_data: gpointer,
        notify_func: GClosureNotify,
    );
    fn g_closure_remove_invalidate_notifier(
        closure: *mut GClosure,
        notify_data: gpointer,
        notify_func: GClosureNotify,
    );
    fn g_closure_add_marshal_guards(
        closure: *mut GClosure,
        pre_marshal_data: gpointer,
        pre_marshal_notify: GClosureNotify,
        post_marshal_data: gpointer,
        post_marshal_notify: GClosureNotify,
    );
    fn g_closure_invalidate(closure: *mut GClosure);
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_parse_name(
        detailed_signal: *const gchar,
        itype: GType,
        signal_id_p: *mut guint,
        detail_p: *mut GQuark,
        force_detail_quark: gboolean,
    ) -> gboolean;
    fn g_signal_connect_closure(
        instance: gpointer,
        detailed_signal: *const gchar,
        closure: *mut GClosure,
        after: gboolean,
    ) -> gulong;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_signal_handlers_destroy(instance: gpointer);
    fn _g_signals_destroy(itype: GType);
    fn g_param_spec_override(name: *const gchar, overridden: *mut GParamSpec) -> *mut GParamSpec;
    static mut g_param_spec_types: *mut GType;
    fn g_strdup_value_contents(value: *const GValue) -> *mut gchar;
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
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GDuplicateFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> gpointer>;
pub type GDir = _GDir;
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
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
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
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
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
pub struct _GTypeInterface {
    pub g_type: GType,
    pub g_instance_type: GType,
}
pub type GTypeInterface = _GTypeInterface;
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
pub type GTypeInterfaceCheckFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub struct _GParameter {
    pub name: *const gchar,
    pub value: GValue,
}
pub type GParameter = _GParameter;
pub type GParamSpecPool = _GParamSpecPool;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
pub type GSignalMatchType = ::core::ffi::c_uint;
pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
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
pub type GInitiallyUnowned = _GObject;
pub type GInitiallyUnownedClass = _GObjectClass;
pub type GWeakNotify = Option<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
pub type GParamSpecOverride = _GParamSpecOverride;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecOverride {
    pub parent_instance: GParamSpec,
    pub overridden: *mut GParamSpec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PspecEntry {
    pub name: *const ::core::ffi::c_char,
    pub pspec: *mut GParamSpec,
}
pub type GObjectNotifyQueue = _GObjectNotifyQueue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectNotifyQueue {
    pub pspecs: *mut GSList,
    pub n_pspecs: guint16,
    pub freeze_count: guint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeakRefData {
    pub atomic_field: gint,
    pub len: guint16,
    pub alloc: guint16,
    pub list: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub one: *mut GWeakRef,
    pub many: *mut *mut GWeakRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub p: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GObjectReal {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub optional_flags: guint,
    pub qdata: *mut GData,
}
pub type GToggleNotify = Option<unsafe extern "C" fn(gpointer, *mut GObject, gboolean) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub notify: GToggleNotify,
    pub data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ToggleRefStack {
    pub n_toggle_refs: guint,
    pub toggle_refs: [C2RustUnnamed_2; 1],
}
pub const NOTIFY: C2RustUnnamed_6 = 0;
pub const G_VALUE_COLLECT_POINTER: C2RustUnnamed_5 = 112;
pub const G_VALUE_COLLECT_DOUBLE: C2RustUnnamed_5 = 100;
pub const G_VALUE_COLLECT_INT64: C2RustUnnamed_5 = 113;
pub const G_VALUE_COLLECT_LONG: C2RustUnnamed_5 = 108;
pub const G_VALUE_COLLECT_INT: C2RustUnnamed_5 = 105;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CArray {
    pub object: *mut GObject,
    pub n_closures: guint,
    pub closures: [*mut GClosure; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WeakRefStack {
    pub object: *mut GObject,
    pub n_weak_refs: guint,
    pub weak_refs: [C2RustUnnamed_3; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub notify: GWeakNotify,
    pub data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: *mut ::core::ffi::c_char,
    pub out: *mut *mut GObject,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_6 = 1;
#[inline]
unsafe extern "C" fn g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    return ref_0;
}
#[inline]
unsafe extern "C" fn g_set_object(
    mut object_ptr: *mut *mut GObject,
    mut new_object: *mut GObject,
) -> gboolean {
    let mut old_object: *mut GObject = *object_ptr;
    if old_object == new_object {
        return 0 as gboolean;
    }
    if !new_object.is_null() {
        g_object_ref(new_object as gpointer);
    }
    *object_ptr = new_object;
    if !old_object.is_null() {
        g_object_unref(old_object as gpointer);
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
static mut quark_closure_array: GQuark = 0 as GQuark;
static mut quark_weak_notifies: GQuark = 0 as GQuark;
static mut quark_toggle_refs: GQuark = 0 as GQuark;
static mut quark_notify_queue: GQuark = 0;
static mut pspec_pool: *mut GParamSpecPool =
    ::core::ptr::null::<GParamSpecPool>() as *mut GParamSpecPool;
static mut gobject_signals: [gulong; 1] = [0 as ::core::ffi::c_int as gulong];
static mut floating_flag_handler: Option<unsafe extern "C" fn(*mut GObject, gint) -> guint> = unsafe {
    Some(object_floating_flag_handler as unsafe extern "C" fn(*mut GObject, gint) -> guint)
};
static mut quark_weak_locations: GQuark = 0 as GQuark;
#[inline(always)]
unsafe extern "C" fn object_get_optional_flags_p(mut object: *mut GObject) -> *mut guint {
    return &raw mut (*(object as *mut GObjectReal)).optional_flags;
}
unsafe extern "C" fn weak_ref_data_ref(mut wrdata: *mut WeakRefData) -> *mut WeakRefData {
    let mut ref_0: gint = 0;
    ref_0 = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*wrdata).atomic_field;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut (*wrdata).atomic_field,
            1 as ::core::ffi::c_int,
        )
    });
    return wrdata;
}
unsafe extern "C" fn weak_ref_data_unref(mut wrdata: *mut WeakRefData) {
    if wrdata.is_null() {
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*wrdata).atomic_field;
            (*wrdata).atomic_field;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*wrdata).atomic_field,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    g_free_sized(
        wrdata as gpointer,
        ::core::mem::size_of::<WeakRefData>() as size_t,
    );
}
unsafe extern "C" fn weak_ref_data_lock(mut wrdata: *mut WeakRefData) {
    if !wrdata.is_null() {
        g_bit_lock(&raw mut (*wrdata).atomic_field as *mut gint, 30 as gint);
    }
}
unsafe extern "C" fn weak_ref_data_unlock(mut wrdata: *mut WeakRefData) {
    if !wrdata.is_null() {
        g_bit_unlock(&raw mut (*wrdata).atomic_field as *mut gint, 30 as gint);
    }
}
unsafe extern "C" fn weak_ref_data_get_or_create_cb(
    mut key_id: GQuark,
    mut data: *mut gpointer,
    mut destroy_notify: *mut GDestroyNotify,
    mut user_data: gpointer,
) -> gpointer {
    let mut wrdata: *mut WeakRefData = *data as *mut WeakRefData;
    let mut object: *mut GObject = user_data as *mut GObject;
    if wrdata.is_null() {
        wrdata = g_malloc_n(1 as gsize, ::core::mem::size_of::<WeakRefData>() as gsize)
            as *mut WeakRefData;
        (*wrdata).atomic_field = 1 as ::core::ffi::c_int as gint;
        (*wrdata).len = 0 as guint16;
        *data = wrdata as gpointer;
        *destroy_notify = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut WeakRefData) -> ()>,
            GDestroyNotify,
        >(Some(
            weak_ref_data_unref as unsafe extern "C" fn(*mut WeakRefData) -> (),
        ));
        object_set_optional_flags(
            object,
            ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as guint,
        );
    }
    return wrdata as gpointer;
}
unsafe extern "C" fn weak_ref_data_get_or_create(mut object: *mut GObject) -> *mut WeakRefData {
    if object.is_null() {
        return ::core::ptr::null_mut::<WeakRefData>();
    }
    return (*glib__private__())
        .g_datalist_id_update_atomic
        .expect("non-null function pointer")(
        &raw mut (*object).qdata,
        quark_weak_locations,
        Some(
            weak_ref_data_get_or_create_cb
                as unsafe extern "C" fn(
                    GQuark,
                    *mut gpointer,
                    *mut GDestroyNotify,
                    gpointer,
                ) -> gpointer,
        ),
        object as gpointer,
    ) as *mut WeakRefData;
}
unsafe extern "C" fn weak_ref_data_get(mut object: *mut GObject) -> *mut WeakRefData {
    return g_datalist_id_get_data(&raw mut (*object).qdata, quark_weak_locations)
        as *mut WeakRefData;
}
unsafe extern "C" fn weak_ref_data_get_surely(mut object: *mut GObject) -> *mut WeakRefData {
    let mut wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    wrdata = weak_ref_data_get(object);
    return wrdata;
}
unsafe extern "C" fn weak_ref_data_list_find(
    mut wrdata: *mut WeakRefData,
    mut weak_ref: *mut GWeakRef,
) -> gint32 {
    if (*wrdata).len as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint {
        if (*wrdata).list.one == weak_ref {
            return 0 as gint32;
        }
    } else {
        let mut i: guint16 = 0;
        i = 0 as guint16;
        while (i as ::core::ffi::c_int) < (*wrdata).len as ::core::ffi::c_int {
            if *(*wrdata).list.many.offset(i as isize) == weak_ref {
                return i as gint32;
            }
            i = i.wrapping_add(1);
        }
    }
    return -(1 as gint32);
}
unsafe extern "C" fn weak_ref_data_list_add(
    mut wrdata: *mut WeakRefData,
    mut weak_ref: *mut GWeakRef,
) -> gboolean {
    if (*wrdata).len as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        (*wrdata).list.one = weak_ref;
    } else {
        if (*wrdata).len as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint {
            let mut weak_ref2: *mut GWeakRef = (*wrdata).list.one;
            (*wrdata).alloc = 4 as guint16;
            (*wrdata).list.many = g_malloc_n(
                (*wrdata).alloc as gsize,
                ::core::mem::size_of::<*mut GWeakRef>() as gsize,
            ) as *mut *mut GWeakRef;
            let ref mut fresh47 = *(*wrdata).list.many.offset(0 as ::core::ffi::c_int as isize);
            *fresh47 = weak_ref2;
        } else if (*wrdata).len as ::core::ffi::c_int == (*wrdata).alloc as ::core::ffi::c_int {
            let mut alloc: guint16 = 0;
            alloc = ((*wrdata).alloc as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint)
                as guint16;
            if (alloc as ::core::ffi::c_int) < (*wrdata).len as ::core::ffi::c_int {
                if (*wrdata).len as ::core::ffi::c_int
                    == 0xffff as ::core::ffi::c_int as guint16 as ::core::ffi::c_int
                {
                    return 0 as gboolean;
                }
                alloc = 0xffff as ::core::ffi::c_int as guint16;
            }
            (*wrdata).list.many = g_realloc_n(
                (*wrdata).list.many as gpointer,
                alloc as gsize,
                ::core::mem::size_of::<*mut GWeakRef>() as gsize,
            ) as *mut *mut GWeakRef;
            (*wrdata).alloc = alloc;
        }
        let ref mut fresh48 = *(*wrdata).list.many.offset((*wrdata).len as isize);
        *fresh48 = weak_ref;
    }
    (*wrdata).len = (*wrdata).len.wrapping_add(1);
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn weak_ref_data_list_remove(
    mut wrdata: *mut WeakRefData,
    mut idx: guint16,
    mut allow_shrink: gboolean,
) -> *mut GWeakRef {
    let mut weak_ref: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    (*wrdata).len = (*wrdata).len.wrapping_sub(1);
    if (*wrdata).len as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
        weak_ref = (*wrdata).list.one;
    } else {
        weak_ref = *(*wrdata).list.many.offset(idx as isize);
        if (*wrdata).len as ::core::ffi::c_uint == 1 as ::core::ffi::c_uint {
            let mut weak_ref2: *mut GWeakRef = *(*wrdata).list.many.offset(
                (if idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as isize,
            );
            g_free((*wrdata).list.many as gpointer);
            (*wrdata).list.one = weak_ref2;
        } else {
            let ref mut fresh22 = *(*wrdata).list.many.offset(idx as isize);
            *fresh22 = *(*wrdata).list.many.offset((*wrdata).len as isize);
            if allow_shrink != 0
                && (*wrdata).len as ::core::ffi::c_uint
                    <= ((*wrdata).alloc as ::core::ffi::c_uint)
                        .wrapping_div(4 as ::core::ffi::c_uint)
            {
                if (*wrdata).alloc as ::core::ffi::c_int
                    == 0xffff as ::core::ffi::c_int as guint16 as ::core::ffi::c_int
                {
                    (*wrdata).alloc = (0xffff as ::core::ffi::c_int as guint16
                        as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_div(2 as ::core::ffi::c_uint)
                        as guint16;
                } else {
                    (*wrdata).alloc = ((*wrdata).alloc as ::core::ffi::c_uint)
                        .wrapping_div(2 as ::core::ffi::c_uint)
                        as guint16 as guint16;
                }
                (*wrdata).list.many = g_realloc_n(
                    (*wrdata).list.many as gpointer,
                    (*wrdata).alloc as gsize,
                    ::core::mem::size_of::<*mut GWeakRef>() as gsize,
                ) as *mut *mut GWeakRef;
            }
        }
    }
    return weak_ref;
}
unsafe extern "C" fn weak_ref_data_has(
    mut object: *mut GObject,
    mut wrdata: *mut WeakRefData,
    mut out_new_wrdata: *mut *mut WeakRefData,
) -> gboolean {
    let mut wrdata2: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    if object.is_null() {
        return wrdata.is_null() as ::core::ffi::c_int;
    }
    if wrdata.is_null() {
        if !out_new_wrdata.is_null() {
            *out_new_wrdata = weak_ref_data_ref(weak_ref_data_get(object));
        }
        return 0 as gboolean;
    }
    wrdata2 = weak_ref_data_get_surely(object);
    if wrdata == wrdata2 {
        if !out_new_wrdata.is_null() {
            *out_new_wrdata = ::core::ptr::null_mut::<WeakRefData>();
        }
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if !out_new_wrdata.is_null() {
        *out_new_wrdata = weak_ref_data_ref(wrdata2);
    }
    return 0 as gboolean;
}
unsafe extern "C" fn object_bit_lock(mut object: *mut GObject, mut lock_bit: guint) {
    g_bit_lock(
        object_get_optional_flags_p(object) as *mut gint as *mut gint,
        3 as gint,
    );
}
unsafe extern "C" fn object_bit_unlock(mut object: *mut GObject, mut lock_bit: guint) {
    g_bit_unlock(
        object_get_optional_flags_p(object) as *mut gint as *mut gint,
        3 as gint,
    );
}
unsafe extern "C" fn g_object_notify_queue_free(mut data: gpointer) {
    let mut nqueue: *mut GObjectNotifyQueue = data as *mut GObjectNotifyQueue;
    g_slist_free((*nqueue).pspecs);
    g_free_sized(
        nqueue as gpointer,
        ::core::mem::size_of::<GObjectNotifyQueue>() as size_t,
    );
}
unsafe extern "C" fn g_object_notify_queue_create_queue_frozen(
    mut object: *mut GObject,
) -> *mut GObjectNotifyQueue {
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    nqueue = g_malloc0_n(
        1 as gsize,
        ::core::mem::size_of::<GObjectNotifyQueue>() as gsize,
    ) as *mut GObjectNotifyQueue;
    *nqueue = _GObjectNotifyQueue {
        pspecs: ::core::ptr::null_mut::<GSList>(),
        n_pspecs: 0,
        freeze_count: 1 as guint16,
    };
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_notify_queue,
        nqueue as gpointer,
        Some(g_object_notify_queue_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    return nqueue;
}
unsafe extern "C" fn g_object_notify_queue_freeze(
    mut object: *mut GObject,
) -> *mut GObjectNotifyQueue {
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    object_bit_lock(object, 2 as guint);
    nqueue = g_datalist_id_get_data(&raw mut (*object).qdata, quark_notify_queue)
        as *mut GObjectNotifyQueue;
    if nqueue.is_null() {
        nqueue = g_object_notify_queue_create_queue_frozen(object);
    } else if (*nqueue).freeze_count as ::core::ffi::c_int >= 65535 as ::core::ffi::c_int {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Free queue for %s (%p) is larger than 65535, called g_object_freeze_notify() too often. Forgot to call g_object_thaw_notify() or infinite loop\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            object,
        );
    } else {
        (*nqueue).freeze_count = (*nqueue).freeze_count.wrapping_add(1);
    }
    object_bit_unlock(object, 2 as guint);
    return nqueue;
}
unsafe extern "C" fn g_object_notify_queue_thaw(
    mut object: *mut GObject,
    mut nqueue: *mut GObjectNotifyQueue,
    mut take_ref: gboolean,
) {
    let mut pspecs_mem: [*mut GParamSpec; 16] = [::core::ptr::null_mut::<GParamSpec>(); 16];
    let mut pspecs: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut free_me: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut slist: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut n_pspecs: guint = 0 as guint;
    object_bit_lock(object, 2 as guint);
    if nqueue.is_null() {
        nqueue = g_datalist_id_get_data(&raw mut (*object).qdata, quark_notify_queue)
            as *mut GObjectNotifyQueue;
    }
    if nqueue.is_null() || (*nqueue).freeze_count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        object_bit_unlock(object, 2 as guint);
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: property-changed notification for %s(%p) is not frozen\0" as *const u8
                as *const gchar,
            b"g_object_notify_queue_thaw\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            object,
        );
        return;
    }
    (*nqueue).freeze_count = (*nqueue).freeze_count.wrapping_sub(1);
    if (*nqueue).freeze_count != 0 {
        object_bit_unlock(object, 2 as guint);
        return;
    }
    pspecs = if (*nqueue).n_pspecs as ::core::ffi::c_int > 16 as ::core::ffi::c_int {
        free_me = g_malloc_n(
            (*nqueue).n_pspecs as gsize,
            ::core::mem::size_of::<*mut GParamSpec>() as gsize,
        ) as *mut *mut GParamSpec;
        free_me
    } else {
        &raw mut pspecs_mem as *mut *mut GParamSpec
    };
    slist = (*nqueue).pspecs;
    while !slist.is_null() {
        let fresh13 = n_pspecs;
        n_pspecs = n_pspecs.wrapping_add(1);
        let ref mut fresh14 = *pspecs.offset(fresh13 as isize);
        *fresh14 = (*slist).data as *mut GParamSpec;
        slist = (*slist).next;
    }
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_notify_queue,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
    object_bit_unlock(object, 2 as guint);
    if n_pspecs != 0 {
        if take_ref != 0 {
            g_object_ref(object as gpointer);
        }
        (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
            .dispatch_properties_changed
            .expect("non-null function pointer")(object, n_pspecs, pspecs);
        if take_ref != 0 {
            g_object_unref(object as gpointer);
        }
    }
    g_free(free_me as gpointer);
}
unsafe extern "C" fn g_object_notify_queue_add(
    mut object: *mut GObject,
    mut nqueue: *mut GObjectNotifyQueue,
    mut pspec: *mut GParamSpec,
    mut in_init: gboolean,
) -> gboolean {
    object_bit_lock(object, 2 as guint);
    if nqueue.is_null() {
        nqueue = g_datalist_id_get_data(&raw mut (*object).qdata, quark_notify_queue)
            as *mut GObjectNotifyQueue;
        if nqueue.is_null() {
            if in_init == 0 {
                object_bit_unlock(object, 2 as guint);
                return 0 as gboolean;
            }
            nqueue = g_object_notify_queue_create_queue_frozen(object);
        }
    }
    if ((*nqueue).n_pspecs as ::core::ffi::c_int) < 65535 as ::core::ffi::c_int {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            795 as ::core::ffi::c_int,
            b"g_object_notify_queue_add\0" as *const u8 as *const ::core::ffi::c_char,
            b"nqueue->n_pspecs < 65535\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_slist_find((*nqueue).pspecs, pspec as gconstpointer).is_null() {
        (*nqueue).pspecs = g_slist_prepend((*nqueue).pspecs, pspec as gpointer);
        (*nqueue).n_pspecs = (*nqueue).n_pspecs.wrapping_add(1);
    }
    object_bit_unlock(object, 2 as guint);
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _g_object_type_init() {
    static mut initialized: gboolean = 0 as gboolean;
    static mut finfo: GTypeFundamentalInfo = _GTypeFundamentalInfo {
        type_flags: (G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
            | G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DEEP_DERIVABLE as ::core::ffi::c_int)
            as GTypeFundamentalFlags,
    };
    let mut info: GTypeInfo = _GTypeInfo {
        class_size: ::core::mem::size_of::<GObjectClass>() as guint16,
        base_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObjectClass) -> ()>,
            GBaseInitFunc,
        >(Some(
            g_object_base_class_init as unsafe extern "C" fn(*mut GObjectClass) -> (),
        )),
        base_finalize: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObjectClass) -> ()>,
            GBaseFinalizeFunc,
        >(Some(
            g_object_base_class_finalize as unsafe extern "C" fn(*mut GObjectClass) -> (),
        )),
        class_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObjectClass) -> ()>,
            GClassInitFunc,
        >(Some(
            g_object_do_class_init as unsafe extern "C" fn(*mut GObjectClass) -> (),
        )),
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: ::core::mem::size_of::<GObject>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GObjectClass) -> ()>,
            GInstanceInitFunc,
        >(Some(
            g_object_init as unsafe extern "C" fn(*mut GObject, *mut GObjectClass) -> (),
        )),
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    static mut value_table: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(g_value_object_init as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: Some(g_value_object_free_value as unsafe extern "C" fn(*mut GValue) -> ()),
            value_copy: Some(
                g_value_object_copy_value as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                g_value_object_peek_pointer as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                g_value_object_collect_value
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                g_value_object_lcopy_value
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    let mut type_0: GType = 0;
    if initialized == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_object_type_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"initialized == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    initialized = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    info.value_table = &raw const value_table;
    type_0 = g_type_register_fundamental(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GObject\0" as *const u8 as *const gchar),
        &raw mut info,
        &raw const finfo,
        G_TYPE_FLAG_NONE,
    );
    if type_0 == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            884 as ::core::ffi::c_int,
            b"_g_object_type_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_OBJECT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_value_register_transform_func(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        Some(
            g_value_object_transform_value
                as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
        ),
    );
}
#[inline]
unsafe extern "C" fn g_object_init_pspec_pool() {
    if ({
        let mut gapg_temp_newval: *mut GParamSpecPool = ::core::ptr::null_mut::<GParamSpecPool>();
        let mut gapg_temp_atomic: *mut *mut GParamSpecPool = &raw mut pspec_pool;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    })
    .is_null()
    {
        let mut pool: *mut GParamSpecPool =
            g_param_spec_pool_new((0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        if ({
            let mut gapcae_oldval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if 0 as ::core::ffi::c_int != 0 {
                pspec_pool;
            } else {
            };
            let fresh2 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut pspec_pool,
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GParamSpecPool),
                pool,
            );
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GParamSpecPool) =
                fresh2.0;
            if fresh2.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) == 0
        {
            g_param_spec_pool_free(pool);
        }
    }
}
unsafe extern "C" fn g_object_base_class_init(mut class: *mut GObjectClass) {
    let mut pclass: *mut GObjectClass =
        g_type_class_peek_parent(class as gpointer) as *mut GObjectClass;
    (*class).flags &= !(0x2 as ::core::ffi::c_int) as gsize;
    if !pclass.is_null() {
        (*pclass).flags |= 0x2 as gsize;
    }
    (*class).construct_properties = if !pclass.is_null() {
        g_slist_copy((*pclass).construct_properties)
    } else {
        ::core::ptr::null_mut::<GSList>()
    };
    (*class).n_construct_properties = g_slist_length((*class).construct_properties) as gsize;
    (*class).get_property = None;
    (*class).set_property = None;
    (*class).pspecs = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    (*class).n_pspecs = 0 as gsize;
}
unsafe extern "C" fn g_object_base_class_finalize(mut class: *mut GObjectClass) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut node: *mut GList = ::core::ptr::null_mut::<GList>();
    _g_signals_destroy((*(class as *mut GTypeClass)).g_type);
    g_slist_free((*class).construct_properties);
    (*class).construct_properties = ::core::ptr::null_mut::<GSList>();
    (*class).n_construct_properties = 0 as gsize;
    list = g_param_spec_pool_list_owned(pspec_pool, (*(class as *mut GTypeClass)).g_type);
    node = list;
    while !node.is_null() {
        let mut pspec: *mut GParamSpec = (*node).data as *mut GParamSpec;
        g_param_spec_pool_remove(pspec_pool, pspec);
        (*pspec).param_id = 0 as guint;
        g_param_spec_unref(pspec);
        node = (*node).next;
    }
    g_list_free(list);
}
unsafe extern "C" fn g_object_do_class_init(mut class: *mut GObjectClass) {
    quark_closure_array =
        g_quark_from_static_string(b"GObject-closure-array\0" as *const u8 as *const gchar);
    quark_weak_notifies =
        g_quark_from_static_string(b"GObject-weak-notifies\0" as *const u8 as *const gchar);
    quark_weak_locations =
        g_quark_from_static_string(b"GObject-weak-locations\0" as *const u8 as *const gchar);
    quark_toggle_refs =
        g_quark_from_static_string(b"GObject-toggle-references\0" as *const u8 as *const gchar);
    quark_notify_queue =
        g_quark_from_static_string(b"GObject-notify-queue\0" as *const u8 as *const gchar);
    g_object_init_pspec_pool();
    (*class).constructor = Some(
        g_object_constructor
            as unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject,
    )
        as Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>;
    (*class).constructed = Some(g_object_constructed as unsafe extern "C" fn(*mut GObject) -> ())
        as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).set_property = Some(
        g_object_do_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*class).get_property = Some(
        g_object_do_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*class).dispose = Some(g_object_real_dispose as unsafe extern "C" fn(*mut GObject) -> ())
        as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).finalize = Some(g_object_finalize as unsafe extern "C" fn(*mut GObject) -> ())
        as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).dispatch_properties_changed = Some(
        g_object_dispatch_properties_changed
            as unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> ()>;
    (*class).notify = None;
    gobject_signals[NOTIFY as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"notify\0" as *const u8 as *const gchar),
        (*(class as *mut GTypeClass)).g_type,
        (G_SIGNAL_RUN_FIRST as ::core::ffi::c_int
            | G_SIGNAL_NO_RECURSE as ::core::ffi::c_int
            | G_SIGNAL_DETAILED as ::core::ffi::c_int
            | G_SIGNAL_NO_HOOKS as ::core::ffi::c_int
            | G_SIGNAL_ACTION as ::core::ffi::c_int) as GSignalFlags,
        64 as ::core::ffi::c_ulong as glong as guint,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
        ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        1 as guint,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) as gulong;
    g_type_add_interface_check(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(object_interface_check_properties as unsafe extern "C" fn(gpointer, gpointer) -> ()),
    );
}
#[inline]
unsafe extern "C" fn install_property_internal(
    mut g_type: GType,
    mut property_id: guint,
    mut pspec: *mut GParamSpec,
) -> gboolean {
    g_param_spec_ref_sink(pspec);
    g_object_init_pspec_pool();
    if !g_param_spec_pool_lookup(pspec_pool, (*pspec).name, g_type, 0 as gboolean).is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"When installing property: type '%s' already has a property named '%s'\0" as *const u8
                as *const gchar,
            g_type_name(g_type),
            (*pspec).name,
        );
        g_param_spec_unref(pspec);
        return 0 as gboolean;
    }
    (*pspec).param_id = property_id;
    g_param_spec_pool_insert(
        pspec_pool,
        g_steal_pointer(&raw mut pspec as gpointer) as *mut GParamSpec,
        g_type,
    );
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn validate_pspec_to_install(mut pspec: *mut GParamSpec) -> gboolean {
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"validate_pspec_to_install\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*pspec).param_id == 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"validate_pspec_to_install\0" as *const u8 as *const ::core::ffi::c_char,
            b"PARAM_SPEC_PARAM_ID (pspec) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int
        & (G_PARAM_READABLE as ::core::ffi::c_int | G_PARAM_WRITABLE as ::core::ffi::c_int)
        != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"validate_pspec_to_install\0" as *const u8 as *const ::core::ffi::c_char,
            b"pspec->flags & (G_PARAM_READABLE | G_PARAM_WRITABLE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT as ::core::ffi::c_int != 0 {
        if (*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"validate_pspec_to_install\0" as *const u8 as *const ::core::ffi::c_char,
                b"(pspec->flags & G_PARAM_CONSTRUCT_ONLY) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
    }
    if (*pspec).flags as ::core::ffi::c_int
        & (G_PARAM_CONSTRUCT as ::core::ffi::c_int | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
        != 0
    {
        if (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int != 0 {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"validate_pspec_to_install\0" as *const u8 as *const ::core::ffi::c_char,
                b"pspec->flags & G_PARAM_WRITABLE\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn validate_and_install_class_property(
    mut class: *mut GObjectClass,
    mut oclass_type: GType,
    mut parent_type: GType,
    mut property_id: guint,
    mut pspec: *mut GParamSpec,
) -> gboolean {
    if validate_pspec_to_install(pspec) == 0 {
        g_param_spec_ref_sink(pspec);
        g_param_spec_unref(pspec);
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int != 0 {
        if (*class).set_property.is_some() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"validate_and_install_class_property\0" as *const u8 as *const ::core::ffi::c_char,
                b"class->set_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_READABLE as ::core::ffi::c_int != 0 {
        if (*class).get_property.is_some() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"validate_and_install_class_property\0" as *const u8 as *const ::core::ffi::c_char,
                b"class->get_property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
    }
    (*class).flags |= 0x1 as gsize;
    if install_property_internal(oclass_type, property_id, pspec) != 0 {
        if (*pspec).flags as ::core::ffi::c_int
            & (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
            != 0
        {
            (*class).construct_properties =
                g_slist_append((*class).construct_properties, pspec as gpointer);
            (*class).n_construct_properties =
                (*class).n_construct_properties.wrapping_add(1 as gsize);
        }
        pspec = g_param_spec_pool_lookup(
            pspec_pool,
            (*pspec).name,
            parent_type,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
        if !pspec.is_null()
            && (*pspec).flags as ::core::ffi::c_int
                & (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                    | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
                != 0
        {
            (*class).construct_properties =
                g_slist_remove((*class).construct_properties, pspec as gconstpointer);
            (*class).n_construct_properties =
                (*class).n_construct_properties.wrapping_sub(1 as gsize);
        }
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    } else {
        return 0 as gboolean;
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_object_class_install_property(
    mut class: *mut GObjectClass,
    mut property_id: guint,
    mut pspec: *mut GParamSpec,
) {
    let mut oclass_type: GType = 0;
    let mut parent_type: GType = 0;
    if ({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_install_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT_CLASS (class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if property_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_install_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    oclass_type = (*(class as *mut GTypeClass)).g_type;
    parent_type = g_type_parent(oclass_type);
    if (*class).flags & 0x2 as gsize != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Attempt to add property %s::%s to class after it was derived\0" as *const u8
                as *const gchar,
            g_type_name((*(class as *mut GTypeClass)).g_type),
            (*pspec).name,
        );
        loop {}
    }
    validate_and_install_class_property(class, oclass_type, parent_type, property_id, pspec);
}
unsafe extern "C" fn compare_pspec_entry(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ae: *const PspecEntry = a as *const PspecEntry;
    let mut be: *const PspecEntry = b as *const PspecEntry;
    return if (*ae).name < (*be).name {
        -(1 as ::core::ffi::c_int)
    } else if (*ae).name > (*be).name {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn find_pspec(
    mut class: *mut GObjectClass,
    mut property_name: *const ::core::ffi::c_char,
) -> *mut GParamSpec {
    let mut pspecs: *const PspecEntry = (*class).pspecs as *const PspecEntry;
    let mut n_pspecs: gsize = (*class).n_pspecs;
    if n_pspecs <= 9223372036854775807 as ::core::ffi::c_long as gsize {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1193 as ::core::ffi::c_int,
            b"find_pspec\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_pspecs <= G_MAXSSIZE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if n_pspecs < 10 as gsize {
        let mut i: gsize = 0 as gsize;
        while i < n_pspecs {
            if (*pspecs.offset(i as isize)).name == property_name {
                return (*pspecs.offset(i as isize)).pspec;
            }
            i = i.wrapping_add(1);
        }
    } else {
        let mut lower: gssize = 0 as gssize;
        let mut upper: gssize =
            ((*class).n_pspecs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gssize;
        let mut mid: gssize = 0;
        while lower <= upper {
            mid = (lower + upper) / 2 as gssize;
            if property_name < (*pspecs.offset(mid as isize)).name {
                upper = mid - 1 as gssize;
            } else if property_name > (*pspecs.offset(mid as isize)).name {
                lower = mid + 1 as gssize;
            } else {
                return (*pspecs.offset(mid as isize)).pspec;
            }
        }
    }
    return g_param_spec_pool_lookup(
        pspec_pool,
        property_name as *const gchar,
        (*(class as *mut GTypeClass)).g_type,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_class_install_properties(
    mut oclass: *mut GObjectClass,
    mut n_pspecs: guint,
    mut pspecs: *mut *mut GParamSpec,
) {
    let mut oclass_type: GType = 0;
    let mut parent_type: GType = 0;
    let mut i: guint = 0;
    if ({
        let mut __class: *mut GTypeClass = oclass as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_install_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT_CLASS (oclass)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_pspecs > 1 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_install_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_pspecs > 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*pspecs.offset(0 as ::core::ffi::c_int as isize)).is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_install_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"pspecs[0] == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*oclass).flags & 0x2 as gsize != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Attempt to add properties to %s after it was derived\0" as *const u8 as *const gchar,
            g_type_name((*(oclass as *mut GTypeClass)).g_type),
        );
        loop {}
    }
    oclass_type = (*(oclass as *mut GTypeClass)).g_type;
    parent_type = g_type_parent(oclass_type);
    i = 1 as guint;
    while i < n_pspecs {
        let mut pspec: *mut GParamSpec = *pspecs.offset(i as isize);
        if validate_and_install_class_property(oclass, oclass_type, parent_type, i, pspec) == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    if (*oclass).pspecs.is_null() {
        let mut entries: *mut PspecEntry = ::core::ptr::null_mut::<PspecEntry>();
        entries = g_malloc_n(
            n_pspecs.wrapping_sub(1 as guint) as gsize,
            ::core::mem::size_of::<PspecEntry>() as gsize,
        ) as *mut PspecEntry;
        i = 1 as guint;
        while i < n_pspecs {
            let ref mut fresh4 = (*entries.offset(i.wrapping_sub(1 as guint) as isize)).name;
            *fresh4 = (**pspecs.offset(i as isize)).name as *const ::core::ffi::c_char;
            let ref mut fresh5 = (*entries.offset(i.wrapping_sub(1 as guint) as isize)).pspec;
            *fresh5 = *pspecs.offset(i as isize);
            i = i.wrapping_add(1);
        }
        qsort(
            entries as *mut ::core::ffi::c_void,
            n_pspecs.wrapping_sub(1 as guint) as size_t,
            ::core::mem::size_of::<PspecEntry>() as size_t,
            Some(
                compare_pspec_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        (*oclass).pspecs = entries as gpointer;
        (*oclass).n_pspecs = n_pspecs.wrapping_sub(1 as guint) as gsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_interface_install_property(
    mut g_iface: gpointer,
    mut pspec: *mut GParamSpec,
) {
    let mut iface_class: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    if g_type_fundamental((*iface_class).g_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_interface_install_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (iface_class->g_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut __inst: *mut GTypeInstance = pspec as *mut GTypeInstance;
        let mut __t: GType = *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize);
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_interface_install_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"!G_IS_PARAM_SPEC_OVERRIDE (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if validate_pspec_to_install(pspec) == 0 {
        g_param_spec_ref_sink(pspec);
        g_param_spec_unref(pspec);
        return;
    }
    install_property_internal((*iface_class).g_type, 0 as guint, pspec);
}
#[inline]
unsafe extern "C" fn param_spec_follow_override(mut pspec: *mut *mut GParamSpec) {
    if (*(*(*pspec as *mut GTypeInstance)).g_class).g_type
        == *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize)
    {
        *pspec = (*(*pspec as *mut GParamSpecOverride)).overridden;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_class_find_property(
    mut class: *mut GObjectClass,
    mut property_name: *const gchar,
) -> *mut GParamSpec {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if ({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_find_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT_CLASS (class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if !property_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_find_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    pspec = find_pspec(class, property_name as *const ::core::ffi::c_char);
    if !pspec.is_null() {
        param_spec_follow_override(&raw mut pspec);
    }
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_interface_find_property(
    mut g_iface: gpointer,
    mut property_name: *const gchar,
) -> *mut GParamSpec {
    let mut iface_class: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    if g_type_fundamental((*iface_class).g_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_interface_find_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (iface_class->g_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if !property_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_interface_find_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    g_object_init_pspec_pool();
    return g_param_spec_pool_lookup(
        pspec_pool,
        property_name,
        (*iface_class).g_type,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_class_override_property(
    mut oclass: *mut GObjectClass,
    mut property_id: guint,
    mut name: *const gchar,
) {
    let mut overridden: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut new: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut parent_type: GType = 0;
    if ({
        let mut __class: *mut GTypeClass = oclass as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_override_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT_CLASS (oclass)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if property_id > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_override_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_override_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    parent_type = g_type_parent((*(oclass as *mut GTypeClass)).g_type);
    if parent_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
        overridden = g_param_spec_pool_lookup(
            pspec_pool,
            name,
            parent_type,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    }
    if overridden.is_null() {
        let mut ifaces: *mut GType = ::core::ptr::null_mut::<GType>();
        let mut n_ifaces: guint = 0;
        ifaces = g_type_interfaces((*(oclass as *mut GTypeClass)).g_type, &raw mut n_ifaces);
        loop {
            let fresh3 = n_ifaces;
            n_ifaces = n_ifaces.wrapping_sub(1);
            if !(fresh3 != 0 && overridden.is_null()) {
                break;
            }
            overridden = g_param_spec_pool_lookup(
                pspec_pool,
                name,
                *ifaces.offset(n_ifaces as isize),
                0 as gboolean,
            );
        }
        g_free(ifaces as gpointer);
    }
    if overridden.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: Can't find property to override for '%s::%s'\0" as *const u8 as *const gchar,
            b"g_object_class_override_property\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(oclass as *mut GTypeClass)).g_type),
            name,
        );
        return;
    }
    new = g_param_spec_override(name, overridden);
    g_object_class_install_property(oclass, property_id, new);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_class_list_properties(
    mut class: *mut GObjectClass,
    mut n_properties_p: *mut guint,
) -> *mut *mut GParamSpec {
    let mut pspecs: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut n: guint = 0;
    if ({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_class_list_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT_CLASS (class)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut GParamSpec>();
    }
    pspecs = g_param_spec_pool_list(pspec_pool, (*(class as *mut GTypeClass)).g_type, &raw mut n);
    if !n_properties_p.is_null() {
        *n_properties_p = n;
    }
    return pspecs;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_interface_list_properties(
    mut g_iface: gpointer,
    mut n_properties_p: *mut guint,
) -> *mut *mut GParamSpec {
    let mut iface_class: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    let mut pspecs: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut n: guint = 0;
    if g_type_fundamental((*iface_class).g_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_interface_list_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (iface_class->g_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut GParamSpec>();
    }
    g_object_init_pspec_pool();
    pspecs = g_param_spec_pool_list(pspec_pool, (*iface_class).g_type, &raw mut n);
    if !n_properties_p.is_null() {
        *n_properties_p = n;
    }
    return pspecs;
}
#[inline]
unsafe extern "C" fn object_get_optional_flags(mut object: *mut GObject) -> guint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            *object_get_optional_flags_p(object);
            *object_get_optional_flags_p(object);
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(object_get_optional_flags_p(
            object,
        ) as *mut gint);
        gaig_temp
    }) as guint;
}
#[inline]
unsafe extern "C" fn object_set_optional_flags(mut object: *mut GObject, mut flags: guint) {
    if 0 as ::core::ffi::c_int != 0 {
        *object_get_optional_flags_p(object);
    } else {
    };
    crate::translated::compat::atomic_or_seqcst(object_get_optional_flags_p(object), flags);
}
#[inline]
unsafe extern "C" fn object_unset_optional_flags(mut object: *mut GObject, mut flags: guint) {
    if 0 as ::core::ffi::c_int != 0 {
        *object_get_optional_flags_p(object);
        !flags;
    } else {
    };
    crate::translated::compat::atomic_and_seqcst(object_get_optional_flags_p(object), !flags);
}
#[no_mangle]
pub unsafe extern "C" fn _g_object_has_signal_handler(mut object: *mut GObject) -> gboolean {
    return (object_get_optional_flags(object)
        & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as guint
        != 0 as guint) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn _g_object_has_notify_handler(mut object: *mut GObject) -> gboolean {
    return ((*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
        .notify
        .is_some()
        || (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
            .dispatch_properties_changed
            != Some(
                g_object_dispatch_properties_changed
                    as unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
            )
        || object_get_optional_flags(object)
            & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as guint
            != 0 as guint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _g_object_set_has_signal_handler(
    mut object: *mut GObject,
    mut signal_id: guint,
) {
    let mut flags: guint = ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as guint;
    if signal_id as gulong == gobject_signals[NOTIFY as ::core::ffi::c_int as usize] {
        flags |= ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as guint;
    }
    object_set_optional_flags(object, flags);
}
#[inline]
unsafe extern "C" fn object_in_construction(mut object: *mut GObject) -> gboolean {
    return (object_get_optional_flags(object)
        & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as guint
        != 0 as guint) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn set_object_in_construction(mut object: *mut GObject) {
    object_set_optional_flags(
        object,
        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as guint,
    );
}
#[inline]
unsafe extern "C" fn unset_object_in_construction(mut object: *mut GObject) {
    object_unset_optional_flags(
        object,
        ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as guint,
    );
}
unsafe extern "C" fn g_object_init(mut object: *mut GObject, mut class: *mut GObjectClass) {
    (*object).ref_count = 1 as guint;
    (*object).qdata = ::core::ptr::null_mut::<GData>();
    if (*class).flags & 0x1 as gsize != 0
        && ((*class).notify.is_some()
            || (*class).dispatch_properties_changed
                != Some(
                    g_object_dispatch_properties_changed
                        as unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
                ))
    {
        g_object_notify_queue_freeze(object);
    }
    set_object_in_construction(object);
}
unsafe extern "C" fn g_object_do_set_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    match property_id {
        _ => {}
    }
    let mut _glib__object: *mut GObject = object;
    let mut _glib__pspec: *mut GParamSpec = pspec;
    let mut _glib__property_id: guint = property_id;
    g_log(
        b"GLib-GObject\0" as *const u8 as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8 as *const gchar,
        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
            as *const ::core::ffi::c_char,
        1727 as ::core::ffi::c_int,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
        _glib__property_id,
        (*_glib__pspec).name,
        g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
        g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
    );
}
unsafe extern "C" fn g_object_do_get_property(
    mut object: *mut GObject,
    mut property_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    match property_id {
        _ => {}
    }
    let mut _glib__object: *mut GObject = object;
    let mut _glib__pspec: *mut GParamSpec = pspec;
    let mut _glib__property_id: guint = property_id;
    g_log(
        b"GLib-GObject\0" as *const u8 as *const gchar,
        G_LOG_LEVEL_WARNING,
        b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8 as *const gchar,
        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
            as *const ::core::ffi::c_char,
        1741 as ::core::ffi::c_int,
        b"property\0" as *const u8 as *const ::core::ffi::c_char,
        _glib__property_id,
        (*_glib__pspec).name,
        g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
        g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
    );
}
unsafe extern "C" fn g_object_real_dispose(mut object: *mut GObject) {
    g_signal_handlers_destroy(object as gpointer);
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_weak_notifies,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_closure_array,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
}
unsafe extern "C" fn g_object_finalize(mut object: *mut GObject) {
    g_datalist_clear(&raw mut (*object).qdata);
}
unsafe extern "C" fn g_object_dispatch_properties_changed(
    mut object: *mut GObject,
    mut n_pspecs: guint,
    mut pspecs: *mut *mut GParamSpec,
) {
    let mut i: guint = 0;
    i = 0 as guint;
    while i < n_pspecs {
        g_signal_emit(
            object as gpointer,
            gobject_signals[NOTIFY as ::core::ffi::c_int as usize] as guint,
            g_param_spec_get_name_quark(*pspecs.offset(i as isize)),
            *pspecs.offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_run_dispose(mut object: *mut GObject) {
    let mut wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_run_dispose\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_run_dispose\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(object as gpointer);
    (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
    if object_get_optional_flags(object)
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as guint
        != 0
    {
        wrdata = weak_ref_data_get_surely(object);
        weak_ref_data_lock(wrdata);
        weak_ref_data_clear_list(wrdata, object);
        weak_ref_data_unlock(wrdata);
    }
    g_object_unref(object as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_freeze_notify(mut object: *mut GObject) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_freeze_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) <= 0 as ::core::ffi::c_int
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Attempting to freeze the notification queue for object %s[%p]; Property notification does not work during instance finalization.\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            object,
        );
        return;
    }
    g_object_notify_queue_freeze(object);
}
#[inline]
unsafe extern "C" fn g_object_notify_by_spec_internal(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    let mut object_flags: guint = 0;
    let mut needs_notify: gboolean = 0;
    let mut in_init: gboolean = 0;
    if !((*pspec).flags as ::core::ffi::c_int) & G_PARAM_READABLE as ::core::ffi::c_int != 0 {
        return;
    }
    param_spec_follow_override(&raw mut pspec);
    object_flags = object_get_optional_flags(object);
    needs_notify = (object_flags & ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as guint
        != 0 as guint
        || ((*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
            .notify
            .is_some()
            || (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
                .dispatch_properties_changed
                != Some(
                    g_object_dispatch_properties_changed
                        as unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
                ))) as ::core::ffi::c_int as gboolean;
    in_init = (object_flags & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as guint
        != 0 as guint) as ::core::ffi::c_int as gboolean;
    if !pspec.is_null() && needs_notify != 0 {
        if g_object_notify_queue_add(
            object,
            ::core::ptr::null_mut::<GObjectNotifyQueue>(),
            pspec,
            in_init,
        ) == 0
        {
            g_object_ref(object as gpointer);
            (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
                .dispatch_properties_changed
                .expect("non-null function pointer")(object, 1 as guint, &raw mut pspec);
            g_object_unref(object as gpointer);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_notify(
    mut object: *mut GObject,
    mut property_name: *const gchar,
) {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !property_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    pspec = g_param_spec_pool_lookup(
        pspec_pool,
        property_name,
        (*(*(object as *mut GTypeInstance)).g_class).g_type,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: object class '%s' has no property named '%s'\0" as *const u8 as *const gchar,
            b"g_object_notify\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            property_name,
        );
    } else {
        g_object_notify_by_spec_internal(object, pspec);
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_object_notify_by_pspec(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_notify_by_pspec\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_instance_is_fundamentally_a(
        pspec as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_notify_by_pspec\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (pspec)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_notify_by_spec_internal(object, pspec);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_thaw_notify(mut object: *mut GObject) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_thaw_notify\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) <= 0 as ::core::ffi::c_int
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Attempting to thaw the notification queue for object %s[%p]; Property notification does not work during instance finalization.\0"
                as *const u8 as *const gchar,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            object,
        );
        return;
    }
    g_object_notify_queue_thaw(
        object,
        ::core::ptr::null_mut::<GObjectNotifyQueue>(),
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn maybe_issue_property_deprecation_warning(mut pspec: *const GParamSpec) {
    static mut already_warned_table: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    static mut enable_diagnostic: *const gchar = ::core::ptr::null::<gchar>();
    static mut already_warned_lock: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    let mut already: gboolean = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            enable_diagnostic;
        } else {
        };
        (({
            let mut gapg_temp_newval: *const gchar = ::core::ptr::null::<gchar>();
            let mut gapg_temp_atomic: *mut *const gchar = &raw mut enable_diagnostic;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut enable_diagnostic as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut value: *const gchar =
            g_getenv(b"G_ENABLE_DIAGNOSTIC\0" as *const u8 as *const gchar);
        if value.is_null() {
            value = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        if 0 as ::core::ffi::c_int != 0 {
            enable_diagnostic = value;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut enable_diagnostic as *mut ::core::ffi::c_void,
            value as guintptr as gpointer,
        );
    }
    if *enable_diagnostic.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '0' as i32
    {
        return;
    }
    g_mutex_lock(&raw mut already_warned_lock);
    if already_warned_table.is_null() {
        already_warned_table = g_hash_table_new(None, None);
    }
    already = g_hash_table_contains(
        already_warned_table,
        (*pspec).name as gpointer as gconstpointer,
    );
    if already == 0 {
        g_hash_table_add(already_warned_table, (*pspec).name as gpointer);
    }
    g_mutex_unlock(&raw mut already_warned_lock);
    if already == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"The property %s:%s is deprecated and shouldn't be used anymore. It will be removed in a future version.\0"
                as *const u8 as *const gchar,
            g_type_name((*pspec).owner_type),
            (*pspec).name,
        );
    }
}
#[inline]
unsafe extern "C" fn consider_issuing_property_deprecation_warning(mut pspec: *const GParamSpec) {
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_DEPRECATED as ::core::ffi::c_int != 0 {
        maybe_issue_property_deprecation_warning(pspec);
    }
}
#[inline]
unsafe extern "C" fn object_get_property(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) {
    let mut inst: *mut GTypeInstance = object as *mut GTypeInstance;
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut param_id: guint = (*pspec).param_id;
    if (*(*inst).g_class).g_type == (*pspec).owner_type {
        class = (*inst).g_class as *mut GObjectClass;
    } else {
        class = g_type_class_peek((*pspec).owner_type) as *mut GObjectClass;
    }
    if !class.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2131 as ::core::ffi::c_int,
            b"object_get_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    param_spec_follow_override(&raw mut pspec);
    consider_issuing_property_deprecation_warning(pspec);
    (*class).get_property.expect("non-null function pointer")(object, param_id, value, pspec);
}
#[inline]
unsafe extern "C" fn object_set_property(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
    mut nqueue: *mut GObjectNotifyQueue,
    mut user_specified: gboolean,
) {
    let mut inst: *mut GTypeInstance = object as *mut GTypeInstance;
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut pclass: *mut GParamSpecClass = ::core::ptr::null_mut::<GParamSpecClass>();
    let mut param_id: guint = (*pspec).param_id;
    if (*(*inst).g_class).g_type == (*pspec).owner_type {
        class = (*inst).g_class as *mut GObjectClass;
    } else {
        class = g_type_class_peek((*pspec).owner_type) as *mut GObjectClass;
    }
    if !class.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2157 as ::core::ffi::c_int,
            b"object_set_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    param_spec_follow_override(&raw mut pspec);
    if user_specified != 0 {
        consider_issuing_property_deprecation_warning(pspec);
    }
    pclass = (*(pspec as *mut GTypeInstance)).g_class as *mut GParamSpecClass;
    if g_value_type_compatible((*(value as *mut GValue)).g_type, (*pspec).value_type) != 0
        && ((*pclass).value_validate.is_none()
            || (*pclass).value_is_valid.is_some()
                && (*pclass).value_is_valid.expect("non-null function pointer")(pspec, value) != 0)
    {
        (*class).set_property.expect("non-null function pointer")(object, param_id, value, pspec);
    } else {
        let mut tmp_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        g_value_init(&raw mut tmp_value, (*pspec).value_type);
        if g_value_transform(value, &raw mut tmp_value) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"unable to set property '%s' of type '%s' from value of type '%s'\0" as *const u8
                    as *const gchar,
                (*pspec).name,
                g_type_name((*pspec).value_type),
                g_type_name((*(value as *mut GValue)).g_type),
            );
        } else if g_param_value_validate(pspec, &raw mut tmp_value) != 0
            && (*pspec).flags as ::core::ffi::c_int & G_PARAM_LAX_VALIDATION as ::core::ffi::c_int
                == 0
        {
            let mut contents: *mut gchar = g_strdup_value_contents(value);
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"value \"%s\" of type '%s' is invalid or out of range for property '%s' of type '%s'\0"
                    as *const u8 as *const gchar,
                contents,
                g_type_name((*(value as *mut GValue)).g_type),
                (*pspec).name,
                g_type_name((*pspec).value_type),
            );
            g_free(contents as gpointer);
        } else {
            (*class).set_property.expect("non-null function pointer")(
                object,
                param_id,
                &raw mut tmp_value,
                pspec,
            );
        }
        g_value_unset(&raw mut tmp_value);
    }
    if (*pspec).flags as ::core::ffi::c_int
        & (G_PARAM_EXPLICIT_NOTIFY as ::core::ffi::c_int | G_PARAM_READABLE as ::core::ffi::c_int)
        == G_PARAM_READABLE as ::core::ffi::c_int
        && !nqueue.is_null()
    {
        g_object_notify_queue_add(object, nqueue, pspec, 0 as gboolean);
    }
}
unsafe extern "C" fn object_interface_check_properties(
    mut check_data: gpointer,
    mut g_iface: gpointer,
) {
    let mut iface_class: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut iface_type: GType = (*iface_class).g_type;
    let mut pspecs: *mut *mut GParamSpec = ::core::ptr::null_mut::<*mut GParamSpec>();
    let mut n: guint = 0;
    class = g_type_class_ref((*iface_class).g_instance_type) as *mut GObjectClass;
    if class.is_null() {
        return;
    }
    if !(({
        let mut __class: *mut GTypeClass = class as *mut GTypeClass;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __class.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_class_is_a(__class, __t);
        }
        __r
    }) == 0)
    {
        pspecs = g_param_spec_pool_list(pspec_pool, iface_type, &raw mut n);
        loop {
            let fresh49 = n;
            n = n.wrapping_sub(1);
            if !(fresh49 != 0) {
                break;
            }
            let mut class_pspec: *mut GParamSpec = g_param_spec_pool_lookup(
                pspec_pool,
                (**pspecs.offset(n as isize)).name,
                (*(class as *mut GTypeClass)).g_type,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
            if class_pspec.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"Object class %s doesn't implement property '%s' from interface '%s'\0"
                        as *const u8 as *const gchar,
                    g_type_name((*(class as *mut GTypeClass)).g_type),
                    (**pspecs.offset(n as isize)).name,
                    g_type_name(iface_type),
                );
            } else if !((**pspecs.offset(n as isize)).flags as ::core::ffi::c_int
                & !((*class_pspec).flags as ::core::ffi::c_int)
                & (G_PARAM_READABLE as ::core::ffi::c_int | G_PARAM_WRITABLE as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int)
            {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"Flags for property '%s' on class '%s' remove functionality compared with the property on interface '%s'\n\0"
                        as *const u8 as *const gchar,
                    (**pspecs.offset(n as isize)).name,
                    g_type_name((*(class as *mut GTypeClass)).g_type),
                    g_type_name(iface_type),
                );
            } else {
                if (**pspecs.offset(n as isize)).flags as ::core::ffi::c_int
                    & G_PARAM_WRITABLE as ::core::ffi::c_int
                    != 0
                {
                    if !((*class_pspec).flags as ::core::ffi::c_int
                        & !((**pspecs.offset(n as isize)).flags as ::core::ffi::c_int)
                        & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                    {
                        g_log(
                            b"GLib-GObject\0" as *const u8 as *const gchar,
                            G_LOG_LEVEL_CRITICAL,
                            b"Flags for property '%s' on class '%s' introduce additional restrictions on writability compared with the property on interface '%s'\n\0"
                                as *const u8 as *const gchar,
                            (**pspecs.offset(n as isize)).name,
                            g_type_name((*(class as *mut GTypeClass)).g_type),
                            g_type_name(iface_type),
                        );
                        continue;
                    }
                }
                match (**pspecs.offset(n as isize)).flags as ::core::ffi::c_int
                    & (G_PARAM_READABLE as ::core::ffi::c_int
                        | G_PARAM_WRITABLE as ::core::ffi::c_int)
                {
                    3 => {
                        if (**pspecs.offset(n as isize)).value_type != (*class_pspec).value_type {
                            g_log(
                                b"GLib-GObject\0" as *const u8 as *const gchar,
                                G_LOG_LEVEL_CRITICAL,
                                b"Read/writable property '%s' on class '%s' has type '%s' which is not exactly equal to the type '%s' of the property on the interface '%s'\n\0"
                                    as *const u8 as *const gchar,
                                (**pspecs.offset(n as isize)).name,
                                g_type_name((*(class as *mut GTypeClass)).g_type),
                                g_type_name(
                                    (*(class_pspec as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(
                                    (*(*pspecs.offset(n as isize) as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(iface_type),
                            );
                        }
                    }
                    1 => {
                        if !((*class_pspec).value_type == (**pspecs.offset(n as isize)).value_type
                            || g_type_is_a(
                                (*class_pspec).value_type,
                                (**pspecs.offset(n as isize)).value_type,
                            ) != 0)
                        {
                            g_log(
                                b"GLib-GObject\0" as *const u8 as *const gchar,
                                G_LOG_LEVEL_CRITICAL,
                                b"Read-only property '%s' on class '%s' has type '%s' which is not equal to or more restrictive than the type '%s' of the property on the interface '%s'\n\0"
                                    as *const u8 as *const gchar,
                                (**pspecs.offset(n as isize)).name,
                                g_type_name((*(class as *mut GTypeClass)).g_type),
                                g_type_name(
                                    (*(class_pspec as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(
                                    (*(*pspecs.offset(n as isize) as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(iface_type),
                            );
                        }
                    }
                    2 => {
                        if !((**pspecs.offset(n as isize)).value_type == (*class_pspec).value_type
                            || g_type_is_a(
                                (**pspecs.offset(n as isize)).value_type,
                                (*class_pspec).value_type,
                            ) != 0)
                        {
                            g_log(
                                b"GLib-GObject\0" as *const u8 as *const gchar,
                                G_LOG_LEVEL_CRITICAL,
                                b"Write-only property '%s' on class '%s' has type '%s' which is not equal to or less restrictive than the type '%s' of the property on the interface '%s' \n\0"
                                    as *const u8 as *const gchar,
                                (**pspecs.offset(n as isize)).name,
                                g_type_name((*(class as *mut GTypeClass)).g_type),
                                g_type_name(
                                    (*(class_pspec as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(
                                    (*(*pspecs.offset(n as isize) as *mut ::core::ffi::c_void
                                        as *mut GParamSpec))
                                        .value_type,
                                ),
                                g_type_name(iface_type),
                            );
                        }
                    }
                    _ => {
                        g_assertion_message_expr(
                            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2349 as ::core::ffi::c_int,
                            b"object_interface_check_properties\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
            }
        }
        g_free(pspecs as gpointer);
    }
    g_type_class_unref(class as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get_type() -> GType {
    return ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_new(
    mut object_type: GType,
    mut first_property_name: *const gchar,
    mut args: ...
) -> gpointer {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut var_args: ::core::ffi::VaList<'_>;
    if first_property_name.is_null() {
        return g_object_new_with_properties(
            object_type,
            0 as guint,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ::core::ptr::null::<GValue>(),
        ) as gpointer;
    }
    var_args = args.clone();
    object = g_object_new_valist(object_type, first_property_name, var_args);
    return object as gpointer;
}
#[inline]
unsafe extern "C" fn g_object_is_aligned(mut object: *mut GObject) -> gboolean {
    return ((object as *mut ::core::ffi::c_void as guintptr).wrapping_rem(
        (if 8 as ::core::ffi::c_ulong as glong
            > (if 8 as ::core::ffi::c_ulong as glong
                > (if 4 as ::core::ffi::c_ulong as glong > 8 as ::core::ffi::c_ulong as glong {
                    4 as ::core::ffi::c_ulong as glong
                } else {
                    8 as ::core::ffi::c_ulong as glong
                })
            {
                8 as ::core::ffi::c_ulong as glong
            } else {
                (if 4 as ::core::ffi::c_ulong as glong > 8 as ::core::ffi::c_ulong as glong {
                    4 as ::core::ffi::c_ulong as glong
                } else {
                    8 as ::core::ffi::c_ulong as glong
                })
            })
        {
            8 as ::core::ffi::c_ulong as glong
        } else {
            (if 8 as ::core::ffi::c_ulong as glong
                > (if 4 as ::core::ffi::c_ulong as glong > 8 as ::core::ffi::c_ulong as glong {
                    4 as ::core::ffi::c_ulong as glong
                } else {
                    8 as ::core::ffi::c_ulong as glong
                })
            {
                8 as ::core::ffi::c_ulong as glong
            } else {
                (if 4 as ::core::ffi::c_ulong as glong > 8 as ::core::ffi::c_ulong as glong {
                    4 as ::core::ffi::c_ulong as glong
                } else {
                    8 as ::core::ffi::c_ulong as glong
                })
            })
        }) as guintptr,
    ) == 0 as guintptr) as ::core::ffi::c_int;
}
unsafe extern "C" fn g_object_new_with_custom_constructor(
    mut class: *mut GObjectClass,
    mut params: *mut GObjectConstructParam,
    mut n_params: guint,
) -> gpointer {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    let mut newly_constructed: gboolean = 0;
    let mut cparams: *mut GObjectConstructParam = ::core::ptr::null_mut::<GObjectConstructParam>();
    let mut free_cparams: gboolean = 0 as gboolean;
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut cvalues: *mut GValue = ::core::ptr::null_mut::<GValue>();
    let mut cvals_used: gint = 0;
    let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut i: guint = 0;
    if (*class).n_construct_properties < 1024 as gsize {
        cparams = (if (::core::mem::size_of::<GObjectConstructParam>() as usize)
            .wrapping_mul((*class).n_construct_properties as usize)
            == 0 as usize
        {
            ::core::ptr::null_mut::<::core::ffi::c_void>()
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (::core::mem::size_of::<GObjectConstructParam>() as usize)
                    .wrapping_mul((*class).n_construct_properties as usize)
                    as usize,
            ));
            memset(
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<GObjectConstructParam>() as size_t)
                    .wrapping_mul((*class).n_construct_properties as size_t),
            )
        }) as *mut GObjectConstructParam;
        cvalues = (if (::core::mem::size_of::<GValue>() as usize)
            .wrapping_mul((*class).n_construct_properties as usize)
            == 0 as usize
        {
            ::core::ptr::null_mut::<::core::ffi::c_void>()
        } else {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                (::core::mem::size_of::<GValue>() as usize)
                    .wrapping_mul((*class).n_construct_properties as usize)
                    as usize,
            ));
            memset(
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<GValue>() as size_t)
                    .wrapping_mul((*class).n_construct_properties as size_t),
            )
        }) as *mut GValue;
    } else {
        cparams = g_malloc0_n(
            (*class).n_construct_properties,
            ::core::mem::size_of::<GObjectConstructParam>() as gsize,
        ) as *mut GObjectConstructParam;
        cvalues = g_malloc0_n(
            (*class).n_construct_properties,
            ::core::mem::size_of::<GValue>() as gsize,
        ) as *mut GValue;
        free_cparams = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    }
    cvals_used = 0 as ::core::ffi::c_int as gint;
    i = 0 as guint;
    node = (*class).construct_properties;
    while !node.is_null() {
        let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        let mut value: *mut GValue = ::core::ptr::null_mut::<GValue>();
        let mut j: guint = 0;
        pspec = (*node).data as *mut GParamSpec;
        value = ::core::ptr::null_mut::<GValue>();
        j = 0 as guint;
        while j < n_params {
            if (*params.offset(j as isize)).pspec == pspec {
                consider_issuing_property_deprecation_warning(pspec);
                value = (*params.offset(j as isize)).value;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if value.is_null() {
            let fresh25 = cvals_used;
            cvals_used = cvals_used + 1;
            value = cvalues.offset(fresh25 as isize) as *mut GValue;
            g_value_init(value, (*pspec).value_type);
            g_param_value_set_default(pspec, value);
        }
        let ref mut fresh26 = (*cparams.offset(i as isize)).pspec;
        *fresh26 = pspec;
        let ref mut fresh27 = (*cparams.offset(i as isize)).value;
        *fresh27 = value;
        i = i.wrapping_add(1);
        node = (*node).next;
    }
    object = (*class).constructor.expect("non-null function pointer")(
        (*class).g_type_class.g_type,
        (*class).n_construct_properties as guint,
        cparams,
    );
    loop {
        let fresh28 = cvals_used;
        cvals_used = cvals_used - 1;
        if !(fresh28 != 0) {
            break;
        }
        g_value_unset(cvalues.offset(cvals_used as isize) as *mut GValue);
    }
    if free_cparams != 0 {
        g_free(cparams as gpointer);
        g_free(cvalues as gpointer);
    }
    if object.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Custom constructor for class %s returned NULL (which is invalid). Please use GInitable instead.\0"
                as *const u8 as *const gchar,
            g_type_name((*(class as *mut GTypeClass)).g_type),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if g_object_is_aligned(object) == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Custom constructor for class %s returned a non-aligned GObject (which is invalid since GLib 2.72). Assuming any code using this object doesn\xE2\x80\x99t require it to be aligned. Please fix your constructor to align to the largest GLib basic type (typically gdouble or guint64).\0"
                as *const u8 as *const gchar,
            g_type_name((*(class as *mut GTypeClass)).g_type),
        );
    }
    newly_constructed = object_in_construction(object);
    if newly_constructed != 0 {
        unset_object_in_construction(object);
    }
    if (*class).flags & 0x1 as gsize != 0 {
        if newly_constructed != 0 && _g_object_has_notify_handler(object) != 0
            || _g_object_has_notify_handler(object) != 0
        {
            nqueue = g_datalist_id_get_data(&raw mut (*object).qdata, quark_notify_queue)
                as *mut GObjectNotifyQueue;
            if nqueue.is_null() {
                nqueue = g_object_notify_queue_freeze(object);
            }
        }
    }
    if newly_constructed != 0
        && (*class).constructed
            != Some(g_object_constructed as unsafe extern "C" fn(*mut GObject) -> ())
    {
        (*class).constructed.expect("non-null function pointer")(object);
    }
    i = 0 as guint;
    while i < n_params {
        if (*(*params.offset(i as isize)).pspec).flags as ::core::ffi::c_int
            & (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
            == 0
        {
            object_set_property(
                object,
                (*params.offset(i as isize)).pspec,
                (*params.offset(i as isize)).value,
                nqueue,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(1);
    }
    if !nqueue.is_null() {
        g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
    }
    return object as gpointer;
}
unsafe extern "C" fn g_object_new_internal(
    mut class: *mut GObjectClass,
    mut params: *mut GObjectConstructParam,
    mut n_params: guint,
) -> gpointer {
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut i: guint = 0;
    if (*class).constructor
        != Some(
            g_object_constructor
                as unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject,
        )
    {
        return g_object_new_with_custom_constructor(class, params, n_params);
    }
    object = g_type_create_instance((*class).g_type_class.g_type) as *mut GObject;
    if g_object_is_aligned(object) != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2605 as ::core::ffi::c_int,
            b"g_object_new_internal\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_is_aligned (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    unset_object_in_construction(object);
    if (*class).flags & 0x1 as gsize != 0 {
        let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
        if _g_object_has_notify_handler(object) != 0 {
            nqueue = g_datalist_id_get_data(&raw mut (*object).qdata, quark_notify_queue)
                as *mut GObjectNotifyQueue;
            if nqueue.is_null() {
                nqueue = g_object_notify_queue_freeze(object);
            }
        }
        node = (*class).construct_properties;
        while !node.is_null() {
            let mut value: *const GValue = ::core::ptr::null::<GValue>();
            let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
            let mut j: guint = 0;
            let mut user_specified: gboolean = 0 as gboolean;
            pspec = (*node).data as *mut GParamSpec;
            value = ::core::ptr::null::<GValue>();
            j = 0 as guint;
            while j < n_params {
                if (*params.offset(j as isize)).pspec == pspec {
                    value = (*params.offset(j as isize)).value;
                    user_specified =
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if value.is_null() {
                value = g_param_spec_get_default_value(pspec);
            }
            object_set_property(object, pspec, value, nqueue, user_specified);
            node = (*node).next;
        }
    }
    if (*class).constructed
        != Some(g_object_constructed as unsafe extern "C" fn(*mut GObject) -> ())
    {
        (*class).constructed.expect("non-null function pointer")(object);
    }
    i = 0 as guint;
    while i < n_params {
        if (*(*params.offset(i as isize)).pspec).flags as ::core::ffi::c_int
            & (G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
            == 0
        {
            object_set_property(
                object,
                (*params.offset(i as isize)).pspec,
                (*params.offset(i as isize)).value,
                nqueue,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(1);
    }
    if !nqueue.is_null() {
        g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
    }
    return object as gpointer;
}
#[inline]
unsafe extern "C" fn g_object_new_is_valid_property(
    mut object_type: GType,
    mut pspec: *mut GParamSpec,
    mut name: *const ::core::ffi::c_char,
    mut params: *mut GObjectConstructParam,
    mut n_params: guint,
) -> gboolean {
    let mut i: guint = 0;
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: object class '%s' has no property named '%s'\0" as *const u8 as *const gchar,
            b"g_object_new_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name(object_type),
            name,
        );
        return 0 as gboolean;
    }
    if !((*pspec).flags as ::core::ffi::c_int) & G_PARAM_WRITABLE as ::core::ffi::c_int != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: property '%s' of object class '%s' is not writable\0" as *const u8
                as *const gchar,
            b"g_object_new_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            (*pspec).name,
            g_type_name(object_type),
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int
        & (G_PARAM_CONSTRUCT as ::core::ffi::c_int | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int)
        != 0
    {
        i = 0 as guint;
        while i < n_params {
            if (*params.offset(i as isize)).pspec == pspec {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i != n_params {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: property '%s' for type '%s' cannot be set twice\0" as *const u8
                    as *const gchar,
                b"g_object_new_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
                name,
                g_type_name(object_type),
            );
            return 0 as gboolean;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_new_with_properties(
    mut object_type: GType,
    mut n_properties: guint,
    mut names: *mut *const ::core::ffi::c_char,
    mut values: *const GValue,
) -> *mut GObject {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut unref_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if g_type_fundamental(object_type)
        == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_new_with_properties\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_OBJECT (object_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    class = g_type_class_peek_static(object_type) as *mut GObjectClass;
    if class.is_null() {
        unref_class = g_type_class_ref(object_type) as *mut GObjectClass;
        class = unref_class;
    }
    if n_properties > 0 as guint {
        let mut i: guint = 0;
        let mut count: guint = 0 as guint;
        let mut params: *mut GObjectConstructParam =
            ::core::ptr::null_mut::<GObjectConstructParam>();
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<GObjectConstructParam>() as usize)
                .wrapping_mul(n_properties as usize) as usize,
        ));
        params = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut GObjectConstructParam;
        i = 0 as guint;
        while i < n_properties {
            let mut pspec: *mut GParamSpec = find_pspec(class, *names.offset(i as isize));
            if !(g_object_new_is_valid_property(
                object_type,
                pspec,
                *names.offset(i as isize),
                params,
                count,
            ) == 0)
            {
                let ref mut fresh30 = (*params.offset(count as isize)).pspec;
                *fresh30 = pspec;
                let ref mut fresh31 = (*params.offset(count as isize)).value;
                *fresh31 = values.offset(i as isize) as *const GValue as *mut GValue;
                count = count.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        object = g_object_new_internal(class, params, count) as *mut GObject;
    } else {
        object = g_object_new_internal(
            class,
            ::core::ptr::null_mut::<GObjectConstructParam>(),
            0 as guint,
        ) as *mut GObject;
    }
    if !unref_class.is_null() {
        g_type_class_unref(unref_class as gpointer);
    }
    return object;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_newv(
    mut object_type: GType,
    mut n_parameters: guint,
    mut parameters: *mut GParameter,
) -> gpointer {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut unref_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if g_type_fundamental(object_type)
        == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_OBJECT (object_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if n_parameters == 0 as guint || !parameters.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_newv\0" as *const u8 as *const ::core::ffi::c_char,
            b"n_parameters == 0 || parameters != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    class = g_type_class_peek_static(object_type) as *mut GObjectClass;
    if class.is_null() {
        unref_class = g_type_class_ref(object_type) as *mut GObjectClass;
        class = unref_class;
    }
    if n_parameters != 0 {
        let mut cparams: *mut GObjectConstructParam =
            ::core::ptr::null_mut::<GObjectConstructParam>();
        let mut i: guint = 0;
        let mut j: guint = 0;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<GObjectConstructParam>() as usize)
                .wrapping_mul(n_parameters as usize) as usize,
        ));
        cparams = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut GObjectConstructParam;
        j = 0 as guint;
        i = 0 as guint;
        while i < n_parameters {
            let mut pspec: *mut GParamSpec = find_pspec(
                class,
                (*parameters.offset(i as isize)).name as *const ::core::ffi::c_char,
            );
            if !(g_object_new_is_valid_property(
                object_type,
                pspec,
                (*parameters.offset(i as isize)).name as *const ::core::ffi::c_char,
                cparams,
                j,
            ) == 0)
            {
                let ref mut fresh32 = (*cparams.offset(j as isize)).pspec;
                *fresh32 = pspec;
                let ref mut fresh33 = (*cparams.offset(j as isize)).value;
                *fresh33 = &raw mut (*parameters.offset(i as isize)).value;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        object = g_object_new_internal(class, cparams, j) as *mut GObject;
    } else {
        object = g_object_new_internal(
            class,
            ::core::ptr::null_mut::<GObjectConstructParam>(),
            0 as guint,
        ) as *mut GObject;
    }
    if !unref_class.is_null() {
        g_type_class_unref(unref_class as gpointer);
    }
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_new_valist(
    mut object_type: GType,
    mut first_property_name: *const gchar,
    mut var_args: ::core::ffi::VaList,
) -> *mut GObject {
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut unref_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if g_type_fundamental(object_type)
        == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_new_valist\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_OBJECT (object_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GObject>();
    }
    class = g_type_class_peek_static(object_type) as *mut GObjectClass;
    if class.is_null() {
        unref_class = g_type_class_ref(object_type) as *mut GObjectClass;
        class = unref_class;
    }
    if !first_property_name.is_null() {
        let mut params_stack: [GObjectConstructParam; 16] = [_GObjectConstructParam {
            pspec: ::core::ptr::null_mut::<GParamSpec>(),
            value: ::core::ptr::null_mut::<GValue>(),
        }; 16];
        let mut values_stack: [GValue; 16] = [_GValue {
            g_type: 0,
            data: [C2RustUnnamed { v_int: 0 }; 2],
        }; 16];
        let mut vtabs_stack: [*mut GTypeValueTable; 16] =
            [::core::ptr::null_mut::<GTypeValueTable>(); 16];
        let mut name: *const gchar = ::core::ptr::null::<gchar>();
        let mut params: *mut GObjectConstructParam =
            &raw mut params_stack as *mut GObjectConstructParam;
        let mut values: *mut GValue = &raw mut values_stack as *mut GValue;
        let mut vtabs: *mut *mut GTypeValueTable =
            &raw mut vtabs_stack as *mut *mut GTypeValueTable;
        let mut n_params: guint = 0 as guint;
        let mut n_params_alloc: guint = (::core::mem::size_of::<[GObjectConstructParam; 16]>()
            as usize)
            .wrapping_div(::core::mem::size_of::<GObjectConstructParam>() as usize)
            as guint;
        name = first_property_name;
        loop {
            let mut error: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut pspec: *mut GParamSpec = find_pspec(class, name as *const ::core::ffi::c_char);
            if g_object_new_is_valid_property(
                object_type,
                pspec,
                name as *const ::core::ffi::c_char,
                params,
                n_params,
            ) == 0
            {
                break;
            }
            if n_params == n_params_alloc {
                let mut i: guint = 0;
                if n_params_alloc as usize
                    == (::core::mem::size_of::<[GObjectConstructParam; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<GObjectConstructParam>() as usize)
                {
                    n_params_alloc = (::core::mem::size_of::<[GObjectConstructParam; 16]>()
                        as usize)
                        .wrapping_div(::core::mem::size_of::<GObjectConstructParam>() as usize)
                        .wrapping_mul(2 as usize) as guint;
                    params = g_malloc_n(
                        n_params_alloc as gsize,
                        ::core::mem::size_of::<GObjectConstructParam>() as gsize,
                    ) as *mut GObjectConstructParam;
                    values = g_malloc_n(
                        n_params_alloc as gsize,
                        ::core::mem::size_of::<GValue>() as gsize,
                    ) as *mut GValue;
                    vtabs = g_malloc_n(
                        n_params_alloc as gsize,
                        ::core::mem::size_of::<*mut GTypeValueTable>() as gsize,
                    ) as *mut *mut GTypeValueTable;
                    memcpy(
                        params as *mut ::core::ffi::c_void,
                        &raw mut params_stack as *mut GObjectConstructParam
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<GObjectConstructParam>() as size_t)
                            .wrapping_mul(n_params as size_t),
                    );
                    memcpy(
                        values as *mut ::core::ffi::c_void,
                        &raw mut values_stack as *mut GValue as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<GValue>() as size_t)
                            .wrapping_mul(n_params as size_t),
                    );
                    memcpy(
                        vtabs as *mut ::core::ffi::c_void,
                        &raw mut vtabs_stack as *mut *mut GTypeValueTable
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<*mut GTypeValueTable>() as size_t)
                            .wrapping_mul(n_params as size_t),
                    );
                } else {
                    n_params_alloc = (n_params_alloc as ::core::ffi::c_uint)
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        as guint as guint;
                    params = g_realloc(
                        params as gpointer,
                        (::core::mem::size_of::<GObjectConstructParam>() as gsize)
                            .wrapping_mul(n_params_alloc as gsize),
                    ) as *mut GObjectConstructParam;
                    values = g_realloc(
                        values as gpointer,
                        (::core::mem::size_of::<GValue>() as gsize)
                            .wrapping_mul(n_params_alloc as gsize),
                    ) as *mut GValue;
                    vtabs = g_realloc(
                        vtabs as gpointer,
                        (::core::mem::size_of::<*mut GTypeValueTable>() as gsize)
                            .wrapping_mul(n_params_alloc as gsize),
                    ) as *mut *mut GTypeValueTable;
                }
                i = 0 as guint;
                while i < n_params {
                    let ref mut fresh6 = (*params.offset(i as isize)).value;
                    *fresh6 = values.offset(i as isize) as *mut GValue;
                    i = i.wrapping_add(1);
                }
            }
            let ref mut fresh7 = (*params.offset(n_params as isize)).pspec;
            *fresh7 = pspec;
            let ref mut fresh8 = (*params.offset(n_params as isize)).value;
            *fresh8 = values.offset(n_params as isize) as *mut GValue;
            memset(
                values.offset(n_params as isize) as *mut GValue as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<GValue>() as size_t,
            );
            let mut g_vci_val: *mut GValue = values.offset(n_params as isize) as *mut GValue;
            let mut g_vci_flags: guint =
                ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
            let mut g_vci_collect_format: *const gchar = ::core::ptr::null::<gchar>();
            let mut g_vci_cvalues: [GTypeCValue; 8] = [
                _GTypeCValue {
                    v_int: 0 as ::core::ffi::c_int,
                },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
                _GTypeCValue { v_int: 0 },
            ];
            let mut g_vci_n_values: guint = 0 as guint;
            let ref mut fresh9 = *vtabs.offset(n_params as isize);
            *fresh9 = g_type_value_table_peek((*pspec).value_type);
            g_vci_collect_format = (**vtabs.offset(n_params as isize)).collect_format;
            (*g_vci_val).g_type = (*pspec).value_type;
            while *g_vci_collect_format != 0 {
                let fresh10 = g_vci_n_values;
                g_vci_n_values = g_vci_n_values.wrapping_add(1);
                let mut g_vci_cvalue: *mut GTypeCValue =
                    (&raw mut g_vci_cvalues as *mut GTypeCValue).offset(fresh10 as isize);
                let fresh11 = g_vci_collect_format;
                g_vci_collect_format = g_vci_collect_format.offset(1);
                match *fresh11 as ::core::ffi::c_int {
                    105 => {
                        (*g_vci_cvalue).v_int = var_args.arg::<gint>();
                    }
                    108 => {
                        (*g_vci_cvalue).v_long = var_args.arg::<glong>();
                    }
                    113 => {
                        (*g_vci_cvalue).v_int64 = var_args.arg::<gint64>();
                    }
                    100 => {
                        (*g_vci_cvalue).v_double = var_args.arg::<gdouble>();
                    }
                    112 => {
                        (*g_vci_cvalue).v_pointer = var_args.arg::<gpointer>();
                    }
                    _ => {
                        g_assertion_message_expr(
                            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2928 as ::core::ffi::c_int,
                            b"g_object_new_valist\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
            }
            error = (**vtabs.offset(n_params as isize))
                .collect_value
                .expect("non-null function pointer")(
                g_vci_val,
                g_vci_n_values,
                &raw mut g_vci_cvalues as *mut GTypeCValue,
                g_vci_flags,
            );
            if !error.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"%s: %s\0" as *const u8 as *const gchar,
                    b"g_object_new_valist\0" as *const u8 as *const ::core::ffi::c_char,
                    error,
                );
                g_value_unset(values.offset(n_params as isize) as *mut GValue);
                g_free(error as gpointer);
                break;
            } else {
                n_params = n_params.wrapping_add(1);
                name = var_args.arg::<*const gchar>();
                if name.is_null() {
                    break;
                }
            }
        }
        object = g_object_new_internal(class, params, n_params) as *mut GObject;
        loop {
            let fresh12 = n_params;
            n_params = n_params.wrapping_sub(1);
            if !(fresh12 != 0) {
                break;
            }
            if (**vtabs.offset(n_params as isize)).value_free.is_some() {
                (**vtabs.offset(n_params as isize))
                    .value_free
                    .expect("non-null function pointer")(
                    (*params.offset(n_params as isize)).value
                );
            }
        }
        if n_params_alloc as usize
            != (::core::mem::size_of::<[GObjectConstructParam; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<GObjectConstructParam>() as usize)
        {
            g_free(params as gpointer);
            g_free(values as gpointer);
            g_free(vtabs as gpointer);
        }
    } else {
        object = g_object_new_internal(
            class,
            ::core::ptr::null_mut::<GObjectConstructParam>(),
            0 as guint,
        ) as *mut GObject;
    }
    if !unref_class.is_null() {
        g_type_class_unref(unref_class as gpointer);
    }
    return object;
}
unsafe extern "C" fn g_object_constructor(
    mut type_0: GType,
    mut n_construct_properties: guint,
    mut construct_params: *mut GObjectConstructParam,
) -> *mut GObject {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    object = g_type_create_instance(type_0) as *mut GObject;
    if n_construct_properties != 0 {
        let mut nqueue: *mut GObjectNotifyQueue = g_object_notify_queue_freeze(object);
        loop {
            let fresh29 = n_construct_properties;
            n_construct_properties = n_construct_properties.wrapping_sub(1);
            if !(fresh29 != 0) {
                break;
            }
            let mut value: *mut GValue = (*construct_params).value;
            let mut pspec: *mut GParamSpec = (*construct_params).pspec;
            construct_params = construct_params.offset(1);
            object_set_property(
                object,
                pspec,
                value,
                nqueue,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        }
        g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
    }
    return object;
}
unsafe extern "C" fn g_object_constructed(mut object: *mut GObject) {}
#[inline]
unsafe extern "C" fn g_object_set_is_valid_property(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut property_name: *const ::core::ffi::c_char,
) -> gboolean {
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: object class '%s' has no property named '%s'\0" as *const u8 as *const gchar,
            b"g_object_set_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            property_name,
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: property '%s' of object class '%s' is not writable\0" as *const u8
                as *const gchar,
            b"g_object_set_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            (*pspec).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int != 0
        && object_in_construction(object) == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: construct property \"%s\" for object '%s' can't be set after construction\0"
                as *const u8 as *const gchar,
            b"g_object_set_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            (*pspec).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_setv(
    mut object: *mut GObject,
    mut n_properties: guint,
    mut names: *mut *const gchar,
    mut values: *const GValue,
) {
    let mut i: guint = 0;
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_setv\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_properties == 0 as guint {
        return;
    }
    g_object_ref(object as gpointer);
    class = (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass;
    if _g_object_has_notify_handler(object) != 0 {
        nqueue = g_object_notify_queue_freeze(object);
    }
    i = 0 as guint;
    while i < n_properties {
        pspec = find_pspec(
            class,
            *names.offset(i as isize) as *const ::core::ffi::c_char,
        );
        if g_object_set_is_valid_property(
            object,
            pspec,
            *names.offset(i as isize) as *const ::core::ffi::c_char,
        ) == 0
        {
            break;
        }
        object_set_property(
            object,
            pspec,
            values.offset(i as isize) as *const GValue,
            nqueue,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    if !nqueue.is_null() {
        g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
    }
    g_object_unref(object as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_valist(
    mut object: *mut GObject,
    mut first_property_name: *const gchar,
    mut var_args: ::core::ffi::VaList,
) {
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_valist\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(object as gpointer);
    if _g_object_has_notify_handler(object) != 0 {
        nqueue = g_object_notify_queue_freeze(object);
    }
    class = (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass;
    name = first_property_name;
    while !name.is_null() {
        let mut value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        let mut error: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut vtab: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
        pspec = find_pspec(class, name as *const ::core::ffi::c_char);
        if g_object_set_is_valid_property(object, pspec, name as *const ::core::ffi::c_char) == 0 {
            break;
        }
        let mut g_vci_val: *mut GValue = &raw mut value;
        let mut g_vci_flags: guint =
            ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
        let mut g_vci_collect_format: *const gchar = ::core::ptr::null::<gchar>();
        let mut g_vci_cvalues: [GTypeCValue; 8] = [
            _GTypeCValue {
                v_int: 0 as ::core::ffi::c_int,
            },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
        ];
        let mut g_vci_n_values: guint = 0 as guint;
        vtab = g_type_value_table_peek((*pspec).value_type);
        g_vci_collect_format = (*vtab).collect_format;
        (*g_vci_val).g_type = (*pspec).value_type;
        while *g_vci_collect_format != 0 {
            let fresh34 = g_vci_n_values;
            g_vci_n_values = g_vci_n_values.wrapping_add(1);
            let mut g_vci_cvalue: *mut GTypeCValue =
                (&raw mut g_vci_cvalues as *mut GTypeCValue).offset(fresh34 as isize);
            let fresh35 = g_vci_collect_format;
            g_vci_collect_format = g_vci_collect_format.offset(1);
            match *fresh35 as ::core::ffi::c_int {
                105 => {
                    (*g_vci_cvalue).v_int = var_args.arg::<gint>();
                }
                108 => {
                    (*g_vci_cvalue).v_long = var_args.arg::<glong>();
                }
                113 => {
                    (*g_vci_cvalue).v_int64 = var_args.arg::<gint64>();
                }
                100 => {
                    (*g_vci_cvalue).v_double = var_args.arg::<gdouble>();
                }
                112 => {
                    (*g_vci_cvalue).v_pointer = var_args.arg::<gpointer>();
                }
                _ => {
                    g_assertion_message_expr(
                        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        3129 as ::core::ffi::c_int,
                        b"g_object_set_valist\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        error = (*vtab).collect_value.expect("non-null function pointer")(
            g_vci_val,
            g_vci_n_values,
            &raw mut g_vci_cvalues as *mut GTypeCValue,
            g_vci_flags,
        );
        if !error.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: %s\0" as *const u8 as *const gchar,
                b"g_object_set_valist\0" as *const u8 as *const ::core::ffi::c_char,
                error,
            );
            g_free(error as gpointer);
            g_value_unset(&raw mut value);
            break;
        } else {
            object_set_property(
                object,
                pspec,
                &raw mut value,
                nqueue,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
            if (*vtab).value_free.is_some() {
                (*vtab).value_free.expect("non-null function pointer")(&raw mut value);
            }
            name = var_args.arg::<*mut gchar>();
        }
    }
    if !nqueue.is_null() {
        g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
    }
    g_object_unref(object as gpointer);
}
#[inline]
unsafe extern "C" fn g_object_get_is_valid_property(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut property_name: *const ::core::ffi::c_char,
) -> gboolean {
    if pspec.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: object class '%s' has no property named '%s'\0" as *const u8 as *const gchar,
            b"g_object_get_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            property_name,
        );
        return 0 as gboolean;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_READABLE as ::core::ffi::c_int == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: property '%s' of object class '%s' is not readable\0" as *const u8
                as *const gchar,
            b"g_object_get_is_valid_property\0" as *const u8 as *const ::core::ffi::c_char,
            (*pspec).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_getv(
    mut object: *mut GObject,
    mut n_properties: guint,
    mut names: *mut *const gchar,
    mut values: *mut GValue,
) {
    let mut i: guint = 0;
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_getv\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n_properties == 0 as guint {
        return;
    }
    g_object_ref(object as gpointer);
    class = (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass;
    memset(
        values as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (n_properties as size_t).wrapping_mul(::core::mem::size_of::<GValue>() as size_t),
    );
    i = 0 as guint;
    while i < n_properties {
        pspec = find_pspec(
            class,
            *names.offset(i as isize) as *const ::core::ffi::c_char,
        );
        if g_object_get_is_valid_property(
            object,
            pspec,
            *names.offset(i as isize) as *const ::core::ffi::c_char,
        ) == 0
        {
            break;
        }
        g_value_init(
            values.offset(i as isize) as *mut GValue,
            (*pspec).value_type,
        );
        object_get_property(object, pspec, values.offset(i as isize) as *mut GValue);
        i = i.wrapping_add(1);
    }
    g_object_unref(object as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get_valist(
    mut object: *mut GObject,
    mut first_property_name: *const gchar,
    mut var_args: ::core::ffi::VaList,
) {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_valist\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(object as gpointer);
    class = (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass;
    name = first_property_name;
    while !name.is_null() {
        let mut value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        let mut error: *mut gchar = ::core::ptr::null_mut::<gchar>();
        pspec = find_pspec(class, name as *const ::core::ffi::c_char);
        if g_object_get_is_valid_property(object, pspec, name as *const ::core::ffi::c_char) == 0 {
            break;
        }
        g_value_init(&raw mut value, (*pspec).value_type);
        object_get_property(object, pspec, &raw mut value);
        let mut g_vl_value: *const GValue = &raw mut value;
        let mut g_vl_flags: guint = 0 as guint;
        let mut g_vl_value_type: GType = (*(g_vl_value as *mut GValue)).g_type;
        let mut g_vl_vtable: *mut GTypeValueTable = g_type_value_table_peek(g_vl_value_type);
        let mut g_vl_lcopy_format: *const gchar = (*g_vl_vtable).lcopy_format;
        let mut g_vl_cvalues: [GTypeCValue; 8] = [
            _GTypeCValue {
                v_int: 0 as ::core::ffi::c_int,
            },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
            _GTypeCValue { v_int: 0 },
        ];
        let mut g_vl_n_values: guint = 0 as guint;
        while *g_vl_lcopy_format != 0 {
            let fresh36 = g_vl_n_values;
            g_vl_n_values = g_vl_n_values.wrapping_add(1);
            let mut g_vl_cvalue: *mut GTypeCValue =
                (&raw mut g_vl_cvalues as *mut GTypeCValue).offset(fresh36 as isize);
            let fresh37 = g_vl_lcopy_format;
            g_vl_lcopy_format = g_vl_lcopy_format.offset(1);
            match *fresh37 as ::core::ffi::c_int {
                105 => {
                    (*g_vl_cvalue).v_int = var_args.arg::<gint>();
                }
                108 => {
                    (*g_vl_cvalue).v_long = var_args.arg::<glong>();
                }
                113 => {
                    (*g_vl_cvalue).v_int64 = var_args.arg::<gint64>();
                }
                100 => {
                    (*g_vl_cvalue).v_double = var_args.arg::<gdouble>();
                }
                112 => {
                    (*g_vl_cvalue).v_pointer = var_args.arg::<gpointer>();
                }
                _ => {
                    g_assertion_message_expr(
                        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        3268 as ::core::ffi::c_int,
                        b"g_object_get_valist\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        error = (*g_vl_vtable)
            .lcopy_value
            .expect("non-null function pointer")(
            g_vl_value,
            g_vl_n_values,
            &raw mut g_vl_cvalues as *mut GTypeCValue,
            g_vl_flags,
        );
        if !error.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: %s\0" as *const u8 as *const gchar,
                b"g_object_get_valist\0" as *const u8 as *const ::core::ffi::c_char,
                error,
            );
            g_free(error as gpointer);
            g_value_unset(&raw mut value);
            break;
        } else {
            g_value_unset(&raw mut value);
            name = var_args.arg::<*mut gchar>();
        }
    }
    g_object_unref(object as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set(
    mut _object: gpointer,
    mut first_property_name: *const gchar,
    mut args: ...
) {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut var_args: ::core::ffi::VaList<'_>;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    var_args = args.clone();
    g_object_set_valist(object, first_property_name, var_args);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get(
    mut _object: gpointer,
    mut first_property_name: *const gchar,
    mut args: ...
) {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut var_args: ::core::ffi::VaList<'_>;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    var_args = args.clone();
    g_object_get_valist(object, first_property_name, var_args);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_property(
    mut object: *mut GObject,
    mut property_name: *const gchar,
    mut value: *const GValue,
) {
    g_object_setv(
        object,
        1 as guint,
        &raw mut property_name,
        value as *const GValue,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get_property(
    mut object: *mut GObject,
    mut property_name: *const gchar,
    mut value: *mut GValue,
) {
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !property_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_property\0" as *const u8 as *const ::core::ffi::c_char,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(object as gpointer);
    pspec = find_pspec(
        (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass,
        property_name as *const ::core::ffi::c_char,
    );
    if g_object_get_is_valid_property(object, pspec, property_name as *const ::core::ffi::c_char)
        != 0
    {
        let mut prop_value: *mut GValue = ::core::ptr::null_mut::<GValue>();
        let mut tmp_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        if (*value).g_type == ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
            g_value_init(value, (*pspec).value_type);
            prop_value = value;
        } else if (*value).g_type == (*pspec).value_type {
            g_value_reset(value);
            prop_value = value;
        } else if g_value_type_transformable((*pspec).value_type, (*value).g_type) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: can't retrieve property '%s' of type '%s' as value of type '%s'\0"
                    as *const u8 as *const gchar,
                b"g_object_get_property\0" as *const u8 as *const ::core::ffi::c_char,
                (*pspec).name,
                g_type_name((*pspec).value_type),
                g_type_name((*value).g_type),
            );
            g_object_unref(object as gpointer);
            return;
        } else {
            g_value_init(&raw mut tmp_value, (*pspec).value_type);
            prop_value = &raw mut tmp_value;
        }
        object_get_property(object, pspec, prop_value);
        if prop_value != value {
            g_value_transform(prop_value, value);
            g_value_unset(&raw mut tmp_value);
        }
    }
    g_object_unref(object as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_connect(
    mut _object: gpointer,
    mut signal_spec: *const gchar,
    mut args: ...
) -> gpointer {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut var_args: ::core::ffi::VaList<'_>;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*object).ref_count > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_connect\0" as *const u8 as *const ::core::ffi::c_char,
            b"object->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return object as gpointer;
    }
    var_args = args.clone();
    while !signal_spec.is_null() {
        let mut callback: GCallback =
            ::core::mem::transmute(var_args.arg::<*mut unsafe extern "C" fn() -> ()>());
        let mut data: gpointer = var_args.arg::<gpointer>();
        if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"signal::\0" as *const u8 as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_data(
                object as gpointer,
                signal_spec.offset(8 as ::core::ffi::c_int as isize),
                callback,
                data,
                None,
                G_CONNECT_DEFAULT,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"object_signal::\0" as *const u8 as *const ::core::ffi::c_char,
            15 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"object-signal::\0" as *const u8 as *const ::core::ffi::c_char,
                15 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_object(
                object as gpointer,
                signal_spec.offset(15 as ::core::ffi::c_int as isize),
                callback,
                data,
                G_CONNECT_DEFAULT,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"swapped_signal::\0" as *const u8 as *const ::core::ffi::c_char,
            16 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"swapped-signal::\0" as *const u8 as *const ::core::ffi::c_char,
                16 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_data(
                object as gpointer,
                signal_spec.offset(16 as ::core::ffi::c_int as isize),
                callback,
                data,
                None,
                G_CONNECT_SWAPPED,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"swapped_object_signal::\0" as *const u8 as *const ::core::ffi::c_char,
            23 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"swapped-object-signal::\0" as *const u8 as *const ::core::ffi::c_char,
                23 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_object(
                object as gpointer,
                signal_spec.offset(23 as ::core::ffi::c_int as isize),
                callback,
                data,
                G_CONNECT_SWAPPED,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"signal_after::\0" as *const u8 as *const ::core::ffi::c_char,
            14 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"signal-after::\0" as *const u8 as *const ::core::ffi::c_char,
                14 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_data(
                object as gpointer,
                signal_spec.offset(14 as ::core::ffi::c_int as isize),
                callback,
                data,
                None,
                G_CONNECT_AFTER,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"object_signal_after::\0" as *const u8 as *const ::core::ffi::c_char,
            21 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"object-signal-after::\0" as *const u8 as *const ::core::ffi::c_char,
                21 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_object(
                object as gpointer,
                signal_spec.offset(21 as ::core::ffi::c_int as isize),
                callback,
                data,
                G_CONNECT_AFTER,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"swapped_signal_after::\0" as *const u8 as *const ::core::ffi::c_char,
            22 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"swapped-signal-after::\0" as *const u8 as *const ::core::ffi::c_char,
                22 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_data(
                object as gpointer,
                signal_spec.offset(22 as ::core::ffi::c_int as isize),
                callback,
                data,
                None,
                (G_CONNECT_SWAPPED as ::core::ffi::c_int | G_CONNECT_AFTER as ::core::ffi::c_int)
                    as GConnectFlags,
            );
        } else if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"swapped_object_signal_after::\0" as *const u8 as *const ::core::ffi::c_char,
            29 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"swapped-object-signal-after::\0" as *const u8 as *const ::core::ffi::c_char,
                29 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            g_signal_connect_object(
                object as gpointer,
                signal_spec.offset(29 as ::core::ffi::c_int as isize),
                callback,
                data,
                (G_CONNECT_SWAPPED as ::core::ffi::c_int | G_CONNECT_AFTER as ::core::ffi::c_int)
                    as GConnectFlags,
            );
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: invalid signal spec \"%s\"\0" as *const u8 as *const gchar,
                b"g_object_connect\0" as *const u8 as *const ::core::ffi::c_char,
                signal_spec,
            );
            break;
        }
        signal_spec = var_args.arg::<*mut gchar>();
    }
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_disconnect(
    mut _object: gpointer,
    mut signal_spec: *const gchar,
    mut args: ...
) {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut var_args: ::core::ffi::VaList<'_>;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*object).ref_count > 0 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
            b"object->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    var_args = args.clone();
    while !signal_spec.is_null() {
        let mut callback: GCallback =
            ::core::mem::transmute(var_args.arg::<*mut unsafe extern "C" fn() -> ()>());
        let mut data: gpointer = var_args.arg::<gpointer>();
        let mut sid: guint = 0 as guint;
        let mut detail: guint = 0 as guint;
        let mut mask: guint = 0 as guint;
        if strncmp(
            signal_spec as *const ::core::ffi::c_char,
            b"any_signal::\0" as *const u8 as *const ::core::ffi::c_char,
            12 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                signal_spec as *const ::core::ffi::c_char,
                b"any-signal::\0" as *const u8 as *const ::core::ffi::c_char,
                12 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            signal_spec = signal_spec.offset(12 as ::core::ffi::c_int as isize);
            mask = (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
                | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
                | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int) as guint;
        } else if strcmp(
            signal_spec as *const ::core::ffi::c_char,
            b"any_signal\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                signal_spec as *const ::core::ffi::c_char,
                b"any-signal\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            signal_spec = signal_spec.offset(10 as ::core::ffi::c_int as isize);
            mask = (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int
                | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int) as guint;
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: invalid signal spec \"%s\"\0" as *const u8 as *const gchar,
                b"g_object_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
                signal_spec,
            );
            break;
        }
        if mask & G_SIGNAL_MATCH_ID as ::core::ffi::c_int as guint != 0
            && g_signal_parse_name(
                signal_spec,
                (*(*(object as *mut GTypeInstance)).g_class).g_type,
                &raw mut sid,
                &raw mut detail,
                0 as gboolean,
            ) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: invalid signal name \"%s\"\0" as *const u8 as *const gchar,
                b"g_object_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
                signal_spec,
            );
        } else if g_signal_handlers_disconnect_matched(
            object as gpointer,
            (mask
                | (if detail != 0 {
                    G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as guint) as GSignalMatchType,
            sid,
            detail as GQuark,
            ::core::ptr::null_mut::<GClosure>(),
            ::core::mem::transmute::<GCallback, gpointer>(callback),
            data,
        ) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: signal handler %p(%p) is not connected\0" as *const u8 as *const gchar,
                b"g_object_disconnect\0" as *const u8 as *const ::core::ffi::c_char,
                callback,
                data,
            );
        }
        signal_spec = var_args.arg::<*mut gchar>();
    }
}
unsafe extern "C" fn weak_refs_notify(mut data: gpointer) {
    let mut wstack: *mut WeakRefStack = data as *mut WeakRefStack;
    let mut i: guint = 0;
    i = 0 as guint;
    while i < (*wstack).n_weak_refs {
        (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize))
            .notify
            .expect("non-null function pointer")(
            (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize)).data,
            (*wstack).object,
        );
        i = i.wrapping_add(1);
    }
    g_free(wstack as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_weak_ref(
    mut object: *mut GObject,
    mut notify: GWeakNotify,
    mut data: gpointer,
) {
    let mut wstack: *mut WeakRefStack = ::core::ptr::null_mut::<WeakRefStack>();
    let mut i: guint = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_weak_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_weak_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) >= 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_weak_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) >= 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    object_bit_lock(object, 1 as guint);
    wstack = g_datalist_id_remove_no_notify(&raw mut (*object).qdata, quark_weak_notifies)
        as *mut WeakRefStack;
    if !wstack.is_null() {
        let fresh41 = (*wstack).n_weak_refs;
        (*wstack).n_weak_refs = (*wstack).n_weak_refs.wrapping_add(1);
        i = fresh41;
        wstack = g_realloc(
            wstack as gpointer,
            (::core::mem::size_of::<WeakRefStack>() as gsize).wrapping_add(
                (::core::mem::size_of::<C2RustUnnamed_3>() as gsize).wrapping_mul(i as gsize),
            ),
        ) as *mut WeakRefStack;
    } else {
        wstack = g_realloc_n(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            1 as gsize,
            ::core::mem::size_of::<WeakRefStack>() as gsize,
        ) as *mut WeakRefStack;
        (*wstack).object = object;
        (*wstack).n_weak_refs = 1 as guint;
        i = 0 as guint;
    }
    let ref mut fresh42 =
        (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize)).notify;
    *fresh42 = notify;
    let ref mut fresh43 =
        (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize)).data;
    *fresh43 = data;
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_weak_notifies,
        wstack as gpointer,
        Some(weak_refs_notify as unsafe extern "C" fn(gpointer) -> ()),
    );
    object_bit_unlock(object, 1 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_weak_unref(
    mut object: *mut GObject,
    mut notify: GWeakNotify,
    mut data: gpointer,
) {
    let mut wstack: *mut WeakRefStack = ::core::ptr::null_mut::<WeakRefStack>();
    let mut found_one: gboolean = 0 as gboolean;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_weak_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_weak_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    object_bit_lock(object, 1 as guint);
    wstack =
        g_datalist_id_get_data(&raw mut (*object).qdata, quark_weak_notifies) as *mut WeakRefStack;
    if !wstack.is_null() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*wstack).n_weak_refs {
            if (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize)).notify
                == notify
                && (*(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize)).data
                    == data
            {
                found_one = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                (*wstack).n_weak_refs = (*wstack).n_weak_refs.wrapping_sub(1 as guint);
                if i != (*wstack).n_weak_refs {
                    *(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3).offset(i as isize) =
                        *(&raw mut (*wstack).weak_refs as *mut C2RustUnnamed_3)
                            .offset((*wstack).n_weak_refs as isize);
                }
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    object_bit_unlock(object, 1 as guint);
    if found_one == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: couldn't find weak ref %p(%p)\0" as *const u8 as *const gchar,
            b"g_object_weak_unref\0" as *const u8 as *const ::core::ffi::c_char,
            notify,
            data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_add_weak_pointer(
    mut object: *mut GObject,
    mut weak_pointer_location: *mut gpointer,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_add_weak_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !weak_pointer_location.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_add_weak_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"weak_pointer_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_weak_ref(
        object,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut gpointer) -> ()>, GWeakNotify>(
            Some(g_nullify_pointer as unsafe extern "C" fn(*mut gpointer) -> ()),
        ),
        weak_pointer_location as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_remove_weak_pointer(
    mut object: *mut GObject,
    mut weak_pointer_location: *mut gpointer,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_remove_weak_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !weak_pointer_location.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_remove_weak_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"weak_pointer_location != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_weak_unref(
        object,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut gpointer) -> ()>, GWeakNotify>(
            Some(g_nullify_pointer as unsafe extern "C" fn(*mut gpointer) -> ()),
        ),
        weak_pointer_location as gpointer,
    );
}
unsafe extern "C" fn object_floating_flag_handler(
    mut object: *mut GObject,
    mut job: gint,
) -> guint {
    let mut oldvalue: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    match job {
        1 => {
            oldvalue = ({
                let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
                let mut gapg_temp_atomic: *mut *mut GData = &raw mut (*object).qdata;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) as gpointer;
            while ({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).qdata as *mut *mut ::core::ffi::c_void);
                } else {
                };
                if 0 as ::core::ffi::c_int != 0 {
                    oldvalue;
                } else {
                };
                oldvalue = oldvalue;
                let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).qdata as *mut *mut ::core::ffi::c_void,
                    *&raw mut oldvalue,
                    (oldvalue as guintptr | 0x2 as guintptr) as *mut ::core::ffi::c_void,
                );
                *&raw mut oldvalue = fresh0.0;
                if fresh0.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {}
            return (oldvalue as gsize & 0x2 as gsize) as guint;
        }
        -1 => {
            oldvalue = ({
                let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
                let mut gapg_temp_atomic: *mut *mut GData = &raw mut (*object).qdata;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) as gpointer;
            while ({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).qdata as *mut *mut ::core::ffi::c_void);
                } else {
                };
                if 0 as ::core::ffi::c_int != 0 {
                    oldvalue;
                } else {
                };
                oldvalue = oldvalue;
                let fresh1 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).qdata as *mut *mut ::core::ffi::c_void,
                    *&raw mut oldvalue,
                    (oldvalue as gsize & !(0x2 as ::core::ffi::c_int as gsize))
                        as *mut ::core::ffi::c_void,
                );
                *&raw mut oldvalue = fresh1.0;
                if fresh1.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {}
            return (oldvalue as gsize & 0x2 as gsize) as guint;
        }
        _ => {
            return (0 as gsize
                != ({
                    let mut gapg_temp_newval: *mut GData = ::core::ptr::null_mut::<GData>();
                    let mut gapg_temp_atomic: *mut *mut GData = &raw mut (*object).qdata;
                    *&raw mut gapg_temp_newval =
                        crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                    gapg_temp_newval
                }) as gsize
                    & 0x2 as gsize) as ::core::ffi::c_int as guint;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_object_is_floating(mut _object: gpointer) -> gboolean {
    let mut object: *mut GObject = _object as *mut GObject;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_is_floating\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return floating_flag_handler.expect("non-null function pointer")(object, 0 as gint)
        as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_ref_sink(mut _object: gpointer) -> gpointer {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut was_floating: gboolean = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_ref_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return object as gpointer;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) >= 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_ref_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) >= 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return object as gpointer;
    }
    g_object_ref(object as gpointer);
    was_floating =
        floating_flag_handler.expect("non-null function pointer")(object, -(1 as gint)) as gboolean;
    if was_floating != 0 {
        g_object_unref(object as gpointer);
    }
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_take_ref(mut _object: gpointer) -> gpointer {
    let mut object: *mut GObject = _object as *mut GObject;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_take_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return object as gpointer;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) >= 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_take_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) >= 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return object as gpointer;
    }
    floating_flag_handler.expect("non-null function pointer")(object, -(1 as gint));
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_force_floating(mut object: *mut GObject) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_force_floating\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) >= 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_force_floating\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) >= 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    floating_flag_handler.expect("non-null function pointer")(object, 1 as gint);
}
unsafe extern "C" fn toggle_refs_get_notify_unlocked(
    mut object: *mut GObject,
    mut out_data: *mut gpointer,
) -> GToggleNotify {
    let mut tstackptr: *mut ToggleRefStack = ::core::ptr::null_mut::<ToggleRefStack>();
    if !(g_datalist_get_flags(&raw mut (*object).qdata) & 0x1 as guint != 0 as guint) {
        return None;
    }
    tstackptr =
        g_datalist_id_get_data(&raw mut (*object).qdata, quark_toggle_refs) as *mut ToggleRefStack;
    if (*tstackptr).n_toggle_refs != 1 as guint {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Unexpected number of toggle-refs. g_object_add_toggle_ref() must be paired with g_object_remove_toggle_ref()\0"
                as *const u8 as *const gchar,
        );
        return None;
    }
    *out_data = (*(&raw mut (*tstackptr).toggle_refs as *mut C2RustUnnamed_2)
        .offset(0 as ::core::ffi::c_int as isize))
    .data;
    return (*(&raw mut (*tstackptr).toggle_refs as *mut C2RustUnnamed_2)
        .offset(0 as ::core::ffi::c_int as isize))
    .notify;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_add_toggle_ref(
    mut object: *mut GObject,
    mut notify: GToggleNotify,
    mut data: gpointer,
) {
    let mut tstack: *mut ToggleRefStack = ::core::ptr::null_mut::<ToggleRefStack>();
    let mut i: guint = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_add_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_add_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) >= 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_add_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) >= 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(object as gpointer);
    object_bit_lock(object, 3 as guint);
    tstack = g_datalist_id_remove_no_notify(&raw mut (*object).qdata, quark_toggle_refs)
        as *mut ToggleRefStack;
    if !tstack.is_null() {
        let fresh44 = (*tstack).n_toggle_refs;
        (*tstack).n_toggle_refs = (*tstack).n_toggle_refs.wrapping_add(1);
        i = fresh44;
        tstack = g_realloc(
            tstack as gpointer,
            (::core::mem::size_of::<ToggleRefStack>() as gsize).wrapping_add(
                (::core::mem::size_of::<C2RustUnnamed_2>() as gsize).wrapping_mul(i as gsize),
            ),
        ) as *mut ToggleRefStack;
    } else {
        tstack = g_realloc_n(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            1 as gsize,
            ::core::mem::size_of::<ToggleRefStack>() as gsize,
        ) as *mut ToggleRefStack;
        (*tstack).n_toggle_refs = 1 as guint;
        i = 0 as guint;
    }
    if (*tstack).n_toggle_refs == 1 as guint {
        g_datalist_set_flags(&raw mut (*object).qdata, 0x1 as guint);
    }
    let ref mut fresh45 =
        (*(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2).offset(i as isize)).notify;
    *fresh45 = notify;
    let ref mut fresh46 =
        (*(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2).offset(i as isize)).data;
    *fresh46 = data;
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_toggle_refs,
        tstack as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
    object_bit_unlock(object, 3 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_remove_toggle_ref(
    mut object: *mut GObject,
    mut notify: GToggleNotify,
    mut data: gpointer,
) {
    let mut tstack: *mut ToggleRefStack = ::core::ptr::null_mut::<ToggleRefStack>();
    let mut found_one: gboolean = 0 as gboolean;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_remove_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_remove_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    object_bit_lock(object, 3 as guint);
    tstack =
        g_datalist_id_get_data(&raw mut (*object).qdata, quark_toggle_refs) as *mut ToggleRefStack;
    if !tstack.is_null() {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*tstack).n_toggle_refs {
            if (*(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2).offset(i as isize)).notify
                == notify
                && ((*(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2).offset(i as isize))
                    .data
                    == data
                    || data.is_null())
            {
                found_one = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
                (*tstack).n_toggle_refs = (*tstack).n_toggle_refs.wrapping_sub(1 as guint);
                if i != (*tstack).n_toggle_refs {
                    *(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2).offset(i as isize) =
                        *(&raw mut (*tstack).toggle_refs as *mut C2RustUnnamed_2)
                            .offset((*tstack).n_toggle_refs as isize);
                }
                if (*tstack).n_toggle_refs == 0 as guint {
                    g_datalist_unset_flags(&raw mut (*object).qdata, 0x1 as guint);
                    g_datalist_id_set_data_full(
                        &raw mut (*object).qdata,
                        quark_toggle_refs,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        None,
                    );
                }
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    object_bit_unlock(object, 3 as guint);
    if found_one != 0 {
        g_object_unref(object as gpointer);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: couldn't find toggle ref %p(%p)\0" as *const u8 as *const gchar,
            b"g_object_remove_toggle_ref\0" as *const u8 as *const ::core::ffi::c_char,
            notify,
            data,
        );
    };
}
unsafe extern "C" fn object_ref(
    mut object: *mut GObject,
    mut out_toggle_notify: *mut GToggleNotify,
    mut out_toggle_data: *mut gpointer,
) -> gpointer {
    let mut toggle_notify: GToggleNotify = None;
    let mut toggle_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut old_ref: gint = 0;
    old_ref = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    });
    loop {
        toggle_notify = None;
        toggle_data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        if old_ref > 1 as ::core::ffi::c_int && old_ref < 2147483647 as ::core::ffi::c_int {
            if !(({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh23 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh23.0;
                if fresh23.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0)
            {
                break;
            }
        } else if old_ref == 1 as ::core::ffi::c_int {
            let mut do_retry: gboolean = 0;
            object_bit_lock(object, 3 as guint);
            toggle_notify = toggle_refs_get_notify_unlocked(object, &raw mut toggle_data);
            do_retry = (({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh24 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh24.0;
                if fresh24.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0) as ::core::ffi::c_int as gboolean;
            object_bit_unlock(object, 3 as guint);
            if !(do_retry != 0) {
                break;
            }
        } else {
            let mut object_already_finalized: gboolean =
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
            *out_toggle_notify = None;
            *out_toggle_data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            if object_already_finalized == 0 {
            } else {
                g_return_if_fail_warning(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"object_ref\0" as *const u8 as *const ::core::ffi::c_char,
                    b"!object_already_finalized\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    *out_toggle_notify = toggle_notify;
    *out_toggle_data = toggle_data;
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_ref(mut _object: gpointer) -> gpointer {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut toggle_notify: GToggleNotify = None;
    let mut toggle_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    object = object_ref(object, &raw mut toggle_notify, &raw mut toggle_data) as *mut GObject;
    if toggle_notify.is_some() {
        toggle_notify.expect("non-null function pointer")(toggle_data, object, 0 as gboolean);
    }
    return object as gpointer;
}
unsafe extern "C" fn _object_unref_clear_weak_locations(
    mut object: *mut GObject,
    mut p_old_ref: *mut gint,
    mut do_unref: gboolean,
) -> gboolean {
    let mut wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    let mut success: gboolean = 0;
    if object_get_optional_flags(object)
        & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as guint
        == 0
    {
        if do_unref != 0 {
            ({
                let mut gaicae_oldval: gint = 1 as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut gint);
                } else {
                };
                let fresh19 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut gint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    0 as ::core::ffi::c_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh19.0;
                if fresh19.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0;
        }
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    wrdata = weak_ref_data_get_surely(object);
    weak_ref_data_lock(wrdata);
    if do_unref != 0 {
        success = ({
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*object).ref_count as *mut gint);
                *p_old_ref;
            } else {
            };
            *p_old_ref = 1 as ::core::ffi::c_int as gint;
            let fresh20 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*object).ref_count as *mut gint,
                *p_old_ref,
                0 as ::core::ffi::c_int,
            );
            *p_old_ref = fresh20.0;
            if fresh20.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gboolean;
    } else {
        *p_old_ref = ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*object).ref_count as *mut gint);
                *(&raw mut (*object).ref_count as *mut gint);
            } else {
            };
            *&raw mut gaig_temp =
                crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
            gaig_temp
        });
        success = (*p_old_ref == 1 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    }
    if success != 0 {
        weak_ref_data_clear_list(wrdata, object);
    }
    weak_ref_data_unlock(wrdata);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_unref(mut _object: gpointer) {
    let mut object: *mut GObject = _object as *mut GObject;
    let mut old_ref: gint = 0;
    let mut toggle_notify: GToggleNotify = None;
    let mut toggle_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut nqueue: *mut GObjectNotifyQueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
    let mut do_retry: gboolean = 0;
    let mut obj_gtype: GType = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    obj_gtype = (*(*(object as *mut GTypeInstance)).g_class).g_type;
    old_ref = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    });
    loop {
        if old_ref > 2 as ::core::ffi::c_int {
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh15 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh15.0;
                if fresh15.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {
                continue;
            }
            return;
        } else if old_ref == 2 as ::core::ffi::c_int {
            object_bit_lock(object, 3 as guint);
            toggle_notify = toggle_refs_get_notify_unlocked(object, &raw mut toggle_data);
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh16 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh16.0;
                if fresh16.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {
                object_bit_unlock(object, 3 as guint);
            } else {
                object_bit_unlock(object, 3 as guint);
                if toggle_notify.is_some() {
                    toggle_notify.expect("non-null function pointer")(
                        toggle_data,
                        object,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                }
                return;
            }
        } else {
            if old_ref != 1 as ::core::ffi::c_int {
                let mut object_already_finalized: gboolean =
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
                if object_already_finalized == 0 {
                } else {
                    g_return_if_fail_warning(
                        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                        b"g_object_unref\0" as *const u8 as *const ::core::ffi::c_char,
                        b"!object_already_finalized\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return;
                }
                return;
            }
            if _object_unref_clear_weak_locations(object, &raw mut old_ref, 0 as gboolean) == 0 {
                continue;
            }
            nqueue = g_object_notify_queue_freeze(object);
            (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
                .dispose
                .expect("non-null function pointer")(object);
            old_ref = ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    (*object).ref_count;
                    (*object).ref_count;
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*object).ref_count as *mut gint,
                );
                gaig_temp
            });
            break;
        }
    }
    loop {
        if old_ref > 1 as ::core::ffi::c_int && !nqueue.is_null() {
            g_object_notify_queue_thaw(object, nqueue, 0 as gboolean);
            nqueue = ::core::ptr::null_mut::<GObjectNotifyQueue>();
        }
        if old_ref > 2 as ::core::ffi::c_int {
            if ({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh17 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh17.0;
                if fresh17.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0
            {
                continue;
            }
            return;
        } else if old_ref == 2 as ::core::ffi::c_int {
            object_bit_lock(object, 3 as guint);
            toggle_notify = toggle_refs_get_notify_unlocked(object, &raw mut toggle_data);
            do_retry = (({
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*object).ref_count as *mut ::core::ffi::c_int);
                    old_ref;
                } else {
                };
                old_ref = old_ref;
                let fresh18 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*object).ref_count as *mut ::core::ffi::c_int,
                    *&raw mut old_ref,
                    old_ref as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                );
                *&raw mut old_ref = fresh18.0;
                if fresh18.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) == 0) as ::core::ffi::c_int as gboolean;
            object_bit_unlock(object, 3 as guint);
            if do_retry != 0 {
                continue;
            }
            if toggle_notify.is_some() {
                toggle_notify.expect("non-null function pointer")(
                    toggle_data,
                    object,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            }
            return;
        } else {
            if _object_unref_clear_weak_locations(
                object,
                &raw mut old_ref,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            ) == 0
            {
                continue;
            }
            g_datalist_id_set_data_full(
                &raw mut (*object).qdata,
                quark_closure_array,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            g_signal_handlers_destroy(object as gpointer);
            g_datalist_id_set_data_full(
                &raw mut (*object).qdata,
                quark_weak_notifies,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            (*((*(object as *mut GTypeInstance)).g_class as *mut GObjectClass))
                .finalize
                .expect("non-null function pointer")(object);
            g_type_free_instance(object as *mut GTypeInstance);
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_clear_object(mut object_ptr: *mut *mut GObject) {
    let mut _pp: *mut *mut GObject = object_ptr as *mut *mut GObject;
    let mut _ptr: *mut GObject = *_pp;
    *_pp = ::core::ptr::null_mut::<GObject>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get_qdata(
    mut object: *mut GObject,
    mut quark: GQuark,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if quark != 0 {
        g_datalist_id_get_data(&raw mut (*object).qdata, quark)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_qdata(
    mut object: *mut GObject,
    mut quark: GQuark,
    mut data: gpointer,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(&raw mut (*object).qdata, quark, data, None);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_dup_qdata(
    mut object: *mut GObject,
    mut quark: GQuark,
    mut dup_func: GDuplicateFunc,
    mut user_data: gpointer,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_dup_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_dup_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_datalist_id_dup_data(&raw mut (*object).qdata, quark, dup_func, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_replace_qdata(
    mut object: *mut GObject,
    mut quark: GQuark,
    mut oldval: gpointer,
    mut newval: gpointer,
    mut destroy: GDestroyNotify,
    mut old_destroy: *mut GDestroyNotify,
) -> gboolean {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_replace_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_replace_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_datalist_id_replace_data(
        &raw mut (*object).qdata,
        quark,
        oldval,
        newval,
        destroy,
        old_destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_qdata_full(
    mut object: *mut GObject,
    mut quark: GQuark,
    mut data: gpointer,
    mut destroy: GDestroyNotify,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_qdata_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_qdata_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
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
pub unsafe extern "C" fn g_object_steal_qdata(
    mut object: *mut GObject,
    mut quark: GQuark,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_steal_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if quark > 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_steal_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_datalist_id_remove_no_notify(&raw mut (*object).qdata, quark);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_get_data(
    mut object: *mut GObject,
    mut key: *const gchar,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_get_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_datalist_get_data(&raw mut (*object).qdata, key);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_data(
    mut object: *mut GObject,
    mut key: *const gchar,
    mut data: gpointer,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        g_quark_from_string(key),
        data,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_dup_data(
    mut object: *mut GObject,
    mut key: *const gchar,
    mut dup_func: GDuplicateFunc,
    mut user_data: gpointer,
) -> gpointer {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_dup_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_dup_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return g_datalist_id_dup_data(
        &raw mut (*object).qdata,
        g_quark_from_string(key),
        dup_func,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_replace_data(
    mut object: *mut GObject,
    mut key: *const gchar,
    mut oldval: gpointer,
    mut newval: gpointer,
    mut destroy: GDestroyNotify,
    mut old_destroy: *mut GDestroyNotify,
) -> gboolean {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_replace_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_replace_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_datalist_id_replace_data(
        &raw mut (*object).qdata,
        g_quark_from_string(key),
        oldval,
        newval,
        destroy,
        old_destroy,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_object_set_data_full(
    mut object: *mut GObject,
    mut key: *const gchar,
    mut data: gpointer,
    mut destroy: GDestroyNotify,
) {
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_data_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_set_data_full\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        g_quark_from_string(key),
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
pub unsafe extern "C" fn g_object_steal_data(
    mut object: *mut GObject,
    mut key: *const gchar,
) -> gpointer {
    let mut quark: GQuark = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_steal_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !key.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_steal_data\0" as *const u8 as *const ::core::ffi::c_char,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    quark = g_quark_try_string(key);
    return if quark != 0 {
        g_datalist_id_remove_no_notify(&raw mut (*object).qdata, quark)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
unsafe extern "C" fn g_value_object_init(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn g_value_object_free_value(mut value: *mut GValue) {
    g_clear_object(
        &raw mut (*(&raw mut (*value).data as *mut C2RustUnnamed)
            .offset(0 as ::core::ffi::c_int as isize))
        .v_pointer as *mut *mut GObject,
    );
}
unsafe extern "C" fn g_value_object_copy_value(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    let mut _object_ptr: C2RustUnnamed_4 = C2RustUnnamed_4 {
        in_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    _object_ptr.in_0 = &raw mut (*(&raw mut (*dest_value).data as *mut C2RustUnnamed)
        .offset(0 as ::core::ffi::c_int as isize))
    .v_pointer as *mut *mut GObject as *mut ::core::ffi::c_char;
    if 0 as ::core::ffi::c_int != 0 {
        let ref mut fresh50 = *(&raw mut (*(&raw mut (*dest_value).data as *mut C2RustUnnamed)
            .offset(0 as ::core::ffi::c_int as isize))
        .v_pointer as *mut *mut GObject);
        *fresh50 = (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GObject;
    } else {
    };
    g_set_object(
        _object_ptr.out,
        (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GObject,
    );
}
unsafe extern "C" fn g_value_object_transform_value(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    if !(*src_value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && ((*(*((*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer
            as *mut GTypeInstance))
            .g_class)
            .g_type
            == (*dest_value).g_type
            || g_type_is_a(
                (*(*((*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer
                    as *mut GTypeInstance))
                    .g_class)
                    .g_type,
                (*dest_value).g_type,
            ) != 0)
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_object_ref((*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer);
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    };
}
unsafe extern "C" fn g_value_object_peek_pointer(mut value: *const GValue) -> gpointer {
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
unsafe extern "C" fn g_value_object_collect_value(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if !(*collect_values.offset(0 as ::core::ffi::c_int as isize))
        .v_pointer
        .is_null()
    {
        let mut object: *mut GObject =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut GObject;
        if (*object).g_type_instance.g_class.is_null() {
            return g_strconcat(
                b"invalid unclassed object pointer for value type '\0" as *const u8 as *const gchar,
                g_type_name((*value).g_type),
                b"'\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        } else if g_value_type_compatible(
            (*(*(object as *mut GTypeInstance)).g_class).g_type,
            (*value).g_type,
        ) == 0
        {
            return g_strconcat(
                b"invalid object type '\0" as *const u8 as *const gchar,
                g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
                b"' for value type '\0" as *const u8 as *const ::core::ffi::c_char,
                g_type_name((*value).g_type),
                b"'\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_object_ref(object as gpointer) as *mut GObject as gpointer;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn g_value_object_lcopy_value(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut object_p: *mut *mut GObject =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut *mut GObject;
    if !object_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_object_lcopy_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"object_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
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
        *object_p = ::core::ptr::null_mut::<GObject>();
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        *object_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GObject;
    } else {
        *object_p =
            g_object_ref((*value).data[0 as ::core::ffi::c_int as usize].v_pointer) as *mut GObject;
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_object(mut value: *mut GValue, mut v_object: gpointer) {
    let mut old: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_set_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_OBJECT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value).data[0 as ::core::ffi::c_int as usize].v_pointer == v_object {
        return;
    }
    old = g_steal_pointer(
        &raw mut (*(&raw mut (*value).data as *mut C2RustUnnamed)
            .offset(0 as ::core::ffi::c_int as isize))
        .v_pointer as gpointer,
    ) as *mut GObject;
    if !v_object.is_null() {
        if g_type_check_instance_is_fundamentally_a(
            v_object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_set_object\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_OBJECT (v_object)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if g_value_type_compatible(
            (*(*(v_object as *mut GTypeInstance)).g_class).g_type,
            (*value).g_type,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_set_object\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_type_compatible (G_OBJECT_TYPE (v_object), G_VALUE_TYPE (value))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_object_ref(v_object) as gpointer;
    }
    g_clear_object(&raw mut old);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_object_take_ownership(
    mut value: *mut GValue,
    mut v_object: gpointer,
) {
    g_value_take_object(value, v_object);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_take_object(mut value: *mut GValue, mut v_object: gpointer) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_take_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_OBJECT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_clear_object(
        &raw mut (*(&raw mut (*value).data as *mut C2RustUnnamed)
            .offset(0 as ::core::ffi::c_int as isize))
        .v_pointer as *mut *mut GObject,
    );
    if !v_object.is_null() {
        if g_type_check_instance_is_fundamentally_a(
            v_object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_take_object\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_OBJECT (v_object)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if g_value_type_compatible(
            (*(*(v_object as *mut GTypeInstance)).g_class).g_type,
            (*value).g_type,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_take_object\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_type_compatible (G_OBJECT_TYPE (v_object), G_VALUE_TYPE (value))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_steal_pointer(&raw mut v_object as gpointer) as gpointer;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_object(mut value: *const GValue) -> gpointer {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_get_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_OBJECT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_dup_object(mut value: *const GValue) -> gpointer {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
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
            b"g_value_dup_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_OBJECT (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_object_ref((*value).data[0 as ::core::ffi::c_int as usize].v_pointer)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_connect_object(
    mut instance: gpointer,
    mut detailed_signal: *const gchar,
    mut c_handler: GCallback,
    mut gobject: gpointer,
    mut connect_flags: GConnectFlags,
) -> gulong {
    if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !detailed_signal.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"detailed_signal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if c_handler.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_connect_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"c_handler != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gulong;
    }
    if !gobject.is_null() {
        let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
        if g_type_check_instance_is_fundamentally_a(
            gobject as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_signal_connect_object\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_IS_OBJECT (gobject)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gulong;
        }
        closure = if connect_flags as ::core::ffi::c_uint
            & G_CONNECT_SWAPPED as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            Some(
                g_cclosure_new_object_swap
                    as unsafe extern "C" fn(GCallback, *mut GObject) -> *mut GClosure,
            )
        } else {
            Some(
                g_cclosure_new_object
                    as unsafe extern "C" fn(GCallback, *mut GObject) -> *mut GClosure,
            )
        }
        .expect("non-null function pointer")(c_handler, gobject as *mut GObject);
        return g_signal_connect_closure(
            instance,
            detailed_signal,
            closure,
            (connect_flags as ::core::ffi::c_uint
                & G_CONNECT_AFTER as ::core::ffi::c_int as ::core::ffi::c_uint)
                as gboolean,
        );
    } else {
        return g_signal_connect_data(
            instance,
            detailed_signal,
            c_handler,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            connect_flags,
        );
    };
}
unsafe extern "C" fn object_remove_closure(mut data: gpointer, mut closure: *mut GClosure) {
    let mut object: *mut GObject = data as *mut GObject;
    let mut carray: *mut CArray = ::core::ptr::null_mut::<CArray>();
    let mut i: guint = 0;
    object_bit_lock(object, 4 as guint);
    carray = g_object_get_qdata(object, quark_closure_array) as *mut CArray;
    i = 0 as guint;
    while i < (*carray).n_closures {
        if *(&raw mut (*carray).closures as *mut *mut GClosure).offset(i as isize) == closure {
            (*carray).n_closures = (*carray).n_closures.wrapping_sub(1);
            if i < (*carray).n_closures {
                let ref mut fresh40 =
                    *(&raw mut (*carray).closures as *mut *mut GClosure).offset(i as isize);
                *fresh40 = *(&raw mut (*carray).closures as *mut *mut GClosure)
                    .offset((*carray).n_closures as isize);
            }
            object_bit_unlock(object, 4 as guint);
            return;
        }
        i = i.wrapping_add(1);
    }
    object_bit_unlock(object, 4 as guint);
    g_assertion_message_expr(
        b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gobject.c\0" as *const u8
            as *const ::core::ffi::c_char,
        5184 as ::core::ffi::c_int,
        b"object_remove_closure\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn destroy_closure_array(mut data: gpointer) {
    let mut carray: *mut CArray = data as *mut CArray;
    let mut object: *mut GObject = (*carray).object;
    let mut i: guint = 0;
    let mut n: guint = (*carray).n_closures;
    i = 0 as guint;
    while i < n {
        let mut closure: *mut GClosure =
            *(&raw mut (*carray).closures as *mut *mut GClosure).offset(i as isize);
        g_closure_remove_invalidate_notifier(
            closure,
            object as gpointer,
            Some(object_remove_closure as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
        );
        g_closure_invalidate(closure);
        i = i.wrapping_add(1);
    }
    g_free(carray as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn g_object_watch_closure(
    mut object: *mut GObject,
    mut closure: *mut GClosure,
) {
    let mut carray: *mut CArray = ::core::ptr::null_mut::<CArray>();
    let mut i: guint = 0;
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_watch_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_watch_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_watch_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->is_invalid == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).in_marshal() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_watch_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->in_marshal == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_object_watch_closure\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_closure_add_invalidate_notifier(
        closure,
        object as gpointer,
        Some(object_remove_closure as unsafe extern "C" fn(gpointer, *mut GClosure) -> ()),
    );
    g_closure_add_marshal_guards(
        closure,
        object as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GClosureNotify>(
            Some(g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer),
        ),
        object as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GClosureNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
    object_bit_lock(object, 4 as guint);
    carray = g_datalist_id_remove_no_notify(&raw mut (*object).qdata, quark_closure_array)
        as *mut CArray;
    if carray.is_null() {
        carray = g_realloc_n(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            1 as gsize,
            ::core::mem::size_of::<CArray>() as gsize,
        ) as *mut CArray;
        (*carray).object = object;
        (*carray).n_closures = 1 as guint;
        i = 0 as guint;
    } else {
        let fresh38 = (*carray).n_closures;
        (*carray).n_closures = (*carray).n_closures.wrapping_add(1);
        i = fresh38;
        carray = g_realloc(
            carray as gpointer,
            (::core::mem::size_of::<CArray>() as gsize).wrapping_add(
                (::core::mem::size_of::<*mut GClosure>() as gsize).wrapping_mul(i as gsize),
            ),
        ) as *mut CArray;
    }
    let ref mut fresh39 = *(&raw mut (*carray).closures as *mut *mut GClosure).offset(i as isize);
    *fresh39 = closure;
    g_datalist_id_set_data_full(
        &raw mut (*object).qdata,
        quark_closure_array,
        carray as gpointer,
        Some(destroy_closure_array as unsafe extern "C" fn(gpointer) -> ()),
    );
    object_bit_unlock(object, 4 as guint);
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_new_object(
    mut sizeof_closure: guint,
    mut object: *mut GObject,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_new_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_new_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_closure_new_simple(sizeof_closure, object as gpointer);
    g_object_watch_closure(object, closure);
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_new_object(
    mut callback_func: GCallback,
    mut object: *mut GObject,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if callback_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"callback_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_cclosure_new(callback_func, object as gpointer, None);
    g_object_watch_closure(object, closure);
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_new_object_swap(
    mut callback_func: GCallback,
    mut object: *mut GObject,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if g_type_check_instance_is_fundamentally_a(
        object as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object_swap\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*object).ref_count;
            (*object).ref_count;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*object).ref_count as *mut gint);
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object_swap\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_atomic_int_get (&object->ref_count) > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if callback_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_object_swap\0" as *const u8 as *const ::core::ffi::c_char,
            b"callback_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_cclosure_new_swap(callback_func, object as gpointer, None);
    g_object_watch_closure(object, closure);
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn g_object_compat_control(mut what: gsize, mut data: gpointer) -> gsize {
    let mut pp: *mut gpointer = ::core::ptr::null_mut::<gpointer>();
    match what {
        1 => return g_initially_unowned_get_type(),
        2 => {
            floating_flag_handler = ::core::mem::transmute::<
                gpointer,
                Option<unsafe extern "C" fn(*mut GObject, gint) -> guint>,
            >(data);
            return 1 as gsize;
        }
        3 => {
            pp = data as *mut gpointer;
            *pp = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GObject, gint) -> guint>,
                gpointer,
            >(floating_flag_handler);
            return 1 as gsize;
        }
        _ => return 0 as gsize,
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_initially_unowned_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_initially_unowned_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
static mut g_initially_unowned_parent_class: gpointer =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
static mut GInitiallyUnowned_private_offset: gint = 0;
unsafe extern "C" fn g_initially_unowned_class_intern_init(mut klass: gpointer) {
    g_initially_unowned_parent_class = g_type_class_peek_parent(klass);
    if GInitiallyUnowned_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut GInitiallyUnowned_private_offset);
    }
    g_initially_unowned_class_init(klass as *mut GInitiallyUnownedClass);
}
#[inline(never)]
unsafe extern "C" fn g_initially_unowned_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GInitiallyUnowned\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GInitiallyUnownedClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_initially_unowned_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GInitiallyUnowned>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GInitiallyUnowned) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                g_initially_unowned_init as unsafe extern "C" fn(*mut GInitiallyUnowned) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn g_initially_unowned_init(mut object: *mut GInitiallyUnowned) {
    g_object_force_floating(object as *mut GObject);
}
unsafe extern "C" fn g_initially_unowned_class_init(mut klass: *mut GInitiallyUnownedClass) {}
unsafe extern "C" fn _weak_ref_clean_pointer(mut ptr: gpointer) -> *mut GObject {
    return g_pointer_bit_lock_mask_ptr(
        ptr,
        0 as guint,
        0 as gboolean,
        0 as guintptr,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GObject;
}
unsafe extern "C" fn _weak_ref_lock(
    mut weak_ref: *mut GWeakRef,
    mut out_object: *mut *mut GObject,
) {
    if !out_object.is_null() {
        let mut ptr: guintptr = 0;
        g_pointer_bit_lock_and_get(
            &raw mut (*weak_ref).priv_0.p as gpointer,
            0 as guint,
            &raw mut ptr,
        );
        *out_object = _weak_ref_clean_pointer(ptr as gpointer);
    } else {
        g_pointer_bit_lock(
            &raw mut (*weak_ref).priv_0.p as *mut ::core::ffi::c_void,
            0 as gint,
        );
    };
}
unsafe extern "C" fn _weak_ref_unlock(mut weak_ref: *mut GWeakRef) {
    g_pointer_bit_unlock(
        &raw mut (*weak_ref).priv_0.p as *mut ::core::ffi::c_void,
        0 as gint,
    );
}
unsafe extern "C" fn _weak_ref_unlock_and_set(
    mut weak_ref: *mut GWeakRef,
    mut object: *mut GObject,
) {
    g_pointer_bit_unlock_and_set(
        &raw mut (*weak_ref).priv_0.p as *mut ::core::ffi::c_void,
        0 as guint,
        object as gpointer,
        0 as guintptr,
    );
}
unsafe extern "C" fn weak_ref_data_clear_list(
    mut wrdata: *mut WeakRefData,
    mut object: *mut GObject,
) {
    while (*wrdata).len as ::core::ffi::c_uint > 0 as ::core::ffi::c_uint {
        let mut weak_ref: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
        let mut ptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        weak_ref = weak_ref_data_list_remove(
            wrdata,
            ((*wrdata).len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
                as guint16,
            0 as gboolean,
        );
        ptr = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = &raw mut (*weak_ref).priv_0.p;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as gpointer;
        if ptr == _weak_ref_clean_pointer(ptr) as gpointer {
            if ({
                let mut gapcae_oldval: gpointer = ptr;
                if 0 as ::core::ffi::c_int != 0 {
                    (*weak_ref).priv_0.p;
                } else {
                };
                let fresh21 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*weak_ref).priv_0.p,
                    *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh21.0;
                if fresh21.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) != 0
            {
                continue;
            }
        }
        _weak_ref_lock(weak_ref, ::core::ptr::null_mut::<*mut GObject>());
        _weak_ref_unlock_and_set(weak_ref, ::core::ptr::null_mut::<GObject>());
    }
}
unsafe extern "C" fn _weak_ref_set(
    mut weak_ref: *mut GWeakRef,
    mut new_object: *mut GObject,
    mut called_by_init: gboolean,
) {
    let mut old_wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    let mut new_wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    let mut old_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    new_wrdata = weak_ref_data_get_or_create(new_object);
    if called_by_init != 0 {
        old_wrdata = ::core::ptr::null_mut::<WeakRefData>();
    } else {
        _weak_ref_lock(weak_ref, &raw mut old_object);
        if old_object == new_object {
            _weak_ref_unlock(weak_ref);
            return;
        }
        old_wrdata = if !old_object.is_null() {
            weak_ref_data_ref(weak_ref_data_get(old_object))
        } else {
            ::core::ptr::null_mut::<WeakRefData>()
        };
        _weak_ref_unlock(weak_ref);
    }
    if !new_wrdata.is_null()
        && !old_wrdata.is_null()
        && (old_wrdata as gpointer as guintptr) < new_wrdata as gpointer as guintptr
    {
        weak_ref_data_lock(old_wrdata);
        weak_ref_data_lock(new_wrdata);
    } else {
        weak_ref_data_lock(new_wrdata);
        weak_ref_data_lock(old_wrdata);
    }
    _weak_ref_lock(weak_ref, &raw mut old_object);
    if weak_ref_data_has(
        old_object,
        old_wrdata,
        ::core::ptr::null_mut::<*mut WeakRefData>(),
    ) == 0
    {
        if !old_object.is_null() {
            weak_ref_data_unlock(old_wrdata);
            weak_ref_data_unlock(new_wrdata);
            _weak_ref_unlock(weak_ref);
            weak_ref_data_unref(old_wrdata);
            return;
        }
    }
    if !old_object.is_null() {
        let mut idx: gint32 = 0;
        idx = weak_ref_data_list_find(old_wrdata, weak_ref);
        if idx < 0 as ::core::ffi::c_int {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"unexpected missing GWeakRef data\0" as *const u8 as *const gchar,
            );
        } else {
            weak_ref_data_list_remove(
                old_wrdata,
                idx as guint16,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        }
    }
    weak_ref_data_unlock(old_wrdata);
    if !new_object.is_null() {
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*new_object).ref_count;
                (*new_object).ref_count;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*new_object).ref_count as *mut gint,
            );
            gaig_temp
        }) < 1 as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"calling g_weak_ref_set() with already destroyed object\0" as *const u8
                    as *const gchar,
            );
            new_object = ::core::ptr::null_mut::<GObject>();
        } else if weak_ref_data_list_add(new_wrdata, weak_ref) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"Too many GWeakRef registered\0" as *const u8 as *const gchar,
            );
            new_object = ::core::ptr::null_mut::<GObject>();
        }
    }
    _weak_ref_unlock_and_set(weak_ref, new_object);
    weak_ref_data_unlock(new_wrdata);
    weak_ref_data_unref(old_wrdata);
}
#[no_mangle]
pub unsafe extern "C" fn g_weak_ref_init(mut weak_ref: *mut GWeakRef, mut object: gpointer) {
    if !weak_ref.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_weak_ref_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"weak_ref\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if object.is_null()
        || g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_weak_ref_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"object == NULL || G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut gaps_temp_atomic: *mut gpointer = &raw mut (*weak_ref).priv_0.p;
    let mut gaps_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if 0 as ::core::ffi::c_int != 0 {
        (*weak_ref).priv_0.p;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
    if !object.is_null() {
        _weak_ref_set(
            weak_ref,
            object as *mut GObject,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_weak_ref_clear(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_set(weak_ref, ::core::ptr::null_mut::<::core::ffi::c_void>());
    (*weak_ref).priv_0.p =
        0xcccccccc as ::core::ffi::c_uint as *mut ::core::ffi::c_void as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_weak_ref_get(mut weak_ref: *mut GWeakRef) -> gpointer {
    let mut wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    let mut new_wrdata: *mut WeakRefData = ::core::ptr::null_mut::<WeakRefData>();
    let mut toggle_notify: GToggleNotify = None;
    let mut toggle_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if !weak_ref.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_weak_ref_get\0" as *const u8 as *const ::core::ffi::c_char,
            b"weak_ref\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    _weak_ref_lock(weak_ref, &raw mut object);
    wrdata = if !object.is_null() {
        weak_ref_data_ref(weak_ref_data_get(object))
    } else {
        ::core::ptr::null_mut::<WeakRefData>()
    };
    _weak_ref_unlock(weak_ref);
    if wrdata.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    loop {
        weak_ref_data_lock(wrdata);
        _weak_ref_lock(weak_ref, &raw mut object);
        if object.is_null() {
            new_wrdata = ::core::ptr::null_mut::<WeakRefData>();
        } else if weak_ref_data_has(object, wrdata, &raw mut new_wrdata) != 0 {
            object =
                object_ref(object, &raw mut toggle_notify, &raw mut toggle_data) as *mut GObject;
        }
        _weak_ref_unlock(weak_ref);
        weak_ref_data_unlock(wrdata);
        weak_ref_data_unref(wrdata);
        if new_wrdata.is_null() {
            break;
        }
        wrdata = new_wrdata;
    }
    if toggle_notify.is_some() {
        toggle_notify.expect("non-null function pointer")(toggle_data, object, 0 as gboolean);
    }
    return object as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_weak_ref_set(mut weak_ref: *mut GWeakRef, mut object: gpointer) {
    if !weak_ref.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_weak_ref_set\0" as *const u8 as *const ::core::ffi::c_char,
            b"weak_ref != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if object.is_null()
        || g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_weak_ref_set\0" as *const u8 as *const ::core::ffi::c_char,
            b"object == NULL || G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    _weak_ref_set(weak_ref, object as *mut GObject, 0 as gboolean);
}
